"""2026-10-21 (session 35): how much of each indexed play of generated long
mixes detections of its own file cover, from `mixes` reports (rides
against their control). A detection counts when it has at least 3
windows and 200 hits, near the confidence rule (an approximation: the
fitted pass asks 240). A play's own part is the play without its
crossfades; a play loses a side of a ride when no such detection
covers any of the part before, or after, the ride.

usage: python3 -I scripts/analysis/ride_coverage.py REPORT ...
"""

import json
import sys

MIN_HITS = 200
MIN_WINDOWS = 3


def confident(d):
    return d["windows"] >= MIN_WINDOWS and d["hits"] >= MIN_HITS


def covered(span, detections):
    spans = sorted((max(span[0], d["start_seconds"]), min(span[1], d["end_seconds"])) for d in detections)
    total, reached = 0.0, span[0]
    for a, b in spans:
        a = max(a, reached)
        if b > a:
            total += b - a
            reached = b
    return total


def main():
    for path in sys.argv[1:]:
        with open(path) as f:
            report = json.load(f)
        shares, lost_side, plays = [], 0, 0
        for result in report["mixes"]:
            for play in result["mix"]["plays"]:
                if play["held_out"]:
                    continue
                own = (play["start_seconds"] + play["fade_in_seconds"], play["end_seconds"] - play["fade_out_seconds"])
                if own[1] <= own[0]:
                    continue
                found = [d for d in result["detections"] if d["asset"] == play["asset"] and confident(d)]
                plays += 1
                shares.append(covered(own, found) / (own[1] - own[0]))
                ride = play.get("ride")
                if ride:
                    before = (own[0], min(own[1], ride["start_seconds"]))
                    after = (max(own[0], ride["start_seconds"] + ride["seconds"]), own[1])
                    if any(s[1] - s[0] >= 30 and covered(s, found) == 0 for s in (before, after)):
                        lost_side += 1
        shares.sort()
        print(
            f"{path}: {plays} indexed plays, own part covered: median {shares[len(shares) // 2]:.2f},"
            f" under half for {sum(s < 0.5 for s in shares)}; a side of 30 s or more of a ride without a detection: {lost_side}"
        )


if __name__ == "__main__":
    main()
