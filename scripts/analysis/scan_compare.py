"""Compares the detections of two scan reports of one set, keyed by asset
and start: added, removed, and the same detections whose hits changed.

usage: scan_compare.py <scan report A> <scan report B>
"""
import json, sys
def load(f):
    return {(d["asset"], round(d["start_seconds"], 3)): d for d in json.load(open(f))["detections"]}
a, b = load(sys.argv[1]), load(sys.argv[2])
for k in sorted(b.keys() - a.keys()):
    d = b[k]; print(f"  added   {d['asset']} {d['start_seconds']:.1f}-{d['end_seconds']:.1f} hits {d['hits']} windows {d['windows']} confident {d['confident']}")
for k in sorted(a.keys() - b.keys()):
    d = a[k]; print(f"  removed {d['asset']} {d['start_seconds']:.1f}-{d['end_seconds']:.1f} hits {d['hits']} windows {d['windows']} confident {d['confident']}")
hits = [(b[k]["hits"] - a[k]["hits"], k[0]) for k in a.keys() & b.keys() if a[k]["hits"] != b[k]["hits"]]
print(f"  same key {len(a.keys() & b.keys())}, hits changed {len(hits)}: {sorted(hits)[:12]}")
