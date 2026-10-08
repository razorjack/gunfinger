"""The peak verifier's measures (`gunfinger-eval --verify`) by group:
identifying detections, false candidates from related recordings (the
same name, a version, the same artist; `labels.py`) and false candidates
from unrelated ones. Reads sweep, scan, robust, mixes and grid reports
written with `--verify`, and asks whether, at equal hits, a verifier
measure separates identifying from false candidates better than hits do.

Per detection and tolerance: `share` (reference peaks found over those in
the span), `support` (share of 1 s slices with a found peak), and the
same with the reference shifted away from the alignment (`chance_share`,
`chance_support`: the mean over the shifts; `chance_share_max`). Derived:
`excess` = share - chance_share and `support_excess` = support -
chance_support.

Pairs given with --pair "<substring a>|<substring b>" are reported
apart and unlabelled (their ground truth waits for the owner).

usage: verifier_groups.py <output.json> <clusters.json> <sets dir> <tags.json>
                          <library-map.json or -> <report>... [--pair a|b]...
"""
import collections
import json
import os
import statistics
import sys
import tomllib

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from labels import label, name  # noqa: E402

args = sys.argv[1:]
pairs = []
while "--pair" in args:
    at = args.index("--pair")
    pairs.append(tuple(args[at + 1].split("|")))
    del args[at:at + 2]
output, clusters_file, sets_dir, tags_file, map_file, *reports = args
tags = json.load(open(tags_file))
stands_for = {} if map_file == "-" else {c["corpus"]: c["stands_for"] for c in json.load(open(map_file))["copies"]}
cluster_of = {}
for members in json.load(open(clusters_file))["duplicates"]:
    for member in members:
        cluster_of[member] = set(members)


def cluster(asset):
    return cluster_of.get(asset, {asset})


def labelled(query, found):
    return label(name(query, tags, stands_for), name(found, tags, stands_for))


def listening_pair(query, found):
    for a, b in pairs:
        if (a in query and b in found) or (b in query and a in found):
            return f"{a} ~ {b}"
    return None


rows = []


def add(group, source, query, found_detection, where):
    verified = found_detection.get("verified") or []
    if not verified:
        return
    row = {"group": group, "source": source, "where": where, "query": query, "found": found_detection["asset"],
           "hits": found_detection["hits"], "windows": found_detection["windows"], "measures": {}}
    if group != "identifying":
        pair = listening_pair(query, found_detection["asset"])
        row["group"] = "listening list" if pair else "false: " + ("unrelated" if labelled(query, found_detection["asset"]) == "unrelated" else "related")
        row["label"] = pair or labelled(query, found_detection["asset"])
    for v in verified:
        shifted = v["shifted"]
        chance_share = statistics.mean(s["share"] for s in shifted)
        chance_support = statistics.mean(s["support"] for s in shifted)
        a = v["aligned"]
        row["measures"][v["tolerance"]] = {
            "reference_peaks": a["reference_peaks"], "found": a["found"], "share": a["share"], "support": a["support"],
            "chance_share": chance_share, "chance_share_max": max(s["share"] for s in shifted),
            "chance_support": chance_support,
            "excess": a["share"] - chance_share, "support_excess": a["support"] - chance_support,
        }
    rows.append(row)


def manifest_slots(set_name):
    manifest = tomllib.load(open(os.path.join(sets_dir, set_name, "tracklist.toml"), "rb"))
    def seconds(text):
        value = 0.0
        for part in text.split(":"):
            value = value * 60 + float(part)
        return value
    tracks = manifest["track"]
    starts = [seconds(t["start"]) for t in tracks]
    slots = []
    for i, t in enumerate(tracks):
        end = starts[i + 1] if i + 1 < len(tracks) else float("inf")
        accepted = set()
        for ref in t.get("reference", []):
            accepted |= cluster(ref)
        slots.append({"label": f"{t['position']:>2}. {t['artist']} - {t['title']}", "start": starts[i], "end": end,
                      "accepted": accepted, "references": t.get("reference", [])})
    return slots


