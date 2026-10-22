"""2026-10-22 (session 37): what pitch rides of a given size and ramp cost
the search, from `mixes --kind rides` reports of one seed under several
`--ride-change` and `--ride-seconds` (every condition plans the same
plays). Per report, over indexed plays with a ride: sides of 30 s or more
before or after the ride with no confident detection of the play's own
file (240 hits in 3 windows), split by the ride's direction (a ride of 0
changes nothing and has none), and sides with 30 s or more uncovered (a
side whose rest the other side's detection reaches is lost all the same); the seconds
of the plays' own parts (crossfades left out) that no such detection
covers; plays with no confident detection at all; wrong confident
detections (the harness's score).

usage: python3 -I scripts/analysis/ride_dose.py REPORT ...
"""

import json
import sys

MIN_HITS, MIN_WINDOWS, MIN_SIDE = 240, 3, 30.0


def confident(d):
    return d["hits"] >= MIN_HITS and d["windows"] >= MIN_WINDOWS


def covered(span, detections):
    spans = sorted((max(span[0], d["start_seconds"]), min(span[1], d["end_seconds"])) for d in detections)
    total, reached = 0.0, span[0]
    for a, b in spans:
        a = max(a, reached)
        if b > a:
            total += b - a
            reached = b
    return total


def summarise(path):
    report = json.load(open(path))
    row = {
        "report": path.rsplit("/", 1)[-1],
        "plays": 0,
        "sides": 0,
        "lost": 0,
        "lost_up": 0,
        "sides_up": 0,
        "lost_down": 0,
        "sides_down": 0,
        "lost_side_seconds": 0.0,
        "sides_30s_uncovered": 0,
        "uncovered_seconds": 0.0,
        "own_seconds": 0.0,
        "plays_without_confident": 0,
        "wrong_confident": 0,
    }
    for result in report["mixes"]:
        row["wrong_confident"] += len(result["score"]["wrong_confident"])
        for play in result["mix"]["plays"]:
            ride = play.get("ride")
            if play["held_out"] or not ride:
                continue
            own = (play["start_seconds"] + play["fade_in_seconds"], play["end_seconds"] - play["fade_out_seconds"])
            if own[1] <= own[0]:
                continue
            row["plays"] += 1
            found = [d for d in result["detections"] if d["asset"] == play["asset"] and confident(d)]
            whole = (play["start_seconds"], play["end_seconds"])
            if covered(whole, found) == 0:
                row["plays_without_confident"] += 1
            row["own_seconds"] += own[1] - own[0]
            row["uncovered_seconds"] += (own[1] - own[0]) - covered(own, found)
            direction = "up" if ride["to_speed"] > play["speed"] else "down" if ride["to_speed"] < play["speed"] else None
            for side in (
                (own[0], min(own[1], ride["start_seconds"])),
                (max(own[0], ride["start_seconds"] + ride["seconds"]), own[1]),
            ):
                if side[1] - side[0] < MIN_SIDE:
                    continue
                row["sides"] += 1
                if side[1] - side[0] - covered(side, found) >= MIN_SIDE:
                    row["sides_30s_uncovered"] += 1
                if direction:
                    row[f"sides_{direction}"] += 1
                if covered(side, found) == 0:
                    row["lost"] += 1
                    if direction:
                        row[f"lost_{direction}"] += 1
                    row["lost_side_seconds"] += side[1] - side[0]
    return row


def main():
    rows = [summarise(path) for path in sys.argv[1:]]
    print(
        "report | ridden plays | sides lost of 30 s+ | up | down | sides with 30 s+ uncovered | seconds in lost sides"
        " | own seconds uncovered | plays without a confident detection | wrong confident"
    )
    for r in rows:
        print(
            f"{r['report']} | {r['plays']} | {r['lost']}/{r['sides']} | {r['lost_up']}/{r['sides_up']}"
            f" | {r['lost_down']}/{r['sides_down']} | {r['sides_30s_uncovered']} | {r['lost_side_seconds']:.0f}"
            f" | {r['uncovered_seconds']:.0f} of {r['own_seconds']:.0f} | {r['plays_without_confident']}"
            f" | {r['wrong_confident']}"
        )
    print(json.dumps(rows))


if __name__ == "__main__":
    main()
