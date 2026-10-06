"""Summarise a robustness report: per condition and speed, the rows plus
hit statistics, the cause of each miss, and the strongest false matches on
held-out excerpts and on indexed excerpts other than a given related one."""
import collections, json, statistics, sys

report = json.load(open(sys.argv[1]))
related = sys.argv[2] if len(sys.argv) > 2 else None
groups = collections.defaultdict(list)
for query in report["queries"]:
    groups[(query["condition"], query["speed"])].append(query)
rows = []
for row in report["rows"]:
    queries = groups[(row["condition"], row["speed"])]
    indexed = [q for q in queries if not q["held_out"]]
    misses = [q for q in indexed if not q["confident"]]
    rows.append({
        **row,
        "median_best_hits": statistics.median(q["best_hits"] for q in indexed),
        "min_best_hits": min(q["best_hits"] for q in indexed),
        "misses_below_min_hits": sum(q["best_hits"] < 200 for q in misses),
        "misses_below_min_windows": sum(q["best_windows"] < 3 for q in misses),
        "held_out_strongest_hits": max(q["strongest_wrong_hits"] for q in queries if q["held_out"]),
        "unrelated_strongest_hits": max(q["strongest_wrong_hits"] for q in indexed if q["asset"] != related),
    })
json.dump({"seed": report["seed"], "related_excerpt_excluded": related, "rows": rows}, sys.stdout, indent=1)
print()
