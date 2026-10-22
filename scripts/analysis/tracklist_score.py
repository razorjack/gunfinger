"""Session 39: score the tracklist `identify` prints against the truth of
generated mixes (`mixes --kind drops`), as pre-registered in
docs/brief-39.md (step 1).

    # a store-only copy of a peak store without a sweep panel's held-out files
    python3 -I scripts/analysis/tracklist_score.py store PANEL STORE OUT_DIR
    # NAS scale: the NAS files of the held-out recordings, to leave out
    python3 -I scripts/analysis/tracklist_score.py exclude PANEL NAS_CLUSTERS OUT.txt
    # scores; CLI_DIR holds `<mix>.json` (identify) and `<mix>.txt` (show);
    # at NAS scale STORE is the NAS store and EXCLUDED the list above
    python3 -I scripts/analysis/tracklist_score.py score MIXES.json CLI_DIR \\
        CLUSTERS.json SHARED_PANEL.json STORE OUT.json [EXCLUDED.txt]
    python3 -I scripts/analysis/tracklist_score.py summary OUT.json...

The entries are rebuilt from each JSON report as `export::entries` builds
them (`same_recording`, `same_tagged_track`, chains joined; the tracklist
leaves out a possible entry whose plays all carry `shares_material_with`),
and every line `show` printed is checked against them: number, start,
name, other names and marks. A mismatch stops the script.
"""
import json
import os
import re
import shutil
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import tracklist_tags  # noqa: E402

SAME_PASSAGE_OVERLAP = 0.5
SAME_RECORDING_SECONDS = 5.0
# How the harness's clusters name the other library's files (`padding`).
NAS_PREFIX = "second-library/"


def store(panel, source, out):
    held_out = set(json.load(open(panel))["held_out"])
    os.makedirs(out, exist_ok=False)
    kept = 0
    for name in sorted(os.listdir(source)):
        path = os.path.join(source, name)
        if name.endswith(".tags"):
            asset = tracklist_tags.tags_of(path)[0]
            if asset in held_out:
                continue
            shutil.copy2(path, out)
            shutil.copy2(path[:-len(".tags")] + ".peaks", out)
            kept += 1
        elif name == "library.txt":
            shutil.copy2(path, out)
    print(f"{kept} records copied, {len(held_out)} held-out files left out")


def exclude(panel, nas_clusters, out):
    """The NAS files of every recording of the NAS clusters file that holds a
    held-out file of the sweep panel, for `identify --exclude-from`."""
    held_out = set(json.load(open(panel))["held_out"])
    excluded = sorted({member[len(NAS_PREFIX):]
                       for members in json.load(open(nas_clusters))["duplicates"]
                       if held_out & set(members)
                       for member in members if member.startswith(NAS_PREFIX)})
    open(out, "w").write("".join(path + "\n" for path in excluded))
    print(f"{len(excluded)} NAS files of held-out recordings")


# --- entries, as export.rs and report.rs build them -------------------------

def words(text):
    return [word.lower() for word in re.split(r"[^\w]|_", text or "") if word]


def compared(text):
    found = words(text)
    return " ".join(found) if found else None


def compared_name(play):
    tags = play.get("tags") or {}
    artist, title = compared(tags.get("artist")), compared(tags.get("title"))
    return (artist, title) if artist and title else None


def overlaps_most_of(a, b):
    overlap = min(a["end_seconds"], b["end_seconds"]) - max(a["start_seconds"], b["start_seconds"])
    shorter = min(a["end_seconds"] - a["start_seconds"], b["end_seconds"] - b["start_seconds"])
    return overlap >= SAME_PASSAGE_OVERLAP * shorter


def over_same_passage(a, b):
    return ("shares_material_with" not in a and "shares_material_with" not in b
            and overlaps_most_of(a, b))


def started(play):
    return play["start_seconds"] - play["track_start_seconds"] / play["speed"]


def one_entry(a, b):
    if not over_same_passage(a, b):
        return False
    lines_up = abs(started(a) - started(b)) <= SAME_RECORDING_SECONDS
    name = compared_name(a)
    return lines_up or (name is not None and name == compared_name(b))


def entries(plays):
    firsts = list(range(len(plays)))
    for later in range(len(plays)):
        for earlier in range(later):
            if one_entry(plays[later], plays[earlier]):
                kept = min(firsts[earlier], firsts[later])
                joined = max(firsts[earlier], firsts[later])
                firsts = [kept if first == joined else first for first in firsts]
    grouped = {}
    for index, first in enumerate(firsts):
        grouped.setdefault(first, []).append(plays[index])
    result = []
    for first in sorted(grouped):
        members = sorted(grouped[first],
                         key=lambda p: (p["confidence"] != "confident", -p["hits"]))
        result.append(members)
    result.sort(key=lambda members: min(p["start_seconds"] for p in members))
    return result


