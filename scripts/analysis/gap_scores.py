"""2026-10-21 (session 34): gaps and method 1's counts against the truth,
counted as `docs/research/2026-10-21-gap-measures.md` registers them, from
the listings `gunfinger-eval gaps` writes.

Groups of listings are named on the command line, in order:

  --plain       scans with nothing left out, one per set
  --leave-outs  the design's slot leave-outs, in the design's order
  --mixes       generated or rendered mixes (exact truth)

A track missed in a set's plain scan (absent, or not in the library at that
scale) is natural; it counts once, from the plain scan, and not again in
the leave-outs. Beside the registered interval, the single-run interval
takes the same mix's plain scan's entries (a deviation, see `plain_interval`).
In the tables by truth a gap counts once per set and set of
missed tracks it holds, first in the plain scan, then in the leave-outs'
order. Gaps holding no missed track count in every scan, and once per set
with both edges within 5 s (beside it).

usage: python3 -I scripts/analysis/gap_scores.py [--plain F ...]
       [--leave-outs F ...] [--mixes F ...] [--json OUT]
"""

import argparse
import json
import statistics
from collections import defaultdict

SAME_EDGES_SECONDS = 5.0
PLAY_LENGTHS = ((0, 30), (30, 40), (40, 50), (50, 1e9))


def load(paths):
    listings = []
    for path in paths:
        with open(path) as f:
            for listing in json.load(f):
                listing["file"] = path.rsplit("/", 1)[-1]
                listings.append(listing)
    return listings


def at_length(listing, length):
    return next(at for at in listing["at"] if at["min_length_seconds"] == length)


def lengths(listings):
    return [at["min_length_seconds"] for at in listings[0]["at"]]


def natural_missed(plain):
    return {
        (listing["name"], track["number"])
        for listing in plain
        for track in listing["tracks"]
        if track["missed"]
    }


def reach(groups, length, natural):
    """Missed tracks and those a listed gap reaches, natural ones once
    (from the plain scans); and the seconds of missed tracks inside listed
    gaps over all their seconds, summed over every scan as it is."""
    total = reached = seconds = seconds_in = 0
    unreached = []
    for group, listings in groups:
        for listing in listings:
            at = at_length(listing, length)
            seconds_in += sum(gap["missed_seconds"] for gap in at["gaps"])
            for track in listing["tracks"]:
                if not track["missed"]:
                    continue
                seconds += track["own_seconds"]
                if group != "plain" and (listing["name"], track["number"]) in natural:
                    continue
                total += 1
                if any(track["number"] in gap["missed_tracks"] for gap in at["gaps"]):
                    reached += 1
                else:
                    unreached.append(
                        (listing["file"], track["number"],
                         round(track["end_seconds"] - track["start_seconds"]))
                    )
    return total, reached, seconds, seconds_in, unreached


def without_missed(listings, length):
    every = []
    for listing in listings:
        for gap in at_length(listing, length)["gaps"]:
            if not gap["missed_tracks"]:
                every.append((listing["name"], listing["file"], gap))
    once = []
    for name, file, gap in every:
        if not any(
            name == other_name
            and abs(gap["start_seconds"] - other["start_seconds"]) <= SAME_EDGES_SECONDS
            and abs(gap["end_seconds"] - other["end_seconds"]) <= SAME_EDGES_SECONDS
            for other_name, _, other in once
        ):
            once.append((name, file, gap))
    return every, once


def quartile_range(gap, intervals):
    if len(intervals) < 2:
        return None
    lower, _, upper = statistics.quantiles(intervals, n=4)
    length = gap["end_seconds"] - gap["start_seconds"]
    return max(1, round(length / upper)), max(1, round(length / lower))


def own_interval_estimate(gap, entry_starts):
    """0154's interval: every interval but the one(s) the gap lies in."""
    intervals = [
        b - a
        for a, b in zip(entry_starts, entry_starts[1:])
        if not (gap["start_seconds"] < b and a < gap["end_seconds"])
    ]
    if len(intervals) < 3:
        return None
    length = gap["end_seconds"] - gap["start_seconds"]
    return max(1, round(length / statistics.median(intervals)))


def plain_interval(gap, plain_entry_starts):
    """A deviation from the pre-registration (docs/brief-34.md, 07:35): the
    interval of the same mix's plain scan, leaving out the intervals the
    gap overlaps, as if the gap's run were the only one missing. Returns
    the estimate and range B from those intervals."""
    if plain_entry_starts is None:
        return None, None
    intervals = [
        b - a
        for a, b in zip(plain_entry_starts, plain_entry_starts[1:])
        if not (gap["start_seconds"] < b and a < gap["end_seconds"])
    ]
    if len(intervals) < 3:
        return None, None
    length = gap["end_seconds"] - gap["start_seconds"]
    return max(1, round(length / statistics.median(intervals))), quartile_range(gap, intervals)


