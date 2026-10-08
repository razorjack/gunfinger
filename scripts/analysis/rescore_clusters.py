"""Which wrong confident detections in saved harness reports a clusters
file explains: the detected file is in the same cluster as the recording
that was queried (sweeps) or credited (scans).

usage: python3 -I scripts/analysis/rescore_clusters.py CLUSTERS REPORT...

Prints one line per wrong confident detection with whether CLUSTERS
explains it, then the counts per report. With OUT set, writes the result
as JSON there.
"""

import json
import os
import sys


def clusters_of(path):
    with open(path) as file:
        clusters = json.load(file)
    cluster = {}
    for members in clusters["duplicates"]:
        for member in members:
            cluster[member] = frozenset(members)
    return lambda asset: cluster.get(asset, frozenset([asset]))


def sweep_wrong(report, cluster_of):
    for query in report["queries"]:
        recording = cluster_of(query["asset"])
        for detection in query["detections"]:
            if detection["confident"] and not detection["correct"]:
                yield {
                    "query": query["asset"],
                    "speed_percent": query["speed_percent"],
                    "held_out": query["held_out"],
                    "detected": detection["asset"],
                    "hits": detection["hits"],
                    "explained": detection["asset"] in recording,
                }


def scan_wrong(report, cluster_of):
    credited = set()
    for track in report["score"]["tracks"]:
        credited.update(found["asset"] for found in track.get("credited", []))
    for detection in report["score"]["wrong_identifications"]:
        joins = sorted(cluster_of(detection["asset"]) & credited)
        yield {
            "detected": detection["asset"],
            "start_seconds": round(detection["start_seconds"], 1),
            "hits": detection["hits"],
            "explained": bool(joins),
            "joins": joins,
        }


def main():
    cluster_of = clusters_of(sys.argv[1])
    result = {}
    for path in sys.argv[2:]:
        with open(path) as file:
            report = json.load(file)
        if "queries" in report:
            wrong = list(sweep_wrong(report, cluster_of))
        else:
            wrong = list(scan_wrong(report, cluster_of))
        explained = sum(1 for item in wrong if item["explained"])
        result[path] = {"wrong": len(wrong), "explained": explained, "detections": wrong}
        print(f"{path}: {len(wrong)} wrong confident, {explained} explained")
        for item in wrong:
            mark = "explained" if item["explained"] else "NOT explained"
            where = item.get("query", item.get("start_seconds"))
            print(f"  {mark}: {where} -> {item['detected']} ({item['hits']} hits)")
    if "OUT" in os.environ:
        with open(os.environ["OUT"], "w") as file:
            json.dump(result, file, indent=1)


if __name__ == "__main__":
    main()
