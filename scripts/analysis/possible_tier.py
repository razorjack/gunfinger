"""Experiment B of session 15: the possible tier at NAS scale, from stored
development reports. Measurement only.

For each threshold (60, 80, 100, 130 hits), the plays a threshold would
show as possible (not confident, strongest segment at the threshold or
more), per hour of audio, labelled:

- right: the recording playing there (the scan's slot accepts the file,
  or the sweep or recall query's cluster holds it);
- related: another version over the played track, shown by audio (a
  `different` verdict between the two clusters, or a pair of 30 hits or
  more between them in the clusters report) or, failing that, by tags
  (`tag_labels.label`: another rip by name, or a version of the title);
- unrelated: neither, so chance.

Plays (and query detections) that differ only in their file are counted
once, as `identify` shows them (`plays::merge_same_audio`): two copies of
one file have the same segments. Query reports keep no spans, so there
equal hits, windows, speed and confidence within one query stand for
equal segments.

Also the right plays each threshold would hide, and the strongest
unrelated candidate of any level below the confident rule.

usage: python3 -I scripts/analysis/possible_tier.py TAGS OUT.json

TAGS is the NAS store's tags (work/s11/tags.json, as `store_tags.py`
wrote them). Reads only the development reports named below; any path
naming the held-out test set is refused.
"""

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import devscore  # noqa: E402
import tag_labels  # noqa: E402

THRESHOLDS = [60, 80, 100, 130]
PAIR_HITS = 30
NAS = devscore.ROOT / "work" / "reports" / "library-nas-dnb-peaks"
SCANS = ["scan-stakka-skynet-knowledge.json", "scan-razorjack-2003-03-29.json"]
LEAVE_OUTS = [
    "scan-stakka-skynet-knowledge-leave-out-3-seed-2026.json",
    "scan-stakka-skynet-knowledge-leave-out-11-seed-2026.json",
    "scan-razorjack-2003-03-29-leave-out-3-seed-2026.json",
]
QUERIES = [f"sweep-seed-{seed}.json" for seed in (2026, 2027, 2028, 2029)] + ["recall-seed-2026.json"]
# Session 18: the third development mix, counted apart and with the two above.
NEW_SCANS = ["scan-nocny-trans-vertical-2002-08-23.json"]
NEW_LEAVE_OUTS = [f"scan-nocny-trans-vertical-2002-08-23-leave-out-3-seed-{seed}.json" for seed in (2026, 2027, 2028)]
EXCERPT_SECONDS = 30.0


class Relations:
    """Which two files are other versions of one another: by audio (verdicts,
    measured pairs between clusters) or by tags."""

    def __init__(self, clusters_file, verdicts_file, tags_file, map_file):
        clusters = devscore.load_json(clusters_file)
        self.cluster = {}
        for number, members in enumerate(clusters["duplicates"]):
            for member in members:
                self.cluster[member] = number
        self.by_audio = set()
        for pair in clusters["pairs"]:
            if pair["hits"] >= PAIR_HITS:
                self.by_audio.add(self.key(pair["query"], pair["found"]))
        with open(verdicts_file) as file:
            for line in file:
                if line.startswith("#") or not line.strip():
                    continue
                verdict, a, b = line.rstrip("\n").split("\t")
                if verdict == "different":
                    self.by_audio.add(self.key(a, b))
        self.tags = devscore.load_json(tags_file)
        self.stands_for = {c["corpus"]: c["stands_for"] for c in devscore.load_json(map_file)["copies"]}

    def of(self, path):
        return self.cluster.get(path, path)

    def key(self, a, b):
        return frozenset((self.of(a), self.of(b)))

    def label(self, played, found):
        if self.of(played) == self.of(found):
            return "right"
        if self.key(played, found) in self.by_audio:
            return "related (audio)"
        by_tags = tag_labels.label(played, found, self.tags, self.stands_for)
        if by_tags in ("same name", "version"):
            return "related (tags)"
        return "unrelated"


def play_hits(play):
    return max(segment["hits"] for segment in play["segments"])


