"""Session 21's rendered mixes around the shared-material panel's played
files (docs/brief-21.md, pre-registration): the plan for `gunfinger-eval
shared-mixes`, and the oracle, the candidates and the feasibility
criterion applied to its report.

usage: shared_mixes.py plan <panel.json> <corpus library dir> <verified scan.json>... <out plan.json>
       shared_mixes.py score [--same <coverage.json>]... [--second-from <report.json>]
                             <panel.json> <clusters.json> <report.json> <verified scan.json>... <out.json>

`plan` writes one mix per seed and development played file with a shared
passage (each file's first direction names it), the files whose clusters
are never drawn (every file of the panel, both halves, and the five
test-derived versions), the panel's recordings, and the development mixes
to search as they are (their audio named by the verified scans, which are
read for nothing else here). Nothing here reads audio.

`score` applies the pre-registration to `shared-mixes`'s report: plays
(ADR 0006: one file's detections at least possible, joined across gaps of
at most 90 s; confident when a segment is), the oracle per play, the
candidates per confident segment and the most favourable segment per
play, and for each candidate the threshold keeping 95% of the true
confident plays and the wrong related plays it rejects. The second sample
is the confident detections the verified scans credit to referenced
tracks, matched to the report's runs of the development mixes. Each
`--same` file lists `planned`, `found` and `coverage` (`pair`'s strongest
alignment at the fitted speed over the shorter file): a pair at 80% or
more is one recording (the clustering rule), joined to the duplicate
clusters. `--second-from` takes the development mixes' runs from another
report of the same build (a report without them, such as the follow-up's).
"""
import json
import math
import os
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import devscore  # noqa: E402

SEEDS = [2101, 2102, 2103]
AROUND = 3
AUDIO = {".mp3", ".m4a", ".opus", ".ogg", ".flac", ".wav", ".aiff"}

# The five test-derived versions (brief 20, shared_panel.py's pattern).
TEST_DERIVED = re.compile(
    r"terminal 2|orders|funktion|clear skyz|beachball", re.IGNORECASE)


def panel_files(family):
    files = {path for recording in family["recordings"] for path in recording}
    return files | set(family["other_versions"]) | set(family["unjudged_same_title"])


def corpus_files(root):
    for directory, _, names in os.walk(root):
        for name in names:
            if os.path.splitext(name)[1].lower() in AUDIO:
                yield os.path.relpath(os.path.join(directory, name), root).replace(os.sep, "/")


def plan(panel_path, library_root, *rest):
    *scans, out_path = rest
    panel = json.load(open(panel_path, encoding="utf-8"))
    played = []
    seen = set()
    for family in panel["families"]:
        if family["half"] != "development":
            continue
        for direction in family["directions"]:
            passage = direction["passage"]
            if not passage or passage.get("control") or direction["played"] in seen:
                continue
            seen.add(direction["played"])
            recording = next(r for r in family["recordings"] if direction["played"] in r)
            played.append({"name": direction["name"], "file": direction["played"],
                           "recording": recording})
    exclude = set()
    for family in panel["families"]:
        exclude |= panel_files(family)
    test_derived = sorted(f for f in corpus_files(library_root) if TEST_DERIVED.search(f))
    exclude |= set(test_derived)
    queries = []
    for path in scans:
        scan = json.load(open(path, encoding="utf-8"))
        queries.append({"name": scan["set"], "audio": f"{scan['set']}/{scan['audio_file']}"})
    out = {
        "seeds": SEEDS,
        "around": AROUND,
        "exclude": sorted(exclude),
        "played": played,
        "recordings": [r for family in panel["families"] for r in family["recordings"]],
        "queries": queries,
    }
    json.dump(out, open(out_path, "w", encoding="utf-8"), indent=1, ensure_ascii=False)
    print(f"{len(played)} played files x {len(SEEDS)} seeds; {len(exclude)} files excluded "
          f"({len(test_derived)} test-derived corpus files); {len(queries)} queries")
    for name in test_derived:
        print("  test-derived:", name)


KEEP = 0.95
REJECT = 0.5
MIN_WRONG = 10
GAP_SECONDS = 90.0
MARKED_HITS = 60
INF = math.inf


def level(detection):
    rule = 240 if detection.get("fitted") else 200
    if detection["hits"] >= rule and detection["windows"] >= 3:
        return "confident"
    return "possible" if detection["hits"] >= 60 else "weak"


