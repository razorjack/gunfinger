"""2026-10-21 (session 34): the single-run interval of gap_scores.py against
scans that leave out one run only.

For each listing of a single-run scan, the gap holding the run's missed
tracks (at 30 s) gets method 1's registered estimate from the scan's own
entries; beside it, the single-run interval's estimate from the same mix's
plain scan, as 0160 computed it. Both are compared with the truth: the
missed tracks the gap holds (at NAS scale only the run's, since the runs
here do not touch Nocny's slot 17). Range B counts only where there is an
estimate, as in gap_scores.py.

usage: python3 -I scripts/analysis/single_run_check.py PLAIN_DIR LISTING ...
(PLAIN_DIR holds the plain scans' listings, scan-<set>-margin-30.json)
"""

import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import gap_scores  # noqa: E402

LENGTH = 30.0


def main():
    plain_dir, paths = sys.argv[1], sys.argv[2:]
    rows = []
    for path in paths:
        for listing in gap_scores.load([path]):
            with open(os.path.join(plain_dir, f"scan-{listing['name']}-margin-30.json")) as f:
                plain = json.load(f)[0]
            at = gap_scores.at_length(listing, LENGTH)
            natural = {t["number"] for t in plain["tracks"] if t["missed"]}
            for gap in at["gaps"]:
                designed = [n for n in gap["missed_tracks"] if n not in natural]
                if not designed:
                    continue
                single, single_b = gap_scores.plain_interval(gap, plain["entry_starts"])
                b = gap_scores.quartile_range(gap, at["intervals"])
                rows.append((listing["file"], len(gap["missed_tracks"]), gap["estimate"], b, single, single_b))
    same = sum(1 for r in rows if r[2] is not None and r[2] == r[4])
    print(f"{len(rows)} gaps; registered estimate equals the single-run one in {same}")
    for k in sorted({r[1] for r in rows}):
        here = [r for r in rows if r[1] == k]
        exact = sum(1 for r in here if r[2] == k)
        in_a = sum(1 for r in here if r[2] is not None and max(1, r[2] - 1) <= k <= r[2] + 1)
        in_b = sum(1 for r in here if r[2] is not None and r[3] and r[3][0] <= k <= r[3][1])
        none = sum(1 for r in here if r[2] is None)
        print(f"  k={k}: {len(here)} gaps, exact {exact}, range A {in_a}, range B {in_b}, no estimate {none}")
    for row in rows:
        print("   ", row)


if __name__ == "__main__":
    main()
