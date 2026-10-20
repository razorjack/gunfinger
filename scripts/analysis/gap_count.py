"""2026-10-20 (experiment 0154): how well the length of a gap in a tracklist
counts the tracks it holds, from the manifests' start times alone.

For each development set and each run of k consecutive tracks (k = 1-4)
with a track before and after it, the run's tracks are taken as missed:
the gap is the time from the first missed track's start to the next
track's start, and the estimate is the gap divided by the median of the
set's other start-to-start intervals, rounded, at least 1. No search runs;
the manifests' starts are approximate, and a real gap would be bounded by
detection edges. Paths naming the held-out test set are refused.

usage: python3 -I scripts/analysis/gap_count.py [MANIFEST ...]
(default: the three development sets' manifests)
"""

import statistics
import sys
import tomllib
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
DEVELOPMENT_SETS = [
    "stakka-skynet-knowledge",
    "razorjack-2003-03-29",
    "nocny-trans-vertical-2002-08-23",
]
HELD_OUT = "ed-rush-optical"
MISSED = (1, 2, 3, 4)


def seconds(text):
    total = 0.0
    for part in text.split(":"):
        total = total * 60 + float(part)
    return total


def starts_of(path):
    with open(path, "rb") as f:
        return [seconds(track["start"]) for track in tomllib.load(f)["track"]]


def count_errors(starts, missed):
    """Estimate minus truth for every run of `missed` tracks between two
    found ones, with at least 3 other intervals to take the median of."""
    intervals = [b - a for a, b in zip(starts, starts[1:])]
    errors = []
    for first in range(1, len(starts) - missed):
        gap = starts[first + missed] - starts[first]
        others = intervals[: first - 1] + intervals[first + missed :]
        if len(others) < 3:
            continue
        estimate = max(1, round(gap / statistics.median(others)))
        errors.append(estimate - missed)
    return errors


def main():
    paths = sys.argv[1:] or [
        ROOT / "corpus/sets" / name / "tracklist.toml" for name in DEVELOPMENT_SETS
    ]
    if any(HELD_OUT in str(path) for path in paths):
        sys.exit("refused: the held-out test set's manifest is never read")
    totals = {missed: Counter() for missed in MISSED}
    for path in paths:
        starts = starts_of(path)
        intervals = [b - a for a, b in zip(starts, starts[1:])]
        lower, _, upper = statistics.quantiles(intervals, n=4)
        print(
            f"{Path(path).parent.name}: {len(starts)} tracks, intervals "
            f"{min(intervals):.0f}-{max(intervals):.0f} s, median "
            f"{statistics.median(intervals):.0f} s, quartiles {lower:.0f}-{upper:.0f} s"
        )
        for missed in MISSED:
            totals[missed].update(count_errors(starts, missed))
    print("missed\tgaps\texact\twithin 1\terrors (estimate - truth: gaps)")
    for missed, errors in totals.items():
        gaps = sum(errors.values())
        within = sum(n for error, n in errors.items() if abs(error) <= 1)
        print(
            f"{missed}\t{gaps}\t{errors[0]} ({errors[0] / gaps:.0%})\t"
            f"{within} ({within / gaps:.0%})\t{dict(sorted(errors.items()))}"
        )


if __name__ == "__main__":
    main()
