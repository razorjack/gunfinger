"""Session 73: `identify --under` (ADR 0025) on the development mixes, their
leave-outs and generated mixes, against the default, as pre-registered in
docs/brief-73.md.

    # the files to leave out for a run of manifest slots, for --exclude-from
    python3 -I scripts/analysis/under_score.py exclude MANIFEST SLOTS CLUSTERS OUT.txt [LIBRARY_MAP]
    # the tracklists of the default and the option: entries lost and gained
    python3 -I scripts/analysis/under_score.py diff DEFAULT.txt OPTION.txt
    # each play found under another, against a manifest
    python3 -I scripts/analysis/under_score.py manifest REPORT.json MANIFEST CLUSTERS [--nas] [--left-out SLOTS]
    # the timing variant's lines (work/bin/s73-timing): per entry and per detection
    python3 -I scripts/analysis/under_score.py fates STDERR... [--manifest MANIFEST CLUSTERS [--nas]]

SLOTS is as `scan --leave-out-slots` takes it (`2-4,6`). CLUSTERS is a
duplicate clusters file; at NAS scale (`--nas`, or a LIBRARY_MAP given)
the reports name NAS files, which the clusters name with `second-library/`.
A play is right when one of its files is in the cluster of a reference of
a slot whose span (its start to the next slot's start) overlaps it, or is
judged the same title as one (`docs/same-titles.txt`, ADR 0022).
Refuses any path naming the held-out test set.
"""

import json
import re
import sys
import tomllib
from pathlib import Path

TEST_SET = ("ed-rush-optical-essential-mix", "EM_1999_02_07")
PREFIX = "second-library/"
ROOT = Path(__file__).resolve().parents[2]


def refuse_test_set(*paths):
    for path in paths:
        if any(name in str(path) for name in TEST_SET):
            raise SystemExit(f"refused: {path} names the held-out test set")


def seconds(text):
    total = 0
    for part in text.split(":"):
        total = 60 * total + int(part)
    return total


def slot_numbers(text):
    numbers = set()
    for part in text.split(","):
        first, _, last = part.partition("-")
        numbers.update(range(int(first), int(last or first) + 1))
    return numbers


def manifest_slots(path):
    """Each slot's position, references and span; the last runs to the end."""
    refuse_test_set(path)
    with open(path, "rb") as file:
        tracks = tomllib.load(file)["track"]
    slots = []
    for index, track in enumerate(tracks):
        end = seconds(tracks[index + 1]["start"]) if index + 1 < len(tracks) else float("inf")
        slots.append({"position": track["position"], "start": seconds(track["start"]),
                      "end": end, "references": track.get("reference", []),
                      "name": f"{track.get('artist')} - {track.get('title')}"})
    return slots


def clusters_of(path):
    cluster = {}
    with open(path) as file:
        duplicates = json.load(file)["duplicates"]
    for members in duplicates:
        for member in members:
            cluster[member] = set(members)
    return cluster


def same_titles():
    pairs = set()
    with open(ROOT / "docs/same-titles.txt") as file:
        lines = file.readlines()
    for line in lines:
        if line.startswith("same-title\t"):
            _, a, b = line.rstrip("\n").split("\t")
            pairs.add((a, b))
            pairs.add((b, a))
    return pairs


def exclude(manifest, slots, clusters, out, library_map=None):
    """Every file of the clusters of the left-out slots' references, as the
    searched library names them: corpus paths, or with a library map NAS
    paths (the NAS clusters' members and the references' NAS copies)."""
    cluster = clusters_of(clusters)
    wanted = slot_numbers(slots)
    copies = {}
    if library_map:
        with open(library_map) as file:
            listed = json.load(file)["copies"]
        for copy in listed:
            copies.setdefault(copy["corpus"], []).append(copy["stands_for"])
    files = set()
    for slot in manifest_slots(manifest):
        if slot["position"] not in wanted:
            continue
        for reference in slot["references"]:
            for member in cluster.get(reference, {reference}) | {reference}:
                if library_map:
                    if member.startswith(PREFIX):
                        files.add(member[len(PREFIX):])
                    files.update(copies.get(member, []))
                elif not member.startswith(PREFIX):
                    files.add(member)
    with open(out, "w") as file:
        file.write("".join(f"{path}\n" for path in sorted(files)))
    print(f"{len(files)} files of slots {slots}")


MARK = re.compile(r" \(under \d+\.\)$")
NUMBER = re.compile(r"^ *\d+\. +")
GAP = re.compile(r"ID - ID \(")


def comparable(text):
    entries, gaps = [], []
    for line in text.splitlines():
        if not line.strip():
            continue
        if GAP.search(line) and not NUMBER.match(line):
            gaps.append(line.strip())
        else:
            entries.append(MARK.sub("", NUMBER.sub("", line)))
    return entries, gaps


