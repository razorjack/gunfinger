"""Session 22: compare two builds' harness reports of the same run, the FFT
lengths' first-pass variant against today's build (docs/brief-22.md, step 2).

A detection changes when any field differs. Plays and detections of the
two builds are paired by file and overlap in query time; a boundary moves
when it differs by more than TOLERANCE seconds. Prints every change of a
confident result and writes the counts and changes as JSON.

usage: fft_compare.py scan|mixes|sweep|grid|robust BEFORE AFTER [OUT.json]
"""
import json, statistics, sys

TOLERANCE = 1.0
RULE_HITS = 240
RULE_WINDOWS = 3
POSSIBLE_HITS = 60


def overlap(a, b):
    return min(a["end_seconds"], b["end_seconds"]) - max(a["start_seconds"], b["start_seconds"])


def pair_up(before, after, key):
    """Pairs items with the same key by largest overlap; returns pairs,
    the unpaired before and the unpaired after."""
    pairs, lost, free = [], [], list(after)
    for item in before:
        same = [other for other in free if key(other) == key(item) and overlap(item, other) > 0]
        if not same:
            lost.append(item)
            continue
        best = max(same, key=lambda other: overlap(item, other))
        free.remove(best)
        pairs.append((item, best))
    return pairs, lost, free


def level(play):
    return "confident" if any(s["confident"] for s in play["segments"]) else "possible"


def strongest(play):
    return max(play["segments"], key=lambda s: s["hits"])


def edge(before, after):
    """Names the rule's edge when a confident result changes there."""
    reasons = []
    for name, threshold in (("hits", RULE_HITS), ("windows", RULE_WINDOWS)):
        b, a = before.get(name), after.get(name)
        if b is not None and a is not None and (b >= threshold) != (a >= threshold):
            reasons.append(f"{name} {b} -> {a} across {threshold}")
    return "; ".join(reasons)


def describe(item):
    hits = item.get("hits", strongest(item)["hits"] if "segments" in item else None)
    return (f"{item['asset']} {item['start_seconds']:.1f}-{item['end_seconds']:.1f} s, "
            f"{hits} hits")


def hit_ratios(pairs, hits):
    ratios = [hits(a) / hits(b) for b, a in pairs if hits(b) > 0]
    if not ratios:
        return None
    return {"median": statistics.median(ratios), "min": min(ratios), "max": max(ratios)}


def compare_detections(before, after):
    same = sum(1 for b, a in zip(before, after) if b == a) if len(before) == len(after) else None
    identical = before == after
    pairs, lost, added = pair_up(before, after, lambda d: d["asset"])
    changes = []
    for b, a in pairs:
        if b != a and b.get("confident") != a.get("confident"):
            changes.append({"before": b, "after": a, "edge": edge(b, a)})
    return {
        "before": len(before), "after": len(after), "identical": identical,
        "same_in_order": same, "paired": len(pairs),
        "changed": sum(1 for b, a in pairs if b != a),
        "lost": len(lost), "added": len(added),
        "confident_lost": [d for d in lost if d.get("confident")],
        "confident_added": [d for d in added if d.get("confident")],
        "confidence_changed": changes,
        "hits_after_over_before": hit_ratios(pairs, lambda d: d["hits"]),
    }


def credited_files(score):
    """(track, file) credited with a confident detection; files credited
    through the clusters are among `credited` too."""
    return {(t["label"], d["asset"]) for t in score["tracks"] for d in t["credited"]}


