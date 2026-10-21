"""2026-10-21 (session 35): method 2 (the gap against itself), the
neighbour discount and the combined estimate against the truth, from the
listings `gunfinger-eval self-match` writes (with `--neighbours`, and with
`--ratios` for `--at-ratios`) and those `gunfinger-eval gaps` writes for
the same reports (for method 1's range B). The rules are the ones
`docs/research/2026-10-21-gap-tests.md` registers; the constants below are
theirs.

- stretches: blocks at MIN_HITS hits spanning MIN_BLOCK_SECONDS or more,
  blocks whose spans overlap merged (session 34's fixed reading);
- discounted: stretches less those lying mostly in a neighbour's
  material, a stretch whose span neighbour detections of NEIGHBOUR_HITS
  hits or more cover for at least NEIGHBOUR_SHARE of it;
- combined: the discounted count limited to method 1's range (A or B,
  RANGE); 0 stays 0; without an estimate, the discounted count.

Gaps holding as many missed tracks at about the same place in several
listings of one set count once (as `self_match_scores.py`); with --mixes,
mixes of different files are told apart.

usage: python3 -I scripts/analysis/gap_tests.py [--mixes] [--at-ratios]
       [--range A|B] [--json OUT] LISTING ...
LISTING is a self-match listing; its `gaps` listing is found beside it
(`self-match-<name>` -> `<name>`).
"""

import argparse
import json
import os
import statistics
from collections import defaultdict

MIN_HITS = 10
MIN_BLOCK_SECONDS = 30.0
NEIGHBOUR_HITS = 10
NEIGHBOUR_SHARE = 0.5
MIN_LENGTH = 30.0


def merged(spans):
    out = []
    for start, end in sorted(spans):
        if out and start < out[-1][1]:
            out[-1][1] = max(out[-1][1], end)
        else:
            out.append([start, end])
    return out


def stretches(blocks):
    at = next(b for b in blocks if b["min_hits"] == MIN_HITS)
    return merged([s for s in at["spans"] if s[1] - s[0] >= MIN_BLOCK_SECONDS])


def covered_share(span, detections):
    covers = merged([[d["start_seconds"], d["end_seconds"]] for d in detections])
    inside = sum(max(0.0, min(span[1], b) - max(span[0], a)) for a, b in covers)
    return inside / (span[1] - span[0])


def discounted(spans, neighbours):
    strong = [d for d in neighbours or [] if d["hits"] >= NEIGHBOUR_HITS]
    return [s for s in spans if covered_share(s, strong) < NEIGHBOUR_SHARE]


def range_a(estimate):
    return (max(1, estimate - 1), estimate + 1) if estimate is not None else None


def range_b(length, intervals):
    if len(intervals) < 3:
        return None
    lower, _, upper = statistics.quantiles(intervals, n=4)
    return (max(1, round(length / upper)), max(1, round(length / lower)))


def limited(count, bounds):
    if count == 0 or bounds is None:
        return count
    return min(max(count, bounds[0]), bounds[1])


def gap_intervals(listing_path):
    """(mix, start) -> intervals at MIN_LENGTH, from the gaps listing."""
    out = {}
    if not os.path.exists(listing_path):
        return out
    with open(listing_path) as f:
        for listing in json.load(f):
            at = next(a for a in listing["at"] if a["min_length_seconds"] == MIN_LENGTH)
            for gap in at["gaps"]:
                out[(listing["name"], round(gap["start_seconds"], 1))] = at["intervals"]
    return out


def rows(paths, mixes, at_ratios, which_range):
    seen = set()
    out = []
    for path in paths:
        directory, name = os.path.split(path)
        intervals = gap_intervals(os.path.join(directory, name.replace("self-match-", "", 1)))
        with open(path) as f:
            for gap in json.load(f):
                mix = (path, gap["mix"]) if mixes else gap["mix"]
                key = (mix, gap["missed_tracks"], round(gap["start_seconds"] / 10))
                if gap["missed_tracks"] and key in seen:
                    continue
                seen.add(key)
                blocks = gap["blocks_at_ratios"] if at_ratios else gap["blocks"]
                spans = stretches(blocks)
                kept = discounted(spans, gap.get("neighbours"))
                length = gap["end_seconds"] - gap["start_seconds"]
                found = intervals.get((gap["mix"], round(gap["start_seconds"], 1)), [])
                bounds = range_a(gap["estimate"]) if which_range == "A" else range_b(length, found)
                out.append(
                    {
                        "file": os.path.basename(path),
                        "mix": gap["mix"],
                        "start_seconds": gap["start_seconds"],
                        "length_seconds": length,
                        "truth": gap["missed_tracks"],
                        "method 1": gap["estimate"],
                        "range": bounds,
                        "stretches": len(spans),
                        "discounted": len(kept),
                        "combined": limited(len(kept), bounds),
                        "combined without discount": limited(len(spans), bounds),
                    }
                )
    return out


METHODS = ["method 1", "stretches", "discounted", "combined without discount", "combined"]


def table(rows):
    summary = {}
    for method in METHODS:
        by_truth = defaultdict(list)
        for row in rows:
            by_truth[min(row["truth"], 5)].append(row[method])
        cells = {}
        for truth, values in sorted(by_truth.items()):
            known = [v for v in values if v is not None]
            cells[truth] = {
                "gaps": len(values),
                "estimated": len(known),
                "exact": sum(v == truth for v in known),
                "within 1": sum(abs(v - truth) <= 1 for v in known),
                "off by 2+": sum(abs(v - truth) >= 2 for v in known),
                "low": sum(v < truth for v in known),
                "high": sum(v > truth for v in known),
            }
        summary[method] = cells
    return summary


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--mixes", action="store_true")
    parser.add_argument("--at-ratios", action="store_true")
    parser.add_argument("--range", choices=["A", "B"], default="A")
    parser.add_argument("--json")
    parser.add_argument("listings", nargs="+")
    args = parser.parse_args()
    found = rows(args.listings, args.mixes, args.at_ratios, args.range)
    summary = table(found)
    for method, cells in summary.items():
        parts = [
            f"k={k}: {c['exact']}/{c['gaps']} exact, {c['within 1']} within 1, "
            f"{c['off by 2+']} off 2+ (low {c['low']}, high {c['high']}"
            + (f", {c['gaps'] - c['estimated']} none" if c["estimated"] < c["gaps"] else "")
            + ")"
            for k, c in cells.items()
        ]
        print(f"{method:>26}: " + "; ".join(parts))
    if args.json:
        with open(args.json, "w") as f:
            json.dump({"rules": {"range": args.range, "at_ratios": args.at_ratios},
                       "summary": summary, "gaps": found}, f, indent=1)


if __name__ == "__main__":
    main()
