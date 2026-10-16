"""Session 23, step 5: the ambiguity note (docs/brief-23.md, "Step 5's
pre-registration"), which is session 21's (c): a confident play is
flagged when each of its confident segments has a file outside its own
recording (the duplicate cluster and the panel recording that holds it)
holding the segment's span of the file with 60 hits or more.

usage:
  ambiguity_note.py lookups-shared SHARED_REPORT OUT
      `gunfinger-eval holders` input for every confident detection,
      except the blend partner's, of the `played left out` and `source
      left out` indexes of a `shared` report (each with its query's
      leave-out).
  ambiguity_note.py lookups-identify LIBRARY_MAP OUT SET=REPORT...
      the same for every confident segment of the confident plays of
      `identify -f json` reports, in the index `identify` searched (the
      corpus files without a NAS copy left out). A NAS file that a
      corpus file stands for is looked up as that corpus file.
  ambiguity_note.py panel PANEL CLUSTERS SHARED_REPORT HOLDERS OUT
      the panel's queries by session 20's oracle under CLUSTERS
      (shared_outcomes.py --clusters), and each confident play of the
      main case and the positive control with the note's value.
  ambiguity_note.py tracklists PANEL CLUSTERS LIBRARY_MAP TAGS HOLDERS OUT SET=REPORT=TRACKLIST...
      the default tracklist `identify` printed for each report
      (TRACKLIST), each entry with the note it would carry (entries
      rebuilt as tracklist_view.py does and matched by number and start),
      and each flagged confident play with the file it names.

TAGS is store_tags.py's output for the NAS store. Nothing here reads
audio or the held-out test set.
"""

import json
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import devscore  # noqa: E402
import shared_outcomes  # noqa: E402
import tracklist_view  # noqa: E402

PREFIX = "second-library/"
MARKED_HITS = 60
GAP_SECONDS = 90.0
SHOWN = 3


def load(path):
    return devscore.load_json(path)


def dump(value, path):
    with open(path, "w", encoding="utf-8") as file:
        json.dump(value, file, indent=1, ensure_ascii=False)


def own_index(clusters_path, panel):
    """A file's own recording, as `shared-mixes` takes it for (c)."""
    cluster_of = devscore.clusters_of(clusters_path)
    recordings = [set(r) for family in panel["families"] for r in family["recordings"]]

    def own_of(path):
        own = set(cluster_of(path))
        for recording in recordings:
            if path in recording:
                own |= recording
        return own
    return own_of


def key(name, asset, start, end):
    return f"{name}|{asset}|{start:.4f}|{end:.4f}"


def lookups_shared(report_path, out_path):
    report = load(report_path)
    lookups = []
    for result in report["results"]:
        if result["index"] == "everything" or result.get("error"):
            continue
        for d in result["detections"]:
            if d["level"] != "confident" or d["role"] == "partner":
                continue
            lookups.append({
                "name": key(f"{result['query']}|{result['index']}", d["asset"],
                            d["track_start_seconds"], d["track_end_seconds"]),
                "asset": d["asset"],
                "track_start_seconds": d["track_start_seconds"],
                "track_end_seconds": d["track_end_seconds"],
                "left_out": result["left_out"],
            })
    dump(lookups, out_path)
    print(f"{len(lookups)} lookups, {len({tuple(l['left_out']) for l in lookups})} leave-outs")


def harness_names(map_path):
    """From a NAS path to the name the harness's index gives the file."""
    copies = {c["stands_for"]: c["corpus"] for c in load(map_path)["copies"]}
    return lambda path: copies.get(path, PREFIX + path)


def set_reports(pairs):
    out = []
    for pair in pairs:
        name, path = pair.split("=")[:2]
        out.append((name, load(path)))
    return out


def printed_lines(pairs):
    """Each set's tracklist as `identify` printed it, by entry number."""
    out = {}
    for pair in pairs:
        parts = pair.split("=")
        lines = {}
        if len(parts) > 2:
            with open(parts[2], encoding="utf-8") as file:
                for line in file:
                    head = line.split(".", 1)[0].strip()
                    if head.isdigit():
                        lines[int(head)] = line.rstrip("\n")
        out[parts[0]] = lines
    return out


def lookups_identify(map_path, out_path, *pairs):
    named = harness_names(map_path)
    # `identify` indexed the NAS store alone: the corpus files without a
    # copy there are left out, so `holders` searches the same files.
    without_copy = sorted(load(map_path)["without_copy"])
    lookups = []
    for set_name, report in set_reports(pairs):
        for play in report["plays"]:
            if play["confidence"] != "confident":
                continue
            for segment in play["segments"]:
                if segment["confidence"] != "confident":
                    continue
                asset = named(play["asset"])
                lookups.append({
                    "name": key(set_name, asset, segment["track_start_seconds"],
                                segment["track_end_seconds"]),
                    "asset": asset,
                    "track_start_seconds": segment["track_start_seconds"],
                    "track_end_seconds": segment["track_end_seconds"],
                    "left_out": without_copy,
                })
    dump(lookups, out_path)
    print(f"{len(lookups)} lookups")