def without_asset(entry):
    return json.dumps({key: value for key, value in entry.items() if key != "asset"}, sort_keys=True)


def same_audio(entries, key):
    """`entries` grouped as `merge_same_audio` groups plays: equal `key`,
    first entry first."""
    groups = {}
    for entry in entries:
        groups.setdefault(key(entry), []).append(entry)
    return list(groups.values())


def play_key(play):
    return tuple(without_asset(segment) for segment in play["segments"])


def scan_rows(names, relations, cluster_of):
    rows = []
    for name in names:
        report = devscore.load_json(NAS / name)
        tracks = devscore.load_set(report["set"], nas=True)
        slots = devscore.slots(tracks, report["duration_seconds"], cluster_of)
        possible = [play for play in report["plays"] if devscore.play_confidence(play) != "confident"]
        for group in same_audio(possible, play_key):
            play = group[0]
            middle = (play["start_seconds"] + play["end_seconds"]) / 2
            playing = playing_slots(tracks, report["duration_seconds"], play)
            label, against = "unrelated", None
            for member in group:
                if devscore.matching_slots(slots, member["asset"], play["start_seconds"], play["end_seconds"]):
                    label, against = "right", None
                    break
                for track in playing:
                    for reference in track["references"]:
                        found = relations.label(reference, member["asset"])
                        if rank(found) > rank(label):
                            label, against = found, reference
            rows.append({"report": name, "asset": play["asset"], "also": [m["asset"] for m in group[1:]],
                         "hits": play_hits(play),
                         "span": [round(play["start_seconds"], 1), round(play["end_seconds"], 1)],
                         "middle": round(middle, 1), "label": label, "against": against,
                         "playing": [track["label"] for track in playing]})
    return rows


def scan_detections(names, relations, cluster_of):
    """Every detection of the scans below the confident rule that no slot
    accepts, labelled against the tracks playing over it: the measured
    null, at every level (0052's "strongest unrelated, development scan")."""
    rows = []
    for name in names:
        report = devscore.load_json(NAS / name)
        tracks = devscore.load_set(report["set"], nas=True)
        slots = devscore.slots(tracks, report["duration_seconds"], cluster_of)
        weak = [detection for detection in report["detections"] if not detection["confident"]]
        for group in same_audio(weak, without_asset):
            detection = group[0]
            if any(
                devscore.matching_slots(slots, member["asset"], detection["start_seconds"], detection["end_seconds"])
                for member in group
            ):
                continue
            label = "unrelated"
            for track in playing_slots(tracks, report["duration_seconds"], detection):
                for reference in track["references"]:
                    for member in group:
                        found = relations.label(reference, member["asset"])
                        if rank(found) > rank(label):
                            label = found
            rows.append({"report": name, "asset": detection["asset"], "hits": detection["hits"],
                         "windows": detection["windows"],
                         "span": [round(detection["start_seconds"], 1), round(detection["end_seconds"], 1)],
                         "label": label})
    return rows


def weak_true_262():
    """True plays found only as possible at 262 tracks, by their hits: the
    robust excerpts (transformations), the window grid's brief plays and
    the generated mixes' plays of 20-60 s. Hits of a true play come from
    the second pass, which counts every posting list, so library size
    changes whether it is found more than its hits."""
    reports = devscore.ROOT / "work" / "reports"
    result = {}
    for name, path in (
        ("robust", reports / "variant-second-pass-skip-0.01" / "robust-seed-2026.json"),
        ("robust, combined damage (current links)", reports / "verified" / "robust-seed-2026.json"),
    ):
        queries = [q for q in devscore.load_json(path)["queries"] if not q["held_out"]]
        hits = sorted(q["best_hits"] for q in queries if not q["confident"] and q["possible"])
        result[name] = {"indexed queries": len(queries), "confident": sum(q["confident"] for q in queries),
                        "found only as possible": len(hits),
                        "below": {t: sum(h < t for h in hits) for t in THRESHOLDS}}
    grid = devscore.load_json(reports / "grid-seed-2026.json")
    hits = sorted(max(m["hits"] for m in q["matching"]) for q in grid["queries"] if q["level"] == "possible")
    result["grid (brief plays of 10-20 s)"] = {"queries": len(grid["queries"]), "found only as possible": len(hits),
                                               "weakest": hits[0] if hits else None,
                                               "below": {t: sum(h < t for h in hits) for t in THRESHOLDS}}
    mixes = devscore.load_json(reports / "mixes-seed-2026.json")
    plays = [p for mix in mixes["mixes"] for p in mix["score"]["plays"] if not p["truth"]["held_out"]]
    hits = sorted(p["best"]["hits"] for p in plays if p["level"] == "possible")
    result["generated mixes"] = {"indexed plays": len(plays), "found only as possible": len(hits),
                                 "weakest": hits[0] if hits else None,
                                 "below": {t: sum(h < t for h in hits) for t in THRESHOLDS}}
    return result


