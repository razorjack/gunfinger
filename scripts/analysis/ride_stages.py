"""2026-10-22 (session 37): where the search loses a side of a pitch ride.

Reads a `mixes` report of ridden long mixes and the trace a diagnostic
build wrote beside it (one `trace-<samples>.jsonl` per mix, each line a
detection of 30 hits or more at one stage: `chains` before the first
overlap suppression, `first_kept` after it, `refined` after the second
pass, `final` after the second suppression). For each indexed play's
side of 30 s or more before or after its ride that no confident
detection of its own file covers (240 hits in 3 windows), names the
furthest stage a detection of that file covering the side reached:

  no-chain      no chain of 30 hits or more overlaps the side
  first-drop    a chain does, dropped by the first suppression
  refine-none   kept, then measured with no window in the second pass
  refined-weak  measured below the rule
  second-drop   confident after the second pass, dropped by the second
                suppression (overlapping a stronger detection of the file)
  other         confident in the final list, yet not covering the side

usage: python3 -I scripts/analysis/ride_stages.py REPORT TRACE_DIR [--min-side 30] [--rows]
"""

import collections
import glob
import json
import os
import sys

MIN_HITS, MIN_WINDOWS = 240, 3
SAMPLE_RATE = 8000


def confident(d):
    return d["hits"] >= MIN_HITS and d["windows"] >= MIN_WINDOWS


def overlap(a, b):
    return max(0.0, min(a[1], b[1]) - max(a[0], b[0]))


def load_traces(trace_dir):
    traces = {}
    for path in glob.glob(os.path.join(trace_dir, "trace-*.jsonl")):
        samples = int(os.path.basename(path)[6:-6])
        stages = collections.defaultdict(list)
        with open(path) as f:
            for line in f:
                d = json.loads(line)
                stages[d["stage"]].append(d)
        traces[samples] = stages
    return traces


def trace_of(mix, traces):
    want = mix["seconds"] * SAMPLE_RATE
    samples = min(traces, key=lambda s: abs(s - want))
    if abs(samples - want) > 2 * SAMPLE_RATE:
        raise SystemExit(f"no trace for {mix['name']} ({mix['seconds']:.1f} s)")
    return traces[samples]


def classify(asset, side, stages):
    """The furthest stage a detection of `asset` overlapping `side` reached,
    and the evidence for it."""
    ids = {
        stage: {d["id"]: d for d in stages[stage] if d["asset"] == asset and overlap(side, (d["start"], d["end"])) > 0}
        for stage in ("chains", "first_kept", "refined", "final")
    }
    if not ids["chains"]:
        return "no-chain", None
    order = ["first-drop", "refine-none", "refined-weak", "second-drop", "other"]
    best, evidence = "first-drop", None
    for cid, chain in ids["chains"].items():
        if cid not in ids["first_kept"]:
            label = "first-drop"
        elif cid not in ids["refined"]:
            label = "refine-none"
        elif not confident(ids["refined"][cid]):
            label = "refined-weak"
        elif cid not in ids["final"]:
            label = "second-drop"
        else:
            label = "other"
        if evidence is None or order.index(label) > order.index(best) or (
            label == best and chain["hits"] > evidence[0]["hits"]
        ):
            best, evidence = label, (chain, ids["refined"].get(cid))
    return best, evidence


def stronger_overlapping(asset, detection, stage_list):
    """Stronger detections of `asset` at a stage that overlap `detection`."""
    return [
        d
        for d in stage_list
        if d["asset"] == asset
        and d["id"] != detection["id"]
        and d["hits"] >= detection["hits"]
        and overlap((d["start"], d["end"]), (detection["start"], detection["end"])) > 0
    ]