def plays_of(detections):
    """The product's plays: per file, segments at least possible joined
    across gaps of at most 90 s."""
    by_asset = defaultdict(list)
    for d in detections:
        if level(d) != "weak":
            by_asset[d["asset"]].append(d)
    plays = []
    for asset, segments in by_asset.items():
        segments.sort(key=lambda d: d["start_seconds"])
        current = None
        for d in segments:
            if current and d["start_seconds"] - current["end"] <= GAP_SECONDS:
                current["segments"].append(d)
                current["end"] = max(current["end"], d["end_seconds"])
            else:
                current = {"asset": asset, "start": d["start_seconds"], "end": d["end_seconds"],
                           "segments": [d]}
                plays.append(current)
    for play in plays:
        play["confident"] = [d for d in play["segments"] if level(d) == "confident"]
    return plays


def holder_hits(d, own_of):
    """(c)'s value under the clusters scored with: the hits of the strongest
    file the lookup found outside the file's own recording (`own_of`),
    whether the report cannot tell it, and that file. A report with `holders` (session 23) lists
    every file found at 30 hits or more; an older one keeps only `holder`,
    the strongest outside the own recording of its run, so where that file
    is now in the own recording a weaker holder may still exist: the value
    is then unknown and counted as 0, which bounds how many marks the
    clusters remove. Where that file has fewer than 60 hits, the value is
    below the mark whatever it is, and is counted as a known 0."""
    own = own_of(d["asset"])
    holder = d.get("holder")
    if holder and holder["asset"] in own and holder["hits"] < MARKED_HITS:
        # Any weaker holder outside the own recording is below the mark too
        # (and a session 23 report omits an empty `holders` list).
        return 0, False, None
    if "holders" in d:
        outside = [h for h in d["holders"] if h["asset"] not in own]
        if outside:
            strongest = max(outside, key=lambda h: h["hits"])
            return strongest["hits"], False, strongest["asset"]
        if holder and holder["asset"] not in own:
            return holder["hits"], False, holder["asset"]
        return 0, False, None
    if holder is None:
        return 0, False, None
    if holder["asset"] in own:
        return 0, True, None
    return holder["hits"], False, holder["asset"]


def segment_values(d, own_of):
    share = (d.get("verified") or {}).get("aligned", {}).get("share")
    sides = [b["count"]["share"] / share for b in d.get("beyond", []) if share]
    c, unknown, holder = holder_hits(d, own_of)
    return {
        "a1": d["hits"] / (d["end_seconds"] - d["start_seconds"]),
        "a2": share if share is not None else 0.0,
        "b": min(sides) if sides else INF,
        "c": c,
        "c unknown": unknown,
        "c holder": holder,
    }


LOW_SUSPICIOUS = ["a1", "a2", "b", "c+a1", "c+a2", "c+b"]
CANDIDATES = ["a1", "a2", "b", "c", "c+a1", "c+a2", "c+b"]


def play_values(play, own_of):
    """Each candidate's value for a play: the most favourable over its
    confident segments. `c unknown`: a segment's (c) the report cannot
    tell under these clusters (`holder_hits`)."""
    segs = [segment_values(d, own_of) for d in play["confident"]]
    out = {}
    for name in ("a1", "a2", "b"):
        out[name] = max(v[name] for v in segs)
        out["c+" + name] = max(INF if v["c"] < MARKED_HITS else v[name] for v in segs)
    weakest = min(segs, key=lambda v: v["c"])
    out["c"] = weakest["c"]
    out["c holder"] = weakest["c holder"]
    out["c unknown"] = any(v["c unknown"] for v in segs)
    return out


def overlaps(play, start, end):
    return play["start"] < end and start < play["end"]


SAME_RECORDING_COVERAGE = 0.8


def cluster_index(clusters, same_pairs=()):
    """Duplicate clusters, joined by the pairs `pair` measured as one
    recording."""
    out = {}

    def join(members):
        merged = set(members)
        for member in members:
            merged |= out.get(member, set())
        for member in merged:
            out[member] = merged

    for members in clusters["duplicates"]:
        join(members)
    for pair in same_pairs:
        if pair["coverage"] >= SAME_RECORDING_COVERAGE:
            join([pair["planned"], pair["found"]])
    return lambda path: out.get(path, {path})


def own_index(clusters, panel):
    """A file's own recording as `shared-mixes` takes it for (c): its
    duplicate cluster and every panel recording that holds it."""
    cluster_of = cluster_index(clusters)
    recordings = [set(r) for family in panel["families"] for r in family["recordings"]]

    def own_of(path):
        own = set(cluster_of(path))
        for recording in recordings:
            if path in recording:
                own |= recording
        return own
    return own_of


