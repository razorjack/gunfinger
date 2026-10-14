"""Session 18: a development scan read track by track, for giving each
miss and each wrong identification a cause.

For each track of the manifest: whether it is identified (ADR 0004, as
`devscore.score_scan`), the files credited, and the strongest detection
of an accepted file (a reference or a cluster member) overlapping its
slot whatever its confidence. Then every confident play that matches no
slot, every possible play that matches no slot, and everything at least
possible inside the slots of absent tracks (`reference = []`), which are
candidates to judge by ear rather than scored results.

usage: python3 -I scripts/analysis/devset_view.py [--nas] CLUSTERS REPORT
"""

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import devscore  # noqa: E402


def clock(seconds):
    seconds = int(round(seconds))
    hours, rest = divmod(seconds, 3600)
    minutes, secs = divmod(rest, 60)
    return f"{hours}:{minutes:02}:{secs:02}" if hours else f"{minutes}:{secs:02}"


def short(asset):
    return asset.removeprefix(devscore.PREFIX)


def describe(found):
    lock = " key-locked" if found.get("key_locked") else ""
    return (f"{clock(found['start_seconds'])}-{clock(found['end_seconds'])} "
            f"{found['hits']} hits {found['windows']} windows speed {found['speed']:.4f}{lock}"
            f" {short(found['asset'])}")


def play_hits(play):
    return max(segment["hits"] for segment in play["segments"])


def main():
    arguments = sys.argv[1:]
    nas = "--nas" in arguments
    clusters, path = [a for a in arguments if a != "--nas"]
    cluster_of = devscore.clusters_of(clusters)
    report = devscore.load_json(path)
    tracks = devscore.load_set(report["set"], nas)
    slots = devscore.slots(tracks, report["duration_seconds"], cluster_of)
    score = devscore.score_scan(report, tracks, cluster_of)
    print(f"{report['set']}: {score['identified']}/{score['referenced']} identified, "
          f"{score['wrong']} wrong, {score['possible']} referenced found only as possible; "
          f"left out {report.get('left_out_tracks')}")
    for slot, row in zip(slots, score["tracks"]):
        track = slot["track"]
        state = ("identified" if row["identified"] else "possible only" if row["possible"]
                 else "missed" if row["referenced"] else "absent")
        print(f"{row['label']} at {clock(track['start'])}: {state}")
        if not row["referenced"]:
            continue
        in_slot = [d for d in report["detections"]
                   if devscore.covers(slot, d["asset"], d["start_seconds"], d["end_seconds"])]
        credited = [d for d in in_slot if d["confident"]]
        by_asset = {}
        for detection in credited:
            if detection["hits"] > by_asset.get(detection["asset"], {"hits": -1})["hits"]:
                by_asset[detection["asset"]] = detection
        for detection in sorted(by_asset.values(), key=lambda d: -d["hits"]):
            print(f"    credited {describe(detection)}")
        if not credited:
            strongest = max(in_slot, key=lambda d: d["hits"], default=None)
            print(f"    strongest accepted detection: {describe(strongest) if strongest else 'none'}")
    print("confident plays matching no slot (wrong):")
    for detection in score["wrong_identifications"]:
        print(f"    {describe(detection)}")
    print("possible plays matching no slot:")
    for play in score["unmatched_possible"]:
        segment = max(play["segments"], key=lambda s: s["hits"])
        print(f"    {clock(play['start_seconds'])}-{clock(play['end_seconds'])} "
              f"strongest segment {describe(segment)}")
    for slot in slots:
        if slot["track"]["references"]:
            continue
        start = slot["track"]["start"]
        end = slot["end"] - devscore.TOLERANCE
        print(f"absent slot {slot['track']['label']} {clock(start)}-{clock(end)}: "
              "plays at least possible overlapping it")
        for play in report["plays"]:
            if play["start_seconds"] < end and start < play["end_seconds"]:
                confidence = devscore.play_confidence(play)
                if confidence == "weak":
                    continue
                segment = max(play["segments"], key=lambda s: s["hits"])
                accepting = [other["track"]["label"].strip() for other in slots
                             if devscore.covers(other, play["asset"], play["start_seconds"], play["end_seconds"])]
                accepted = f" (accepted for {', '.join(accepting)})" if accepting else ""
                print(f"    {confidence}: {clock(play['start_seconds'])}-{clock(play['end_seconds'])} "
                      f"strongest segment {describe(segment)}{accepted}")


main()
