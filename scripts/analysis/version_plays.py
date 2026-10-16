"""Session 24: the plays `gunfinger-eval versions` verifies, and the
scores of its output against the pre-registration (docs/brief-24.md,
step 3).

usage:
  version_plays.py calibration PANEL CLUSTERS SHARED_REPORT HOLDERS OUT
      the panel's development half (a `shared` report): the wrong
      confident plays of the `played left out` index (negatives) and the
      right confident plays of the `source left out` index (the positive
      control), by session 20's oracle under CLUSTERS, as
      ambiguity_note.py's `panel` forms them; siblings from HOLDERS
      (`gunfinger-eval holders` output for their confident segments).
  version_plays.py e1 PANEL CLUSTERS CRITERIA DEV_REPORT OUT
      the true plays of the three development mixes (`second` in
      CRITERIA, `shared_mixes.py score` output) with their segments from
      DEV_REPORT (`shared-mixes` runs of the mixes as they are); siblings
      from each confident segment's `holders`.
  version_plays.py e2 PANEL CLUSTERS CRITERIA MIXES_REPORT OUT
      the rendered mixes' wrong related and right drawn plays of the
      `played left out` runs and the right played plays of the `source
      left out` runs (`rows` in CRITERIA) with their segments from
      MIXES_REPORT; siblings: each confident segment's strongest holder.
  version_plays.py threshold VERIFIED OUT
      T by the pre-registered rule from the calibration's negatives, and
      the positives kept at T.
  version_plays.py evaluate T VERIFIED... OUT
      plays rejected at T (S1 >= T) by set and label.

A play's segments are the product's: one file's detections at least
possible, joined across gaps of at most 90 s. Nothing here reads audio
or the held-out test set.
"""

import json
import math
import sys
import tomllib
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import ambiguity_note  # noqa: E402
import devscore  # noqa: E402
import shared_mixes  # noqa: E402
import shared_outcomes  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
SIBLING_HITS = 60
QUERIES = ROOT / "work/shared/shared-material-seed-2026-development-83361e93ae0502ae"
RENDERED_MIXES = ROOT / "work/shared-mixes/shared-mixes-s21"
DEVELOPMENT_SETS = ("stakka-skynet-knowledge", "razorjack-2003-03-29", "nocny-trans-vertical-2002-08-23")


def load(path):
    return devscore.load_json(path)


def dump(value, path):
    with open(path, "w", encoding="utf-8") as file:
        json.dump(value, file, indent=1, ensure_ascii=False)


def segment(d):
    return {"start_seconds": d["start_seconds"], "end_seconds": d["end_seconds"],
            "track_start_seconds": d["track_start_seconds"], "speed": d["speed"]}


def play_entry(identifier, mix, asset, segments, siblings, meta):
    return {"id": identifier, "mix": str(mix), "reference": asset,
            "siblings": sorted(siblings), "segments": [segment(d) for d in segments], "meta": meta}


def calibration(panel_path, clusters_path, report_path, holders_path, out_path):
    panel = load(panel_path)
    report = load(report_path)
    holders = ambiguity_note.holders_of(holders_path)
    own_of = ambiguity_note.own_index(clusters_path, panel)
    recordings = shared_outcomes.expanded(report, shared_outcomes.clusters_of(clusters_path))
    roles = shared_outcomes.roles_of(panel)
    plays = []
    for result in report["results"]:
        if result["index"] == "everything":
            continue
        family, versions, unjudged, control = roles[result["scenario"]]
        if control:
            continue
        outcome, _, _, _ = shared_outcomes.outcome(
            result, versions, unjudged, recordings.get(result["scenario"]))
        found = [dict(d, role=shared_outcomes.role(
            dict(d, role=shared_outcomes.reroled(d, recordings.get(result["scenario"]))),
            versions, unjudged)) for d in result["detections"]
            if d["level"] != "weak" and d["role"] != "partner"]
        for play in ambiguity_note.plays_of(found):
            confident = [d for d in play["segments"] if d["level"] == "confident"]
            if not confident:
                continue
            role = confident[0]["role"]
            if result["index"] == "played left out" and outcome == "wrong confident" and role != "played":
                label = "negative"
            elif result["index"] == "source left out" and role == "played":
                label = "positive"
            else:
                continue
            own = own_of(play["asset"])
            siblings = set()
            for d in confident:
                name = ambiguity_note.key(f"{result['query']}|{result['index']}", d["asset"],
                                          d["track_start_seconds"], d["track_end_seconds"])
                looked = holders.get(name)
                for h in (looked or {}).get("holders", []):
                    if h["hits"] >= SIBLING_HITS and h["asset"] not in own:
                        siblings.add(h["asset"])
            identifier = f"cal|{result['query']}|{result['index']}|{play['asset']}|{play['start']:.2f}"
            plays.append(play_entry(identifier, QUERIES / f"{result['query']}.mp3", play["asset"],
                                    play["segments"], siblings,
                                    {"set": "calibration", "label": label, "role": role,
                                     "family": family, "direction": result["scenario"],
                                     "kind": result["kind"], "query": result["query"],
                                     "index": result["index"],
                                     "hits": max(d["hits"] for d in confident)}))
    dump({"plays": plays}, out_path)
    print(Counter((p["meta"]["label"], p["meta"]["role"]) for p in plays))
    print(f"negatives from {len({p['meta']['query'] for p in plays if p['meta']['label'] == 'negative'})} queries")