def family_roles(panel):
    """For each development played file: (family, played recording files,
    related or version files, unjudged files)."""
    out = {}
    for family in panel["families"]:
        for direction in family["directions"]:
            played = direction["played"]
            own = next(set(r) for r in family["recordings"] if played in r)
            versions = {f for r in family["recordings"] for f in r} - own
            versions |= set(family["other_versions"])
            out[played] = (family["name"], own, versions, set(family["unjudged_same_title"]))
    return out


def classify(play, mix, played_file, roles, cluster_of):
    family, own, versions, unjudged = roles[played_file]
    centre = next(p for p in mix["plays"] if p["asset"] == played_file)
    span = (centre["start_seconds"], centre["end_seconds"])
    if play["asset"] in own and overlaps(play, *span):
        return "right", "played"
    for drawn in mix["plays"]:
        if drawn is centre:
            continue
        if play["asset"] in cluster_of(drawn["asset"]) and overlaps(play, drawn["start_seconds"], drawn["end_seconds"]):
            return "right", "drawn"
    if overlaps(play, *span):
        if play["asset"] in versions:
            return "wrong related", family
        if play["asset"] in unjudged:
            return "wrong unjudged", family
    return "wrong other", family


def threshold(values, low_suspicious):
    ordered = sorted(values)
    n = len(ordered)
    if low_suspicious:
        return ordered[n - math.ceil(KEEP * n)]
    return ordered[math.ceil(KEEP * n) - 1]


def kept(value, cut, low_suspicious):
    return value >= cut if low_suspicious else value <= cut


def summary(values):
    finite = sorted(v for v in values if v != INF)
    if not finite:
        return {"n": len(values), "untestable": len(values)}
    q = lambda f: finite[min(len(finite) - 1, int(f * (len(finite) - 1) + 0.5))]
    return {"n": len(values), "untestable": len(values) - len(finite), "min": finite[0],
            "q25": q(0.25), "median": q(0.5), "q75": q(0.75), "max": finite[-1]}


def score(*arguments):
    arguments = list(arguments)
    same_pairs = []
    second_from = None
    while arguments[0] in ("--same", "--second-from"):
        if arguments[0] == "--same":
            same_pairs += json.load(open(arguments[1], encoding="utf-8"))
        else:
            second_from = arguments[1]
        arguments = arguments[2:]
    panel_path, clusters_path, report_path, *scans, out_path = arguments
    panel = json.load(open(panel_path, encoding="utf-8"))
    clusters = json.load(open(clusters_path, encoding="utf-8"))
    cluster_of = cluster_index(clusters, same_pairs)
    own_of = own_index(clusters, panel)
    report = json.load(open(report_path, encoding="utf-8"))
    roles = family_roles(panel)
    mixes = {m["mix"]["name"]: m for m in report["mixes"]}
    rows = []
    for run in report["runs"]:
        rendered = mixes.get(run["name"])
        if rendered is None:
            continue
        played_file = rendered["played"]["file"]
        for play in plays_of(run["detections"]):
            if not play["confident"]:
                continue
            outcome, role = classify(play, rendered["mix"], played_file, roles, cluster_of)
            rows.append({"run": run["index"], "mix": run["name"], "seed": rendered["seed"],
                         "played": rendered["played"]["name"], "asset": play["asset"],
                         "outcome": outcome, "role": role, "start": play["start"], "end": play["end"],
                         "hits": max(d["hits"] for d in play["confident"]),
                         "values": play_values(play, own_of)})
    true = [r for r in rows if r["outcome"] == "right" and (
        (r["run"] == "played left out" and r["role"] == "drawn")
        or (r["run"] == "source left out" and r["role"] == "played"))]
    wrong = [r for r in rows if r["outcome"] == "wrong related" and r["run"] == "played left out"]
    second = second_sample(json.load(open(second_from, encoding="utf-8")) if second_from else report,
                           scans, cluster_index(clusters), own_of)
    print(f"confident plays by run and outcome:")
    for (run, outcome, role), count in sorted(Counter((r["run"], r["outcome"], r["role"]) for r in rows).items()):
        print(f"  {run:<16} {outcome:<15} {role:<18} {count}")
    print(f"primary sample: {len(true)} true confident plays "
          f"({sum(r['run'] == 'source left out' for r in true)} positive-control), "
          f"{len(wrong)} wrong related; second sample: {len(second)} true plays")
    unknown = {
        "true": sum(r["values"]["c unknown"] for r in true),
        "wrong": sum(r["values"]["c unknown"] for r in wrong),
        "second": sum(p["values"]["c unknown"] for p in second),
    }
    decisions = {}
    for name in CANDIDATES:
        low = name in LOW_SUSPICIOUS
        cut = threshold([r["values"][name] for r in true], low)
        true_kept = sum(kept(r["values"][name], cut, low) for r in true)
        rejected = sum(not kept(r["values"][name], cut, low) for r in wrong)
        second_kept = sum(kept(p["values"][name], cut, low) for p in second)
        promising = len(wrong) >= MIN_WRONG and rejected >= REJECT * len(wrong)
        if name.startswith("c") and any(unknown.values()):
            promising = None
        decisions[name] = {
            "threshold": cut, "low is suspicious": low,
            "true kept": true_kept, "true": len(true),
            "wrong rejected": rejected, "wrong": len(wrong),
            "second kept": second_kept, "second": len(second),
            "promising": promising,
            "true values": summary([r["values"][name] for r in true]),
            "wrong values": summary([r["values"][name] for r in wrong]),
            "second values": summary([p["values"][name] for p in second]),
        }
        print(f"  {name:<5} threshold {cut:.4g}: keeps {true_kept}/{len(true)} true, rejects "
              f"{rejected}/{len(wrong)} wrong related, keeps {second_kept}/{len(second)} of the second "
              f"sample -> " + ("undetermined: some (c) values unknown" if promising is None
                               else "PROMISING" if promising else "not promising"))
    marked = {
        "true marked": sum(r["values"]["c"] >= MARKED_HITS for r in true),
        "wrong marked": sum(r["values"]["c"] >= MARKED_HITS for r in wrong),
        "second marked": sum(p["values"]["c"] >= MARKED_HITS for p in second),
    }
    print(f"  (c) at {MARKED_HITS} hits: marks {marked['true marked']}/{len(true)} true, "
          f"{marked['wrong marked']}/{len(wrong)} wrong related, {marked['second marked']}/{len(second)} second")
    marked["unknown"] = unknown
    if any(unknown.values()):
        print(f"  (c) unknown under these clusters (the run's holder is now in the own recording; "
              f"counted unmarked): {unknown['true']} true, {unknown['wrong']} wrong related, "
              f"{unknown['second']} second")
    print("wrong related plays by played file (played left out):")
    by_direction = defaultdict(list)
    for r in wrong:
        by_direction[r["played"]].append(r)
    for played, cell in sorted(by_direction.items()):
        print(f"  {played}: {len(cell)}: " + "; ".join(
            f"s{r['seed'] % 100} {r['asset'].rsplit('/', 1)[-1][:40]} {r['hits']}" for r in cell))
    for r in rows:
        if r["outcome"] in ("wrong other", "wrong unjudged"):
            print(f"  {r['outcome']} [{r['run']}] {r['mix']}: {r['asset']} {r['hits']}")
    json.dump({"pre-registration": "docs/brief-21.md", "decisions": decisions, "c at 60": marked,
               "rows": rows, "second": second},
              open(out_path, "w", encoding="utf-8"), indent=1, ensure_ascii=False,
              default=lambda v: None)