def rank(label):
    return {"unrelated": 0, "related (tags)": 1, "related (audio)": 2, "right": 3}[label]


def playing_slots(tracks, duration, play):
    """The tracks whose own time (start to the next start, no tolerance)
    overlaps the play or detection."""
    playing = []
    for position, track in enumerate(tracks):
        end = tracks[position + 1]["start"] if position + 1 < len(tracks) else duration
        if track["start"] < play["end_seconds"] and play["start_seconds"] < end:
            playing.append(track)
    return playing


def query_rows(relations, cluster_of):
    """Each detection below the confident rule, and each indexed query's
    strongest right detection when no right detection is confident (found
    only as possible: `only_possible`)."""
    rows = []
    for name in QUERIES:
        report = devscore.load_json(NAS / name)
        for query in report["queries"]:
            recording = cluster_of(query["asset"])
            right = [d for d in query["detections"] if d["asset"] in recording]
            only = None
            if not query["held_out"] and right and not any(d["confident"] for d in right):
                only = max(right, key=lambda d: d["hits"])
            weak = [detection for detection in query["detections"] if not detection["confident"]]
            for group in same_audio(weak, without_asset):
                labels = [
                    "right" if member["asset"] in recording else relations.label(query["asset"], member["asset"])
                    for member in group
                ]
                row = {"report": name, "query": query["asset"], "held_out": query["held_out"],
                       "speed_percent": query.get("speed_percent"), "asset": group[0]["asset"],
                       "also": [member["asset"] for member in group[1:]],
                       "hits": group[0]["hits"], "label": max(labels, key=rank)}
                if any(member is only for member in group):
                    row["only_possible"] = True
                rows.append(row)
    return rows


def audio_hours(names, scans):
    total = 0.0
    for name in names:
        report = devscore.load_json(NAS / name)
        if scans:
            total += report["duration_seconds"]
        else:
            total += EXCERPT_SECONDS * len(report["queries"])
    return total / 3600


def tally(rows, hours):
    table = {}
    for threshold in THRESHOLDS:
        shown = [row for row in rows if row["hits"] >= threshold]
        counts = {}
        for row in shown:
            counts[row["label"]] = counts.get(row["label"], 0) + 1
        table[threshold] = {
            "possible": len(shown),
            "per hour": round(len(shown) / hours, 2),
            "by label": counts,
            "right hidden (shown at 60, below the threshold)": sum(
                1 for row in rows if row["label"] == "right" and devscore.POSSIBLE_HITS <= row["hits"] < threshold
            ),
            "found only as possible": sum(1 for row in shown if row.get("only_possible")),
            "found only as possible, hidden": sum(
                1 for row in rows if row.get("only_possible") and row["hits"] < threshold
            ),
        }
    return table


