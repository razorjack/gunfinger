"""Summarise the scan and sweep reports of one reports directory (one
matcher at one library size): scores, wrong identifications, possible
plays, the weakest identifying detection, the strongest false candidates
with their tags, false candidates per query, and which assets are false
candidates most often.

usage: nas_scale_summary.py <reports dir> <tags.json> <output.json> [rule hits]
"""
import collections, glob, json, os, statistics, sys

reports, tags_file, output = sys.argv[1:4]
rule = int(sys.argv[4]) if len(sys.argv) > 4 else 200
PREFIX = "second-library/"
tags = json.load(open(tags_file))


def name(asset):
    fields = tags.get(asset[len(PREFIX):], {}) if asset.startswith(PREFIX) else {}
    if "artist" in fields and "title" in fields:
        return f"{fields['artist']} - {fields['title']}"
    return os.path.basename(asset)


def candidate(detection, where):
    return {"hits": detection["hits"], "windows": detection["windows"], "asset": detection["asset"],
            "name": name(detection["asset"]), "where": where}


summary = {"reports": reports, "rule_hits": rule, "scans": {}, "sweeps": {}}
false_by_asset = collections.Counter()

for path in sorted(glob.glob(os.path.join(reports, "scan-*.json"))):
    scan = json.load(open(path))
    score = scan["score"]
    # As calibrate: each referenced track's strongest candidate, unless left out.
    identifying = [(t["strongest_candidate"] or {"hits": 0})["hits"] for t in score["tracks"]
                   if t["referenced"] and t["label"] not in scan["left_out_tracks"]]
    false = score["false_candidates"]
    for d in false:
        false_by_asset[d["asset"]] += 1
    strongest = sorted(false, key=lambda d: -d["hits"])[:8]
    summary["scans"][os.path.basename(path)] = {
        "identified": score["identified"], "referenced": score["referenced"], "wrong": score["wrong"],
        "possible": score["possible"], "unmatched_possible": len(score["unmatched_possible"]),
        "wall_seconds": scan.get("wall_seconds"),
        "wrong_identifications": [candidate(d, round(d["start_seconds"])) for d in score["wrong_identifications"]],
        "unmatched_possible_plays": [{"asset": p["asset"], "name": name(p["asset"]), "start": round(p["start_seconds"]),
                                      "end": round(p["end_seconds"])} for p in score["unmatched_possible"]],
        "credited_through_cluster": sorted({d["asset"] for t in score["tracks"] for d in t["credited_through_cluster"]}),
        "weakest_identifying": min(identifying, default=None),
        "false_candidates": len(false),
        "false_30": sum(d["hits"] >= 30 for d in false),
        "false_60": sum(d["hits"] >= 60 for d in false),
        "strongest_false": [candidate(d, round(d["start_seconds"])) for d in strongest],
        "detections": len(scan["detections"]),
    }

for path in sorted(glob.glob(os.path.join(reports, "sweep-seed-*.json"))):
    sweep = json.load(open(path))
    queries = sweep["queries"]
    indexed = [q for q in queries if not q["held_out"]]
    # As calibrate: each indexed query's strongest correct detection.
    identifying = [max((d["hits"] for d in q["detections"] if d["correct"]), default=0) for q in indexed]
    false = [(d, q) for q in queries for d in q["detections"] if not d["correct"]]
    for d, _ in false:
        false_by_asset[d["asset"]] += 1
    per_query = [sum(not d["correct"] for d in q["detections"]) for q in queries]
    strongest = sorted(false, key=lambda pair: -pair[0]["hits"])[:8]
    not_indexed = [d["hits"] for q in queries if q["held_out"] for d in q["detections"]]
    summary["sweeps"][os.path.basename(path)] = {
        "recalled": sum(row["recalled"] for row in sweep["per_speed"]),
        "indexed_excerpts": sum(row["indexed_excerpts"] for row in sweep["per_speed"]),
        "wrong": sum(row["wrong_answers"] for row in sweep["per_speed"]),
        "max_speed_error_percent": max(row["max_speed_error_percent"] for row in sweep["per_speed"]),
        "indexed_assets": sweep["indexed_assets"],
        "weakest_identifying": min(identifying, default=None),
        "false_candidates": len(false),
        "false_per_query_mean": statistics.mean(per_query),
        "false_per_query_max": max(per_query),
        "false_30": sum(d["hits"] >= 30 for d, _ in false),
        "false_60": sum(d["hits"] >= 60 for d, _ in false),
        "strongest_not_indexed": max(not_indexed, default=0),
        "strongest_false": [candidate(d, f"{q['asset']} at {q['speed_percent']:+.0f}%") for d, q in strongest],
    }

summary["most_frequent_false_assets"] = [{"asset": a, "name": name(a), "times": n} for a, n in false_by_asset.most_common(30)]
json.dump(summary, open(output, "w"), indent=1)
for kind in ("scans", "sweeps"):
    for report, row in summary[kind].items():
        brief = {k: v for k, v in row.items() if k not in ("strongest_false", "unmatched_possible_plays", "wrong_identifications", "credited_through_cluster")}
        print(report, json.dumps(brief))
        for item in row.get("wrong_identifications", []):
            print("   WRONG", item)
        for item in row["strongest_false"][:3]:
            print("   false", item["hits"], item["windows"], item["name"], "|", item["where"])
print("most frequent false candidates:")
for item in summary["most_frequent_false_assets"][:10]:
    print("  ", item["times"], item["name"])
