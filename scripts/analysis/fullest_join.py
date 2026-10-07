"""Join `gunfinger-eval fullest` (each record's postings in the fullest 1%
of lists) with the NAS-scale scan and sweep reports of both matchers:
are records whose postings sit in the fullest lists ("break-heavy") the
false candidates, and does skipping those lists keep them out, or hide
the records themselves when they are played?

usage: fullest_join.py <fullest.json> <today's reports dir> <skip reports dir>
                       <tags.json> <output.json>
"""
import collections, glob, json, os, statistics, sys

fullest_file, today_dir, skip_dir, tags_file, output = sys.argv[1:6]
PREFIX = "second-library/"
fullest = json.load(open(fullest_file))
tags = json.load(open(tags_file))
share = {a["path"]: a["skipped"] / max(a["postings"], 1) for a in fullest["assets"]}
ranked = sorted(share, key=share.get, reverse=True)
top_count = len(ranked) // 100
break_heavy = set(ranked[:top_count])


def name(asset):
    fields = tags.get(asset[len(PREFIX):], {}) if asset.startswith(PREFIX) else {}
    if "artist" in fields and "title" in fields:
        return f"{fields['artist']} - {fields['title']}"
    return os.path.basename(asset)


def false_candidates(reports):
    counts = collections.Counter()
    for path in glob.glob(os.path.join(reports, "scan-*.json")):
        for d in json.load(open(path))["score"]["false_candidates"]:
            counts[d["asset"]] += 1
    for path in glob.glob(os.path.join(reports, "sweep-seed-*.json")):
        for q in json.load(open(path))["queries"]:
            for d in q["detections"]:
                if not d["correct"]:
                    counts[d["asset"]] += 1
    return counts


def own_hits(reports):
    """Each indexed sweep excerpt's strongest correct hits, by (seed, asset, speed)."""
    hits = {}
    for path in glob.glob(os.path.join(reports, "sweep-seed-*.json")):
        sweep = json.load(open(path))
        for q in sweep["queries"]:
            if not q["held_out"]:
                key = (sweep["seed"], q["asset"], q["speed_percent"], round(q["start_seconds"], 3))
                hits[key] = max((d["hits"] for d in q["detections"] if d["correct"]), default=0)
    return hits


def rank_correlation(xs, ys):
    def ranks(values):
        order = sorted(range(len(values)), key=values.__getitem__)
        result = [0.0] * len(values)
        for rank, index in enumerate(order):
            result[index] = rank
        return result
    rx, ry = ranks(xs), ranks(ys)
    mx, my = statistics.mean(rx), statistics.mean(ry)
    cov = sum((a - mx) * (b - my) for a, b in zip(rx, ry))
    return cov / (sum((a - mx) ** 2 for a in rx) * sum((b - my) ** 2 for b in ry)) ** 0.5


today, skip = false_candidates(today_dir), false_candidates(skip_dir)
assets = list(share)
summary = {
    "fullest": {k: fullest[k] for k in ("share", "lists", "skipped_lists", "shortest_skipped", "postings", "skipped_postings")},
    "skipped_share_of_postings": {
        "median": statistics.median(share.values()),
        "p90": statistics.quantiles(share.values(), n=10)[-1],
        "p99": statistics.quantiles(share.values(), n=100)[-1],
        "max": max(share.values()),
    },
    "break_heavy": {"definition": "the 1% of records with the highest share of postings in the fullest lists",
                    "records": top_count, "threshold_share": share[ranked[top_count - 1]]},
    "top_records": [{"asset": a, "name": name(a), "share": round(share[a], 3), "false_today": today[a], "false_skip": skip[a]}
                    for a in ranked[:30]],
    "false_candidate_appearances": {
        "today": sum(today.values()), "skip": sum(skip.values()),
        "today_break_heavy": sum(today[a] for a in break_heavy), "skip_break_heavy": sum(skip[a] for a in break_heavy),
        "distinct_today": len(today), "distinct_skip": len(skip),
    },
    "rank_correlation_share_vs_false_today": rank_correlation([share[a] for a in assets], [today[a] for a in assets]),
    "rank_correlation_share_vs_false_skip": rank_correlation([share[a] for a in assets], [skip[a] for a in assets]),
    "most_frequent_false_today": [{"asset": a, "name": name(a), "times": n, "share": round(share.get(a, 0), 3), "skip_times": skip[a]}
                                  for a, n in today.most_common(20)],
    "most_frequent_false_skip": [{"asset": a, "name": name(a), "times": n, "share": round(share.get(a, 0), 3), "today_times": today[a]}
                                 for a, n in skip.most_common(20)],
}
hits_today, hits_skip = own_hits(today_dir), own_hits(skip_dir)
common = sorted(set(hits_today) & set(hits_skip))
if common:
    by_share = sorted(common, key=lambda key: -share.get(key[1], 0))
    heaviest = by_share[: max(1, len(by_share) // 10)]
    summary["own_excerpts"] = {
        "excerpts_in_both": len(common),
        "missed_today": sum(hits_today[k] < 200 for k in common),
        "missed_skip": sum(hits_skip[k] < 240 for k in common),
        "heaviest_tenth": {
            "excerpts": len(heaviest),
            "median_share": statistics.median(share.get(k[1], 0) for k in heaviest),
            "median_hits_today": statistics.median(hits_today[k] for k in heaviest),
            "median_hits_skip": statistics.median(hits_skip[k] for k in heaviest),
            "below_rule_skip": sum(hits_skip[k] < 240 for k in heaviest),
        },
        "rank_correlation_share_vs_skip_over_today": rank_correlation(
            [share.get(k[1], 0) for k in common], [hits_skip[k] / max(hits_today[k], 1) for k in common]),
    }
json.dump(summary, open(output, "w"), indent=1)
print(json.dumps({k: v for k, v in summary.items() if k not in ("top_records", "most_frequent_false_today", "most_frequent_false_skip")}, indent=1))
for row in summary["top_records"][:12]:
    print(f"  {row['share']:.3f}  today {row['false_today']:5d}  skip {row['false_skip']:4d}  {row['name']}")
print("most frequent false candidates, today's matcher:")
for row in summary["most_frequent_false_today"][:12]:
    print(f"  {row['times']:5d}  share {row['share']:.3f}  skip {row['skip_times']:4d}  {row['name']}")
