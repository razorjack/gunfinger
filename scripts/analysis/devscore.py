"""Development scores recomputed offline from stored harness reports, as
`gunfinger-eval`'s `scoring.rs` computes them, under any clusters file:
scans (identified, wrong, possible, possible plays matching no track) and
sweeps or recall (each confident answer right or wrong).

A library module for the session 15 analyses, and a command:

usage: python3 -I scripts/analysis/devscore.py [--nas] CLUSTERS REPORT...

Prints each report's score under CLUSTERS beside the score stored in it.
`--nas` scores scans against the manifests' `second-library/` references
too, as for reports of runs with `--other-peaks-dir`.
Refuses any path naming the held-out test set: its manifest and reports
are never read.
"""

import json
import sys
import tomllib
from pathlib import Path

TEST_SET = "ed-rush-optical-essential-mix"
PREFIX = "second-library/"
TOLERANCE = 90.0
FITTED = {"hits": 240, "windows": 3}
LADDER = {"hits": 200, "windows": 3}
POSSIBLE_HITS = 60
ROOT = Path(__file__).resolve().parents[2]


def refuse_test_set(path):
    if TEST_SET in str(path):
        raise SystemExit(f"refused: {path} names the held-out test set")


def load_json(path):
    refuse_test_set(path)
    with open(path) as file:
        return json.load(file)


def clusters_of(path):
    """A function from a file to the set of files of its cluster."""
    clusters = load_json(path)
    cluster = {}
    for members in clusters["duplicates"]:
        for member in members:
            cluster[member] = frozenset(members)
    return lambda asset: cluster.get(asset, frozenset([asset]))


def seconds(start):
    parts = [float(part) for part in start.split(":")]
    total = 0.0
    for part in parts:
        total = total * 60 + part
    return total


def load_set(name, nas, corpus=ROOT / "corpus"):
    """The manifest's tracks: label, start, references. Without the NAS,
    `second-library/` references are set aside, as `manifest.rs` does."""
    refuse_test_set(name)
    with open(Path(corpus) / "sets" / name / "tracklist.toml", "rb") as file:
        manifest = tomllib.load(file)
    tracks = []
    for track in manifest["track"]:
        references = [
            reference
            for reference in track.get("reference", [])
            if nas or not reference.startswith(PREFIX)
        ]
        tracks.append(
            {
                "position": track["position"],
                "label": f"{track['position']:>2}. {track['artist']} - {track['title']}",
                "start": seconds(track["start"]),
                "references": references,
            }
        )
    return tracks


def confidence(segment, rule=None):
    """`Evidence::confidence` with the stored counts, under the rule of the
    pass that measured them (`Pass::rule`) unless `rule` is given."""
    rule = rule or rule_of(segment)
    if segment["hits"] >= rule["hits"] and segment["windows"] >= rule["windows"]:
        return "confident"
    if segment["hits"] >= POSSIBLE_HITS:
        return "possible"
    return "weak"


def play_confidence(play, rule=None):
    """A play takes its strongest segment's confidence (ADR 0013)."""
    order = {"weak": 0, "possible": 1, "confident": 2}
    return max((confidence(segment, rule) for segment in play["segments"]),
               key=order.get, default="weak")


def rule_of(found):
    # Reports omit `fitted` when false (`skip_serializing_if`).
    return FITTED if found.get("fitted", False) else LADDER


def slots(tracks, duration, cluster_of):
    result = []
    for position, track in enumerate(tracks):
        next_start = tracks[position + 1]["start"] if position + 1 < len(tracks) else duration
        accepted = set()
        for reference in track["references"]:
            accepted |= cluster_of(reference)
        result.append(
            {
                "track": track,
                "start": track["start"] - TOLERANCE,
                "end": next_start + TOLERANCE,
                "accepted": accepted,
            }
        )
    return result


def covers(slot, asset, start, end):
    return start < slot["end"] and slot["start"] < end and asset in slot["accepted"]


def matching_slots(slots_, asset, start, end):
    return [slot for slot in slots_ if covers(slot, asset, start, end)]


def score_scan(report, tracks, cluster_of, confident=None):
    """`scoring::score`. `confident(detection)` decides which detections
    count (default: the stored flag), so a candidate rule can demote some."""
    confident = confident or (lambda detection: detection["confident"])
    slots_ = slots(tracks, report["duration_seconds"], cluster_of)
    found = report["detections"]
    identified = possible = referenced = 0
    per_track = []
    for slot in slots_:
        track = slot["track"]
        is_referenced = bool(track["references"])
        in_slot = [d for d in found if covers(slot, d["asset"], d["start_seconds"], d["end_seconds"])]
        credited = [d for d in in_slot if confident(d)]
        hit = is_referenced and bool(credited)
        maybe = (
            is_referenced
            and not hit
            and any(covers(slot, p["asset"], p["start_seconds"], p["end_seconds"]) for p in report["plays"])
        )
        referenced += is_referenced
        identified += hit
        possible += maybe
        per_track.append({"label": track["label"], "referenced": is_referenced,
                          "identified": hit, "possible": maybe,
                          "credited": sorted({d["asset"] for d in credited})})
    wrong = [
        d for d in found
        if confident(d) and not matching_slots(slots_, d["asset"], d["start_seconds"], d["end_seconds"])
    ]
    wrong.sort(key=lambda d: -d["hits"])
    unmatched_possible = [
        p for p in report["plays"]
        if play_confidence(p) == "possible"
        and not matching_slots(slots_, p["asset"], p["start_seconds"], p["end_seconds"])
    ]
    return {
        "referenced": referenced,
        "identified": identified,
        "wrong": len(wrong),
        "possible": possible,
        "tracks": per_track,
        "wrong_identifications": wrong,
        "unmatched_possible": unmatched_possible,
    }


def rescore_queries(report, cluster_of):
    """Sweep or recall queries: each confident detection right when its
    file is in the query's cluster. Returns (right, wrong) counts of
    confident detections and the wrong ones."""
    right = 0
    wrong = []
    for query in report["queries"]:
        recording = cluster_of(query["asset"])
        for detection in query["detections"]:
            if not detection["confident"]:
                continue
            if detection["asset"] in recording:
                right += 1
            else:
                wrong.append((query, detection))
    return right, wrong


def main():
    arguments = sys.argv[1:]
    nas = "--nas" in arguments
    if nas:
        arguments.remove("--nas")
    cluster_of = clusters_of(arguments[0])
    for path in arguments[1:]:
        report = load_json(path)
        if "queries" in report:
            right, wrong = rescore_queries(report, cluster_of)
            stored = sum(
                1 for query in report["queries"] for d in query["detections"]
                if d["confident"] and not d["correct"]
            )
            print(f"{path}: {right} right and {len(wrong)} wrong confident answers (stored: {stored} wrong)")
            continue
        score = score_scan(report, load_set(report["set"], nas), cluster_of)
        stored = report["score"]
        print(
            f"{path}: {score['identified']}/{score['referenced']} identified, {score['wrong']} wrong, "
            f"{score['possible']} possible, {len(score['unmatched_possible'])} possible plays matching no track "
            f"(stored: {stored['identified']}/{stored['referenced']}, {stored['wrong']} wrong, "
            f"{stored['possible']} possible, {len(stored['unmatched_possible'])})"
        )


if __name__ == "__main__":
    main()
