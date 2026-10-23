"""Session 40: where two recordings stop lining up, as evidence for same or
different (`docs/brief-40.md`, the pre-registration).

`populations` draws the labelled pairs and writes one pair file per
population for `gunfinger-eval pair`, and `populations.json` with each
pair's label:

- a: every verdict by ear whose two files have records (`judged same`,
  `judged different`);
- b: a seeded sample of the clusters' joins at 80% coverage or more that
  no verdict touches (neither file named in a verdict), identical copies
  (the library map) left out, each pair once (`presumed same`; session
  40's draw, made before that fix, holds two pairs in both directions);
- c: files of two different recordings of one family of the shared
  panels, at most PER_RECORDING_PAIR file pairs per two recordings,
  seeded, pairs a verdict names left out (`panel different`);
- d: the strongest pair of each two clusters the clusters keep apart, no
  verdict touching them, whose files' compared artists and titles both
  differ, not in c (`presumed different`).

`measures` reads `pair` reports and computes each pair's measures from
its alignments (`measures_of`): coverage, supported time, hits per second
of the main alignment, the near-offset shares, the longest stretch
without a near hit and the far offset jumps.

Every population leaves out pairs with a file in a folder experiment
0065 lists as a mixed-CD candidate (CANDIDATES,
`docs/experiments/data/0065-nas-candidates.json`, joined boundaries of
MIXED_CD_SHARE or more): nothing runs on those.

`scores` orders the pairs of two labelled groups by each measure (the
AUC), fits the best threshold on the judged pairs with leave-one-out
cross-validation, lists the pairs each measure gets wrong and places the
other populations against the threshold fitted on all judged pairs.

usage:
  pair_offsets.py populations CLUSTERS VERDICTS NAS_STORE CORPUS_STORE MAP CANDIDATES OUT_DIR
                  PANEL...
  pair_offsets.py measures OUT.json PAIRS_REPORT...
  pair_offsets.py scores POPULATIONS MEASURES OUT.json
"""

import collections
import itertools
import json
import math
import os
import random
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from tracklist_tags import compared, tags_of  # noqa: E402

PREFIX = "second-library/"
FITTED = "peaks, at the fitted speed"
LADDER = "peaks, turntable ladder"
JOIN_COVERAGE = 0.8
MIXED_CD_SHARE = 0.75
SEED_B, SIZE_B = 4001, 100
SEED_C, PER_RECORDING_PAIR = 4002, 4
SIZE_D = 60
NEAR = (2.0, 5.0, 10.0, 20.0)
GAP_NEAR = 10.0
JUMP = 10.0
# The two pairs the hypothesis came from (2026-10-22), reported apart.
SOURCE_PAIRS = {
    ("extra/b-bad_company-china_cup.mp3",
     "second-library/__youtube_archivists/sicknote-classic-dnb/ƆƐI3C - China Cup.opus"),
    ("extra/rawkuts-gridlok-rkt337-2001-sour/a-gridlok-dilusion-sour.mp3",
     "second-library/__youtube_archivists/dfect-dnb/Gridlok - Dilusion.opus"),
}
# Each measure and whether a higher value means the same recording.
MEASURES = [
    ("coverage", True),
    ("supported", True),
    ("hits_per_second", True),
    ("near_2", True),
    ("near_5", True),
    ("near_10", True),
    ("near_20", True),
    ("longest_gap", False),
    ("far_jumps", False),
]
NO_ALIGNMENT_JUMPS = 1e9


def key(a, b):
    return tuple(sorted((a, b)))


# Measures --------------------------------------------------------------


def shorter_span(alignment, query_is_shorter):
    if query_is_shorter:
        return alignment["query_start_seconds"], alignment["query_end_seconds"]
    return alignment["found_start_seconds"], alignment["found_end_seconds"]


def search(report, name):
    return next((s["alignments"] for s in report["searches"] if s["name"] == name), [])


