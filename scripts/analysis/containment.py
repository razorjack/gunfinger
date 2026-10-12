"""Experiment A of session 15: a candidate containment rule, applied offline
to stored reports. Measurement only; nothing in the product or the harness
reads it.

The rule: a confident play is reported as shared material, not as an
identification, when its span lies inside the span of a stronger confident
play (more hits, over the same passage of the mix) of a file from another
duplicate cluster. Parameters:

- tolerance: seconds the contained play may reach outside the stronger
  play's span, at either end;
- ratio: optionally, only when the contained play's hits are at most this
  share of the stronger play's (experiment 0076: 2-24% for remixes,
  55-60% for The Nine's VIP; copies of one recording over one passage of
  the 2003 mix 30-100%).

A play's hits are the sum of its segments' hits (`FoundPlay::hits`).
Containment is judged against the confident plays as found, once: a play
demoted by the rule still contains weaker plays (no cascade).

usage: python3 -I scripts/analysis/containment.py OUT.json

Reads the reports named in REPORTS below (development reports only; any
path naming the held-out test set is refused) and writes, per report kind
and grid point, the confident plays it demotes (wrong ones removed, right
ones demoted), what it cannot fix, and the scores before and after.
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import devscore  # noqa: E402

TOLERANCES = [0.0, 10.0, 30.0]
RATIOS = [None, 0.75, 0.5, 0.25]
# What "over the same passage" holds against: the stronger play's whole
# span (`play`, which bridges the gaps of up to 90 s that `plays::group`
# joins), or one of its passages (`passage`: its segments, joined where
# they are at most PASSAGE_GAP apart).
SPANS = ["play", "passage"]
PASSAGE_GAP = 10.0

WORK = devscore.ROOT / "work"
CLUSTERS_262 = WORK / "reports" / "duplicate-clusters.json"
CLUSTERS_NAS = WORK / "reports" / "library-nas-dnb-peaks" / "duplicate-clusters.json"
NAS = WORK / "reports" / "library-nas-dnb-peaks"
MAP = NAS / "library-map.json"
SHARED = WORK / "s11" / "item8c"
# Session 15: the pairs judged different by ear on 2026-10-12, not used
# in 0076 (Dozer's corpus file has no NAS record, so run 3 has no `out`).
NEW_SHARED = WORK / "s15" / "shared"

SCANS = {
    "stakka scan": ["scan-stakka-skynet-knowledge.json"],
    "stakka leave-outs": [
        "scan-stakka-skynet-knowledge-leave-out-3-seed-2026.json",
        "scan-stakka-skynet-knowledge-leave-out-11-seed-2026.json",
    ],
    "2003 scan": ["scan-razorjack-2003-03-29.json"],
    "2003 leave-out": ["scan-razorjack-2003-03-29-leave-out-3-seed-2026.json"],
}
SWEEPS = [f"sweep-seed-{seed}.json" for seed in (2026, 2027, 2028, 2029)]


def rule(span, tolerance, ratio):
    label = f"{span}, tolerance {tolerance:g} s, ratio " + ("none" if ratio is None else f"{ratio:g}")
    return {"span": span, "tolerance": tolerance, "ratio": ratio, "label": label}


GRID = [rule(span, tolerance, ratio) for span in SPANS for tolerance in TOLERANCES for ratio in RATIOS]


def passages(spans):
    """Segment spans joined where they overlap or lie at most PASSAGE_GAP
    apart."""
    joined = []
    for start, end in sorted(spans):
        if joined and start - joined[-1][1] <= PASSAGE_GAP:
            joined[-1][1] = max(joined[-1][1], end)
        else:
            joined.append([start, end])
    return joined


def holds(other, play, grid_point):
    tolerance = grid_point["tolerance"]
    spans = [[other["start"], other["end"]]] if grid_point["span"] == "play" else other["passages"]
    return any(play["start"] >= start - tolerance and play["end"] <= end + tolerance for start, end in spans)


def container(play, plays, same_recording, grid_point):
    """The stronger confident play of another cluster whose span (or one of
    whose passages) holds `play`'s, or None."""
    ratio = grid_point["ratio"]
    for other in plays:
        if other is play or not other["confident"] or other["hits"] <= play["hits"]:
            continue
        if same_recording(play, other):
            continue
        if not holds(other, play, grid_point):
            continue
        if ratio is not None and play["hits"] > ratio * other["hits"]:
            continue
        return other
    return None


def demoted(plays, same_recording, grid_point):
    """Each confident play the rule demotes, with the play that holds it."""
    found = []
    for play in plays:
        if play["confident"]:
            holder = container(play, plays, same_recording, grid_point)
            if holder is not None:
                found.append((play, holder))
    return found


