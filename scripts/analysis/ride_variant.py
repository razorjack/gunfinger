"""2026-10-22 (session 37): a matching variant against the default on the
same mixes or scans (`--overlap-by-chains`, brief 05:20).

  recovery DEFAULT VARIANT   rides `mixes` reports: of the sides of 30 s
      or more the default loses (no confident detection of the play's own
      file; 240 hits in 3 windows), how many the variant does not lose, and
      what share of their missing seconds it covers; the same for sides
      with 30 s or more uncovered (0176)
  wrong DEFAULT VARIANT      `mixes` or `scan` reports: wrong confident
      detections in each, those only the variant has, and right answers
      (confident plays of indexed tracks; identified tracks) in each

usage: python3 -I scripts/analysis/ride_variant.py recovery|wrong DEFAULT VARIANT
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


def sides(report):
    """Each ridden indexed play's sides of 30 s or more, with the mix's
    confident detections of the play's file."""
    for result in report["mixes"]:
        for play in result["mix"]["plays"]:
            ride = play.get("ride")
            if play["held_out"] or not ride:
                continue
            own = (play["start_seconds"] + play["fade_in_seconds"], play["end_seconds"] - play["fade_out_seconds"])
            for side in (
                (own[0], min(own[1], ride["start_seconds"])),
                (max(own[0], ride["start_seconds"] + ride["seconds"]), own[1]),
            ):
                if side[1] - side[0] >= MIN_SIDE:
                    yield (result["mix"]["name"], play["asset"], side)


def found(report):
    """Confident detections by mix and file."""
    by_file = {}
    for result in report["mixes"]:
        for d in result["detections"]:
            if confident(d):
                by_file.setdefault((result["mix"]["name"], d["asset"]), []).append(d)
    return by_file


def recovery(default_path, variant_path):
    default, variant = json.load(open(default_path)), json.load(open(variant_path))
    default_found, variant_found = found(default), found(variant)
    for name, lost in (
        ("no confident detection", lambda side, ds: covered(side, ds) == 0),
        ("30 s or more uncovered", lambda side, ds: side[1] - side[0] - covered(side, ds) >= MIN_SIDE),
    ):
        count = recovered = 0
        seconds = recovered_seconds = 0.0
        for mix, asset, side in sides(default):
            ds = default_found.get((mix, asset), [])
            if not lost(side, ds):
                continue
            vs = variant_found.get((mix, asset), [])
            count += 1
            recovered += not lost(side, vs)
            missing = side[1] - side[0] - covered(side, ds)
            seconds += missing
            recovered_seconds += covered(side, vs) - covered(side, ds)
        share = recovered_seconds / seconds if seconds else 1.0
        print(
            f"{default_path}: sides lost ({name}) {count}; not lost by the variant {recovered}"
            f" ({recovered / count if count else 1:.1%}); their missing seconds {seconds:.0f},"
            f" recovered {recovered_seconds:.0f} ({share:.1%})"
        )


def outcome(report):
    """Wrong confident detections as (place, asset, start, end), and the
    right answers, of a mixes or scan report."""
    if "mixes" in report:
        wrong = [
            (r["mix"]["name"], d["asset"], d["start_seconds"], d["end_seconds"])
            for r in report["mixes"]
            for d in r["score"]["wrong_confident"]
        ]
        right = sum(
            1 for r in report["mixes"] for p in r["score"]["plays"] if not p["truth"]["held_out"] and p["level"] == "confident"
        )
        return wrong, right
    wrong = [
        (report["set"], d["asset"], d["start_seconds"], d["end_seconds"]) for d in report["score"]["wrong_identifications"]
    ]
    return wrong, report["score"]["identified"]


def wrong(default_path, variant_path):
    default_wrong, default_right = outcome(json.load(open(default_path)))
    variant_wrong, variant_right = outcome(json.load(open(variant_path)))
    new = [
        w
        for w in variant_wrong
        if not any(w[0] == d[0] and w[1] == d[1] and w[2] < d[3] and d[2] < w[3] for d in default_wrong)
    ]
    print(
        f"{variant_path}: wrong confident {len(default_wrong)} -> {len(variant_wrong)}, only the variant's {len(new)};"
        f" right {default_right} -> {variant_right}"
    )
    for w in new:
        print("   new wrong:", w)


def main():
    command, default_path, variant_path = sys.argv[1:4]
    {"recovery": recovery, "wrong": wrong}[command](default_path, variant_path)


if __name__ == "__main__":
    main()
