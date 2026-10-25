"""Session 48: features of marked plays (ADR 0019's `shares_material_with`)
that may tell a record playing from shared material, as pre-registered in
docs/brief-48.md (step 1).

    # the (marked file, partner file) pairs for `gunfinger-eval pair`; with
    # --nas both are named as the harness names the other library's files
    python3 -I scripts/analysis/marked_features.py pairs [--nas] OUT.txt CLI_DIR...
    # the features of every marked play of a seed's mixes
    python3 -I scripts/analysis/marked_features.py features MIXES.json CLI_DIR \\
        CLUSTERS.json SHARED_PANEL.json STORE PAIRS.json OUT.json \\
        [--excluded EXCLUDED.txt] [--verified HARNESS_MIXES.json]
    # per class and scenario: ranges, AUC and the rules
    python3 -I scripts/analysis/marked_features.py summary OUT.json...
    # the marked plays of saved reports without truth (the development
    # mixes), with each rule's call; REPORT.json... after the pair report
    python3 -I scripts/analysis/marked_features.py known [--nas] PAIRS.json REPORT.json...
    # the merge measure: unrelated double drops printed as one entry
    python3 -I scripts/analysis/marked_features.py merges MIXES.json CLI_DIR \
        CLUSTERS.json SHARED_PANEL.json STORE [--excluded EXCLUDED.txt]

Classes: `playing` (a truth play of the marked play's recording overlaps
it; split by its scenario), `shared` (none, but a truth play of its family
does: a related version playing), `other` (neither; in no rule's score).
"""
import json
import os
import statistics
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import tracklist_score as t  # noqa: E402

# An alignment counts when a search would call it possible
# (`MIN_POSSIBLE_HITS`).
COUNTING_HITS = 60
# How far outside an alignment's query span the marked file time may lie
# (`PASSAGE_GAP_SECONDS`).
SPAN_SLACK = 10.0
SEARCH = "peaks, turntable ladder"
# The rules fixed before any feature was computed (brief 48, 02:43): each
# says whether a marked play would be printed as a record playing.
RULES = {
    "R1 F1 < 60": lambda f: f["F1"] < COUNTING_HITS,
    "R2 F2 > 10 s or none": lambda f: f["F2"] is None or f["F2"] > 10.0,
    "R3 F3 >= 60": lambda f: f["F3"] >= COUNTING_HITS,
}
# Which side of each feature reads as a record playing: lower (-1) or
# higher (+1); a feature without a value counts as the playing side's end.
DIRECTION = {"F1": -1, "F2": 1, "F3": 1, "F3s": 1, "F4": None, "F4x": None, "F5": 1}
UNRELATED = ("unrelated-drop", "quiet", "brief")


def strongest(play):
    return max(play["segments"], key=lambda segment: segment["hits"])


def file_time(segment, at):
    """A segment's file time at mix time `at`, linear within it (or
    extrapolated at its own rate)."""
    rate = ((segment["track_end_seconds"] - segment["track_start_seconds"])
            / (segment["end_seconds"] - segment["start_seconds"]))
    return segment["track_start_seconds"] + (at - segment["start_seconds"]) * rate


def holding(play, at):
    def distance(segment):
        return max(segment["start_seconds"] - at, at - segment["end_seconds"], 0.0)
    return min(play["segments"], key=distance)


def outside(span, spans):
    """The length of `span` outside the union of `spans`."""
    start, end = sorted(span)
    covered = 0.0
    cursor = start
    for low, high in sorted(spans):
        low, high = max(low, cursor), min(high, end)
        if high > low:
            covered += high - low
            cursor = high
    return (end - start) - covered