def holders_of(path):
    return {looked["name"]: looked for looked in load(path)}


def strongest_outside(looked, own):
    """(c)'s value for one segment: hits and file of the strongest holder
    outside `own`, and every such holder at the mark."""
    if looked is None or looked.get("error"):
        return None, None, []
    outside = [h for h in looked["holders"] if h["asset"] not in own]
    marking = [h for h in outside if h["hits"] >= MARKED_HITS]
    if not outside:
        return 0, None, []
    best = max(outside, key=lambda h: h["hits"])
    return best["hits"], best["asset"], marking


def plays_of(detections):
    """One file's detections joined across gaps of at most 90 s."""
    by_asset = defaultdict(list)
    for d in detections:
        by_asset[d["asset"]].append(d)
    plays = []
    for asset, found in by_asset.items():
        found.sort(key=lambda d: d["start_seconds"])
        current = None
        for d in found:
            if current and d["start_seconds"] - current["end"] <= GAP_SECONDS:
                current["segments"].append(d)
                current["end"] = max(current["end"], d["end_seconds"])
            else:
                current = {"asset": asset, "start": d["start_seconds"],
                           "end": d["end_seconds"], "segments": [d]}
                plays.append(current)
    return plays


def play_note(segments, lookup_name, holders, own):
    """The play's value (the lowest over its confident segments), the file
    holding that segment, the holders marking every segment, and whether
    a lookup is missing."""
    values = []
    for segment in segments:
        looked = holders.get(lookup_name(segment))
        values.append(strongest_outside(looked, own))
    if any(v[0] is None for v in values):
        return {"value": None, "flagged": None, "holder": None, "names": []}
    value, holder, _ = min(values, key=lambda v: v[0])
    flagged = value >= MARKED_HITS
    names = {}
    if flagged:
        for _, _, marking in values:
            for h in marking:
                names[h["asset"]] = max(names.get(h["asset"], 0), h["hits"])
    return {"value": value, "flagged": flagged, "holder": holder,
            "names": sorted(names.items(), key=lambda item: -item[1])}


def panel(panel_path, clusters_path, report_path, holders_path, out_path):
    panel_ = load(panel_path)
    report = load(report_path)
    holders = holders_of(holders_path)
    own_of = own_index(clusters_path, panel_)
    recordings = shared_outcomes.expanded(report, shared_outcomes.clusters_of(clusters_path))
    roles = shared_outcomes.roles_of(panel_)
    rows = []
    for result in report["results"]:
        if result["index"] == "everything":
            continue
        family, versions, unjudged, control = roles[result["scenario"]]
        outcome, _, _, _ = shared_outcomes.outcome(
            result, versions, unjudged, recordings.get(result["scenario"]))
        # Plays as defined: one file's detections at least possible,
        # joined across gaps of at most 90 s; the note takes the confident ones.
        found = [dict(d, role=shared_outcomes.role(
            dict(d, role=shared_outcomes.reroled(d, recordings.get(result["scenario"]))),
            versions, unjudged)) for d in result["detections"]
            if d["level"] != "weak" and d["role"] != "partner"]
        plays = []
        for play in plays_of(found):
            confident = [d for d in play["segments"] if d["level"] == "confident"]
            if not confident:
                continue
            name = lambda d, q=result: key(f"{q['query']}|{q['index']}", d["asset"],
                                           d["track_start_seconds"], d["track_end_seconds"])
            note = play_note(confident, name, holders, own_of(play["asset"]))
            plays.append({"asset": play["asset"], "role": confident[0]["role"],
                          "hits": max(d["hits"] for d in confident), **note})
        rows.append({"direction": result["scenario"], "family": family, "control": control,
                     "query": result["query"], "kind": result["kind"], "index": result["index"],
                     "outcome": outcome, "plays": plays})
    dump({"rows": rows}, out_path)
    main = [r for r in rows if r["index"] == "played left out" and not r["control"]]
    wrong_queries = [r for r in main if r["outcome"] == "wrong confident"]
    wrong_plays = [p for r in wrong_queries for p in r["plays"] if p["role"] != "played"]
    print(f"main case: {len(main)} queries, {len(wrong_queries)} wrong confident")
    print(f"  wrong plays {len(wrong_plays)}: flagged {sum(bool(p['flagged']) for p in wrong_plays)}, "
          f"lookup missing {sum(p['flagged'] is None for p in wrong_plays)}")
    every = sum(all(p["flagged"] for p in r["plays"] if p["role"] != "played") for r in wrong_queries)
    some = sum(any(p["flagged"] for p in r["plays"] if p["role"] != "played") for r in wrong_queries)
    print(f"  wrong queries with every wrong play flagged {every}, with one at least {some}")
    by_family = defaultdict(Counter)
    for r in wrong_queries:
        for p in r["plays"]:
            if p["role"] != "played":
                by_family[r["family"]]["flagged" if p["flagged"] else "not flagged"] += 1
    for family, counts in sorted(by_family.items()):
        print(f"    {family}: {dict(counts)}")
    right = [p for r in main for p in r["plays"] if p["role"] == "played"]
    print(f"  right confident plays in the main case (rips the index kept): {len(right)}, "
          f"flagged {sum(bool(p['flagged']) for p in right)}")
    control = [p for r in rows if r["index"] == "source left out" and not r["control"]
               for p in r["plays"] if p["role"] == "played"]
    print(f"positive control: {len(control)} right confident plays, "
          f"flagged {sum(bool(p['flagged']) for p in control)}")