def diff(default, option):
    """Entries lost (in the default's tracklist, not the option's) and gained,
    numbers and `(under N.)` marks taken off; gap lines compared apart."""
    refuse_test_set(default, option)
    with open(default) as file:
        old, old_gaps = comparable(file.read())
    with open(option) as file:
        shown = file.read()
    new, new_gaps = comparable(shown)
    lost = [line for line in old if line not in new]
    gained = [line for line in new if line not in old]
    print(json.dumps({"default": default, "entries": [len(old), len(new)], "lost": lost,
                      "gained": gained, "gaps_changed": old_gaps != new_gaps,
                      "under_marks": [line for line in shown.splitlines() if MARK.search(line)]},
                     ensure_ascii=False))


class Truth:
    def __init__(self, manifest, clusters, nas):
        self.slots = manifest_slots(manifest)
        self.cluster = clusters_of(clusters)
        self.same = same_titles()
        self.prefix = PREFIX if nas else ""

    def recording(self, path):
        return self.cluster.get(path, {path})

    def judge(self, files, start, end, left_out=()):
        """The slots playing then whose track one of `files` (as the report
        names them) is, and every slot playing then."""
        keys = {self.prefix + path for path in files}
        playing = [slot for slot in self.slots if slot["start"] < end and start < slot["end"]]
        right = []
        for slot in playing:
            references = set()
            for reference in slot["references"]:
                references |= self.recording(reference) | {reference}
            same_title = any((key, other) in self.same for key in keys for other in references)
            if keys & references or same_title:
                right.append(slot)
        return right, [slot for slot in playing if slot["position"] in left_out]


def files_of(play):
    return [play["asset"]] + play.get("same_audio", [])


def manifest_score(report_path, manifest, clusters, *flags):
    refuse_test_set(report_path, manifest)
    nas = "--nas" in flags
    left_out = slot_numbers(flags[flags.index("--left-out") + 1]) if "--left-out" in flags else set()
    truth = Truth(manifest, clusters, nas)
    report = json.load(open(report_path))
    if "under" not in report:
        raise SystemExit(f"{report_path}: not searched with --under")
    rows = []
    for found in report["under"]:
        right, playing_left_out = truth.judge(files_of(found), found["start_seconds"],
                                              found["end_seconds"], left_out)
        playing = [slot["position"] for slot in truth.slots
                   if slot["start"] < found["end_seconds"] and found["start_seconds"] < slot["end"]]
        rows.append({
            "asset": found["asset"], "confidence": found["confidence"], "hits": found["hits"],
            "start": round(found["start_seconds"], 1), "end": round(found["end_seconds"], 1),
            "under_play": found["under"]["play"], "under_asset": found["under"]["asset"],
            "right": bool(right), "slots_playing": playing,
            "right_slots": [slot["position"] for slot in right],
            "start_error": round(found["start_seconds"] - right[0]["start"], 1) if right else None,
            "in_left_out_slot": [slot["position"] for slot in playing_left_out]})
    print(json.dumps({"report": report_path, "plays": len(report["plays"]),
                      "under": rows}, ensure_ascii=False))


def fates(*arguments):
    """Per recording: the entries' times and the detections' fates."""
    paths = list(arguments)
    truth = None
    if "--manifest" in paths:
        at = paths.index("--manifest")
        truth = Truth(paths[at + 1], paths[at + 2], "--nas" in paths)
        paths = paths[:at]
    refuse_test_set(*paths)
    recordings = []
    for path in paths:
        for line in open(path):
            fields = line.rstrip("\n").split("\t")
            kind = fields[0]
            if kind == "s73-recording":
                recordings.append({"recording": Path(fields[1]).name, "entries": [],
                                   "detections": [], "shared": [], "one_track": [], "totals": []})
            elif kind == "s73-entry":
                recordings[-1]["entries"].append(dict(zip(
                    ["taken_out", "start", "span", "segments", "decode", "mask", "search",
                     "shared", "kept"], [fields[1]] + [float(x) for x in fields[2:]])))
            elif kind == "s73-entry-total":
                recordings[-1]["totals"].append(float(fields[2]))
            elif kind == "s73-detection":
                detection = dict(zip(["taken_out", "asset", "start", "end", "level", "hits",
                                      "fate"], fields[1:]))
                if truth:
                    right, _ = truth.judge([detection["asset"]], float(detection["start"]),
                                           float(detection["end"]))
                    detection["playing"] = bool(right)
                recordings[-1]["detections"].append(detection)
            elif kind == "s73-shared":
                recordings[-1]["shared"].append((fields[1], fields[2]))
            elif kind == "s73-one-track":
                recordings[-1]["one_track"].append(fields[2])
    for recording in recordings:
        shared = set(recording["shared"])
        for detection in recording["detections"]:
            if detection["fate"] == "candidate" and (detection["taken_out"], detection["asset"]) in shared:
                detection["fate"] = "shared-material"
    print(json.dumps(recordings, ensure_ascii=False))


if __name__ == "__main__":
    command, *rest = sys.argv[1:]
    {"exclude": exclude, "diff": diff, "manifest": manifest_score, "fates": fates}[command](*rest)