def main():
    tags_file, out = sys.argv[1:3]
    clusters_file = NAS / "duplicate-clusters.json"
    relations = Relations(clusters_file, devscore.ROOT / "docs" / "pair-verdicts.txt", tags_file,
                          NAS / "library-map.json")
    cluster_of = devscore.clusters_of(clusters_file)
    scans = scan_rows(SCANS, relations, cluster_of)
    leave_outs = scan_rows(LEAVE_OUTS, relations, cluster_of)
    queries = query_rows(relations, cluster_of)
    new_scans = scan_rows(NEW_SCANS, relations, cluster_of)
    new_leave_outs = scan_rows(NEW_LEAVE_OUTS, relations, cluster_of)
    every_mix = SCANS + NEW_SCANS
    result = {
        "thresholds": THRESHOLDS,
        "scans": {"hours": round(audio_hours(SCANS, True), 3), "table": tally(scans, audio_hours(SCANS, True)),
                  "plays": scans},
        "leave-outs": {"hours": round(audio_hours(LEAVE_OUTS, True), 3),
                       "table": tally(leave_outs, audio_hours(LEAVE_OUTS, True)), "plays": leave_outs},
        "new mix": {"hours": round(audio_hours(NEW_SCANS, True), 3),
                    "table": tally(new_scans, audio_hours(NEW_SCANS, True)), "plays": new_scans},
        "new mix leave-outs": {"hours": round(audio_hours(NEW_LEAVE_OUTS, True), 3),
                               "table": tally(new_leave_outs, audio_hours(NEW_LEAVE_OUTS, True)),
                               "plays": new_leave_outs},
        "three mixes": {"hours": round(audio_hours(every_mix, True), 3),
                        "table": tally(scans + new_scans, audio_hours(every_mix, True))},
        "queries": {"hours": round(audio_hours(QUERIES, False), 3),
                    "table": tally(queries, audio_hours(QUERIES, False)),
                    "strongest": {}},
    }
    for label in ("right", "related (audio)", "related (tags)", "unrelated"):
        strongest = sorted((row for row in queries if row["label"] == label), key=lambda row: -row["hits"])[:15]
        result["queries"]["strongest"][label] = strongest
    null = scan_detections(SCANS + LEAVE_OUTS + NEW_SCANS + NEW_LEAVE_OUTS, relations, cluster_of)
    result["scan detections below the rule, matching no track"] = {
        label: sorted((row for row in null if row["label"] == label), key=lambda row: -row["hits"])[:10]
        for label in ("related (audio)", "related (tags)", "unrelated")
    }
    result["weak true plays at 262 tracks"] = weak_true_262()
    unrelated = [row for row in null + queries if row["label"] == "unrelated"]
    result["strongest unrelated"] = sorted(unrelated, key=lambda row: -row["hits"])[:10]
    held_out = [row for row in queries if row.get("held_out")]
    result["strongest on audio not in the index"] = sorted(held_out, key=lambda row: -row["hits"])[:10]
    with open(out, "w") as file:
        json.dump(result, file, indent=1, ensure_ascii=False)
    for part in ("scans", "leave-outs", "new mix", "new mix leave-outs", "three mixes", "queries"):
        print(f"{part}: {result[part]['hours']} h of audio")
        for threshold, row in result[part]["table"].items():
            print(f"  {threshold}: {row}")
    print("development scan plays below the confident rule:")
    for row in scans + leave_outs + new_scans + new_leave_outs:
        print(f"  {row['report']} {row['hits']} {row['label']} {row['asset'][-60:]} at {row['span']} "
              f"(playing: {row['playing']}; against {row['against']})")
    print("strongest unrelated:")
    for row in result["strongest unrelated"]:
        print(f"  {row['hits']} {row['report']} {row.get('query', '')[-50:]} -> {row['asset'][-60:]}")
    print("scan detections matching no track, strongest per label:")
    for label, rows in result["scan detections below the rule, matching no track"].items():
        for row in rows[:3]:
            print(f"  {label}: {row['hits']} in {row['windows']} windows {row['report']} {row['asset'][-60:]} at {row['span']}")
    print("weak true plays at 262 tracks:")
    for name, row in result["weak true plays at 262 tracks"].items():
        print(f"  {name}: {row}")
    print("strongest on audio not in the index:")
    for row in result["strongest on audio not in the index"][:5]:
        print(f"  {row['hits']} {row['label']} {row['report']} {row['query'][-50:]} -> {row['asset'][-60:]}")


if __name__ == "__main__":
    main()
