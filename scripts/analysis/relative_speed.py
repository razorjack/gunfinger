"""Recall against the speed relative to the indexed rip (roadmap, "The edge
of the ladder"): `sweep --other-rips` reports at NAS scale, where each
excerpt's source file and its identical copies are left out and only
other rips of the recording can answer.

A rip's speed relative to the source comes from the clusters' pairs (the
found file's time over the query file's at the pair's alignment), chained
through the cluster when two files were not compared directly. An excerpt
played at speed s needs the rung k(1 + s) to find a rip at k; its relative
speed is that rung minus 1, for the rip nearest to 1 among those left in
the index. Also counts the NAS clusters whose members differ by more than
4% in speed.

usage: relative_speed.py <output.json> <nas clusters.json> <corpus clusters.json>
                         <library-map.json> <panels dir> <reports dir>...
"""
import collections
import json
import math
import os
import sys

PREFIX = "second-library/"
output, nas_clusters, corpus_clusters, map_file, panels, *report_dirs = sys.argv[1:]
clusters = json.load(open(nas_clusters))
pairs = clusters["pairs"] + json.load(open(corpus_clusters))["pairs"]
library_map = json.load(open(map_file))
cluster_of = {}
for members in clusters["duplicates"]:
    for member in members:
        cluster_of[member] = set(members)

edges = collections.defaultdict(dict)
for p in pairs:
    if p["same_recording"]:
        edges[p["query"]][p["found"]] = p["speed"]
        edges[p["found"]][p["query"]] = 1.0 / p["speed"]


def speeds_from(source):
    """Each file's time over the source's, through the chain of pairs."""
    speeds, queue = {source: 1.0}, [source]
    while queue:
        here = queue.pop()
        for there, k in edges[here].items():
            if there not in speeds:
                speeds[there] = speeds[here] * k
                queue.append(there)
    return speeds


copies = {c["corpus"]: c for c in library_map["copies"]}


def identical(asset):
    found = {asset}
    own = copies.get(asset)
    if own:
        for c in library_map["copies"]:
            if c["stands_for"] == own["stands_for"]:
                found.add(c["corpus"])
                found |= {PREFIX + path for path in [c["stands_for"], *c.get("also", [])]}
    return found


def excluded_for(seed):
    panel = json.load(open(os.path.join(panels, f"sweep-seed-{seed}.json")))
    out = set()
    for d in panel["draws"]:
        if not d["held_out"] and cluster_of.get(d["asset"], {d["asset"]}) - identical(d["asset"]):
            out |= identical(d["asset"])
    return out


BINS = [(0, 4), (4, 6), (6, 7), (7, 8), (8, 8.4), (8.4, 9), (9, 9.2), (9.2, 10), (10, 15), (15, 100)]
summary = {"reports": {}, "spread": {}}
for report_dir in report_dirs:
    rows = []
    for name in sorted(os.listdir(report_dir)):
        if not (name.startswith("sweep-seed-") and name.endswith("-other-rips.json")):
            continue
        report = json.load(open(os.path.join(report_dir, name)))
        excluded = excluded_for(report["seed"])
        for q in report["queries"]:
            speeds = speeds_from(q["asset"])
            rips = [m for m in cluster_of.get(q["asset"], set()) if m not in excluded]
            relative = {m: speeds[m] * (1 + q["speed_percent"] / 100) - 1 for m in rips if m in speeds}
            nearest = min(relative.values(), key=abs) if relative else None
            correct = [d for d in q["detections"] if d["correct"] and d["confident"]]
            weak = max((d for d in q["detections"] if d["correct"] and not d["confident"]),
                       key=lambda d: d["hits"], default=None)
            wrong = [d for d in q["detections"] if not d["correct"] and d["confident"]]
            rows.append({"seed": report["seed"], "asset": q["asset"], "speed_percent": q["speed_percent"],
                         "rips": len(rips), "rips_with_speed": len(relative),
                         "relative_percent": None if nearest is None else 100 * nearest,
                         "recalled": bool(correct), "best_hits": max((d["hits"] for d in correct), default=0),
                         "found": correct[0]["asset"] if correct else None,
                         "found_speed_percent": correct[0]["speed_percent"] if correct else None,
                         "weak_correct": None if weak is None else (weak["asset"], weak["hits"], weak["windows"]),
                         "wrong": [(d["asset"], d["hits"]) for d in wrong]})
    table = []
    for low, high in BINS:
        cell = [r for r in rows if r["relative_percent"] is not None and low <= abs(r["relative_percent"]) < high]
        table.append({"relative_percent": f"{low}-{high}", "queries": len(cell),
                      "recalled": sum(r["recalled"] for r in cell)})
    misses = [r for r in rows if not r["recalled"]]
    summary["reports"][report_dir] = {
        "queries": len(rows), "recalled": sum(r["recalled"] for r in rows),
        "without_speed": sum(r["relative_percent"] is None for r in rows),
        "wrong_confident": sum(len(r["wrong"]) for r in rows),
        "wrong_list": [r for r in rows if r["wrong"]],
        "by_relative_speed": table,
        "misses": sorted(misses, key=lambda r: -abs(r["relative_percent"] or 0)),
        "rows": rows,
    }
    print(f"\n{report_dir}: {summary['reports'][report_dir]['recalled']}/{len(rows)} recalled, "
          f"{summary['reports'][report_dir]['wrong_confident']} wrong confident, "
          f"{summary['reports'][report_dir]['without_speed']} without a pair speed")
    for cell in table:
        if cell["queries"]:
            print(f"  |relative| {cell['relative_percent']:>9}%: {cell['recalled']}/{cell['queries']}")
    for r in misses[:40]:
        print(f"  missed {r['asset'][:60]} at {r['speed_percent']:+.0f}%: relative {r['relative_percent']:+.2f}%, "
              f"weaker correct detection {r['weak_correct']}")

# Clusters whose members differ by more than 4% in speed.
spreads = []
for members in clusters["duplicates"]:
    first = sorted(members)[0]
    speeds = speeds_from(first)
    known = [speeds[m] for m in members if m in speeds]
    spread = max(known) / min(known) - 1 if len(known) > 1 else 0.0
    spreads.append((spread, len(members), len(known), first))
summary["spread"] = {
    "clusters": len(spreads),
    "over_2_percent": sum(s > 0.02 for s, *_ in spreads),
    "over_4_percent": sum(s > 0.04 for s, *_ in spreads),
    "over_6_percent": sum(s > 0.06 for s, *_ in spreads),
    "largest": sorted(spreads, reverse=True)[:15],
    "members_without_speed": sum(n - k for _, n, k, _ in spreads),
}
print("\nclusters:", {k: v for k, v in summary["spread"].items() if k != "largest"})
for s, n, k, first in summary["spread"]["largest"][:8]:
    print(f"  spread {100 * s:.1f}% over {n} members: {first[:70]}")
json.dump(summary, open(output, "w"), indent=1)
