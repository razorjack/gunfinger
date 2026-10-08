"""Coverage against supported time for pairs of files (roadmap, "A minimum
span instead of 3 windows" and "Clustering precision"), from a
`gunfinger-eval pair` report.

Coverage is the span from the first to the last hit of the clusters'
alignment, as a share of the shorter file (from the clusters report).
At the fitted speed `pair` splits that evidence into alignments at their
own offsets (edits, drift); supported time is the share of the shorter
file's seconds that hold an aligned hit, summed over those alignments.
The sum counts a second twice where two alignments cover the same part
of the shorter file; `overlap` gives that part. A groups file names
which pairs are sparse joins, borderline pairs and control joins.

usage: supported_time.py <output.json> <pairs report.json> <groups.json> <clusters.json>
"""
import json
import statistics
import sys

output, report_file, groups_file, clusters_file = sys.argv[1:]
report = json.load(open(report_file))
groups = json.load(open(groups_file))
group_of = {}
for group, pairs in groups.items():
    for pair in pairs:
        group_of[(pair[0], pair[1])] = group
clustered = {}
for p in json.load(open(clusters_file))["pairs"]:
    clustered[(p["query"], p["found"])] = p
    clustered.setdefault((p["found"], p["query"]), p)


def overlap_seconds(intervals):
    """Seconds covered by more than one interval."""
    events = sorted([(a, 1) for a, _ in intervals] + [(b, -1) for _, b in intervals])
    depth, last, twice = 0, None, 0.0
    for at, step in events:
        if depth > 1 and last is not None:
            twice += at - last
        depth += step
        last = at
    return twice


rows = []
for pair in report:
    key = (pair["query"], pair["found"])
    shorter = min(pair["query_seconds"], pair["found_seconds"])
    query_is_shorter = pair["query_seconds"] <= pair["found_seconds"]
    fitted = [a for s in pair["searches"] for a in s["alignments"] if a.get("supported") is not None]
    cluster_pair = clustered.get(key)
    intervals = [(a["query_start_seconds"], a["query_end_seconds"]) if query_is_shorter
                 else (a["found_start_seconds"], a["found_end_seconds"]) for a in fitted]
    rows.append({
        "group": group_of.get(key), "query": pair["query"], "found": pair["found"],
        "shorter_seconds": shorter,
        "clusters_coverage": cluster_pair["coverage"] if cluster_pair else None,
        "clusters_hits": cluster_pair["hits"] if cluster_pair else None,
        "fitted_alignments": len(fitted),
        "fitted_hits": sum(a["hits"] for a in fitted),
        "fitted_coverage": sum(a["coverage"] for a in fitted),
        "supported": sum(a["supported"] for a in fitted),
        "overlap": overlap_seconds(intervals) / shorter,
        "strongest": max(fitted, key=lambda a: a["hits"]) if fitted else None,
    })

summary = {}
for group in groups:
    members = [r for r in rows if r["group"] == group]
    if not members:
        continue
    summary[group] = {
        "pairs": len(members),
        "clusters_coverage_median": statistics.median(r["clusters_coverage"] for r in members if r["clusters_coverage"] is not None),
        "supported_median": statistics.median(r["supported"] for r in members),
        "supported_range": [min(r["supported"] for r in members), max(r["supported"] for r in members)],
    }
json.dump({"summary": summary, "rows": rows}, open(output, "w"), indent=1)
for group, s in summary.items():
    print(f"{group}: {s}")
for r in sorted(rows, key=lambda r: (str(r["group"]), -(r["clusters_coverage"] or 0))):
    coverage = "-" if r["clusters_coverage"] is None else f"{r['clusters_coverage']:.3f}"
    overlap = f" (overlap {r['overlap']:.3f})" if r["overlap"] > 0.005 else ""
    print(f"  {r['group']:10} clusters {coverage:>5} ({r['clusters_hits']} hits)  fitted: {r['fitted_alignments']:2} alignments "
          f"{r['fitted_hits']:6} hits, coverage {r['fitted_coverage']:.3f}, supported {r['supported']:.3f}{overlap}"
          f"  {r['query'][-38:]} | {r['found'][-38:]}")