def compare_scan(before, after):
    sb, sa = before["score"], after["score"]
    out = {"score": {k: [sb[k], sa[k]] for k in ("referenced", "identified", "wrong", "possible")}}
    files_b, files_a = credited_files(sb), credited_files(sa)
    out["right_files_lost"] = sorted(files_b - files_a)
    out["right_files_gained"] = sorted(files_a - files_b)
    out["tracks_changed"] = [
        {"label": tb["label"], "identified": [tb["identified"], ta["identified"]],
         "possible": [tb["possible"], ta["possible"]]}
        for tb, ta in zip(sb["tracks"], sa["tracks"])
        if (tb["identified"], tb["possible"]) != (ta["identified"], ta["possible"])]
    wp, wl, wa = pair_up(sb["wrong_identifications"], sa["wrong_identifications"], lambda d: d["asset"])
    out["wrong_confident"] = {"before": len(sb["wrong_identifications"]),
                              "after": len(sa["wrong_identifications"]),
                              "removed": wl, "added": wa}
    pp, pl, pa = pair_up(sb["unmatched_possible"], sa["unmatched_possible"], lambda p: p["asset"])
    out["unmatched_possible"] = {"before": len(sb["unmatched_possible"]),
                                 "after": len(sa["unmatched_possible"]),
                                 "lost": [describe(p) for p in pl], "added": [describe(p) for p in pa]}
    out["plays"] = compare_plays(before["plays"], after["plays"])
    out["detections"] = compare_detections(before["detections"], after["detections"])
    return out


def compare_plays(before, after):
    pairs, lost, added = pair_up(before, after, lambda p: p["asset"])
    level_changes, moved = [], []
    for b, a in pairs:
        if level(b) != level(a):
            level_changes.append({"asset": b["asset"], "before": level(b), "after": level(a),
                                  "hits": [strongest(b)["hits"], strongest(a)["hits"]],
                                  "windows": [strongest(b)["windows"], strongest(a)["windows"]],
                                  "segments": [len(b["segments"]), len(a["segments"])],
                                  "edge": edge(strongest(b), strongest(a))})
        ds = a["start_seconds"] - b["start_seconds"]
        de = a["end_seconds"] - b["end_seconds"]
        if abs(ds) > TOLERANCE or abs(de) > TOLERANCE:
            moved.append({"asset": b["asset"], "level": [level(b), level(a)],
                          "start": [b["start_seconds"], a["start_seconds"]],
                          "end": [b["end_seconds"], a["end_seconds"]],
                          "segments": [len(b["segments"]), len(a["segments"])]})
    shifts = [max(abs(a["start_seconds"] - b["start_seconds"]), abs(a["end_seconds"] - b["end_seconds"]))
              for b, a in pairs]
    return {
        "before": len(before), "after": len(after), "paired": len(pairs),
        "identical": sum(1 for b, a in pairs if b == a),
        "lost": [{"asset": p["asset"], "level": level(p), "hits": strongest(p)["hits"],
                  "start": p["start_seconds"], "end": p["end_seconds"]} for p in lost],
        "added": [{"asset": p["asset"], "level": level(p), "hits": strongest(p)["hits"],
                   "start": p["start_seconds"], "end": p["end_seconds"]} for p in added],
        "level_changed": level_changes, "boundaries_moved": moved,
        "largest_boundary_shift": max(shifts) if shifts else None,
        "hits_after_over_before": hit_ratios(pairs, lambda p: strongest(p)["hits"]),
    }