def printed(entry):
    return entry[0]["confidence"] == "confident" or any(
        "shares_material_with" not in play for play in entry)


def track_name(play_or_tags, asset):
    tags = play_or_tags or {}
    title = tags.get("title")
    if title is None:
        return os.path.splitext(os.path.basename(asset))[0]
    artist = tags.get("artist")
    if artist is not None and title.startswith(artist) and title[len(artist):].startswith(" - "):
        title = title[len(artist) + 3:]
    return f"{artist} - {title}" if artist is not None else title


def name_of(asset, plays):
    for play in plays:
        if play["asset"] == asset and play.get("tags") is not None:
            return track_name(play["tags"], asset)
        if asset in play.get("same_audio_tags", {}):
            return track_name(play["same_audio_tags"][asset], asset)
    raise SystemExit(f"no tags for {asset}: names from files are not checked here")


def other_names(entry, plays):
    seen = [name_of(entry[0]["asset"], plays).lower()]
    others = []
    assets = [play["asset"] for play in entry[1:]] + [
        asset for play in entry for asset, tags in play.get("same_audio_tags", {}).items()
        if (tags.get("title") or "").strip()]
    for asset in assets:
        full = name_of(asset, plays)
        if full.lower() not in seen:
            seen.append(full.lower())
            others.append(full)
    return others


def titles_disagree(entry):
    titles = []
    for play in entry:
        if play["confidence"] != "confident":
            continue
        for tags in [play.get("tags")] + list(play.get("same_audio_tags", {}).values()):
            if tags is not None and compared(tags.get("title")):
                titles.append(compared(tags.get("title")))
    return any(play.get("other_titles") for play in entry) or len(set(titles)) > 1


def timecode_seconds(text):
    seconds = 0
    for part in text.split(":"):
        seconds = 60 * seconds + int(part)
    return seconds


LINE = re.compile(r"^ *(\d+)\. +(\S+)  (.*?)(?:  \((also|titles disagree): (.*)\))?( \(possible\))?$")


def check_lines(shown, report_entries, plays, label):
    lines = [line for line in shown.splitlines() if line]
    if lines == ["no tracks found"]:
        lines = []
    if len(lines) != len(report_entries):
        raise SystemExit(f"{label}: {len(lines)} lines printed, {len(report_entries)} entries rebuilt")
    for number, (line, entry) in enumerate(zip(lines, report_entries), 1):
        match = LINE.match(line)
        if not match:
            raise SystemExit(f"{label}: cannot read line {line!r}")
        start = min(play["start_seconds"] for play in entry)
        others = other_names(entry, plays)
        flag = ("titles disagree" if titles_disagree(entry) else "also") if others else None
        expected = (number, int(start), name_of(entry[0]["asset"], plays), flag,
                    ", ".join(others) if others else None,
                    entry[0]["confidence"] != "confident")
        found = (int(match.group(1)), timecode_seconds(match.group(2)), match.group(3),
                 match.group(4), match.group(5), match.group(6) is not None)
        if expected != found:
            raise SystemExit(f"{label}: line {line!r} does not match {expected}")


# --- truth ---------------------------------------------------------------------

class Library:
    """Recordings, families and indexed files. At NAS scale (`excluded`
    given) the reports name NAS files, which the clusters and the panel name
    with `NAS_PREFIX`; the indexed files are the store's less the excluded."""

    def __init__(self, clusters_path, panel_path, store_copy, excluded=None):
        self.prefix = NAS_PREFIX if excluded else ""
        self.cluster = {}
        for members in json.load(open(clusters_path))["duplicates"]:
            for path in members:
                self.cluster[path] = tuple(members)
        self.family = {}
        for family in json.load(open(panel_path))["families"]:
            for recording in family["recordings"]:
                for path in recording:
                    self.family[path] = family["name"]
        self.indexed = set()
        self.names = {}
        for name in os.listdir(store_copy):
            if name.endswith(".tags"):
                asset, artist, title = tracklist_tags.tags_of(os.path.join(store_copy, name))
                self.indexed.add(self.prefix + asset)
                self.names[asset] = (compared(artist), compared(title))

        if excluded:
            self.indexed -= {self.prefix + line.strip() for line in open(excluded) if line.strip()}

    def key(self, asset):
        """A file of a report as the clusters name it."""
        return self.prefix + asset

    def recording(self, asset):
        return self.cluster.get(asset, (asset,))

    def family_of(self, asset):
        return self.family.get(asset, self.recording(asset))

    def indexed_files(self, recording):
        return [path for path in recording if path in self.indexed]


def overlap(a, b):
    return a["start_seconds"] < b["end_seconds"] and b["start_seconds"] < a["end_seconds"]


