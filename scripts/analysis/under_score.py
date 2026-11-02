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
    # A B B A timings: LOGS/cost-NAME-{a1,b1,b2,a2}.err and REPORTS/NAME-*.json
    python3 -I scripts/analysis/under_score.py cost LOGS REPORTS NAME...
    # budget variants from the timing variant's entries and the gained entries
    python3 -I scripts/analysis/under_score.py budget FATES.json GAINED.json REPORTS [--manifest]

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
from collections import Counter
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
    lost = list((Counter(old) - Counter(new)).elements())
    gained = list((Counter(new) - Counter(old)).elements())
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
                                   "detections": [], "shared": [], "one_track": []})
            elif kind == "s73-entry":
                recordings[-1]["entries"].append(dict(zip(
                    ["taken_out", "start", "span", "segments", "decode", "mask", "search",
                     "shared", "kept"], [fields[1]] + [float(x) for x in fields[2:]])))
            elif kind == "s73-entry-total":
                entries = recordings[-1]["entries"]
                if not entries or entries[-1]["taken_out"] != fields[1] or "total" in entries[-1]:
                    # The entry stopped before its own line (a decode that failed).
                    entries.append({"taken_out": fields[1], "failed": True})
                entries[-1]["total"] = float(fields[2])
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
                recordings[-1]["one_track"].append(dict(zip(
                    ["taken_out", "asset", "start", "end", "hits"], fields[1:])))
    for recording in recordings:
        shared = set(recording["shared"])
        for detection in recording["detections"]:
            if detection["fate"] != "candidate":
                continue
            if (detection["taken_out"], detection["asset"]) in shared:
                detection["fate"] = "shared-material"
            elif any(dropped["taken_out"] == detection["taken_out"]
                     and float(dropped["start"]) < float(detection["end"])
                     and float(detection["start"]) < float(dropped["end"])
                     for dropped in recording["one_track"]):
                # Grouped into a play dropped as one track with a confident
                # play; the play may be named by another of its files.
                detection["fate"] = "one-track"
    print(json.dumps(recordings, ensure_ascii=False))


def budget(fates_path, gained_path, reports, *flags):
    """Budget variants computed from the timing variant's entries (`fates`
    output) and the gained entries: the step only for entries whose
    taken-out play spans at least L seconds, or for the K longest entries
    of each mix. GAINED is `tracklist_score.py score` output (rows with
    `under_entries`) or, with `--manifest`, a JSON list of `manifest`
    outputs; REPORTS holds `<recording stem>.json`, the option's reports,
    to number the plays taken out."""
    with open(fates_path) as file:
        recordings = json.load(file)
    with open(gained_path) as file:
        gained_file = json.load(file)
    gained = {}
    if "--manifest" in flags:
        for result in gained_file:
            stem = Path(result["report"]).stem
            gained[stem] = [(row["under_play"], row["right"]) for row in result["under"]
                            if row["confidence"] == "confident"]
    else:
        for row in gained_file["rows"]:
            gained[row["mix"]] = [(entry["under_play"], entry["right"])
                                  for entry in row["under_entries"]]
    variants = [("all", None, None)] + [(f"span >= {length} s", length, None)
                                        for length in (60, 120, 180)] + [
        (f"{count} longest", None, count) for count in (8, 16)]
    totals = {name: {"seconds": 0.0, "right": 0, "wrong": 0, "entries": 0}
              for name, _, _ in variants}
    for recording in recordings:
        stem = Path(recording["recording"]).stem
        with open(Path(reports) / f"{stem}.json") as file:
            plays = json.load(file)["plays"]
        number = {(play["asset"], round(play["start_seconds"], 3)): index + 1
                  for index, play in enumerate(plays)}
        entries = recording["entries"]
        for entry in entries:
            entry["number"] = number.get((entry["taken_out"], round(entry.get("start", -1), 3)))
        longest = sorted(entries, key=lambda entry: -entry.get("span", 0.0))
        for name, length, count in variants:
            if length is not None:
                kept = [entry for entry in entries if entry.get("span", 0.0) >= length]
            elif count is not None:
                kept = longest[:count]
            else:
                kept = entries
            numbers = {entry["number"] for entry in kept}
            totals[name]["seconds"] += sum(entry["total"] for entry in kept)
            totals[name]["entries"] += len(kept)
            for play, right in gained.get(stem, []):
                if play in numbers:
                    totals[name]["right" if right else "wrong"] += 1
    print("| Variant | Entries searched under | Step time (s) | Gained entries right | Wrong |")
    print("|---|--:|--:|--:|--:|")
    for name, _, _ in variants:
        row = totals[name]
        print(f"| {name} | {row['entries']} | {row['seconds']:.1f} | {row['right']} | {row['wrong']} |")


