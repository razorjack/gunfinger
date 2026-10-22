"""2026-10-22 (session 37): candidate speed changes within plays in a real
mix, from a `scan` report and, optionally, the trace a diagnostic build
wrote for it (`ride_stages.py` describes the trace). Candidates only:
nobody has listened to them.

1. Plays of one file with two or more confident segments: each segment's
   fitted speed, the change of speed between neighbours and the hole
   between them.
2. With a trace: detections confident after the second pass and dropped by
   the second overlap suppression, beside the kept detection of the same
   file they overlap (what `--overlap-by-chains` would keep).

usage: python3 -I scripts/analysis/ride_candidates.py SCAN_REPORT [TRACE_DIR]
"""

import glob
import json
import os
import sys

MIN_HITS, MIN_WINDOWS = 240, 3


def confident(d):
    return d["hits"] >= MIN_HITS and d["windows"] >= MIN_WINDOWS


def clock(seconds):
    return f"{int(seconds // 60)}:{seconds % 60:04.1f}"


def main():
    report = json.load(open(sys.argv[1]))
    print(f"{report['set']}: plays of one file with several confident segments")
    for play in report["plays"]:
        segments = [s for s in play["segments"] if s["confident"]]
        if len(segments) < 2:
            continue
        parts = [f"{clock(s['start_seconds'])}-{clock(s['end_seconds'])} at {s['speed']:.4f}" for s in segments]
        changes = [
            f"{b['speed'] / a['speed'] - 1:+.2%} after a hole of {b['start_seconds'] - a['end_seconds']:.1f} s"
            for a, b in zip(segments, segments[1:])
        ]
        print(f"  {os.path.basename(play['asset'])}: {'; '.join(parts)} ({', '.join(changes)})")
    if len(sys.argv) < 3:
        return
    stages = {}
    for path in glob.glob(os.path.join(sys.argv[2], "trace-*.jsonl")):
        for line in open(path):
            d = json.loads(line)
            stages.setdefault(d["stage"], []).append(d)
    final_ids = {d["id"] for d in stages.get("final", []) if d["asset"]}
    if "default_final" in stages:
        final_ids = {d["id"] for d in stages["default_final"]}
    print(f"{report['set']}: confident after the second pass, dropped by the second suppression")
    for d in sorted(stages.get("refined", []), key=lambda d: d["start"]):
        if not confident(d) or d["id"] in final_ids:
            continue
        kept = [
            k
            for k in stages["refined"]
            if k["asset"] == d["asset"] and k["id"] in final_ids and k["start"] < d["end"] and d["start"] < k["end"]
        ]
        beside = "; ".join(
            f"kept {clock(k['start'])}-{clock(k['end'])} at {k['speed']:.4f}, {k['hits']} hits"
            f" ({d['speed'] / k['speed'] - 1:+.2%})"
            for k in kept
        )
        print(
            f"  {os.path.basename(d['asset'])}: {clock(d['start'])}-{clock(d['end'])} at {d['speed']:.4f},"
            f" {d['hits']} hits in {d['windows']} windows; {beside}"
        )


if __name__ == "__main__":
    main()