def files(play):
    return [play["asset"]] + play.get("same_audio", [])


def holds(play, recording):
    return any(path in recording for path in files(play))


def score_mix(mix, report, shown, library):
    plays = report["plays"]
    all_entries = entries(plays)
    shown_entries = [entry for entry in all_entries if printed(entry)]
    check_lines(shown, shown_entries, plays, mix["name"])
    for play in plays:
        play["asset"] = library.key(play["asset"])
        play["same_audio"] = [library.key(path) for path in play.get("same_audio", [])]
    truth = mix["plays"]
    for play in truth:
        play["recording"] = library.recording(play["asset"])
        play["family"] = library.family_of(play["asset"])
    entry_of = {}
    for number, entry in enumerate(all_entries):
        for play in entry:
            entry_of[id(play)] = number
    shown_numbers = {id(entry): True for entry in shown_entries}

    def named(entry):
        return library.recording(entry[0]["asset"])

    truth_rows = []
    for index, play in enumerate(truth):
        if play["held_out"]:
            continue
        recording, family = play["recording"], play["family"]
        own = [found for found in plays if holds(found, recording) and overlap(found, play)]
        confident = any(found["confidence"] == "confident" for found in own)
        naming = [entry for entry in shown_entries if named(entry) == recording and any(
            holds(found, recording) and overlap(found, play) for found in entry)]
        family_naming = [entry for entry in shown_entries
                         if library.family_of(entry[0]["asset"]) == family and any(
                             library.family_of(found["asset"]) == family and overlap(found, play)
                             for found in entry)]
        if naming:
            outcome = "printed " + ("confident" if naming[0][0]["confidence"] == "confident"
                                    else "possible")
        elif any("shares_material_with" in found for found in own):
            outcome = "left out"
        elif any(id(all_entries[entry_of[id(found)]]) in shown_numbers for found in own):
            outcome = "under another entry"
        elif own:
            outcome = "found, not printed"
        else:
            outcome = "not found"
        truth_rows.append({
            "play": index,
            "scenario": play.get("scenario"),
            "side": play.get("side"),
            "inside": inside_of(play, truth),
            "seconds": play["end_seconds"] - play["start_seconds"],
            "gain_db": play.get("gain_db", 0.0),
            "asset": play["asset"],
            "today_confident": confident,
            "in_json": [found["confidence"] + (" marked" if "shares_material_with" in found else "")
                        for found in own],
            "outcome": outcome,
            "entries_naming": len(naming),
            "family_printed": bool(family_naming),
        })

    entry_rows = []
    for entry in shown_entries:
        recording = named(entry)
        of_named = [found for found in entry if holds(found, recording)]
        right = [play for play in truth if not play["held_out"] and play["recording"] == recording
                 and any(overlap(found, play) for found in of_named)]
        row = {"start_seconds": min(found["start_seconds"] for found in entry),
               "confidence": entry[0]["confidence"], "name": recording[0], "right": bool(right)}
        if not right:
            related = any(play["family"] == library.family_of(entry[0]["asset"])
                          and any(overlap(found, play) for found in entry) for play in truth)
            row["extra"] = "related" if related else "unrelated"
        recordings = {}
        for found in entry:
            for play in truth:
                if not play["held_out"] and holds(found, play["recording"]) and overlap(found, play):
                    recordings[play["recording"]] = play["family"]
        if len(recordings) > 1:
            row["merged"] = "within family" if len(set(map(str, recordings.values()))) == 1 \
                else "across families"
            row["merged_scenarios"] = sorted({play.get("scenario") for play in truth
                                             if play["recording"] in recordings
                                             and any(overlap(found, play) for found in entry)})
        if right:
            held = {path for found in entry for path in files(found)}
            missing = [path for path in library.indexed_files(recording) if path not in held]
            elsewhere = [path for path in missing if any(
                path in files(found) and overlap(found, {"start_seconds": row["start_seconds"],
                                                         "end_seconds": max(f["end_seconds"] for f in entry)})
                for found in plays)]
            row["indexed_files"] = len(library.indexed_files(recording))
            row["files_missing"] = len(missing)
            row["files_missing_found_elsewhere"] = len(elsewhere)
        names = {}
        for found in entry:
            for path, tags in [(found["asset"], found.get("tags"))] + list(
                    found.get("same_audio_tags", {}).items()):
                name = compared_name({"tags": tags})
                if name:
                    names.setdefault(name, set()).add(found["confidence"])
        if len(names) > 1:
            row["titles"] = len(names)
            row["titles_flagged"] = titles_disagree(entry)
            row["other_title_levels"] = sorted({level for name, levels in names.items()
                                               if name != compared_name(entry[0])
                                               for level in levels})
        entry_rows.append(row)

    wrong_confident = [found["asset"] for found in plays if found["confidence"] == "confident"
                       and not any(holds(found, play["recording"]) and overlap(found, play)
                                   for play in truth if not play["held_out"])]
    return {"mix": mix["name"], "truth": truth_rows, "entries": entry_rows,
            "wrong_confident": wrong_confident,
            "entries_printed": len(shown_entries)}


