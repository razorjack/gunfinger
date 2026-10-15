"""Session 21's rendered mixes around the shared-material panel's played
files (docs/brief-21.md, pre-registration): the plan for `gunfinger-eval
shared-mixes`, and the oracle, the candidates and the feasibility
criterion applied to its report.

usage: shared_mixes.py plan <panel.json> <corpus library dir> <verified scan.json>... <out plan.json>
       shared_mixes.py score <panel.json> <clusters.json> <report.json> <verified scan.json>... <out.json>

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
tracks, matched to the report's runs of the development mixes.
"""
import json
import math
import os
import re
import sys
from collections import Counter, defaultdict

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


def segment_values(d):
    share = (d.get("verified") or {}).get("aligned", {}).get("share")
    sides = [b["count"]["share"] / share for b in d.get("beyond", []) if share]
    holder = d.get("holder")
    return {
        "a1": d["hits"] / (d["end_seconds"] - d["start_seconds"]),
        "a2": share if share is not None else 0.0,
        "b": min(sides) if sides else INF,
        "c": holder["hits"] if holder else 0,
    }


LOW_SUSPICIOUS = ["a1", "a2", "b", "c+a1", "c+a2", "c+b"]
CANDIDATES = ["a1", "a2", "b", "c", "c+a1", "c+a2", "c+b"]


def play_values(play):
    """Each candidate's value for a play: the most favourable over its
    confident segments."""
    segs = [segment_values(d) for d in play["confident"]]
    out = {}
    for name in ("a1", "a2", "b"):
        out[name] = max(v[name] for v in segs)
        out["c+" + name] = max(INF if v["c"] < MARKED_HITS else v[name] for v in segs)
    out["c"] = min(v["c"] for v in segs)
    return out


def overlaps(play, start, end):
    return play["start"] < end and start < play["end"]


def cluster_index(clusters):
    out = {}
    for members in clusters["duplicates"]:
        for member in members:
            out[member] = set(members)
    return lambda path: out.get(path, {path})


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


def score(panel_path, clusters_path, report_path, *rest):
    *scans, out_path = rest
    panel = json.load(open(panel_path, encoding="utf-8"))
    cluster_of = cluster_index(json.load(open(clusters_path, encoding="utf-8")))
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
                         "values": play_values(play)})
    true = [r for r in rows if r["outcome"] == "right" and (
        (r["run"] == "played left out" and r["role"] == "drawn")
        or (r["run"] == "source left out" and r["role"] == "played"))]
    wrong = [r for r in rows if r["outcome"] == "wrong related" and r["run"] == "played left out"]
    second = second_sample(report, scans)
    print(f"confident plays by run and outcome:")
    for (run, outcome, role), count in sorted(Counter((r["run"], r["outcome"], r["role"]) for r in rows).items()):
        print(f"  {run:<16} {outcome:<15} {role:<18} {count}")
    print(f"primary sample: {len(true)} true confident plays "
          f"({sum(r['run'] == 'source left out' for r in true)} positive-control), "
          f"{len(wrong)} wrong related; second sample: {len(second)} true plays")
    decisions = {}
    for name in CANDIDATES:
        low = name in LOW_SUSPICIOUS
        cut = threshold([r["values"][name] for r in true], low)
        true_kept = sum(kept(r["values"][name], cut, low) for r in true)
        rejected = sum(not kept(r["values"][name], cut, low) for r in wrong)
        second_kept = sum(kept(p["values"][name], cut, low) for p in second)
        promising = len(wrong) >= MIN_WRONG and rejected >= REJECT * len(wrong)
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
              f"sample -> {'PROMISING' if promising else 'not promising'}")
    marked = {
        "true marked": sum(r["values"]["c"] >= MARKED_HITS for r in true),
        "wrong marked": sum(r["values"]["c"] >= MARKED_HITS for r in wrong),
        "second marked": sum(p["values"]["c"] >= MARKED_HITS for p in second),
    }
    print(f"  (c) at {MARKED_HITS} hits: marks {marked['true marked']}/{len(true)} true, "
          f"{marked['wrong marked']}/{len(wrong)} wrong related, {marked['second marked']}/{len(second)} second")
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


def second_sample(report, scans):
    """Plays of the development mixes (the report's `everything` runs of
    the queries) holding a confident detection a verified scan credits to
    a referenced track."""
    runs = {run["name"]: run for run in report["runs"] if run["name"] not in
            {m["mix"]["name"] for m in report["mixes"]}}
    out = []
    for path in scans:
        scan = json.load(open(path, encoding="utf-8"))
        credited = {(d["asset"], round(d["start_seconds"], 6), round(d["end_seconds"], 6))
                    for track in scan["score"]["tracks"] for d in track.get("credited", [])
                    if d.get("confident")}
        run = runs[scan["set"]]
        matched = set()
        for play in plays_of(run["detections"]):
            keys = {(d["asset"], round(d["start_seconds"], 6), round(d["end_seconds"], 6))
                    for d in play["confident"]}
            if keys & credited:
                matched |= keys & credited
                out.append({"set": scan["set"], "asset": play["asset"], "start": play["start"],
                            "end": play["end"], "values": play_values(play)})
        if matched != credited:
            print(f"  {scan['set']}: {len(credited - matched)} credited detections not found in the run")
    return out


if __name__ == "__main__":
    command, *arguments = sys.argv[1:]
    {"plan": plan, "score": score}[command](*arguments)