def closest_call(plays, same_recording):
    """The confident play nearest to being contained: for each confident
    play, the stronger confident play of another cluster that overlaps it
    and the tolerance that containment would need (seconds outside that
    play's span); the smallest over the report, or None."""
    best, smallest = None, None
    for play in plays:
        if not play["confident"]:
            continue
        for other in plays:
            if other is play or not other["confident"] or other["hits"] <= play["hits"]:
                continue
            if same_recording(play, other) or other["end"] <= play["start"] or play["end"] <= other["start"]:
                continue
            needed = max(other["start"] - play["start"], play["end"] - other["end"], 0.0)
            if smallest is None or needed < smallest:
                smallest = needed
                best = {"tolerance needed": round(needed, 1), "play": play["asset"],
                        "span": [round(play["start"], 1), round(play["end"], 1)], "hits": play["hits"],
                        "other": other["asset"], "other_span": [round(other["start"], 1), round(other["end"], 1)],
                        "other_hits": other["hits"], "ratio": round(play["hits"] / other["hits"], 3)}
    return best


def harness_plays(report):
    plays = []
    for play in report["plays"]:
        plays.append(
            {
                "asset": play["asset"],
                "files": {play["asset"]},
                "start": play["start_seconds"],
                "end": play["end_seconds"],
                "hits": sum(segment["hits"] for segment in play["segments"]),
                "confident": devscore.play_confidence(play) == "confident",
                "segments": play["segments"],
                "passages": passages([(seg["start_seconds"], seg["end_seconds"]) for seg in play["segments"]]),
            }
        )
    return plays


def clustered(cluster_of):
    def same_recording(a, b):
        return any(file in cluster_of(other) for file in a["files"] for other in b["files"])

    return same_recording


def segment_key(found):
    return (found["asset"], found["start_seconds"], found["end_seconds"])


def scan_kind(paths, tracks_of, cluster_of):
    """Scans: plays from the report, truth from the manifest's slots."""
    same_recording = clustered(cluster_of)
    rows = []
    for grid_point in GRID:
        totals = {"confident plays": 0, "wrong plays": 0, "wrong removed": 0,
                  "right demoted": 0, "wrong not contained": 0,
                  "identified": 0, "identified after": 0, "referenced": 0,
                  "wrong detections": 0, "wrong detections after": 0}
        examples = []
        for path in paths:
            report = devscore.load_json(path)
            tracks = tracks_of(report["set"])
            slots = devscore.slots(tracks, report["duration_seconds"], cluster_of)
            plays = harness_plays(report)
            out = demoted(plays, same_recording, grid_point)
            out_keys = {segment_key(s) for play, _ in out for s in play["segments"]}
            before = devscore.score_scan(report, tracks, cluster_of)
            after = devscore.score_scan(
                report, tracks, cluster_of,
                confident=lambda d: d["confident"] and segment_key(d) not in out_keys,
            )
            demoted_ids = {id(play) for play, _ in out}
            for play in plays:
                if not play["confident"]:
                    continue
                right = bool(devscore.matching_slots(slots, play["asset"], play["start"], play["end"]))
                totals["confident plays"] += 1
                totals["wrong plays"] += not right
                if id(play) in demoted_ids:
                    totals["right demoted" if right else "wrong removed"] += 1
                elif not right:
                    totals["wrong not contained"] += 1
            for play, holder in out:
                right = bool(devscore.matching_slots(slots, play["asset"], play["start"], play["end"]))
                examples.append(describe(path, play, holder, right))
            totals["referenced"] += before["referenced"]
            totals["identified"] += before["identified"]
            totals["identified after"] += after["identified"]
            totals["wrong detections"] += before["wrong"]
            totals["wrong detections after"] += after["wrong"]
        rows.append({"rule": grid_point["label"], **totals, "demoted": examples})
    calls = {}
    for path in paths:
        report = devscore.load_json(path)
        calls[Path(str(path)).name] = closest_call(harness_plays(report), same_recording)
    rows.append({"closest calls": calls})
    return rows


def describe(path, play, holder, right):
    return {
        "report": Path(str(path)).name,
        "right": right,
        "play": play["asset"],
        "span": [round(play["start"], 1), round(play["end"], 1)],
        "hits": play["hits"],
        "inside": holder["asset"],
        "inside_span": [round(holder["start"], 1), round(holder["end"], 1)],
        "inside_hits": holder["hits"],
        "ratio": round(play["hits"] / holder["hits"], 3),
    }