for path in reports:
    report = json.load(open(path))
    source = os.path.basename(os.path.dirname(path)) + "/" + os.path.basename(path)
    if "queries" in report and "per_speed" in report:  # sweep
        for q in report["queries"]:
            where = f"sweep {report['seed']} {q['asset']} at {q['speed_percent']:+.0f}%"
            for d in q["detections"]:
                add("identifying" if d["correct"] else "false", source, q["asset"], d, where)
    elif "score" in report and "set" in report:  # scan
        slots = manifest_slots(report["set"])
        left_out = set(report["left_out_tracks"])
        for d in report["detections"]:
            covering = [s for s in slots if d["asset"] in s["accepted"] and s["label"] not in left_out
                        and d["start_seconds"] < s["end"] + 90 and s["start"] - 90 < d["end_seconds"]]
            middle = (d["start_seconds"] + d["end_seconds"]) / 2
            slot = max((s for s in slots if s["start"] <= middle), key=lambda s: s["start"], default=slots[0])
            played = slot["label"]
            add("identifying" if covering else "false", source, played, d, f"{report['set']} at {d['start_seconds']:.0f} s")
    elif "conditions" in report or ("rows" in report and "queries" in report):  # robust
        for q in report["queries"]:
            for d in q.get("verified", []):
                where = f"robust {q['condition']} {q['asset']} at {q['speed']:.3f}"
                add("identifying" if d["role"] in ("own", "partner") else "false", source, q["asset"], d, where)
    elif "mixes" in report:
        for result in report["mixes"]:
            plays = result["mix"]["plays"]
            for d in result["detections"]:
                matching = [p for p in plays if d["asset"] in cluster(p["asset"])
                            and d["start_seconds"] < p["end_seconds"] and p["start_seconds"] < d["end_seconds"]]
                middle = (d["start_seconds"] + d["end_seconds"]) / 2
                played = next((p["asset"] for p in plays if p["start_seconds"] <= middle <= p["end_seconds"]), plays[0]["asset"])
                add("identifying" if matching else "false", source, played, d, f"{result['mix']['name']} at {d['start_seconds']:.0f} s")
    elif "briefs" in report:  # grid
        for q in report["queries"]:
            brief = report["briefs"][q["brief"]]
            where = f"grid brief {q['brief']} {q['seconds']:.0f} s offset {q['offset_seconds']}"
            for d in q["matching"]:
                add("identifying", source, brief["asset"], d, where)
            for d in q.get("false_candidates", []):
                add("false", source, brief["asset"], d, where)
    else:
        print(f"unknown report kind: {path}", file=sys.stderr)


def auc(positives, negatives):
    """The chance that a random identifying detection scores above a random false one (ties count half)."""
    if not positives or not negatives:
        return None
    wins = 0.0
    for p in positives:
        for n in negatives:
            wins += 1.0 if p > n else 0.5 if p == n else 0.0
    return wins / (len(positives) * len(negatives))


HIT_BINS = [(0, 20), (20, 40), (40, 60), (60, 120), (120, 240), (240, 10**9)]
tolerances = sorted({t for r in rows for t in r["measures"]})
summary = {"reports": reports, "detections": len(rows), "groups": dict(collections.Counter(r["group"] for r in rows)),
           "by_tolerance": {}}
