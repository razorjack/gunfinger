"""Session 84: detections of 60 hits or more that differ between two scan reports.

gap_search_scan_diff.py REF.json ARM.json
"""
import json
import sys


def strong(report):
    return {(d["asset"], round(d["start_seconds"], 1), round(d["end_seconds"], 1), d["hits"],
             d["confident"]) for d in report["detections"] if d["hits"] >= 60}


a, b = (json.load(open(p)) for p in sys.argv[1:3])
added, removed = strong(b) - strong(a), strong(a) - strong(b)
weak = len(b["detections"]) - len(a["detections"])
print(f"detections {len(a['detections'])} -> {len(b['detections'])}; 60+ added {len(added)}, removed {len(removed)}")
for row in sorted(added):
    print("  +", row)
for row in sorted(removed):
    print("  -", row)