def query_plays(query):
    """A sweep or recall query's detections at least possible as plays over
    one passage: the excerpt is 30 s, so every confident detection (3
    windows) covers nearly all of it and spans are taken as equal."""
    by_asset = {}
    for detection in query["detections"]:
        if devscore.confidence(detection) == "weak":
            continue
        play = by_asset.setdefault(
            detection["asset"],
            {"asset": detection["asset"], "files": {detection["asset"]}, "start": 0.0,
             "end": 30.0, "passages": [[0.0, 30.0]], "hits": 0, "confident": False},
        )
        play["hits"] += detection["hits"]
        play["confident"] |= detection["confident"]
    return list(by_asset.values())


def query_kind(paths, cluster_of):
    same_recording = clustered(cluster_of)
    rows = []
    for grid_point in GRID:
        totals = {"queries": 0, "confident plays": 0, "wrong plays": 0, "wrong removed": 0,
                  "right demoted": 0, "wrong not contained": 0,
                  "wrong on held-out queries": 0, "queries losing their answer": 0}
        examples = []
        for path in paths:
            report = devscore.load_json(path)
            for query in report["queries"]:
                totals["queries"] += 1
                plays = query_plays(query)
                recording = cluster_of(query["asset"])
                out = demoted(plays, same_recording, grid_point)
                demoted_ids = {id(play) for play, _ in out}
                had_answer = any(p["confident"] and p["asset"] in recording for p in plays)
                keeps_answer = any(
                    p["confident"] and p["asset"] in recording and id(p) not in demoted_ids for p in plays
                )
                totals["queries losing their answer"] += had_answer and not keeps_answer
                for play in plays:
                    if not play["confident"]:
                        continue
                    right = play["asset"] in recording
                    totals["confident plays"] += 1
                    totals["wrong plays"] += not right
                    if id(play) in demoted_ids:
                        totals["right demoted" if right else "wrong removed"] += 1
                    elif not right:
                        totals["wrong not contained"] += 1
                        totals["wrong on held-out queries"] += query["held_out"]
                for play, holder in out:
                    item = describe(path, play, holder, play["asset"] in recording)
                    item["query"] = query["asset"]
                    item["speed_percent"] = query.get("speed_percent")
                    examples.append(item)
        rows.append({"rule": grid_point["label"], **totals, "demoted": examples})
    return rows


def grouped(detections, rule=None):
    """`plays::group`: detections of one file at least possible, with gaps
    of at most 90 s, as one play."""
    by_asset = {}
    for detection in sorted(detections, key=lambda d: d["start_seconds"]):
        if devscore.confidence(detection, rule) == "weak":
            continue
        plays = by_asset.setdefault(detection["asset"], [])
        if plays and detection["start_seconds"] - plays[-1]["end"] <= 90.0:
            play = plays[-1]
            play["end"] = max(play["end"], detection["end_seconds"])
            play["spans"].append((detection["start_seconds"], detection["end_seconds"]))
            play["passages"] = passages(play["spans"])
            play["hits"] += detection["hits"]
            play["confident"] |= devscore.confidence(detection, rule) == "confident"
        else:
            span = (detection["start_seconds"], detection["end_seconds"])
            plays.append({"asset": detection["asset"], "files": {detection["asset"]},
                          "start": span[0], "end": span[1], "spans": [span], "passages": passages([span]),
                          "hits": detection["hits"],
                          "confident": devscore.confidence(detection, rule) == "confident"})
    return [play for plays in by_asset.values() for play in plays]


def mixes_kind(path, cluster_of):
    """Generated mixes with exact truth (crossfades, bass swaps, cuts, plays
    of 20-60 s, a returning track, held-out tracks): a play is right when a
    truth play of its cluster overlaps it."""
    same_recording = clustered(cluster_of)
    report = devscore.load_json(path)
    rows = []
    for grid_point in GRID:
        totals = {"mixes": 0, "confident plays": 0, "wrong plays": 0, "wrong removed": 0,
                  "right demoted": 0, "wrong not contained": 0, "truth plays losing their play": 0}
        examples = []
        for mix in report["mixes"]:
            truth = mix["mix"]["plays"]
            plays = grouped(mix["detections"])
            out = demoted(plays, same_recording, grid_point)
            demoted_ids = {id(play) for play, _ in out}

            def right(play):
                return any(
                    play["asset"] in cluster_of(t["asset"]) and t["start_seconds"] < play["end"]
                    and play["start"] < t["end_seconds"]
                    for t in truth
                )

            totals["mixes"] += 1
            for play in plays:
                if not play["confident"]:
                    continue
                totals["confident plays"] += 1
                totals["wrong plays"] += not right(play)
                if id(play) in demoted_ids:
                    totals["right demoted" if right(play) else "wrong removed"] += 1
                elif not right(play):
                    totals["wrong not contained"] += 1
            for t in truth:
                mine = [p for p in plays if p["confident"] and p["asset"] in cluster_of(t["asset"])
                        and t["start_seconds"] < p["end"] and p["start"] < t["end_seconds"]]
                if mine and all(id(p) in demoted_ids for p in mine):
                    totals["truth plays losing their play"] += 1
            for play, holder in out:
                examples.append(describe(mix["mix"]["name"], play, holder, right(play)))
        rows.append({"rule": grid_point["label"], **totals, "demoted": examples})
    calls = {}
    for mix in report["mixes"]:
        calls[mix["mix"]["name"]] = closest_call(grouped(mix["detections"]), same_recording)
    rows.append({"closest calls": calls})
    return rows