def features(marked, partner, alignments, verified):
    at = (strongest(marked)["start_seconds"] + strongest(marked)["end_seconds"]) / 2
    m = file_time(strongest(marked), at)
    p = file_time(holding(partner, at), at)
    counting = [a for a in alignments if a["hits"] >= COUNTING_HITS]
    near = [a for a in counting
            if a["query_start_seconds"] - SPAN_SLACK <= m <= a["query_end_seconds"] + SPAN_SLACK]
    errors = [(abs(a["offset_seconds"] + a["speed"] * m - p), a) for a in near]
    f2 = min(errors, key=lambda e: e[0]) if errors else None
    spans = [(a["query_start_seconds"], a["query_end_seconds"]) for a in counting]
    f3 = f3s = 0.0
    for segment in marked["segments"]:
        span = (segment["track_start_seconds"], segment["track_end_seconds"])
        length = abs(span[1] - span[0])
        away = outside(span, spans)
        f3s += away
        f3 += segment["hits"] * (away / length if length else 0.0)
    ratio = partner["speed"] / marked["speed"]
    f5 = abs(ratio / f2[1]["speed"] - 1) if f2 else abs(ratio - 1)
    return {
        "F1": max((a["hits"] for a in alignments), default=0),
        "F2": f2[0] if f2 else None,
        "F3": f3,
        "F3s": f3s,
        "F4": verified["aligned"]["share"] if verified else None,
        "F4x": (verified["aligned"]["share"]
                - statistics.mean(c["share"] for c in verified["shifted"])) if verified else None,
        "F5": f5,
        "marked_file_seconds": m,
        "partner_file_seconds": p,
    }


def verification(harness_mix, marked):
    """The harness detection of the marked play's file overlapping its
    strongest segment most, and its verifier's measures."""
    segment = strongest(marked)
    best, most = None, 0.0
    for found in harness_mix["detections"]:
        if found["asset"] != marked["asset"] or "verified" not in found:
            continue
        shared = (min(found["end_seconds"], segment["end_seconds"])
                  - max(found["start_seconds"], segment["start_seconds"]))
        if shared > most:
            best, most = found["verified"], shared
    return best


def pair_key(asset, nas):
    return (t.NAS_PREFIX if nas else "") + asset


def pairs(out, cli_dirs, nas):
    wanted = set()
    for directory in cli_dirs:
        for name in sorted(os.listdir(directory)):
            if not name.endswith(".json"):
                continue
            plays = json.load(open(os.path.join(directory, name)))["plays"]
            for play in plays:
                if "shares_material_with" in play:
                    partner = plays[play["shares_material_with"]["play"] - 1]
                    wanted.add((pair_key(play["asset"], nas), pair_key(partner["asset"], nas)))
    open(out, "w").write("".join(f"{a}\t{b}\n" for a, b in sorted(wanted)))
    print(f"{len(wanted)} pairs")


def run(mixes_path, cli_dir, clusters, panel, store_copy, pairs_path, out,
        excluded=None, verified_path=None):
    library = t.Library(clusters, panel, store_copy, excluded)
    nas = excluded is not None
    searched = {}
    for report in json.load(open(pairs_path)):
        search = next((s for s in report["searches"] if s["name"] == SEARCH), {"alignments": []})
        searched[(report["query"], report["found"])] = search["alignments"]
    listed = set()
    for pair in json.load(open(clusters))["pairs"]:
        listed.add((pair["query"], pair["found"]))
        listed.add((pair["found"], pair["query"]))
    harness = ({result["mix"]["name"]: result for result in json.load(open(verified_path))["mixes"]}
               if verified_path else {})
    rows = []
    for result in json.load(open(mixes_path))["mixes"]:
        mix = result["mix"]
        plays = json.load(open(os.path.join(cli_dir, mix["name"] + ".json")))["plays"]
        for truth in mix["plays"]:
            truth["recording"] = library.recording(truth["asset"])
            truth["family"] = library.family_of(truth["asset"])
        for number, play in enumerate(plays, 1):
            if "shares_material_with" not in play:
                continue
            partner = plays[play["shares_material_with"]["play"] - 1]
            key = library.key(play["asset"])
            keyed = dict(play, asset=key, same_audio=[library.key(x) for x in play.get("same_audio", [])])
            playing = [tp for tp in mix["plays"] if t.holds(keyed, tp["recording"]) and t.overlap(keyed, tp)]
            related = any(tp["family"] == library.family_of(key) and t.overlap(keyed, tp)
                          for tp in mix["plays"])
            if playing:
                label, scenario = "playing", playing[0].get("scenario", "-")
            else:
                label, scenario = ("shared" if related else "other"), "-"
            alignments = searched[(pair_key(play["asset"], nas), pair_key(partner["asset"], nas))]
            verified = verification(harness[mix["name"]], play) if harness else None
            row = features(play, partner, alignments, verified)
            row.update({
                "mix": mix["name"], "play": number, "asset": play["asset"],
                "partner": partner["asset"], "class": label, "scenario": scenario,
                "best_hits": strongest(play)["hits"],
                "seconds": play["end_seconds"] - play["start_seconds"],
                "F1c": (pair_key(play["asset"], nas), pair_key(partner["asset"], nas)) in listed
                or (key, library.key(partner["asset"])) in listed,
            })
            rows.append(row)
    json.dump(rows, open(out, "w"), indent=1)
    counts = {}
    for row in rows:
        counts[(row["class"], row["scenario"])] = counts.get((row["class"], row["scenario"]), 0) + 1
    print(out, sorted(counts.items()))


