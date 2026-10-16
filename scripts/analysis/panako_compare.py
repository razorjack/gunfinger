"""Session 24b: Panako as an outside benchmark beside Gunfinger, scored as
pre-registered in `docs/brief-24b.md` (step 3) and experiment 0130.

usage: python3 -I scripts/analysis/panako_compare.py COMMAND MAIN ...

MAIN is the checkout whose `corpus/` and `work/` hold the audio, the
renders and Gunfinger's saved reports (read only). Commands:

  plan MAIN LIBRARY OUT      the per-seed index lists (OUT/index-seed-N.txt,
                             absolute paths) from LIBRARY (one
                             library-relative path per line)
  gunfinger MAIN             Gunfinger's columns from its saved reports
  sweep MAIN SEED RESULTS    Panako's sweep answers (its `query` output)
  scan MAIN SET RESULTS      Panako's detections in a development mix
  mixes MAIN RESULTS...      Panako's detections in seed 2026's mixes
                             (one output file per mix, named mix-NN*)
  selfcheck MAIN             the sweep scorer on made-up perfect answers

Panako's result lines (README) are `;`-separated: index, total, query
path, query start and stop, match path, match id, match start and stop,
score, time factor, frequency factor, share of seconds with a match. The
time factor is Gunfinger's speed. Every printed result is a confident
answer; `--min-score S` keeps those with a score of at least S (the
stricter acceptance chosen on seed 2026's sweep). Prints JSON with file
names only. Refuses any path naming the held-out test set.
"""

import json
import re
import statistics
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import devscore  # noqa: E402

SEEDS = (2026, 2027, 2028, 2029)
SPEEDS = (-8.0, -5.0, -3.0, -1.0, 0.0, 1.0, 3.0, 5.0, 8.0)
START_LIMIT = 3.0
SPEED_LIMIT = 1.0
RENDER = re.compile(r"(\d{3})-([+-]\d+)\.mp3$")
MIX = re.compile(r"(mix-\d{2})")
PANELS = Path(__file__).resolve().parents[2] / "docs" / "panels"


def paths(main):
    main = Path(main)
    return {
        "library": main / "corpus" / "library",
        "corpus": main / "corpus",
        "baseline": main / "work" / "baselines" / "session-24-start",
        "reports": main / "work" / "reports",
        "mixes": main / "work" / "mixes" / "seed-2026",
    }


def load_json(path):
    return devscore.load_json(path)


def panel(seed):
    return load_json(PANELS / f"sweep-seed-{seed}.json")


def held_out(seed, cluster_of):
    """The seed's held-out files, each extended to its cluster
    (`Plan::for_seed`)."""
    return set().union(*(cluster_of(asset) for asset in panel(seed)["held_out"]))


def answers(results, library, min_score=0):
    """Panako's result lines as answers. The query path and the last seven
    fields are fixed; the match path is what lies between."""
    found = []
    prefix = str(library).rstrip("/") + "/"
    with open(results) as file:
        for line in file:
            fields = [field.strip() for field in line.split(";")]
            if len(fields) < 13 or not fields[0].isdigit():
                continue
            match = ";".join(fields[5:-7])
            answer = {
                "query": fields[2],
                "query_start": float(fields[3]),
                "query_stop": float(fields[4]),
                "asset": match[len(prefix):] if match.startswith(prefix) else match,
                "match_start": float(fields[-6]),
                "match_stop": float(fields[-5]),
                "score": int(float(fields[-4])),
                "time_factor": float(fields[-3].rstrip("%").strip()),
                "frequency_factor": float(fields[-2].rstrip("%").strip()),
                "seconds_with_match": float(fields[-1].rstrip("%").strip()),
            }
            if answer["score"] >= min_score:
                found.append(answer)
    return found


def sweep_truth(seed):
    draws = panel(seed)["draws"]
    return {
        f"{number:03}-{speed:+.0f}.mp3": {**draws[number], "speed_percent": speed}
        for number in range(len(draws))
        for speed in SPEEDS
    }