def robust_kind(path):
    """Robust reports keep per query only the own recording's best hits,
    the count of wrong confident answers and the strongest wrong hits, and
    whether a blend's partner was confident. Wrong confident answers: none
    in the current matcher's report, so the rule has nothing to remove and
    no wrong answer stronger than a right one to demote it; a blend's two
    tracks cover the same 30 s, so the weaker of a confident pair would be
    demoted by construction (spans are equal in an excerpt)."""
    report = devscore.load_json(path)
    queries = report["queries"]
    return {
        "report": str(Path(path).relative_to(devscore.ROOT)),
        "queries": len(queries),
        "own confident": sum(q["confident"] for q in queries),
        "wrong confident": sum(q["wrong_confident"] for q in queries),
        "wrong stronger than own": sum(
            1 for q in queries if q["wrong_confident"] and q["strongest_wrong_hits"] > q["best_hits"]
        ),
        "blends with both confident": sum(1 for q in queries if q["confident"] and q["partner_confident"]),
        "held-out queries": sum(q["held_out"] for q in queries),
    }


def shared_kind(cluster_of, copies, folder, played_in, left_out):
    """Shared passages played whole from one file (`in-N`, experiment 0076's
    method) and again with the played file's cluster left out (`out-N`):
    right is the played file's cluster, any other confident play is wrong.
    `dropped` simulates leaving out the played file alone (its other rips
    kept) by removing its play from `in-N`; it assumes the other plays keep
    their hits, as leaving the whole cluster out did in 0076."""
    plan = devscore.load_json(folder / "plan" / "plan.json")

    def names(asset):
        name = devscore.PREFIX + asset
        return {name, copies.get(asset, name)}

    def files_cluster(files):
        return set().union(*(cluster_of(file) for file in files))

    rows = {}
    for mode, numbers in (("in", played_in), ("out", left_out), ("dropped", played_in)):
        rows[mode] = []
        for grid_point in GRID:
            totals = {"runs": 0, "confident plays": 0, "wrong plays": 0, "wrong removed": 0,
                      "right demoted": 0, "wrong not contained": 0,
                      "related confident": 0, "related removed": 0,
                      "runs with a wrong play": 0, "runs with a wrong play after": 0,
                      "runs losing the right recording": 0}
            examples = []
            for number in numbers:
                item = plan[number - 1]
                report = devscore.load_json(folder / f"{'out' if mode == 'out' else 'in'}-{number}.json")
                played = cluster_of(item["played"])
                related = cluster_of(item["related"])
                plays = []
                for play in report["plays"]:
                    rips = [(asset, names(asset)) for asset in [play["asset"], *play.get("same_audio", [])]]
                    if mode == "dropped":
                        rips = [(asset, rip) for asset, rip in rips if item["played"] not in rip]
                        if not rips:
                            continue
                    files = set().union(*(rip for _, rip in rips))
                    plays.append(
                        {
                            "asset": rips[0][0],
                            "files": files,
                            "start": play["start_seconds"],
                            "end": play["end_seconds"],
                            "hits": sum(segment["hits"] for segment in play["segments"]),
                            "confident": play["confidence"] == "confident",
                            "passages": passages([(seg["start_seconds"], seg["end_seconds"])
                                                  for seg in play["segments"]]),
                        }
                    )
                same_recording = lambda a, b: bool(files_cluster(a["files"]) & files_cluster(b["files"]))
                out = demoted(plays, same_recording, grid_point)
                demoted_ids = {id(play) for play, _ in out}
                totals["runs"] += 1
                confident = [play for play in plays if play["confident"]]
                is_right = lambda play: bool(files_cluster(play["files"]) & played)
                totals["runs with a wrong play"] += any(not is_right(p) for p in confident)
                totals["runs with a wrong play after"] += any(
                    not is_right(p) and id(p) not in demoted_ids for p in confident
                )
                totals["runs losing the right recording"] += any(is_right(p) for p in confident) and not any(
                    is_right(p) and id(p) not in demoted_ids for p in confident
                )
                for play in plays:
                    if not play["confident"]:
                        continue
                    right = bool(files_cluster(play["files"]) & played)
                    is_related = bool(files_cluster(play["files"]) & related)
                    totals["confident plays"] += 1
                    totals["wrong plays"] += not right
                    totals["related confident"] += is_related
                    if id(play) in demoted_ids:
                        totals["right demoted" if right else "wrong removed"] += 1
                        totals["related removed"] += is_related
                    elif not right:
                        totals["wrong not contained"] += 1
                for play, holder in out:
                    entry = describe(f"{mode}-{number}", play, holder,
                                     bool(files_cluster(play["files"]) & played))
                    entry["why"] = item["why"]
                    examples.append(entry)
            rows[mode].append({"rule": grid_point["label"], **totals, "demoted": examples})
    return rows