def main():
    args = [arg for arg in sys.argv[1:] if arg != "--rows"]
    min_side = 30.0
    if "--min-side" in args:
        i = args.index("--min-side")
        min_side = float(args[i + 1])
        del args[i : i + 2]
    report_path, trace_dir = args
    report = json.load(open(report_path))
    traces = load_traces(trace_dir)
    counts = collections.Counter()
    plays = sides = 0
    examples = []
    for result in report["mixes"]:
        mix = result["mix"]
        stages = trace_of(mix, traces)
        for play in mix["plays"]:
            ride = play.get("ride")
            if play["held_out"] or not ride:
                continue
            own = (play["start_seconds"] + play["fade_in_seconds"], play["end_seconds"] - play["fade_out_seconds"])
            if own[1] <= own[0]:
                continue
            plays += 1
            found = [d for d in result["detections"] if d["asset"] == play["asset"] and confident(d)]
            for name, side in (
                ("before", (own[0], min(own[1], ride["start_seconds"]))),
                ("after", (max(own[0], ride["start_seconds"] + ride["seconds"]), own[1])),
            ):
                if side[1] - side[0] < min_side:
                    continue
                sides += 1
                if any(overlap(side, (d["start_seconds"], d["end_seconds"])) > 0 for d in found):
                    continue
                label, evidence = classify(play["asset"], side, stages)
                counts[label] += 1
                row = {
                    "mix": mix["name"],
                    "asset": os.path.basename(play["asset"]),
                    "side": name,
                    "span": [round(side[0], 1), round(side[1], 1)],
                    "change": round(ride["to_speed"] / play["speed"] - 1, 4),
                    "ramp": round(ride["seconds"], 1),
                    "stage": label,
                }
                row["truth_speed"] = round(play["speed"] if name == "before" else ride["to_speed"], 5)
                if evidence:
                    chain, refined = evidence
                    row["chain"] = [chain["start"], chain["end"], chain["hits"], chain["windows"]]
                    if refined:
                        row["refined"] = [refined["start"], refined["end"], refined["hits"], refined["windows"]]
                        winners = stronger_overlapping(play["asset"], refined, stages["refined"])
                        row["dropped_for"] = [[w["start"], w["end"], w["hits"]] for w in winners[:1]]
                        row["refined_speed"] = refined["speed"]
                        if winners:
                            w = winners[0]
                            row["overlap"] = round(overlap((refined["start"], refined["end"]), (w["start"], w["end"])), 2)
                            # How far the second pass moved the dropped side's end that faces the winner.
                            facing_end = "end" if w["start"] > refined["start"] else "start"
                            row["moved"] = round(abs(refined[facing_end] - chain[facing_end]), 2)
                            winner_chain = next((c for c in stages["chains"] if c["id"] == w["id"]), None)
                            if winner_chain:
                                other = "start" if facing_end == "end" else "end"
                                row["winner_moved"] = round(abs(w[other] - winner_chain[other]), 2)
                                row["chains_overlap"] = round(overlap((chain["start"], chain["end"]), (winner_chain["start"], winner_chain["end"])), 2)
                    else:
                        winners = stronger_overlapping(play["asset"], chain, stages["chains"])
                        row["dropped_for"] = [[w["start"], w["end"], w["hits"]] for w in winners[:1]]
                examples.append(row)
    lost = sum(counts.values())
    print(f"{report_path}: {plays} ridden indexed plays, {sides} sides of {min_side:.0f} s or more, {lost} lost")
    for label in ("no-chain", "first-drop", "refine-none", "refined-weak", "second-drop", "other"):
        print(f"  {label:13} {counts[label]}")
    def quantiles(key):
        values = sorted(row[key] for row in examples if key in row)
        if not values:
            return "none"
        pick = lambda q: values[min(len(values) - 1, int(q * len(values)))]
        return f"n {len(values)}, min {values[0]}, median {pick(0.5)}, 90% {pick(0.9)}, max {values[-1]}"

    print(f"  overlap with the stronger side after the second pass (s): {quantiles('overlap')}")
    print(f"  overlap of the two sides' first-pass chains (s): {quantiles('chains_overlap')}")
    print(f"  dropped side's facing end moved by the second pass (s): {quantiles('moved')}")
    print(f"  stronger side's facing end moved by the second pass (s): {quantiles('winner_moved')}")
    errors = sorted(abs(row["refined_speed"] / row["truth_speed"] - 1) for row in examples if "refined_speed" in row)
    if errors:
        print(f"  dropped side's fitted speed against its true speed: median {errors[len(errors) // 2]:.4%}, max {errors[-1]:.4%}")
    if "--rows" in sys.argv:
        for row in examples:
            print("   ", json.dumps(row))


if __name__ == "__main__":
    main()
