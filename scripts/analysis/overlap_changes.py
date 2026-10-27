"""2026-10-27 (session 53): detections a change of the second suppression
(ADR 0021) adds or removes, report against report.

  scan BEFORE AFTER    `scan` reports: each detection in one and not the
      other, with the stronger overlapping detection of its file in the
      report that keeps the stronger one, and their speed difference as a
      share of the stronger one's speed
  sweep BEFORE AFTER   `sweep` reports: per query, each detection in one
      and not the other, with the speed difference to the strongest
      detection of its file in the same query

A detection added by ADR 0021 overlaps a stronger one of its file more
than 0.6% away in speed; one removed overlaps, within 0.6%, a stronger
one that the rule before dropped.

usage: python3 -I scripts/analysis/overlap_changes.py scan|sweep BEFORE AFTER
"""

import json
import sys


def key(d):
    return tuple(sorted((k, v) for k, v in d.items() if not isinstance(v, (list, dict))))


def level(d):
    if d.get("confident"):
        return "confident"
    return "possible" if d["hits"] >= 60 else "weak"


def speed_gap(d, stronger):
    return d["speed"] / stronger["speed"] - 1.0


def stronger_overlapping(d, detections):
    return [
        o
        for o in detections
        if o["asset"] == d["asset"]
        and o["hits"] > d["hits"]
        and o["start_seconds"] < d["end_seconds"]
        and d["start_seconds"] < o["end_seconds"]
    ]


def describe(d):
    return (
        f'{level(d)} {d["hits"]} hits {d["start_seconds"]:.1f}-{d["end_seconds"]:.1f} s '
        f'speed {d["speed"]:.5f} {d["asset"]}'
    )


def scan(before_path, after_path):
    before, after = (json.load(open(p))["detections"] for p in (before_path, after_path))
    before_keys, after_keys = {key(d) for d in before}, {key(d) for d in after}
    added = [d for d in after if key(d) not in before_keys]
    removed = [d for d in before if key(d) not in after_keys]
    print(f"{after_path}: {len(before)} -> {len(after)} detections, {len(added)} added, {len(removed)} removed")
    for label, changed, where in (("added", added, after), ("removed", removed, after)):
        for d in sorted(changed, key=lambda d: -d["hits"]):
            print(f"  {label}: {describe(d)}")
            for o in stronger_overlapping(d, where):
                print(f"    beside {describe(o)}; speed gap {100 * speed_gap(d, o):+.3f}%")


def sweep(before_path, after_path):
    before, after = (json.load(open(p))["queries"] for p in (before_path, after_path))
    changed = 0
    for i, (b, a) in enumerate(zip(before, after)):
        if b["detections"] == a["detections"]:
            continue
        changed += 1
        bk, ak = {key(d) for d in b["detections"]}, {key(d) for d in a["detections"]}
        for label, ds, other in (("added", a["detections"], bk), ("removed", b["detections"], ak)):
            for d in ds:
                if key(d) in other:
                    continue
                same = [o for o in a["detections"] if o["asset"] == d["asset"] and o is not d]
                gaps = ", ".join(
                    f'{100 * ((1 + d["speed_percent"] / 100) / (1 + o["speed_percent"] / 100) - 1):+.3f}% to {o["hits"]} hits'
                    for o in same
                )
                lv = "confident" if d.get("confident") else ("possible" if d["hits"] >= 60 else "weak")
                print(
                    f'query {i} ({b["asset"]} at {b["speed_percent"]:+}%): {label} {lv} {d["hits"]} hits '
                    f'{d["speed_percent"]:+.3f}% {d["asset"]}' + (f"; beside {gaps}" if gaps else "")
                )
    print(f"{after_path}: {changed} of {len(before)} queries with other detections")


def main():
    command, before_path, after_path = sys.argv[1:4]
    {"scan": scan, "sweep": sweep}[command](before_path, after_path)


if __name__ == "__main__":
    main()