def score_sweep(seed, found, cluster_of):
    """`sweep::speed_row`: recall per speed over the indexed queries, every
    answer outside the query's cluster wrong; speed and start errors of
    each query's strongest right answer, and a stricter recall without
    answers off by more than START_LIMIT s or SPEED_LIMIT points."""
    truth = sweep_truth(seed)
    by_query = {}
    for answer in found:
        name = Path(answer["query"]).name
        if not RENDER.search(name) or name not in truth:
            raise SystemExit(f"not a render of seed {seed}: {answer['query']}")
        by_query.setdefault(name, []).append(answer)
    rows, wrong, other_position = [], [], []
    for speed in SPEEDS:
        recalled = strict = 0
        speed_errors, start_errors = [], []
        queries = [name for name, query in truth.items() if query["speed_percent"] == speed]
        for name in queries:
            query = truth[name]
            ratio = 1 + speed / 100
            own = cluster_of(query["asset"])
            right = []
            for answer in by_query.get(name, []):
                if answer["asset"] in own:
                    right.append(answer)
                else:
                    wrong.append({"query": name, "truth": query["asset"], "held_out": query["held_out"],
                                  "asset": answer["asset"], "score": answer["score"]})
            if query["held_out"] or not right:
                continue
            recalled += 1
            best = max(right, key=lambda answer: answer["score"])
            speed_error = (best["time_factor"] - ratio) * 100
            start_error = best["match_start"] - (query["start_seconds"] + ratio * best["query_start"])
            speed_errors.append(abs(speed_error))
            start_errors.append(abs(start_error))
            if abs(start_error) <= START_LIMIT and abs(speed_error) <= SPEED_LIMIT:
                strict += 1
            else:
                other_position.append({"query": name, "asset": best["asset"],
                                       "speed_error_points": speed_error, "start_error_seconds": start_error})
        indexed = sum(1 for name in queries if not truth[name]["held_out"])
        rows.append({
            "speed_percent": speed,
            "indexed": indexed,
            "recalled": recalled,
            "recalled_in_position": strict,
            "wrong": sum(1 for w in wrong if truth[w["query"]]["speed_percent"] == speed),
            "mean_speed_error_points": statistics.fmean(speed_errors) if speed_errors else None,
            "max_speed_error_points": max(speed_errors, default=None),
            "median_start_error_seconds": statistics.median(start_errors) if start_errors else None,
            "max_start_error_seconds": max(start_errors, default=None),
        })
    return {"seed": seed, "per_speed": rows, "wrong": wrong, "other_position": other_position,
            "strictest_wrong_score": max((w["score"] for w in wrong), default=None)}


def boundaries(tracks, report, cluster_of, only=None):
    """Per identified track, the earliest start of its credited confident
    detections minus the manifest's start."""
    slots = devscore.slots(tracks, report["duration_seconds"], cluster_of)
    result = []
    for slot in slots:
        position = slot["track"]["position"]
        if only is not None and position not in only:
            continue
        credited = [d["start_seconds"] for d in report["detections"]
                    if d["confident"] and devscore.covers(slot, d["asset"], d["start_seconds"], d["end_seconds"])]
        if slot["track"]["references"] and credited:
            result.append({"position": position, "error_seconds": min(credited) - slot["track"]["start"]})
    return result


BOUNDARY_TRACKS = {"razorjack-2003-03-29": {4}}
SETS = ("stakka-skynet-knowledge", "razorjack-2003-03-29", "nocny-trans-vertical-2002-08-23")


def score_set(main, name, report, cluster_of):
    tracks = devscore.load_set(name, False, paths(main)["corpus"])
    score = devscore.score_scan(report, tracks, cluster_of)
    return {
        "set": name,
        "referenced": score["referenced"],
        "identified": score["identified"],
        "found": [t["label"] for t in score["tracks"] if t["identified"]],
        "missed": [t["label"] for t in score["tracks"] if t["referenced"] and not t["identified"]],
        "wrong": [{"asset": d["asset"], "start_seconds": d["start_seconds"]}
                  for d in score["wrong_identifications"]],
        "boundaries": boundaries(tracks, report, cluster_of, BOUNDARY_TRACKS.get(name)),
    }


def score_mix(mix, detections, cluster_of):
    """`mixes::score` with every detection confident."""
    plays = mix["plays"]

    def matches(detection, play):
        return (detection["asset"] in cluster_of(play["asset"])
                and detection["start_seconds"] < play["end_seconds"]
                and play["start_seconds"] < detection["end_seconds"])

    found = [any(matches(d, play) for d in detections) for play in plays]
    return {
        "mix": mix["name"],
        "plays": len(plays),
        "indexed_plays": sum(1 for play in plays if not play["held_out"]),
        "found": sum(found),
        "missed": [play["asset"] for play, hit in zip(plays, found) if not hit and not play["held_out"]],
        "wrong": [d["asset"] for d in detections if not any(matches(d, play) for play in plays)],
    }


