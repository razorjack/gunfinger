"""Two `gunfinger-eval shared` reports of the same plan compared query by
query (session 21, step 6: a variant of the matcher against today's): for
each query and index, the confident files of the played recording (right)
and the other confident files (wrong titles; the blend partner is set
aside, as in shared_outcomes.py), added or removed by the variant.

usage: shared_compare.py <today's report.json> <variant's report.json> <out.json>
"""
import json
import sys
from collections import Counter


def confident(result):
    found = [d for d in result["detections"] if d["level"] == "confident" and d["role"] != "partner"]
    right = {d["asset"] for d in found if d["role"] == "played"}
    wrong = {d["asset"] for d in found if d["role"] != "played"}
    return right, wrong


def outcome(result):
    right, wrong = confident(result)
    if result.get("error"):
        return "not rendered"
    if wrong:
        return "wrong confident"
    if right:
        return "right confident"
    if any(d["level"] == "possible" and d["role"] != "partner" for d in result["detections"]):
        return "possible only"
    return "nothing"


def main(today_path, variant_path, out_path):
    today = {(r["query"], r["index"]): r for r in json.load(open(today_path, encoding="utf-8"))["results"]}
    variant = {(r["query"], r["index"]): r for r in json.load(open(variant_path, encoding="utf-8"))["results"]}
    if today.keys() != variant.keys():
        sys.exit(f"the reports hold different queries: {len(today.keys() ^ variant.keys())} differ")
    rows = []
    changes = Counter()
    for key in sorted(today):
        a, b = today[key], variant[key]
        right_a, wrong_a = confident(a)
        right_b, wrong_b = confident(b)
        row = {"query": key[0], "index": key[1], "today": outcome(a), "variant": outcome(b),
               "wrong added": sorted(wrong_b - wrong_a), "wrong removed": sorted(wrong_a - wrong_b),
               "right lost": sorted(right_a - right_b), "right gained": sorted(right_b - right_a)}
        rows.append(row)
        changes[(key[1], row["today"], row["variant"])] += 1
        for name in ("wrong added", "wrong removed", "right lost", "right gained"):
            changes[(key[1], name)] += len(row[name])
    print(f"{len(rows)} query results compared")
    for key, count in sorted(changes.items()):
        if len(key) == 3 and key[1] == key[2]:
            continue
        print(f"  {' / '.join(key)}: {count}")
    for row in rows:
        if row["today"] != row["variant"] or row["wrong added"] or row["wrong removed"] or row["right lost"]:
            print(f"  {row['query']} [{row['index']}] {row['today']} -> {row['variant']}"
                  + "".join(f"; {name}: {', '.join(f.rsplit('/', 1)[-1][:50] for f in row[name])}"
                            for name in ("wrong added", "wrong removed", "right lost", "right gained")
                            if row[name]))
    json.dump({"rows": rows}, open(out_path, "w", encoding="utf-8"), indent=1, ensure_ascii=False)


if __name__ == "__main__":
    main(*sys.argv[1:])
