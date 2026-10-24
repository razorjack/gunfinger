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
      with the variant, and the reverse; for `mixes`, also the wrong
      confident detections in each and every one the variant adds (by
      mix and file, counting an added segment of a file the default
      already names)
  tracklists DEFAULT VARIANT  `tracklist_score.py score` outputs of the
      same mixes (session 45): wrong confident plays added (by mix and
      file), truth plays confident by default and not with the variant,
      truth plays printed by default and not with the variant, and the
      printed entries each has that the other lacks

usage: python3 -I scripts/analysis/variant_compare.py shared|sweep|answers|tracklists DEFAULT VARIANT
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
    # Beside the criteria: each confident detection one report has and the
    # other lacks; for an added one, its speed difference (%) to the
    # nearest-speed stronger overlapping detection of its file.
    added_confident, removed_confident = [], []
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
        ident = lambda x: (x["asset"], x["start_seconds"], x["end_seconds"], x["hits"])  # noqa: E731
        d_ids = {ident(x) for x in d["detections"]}
        v_ids = {ident(x) for x in v["detections"]}
        for x in vc:
            if ident(x) not in d_ids:
                stronger = [y for y in v["detections"] if y["asset"] == x["asset"] and y["hits"] > x["hits"]
                            and y["start_seconds"] < x["end_seconds"] and x["start_seconds"] < y["end_seconds"]]
                gap = min((abs(x["speed"] / y["speed"] - 1) for y in stronger), default=None)
                added_confident.append((key, x["role"], x["asset"], round(x["start_seconds"], 1),
                                        round(x["end_seconds"], 1), x["hits"],
                                        None if gap is None else round(100 * gap, 3)))
        for x in dc:
            if ident(x) not in v_ids:
                removed_confident.append((key, x["role"], x["asset"], round(x["start_seconds"], 1),
                                          round(x["end_seconds"], 1), x["hits"]))
    print(f"queries {len(d_queries)} and {len(v_queries)}")
    for role in sorted({r for r, _ in totals}):
        print(f"  confident detections, role {role}: default {totals[(role, 'default')]}, variant {totals[(role, 'variant')]}")
    print(f"  confident segments of role other added (query, file): {len(added_wrong)}")
    for row in added_wrong:
        print("    added wrong:", row)
    print(f"  queries losing a confident played or partner detection: {len(lost_right)}")
    for row in lost_right:
        print("    lost right:", row)
    print(f"  confident detections added: {len(added_confident)}; removed: {len(removed_confident)}")
    for row in added_confident:
        print("    added confident (query, role, file, start, end, hits, speed gap %):", row)
    for row in removed_confident:
        print("    removed confident:", row)


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

    reports = [json.load(open(p)) for p in (default_path, variant_path)]
    default, variant = (right(report) for report in reports)
    print(f"{variant_path}: right {len(default)} -> {len(variant)}; lost {sorted(default - variant)}; gained {sorted(variant - default)}")
    if "mixes" in reports[0]:
        wrong = [
            {r["mix"]["name"]: r["score"]["wrong_confident"] for r in report["mixes"]}
            for report in reports
        ]
        print(f"  wrong confident detections {sum(map(len, wrong[0].values()))} -> {sum(map(len, wrong[1].values()))}")
        for mix in sorted(wrong[1]):
            before = collections.Counter(x["asset"] for x in wrong[0].get(mix, []))
            after = collections.Counter(x["asset"] for x in wrong[1][mix])
            for asset in sorted(after):
                if after[asset] > before[asset]:
                    print("    added wrong:", mix, asset, before[asset], "->", after[asset],
                          [(round(x["start_seconds"], 1), round(x["end_seconds"], 1), x["hits"])
                           for x in wrong[1][mix] if x["asset"] == asset])


def tracklists(default_path, variant_path):
    rows = [{row["mix"]: row for row in json.load(open(p))["rows"]} for p in (default_path, variant_path)]
    d_rows, v_rows = rows
    count = lambda rs, f: sum(f(row) for row in rs.values())  # noqa: E731
    print(f"mixes {len(d_rows)} and {len(v_rows)}; entries printed "
          f"{count(d_rows, lambda r: r['entries_printed'])} -> {count(v_rows, lambda r: r['entries_printed'])}; "
          f"wrong confident plays {count(d_rows, lambda r: len(r['wrong_confident']))} -> "
          f"{count(v_rows, lambda r: len(r['wrong_confident']))}")
    entry = lambda e: (int(e["start_seconds"]), e["name"], e["confidence"], e.get("extra"), e.get("merged"))  # noqa: E731
    for mix in sorted(d_rows):
        d, v = d_rows[mix], v_rows[mix]
        before = collections.Counter(d["wrong_confident"])
        after = collections.Counter(v["wrong_confident"])
        for asset in sorted(after):
            if after[asset] > before[asset]:
                print("  added wrong:", mix, asset, before[asset], "->", after[asset])
        if len(d["truth"]) != len(v["truth"]):
            raise SystemExit(f"{mix}: the two scores hold different truth plays")
        for dt, vt in zip(d["truth"], v["truth"]):
            if dt["today_confident"] and not vt["today_confident"]:
                print("  lost confident:", mix, dt["play"], dt["scenario"], dt["side"], dt["asset"])
            if dt["outcome"].startswith("printed") and not vt["outcome"].startswith("printed"):
                print("  lost printed:", mix, dt["play"], dt["scenario"], dt["side"], dt["asset"], dt["outcome"], "->", vt["outcome"])
            elif dt["outcome"] != vt["outcome"]:
                print("  outcome changed:", mix, dt["play"], dt["scenario"], dt["side"], dt["asset"], dt["outcome"], "->", vt["outcome"])
        d_entries = collections.Counter(map(entry, d["entries"]))
        v_entries = collections.Counter(map(entry, v["entries"]))
        for e in sorted((v_entries - d_entries).elements(), key=str):
            print("  entry only with the variant:", mix, e)
        for e in sorted((d_entries - v_entries).elements(), key=str):
            print("  entry only by default:", mix, e)


def main():
    command, default_path, variant_path = sys.argv[1:4]
    {"shared": shared, "sweep": sweep, "answers": answers, "tracklists": tracklists}[command](default_path, variant_path)


if __name__ == "__main__":
    main()
