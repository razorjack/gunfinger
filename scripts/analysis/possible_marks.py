"""Our decision of 2026-10-14 (ADR 0019): the default tracklist
leaves out a possible play that lies inside a confident play of another
recording, and "inside" is tested on the confident play's passages instead
of its whole span. Measurement only (experiment 0107).

Two tests mark a possible play as sharing material with a confident play
of another file:

- span: it lies within that play's first and last second, as
  `mark_shared_material` (`crates/gunfinger/src/report.rs`) tested before
  ADR 0019;
- passage: it lies within one of that play's passages, its segments
  joined where they are at most PASSAGE_GAP apart (experiment 0091).

Neither marks a play of the track playing there: `mark_shared_material`
skips a possible play when a confident play overlapping most of it lines
up with it or carries the same artist and title, and the play joins that
entry. The harness reports store neither alignment nor tags, so a
confident play of the same duplicate cluster stands in for both.

For every stored development report it counts the possible plays, those
each test marks, and, of those the passage test marks (the lines the
default tracklist would leave out), the right ones: plays of the track
playing there by the manifest or the generated truth. A right play left
out is lost when its track has no other line in the tracklist: no
confident play and no unmarked possible play of an accepted file over the
same slot, truth play or excerpt.

Sweep and recall reports store no times for their detections; their
excerpts are 30 s, so every play is taken to span the excerpt, as
`containment.py` does, and both tests then mark the same plays.

usage: python3 -I scripts/analysis/possible_marks.py OUT.json

Refuses any path naming the held-out test set.
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import containment  # noqa: E402
import devscore  # noqa: E402

PASSAGE_GAP = containment.PASSAGE_GAP
WORK = devscore.ROOT / "work"
REPORTS = WORK / "reports"
NAS = containment.NAS


def inside_span(play, other):
    return other["start"] <= play["start"] and play["end"] <= other["end"]


def inside_passage(play, other):
    return any(start <= play["start"] and play["end"] <= end for start, end in other["passages"])


TESTS = {"span": inside_span, "passage": inside_passage}


def overlaps_most_of(a, b):
    overlap = min(a["end"], b["end"]) - max(a["start"], b["start"])
    return overlap >= 0.5 * min(a["end"] - a["start"], b["end"] - b["start"])


def holder(play, plays, inside, same_recording):
    """The first confident play of another file that holds `play`, or None,
    in report order as `mark_shared_material` takes it."""
    if play["level"] != "possible" or any(
        other["level"] == "confident" and same_recording(play, other) and overlaps_most_of(play, other)
        for other in plays
    ):
        return None
    for other in plays:
        if other["level"] == "confident" and other["asset"] != play["asset"] and inside(play, other):
            return other
    return None


def with_levels(plays, level):
    for play in plays:
        play["level"] = level(play)
    return [play for play in plays if play["level"] != "weak"]


def scan_plays(report):
    plays = containment.harness_plays(report)
    for play, stored in zip(plays, report["plays"]):
        play["level"] = devscore.play_confidence(stored)
    return [play for play in plays if play["level"] != "weak"]


def grouped_plays(detections):
    plays = containment.grouped(detections)
    return with_levels(plays, lambda play: "confident" if play["confident"] else "possible")


def query_plays(query):
    plays = containment.query_plays(query)
    return with_levels(plays, lambda play: "confident" if play["confident"] else "possible")


def assess(plays, truths, same_recording):
    """`truths`: for a play, the keys of the truth items it is right for
    (slots, truth plays or the excerpt's recording). Counts per test, and
    the plays the passage test leaves out."""
    counts = {"possible plays": sum(play["level"] == "possible" for play in plays)}
    marks = {name: {id(play): holder(play, plays, inside, same_recording) for play in plays}
             for name, inside in TESTS.items()}
    for name in TESTS:
        counts[f"marked ({name})"] = sum(mark is not None for mark in marks[name].values())
    left_out = [play for play in plays if marks["passage"][id(play)] is not None]
    listed = [play for play in plays if play["level"] == "confident" or marks["passage"][id(play)] is None]
    right, lost, details = 0, 0, []
    for play in left_out:
        keys = truths(play)
        if keys:
            right += 1
            if not any(truths(other) & keys for other in listed):
                lost += 1
        holding = marks["passage"][id(play)]
        details.append({
            "play": play["asset"], "span": [round(play["start"], 1), round(play["end"], 1)],
            "hits": play["hits"], "right": bool(keys),
            "inside": holding["asset"], "inside_span": [round(holding["start"], 1), round(holding["end"], 1)],
        })
    relabelled = [
        {"play": play["asset"], "span": [round(play["start"], 1), round(play["end"], 1)],
         "inside_span_only": marks["span"][id(play)]["asset"], "right": bool(truths(play))}
        for play in plays
        if marks["span"][id(play)] is not None and marks["passage"][id(play)] is None
    ]
    counts.update({"left out (passage)": len(left_out), "left out and right": right,
                   "left out, right and lost": lost})
    return counts, details, relabelled


def add(total, counts):
    for key, value in counts.items():
        total[key] = total.get(key, 0) + value


def scans(paths, nas, cluster_of):
    total, details, relabelled = {}, [], []
    for path in paths:
        report = devscore.load_json(path)
        tracks = devscore.load_set(report["set"], nas)
        slots = devscore.slots(tracks, report["duration_seconds"], cluster_of)

        def truths(play, slots=slots):
            return {slot["track"]["position"]
                    for slot in devscore.matching_slots(slots, play["asset"], play["start"], play["end"])}

        counts, found, moved = assess(scan_plays(report), truths, containment.clustered(cluster_of))
        add(total, counts)
        for item in found + moved:
            item["report"] = path.name
        details += found
        relabelled += moved
    return {"counts": total, "left out": details, "relabelled": relabelled}


def mixes(path, cluster_of):
    total, details, relabelled = {}, [], []
    for mix in devscore.load_json(path)["mixes"]:
        truth = mix["mix"]["plays"]

        def truths(play, truth=truth):
            return {index for index, t in enumerate(truth)
                    if play["asset"] in cluster_of(t["asset"])
                    and t["start_seconds"] < play["end"] and play["start"] < t["end_seconds"]}

        counts, found, moved = assess(grouped_plays(mix["detections"]), truths, containment.clustered(cluster_of))
        add(total, counts)
        for item in found + moved:
            item["report"] = f"{path.name} {mix['mix']['name']}"
        details += found
        relabelled += moved
    return {"counts": total, "left out": details, "relabelled": relabelled}


def queries(paths, cluster_of):
    total, details = {"queries": 0}, []
    for path in paths:
        for query in devscore.load_json(path)["queries"]:
            total["queries"] += 1
            recording = cluster_of(query["asset"])

            def truths(play, recording=recording):
                return {"own"} if play["asset"] in recording else set()

            counts, found, _ = assess(query_plays(query), truths, containment.clustered(cluster_of))
            add(total, counts)
            for item in found:
                item["report"] = f"{path.name} {query['asset']} {query.get('speed_percent')}"
                # A right play left out inside a confident play of its own
                # recording (another rip or upload) repeats a listed title.
                item["inside own recording"] = item["inside"] in recording
            total["left out and right, inside own recording"] = total.get(
                "left out and right, inside own recording", 0
            ) + sum(item["right"] and item["inside own recording"] for item in found)
            details += found
    return {"counts": total, "left out": details}


def main():
    out = sys.argv[1]
    cluster_262 = devscore.clusters_of(containment.CLUSTERS_262)
    cluster_nas = devscore.clusters_of(containment.CLUSTERS_NAS)
    result = {}
    for size, folder, cluster_of, nas in (("262", REPORTS, cluster_262, False), ("NAS", NAS, cluster_nas, True)):
        kinds = dict(containment.SCANS, **(containment.NAS_SCANS if nas else {}))
        for kind, names in kinds.items():
            result[f"{kind} ({size})"] = scans([folder / name for name in names], nas, cluster_of)
        result[f"sweeps ({size})"] = queries([folder / name for name in containment.SWEEPS], cluster_of)
    result["recall (NAS)"] = queries([NAS / "recall-seed-2026.json"], cluster_nas)
    for seed in (2026, 7):
        result[f"generated mixes, seed {seed} (262)"] = mixes(REPORTS / f"mixes-seed-{seed}.json", cluster_262)
    with open(out, "w") as file:
        json.dump(result, file, indent=1)
    for kind, data in result.items():
        print(f"{kind}: " + ", ".join(f"{key} {value}" for key, value in data["counts"].items()))
        for item in data["left out"]:
            if "sweep" in kind or "recall" in kind:
                continue
            print(f"    left out: {item['play'][-60:]} {item['span']} {item['hits']} hits, "
                  f"{'RIGHT' if item['right'] else 'not right'}, inside {item['inside'][-50:]} {item['inside_span']}")
        for item in data.get("relabelled", []):
            print(f"    span only, now plain possible: {item['play'][-60:]} {item['span']} "
                  f"{'RIGHT' if item['right'] else 'not right'} (was inside {item['inside_span_only'][-50:]})")


if __name__ == "__main__":
    main()