def gunfinger(main):
    where = paths(main)
    cluster_of = devscore.clusters_of(where["reports"] / "duplicate-clusters.json")
    sweeps = []
    for seed in SEEDS:
        folder = where["baseline"] if seed == 2026 else where["reports"]
        report = load_json(folder / f"sweep-seed-{seed}.json")
        _, wrong = devscore.rescore_queries(report, cluster_of)
        sweeps.append({"seed": seed, "indexed_assets": report["indexed_assets"],
                       "per_speed": report["per_speed"], "wrong": len(wrong)})
    sets = []
    for name in SETS:
        folder = where["baseline"] if name == SETS[0] else where["reports"]
        sets.append(score_set(main, name, load_json(folder / f"scan-{name}.json"), cluster_of))
    mixes = load_json(where["reports"] / "mixes-seed-2026.json")
    mix_rows = []
    for result in mixes["mixes"]:
        plays = result["score"]["plays"]
        mix_rows.append({
            "mix": result["mix"]["name"],
            "plays": len(plays),
            "indexed_plays": sum(1 for play in plays if not play["truth"]["held_out"]),
            "found": sum(1 for play in plays if play["level"] == "confident"),
            "wrong": len(result["score"]["wrong_confident"]),
        })
    return {"sweeps": sweeps, "sets": sets, "mixes": mix_rows}


def plan(main, library_list, out):
    where = paths(main)
    cluster_of = devscore.clusters_of(where["reports"] / "duplicate-clusters.json")
    library = Path(library_list).read_text().splitlines()
    out = Path(out)
    out.mkdir(parents=True, exist_ok=True)
    counts = {}
    for seed in SEEDS:
        left_out = held_out(seed, cluster_of)
        indexed = [path for path in library if path not in left_out]
        (out / f"index-seed-{seed}.txt").write_text(
            "".join(f"{where['library'] / path}\n" for path in indexed))
        counts[seed] = len(indexed)
    return {"library": len(library), "indexed_per_seed": counts}


def selfcheck(main):
    """Perfect made-up answers must give full recall in position and no
    wrong answer; one answer of another file must count as wrong."""
    where = paths(main)
    cluster_of = devscore.clusters_of(where["reports"] / "duplicate-clusters.json")
    truth = sweep_truth(2026)
    found = []
    for name, query in truth.items():
        if query["held_out"]:
            continue
        ratio = 1 + query["speed_percent"] / 100
        found.append({"query": f"/x/{name}", "query_start": 1.0, "query_stop": 29.0,
                      "asset": query["asset"], "match_start": query["start_seconds"] + ratio,
                      "match_stop": 0.0, "score": 50, "time_factor": ratio,
                      "frequency_factor": ratio, "seconds_with_match": 1.0})
    stranger = next(q for q in truth.values() if q["asset"] not in cluster_of(found[0]["asset"]))
    found.append({**found[0], "asset": stranger["asset"], "score": 7})
    score = score_sweep(2026, found, cluster_of)
    assert all(row["recalled_in_position"] == row["indexed"] == 60 for row in score["per_speed"]), score
    assert len(score["wrong"]) == 1 and score["strictest_wrong_score"] == 7, score["wrong"]
    return {"selfcheck": "passed"}


def main():
    arguments = sys.argv[1:]
    min_score = 0
    if "--min-score" in arguments:
        at = arguments.index("--min-score")
        min_score = int(arguments[at + 1])
        del arguments[at:at + 2]
    command, main_checkout, *rest = arguments
    where = paths(main_checkout)
    cluster_of = devscore.clusters_of(where["reports"] / "duplicate-clusters.json")
    if command == "plan":
        result = plan(main_checkout, *rest)
    elif command == "gunfinger":
        result = gunfinger(main_checkout)
    elif command == "sweep":
        seed, results = int(rest[0]), rest[1]
        result = score_sweep(seed, answers(results, where["library"], min_score), cluster_of)
    elif command == "scan":
        name, results = rest
        devscore.refuse_test_set(name)
        report = load_json(where["reports"] / f"scan-{name}.json")
        detections = [{"asset": a["asset"], "start_seconds": a["query_start"], "end_seconds": a["query_stop"],
                       "confident": True, "hits": a["score"]}
                      for a in answers(results, where["library"], min_score)]
        made = {"duration_seconds": report["duration_seconds"], "detections": detections, "plays": []}
        result = score_set(main_checkout, name, made, cluster_of)
    elif command == "mixes":
        result = []
        for results in rest:
            name = MIX.search(Path(results).name).group(1)
            mix = load_json(where["mixes"] / f"{name}.truth.json")
            detections = [{"asset": a["asset"], "start_seconds": a["query_start"],
                           "end_seconds": a["query_stop"]}
                          for a in answers(results, where["library"], min_score)]
            result.append(score_mix(mix, detections, cluster_of))
    elif command == "selfcheck":
        result = selfcheck(main_checkout)
    else:
        raise SystemExit(__doc__)
    json.dump(result, sys.stdout, indent=1)
    print()


if __name__ == "__main__":
    main()
