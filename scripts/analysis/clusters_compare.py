"""Compares two duplicate-clusters reports (`gunfinger-eval clusters`, for
example the Mac's and the Linux machine's): the clusters as sets of files,
then the pairs (query, found) with same_recording, hits, coverage and
speed, and the change in hits.

usage: clusters_compare.py <duplicate-clusters.json A> <duplicate-clusters.json B>
"""
import json, sys
a = json.load(open(sys.argv[1])); b = json.load(open(sys.argv[2]))
print("criterion equal:", a["criterion"] == b["criterion"])
ca = {frozenset(c) for c in a["duplicates"]}; cb = {frozenset(c) for c in b["duplicates"]}
print(f"clusters: {len(ca)} / {len(cb)}, equal {len(ca & cb)}, only first {len(ca - cb)}, only second {len(cb - ca)}")
for c in ca - cb: print("  only first:", sorted(c))
for c in cb - ca: print("  only second:", sorted(c))
pa = {(p["query"], p["found"]): p for p in a["pairs"]}; pb = {(p["query"], p["found"]): p for p in b["pairs"]}
print(f"pairs: {len(pa)} / {len(pb)}, both {len(pa.keys() & pb.keys())}")
for k in sorted(pa.keys() - pb.keys()): print("  only first:", k, pa[k]["hits"], round(pa[k]["coverage"], 4), pa[k]["same_recording"])
for k in sorted(pb.keys() - pa.keys()): print("  only second:", k, pb[k]["hits"], round(pb[k]["coverage"], 4), pb[k]["same_recording"])
same = {f: 0 for f in ["same_recording", "hits", "coverage", "speed", "all fields"]}
hits_delta = []
for k in sorted(pa.keys() & pb.keys()):
    x, y = pa[k], pb[k]
    for f in ["same_recording", "hits", "coverage", "speed"]:
        same[f] += x[f] == y[f]
    same["all fields"] += x == y
    if x["hits"] != y["hits"]: hits_delta.append(y["hits"] - x["hits"])
    if x["same_recording"] != y["same_recording"]: print("  verdict differs:", k)
print("equal on both:", same)
print("hits second - first:", sorted(hits_delta))