def compare_mixes(before, after):
    out = {"plays": 0, "level_changed": [], "credited_changed": [], "boundaries_moved": [],
           "worse": [], "wrong_confident": [], "wrong_possible": [], "levels": {}}
    shifts = []
    for mb, ma in zip(before["mixes"], after["mixes"]):
        name = mb["mix"]["name"]
        assert name == ma["mix"]["name"] and mb["mix"] == ma["mix"], name
        for pb, pa in zip(mb["score"]["plays"], ma["score"]["plays"]):
            out["plays"] += 1
            truth = pb["truth"]
            tag = f"{name} {truth['asset']} {truth['start_seconds']:.1f}-{truth['end_seconds']:.1f} s"
            key = ("held out" if truth["held_out"] else "indexed", pb["level"], pa["level"])
            out["levels"][" / ".join(key)] = out["levels"].get(" / ".join(key), 0) + 1
            if pb["level"] != pa["level"]:
                out["level_changed"].append({"play": tag, "before": pb["level"], "after": pa["level"],
                                             "best": [pb.get("best"), pa.get("best")],
                                             "edge": edge(pb["best"] or {}, pa["best"] or {})})
                continue
            if not pb.get("best") or not pa.get("best"):
                continue
            if pb["best"]["asset"] != pa["best"]["asset"]:
                out["credited_changed"].append({"play": tag, "best": [pb["best"]["asset"], pa["best"]["asset"]]})
            for side in ("start_late_seconds", "end_early_seconds"):
                b, a = pb.get(side), pa.get(side)
                if b is None or a is None:
                    continue
                shifts.append(abs(a - b))
                if abs(a - b) > TOLERANCE:
                    row = {"play": tag, "side": side, "error": [b, a], "level": pa["level"]}
                    out["boundaries_moved"].append(row)
                    if abs(a) - abs(b) > TOLERANCE:
                        out["worse"].append(row)
        for kind in ("wrong_confident", "wrong_possible"):
            pairs, lost, added = pair_up(mb["score"][kind], ma["score"][kind], lambda d: d["asset"])
            for d in lost:
                out[kind].append({"mix": name, "change": "removed", "detection": describe(d)})
            for d in added:
                out[kind].append({"mix": name, "change": "added", "detection": describe(d)})
            out[kind + "_count"] = out.get(kind + "_count", [0, 0])
            out[kind + "_count"][0] += len(mb["score"][kind])
            out[kind + "_count"][1] += len(ma["score"][kind])
    out["largest_boundary_shift"] = max(shifts) if shifts else None
    out["boundary_shifts_over_0.1s"] = sum(1 for s in shifts if s > 0.1)
    out["boundary_sides"] = len(shifts)
    all_b = [d for m in before["mixes"] for d in m["detections"]]
    all_a = [d for m in after["mixes"] for d in m["detections"]]
    out["detections"] = {"before": len(all_b), "after": len(all_a),
                         "identical_mixes": sum(1 for mb, ma in zip(before["mixes"], after["mixes"])
                                                if mb["detections"] == ma["detections"])}
    return out


def sweep_answer(query):
    confident = [d for d in query["detections"] if d["confident"]]
    if any(not d["correct"] for d in confident):
        return "wrong"
    if confident:
        return "right"
    possible = [d for d in query["detections"] if d["hits"] >= POSSIBLE_HITS]
    if possible:
        return "possible right" if all(d["correct"] for d in possible) else "possible wrong"
    return "none"


def bar(report):
    within = [r for r in report["per_speed"] if abs(r["speed_percent"]) <= 5]
    recalled = sum(r["recalled"] for r in within)
    indexed = sum(r["indexed_excerpts"] for r in within)
    wrong = sum(r["wrong_answers"] for r in report["per_speed"])
    return {"recall_within_5": recalled / indexed, "recalled": recalled, "indexed": indexed,
            "wrong": wrong, "passes": recalled / indexed >= 0.95 and wrong == 0}


def compare_sweep(before, after):
    out = {"queries": len(before["queries"]), "identical": 0, "answer_changed": [],
           "bar": [bar(before), bar(after)],
           "per_speed": [[r["speed_percent"], rb["recalled"], r["recalled"], rb["wrong_answers"], r["wrong_answers"]]
                         for rb, r in zip(before["per_speed"], after["per_speed"])]}
    pairs = []
    for qb, qa in zip(before["queries"], after["queries"]):
        assert (qb["asset"], qb["speed_percent"], qb["start_seconds"]) == \
            (qa["asset"], qa["speed_percent"], qa["start_seconds"])
        if qb["detections"] == qa["detections"]:
            out["identical"] += 1
        ab, aa = sweep_answer(qb), sweep_answer(qa)
        if ab != aa:
            best = lambda q: max(q["detections"], key=lambda d: d["hits"]) if q["detections"] else {}
            out["answer_changed"].append({"asset": qb["asset"], "held_out": qb["held_out"],
                                          "speed_percent": qb["speed_percent"], "answer": [ab, aa],
                                          "best": [best(qb), best(qa)], "edge": edge(best(qb), best(qa))})
        own = lambda q: [d for d in q["detections"] if d["correct"]]
        if own(qb) and own(qa):
            pairs.append((max(own(qb), key=lambda d: d["hits"]), max(own(qa), key=lambda d: d["hits"])))
    out["hits_after_over_before"] = hit_ratios(pairs, lambda d: d["hits"])
    return out


