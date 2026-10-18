"""Compares two sweep reports of one seed (for example a baseline's and a
rerun's) query by query: identical detections, detections that differ
only in speed, queries whose set of (asset, confident) detections
differs, queries whose confident answers differ, and the change in hits
of the same detections (relative, for the confident ones).

usage: sweep_compare.py <sweep-seed-N.json A> <sweep-seed-N.json B>
"""
import json, sys
from collections import Counter
a = json.load(open(sys.argv[1])); b = json.load(open(sys.argv[2]))
assert len(a["queries"]) == len(b["queries"])
identical = 0; deltas = Counter(); rel = []; conf_change = 0; detection_sets = 0; speed_only = 0
for qa, qb in zip(a["queries"], b["queries"]):
    assert (qa["asset"], qa["speed_percent"], qa["start_seconds"]) == (qb["asset"], qb["speed_percent"], qb["start_seconds"])
    if qa["detections"] == qb["detections"]:
        identical += 1; continue
    ka = {(d["asset"], d["confident"]): d for d in qa["detections"]}
    kb = {(d["asset"], d["confident"]): d for d in qb["detections"]}
    if ka.keys() != kb.keys(): detection_sets += 1
    ca = sorted(d["asset"] for d in qa["detections"] if d["confident"]); cb = sorted(d["asset"] for d in qb["detections"] if d["confident"])
    if ca != cb: conf_change += 1
    hits_equal = True
    for k in ka.keys() & kb.keys():
        x, y = ka[k]["hits"], kb[k]["hits"]
        if x != y:
            hits_equal = False; deltas[y - x] += 1
            if k[1]: rel.append(abs(y - x) / x)
    if hits_equal and ka.keys() == kb.keys(): speed_only += 1
print(f"identical {identical}/720; differing only in speed {speed_only}; detection set differs {detection_sets}; confident answers differ {conf_change}")
print("hit changes (second - first):", dict(sorted(deltas.items())))
rel.sort()
if rel: print(f"confident detections with changed hits: {len(rel)}, median |change| {rel[len(rel)//2]*100:.2f}%, max {rel[-1]*100:.2f}%")