def mix_audio(set_name):
    with open(ROOT / "corpus/sets" / set_name / "tracklist.toml", "rb") as file:
        return ROOT / "corpus/sets" / set_name / tomllib.load(file)["audio"]


def find_play(detections, asset, start, end):
    for play in shared_mixes.plays_of(detections):
        if play["asset"] == asset and math.isclose(play["start"], start) and math.isclose(play["end"], end):
            return play
    raise SystemExit(f"no play of {asset} at {start}-{end}")


def e1(panel_path, clusters_path, criteria_path, report_path, out_path):
    panel = load(panel_path)
    own_of = shared_mixes.own_index(load(clusters_path), panel)
    report = load(report_path)
    runs = {run["name"]: run for run in report["runs"] if run["index"] == "everything"}
    plays = []
    for true in load(criteria_path)["second"]:
        assert true["set"] in DEVELOPMENT_SETS, true["set"]
        play = find_play(runs[true["set"]]["detections"], true["asset"], true["start"], true["end"])
        own = own_of(play["asset"])
        siblings = {h["asset"] for d in play["confident"] for h in d.get("holders", [])
                    if h["hits"] >= SIBLING_HITS and h["asset"] not in own}
        identifier = f"e1|{true['set']}|{play['asset']}|{play['start']:.2f}"
        plays.append(play_entry(identifier, mix_audio(true["set"]), play["asset"], play["segments"],
                                siblings, {"set": "E1", "label": "true", "mix": true["set"],
                                           "start": play["start"], "end": play["end"],
                                           "hits": max(d["hits"] for d in play["confident"])}))
    dump({"plays": plays}, out_path)
    print(f"E1: {len(plays)} plays", Counter(p["meta"]["mix"] for p in plays))


def e2(panel_path, clusters_path, criteria_path, report_path, out_path):
    panel = load(panel_path)
    own_of = shared_mixes.own_index(load(clusters_path), panel)
    report = load(report_path)
    runs = {(run["name"], run["index"]): run for run in report["runs"]}
    plays = []
    for row in load(criteria_path)["rows"]:
        if row["run"] == "played left out" and row["outcome"] == "wrong related":
            label = "wrong related"
        elif row["run"] == "played left out" and row["outcome"] == "right" and row["role"] == "drawn":
            label = "true drawn"
        elif row["run"] == "source left out" and row["outcome"] == "right" and row["role"] == "played":
            label = "positive control"
        else:
            continue
        play = find_play(runs[(row["mix"], row["run"])]["detections"], row["asset"], row["start"], row["end"])
        own = own_of(play["asset"])
        siblings = {d["holder"]["asset"] for d in play["confident"] if d.get("holder")
                    and d["holder"]["hits"] >= SIBLING_HITS and d["holder"]["asset"] not in own}
        identifier = f"e2|{row['mix']}|{row['run']}|{play['asset']}|{play['start']:.2f}"
        plays.append(play_entry(identifier, RENDERED_MIXES / f"{row['mix']}.mp3", play["asset"],
                                play["segments"], siblings,
                                {"set": "E2", "label": label, "mix": row["mix"], "run": row["run"],
                                 "played": row["played"], "role": row["role"],
                                 "start": play["start"], "end": play["end"], "hits": row["hits"]}))
    dump({"plays": plays}, out_path)
    print(f"E2: {len(plays)} plays", Counter(p["meta"]["label"] for p in plays))