TIME_FIELDS = {
    "wall": "Elapsed (wall clock) time (h:mm:ss or m:ss)",
    "user": "User time (seconds)",
    "system": "System time (seconds)",
    "rss_kb": "Maximum resident set size (kbytes)",
}
UNDER_LINE = re.compile(r"searched under (\d+) confident tracks in ([\d.]+) s")


def clock(text):
    total = 0.0
    for part in text.split(":"):
        total = 60 * total + float(part)
    return total


def timed(stderr):
    """`/usr/bin/time -v`'s figures and the step's own line (`--verbose`)."""
    with open(stderr) as file:
        lines = file.read().splitlines()
    found = {}
    for line in lines:
        for key, label in TIME_FIELDS.items():
            if line.strip().startswith(label + ":"):
                value = line.split(": ")[-1].strip()
                found[key] = clock(value) if key == "wall" else float(value)
        match = UNDER_LINE.search(line)
        if match:
            found["under_entries"] = int(match.group(1))
            found["under_seconds"] = float(match.group(2))
    return found


def without_under(report):
    report = json.loads(json.dumps(report))
    report.pop("under", None)
    report.get("search", {}).pop("under", None)
    return report


def cost(logs, reports, *names):
    """Each NAME's runs `<logs>/cost-NAME-{a1,b1,b2,a2}.err` (time -v) and
    `<reports>/NAME-{a1,b1,b2,a2}.json`: the means of the default's (a) and
    the option's (b) runs, the added time per confident entry searched
    under, per confident play and per hour of mix; whether the two runs of
    each kind gave the same JSON and the two kinds the same plays."""
    rows = []
    for name in names:
        runs = {run: timed(f"{logs}/cost-{name}-{run}.err") for run in ("a1", "b1", "b2", "a2")}
        loaded = {}
        for run in runs:
            with open(f"{reports}/{name}-{run}.json") as file:
                loaded[run] = json.load(file)
        mean = lambda kind, key: sum(runs[kind + n][key] for n in "12") / 2  # noqa: E731
        a, b = mean("a", "wall"), mean("b", "wall")
        cpu_a = mean("a", "user") + mean("a", "system")
        cpu_b = mean("b", "user") + mean("b", "system")
        report = loaded["b1"]
        confident = sum(play["confidence"] == "confident" for play in report["plays"])
        entries = runs["b1"].get("under_entries")
        hours = report["query"]["duration_seconds"] / 3600
        rows.append({
            "name": name, "wall_a": round(a, 2), "wall_b": round(b, 2),
            "added": round(b - a, 2), "ratio": round(b / a, 2),
            "cpu_a": round(cpu_a, 1), "cpu_b": round(cpu_b, 1),
            "rss_a_mb": round(max(runs[r]["rss_kb"] for r in ("a1", "a2")) / 1024),
            "rss_b_mb": round(max(runs[r]["rss_kb"] for r in ("b1", "b2")) / 1024),
            "step_seconds": [runs[r].get("under_seconds") for r in ("b1", "b2")],
            "entries": entries, "confident_plays": confident,
            "added_per_entry": round((b - a) / entries, 2) if entries else None,
            "added_per_play": round((b - a) / confident, 2) if confident else None,
            "added_per_hour": round((b - a) / hours, 1),
            "same_a": loaded["a1"] == loaded["a2"], "same_b": loaded["b1"] == loaded["b2"],
            "same_plays": without_under(loaded["b1"]) == loaded["a1"],
            "found_under": len(report.get("under") or []),
        })
    print(json.dumps(rows, ensure_ascii=False, indent=1))


if __name__ == "__main__":
    command, *rest = sys.argv[1:]
    {"exclude": exclude, "diff": diff, "manifest": manifest_score, "fates": fates,
     "cost": cost, "budget": budget}[command](*rest)