groups = ["identifying", "false: related", "false: unrelated", "listening list"]
for t in tolerances:
    per = {"by_group": {}, "by_hits": [], "separation": {}}
    for g in groups:
        members = [r for r in rows if r["group"] == g]
        if not members:
            continue
        m = [r["measures"][t] for r in members]
        per["by_group"][g] = {
            "count": len(members),
            **{f"{k}_median": statistics.median(x[k] for x in m) for k in ("share", "support", "chance_share", "chance_support", "excess", "support_excess")},
            "excess_min": min(x["excess"] for x in m), "excess_max": max(x["excess"] for x in m),
        }
    for low, high in HIT_BINS:
        cell = {"hits": f"{low}-{high if high < 10**9 else ''}"}
        for g in groups:
            members = [r["measures"][t] for r in rows if r["group"] == g and low <= r["hits"] < high]
            if members:
                cell[g] = {"count": len(members),
                           "excess_median": round(statistics.median(x["excess"] for x in members), 4),
                           "excess_range": [round(min(x["excess"] for x in members), 4), round(max(x["excess"] for x in members), 4)],
                           "share_median": round(statistics.median(x["share"] for x in members), 4),
                           "support_excess_median": round(statistics.median(x["support_excess"] for x in members), 4)}
        per["by_hits"].append(cell)
    # At equal hits: only detections in the hit range both groups reach.
    true_rows = [r for r in rows if r["group"] == "identifying"]
    false_rows = [r for r in rows if r["group"].startswith("false")]
    if true_rows and false_rows:
        low = min(r["hits"] for r in true_rows)
        high = max(r["hits"] for r in false_rows)
        overlap_true = [r for r in true_rows if low <= r["hits"] <= high]
        overlap_false = [r for r in false_rows if low <= r["hits"] <= high]
        per["separation"] = {
            "overlap_hits": [low, high], "identifying": len(overlap_true), "false": len(overlap_false),
            **{f"auc_{k}": auc([r["measures"][t][k] if k != "hits" else r["hits"] for r in overlap_true],
                               [r["measures"][t][k] if k != "hits" else r["hits"] for r in overlap_false])
               for k in ("hits", "share", "support", "excess", "support_excess")},
        }
        for kind in ("false: related", "false: unrelated"):
            subset = [r for r in overlap_false if r["group"] == kind]
            per["separation"][kind] = {k: auc([r["measures"][t][k] if k != "hits" else r["hits"] for r in overlap_true],
                                              [r["measures"][t][k] if k != "hits" else r["hits"] for r in subset])
                                       for k in ("hits", "excess", "support_excess")} if subset else None
    weakest = sorted(true_rows, key=lambda r: r["hits"])[:10]
    per["weakest_identifying"] = [{"hits": r["hits"], "windows": r["windows"], "where": r["where"], **{k: round(v, 4) for k, v in r["measures"][t].items()}} for r in weakest]
    strongest_false = sorted(false_rows, key=lambda r: -r["hits"])[:10]
    per["strongest_false"] = [{"hits": r["hits"], "group": r["group"], "label": r.get("label"), "found": r["found"], "where": r["where"], **{k: round(v, 4) for k, v in r["measures"][t].items()}} for r in strongest_false]
    per["highest_excess_false"] = [{"hits": r["hits"], "group": r["group"], "found": r["found"], "where": r["where"], **{k: round(v, 4) for k, v in r["measures"][t].items()}}
                                   for r in sorted(false_rows, key=lambda r: -r["measures"][t]["excess"])[:10]]
    per["listening_list"] = [{"hits": r["hits"], "windows": r["windows"], "pair": r["label"], "where": r["where"], **{k: round(v, 4) for k, v in r["measures"][t].items()}}
                             for r in sorted((r for r in rows if r["group"] == "listening list"), key=lambda r: -r["hits"])]
    summary["by_tolerance"][t] = per

json.dump({"summary": summary, "rows": rows}, open(output, "w"), indent=1)
print(f"{len(rows)} detections: {summary['groups']}")
for t, per in summary["by_tolerance"].items():
    print(f"\n== tolerance {t} (bins x frames)")
    for g, s in per["by_group"].items():
        print(f"  {g:18} n={s['count']:5}  share {s['share_median']:.3f} (chance {s['chance_share_median']:.3f})  "
              f"excess {s['excess_median']:+.3f} [{s['excess_min']:+.3f}, {s['excess_max']:+.3f}]  "
              f"support {s['support_median']:.2f} (chance {s['chance_support_median']:.2f})")
    sep = per["separation"]
    if sep:
        print(f"  at equal hits {sep['overlap_hits']}: {sep['identifying']} identifying, {sep['false']} false; AUC "
              + ", ".join(f"{k[4:]} {v:.3f}" for k, v in sep.items() if k.startswith("auc_") and v is not None))
        for kind in ("false: related", "false: unrelated"):
            if sep.get(kind):
                print(f"    vs {kind}: " + ", ".join(f"{k} {v:.3f}" for k, v in sep[kind].items()))
    for cell in per["by_hits"]:
        parts = [f"{g.split(': ')[-1]} {c['count']} ex {c['excess_median']:+.3f} {c['excess_range']}" for g, c in cell.items() if g != "hits"]
        print(f"  hits {cell['hits']:>8}: " + "; ".join(parts))