def measures_of(report):
    """The measures of one `pair` report (definitions in brief 40, step 1)."""
    shorter = min(report["query_seconds"], report["found_seconds"])
    query_is_shorter = report["query_seconds"] <= report["found_seconds"]
    fitted = search(report, FITTED)
    ladder = search(report, LADDER)

    def coverage_of(alignment):
        start, end = shorter_span(alignment, query_is_shorter)
        return (end - start) / shorter

    seconds = math.ceil(shorter)
    result = {
        "query": report["query"],
        "found": report["found"],
        "shorter_seconds": shorter,
        "alignments": len(fitted),
        "coverage": max((coverage_of(a) for a in ladder + fitted), default=0.0),
        "supported": sum(a.get("supported") or 0.0 for a in fitted),
    }
    if not fitted:
        result.update({"hits_per_second": 0.0, "longest_gap": float(seconds),
                       "far_jumps": NO_ALIGNMENT_JUMPS, "main_offset": None})
        result.update({f"near_{int(t)}": 0.0 for t in NEAR})
        return result
    main = fitted[0]
    span = main["query_end_seconds"] - main["query_start_seconds"]
    result["hits_per_second"] = main["hits"] / span if span > 0 else 0.0
    result["main_offset"] = main["offset_seconds"]

    def near(t):
        held = set()
        for a in fitted:
            if abs(a["offset_seconds"] - main["offset_seconds"]) <= t:
                held.update(a.get("supported_seconds") or [])
        return held

    for t in NEAR:
        result[f"near_{int(t)}"] = len(near(t)) / shorter
    result["longest_gap"] = float(longest_run_without(near(GAP_NEAR), seconds))
    ordered = sorted(fitted, key=lambda a: shorter_span(a, query_is_shorter)[0])
    result["far_jumps"] = sum(
        1 for a, b in zip(ordered, ordered[1:])
        if abs(a["offset_seconds"] - b["offset_seconds"]) > JUMP)
    return result


def longest_run_without(held, seconds):
    """The longest run of the seconds 0..seconds-1 not in `held`."""
    longest = run = 0
    for second in range(seconds):
        run = 0 if second in held else run + 1
        longest = max(longest, run)
    return longest


# Scores ----------------------------------------------------------------


def auc(same, different, higher_is_same):
    """The share of (same, different) pairs the measure orders correctly,
    ties counting half."""
    if not same or not different:
        return None
    sign = 1 if higher_is_same else -1
    right = 0.0
    for s in same:
        for d in different:
            if sign * s > sign * d:
                right += 1
            elif s == d:
                right += 0.5
    return right / (len(same) * len(different))


def says_same(value, threshold, higher_is_same):
    return value >= threshold if higher_is_same else value <= threshold