def main():
    out = sys.argv[1]
    cluster_262 = devscore.clusters_of(CLUSTERS_262)
    cluster_nas = devscore.clusters_of(CLUSTERS_NAS)
    copies = {
        copy["stands_for"]: copy["corpus"]
        for copy in devscore.load_json(MAP)["copies"]
    }
    result = {"grid": [point["label"] for point in GRID], "kinds": {}}
    for size, folder, cluster_of, nas in (
        ("262", WORK / "reports", cluster_262, False),
        ("NAS", NAS, cluster_nas, True),
    ):
        tracks_of = lambda name, nas=nas: devscore.load_set(name, nas)
        for kind, names in SCANS.items():
            result["kinds"][f"{kind} ({size})"] = scan_kind(
                [folder / name for name in names], tracks_of, cluster_of
            )
        result["kinds"][f"sweeps ({size})"] = query_kind([folder / name for name in SWEEPS], cluster_of)
    result["kinds"]["recall (NAS)"] = query_kind([NAS / "recall-seed-2026.json"], cluster_nas)
    result["kinds"]["generated mixes (262)"] = mixes_kind(WORK / "reports" / "mixes-seed-2026.json", cluster_262)
    shared = shared_kind(cluster_nas, copies, SHARED, range(1, 35), range(1, 9))
    result["kinds"]["0076 passages, played file indexed"] = shared["in"]
    result["kinds"]["0076 passages, played cluster left out"] = shared["out"]
    result["kinds"]["0076 passages, played file's play dropped (other rips kept; simulated)"] = shared["dropped"]
    if (NEW_SHARED / "out-7.json").exists():
        new = shared_kind(cluster_nas, copies, NEW_SHARED, range(1, 8), [1, 2, 4, 5, 6, 7])
        result["kinds"]["new judged pairs, played file indexed"] = new["in"]
        result["kinds"]["new judged pairs, played cluster left out"] = new["out"]
        result["kinds"]["new judged pairs, played file's play dropped (simulated)"] = new["dropped"]
    result["robust (262)"] = robust_kind(
        WORK / "reports" / "variant-second-pass-skip-0.01" / "robust-seed-2026.json"
    )
    with open(out, "w") as file:
        json.dump(result, file, indent=1)
    keys = ["confident plays", "wrong plays", "wrong removed", "right demoted", "wrong not contained"]
    for kind, rows in result["kinds"].items():
        print(kind)
        for row in rows:
            if "closest calls" in row:
                for name, call in row["closest calls"].items():
                    print(f"  closest call in {name}: {call}")
                continue
            extra = ""
            if "identified" in row:
                extra = (f"; identified {row['identified']}->{row['identified after']}/{row['referenced']}, "
                         f"wrong detections {row['wrong detections']}->{row['wrong detections after']}")
            if "queries losing their answer" in row:
                extra = f"; queries losing their answer {row['queries losing their answer']}"
            if "runs" in row:
                extra = (f"; runs with a wrong play {row['runs with a wrong play']}->"
                         f"{row['runs with a wrong play after']}/{row['runs']}, "
                         f"losing the right recording {row['runs losing the right recording']}, "
                         f"related confident {row['related confident']}, removed {row['related removed']}")
            print(f"  {row['rule']:<38} " + ", ".join(f"{key} {row[key]}" for key in keys) + extra)
    print("robust (262):", result["robust (262)"])


if __name__ == "__main__":
    main()