def auc(playing, shared, direction):
    """The chance that a playing value lies on the playing side of a shared
    one, ties half; a missing value is the playing side's end."""
    end = float("inf")
    def value(v):
        return end if v is None else direction * v
    pairs = [(value(a), value(b)) for a in playing for b in shared]
    if not pairs:
        return None
    return sum(1.0 if a > b else 0.5 if a == b else 0.0 for a, b in pairs) / len(pairs)


def best_threshold(rows, feature, direction):
    """The threshold printing the most playing plays while letting in no
    shared one (`>= threshold` on the direction's side)."""
    shared = [direction * r[feature] for r in rows if r["class"] == "shared" and r[feature] is not None]
    bar = max(shared, default=float("-inf"))
    printed = sum(1 for r in rows if r["class"] == "playing" and r[feature] is not None
                  and direction * r[feature] > bar)
    return direction * bar, printed


def described(values):
    values = sorted(v for v in values if v is not None)
    if not values:
        return "-"
    return f"{values[0]:.3g}/{statistics.median(values):.3g}/{values[-1]:.3g}"


def summary(*paths):
    rows = [row for path in paths for row in json.load(open(path))]
    groups = {}
    for row in rows:
        groups.setdefault((row["class"], row["scenario"]), []).append(row)
    print(f"{len(rows)} marked plays")
    names = ["F1", "F2", "F3", "F3s", "F4", "F4x", "F5", "best_hits", "seconds"]
    for (label, scenario), group in sorted(groups.items()):
        missing_f2 = sum(1 for r in group if r["F2"] is None)
        listed = sum(1 for r in group if r["F1c"])
        print(f"  {label:8} {scenario:15} n={len(group):3} F2 none {missing_f2:3} clusters' pair {listed:3}")
        for name in names:
            print(f"      {name:10} min/median/max {described(r[name] for r in group)}")
    playing = [r for r in rows if r["class"] == "playing"]
    unrelated = [r for r in playing if r["scenario"] in UNRELATED]
    shared = [r for r in rows if r["class"] == "shared"]
    for name, direction in DIRECTION.items():
        if direction is None:
            above = auc([r[name] for r in playing], [r[name] for r in shared], 1)
            print(f"  AUC {name}: higher reads playing {above}")
            continue
        print(f"  AUC {name}: all playing {auc([r[name] for r in playing], [r[name] for r in shared], direction)},"
              f" unrelated scenarios {auc([r[name] for r in unrelated], [r[name] for r in shared], direction)}")
    for name in ("F1", "F2", "F3", "F4", "F4x", "F5"):
        for direction in ((-1,) if name == "F1" else (1,) if DIRECTION[name] else (1, -1)):
            bar, printed = best_threshold(rows, name, direction)
            print(f"  best {name} ({'higher' if direction > 0 else 'lower'} prints): beyond {bar:.4g} prints {printed} of {len(playing)} playing, no shared")
    for rule, prints in RULES.items():
        line = [f"shared let in {sum(prints(r) for r in shared)} of {len(shared)}"]
        for scenario in sorted({r['scenario'] for r in playing}):
            group = [r for r in playing if r["scenario"] == scenario]
            line.append(f"{scenario} {sum(prints(r) for r in group)}/{len(group)}")
        others = [r for r in rows if r["class"] == "other"]
        line.append(f"other {sum(prints(r) for r in others)}/{len(others)}")
        print(f"  {rule}: unrelated printed {sum(prints(r) for r in unrelated)}/{len(unrelated)}; " + ", ".join(line))