def s1_order(value):
    """Undefined S1 sorts lowest."""
    return -1.0 if value is None else value


def threshold(verified_path, out_path):
    verified = load(verified_path)
    negatives = [v for v in verified if v["meta"]["label"] == "negative"]
    positives = [v for v in verified if v["meta"]["label"] == "positive"]
    errors = [v["id"] for v in verified if v.get("error")]
    ordered = sorted((s1_order(v["summaries"]["s1"]) for v in negatives), reverse=True)
    at = ordered[math.ceil(len(ordered) / 2) - 1]
    reaches_half = at > 0
    if not reaches_half:
        above = [value for value in ordered if value > 0]
        at = min(above) if above else math.inf
    rejected = lambda v: v["summaries"]["s1"] is not None and v["summaries"]["s1"] >= at
    result = {
        "pre-registration": "docs/brief-24.md, step 3",
        "T": at, "reaches half": reaches_half,
        "negatives": len(negatives), "negatives rejected": sum(map(rejected, negatives)),
        "negatives undefined": sum(v["summaries"]["s1"] is None for v in negatives),
        "positives": len(positives), "positives kept": sum(not rejected(v) for v in positives),
        "positives undefined": sum(v["summaries"]["s1"] is None for v in positives),
        "errors": errors,
        "negatives by family": by_group(negatives, "family", rejected),
        "positives by family": by_group(positives, "family", rejected),
        "negatives by kind": by_group(negatives, "kind", rejected),
        "s1 negatives": quantiles([v["summaries"]["s1"] for v in negatives]),
        "s1 positives": quantiles([v["summaries"]["s1"] for v in positives]),
        "s2 negatives": quantiles([v["summaries"]["s2"] for v in negatives]),
        "s2 positives": quantiles([v["summaries"]["s2"] for v in positives]),
        "s3 negatives": quantiles([v["summaries"]["s3_seconds"] for v in negatives]),
        "s3 positives": quantiles([v["summaries"]["s3_seconds"] for v in positives]),
    }
    dump(result, out_path)
    for name, value in result.items():
        print(f"{name}: {value}")


def by_group(plays, field, rejected):
    groups = defaultdict(lambda: [0, 0])
    for v in plays:
        groups[v["meta"][field]][0] += 1
        groups[v["meta"][field]][1] += rejected(v)
    return {name: f"{r} of {n} rejected" for name, (n, r) in sorted(groups.items())}


def quantiles(values):
    defined = sorted(v for v in values if v is not None)
    if not defined:
        return {"n": len(values), "undefined": len(values)}
    q = lambda f: round(defined[min(len(defined) - 1, int(f * (len(defined) - 1) + 0.5))], 3)
    return {"n": len(values), "undefined": len(values) - len(defined), "min": q(0),
            "q25": q(0.25), "median": q(0.5), "q75": q(0.75), "q95": q(0.95), "max": q(1)}


def evaluate(at, *paths):
    *verified_paths, out_path = paths
    at = float(at)
    rows = []
    for path in verified_paths:
        for v in load(path):
            s = v["summaries"]
            rows.append({"id": v["id"], **v["meta"], "s1": s["s1"], "s2": s["s2"],
                         "s3": s["s3_seconds"], "supported": s["supported"],
                         "contradicted": s["contradicted"], "unobservable": s["unobservable"],
                         "silent": s["silent"], "error": v.get("error"),
                         "rejected": s["s1"] is not None and s["s1"] >= at})
    counts = defaultdict(lambda: [0, 0, 0])
    for r in rows:
        cell = counts[(r["set"], r["label"])]
        cell[0] += 1
        cell[1] += r["rejected"]
        cell[2] += r["s1"] is None
    summary = {f"{s} {label}": {"plays": n, "rejected": k, "s1 undefined": u}
               for (s, label), (n, k, u) in sorted(counts.items())}
    for name, cell in summary.items():
        print(f"{name}: {cell}")
    for key in sorted({(r["set"], r["label"]) for r in rows}):
        cell = [r for r in rows if (r["set"], r["label"]) == key]
        print(f"  {key}: S1 {quantiles([r['s1'] for r in cell])}")
    dump({"T": at, "summary": summary, "plays": rows}, out_path)


if __name__ == "__main__":
    command, *arguments = sys.argv[1:]
    {"calibration": calibration, "e1": e1, "e2": e2, "threshold": threshold,
     "evaluate": evaluate}[command](*arguments)
