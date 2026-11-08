"""Session 84: the share of a scan the search over the gaps covers (gap_search::stretches).

gap_search_share.py SCAN.json ...: per report, the uncovered stretches of 10 s or more, padded by
10 s either side and merged, as seconds and a share of the mix.
"""
import json
import sys

MIN, PAD = 10.0, 10.0


def stretches(report):
    covered = []
    for play in report["plays"]:
        if any(s["confident"] for s in play["segments"]):
            covered += [(s["start_seconds"], s["end_seconds"]) for s in play["segments"]]
    covered.sort()
    end = report["duration_seconds"]
    gaps, reached = [], 0.0
    for a, b in covered:
        if a > reached:
            gaps.append((reached, min(a, end)))
        reached = max(reached, b)
    if reached < end:
        gaps.append((reached, end))
    padded = []
    for a, b in gaps:
        if b - a < MIN:
            continue
        a, b = max(0.0, a - PAD), min(end, b + PAD)
        if padded and a <= padded[-1][1]:
            padded[-1] = (padded[-1][0], max(padded[-1][1], b))
        else:
            padded.append((a, b))
    return gaps, padded


for path in sys.argv[1:]:
    with open(path) as f:
        report = json.load(f)
    gaps, padded = stretches(report)
    raw = sum(b - a for a, b in gaps)
    searched = sum(b - a for a, b in padded)
    d = report["duration_seconds"]
    print(f"{path}: duration {d:.0f} s; uncovered {raw:.0f} s ({raw / d:.1%}); "
          f"searched over the gaps {searched:.0f} s ({searched / d:.1%}) in {len(padded)} stretches")
