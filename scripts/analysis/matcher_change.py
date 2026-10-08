"""What a change of matcher did to the development scans: for each scan of
the standard evaluation, each referenced track's strongest credited
detection before and after (hits, windows, ratio), and the detections
matching no track by level. Reads two report directories.

usage: matcher_change.py <before reports dir> <after reports dir>
"""
import json, os, sys

before_dir, after_dir = sys.argv[1], sys.argv[2]
names = ["scan-stakka-skynet-knowledge.json",
         "scan-stakka-skynet-knowledge-leave-out-3-seed-2026.json",
         "scan-stakka-skynet-knowledge-leave-out-11-seed-2026.json"]


def best(track):
    credited = track["credited"] + track["credited_through_cluster"]
    return max(credited, key=lambda d: d["hits"]) if credited else None


def unmatched(report):
    credited = set()
    for track in report["score"]["tracks"]:
        for d in track["credited"] + track["credited_through_cluster"]:
            credited.add((d["asset"], round(d["start_seconds"], 1)))
    rest = [d for d in report["detections"] if (d["asset"], round(d["start_seconds"], 1)) not in credited]
    return rest


summary = {}
for name in names:
    a = json.load(open(os.path.join(before_dir, name)))
    b = json.load(open(os.path.join(after_dir, name)))
    rows = []
    for ta, tb in zip(a["score"]["tracks"], b["score"]["tracks"]):
        da, db = best(ta), best(tb)
        if da and db:
            rows.append({"track": ta["label"].strip(), "before": [da["hits"], da["windows"]],
                         "after": [db["hits"], db["windows"]], "ratio": round(db["hits"] / da["hits"], 3)})
    ua, ub = unmatched(a), unmatched(b)
    level = lambda ds, lo: sum(1 for d in ds if d["hits"] >= lo)
    summary[name] = {
        "tracks": rows,
        "unmatched_before": {"all": len(ua), "30+": level(ua, 30), "60+": level(ua, 60), "strongest": max((d["hits"] for d in ua), default=0)},
        "unmatched_after": {"all": len(ub), "30+": level(ub, 30), "60+": level(ub, 60), "strongest": max((d["hits"] for d in ub), default=0)},
        "wall_before": round(a["wall_seconds"], 1), "wall_after": round(b["wall_seconds"], 1),
    }
    ratios = [r["ratio"] for r in rows]
    print(name, "ratios", min(ratios) if ratios else None, "-", max(ratios) if ratios else None,
          "unmatched", summary[name]["unmatched_before"], "->", summary[name]["unmatched_after"],
          "wall", summary[name]["wall_before"], "->", summary[name]["wall_after"])
    for r in rows:
        print("   ", r["track"][:50], r["before"], "->", r["after"], r["ratio"])
json.dump(summary, open(os.environ.get("OUT", "/dev/null"), "w"), indent=1)
