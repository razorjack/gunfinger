"""2026-10-22 (session 38): a matching variant against the default, report
against report, for the criteria of brief 38 (08:26).

  shared DEFAULT VARIANT   `shared` reports: per query (scenario, kind,
      length, index), confident detections by role; wrong are those of
      role `other`. Prints the counts, every confident detection of role
      `other` the variant adds (by query and file, counting an added
      segment of a file the default already names), and every query whose
      confident detection of the played recording (or, in a blend, the
      partner) the variant loses
  sweep DEFAULT VARIANT    `sweep` reports: wrong confident detections
      and indexed queries confident on their own cluster in each, queries
      that gain a wrong one or lose a right one, and queries whose
      detections differ
  answers DEFAULT VARIANT  `mixes` or `scan` reports: each indexed play
      (or referenced track) confident (identified) by default and not
      with the variant, and the reverse

usage: python3 -I scripts/analysis/variant_compare.py shared|sweep|answers DEFAULT VARIANT
"""

import collections
import json
import sys


def query_key(q):
    return (q["scenario"], q["kind"], q["seconds"], q["index"])


def shared(default_path, variant_path):
    default, variant = (json.load(open(p)) for p in (default_path, variant_path))
    by_key = lambda r: {query_key(q): q for q in r["results"] if not q.get("error")}
    d_queries, v_queries = by_key(default), by_key(variant)
    totals = collections.Counter()
    added_wrong, lost_right = [], []
    for key in sorted(set(d_queries) | set(v_queries), key=str):
        d, v = d_queries.get(key), v_queries.get(key)
        if d is None or v is None:
            print("   query in one report only:", key)
            continue
        conf = lambda q: [x for x in q["detections"] if x["level"] == "confident"]
        dc, vc = conf(d), conf(v)
        for role, n in collections.Counter(x["role"] for x in dc).items():
            totals[(role, "default")] += n
        for role, n in collections.Counter(x["role"] for x in vc).items():
            totals[(role, "variant")] += n
        d_other = collections.Counter(x["asset"] for x in dc if x["role"] == "other")
        v_other = collections.Counter(x["asset"] for x in vc if x["role"] == "other")
        for asset in v_other:
            if v_other[asset] > d_other[asset]:
                added_wrong.append((key, asset, d_other[asset], v_other[asset]))
        for role in ("played", "partner"):
            if any(x["role"] == role for x in dc) and not any(x["role"] == role for x in vc):
                lost_right.append((key, role))
    print(f"queries {len(d_queries)} and {len(v_queries)}")
    for role in sorted({r for r, _ in totals}):
        print(f"  confident detections, role {role}: default {totals[(role, 'default')]}, variant {totals[(role, 'variant')]}")
    print(f"  confident segments of role other added (query, file): {len(added_wrong)}")
    for row in added_wrong:
        print("    added wrong:", row)
    print(f"  queries losing a confident played or partner detection: {len(lost_right)}")
    for row in lost_right:
        print("    lost right:", row)


def sweep(default_path, variant_path):
    default, variant = (json.load(open(p)) for p in (default_path, variant_path))
    queries = lambda r: {(q["asset"], q["speed_percent"], q["start_seconds"]): q for q in r["queries"]}
    dq, vq = queries(default), queries(variant)
    wrong = lambda q: sum(1 for d in q["detections"] if d["confident"] and not d["correct"])
    right = lambda q: not q["held_out"] and any(d["confident"] and d["correct"] for d in q["detections"])
    print(
        f"queries {len(dq)} and {len(vq)}; wrong confident detections {sum(map(wrong, dq.values()))} ->"
        f" {sum(map(wrong, vq.values()))}; right {sum(map(right, dq.values()))} -> {sum(map(right, vq.values()))}"
    )
    for key in sorted(dq, key=str):
        if key not in vq:
            print("   query in the default only:", key)
            continue
        if wrong(vq[key]) > wrong(dq[key]):
            print("   gains a wrong confident detection:", key)
        if right(dq[key]) and not right(vq[key]):
            print("   loses a right answer:", key)
        if vq[key]["detections"] != dq[key]["detections"]:
            print("   detections differ:", key)


def answers(default_path, variant_path):
    def right(report):
        if "mixes" in report:
            return {
                (r["mix"]["name"], i)
                for r in report["mixes"]
                for i, p in enumerate(r["score"]["plays"])
                if not p["truth"]["held_out"] and p["level"] == "confident"
            }
        return {t["label"] for t in report["score"]["tracks"] if t["identified"]}

    default, variant = (right(json.load(open(p))) for p in (default_path, variant_path))
    print(f"{variant_path}: right {len(default)} -> {len(variant)}; lost {sorted(default - variant)}; gained {sorted(variant - default)}")


def main():
    command, default_path, variant_path = sys.argv[1:4]
    {"shared": shared, "sweep": sweep, "answers": answers}[command](default_path, variant_path)


if __name__ == "__main__":
    main()