def by_truth(groups, length):
    """Each gap once per set and set of missed tracks it holds."""
    seen = set()
    rows = defaultdict(list)
    plain_starts = {
        listing["name"]: listing["entry_starts"]
        for group, listings in groups
        if group == "plain"
        for listing in listings
    }
    for _, listings in groups:
        for listing in listings:
            at = at_length(listing, length)
            for gap in at["gaps"]:
                truth = len(gap["missed_tracks"])
                if truth:
                    key = (listing["name"], tuple(gap["missed_tracks"]))
                    if key in seen:
                        continue
                    seen.add(key)
                single, single_b = plain_interval(gap, plain_starts.get(listing["name"]))
                rows[min(truth, 5)].append(
                    {
                        "single_run": single,
                        "single_run_b": single_b,
                        "file": listing["file"],
                        "length": round(gap["end_seconds"] - gap["start_seconds"], 1),
                        "estimate": gap["estimate"],
                        "range_b": quartile_range(gap, at["intervals"]),
                        "own_interval": own_interval_estimate(gap, listing["entry_starts"]),
                        "truth": truth,
                        "median": at["median_interval_seconds"],
                    }
                )
    return rows


def range_a(estimate):
    return max(1, estimate - 1), estimate + 1


def summarise(rows):
    out = {}
    for truth, gaps in sorted(rows.items()):
        estimated = [gap for gap in gaps if gap["estimate"] is not None]
        errors = [gap["estimate"] - truth for gap in estimated]
        in_a = [range_a(gap["estimate"])[0] <= truth <= range_a(gap["estimate"])[1] for gap in estimated]
        with_b = [gap for gap in estimated if gap["range_b"]]
        in_b = [gap["range_b"][0] <= truth <= gap["range_b"][1] for gap in with_b]
        own = [gap["own_interval"] - truth for gap in gaps if gap["own_interval"] is not None]
        single = [gap for gap in gaps if gap["single_run"] is not None]
        single_errors = [gap["single_run"] - truth for gap in single]
        single_b = [gap for gap in single if gap["single_run_b"]]
        out[truth] = {
            "gaps": len(gaps),
            "no_estimate": len(gaps) - len(estimated),
            "exact": errors.count(0),
            "off_by_one": sum(1 for e in errors if abs(e) == 1),
            "off_by_two_or_more": sum(1 for e in errors if abs(e) >= 2),
            "errors": dict(sorted(((e, errors.count(e)) for e in set(errors)))),
            "range_a_holds": sum(in_a),
            "range_b_holds": sum(in_b),
            "range_b_gaps": len(with_b),
            "range_b_mean_width": (
                statistics.mean(gap["range_b"][1] - gap["range_b"][0] + 1 for gap in with_b)
                if with_b else None
            ),
            "own_interval_exact": own.count(0),
            "own_interval_gaps": len(own),
            "single_run": {
                "gaps": len(single),
                "exact": single_errors.count(0),
                "off_by_one": sum(1 for e in single_errors if abs(e) == 1),
                "off_by_two_or_more": sum(1 for e in single_errors if abs(e) >= 2),
                "errors": dict(sorted(((e, single_errors.count(e)) for e in set(single_errors)))),
                "range_a_holds": sum(
                    1 for gap in single if range_a(gap["single_run"])[0] <= truth <= range_a(gap["single_run"])[1]
                ),
                "range_b_holds": sum(
                    1 for gap in single_b if gap["single_run_b"][0] <= truth <= gap["single_run_b"][1]
                ),
                "range_b_gaps": len(single_b),
                "range_b_mean_width": (
                    statistics.mean(g["single_run_b"][1] - g["single_run_b"][0] + 1 for g in single_b)
                    if single_b else None
                ),
            },
        }
    return out


def interval_bias(listings, length):
    rows = []
    for listing in listings:
        at = at_length(listing, length)
        starts = sorted(track["start_seconds"] for track in listing["tracks"])
        truth = [b - a for a, b in zip(starts, starts[1:])]
        rows.append(
            {
                "file": listing["file"],
                "found_median": at["median_interval_seconds"],
                "truth_median": statistics.median(truth) if len(truth) else None,
            }
        )
    return rows


