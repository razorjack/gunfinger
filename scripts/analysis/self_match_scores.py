"""2026-10-21 (session 34): the self-match prototype's block counts against
the truth, from the listings `gunfinger-eval self-match` writes.

For each threshold (hits per cell and lag) and each number of missed
tracks a gap holds, three counts are compared with the truth: the blocks
as listed; blocks spanning at least MIN_BLOCK_SECONDS; and stretches,
where blocks whose spans overlap are merged (interleaved blocks are taken
as one record). Gaps holding the same missed tracks in several listings of
one mix count once, as in gap_scores.py.

usage: python3 -I scripts/analysis/self_match_scores.py LISTING ...
"""

import json
import sys
from collections import defaultdict

MIN_BLOCK_SECONDS = 30.0


def stretches(spans):
    merged = []
    for start, end in sorted(spans):
        if merged and start < merged[-1][1]:
            merged[-1][1] = max(merged[-1][1], end)
        else:
            merged.append([start, end])
    return len(merged)


def counts(blocks):
    long_spans = [span for span in blocks["spans"] if span[1] - span[0] >= MIN_BLOCK_SECONDS]
    return {
        "blocks": len(blocks["spans"]),
        "long blocks": len(long_spans),
        "stretches": stretches(long_spans),
    }


def main():
    seen = set()
    rows = defaultdict(lambda: defaultdict(list))
    for path in sys.argv[1:]:
        with open(path) as f:
            for gap in json.load(f):
                key = (gap["mix"], gap["missed_tracks"], round(gap["start_seconds"] / 10))
                if gap["missed_tracks"] and key in seen:
                    continue
                seen.add(key)
                truth = min(gap["missed_tracks"], 5)
                for blocks in gap["blocks"]:
                    for name, count in counts(blocks).items():
                        rows[(blocks["min_hits"], name)][truth].append(count)
                    rows[(blocks["min_hits"], "method 1")][truth].append(gap["estimate"])
    for (min_hits, name), by_truth in sorted(rows.items()):
        if name == "method 1" and min_hits != min(k for k, _ in rows):
            continue
        cells = []
        for truth, values in sorted(by_truth.items()):
            known = [v for v in values if v is not None]
            exact = sum(1 for v in known if v == truth)
            within = sum(1 for v in known if abs(v - truth) <= 1)
            mean = sum(known) / len(known) if known else float("nan")
            cells.append(f"k={truth}: n {len(values)}, mean {mean:.1f}, exact {exact}, within 1 {within}")
        label = name if name == "method 1" else f"{name} at {min_hits}"
        print(f"{label:>18}: " + "; ".join(cells))


if __name__ == "__main__":
    main()
