"""Session 18: the fire-and-forget view of an `identify` report (ADR 0017).
Gunfinger's user reads the default tracklist and checks every file an entry
names. For each entry: its files, and whether one of them is a file the
manifest accepts for a track playing there (a reference or a member of its
duplicate cluster); an entry none of whose files is accepted shows only
a wrong title. Then, for each track of the manifest, the entries over it.

Entries are rebuilt from the JSON report as `gunfinger` builds them
(`export::entries`, `report::mark_shared_material`): plays over the same
passage (overlapping for half the shorter one) are one entry when the
track would have started within 5 s of the same mix time or their files'
artist and title tags compare equal; chains join; a possible play inside
a confident play of another file is marked as sharing material and never
joined.

usage: python3 -I scripts/analysis/tracklist_view.py CLUSTERS SET REPORT
(CLUSTERS: the NAS duplicate clusters; REPORT: `identify -f json`; the
report's assets are library paths, named `second-library/<path>` here.)
"""

import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import devscore  # noqa: E402

SAME_PASSAGE_OVERLAP = 0.5
SAME_RECORDING_SECONDS = 5.0


def compared(text):
    words = [w.lower() for w in re.split(r"[^\w]|_", text or "") if w]
    return " ".join(words) or None


def compared_name(play):
    tags = play.get("tags") or {}
    artist, title = compared(tags.get("artist")), compared(tags.get("title"))
    return (artist, title) if artist and title else None


def name(play_or_tags, asset):
    tags = play_or_tags or {}
    if tags.get("title"):
        return f"{tags['artist']} - {tags['title']}" if tags.get("artist") else tags["title"]
    return Path(asset).stem


def mark_shared(plays):
    for play in plays:
        play["shares"] = None
        if play["confidence"] != "possible":
            continue
        for number, other in enumerate(plays):
            if (other["confidence"] == "confident" and other["asset"] != play["asset"]
                    and play["asset"] not in other.get("same_audio", [])
                    and other["start_seconds"] <= play["start_seconds"]
                    and play["end_seconds"] <= other["end_seconds"]):
                play["shares"] = number
                break


def over_same_passage(a, b):
    if a["shares"] is not None or b["shares"] is not None:
        return False
    overlap = min(a["end_seconds"], b["end_seconds"]) - max(a["start_seconds"], b["start_seconds"])
    shorter = min(a["end_seconds"] - a["start_seconds"], b["end_seconds"] - b["start_seconds"])
    return overlap >= SAME_PASSAGE_OVERLAP * shorter


def started_at(play):
    return play["start_seconds"] - play["track_start_seconds"] / play["speed"]


def joined(a, b):
    if not over_same_passage(a, b):
        return False
    if abs(started_at(a) - started_at(b)) <= SAME_RECORDING_SECONDS:
        return True
    return compared_name(a) is not None and compared_name(a) == compared_name(b)


def entries(plays):
    firsts = list(range(len(plays)))
    for later in range(len(plays)):
        for earlier in range(later):
            if joined(plays[later], plays[earlier]):
                kept, gone = sorted((firsts[earlier], firsts[later]))
                firsts = [kept if first == gone else first for first in firsts]
    groups = {}
    for play, first in zip(plays, firsts):
        groups.setdefault(first, []).append(play)
    result = []
    for group in groups.values():
        group.sort(key=lambda p: (p["confidence"] != "confident", -p["hits"]))
        result.append(group)
    result.sort(key=lambda g: min(p["start_seconds"] for p in g))
    return result


def clock(seconds):
    seconds = int(seconds)
    hours, rest = divmod(seconds, 3600)
    minutes, secs = divmod(rest, 60)
    return f"{hours}:{minutes:02}:{secs:02}" if hours else f"{minutes}:{secs:02}"


def main():
    clusters, set_name, report_path = sys.argv[1:4]
    cluster_of = devscore.clusters_of(clusters)
    report = devscore.load_json(report_path)
    tracks = devscore.load_set(set_name, nas=True)
    duration = report["query"]["duration_seconds"]
    slots = devscore.slots(tracks, duration, cluster_of)
    plays = report["plays"]
    mark_shared(plays)
    for play in plays:
        play["files"] = [devscore.PREFIX + a for a in [play["asset"], *play.get("same_audio", [])]]
    groups = entries(plays)
    covered = {id(slot): [] for slot in slots}
    print(f"{len(groups)} entries")
    for number, group in enumerate(groups, 1):
        first = group[0]
        start = min(p["start_seconds"] for p in group)
        end = max(p["end_seconds"] for p in group)
        tolerant = [s for s in slots if any(
            devscore.covers(s, f, p["start_seconds"], p["end_seconds"]) for p in group for f in p["files"])]
        right = sorted({s["track"]["label"].strip() for s in tolerant})
        level = first["confidence"] + (" (shares material)" if first["shares"] is not None else "")
        verdict = ("right file among them: " + "; ".join(right)) if right else "ONLY WRONG FILES"
        print(f"{number:>2}. {clock(start)}-{clock(end)} {level}: {name(first.get('tags'), first['asset'])}"
              f"  [{verdict}]")
        for play in group:
            for file in play["files"]:
                accepted = [s["track"]["label"].strip() for s in slots
                            if devscore.covers(s, file, play["start_seconds"], play["end_seconds"])]
                tags = play.get("tags") if file == devscore.PREFIX + play["asset"] else \
                    play.get("same_audio_tags", {}).get(file[len(devscore.PREFIX):])
                print(f"      {play['confidence']:<9} {play['hits']:>5} hits {clock(play['start_seconds'])}"
                      f"-{clock(play['end_seconds'])} {name(tags, file)} | {file[len(devscore.PREFIX):]}"
                      f"{'  <- ' + ', '.join(accepted) if accepted else ''}")
        for slot in tolerant:
            covered[id(slot)].append(number)
    print("tracks:")
    for slot in slots:
        state = "absent" if not slot["track"]["references"] else (
            f"entries {covered[id(slot)]}" if covered[id(slot)] else "no entry with its file")
        print(f"  {slot['track']['label']} at {clock(slot['track']['start'])}: {state}")


main()