def known(pairs_path, reports, nas):
    searched = {}
    for report in json.load(open(pairs_path)):
        search = next((s for s in report["searches"] if s["name"] == SEARCH), {"alignments": []})
        searched[(report["query"], report["found"])] = search["alignments"]
    for path in reports:
        plays = json.load(open(path))["plays"]
        for number, play in enumerate(plays, 1):
            if "shares_material_with" not in play:
                continue
            partner = plays[play["shares_material_with"]["play"] - 1]
            alignments = searched[(pair_key(play["asset"], nas), pair_key(partner["asset"], nas))]
            f = features(play, partner, alignments, None)
            calls = ", ".join(f"{rule.split()[0]} {'prints' if prints(f) else 'leaves out'}"
                              for rule, prints in RULES.items())
            f2 = "none" if f["F2"] is None else f"{f['F2']:.1f}"
            print(f"{os.path.basename(path)} play {number} {play['asset']} ({play['end_seconds'] - play['start_seconds']:.0f} s)"
                  f" in {partner['asset']}: F1 {f['F1']}, F2 {f2}, F3 {f['F3']:.0f} ({f['F3s']:.0f} s), F5 {f['F5']:.4f}; {calls}")


def started(truth):
    """The mix time at which a truth play's track would have started, as
    `same_recording` compares it."""
    return truth["start_seconds"] - truth["source_start_seconds"] / truth["speed"]


def merges(mixes_path, cli_dir, clusters, panel, store_copy, excluded=None):
    """Per unrelated double drop: whether its drops were lined up, whether
    the tracks would have started within `SAME_RECORDING_SECONDS` by the
    truth, and whether one printed entry holds plays of both records."""
    library = t.Library(clusters, panel, store_copy, excluded)
    blocks = placed = close = merged = merged_close = 0
    for result in json.load(open(mixes_path))["mixes"]:
        mix = result["mix"]
        plays = json.load(open(os.path.join(cli_dir, mix["name"] + ".json")))["plays"]
        for play in plays:
            play["asset"] = library.key(play["asset"])
            play["same_audio"] = [library.key(x) for x in play.get("same_audio", [])]
        printed = [entry for entry in t.entries(plays) if t.printed(entry)]
        truths = mix["plays"]
        for first, second in zip(truths, truths[1:]):
            if (first.get("scenario"), first.get("side"), second.get("side")) != ("unrelated-drop", "first", "second"):
                continue
            blocks += 1
            placed += first.get("drop_seconds") is not None
            near = abs(started(first) - started(second)) <= t.SAME_RECORDING_SECONDS
            close += near
            def has(entry, truth):
                recording = library.recording(truth["asset"])
                return any(t.holds(p, recording) and t.overlap(p, truth) for p in entry)
            joined = any(has(entry, first) and has(entry, second) for entry in printed)
            merged += joined
            merged_close += joined and near
    print(f"{mixes_path}: {blocks} unrelated drops, drops lined up {placed}, "
          f"tracks would have started within {t.SAME_RECORDING_SECONDS:g} s {close}, "
          f"one printed entry {merged} ({merged_close} of them within)")


if __name__ == "__main__":
    command, *arguments = sys.argv[1:]
    if command == "pairs":
        nas = "--nas" in arguments
        rest = [a for a in arguments if a != "--nas"]
        pairs(rest[0], rest[1:], nas)
    elif command == "features":
        options = {}
        positional = []
        it = iter(arguments)
        for argument in it:
            if argument in ("--excluded", "--verified"):
                options[argument[2:]] = next(it)
            else:
                positional.append(argument)
        run(*positional, excluded=options.get("excluded"), verified_path=options.get("verified"))
    elif command == "summary":
        summary(*arguments)
    elif command == "known":
        nas = "--nas" in arguments
        rest = [a for a in arguments if a != "--nas"]
        known(rest[0], rest[1:], nas)
    elif command == "merges":
        if "--excluded" in arguments:
            at = arguments.index("--excluded")
            merges(*arguments[:at], excluded=arguments[at + 1])
        else:
            merges(*arguments)
    else:
        sys.exit(__doc__)