def inside_of(play, truth):
    if play.get("side") != "second":
        return None
    first = [other for other in truth if other.get("scenario") == play.get("scenario")
             and other.get("side") == "first" and overlap(other, play)]
    return bool(first) and play["end_seconds"] < first[0]["end_seconds"]


def score(mixes_path, cli_dir, clusters, panel, store_copy, out, excluded=None):
    library = Library(clusters, panel, store_copy, excluded)
    report = json.load(open(mixes_path))
    rows = []
    for result in report["mixes"]:
        mix = result["mix"]
        cli = json.load(open(os.path.join(cli_dir, mix["name"] + ".json")))
        shown = open(os.path.join(cli_dir, mix["name"] + ".txt")).read()
        rows.append(score_mix(mix, cli, shown, library))
    json.dump({"mixes": mixes_path, "rows": rows}, open(out, "w"), indent=1)
    print(f"{len(rows)} mixes scored, every printed line matches its rebuilt entry")


def summary(*paths):
    rows = [row for path in paths for row in json.load(open(path))["rows"]]
    truth = [play for row in rows for play in row["truth"]]

    def group(play):
        scenario = play["scenario"] or "-"
        if play["side"] == "second":
            return f"{scenario} second ({'inside' if play['inside'] else 'through'})"
        return f"{scenario} {play['side']}" if play["side"] else scenario

    outcomes = ["printed confident", "printed possible", "left out", "under another entry",
                "found, not printed", "not found"]
    print("| Scenario | Plays | Today confident | " + " | ".join(o.capitalize() for o in outcomes)
          + " | Family printed | Split |")
    print("|---|" + "--:|" * (len(outcomes) + 4))
    for name in sorted({group(play) for play in truth}):
        here = [play for play in truth if group(play) == name]
        counts = [sum(play["outcome"] == outcome for play in here) for outcome in outcomes]
        print(f"| {name} | {len(here)} | {sum(p['today_confident'] for p in here)} | "
              + " | ".join(map(str, counts))
              + f" | {sum(p['family_printed'] for p in here)} | "
              f"{sum(p['entries_naming'] > 1 for p in here)} |")
    entries = [entry for row in rows for entry in row["entries"]]
    count = lambda test: sum(1 for entry in entries if test(entry))  # noqa: E731
    print(f"\nentries printed {len(entries)}: right {count(lambda e: e['right'])}; extra confident "
          f"related {count(lambda e: e.get('extra') == 'related' and e['confidence'] == 'confident')}, "
          f"unrelated {count(lambda e: e.get('extra') == 'unrelated' and e['confidence'] == 'confident')}; "
          f"extra possible related {count(lambda e: e.get('extra') == 'related' and e['confidence'] != 'confident')}, "
          f"unrelated {count(lambda e: e.get('extra') == 'unrelated' and e['confidence'] != 'confident')}")
    merged = [entry for entry in entries if "merged" in entry]
    print(f"merged entries: {len(merged)}; within a family "
          f"{sum(e['merged'] == 'within family' for e in merged)}, across families "
          f"{sum(e['merged'] == 'across families' for e in merged)}; scenarios "
          f"{sorted({tuple(e['merged_scenarios']) for e in merged})}")
    multi = [entry for entry in entries if entry.get("indexed_files", 0) >= 2]
    print(f"right entries of recordings with 2+ indexed files: {len(multi)}; missing a file "
          f"{sum(e['files_missing'] > 0 for e in multi)} (a play of it elsewhere over the entry "
          f"{sum(e['files_missing_found_elsewhere'] > 0 for e in multi)})")
    titled = [entry for entry in entries if "titles" in entry]
    print(f"entries whose files carry 2+ names: {len(titled)}; flagged "
          f"{sum(e['titles_flagged'] for e in titled)}; not flagged with a confident other title "
          f"{sum(not e['titles_flagged'] and 'confident' in e['other_title_levels'] for e in titled)}, "
          f"with possible ones only "
          f"{sum(not e['titles_flagged'] and e['other_title_levels'] == ['possible'] for e in titled)}")
    print(f"wrong confident plays: {sum(len(row['wrong_confident']) for row in rows)}; "
          f"mixes {len(rows)}")


if __name__ == "__main__":
    command, *arguments = sys.argv[1:]
    {"store": store, "exclude": exclude, "score": score, "summary": summary}[command](*arguments)