def compare_grid(before, after):
    out = {"queries": len(before["queries"]), "identical": 0, "level_changed": [], "wrong_confident": [0, 0]}
    for qb, qa in zip(before["queries"], after["queries"]):
        assert (qb["brief"], qb["seconds"], qb["offset_seconds"]) == (qa["brief"], qa["seconds"], qa["offset_seconds"])
        out["identical"] += qb == qa
        out["wrong_confident"][0] += qb["wrong_confident"]
        out["wrong_confident"][1] += qa["wrong_confident"]
        if qb["level"] != qa["level"]:
            out["level_changed"].append({"brief": qb["brief"], "seconds": qb["seconds"],
                                         "offset": qb["offset_seconds"], "level": [qb["level"], qa["level"]],
                                         "matching": [qb["matching"], qa["matching"]]})
    levels = lambda r: {f"{s} s": sum(1 for q in r["queries"] if q["seconds"] == s and q["level"] == "confident")
                        for s in sorted({q["seconds"] for q in r["queries"]})}
    out["confident_by_length"] = [levels(before), levels(after)]
    return out


def compare_robust(before, after):
    out = {"queries": len(before["queries"]), "identical": 0, "changed": [], "rows_changed": []}
    for qb, qa in zip(before["queries"], after["queries"]):
        assert (qb["condition"], qb["speed"], qb["number"], qb["asset"]) == \
            (qa["condition"], qa["speed"], qa["number"], qa["asset"])
        timing = ("search_seconds",)
        out["identical"] += all(qb[k] == qa[k] for k in qb if k not in timing)
        fields = ("confident", "possible", "partner_confident", "wrong_confident", "wrong_possible")
        if any(qb[f] != qa[f] for f in fields):
            out["changed"].append({"condition": qb["condition"], "speed": qb["speed"], "asset": qb["asset"],
                                   "held_out": qb["held_out"],
                                   **{f: [qb[f], qa[f]] for f in fields if qb[f] != qa[f]},
                                   "best_hits": [qb["best_hits"], qa["best_hits"]],
                                   "best_windows": [qb["best_windows"], qa["best_windows"]],
                                   "strongest_wrong_hits": [qb["strongest_wrong_hits"], qa["strongest_wrong_hits"]]})
    for rb, ra in zip(before["rows"], after["rows"]):
        fields = ("confident", "possible_or_better", "partner_confident", "wrong_confident", "wrong_possible")
        if any(rb[f] != ra[f] for f in fields):
            out["rows_changed"].append({"condition": rb["condition"], "speed": rb["speed"],
                                        **{f: [rb[f], ra[f]] for f in fields if rb[f] != ra[f]}})
    out["totals"] = {f: [sum(r[f] for r in before["rows"]), sum(r[f] for r in after["rows"])]
                     for f in ("indexed", "confident", "possible_or_better", "wrong_confident", "wrong_possible")}
    return out


def main():
    kind, before_path, after_path = sys.argv[1:4]
    before, after = json.load(open(before_path)), json.load(open(after_path))
    result = {"scan": compare_scan, "mixes": compare_mixes, "sweep": compare_sweep,
              "grid": compare_grid, "robust": compare_robust}[kind](before, after)
    text = json.dumps(result, indent=1, default=str)
    if len(sys.argv) > 4:
        open(sys.argv[4], "w").write(text)
    print(text)


if __name__ == "__main__":
    main()