def credited_detections(scan, cluster_of):
    """The scan's confident detections credited to a referenced track under
    `cluster_of`, as `scoring::score` credits them (devscore.py); with the
    clusters of the scan's run, the stored `credited` lists."""
    tracks = devscore.load_set(scan["set"], nas=True)
    slots = devscore.slots(tracks, scan["duration_seconds"], cluster_of)
    return {(d["asset"], round(d["start_seconds"], 6), round(d["end_seconds"], 6))
            for d in scan["detections"] if d["confident"]
            and any(devscore.covers(slot, d["asset"], d["start_seconds"], d["end_seconds"])
                    for slot in slots)}


def second_sample(report, scans, cluster_of, own_of):
    """Plays of the development mixes (the report's `everything` runs of
    the queries) holding a confident detection a verified scan credits to
    a referenced track, under the clusters scored with."""
    runs = {run["name"]: run for run in report["runs"] if run["name"] not in
            {m["mix"]["name"] for m in report["mixes"]}}
    out = []
    for path in scans:
        scan = json.load(open(path, encoding="utf-8"))
        credited = credited_detections(scan, cluster_of)
        run = runs[scan["set"]]
        matched = set()
        for play in plays_of(run["detections"]):
            keys = {(d["asset"], round(d["start_seconds"], 6), round(d["end_seconds"], 6))
                    for d in play["confident"]}
            if keys & credited:
                matched |= keys & credited
                out.append({"set": scan["set"], "asset": play["asset"], "start": play["start"],
                            "end": play["end"], "values": play_values(play, own_of)})
        if matched != credited:
            print(f"  {scan['set']}: {len(credited - matched)} credited detections not found in the run")
    return out


if __name__ == "__main__":
    command, *arguments = sys.argv[1:]
    {"plan": plan, "score": score}[command](*arguments)