def best_threshold(labelled, higher_is_same):
    """The threshold with the fewest errors on (value, is_same) pairs:
    candidates are the midpoints between consecutive distinct values and
    one beyond each end; among ties the lower median candidate."""
    values = sorted({v for v, _ in labelled})
    candidates = [values[0] - 1.0] + [
        (a + b) / 2 for a, b in zip(values, values[1:])] + [values[-1] + 1.0]

    def errors(threshold):
        return sum(1 for v, same in labelled
                   if says_same(v, threshold, higher_is_same) != same)

    counts = [errors(c) for c in candidates]
    fewest = min(counts)
    tied = [c for c, n in zip(candidates, counts) if n == fewest]
    return tied[(len(tied) - 1) // 2], fewest


def leave_one_out(labelled, higher_is_same):
    """The indices of the pairs misjudged by the threshold fitted on the
    others."""
    wrong = []
    for i, (value, same) in enumerate(labelled):
        threshold, _ = best_threshold(labelled[:i] + labelled[i + 1:], higher_is_same)
        if says_same(value, threshold, higher_is_same) != same:
            wrong.append(i)
    return wrong


def name_of(row):
    return f"{os.path.basename(row['query'])} ~ {os.path.basename(row['found'])}"


def scores(populations_file, measures_file, out):
    labels = {key(p["query"], p["found"]): p for p in load(populations_file)["pairs"]}
    rows = []
    for row in load(measures_file):
        label = labels.get(key(row["query"], row["found"]))
        if label:
            rows.append({**row, "population": label["population"], "label": label["label"]})

    def group(population, label, sources=False):
        return [r for r in rows if r["population"] == population and r["label"] == label
                and (sources or key(r["query"], r["found"]) not in SOURCE_KEYS)]

    judged_same, judged_different = group("a", "judged same"), group("a", "judged different")
    presumed_same = group("b", "presumed same")
    panel, presumed_different = group("c", "panel different"), group("d", "presumed different")
    comparisons = {
        "judged same ~ judged different": (judged_same, judged_different),
        "the same with the source pairs": (group("a", "judged same", True),
                                           group("a", "judged different", True)),
        "presumed same (b) ~ judged different": (presumed_same, judged_different),
        "presumed same (b) ~ panel different (c)": (presumed_same, panel),
        "presumed same (b) ~ presumed different (d)": (presumed_same, presumed_different),
        "all same ~ all different": (judged_same + presumed_same,
                                     judged_different + panel + presumed_different),
    }
    sources = [r for r in rows if key(r["query"], r["found"]) in SOURCE_KEYS]
    judged = judged_same + judged_different
    result = {"counts": {name: [len(s), len(d)] for name, (s, d) in comparisons.items()},
              "measures": {}}
    for measure, higher in MEASURES:
        labelled = [(r[measure], r["label"] == "judged same") for r in judged]
        threshold, errors = best_threshold(labelled, higher)
        loo = leave_one_out(labelled, higher)
        wrong = [name_of(r) for r, (v, s) in zip(judged, labelled)
                 if says_same(v, threshold, higher) != s]

        def placed(group_rows, as_same):
            return [name_of(r) for r in group_rows
                    if says_same(r[measure], threshold, higher) != as_same]

        result["measures"][measure] = {
            "higher_is_same": higher,
            "auc": {name: auc([r[measure] for r in s], [r[measure] for r in d], higher)
                    for name, (s, d) in comparisons.items()},
            "threshold": threshold,
            "errors": errors,
            "wrong": wrong,
            "loo_errors": len(loo),
            "loo_wrong": [name_of(judged[i]) for i in loo],
            "presumed_same_below": placed(presumed_same, True),
            "panel_different_above": placed(panel, False),
            "presumed_different_above": placed(presumed_different, False),
            "source_pairs": {name_of(r): {"value": r[measure], "judged": r["label"],
                                          "says_same": says_same(r[measure],
                                                                 threshold, higher)}
                             for r in sources},
        }
    with open(out, "w") as file:
        json.dump(result, file, indent=1, ensure_ascii=False)
    print_scores(result)


SOURCE_KEYS = {key(a, b) for a, b in SOURCE_PAIRS}


def print_scores(result):
    print("pairs (same, different):", json.dumps(result["counts"], ensure_ascii=False))
    for measure, s in result["measures"].items():
        aucs = " ".join(f"{v:.3f}" if v is not None else "-" for v in s["auc"].values())
        print(f"{measure:16} AUC {aucs}  threshold {s['threshold']:.4g} errors {s['errors']}"
              f" LOO {s['loo_errors']}  (b) below {len(s['presumed_same_below'])}"
              f"  (c) above {len(s['panel_different_above'])}"
              f"  (d) above {len(s['presumed_different_above'])}")


# Populations -----------------------------------------------------------


def load(path):
    with open(path) as file:
        return json.load(file)


def verdicts_of(path):
    verdicts = []
    with open(path) as file:
        for line in file:
            if line.startswith("#") or not line.strip():
                continue
            verdict, a, b = line.rstrip("\n").split("\t")
            verdicts.append((verdict, a, b))
    return verdicts


def store_tags(store, prefix):
    tags = {}
    for name in os.listdir(store):
        if name.endswith(".tags"):
            source, artist, title = tags_of(os.path.join(store, name))
            tags[prefix + source] = (compared(artist), compared(title))
    return tags


def populations(arguments):
    clusters_file, verdicts_file, nas_store, corpus_store, map_file, candidates, out = (
        arguments[:7])
    panels = arguments[7:]
    mixed_cds = {f["folder"] for f in load(candidates)["folder_edges"]
                 if f["folder"] and f["share_joined"] >= MIXED_CD_SHARE}
    clusters = load(clusters_file)
    verdicts = verdicts_of(verdicts_file)
    # Every record has a `.tags` file beside it, so these are the files
    # with records.
    tags = {**store_tags(nas_store, PREFIX), **store_tags(corpus_store, "")}
    recorded = {path for path in tags
                if not (path.startswith(PREFIX)
                        and os.path.dirname(path[len(PREFIX):]) in mixed_cds)}
    named = {path for _, a, b in verdicts for path in (a, b)}
    cluster_of = {m: f"cluster {n}" for n, members in enumerate(clusters["duplicates"])
                  for m in members}

    def cluster(path):
        return cluster_of.get(path, path)

    judged_clusters = {key(cluster(a), cluster(b)) for _, a, b in verdicts}
    identical = set()
    for copies in load(map_file)["copies"]:
        files = [copies["corpus"]] + [PREFIX + p for p in [copies["stands_for"],
                                                            *copies.get("also", [])]]
        identical.update(key(a, b) for a, b in itertools.combinations(files, 2))

    chosen, left_out = [], collections.Counter()
    for verdict, a, b in verdicts:
        if a in recorded and b in recorded:
            chosen.append(entry("a", f"judged {verdict}", a, b))
        else:
            why = "in a mixed-CD candidate folder" if a in tags and b in tags else (
                "without a record")
            left_out[f"a: a file {why}"] += 1
    joins = sorted(
        (p for p in clusters["pairs"]
         if p["same_recording"] and p.get("aural_verdict") is None
         and p["coverage"] >= JOIN_COVERAGE and p["query"] not in named
         and p["found"] not in named and key(p["query"], p["found"]) not in identical
         and p["query"] in recorded and p["found"] in recorded),
        key=lambda p: (p["query"], p["found"]))
    # The clusters file lists many pairs in both directions: draw each once.
    joins = list({key(p["query"], p["found"]): p for p in joins}.values())
    for p in random.Random(SEED_B).sample(joins, min(SIZE_B, len(joins))):
        chosen.append(entry("b", "presumed same", p["query"], p["found"]))
    verdict_pairs = {key(a, b) for _, a, b in verdicts}
    panel_pairs = set()
    rng = random.Random(SEED_C)
    for panel in panels:
        for family in load(panel)["families"]:
            for first, second in itertools.combinations(family["recordings"], 2):
                files = [(a, b) for a in first for b in second
                         if key(a, b) not in verdict_pairs and a in recorded and b in recorded]
                for a, b in rng.sample(files, min(PER_RECORDING_PAIR, len(files))):
                    chosen.append(entry("c", "panel different", a, b, family["name"]))
                panel_pairs.update(key(a, b) for a in first for b in second)
    apart = {}
    for p in clusters["pairs"]:
        a, b = p["query"], p["found"]
        clusters_key = key(cluster(a), cluster(b))
        if (p["same_recording"] or p.get("aural_verdict") is not None
                or clusters_key in judged_clusters or key(a, b) in panel_pairs
                or a not in recorded or b not in recorded):
            continue
        (artist_a, title_a), (artist_b, title_b) = tags.get(a, (None, None)), tags.get(
            b, (None, None))
        if not (artist_a and title_a and artist_b and title_b):
            continue
        if artist_a == artist_b or title_a == title_b:
            continue
        best = apart.get(clusters_key)
        if best is None or (p["hits"], p["coverage"]) > (best["hits"], best["coverage"]):
            apart[clusters_key] = p
    strongest = sorted(apart.values(), key=lambda p: (-p["hits"], -p["coverage"],
                                                      p["query"], p["found"]))
    for p in strongest[:SIZE_D]:
        chosen.append(entry("d", "presumed different", p["query"], p["found"]))
    os.makedirs(out, exist_ok=True)
    for population in "abcd":
        with open(os.path.join(out, f"s40-{population}.txt"), "w") as file:
            for e in chosen:
                if e["population"] == population:
                    file.write(f"{e['query']}\t{e['found']}\n")
    counts = collections.Counter((e["population"], e["label"]) for e in chosen)
    summary = {"counts": {f"{p} {label}": n for (p, label), n in sorted(counts.items())},
               "left_out": dict(left_out), "joins_to_draw_from": len(joins),
               "apart_cluster_pairs_with_different_tags": len(apart)}
    with open(os.path.join(out, "populations.json"), "w") as file:
        json.dump({**summary, "pairs": chosen}, file, indent=1, ensure_ascii=False)
    print(json.dumps(summary, indent=1, ensure_ascii=False))


def entry(population, label, query, found, family=None):
    e = {"population": population, "label": label, "query": query, "found": found}
    if family:
        e["family"] = family
    return e


def measures(arguments):
    out, reports = arguments[0], arguments[1:]
    rows = [measures_of(report) for path in reports for report in load(path)]
    with open(out, "w") as file:
        json.dump(rows, file, indent=1, ensure_ascii=False)
    print(f"{len(rows)} pairs measured")


def main():
    command, arguments = sys.argv[1], sys.argv[2:]
    if command == "populations":
        populations(arguments)
    elif command == "measures":
        measures(arguments)
    elif command == "scores":
        scores(*arguments[:3])
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main()