def missed_plays_by_length(listings, length):
    counts = {bucket: [0, 0] for bucket in PLAY_LENGTHS}
    for listing in listings:
        gaps = at_length(listing, length)["gaps"]
        for track in listing["tracks"]:
            if not track["missed"]:
                continue
            span = track["end_seconds"] - track["start_seconds"]
            bucket = next(b for b in PLAY_LENGTHS if b[0] <= span < b[1])
            counts[bucket][0] += 1
            counts[bucket][1] += any(track["number"] in gap["missed_tracks"] for gap in gaps)
    return {f"{low}-{min(high, 999):.0f} s": value for (low, high), value in counts.items()}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--plain", nargs="*", default=[])
    parser.add_argument("--leave-outs", nargs="*", default=[])
    parser.add_argument("--mixes", nargs="*", default=[])
    parser.add_argument("--json")
    args = parser.parse_args()
    plain, leave_outs, mixes = load(args.plain), load(args.leave_outs), load(args.mixes)
    natural = natural_missed(plain)
    summary = {}
    scans = [("plain", plain), ("leave-outs", leave_outs)]
    every = plain + leave_outs + mixes
    for length in lengths(every):
        entry = {}
        if plain or leave_outs:
            total, reached, seconds, seconds_in, unreached = reach(scans, length, natural)
            plain_every, plain_once = without_missed(plain, length)
            out_every, out_once = without_missed(leave_outs, length)
            listed_out = sum(len(at_length(l, length)["gaps"]) for l in leave_outs)
            listed_plain = sum(len(at_length(l, length)["gaps"]) for l in plain)
            entry["scans"] = {
                "missed_tracks": total,
                "reached": reached,
                "missed_seconds": seconds,
                "missed_seconds_in_gaps": seconds_in,
                "unreached": unreached,
                "plain_gaps": listed_plain,
                "plain_without_missed": len(plain_every),
                "leave_out_gaps": listed_out,
                "leave_out_without_missed": len(out_every),
                "leave_out_without_missed_once": len(out_once),
                "without_missed_examples": [
                    {
                        "file": file,
                        "start": round(gap["start_seconds"], 1),
                        "end": round(gap["end_seconds"], 1),
                        "indexed_tracks": gap["indexed_tracks"],
                        "indexed_seconds": gap["indexed_seconds"],
                        "transition_seconds": gap["transition_seconds"],
                        "possible_plays": gap["possible_plays"],
                    }
                    for _, file, gap in plain_once + out_once
                ],
                "by_truth": summarise(by_truth(scans, length)),
                "interval_bias": interval_bias(plain, length),
            }
        if mixes:
            total, reached, seconds, seconds_in, unreached = reach([("mixes", mixes)], length, set())
            every_gap, _ = without_missed(mixes, length)
            entry["mixes"] = {
                "missed_tracks": total,
                "reached": reached,
                "missed_seconds": seconds,
                "missed_seconds_in_gaps": seconds_in,
                "gaps": sum(len(at_length(l, length)["gaps"]) for l in mixes),
                "without_missed": len(every_gap),
                "without_missed_holding_indexed": sum(1 for _, _, g in every_gap if g["indexed_tracks"]),
                "reached_by_play_length": missed_plays_by_length(mixes, length),
                "by_truth": summarise(by_truth([("mixes", mixes)], length)),
            }
        summary[f"{length:.0f}"] = entry
        print_entry(length, entry)
    if args.json:
        with open(args.json, "w") as f:
            json.dump(summary, f, indent=1)


def print_entry(length, entry):
    print(f"at least {length:.0f} s")
    for kind, data in entry.items():
        share = 100 * data["missed_seconds_in_gaps"] / max(1, data["missed_seconds"])
        line = (
            f"  {kind}: missed tracks reached {data['reached']}/{data['missed_tracks']}, "
            f"missed seconds in gaps {share:.1f}%"
        )
        if kind == "scans":
            line += (
                f"; plain: {data['plain_gaps']} gaps, {data['plain_without_missed']} without a missed track"
                f"; leave-outs: {data['leave_out_gaps']} gaps, {data['leave_out_without_missed']} without"
                f" ({data['leave_out_without_missed_once']} once per set)"
            )
        else:
            line += (
                f"; {data['gaps']} gaps, {data['without_missed']} without a missed track"
                f" ({data['without_missed_holding_indexed']} holding indexed time)"
                f"; reached by play length {data['reached_by_play_length']}"
            )
        print(line)
        for truth, row in data["by_truth"].items():
            print(
                f"    holding {truth}: {row['gaps']} gaps, exact {row['exact']}, off by one {row['off_by_one']},"
                f" off by 2+ {row['off_by_two_or_more']}, range A holds {row['range_a_holds']},"
                f" range B holds {row['range_b_holds']}/{row['range_b_gaps']}"
                f" (mean width {row['range_b_mean_width'] or 0:.1f}), no estimate {row['no_estimate']};"
                f" 0154's interval exact {row['own_interval_exact']}/{row['own_interval_gaps']}; errors {row['errors']}"
            )
            single = row["single_run"]
            if single["gaps"]:
                print(
                    f"      single-run interval: {single['gaps']} gaps, exact {single['exact']},"
                    f" off by one {single['off_by_one']}, off by 2+ {single['off_by_two_or_more']},"
                    f" range A holds {single['range_a_holds']}, range B holds"
                    f" {single['range_b_holds']}/{single['range_b_gaps']}"
                    f" (mean width {single['range_b_mean_width'] or 0:.1f}); errors {single['errors']}"
                )


if __name__ == "__main__":
    main()
