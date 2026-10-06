"""Null for summed play evidence: in scan reports, the strongest sums of
same-asset detection groups (gap <= 90 s, speeds within a tolerance, each
segment >= a minimum) that are credited to no track. Synthetic assets in
scale scans are always false. Experiment 0011."""
import json, sys

GAP = 90.0

def false_sums(report, min_hits, tolerance):
    credited = {(c["asset"], c["start_seconds"]) for t in report["score"]["tracks"] for c in t["credited"]}
    by_asset = {}
    for d in report["detections"]:
        if d["hits"] >= min_hits:
            by_asset.setdefault(d["asset"], []).append(d)
    sums = []
    for asset, ds in by_asset.items():
        ds.sort(key=lambda d: d["start_seconds"])
        group = [ds[0]]
        for d in ds[1:]:
            end = max(x["end_seconds"] for x in group)
            if d["start_seconds"] - end <= GAP and abs(d["speed"] - group[-1]["speed"]) <= tolerance:
                group.append(d)
            else:
                sums.append(group)
                group = [d]
        sums.append(group)
    out = []
    for g in sums:
        if any((d["asset"], d["start_seconds"]) in credited for d in g):
            continue
        out.append((sum(d["hits"] for d in g), len(g), sum(d["windows"] for d in g), g[0]["asset"]))
    out.sort(reverse=True)
    return out

for path in sys.argv[1:]:
    report = json.load(open(path))
    single = max((d["hits"] for d in report["detections"] if d["asset"].startswith("synthetic/")), default=0)
    print(path.split("/")[-1], "strongest single synthetic detection:", single)
    for min_hits in (0, 20, 30, 60):
        for tolerance in (0.002, 0.005):
            sums = [s for s in false_sums(report, min_hits, tolerance) if s[1] > 1]
            print(f"  segments >= {min_hits}, speeds within {tolerance:.1%}: {len(sums)} false multi-segment groups; strongest {sums[:3]}")