def tag_names(tags_path, map_path):
    tags = load(tags_path)
    corpus = {c["corpus"]: c["stands_for"] for c in load(map_path)["copies"]}

    def name_of(asset):
        path = asset[len(PREFIX):] if asset.startswith(PREFIX) else corpus.get(asset)
        return tracklist_view.name(tags.get(path) if path else None, asset)
    return name_of


def note_text(names, name_of):
    shown = []
    for asset, _ in names:
        name = name_of(asset)
        if name not in shown:
            shown.append(name)
    text = "; ".join(shown[:SHOWN])
    if len(shown) > SHOWN:
        text += f" and {len(shown) - SHOWN} more"
    return f"(shares this passage with: {text})"


def tracklists(panel_path, clusters_path, map_path, tags_path, holders_path, out_path, *pairs):
    panel_ = load(panel_path)
    holders = holders_of(holders_path)
    own_of = own_index(clusters_path, panel_)
    named = harness_names(map_path)
    name_of = tag_names(tags_path, map_path)
    cluster_of = devscore.clusters_of(clusters_path)
    printed = printed_lines(pairs)
    out = {}
    for set_name, report in set_reports(pairs):
        tracks = devscore.load_set(set_name, nas=True)
        slots = devscore.slots(tracks, report["query"]["duration_seconds"], cluster_of)
        plays = report["plays"]
        tracklist_view.mark_shared(plays)
        for play in plays:
            play["files"] = [PREFIX + a for a in [play["asset"], *play.get("same_audio", [])]]
            if play["confidence"] == "confident":
                asset = named(play["asset"])
                confident = [s for s in play["segments"] if s["confidence"] == "confident"]
                name = lambda s, a=asset, n=set_name: key(n, a, s["track_start_seconds"],
                                                            s["track_end_seconds"])
                play["note"] = play_note(confident, name, holders, own_of(asset))
            else:
                play["note"] = None
        lines, flagged_plays = [], []
        number = 0
        for group in tracklist_view.entries(plays):
            first = group[0]
            if first["shares"] is not None:
                continue
            number += 1
            start = min(p["start_seconds"] for p in group)
            right = sorted({s["track"]["label"].strip() for s in slots for p in group
                            for f in p["files"]
                            if devscore.covers(s, f, p["start_seconds"], p["end_seconds"])})
            others = []
            for play in group[1:]:
                n = tracklist_view.name(play.get("tags"), play["asset"])
                if n != tracklist_view.name(first.get("tags"), first["asset"]) and n not in others:
                    others.append(n)
            noted = [p for p in group if p["note"] and p["note"]["flagged"]]
            names = []
            for play in noted:
                names += play["note"]["names"]
                flagged_plays.append({
                    "entry": number, "asset": play["asset"], "hits": play["hits"],
                    "start": play["start_seconds"], "end": play["end_seconds"],
                    "value": play["note"]["value"], "holder": play["note"]["holder"],
                    "names": play["note"]["names"], "right": right,
                })
            names.sort(key=lambda item: -item[1])
            rebuilt = (f"{number:>2}. {tracklist_view.clock(start)} "
                       f"{tracklist_view.name(first.get('tags'), first['asset'])}"
                       + (f" ({first['confidence']})" if first["confidence"] != "confident" else "")
                       + (f" (also: {'; '.join(others)})" if others else ""))
            line = printed[set_name].get(number)
            matches = line is not None and tracklist_view.clock(start) in line
            line = (line if matches else rebuilt + "   [rebuilt; the printed line differs]")
            line += ("  " + note_text(names, name_of) if names else "")
            lines.append({"line": line, "right": right, "noted": bool(names), "printed": matches,
                          "missing": any(p["note"] and p["note"]["flagged"] is None for p in group)})
        out[set_name] = {"lines": lines, "flagged": flagged_plays}
        confident_plays = [p for p in plays if p["confidence"] == "confident"]
        if len(printed[set_name]) not in (0, len(lines)):
            print(f"  {set_name}: {len(printed[set_name])} printed entries, {len(lines)} rebuilt")
        print(f"== {set_name}: {len(lines)} entries, {sum(l['noted'] for l in lines)} noted; "
              f"{len(confident_plays)} confident plays, "
              f"{sum(bool(p['note']['flagged']) for p in confident_plays)} flagged, "
              f"{sum(p['note']['flagged'] is None for p in confident_plays)} without a lookup")
        for entry in lines:
            print("  " + entry["line"] + ("" if entry["right"] else "   [no right file]"))
    dump(out, out_path)


if __name__ == "__main__":
    command, *arguments = sys.argv[1:]
    {"lookups-shared": lookups_shared, "lookups-identify": lookups_identify,
     "panel": panel, "tracklists": tracklists}[command](*arguments)
