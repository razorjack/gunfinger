"""Each C-index report restricted to the queries of (a)'s 20 observable
sources, so that (a)'s conditions compare with the same queries (with
the median of each query's strongest own-cluster detection); and per
report, the strongest own-cluster detection's hits over its observable
indexed queries (median, 10th and 90th percentile). Session 28,
experiment 0145.

usage: recall_subsets.py REPORT_DIR [OUT.json]
"""
import json
import statistics
import sys

D = sys.argv[1].rstrip("/") + "/"
own = lambda q: [d for d in q["detections"] if d["correct"]]
observable = {
    q["asset"]
    for q in json.load(open(D + "recall-seed-2026-other-rips.json"))["queries"]
    if not q["held_out"] and not q.get("unobservable")
}
result = {"observable_sources": len(observable)}
for name in ["", "-other-rips", "-damaged", "-other-rips-damaged", "-brief", "-other-rips-brief",
             "-damaged-brief", "-other-rips-damaged-brief"]:
    try:
        queries = json.load(open(D + f"recall-seed-2026{name}.json"))["queries"]
    except FileNotFoundError:
        continue
    rows = {}
    for q in queries:
        if q["held_out"] or q["asset"] not in observable:
            continue
        key = f"{q['brief']['seconds']:.0f} s" if "brief" in q else "all"
        row = rows.setdefault(key, {"queries": 0, "confident": 0, "possible_or_better": 0, "hits": []})
        row["queries"] += 1
        row["confident"] += any(d["confident"] for d in own(q))
        row["possible_or_better"] += any(d["confident"] or d["hits"] >= 60 for d in own(q))
        row["hits"].append(max([d["hits"] for d in own(q)], default=0))
    for row in rows.values():
        row["median_strongest_own_hits"] = statistics.median(row.pop("hits"))
    best = sorted(
        max([d["hits"] for d in own(q)], default=0)
        for q in queries
        if not q["held_out"] and not q.get("unobservable")
    )
    n = len(best)
    hits = {"queries": n, "median": statistics.median(best), "p10": best[n // 10], "p90": best[9 * n // 10]}
    result["recall-seed-2026" + name] = {"observable_sources_only": rows, "strongest_own_hits": hits}
    print(name or "clean", rows, hits)
if len(sys.argv) > 2:
    json.dump(result, open(sys.argv[2], "w"), indent=1)
