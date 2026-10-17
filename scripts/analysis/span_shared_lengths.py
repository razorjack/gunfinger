"""Session 27: where the variants change the shared panels' outcomes,
by query length (offline, from `span_rule.py shared` counts and the
variant build's reports).

Today's rule counts 10 s windows from the query's start, so a query file
of at most 20 s holds 2 windows and can never be confident under it. For
each variant this prints the changed queries by the windows their file
holds, and for each new wrong confident detection whether today's rule
already names the same file confidently in another query of the same
scenario and index.

usage: span_shared_lengths.py COUNTS.json REPORT.json [OUT.json]
"""
import json
import math
import sys
from collections import Counter, defaultdict

counts_path, report_path = sys.argv[1], sys.argv[2]
for path in sys.argv[1:]:
    if "ed-rush" in path:
        raise SystemExit(f"refused: {path} names the held-out test set")
counts = json.load(open(counts_path))
report = json.load(open(report_path))
results = {(r["query"], r["index"]): r for r in report["results"]}


def windows_held(query, index):
    return math.ceil(results[(query, index)]["seconds"] / 10.0 - 1e-9)


# Today's confident wrong files per (scenario, index), from today's rule.
today_wrong = defaultdict(set)
for r in report["results"]:
    for d in r["detections"]:
        if d["hits"] >= 240 and d["windows"] >= 3:
            today_wrong[(r["scenario"], r["index"])].add((d["asset"], r["query"]))

out = {"counts": counts_path.split("/")[-1], "rules": {}}
for rule, v in counts["rules"].items():
    by_windows = Counter()
    for c in v["changed"]:
        by_windows[(windows_held(c["query"], c["index"]), c["today"], c["variant"])] += 1
    new = []
    for w in v["new_wrong_confident"]:
        scenario = results[(w["query"], w["index"])]["scenario"]
        already = sorted({q for asset, q in today_wrong[(scenario, w["index"])]
                          if asset == w["asset"] and q != w["query"]})
        new.append({"query": w["query"], "index": w["index"],
                    "query_seconds": results[(w["query"], w["index"])]["seconds"],
                    "windows_held": windows_held(w["query"], w["index"]),
                    "same_file_confident_today_in": already})
    out["rules"][rule] = {
        "changed_by_windows_held": [{"windows_held": k[0], "today": k[1], "variant": k[2], "queries": n}
                                    for k, n in sorted(by_windows.items())],
        "new_wrong_confident": new}
    if not v["changed"] and not new:
        continue
    print(f"{rule}:")
    for k, n in sorted(by_windows.items()):
        print(f"  {n:3} changed queries in files of {k[0]} windows: {k[1]} -> {k[2]}")
    for w in new:
        print(f"  new wrong {w['query']} ({w['index']}, {w['query_seconds']} s, {w['windows_held']} windows); "
              f"same file confident today in {len(w['same_file_confident_today_in'])} other queries")
if len(sys.argv) > 3:
    json.dump(out, open(sys.argv[3], "w"), indent=1)
