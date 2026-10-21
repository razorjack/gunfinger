"""2026-10-21 (session 35): method 2's stretches by k on the gaps of saved
`self-match` listings, split into gaps holding no indexed track (only
missed records) and the rest, as experiment 0166 reports them beside the
registered all-gaps figures. With --at-ratios, the blocks with repeats at
other speed ratios are counted.

usage: python3 -I scripts/analysis/ride_gaps.py [--at-ratios] LISTING ...
"""

import json
import sys

sys.path.insert(0, "scripts/analysis")
from gap_tests import stretches  # noqa: E402


def main():
    at_ratios = "--at-ratios" in sys.argv[1:]
    rows = {}
    for path in [arg for arg in sys.argv[1:] if arg != "--at-ratios"]:
        with open(path) as f:
            for gap in json.load(f):
                k = gap["missed_tracks"]
                if not 1 <= k <= 4:
                    continue
                key = "only missed records" if gap["indexed_tracks"] == 0 else "with an indexed track"
                count = len(stretches(gap["blocks_at_ratios"] if at_ratios else gap["blocks"]))
                row = rows.setdefault(key, {}).setdefault(k, [0, 0, 0])
                row[0] += 1
                row[1] += count == k
                row[2] += abs(count - k) <= 1
    for key, by_k in sorted(rows.items()):
        total = [sum(row[i] for row in by_k.values()) for i in range(3)]
        cells = ", ".join(f"k={k}: {row[1]}/{row[0]}" for k, row in sorted(by_k.items()))
        print(f"{key}: exact {cells}; pooled exact {total[1]}/{total[0]}, within one {total[2]}/{total[0]}")


if __name__ == "__main__":
    main()
