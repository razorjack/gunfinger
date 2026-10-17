"""Session 27 (experiments 0136 and up): today's confident rule against the
variants that replace its 3-window condition, counted offline from saved
harness reports.

Today (`confidence.rs`, `FITTED_RULE`): a segment is confident at 240
hits in 3 windows (200 for the single pass), possible at 60 hits.
Variants keep the hits and the possible tier and replace `windows >= 3`:
`span:S`, the first to the last aligned hit at least S seconds;
`supported:S`, at least S distinct whole seconds after the first hit
holding an aligned hit (`supported_seconds`, written only by session
27's variant build). Each panel is first scored under today's rule and
checked against the levels and scores stored in the report; a mismatch
is printed and the panel's variant counts are not to be used.

usage:
  span_rule.py grid REPORT OUT
  span_rule.py mixes CLUSTERS OUT REPORT...
  span_rule.py scans [--nas] CLUSTERS OUT REPORT...
  span_rule.py shared CLUSTERS PANEL REPORT OUT
  span_rule.py queries OUT REPORT...        (sweep or recall reports)
  span_rule.py robust OUT REPORT
  span_rule.py identity M_REPORT V_REPORT   (V holds M, timings aside)

Refuses any path naming the held-out test set.
"""

import json
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import devscore  # noqa: E402
import shared_outcomes  # noqa: E402

TODAY = ("windows", 3)
RULES = [TODAY, ("span", 10), ("span", 15), ("span", 20),
         ("supported", 10), ("supported", 15), ("supported", 20)]
ORDER = {"none": -1, "weak": 0, "possible": 1, "confident": 2}


def name(rule):
    return "3 windows" if rule == TODAY else f"{rule[0]} {rule[1]} s"


def available(rule, detections):
    """A supported-time rule needs the variant build's field."""
    return rule[0] != "supported" or all("supported_seconds" in d for d in detections)


def span(d):
    return d["end_seconds"] - d["start_seconds"]


def level(d, rule):
    """`Evidence::confidence` with `windows >= 3` replaced by `rule`."""
    min_hits = 240 if d.get("fitted", False) else 200
    kind, value = rule
    if kind == "windows":
        held = d["windows"] >= value
    elif kind == "span":
        held = span(d) >= value
    else:
        held = d["supported_seconds"] >= value
    if d["hits"] >= min_hits and held:
        return "confident"
    if d["hits"] >= 60:
        return "possible"
    return "weak"


def cause(d):
    out = {"asset": d["asset"], "hits": d["hits"], "windows": d["windows"]}
    if "start_seconds" in d and "end_seconds" in d:
        out["start"] = round(d["start_seconds"], 2)
        out["span"] = round(span(d), 2)
    if "supported_seconds" in d:
        out["supported"] = d["supported_seconds"]
    return out


def load(path):
    devscore.refuse_test_set(path)
    return devscore.load_json(path)


def write(out, result):
    devscore.refuse_test_set(out)
    with open(out, "w") as file:
        json.dump(result, file, indent=1)


def check(label, ok, problems):
    if not ok:
        problems.append(label)


# --- grid ---------------------------------------------------------------

def grid(path, out):
    report = load(path)
    queries = report["queries"]
    problems = []
    for q in queries:
        best = max(q["matching"], key=lambda d: d["hits"], default=None)
        stored = q["level"]
        computed = level(best, TODAY) if best else "none"
        check(f"brief {q['brief']} {q['seconds']} s offset {q['offset_seconds']}: "
              f"stored {stored}, computed {computed}", stored == computed, problems)
    false_over_240 = [q for q in queries if q["strongest_false_hits"] >= 240]
    lengths = sorted({q["seconds"] for q in queries})
    briefs = sorted({q["brief"] for q in queries})
    result = {"report": Path(path).name, "problems": problems,
              "queries": len(queries),
              "strongest_false_hits": max(q["strongest_false_hits"] for q in queries),
              "queries_with_false_over_240": len(false_over_240),
              "wrong_confident_stored": sum(q["wrong_confident"] for q in queries),
              "rules": {}}
    for rule in RULES:
        if not all(available(rule, q["matching"]) for q in queries):
            continue
        confident = {(q["brief"], q["seconds"], q["offset_seconds"]):
                     any(level(d, rule) == "confident" for d in q["matching"]) for q in queries}
        rows = []
        for seconds in lengths:
            here = [q for q in queries if q["seconds"] == seconds]
            flips = sum(1 for b in briefs
                        if len({confident[(b, seconds, q["offset_seconds"])]
                                for q in here if q["brief"] == b}) > 1)
            rows.append({"seconds": seconds, "queries": len(here),
                         "confident": sum(confident[(q["brief"], seconds, q["offset_seconds"])]
                                          for q in here),
                         "flips": flips})
        shortest_any = min((r["seconds"] for r in rows if r["confident"]), default=None)
        shortest_all = min((r["seconds"] for r in rows
                            if r["confident"] == r["queries"]
                            and all(r2["confident"] == r2["queries"]
                                    for r2 in rows if r2["seconds"] >= r["seconds"])),
                           default=None)
        spans = [d for q in queries for d in q["matching"] if level(d, rule) == "confident"]
        result["rules"][name(rule)] = {
            "rows": rows,
            "confident": sum(r["confident"] for r in rows),
            "flips": sum(r["flips"] for r in rows),
            "shortest_length_any_confident": shortest_any,
            "shortest_length_all_confident": shortest_all,
            "shortest_confident_span": round(min(span(d) for d in spans), 2) if spans else None,
            "shortest_confident_supported": min((d["supported_seconds"] for d in spans
                                                 if "supported_seconds" in d), default=None),
            "changed": [
                {"brief": q["brief"], "seconds": q["seconds"], "offset": q["offset_seconds"],
                 "today": any(level(d, TODAY) == "confident" for d in q["matching"]),
                 "variant": confident[(q["brief"], q["seconds"], q["offset_seconds"])],
                 "segments": [cause(d) for d in q["matching"] if level(d, TODAY) != level(d, rule)]}
                for q in queries
                if any(level(d, TODAY) != level(d, rule) for d in q["matching"])
            ],
        }
    write(out, result)
    print(f"grid {Path(path).name}: {len(queries)} queries; reproduction problems: {len(problems)}; "
          f"strongest false {result['strongest_false_hits']} hits "
          f"({len(false_over_240)} queries with a false detection of 240 or more)")
    for rule_name, r in result["rules"].items():
        cells = "  ".join(f"{row['seconds']:.0f}s {row['confident']}/{row['queries']} ({row['flips']})"
                          for row in r["rows"])
        print(f"  {rule_name:>13}: {cells}  | total {r['confident']}, flips {r['flips']}, "
              f"shortest any {r['shortest_length_any_confident']}, all {r['shortest_length_all_confident']}, "
              f"span {r['shortest_confident_span']}")


# --- generated mixes ------------------------------------------------------

def mix_score(mix, detections, cluster_of, rule):
    """`mixes::score` under `rule`: each planned play's level is its
    strongest (most hits) matching detection's; a detection matching no
    play is wrong at its level."""
    plays = mix["plays"]
    accepted = [cluster_of(p["asset"]) for p in plays]

    def matches(d, i):
        p = plays[i]
        return (d["asset"] in accepted[i]
                and d["start_seconds"] < p["end_seconds"] and p["start_seconds"] < d["end_seconds"])

    levels = []
    for i in range(len(plays)):
        matching = [d for d in detections if matches(d, i)]
        best = max(matching, key=lambda d: d["hits"], default=None)
        levels.append(level(best, rule) if best else "none")
    unmatched = [d for d in detections if not any(matches(d, i) for i in range(len(plays)))]
    return levels, [d for d in unmatched if level(d, rule) == "confident"], \
        [d for d in unmatched if level(d, rule) == "possible"]


def mixes(clusters, out, paths):
    cluster_of = devscore.clusters_of(clusters)
    result = {"reports": [], "problems": [], "rules": {}}
    loaded = [(path, load(path)) for path in paths]
    for path, report in loaded:
        result["reports"].append(Path(path).name)
        for m in report["mixes"]:
            levels, wrong, possible = mix_score(m["mix"], m["detections"], cluster_of, TODAY)
            stored = [p["level"] for p in m["score"]["plays"]]
            check(f"{m['mix']['name']}: play levels", levels == stored, result["problems"])
            check(f"{m['mix']['name']}: wrong confident",
                  len(wrong) == len(m["score"]["wrong_confident"]), result["problems"])
            check(f"{m['mix']['name']}: wrong possible",
                  len(possible) == len(m["score"]["wrong_possible"]), result["problems"])
    for rule in RULES:
        if not all(available(rule, m["detections"]) for _, r in loaded for m in r["mixes"]):
            continue
        counts = Counter()
        changed, new_wrong, shortest = [], [], []
        for _, report in loaded:
            for m in report["mixes"]:
                today, wrong_today, _ = mix_score(m["mix"], m["detections"], cluster_of, TODAY)
                levels, wrong, possible = mix_score(m["mix"], m["detections"], cluster_of, rule)
                for play, before, after in zip(m["mix"]["plays"], today, levels):
                    if play["held_out"]:
                        continue
                    counts[after] += 1
                    if before != after:
                        changed.append({"mix": m["mix"]["name"], "asset": play["asset"],
                                        "seconds": round(play["end_seconds"] - play["start_seconds"], 1),
                                        "today": before, "variant": after})
                counts["wrong confident"] += len(wrong)
                counts["wrong possible"] += len(possible)
                keys = {(d["asset"], d["start_seconds"]) for d in wrong_today}
                new_wrong += [cause(d) | {"mix": m["mix"]["name"]} for d in wrong
                              if (d["asset"], d["start_seconds"]) not in keys]
                shortest += [span(d) for d in m["detections"] if level(d, rule) == "confident"
                             and d not in wrong]
        result["rules"][name(rule)] = {"plays": dict(counts), "changed": changed,
                                       "new_wrong_confident": new_wrong,
                                       "shortest_confident_span": round(min(shortest), 2) if shortest else None}
    write(out, result)
    print(f"mixes {', '.join(result['reports'])}: reproduction problems: {len(result['problems'])}")
    for rule_name, r in result["rules"].items():
        p = r["plays"]
        print(f"  {rule_name:>13}: confident {p.get('confident', 0)}, possible {p.get('possible', 0)}, "
              f"weak {p.get('weak', 0)}, none {p.get('none', 0)}; wrong confident {p.get('wrong confident', 0)} "
              f"(new {len(r['new_wrong_confident'])}), wrong possible {p.get('wrong possible', 0)}; "
              f"changed {len(r['changed'])}; shortest span {r['shortest_confident_span']}")


# --- scans ------------------------------------------------------------------

def tracks_of(report, nas):
    """The manifest as `scoring.rs` reads it: a left-out track stays
    referenced; its files are only missing from the index."""
    return devscore.load_set(report["set"], nas)


def scans(nas, clusters, out, paths):
    cluster_of = devscore.clusters_of(clusters)
    result = {"reports": {}, "problems": []}
    for path in paths:
        report = load(path)
        tracks = tracks_of(report, nas)
        score = devscore.score_scan(report, tracks, cluster_of,
                                    confident=lambda d: level(d, TODAY) == "confident")
        stored = report["score"]
        flags = all(d["confident"] == (level(d, TODAY) == "confident") for d in report["detections"])
        check(f"{Path(path).name}: stored confident flags", flags, result["problems"])
        for key in ("identified", "referenced", "wrong", "possible"):
            check(f"{Path(path).name}: {key} {score[key]} against stored {stored[key]}",
                  score[key] == stored[key], result["problems"])
        check(f"{Path(path).name}: possible plays matching no track",
              len(score["unmatched_possible"]) == len(stored["unmatched_possible"]), result["problems"])
        slots_ = devscore.slots(tracks, report["duration_seconds"], cluster_of)
        per_rule = {}
        for rule in RULES:
            if not available(rule, report["detections"]):
                continue
            s = devscore.score_scan(report, tracks, cluster_of,
                                    confident=lambda d, rule=rule: level(d, rule) == "confident")
            changed = []
            for d in report["detections"]:
                if level(d, TODAY) != level(d, rule):
                    true = bool(devscore.matching_slots(slots_, d["asset"], d["start_seconds"], d["end_seconds"]))
                    changed.append(cause(d) | {"today": level(d, TODAY), "variant": level(d, rule),
                                               "in_a_slot": true})
            today_wrong = {(d["asset"], d["start_seconds"]) for d in score["wrong_identifications"]}
            confident_true = [d for d in report["detections"] if level(d, rule) == "confident"
                              and devscore.matching_slots(slots_, d["asset"], d["start_seconds"], d["end_seconds"])]
            per_rule[name(rule)] = {
                "identified": s["identified"], "referenced": s["referenced"], "wrong": s["wrong"],
                "new_wrong": [cause(d) for d in s["wrong_identifications"]
                              if (d["asset"], d["start_seconds"]) not in today_wrong],
                "tracks": [t["label"] for t in s["tracks"] if t["identified"]],
                "changed": changed,
                "shortest_confident_true_span": round(min(span(d) for d in confident_true), 2)
                if confident_true else None,
            }
        result["reports"][Path(path).name] = per_rule
    write(out, result)
    print(f"scans: reproduction problems: {len(result['problems'])}")
    for problem in result["problems"]:
        print("  PROBLEM", problem)
    for report_name, per_rule in result["reports"].items():
        print(f"  {report_name}")
        today_tracks = set(per_rule[name(TODAY)]["tracks"])
        for rule_name, r in per_rule.items():
            gained = sorted(set(r["tracks"]) - today_tracks)
            lost = sorted(today_tracks - set(r["tracks"]))
            print(f"    {rule_name:>13}: {r['identified']}/{r['referenced']}, wrong {r['wrong']} "
                  f"(new {len(r['new_wrong'])}); segments changed {len(r['changed'])} "
                  f"({sum(c['in_a_slot'] for c in r['changed'])} in a slot); "
                  f"tracks gained {len(gained)}, lost {len(lost)}; "
                  f"shortest true span {r['shortest_confident_true_span']}")


# --- shared panels ------------------------------------------------------------

def shared(clusters, panel_path, path, out):
    panel = load(panel_path)
    report = load(path)
    # `SharedFound` keeps no `fitted` flag. The default matcher refines
    # every candidate with 10 first-pass hits or more and keeps no other
    # above weak, so every detection that can reach a rule was fitted.
    for r in report["results"]:
        for d in r["detections"]:
            d.setdefault("fitted", True)
    recordings = shared_outcomes.expanded(report, shared_outcomes.clusters_of(clusters))
    roles = shared_outcomes.roles_of(panel)
    problems = []
    for r in report["results"]:
        for d in r["detections"]:
            check(f"{r['query']} {r['index']}: {d['asset']} stored {d['level']}",
                  d["level"] == level(d, TODAY), problems)
    result = {"report": Path(path).name, "problems": problems, "rules": {}}

    def outcomes(rule):
        rows = []
        for r in report["results"]:
            family, versions, unjudged, control = roles[r["scenario"]]
            releveled = dict(r, detections=[dict(d, level=level(d, rule)) for d in r["detections"]])
            outcome, wrong, played_too, _ = shared_outcomes.outcome(
                releveled, versions, unjudged, recordings.get(r["scenario"]))
            found = [dict(d, role=shared_outcomes.role(
                dict(d, role=shared_outcomes.reroled(d, recordings.get(r["scenario"]))), versions, unjudged))
                for d in releveled["detections"] if d["role"] != "partner"]
            wrong_all = [d for d in found if d["level"] == "confident" and d["role"] != "played"]
            rows.append({"query": r["query"], "kind": r["kind"], "index": r["index"],
                         "control": control, "outcome": outcome, "wrong": wrong_all,
                         "played_confident": any(d["level"] == "confident" and d["role"] == "played"
                                                 for d in found)})
        return rows

    today = outcomes(TODAY)
    for rule in RULES:
        if not all(available(rule, r["detections"]) for r in report["results"]):
            continue
        rows = outcomes(rule)
        table = defaultdict(Counter)
        new_wrong, changed = [], []
        for before, after in zip(today, rows):
            table[after["index"] + (" (control)" if after["control"] else "")][after["outcome"]] += 1
            keys = {(d["asset"], d["start_seconds"]) for d in before["wrong"]}
            for d in after["wrong"]:
                if (d["asset"], d["start_seconds"]) not in keys:
                    new_wrong.append(cause(d) | {"query": after["query"], "index": after["index"],
                                                "role": d["role"]})
            if before["outcome"] != after["outcome"] or before["played_confident"] != after["played_confident"]:
                changed.append({"query": after["query"], "index": after["index"],
                                "today": before["outcome"], "variant": after["outcome"],
                                "played_confident": [before["played_confident"], after["played_confident"]]})
        result["rules"][name(rule)] = {"outcomes": {k: dict(v) for k, v in sorted(table.items())},
                                       "new_wrong_confident": new_wrong, "changed": changed}
    write(out, result)
    print(f"shared {Path(path).name}: {len(report['results'])} results; reproduction problems: {len(problems)}")
    for rule_name, r in result["rules"].items():
        left = r["outcomes"].get("played left out", {})
        print(f"  {rule_name:>13}: played left out {dict(left)}; new wrong confident detections "
              f"{len(r['new_wrong_confident'])}; changed queries {len(r['changed'])}")


# --- sweeps and recall ----------------------------------------------------------

def queries(out, paths):
    result = {"reports": {}, "problems": []}
    for path in paths:
        report = load(path)
        qs = report["queries"]
        flags = all(d["confident"] == (level(d, TODAY) == "confident")
                    for q in qs for d in q["detections"])
        check(f"{Path(path).name}: stored confident flags", flags, result["problems"])
        stored_recalled = sum(row["recalled"] for row in report["per_speed"])
        stored_wrong = sum(row["wrong_answers"] for row in report["per_speed"])
        has_span = all("start_seconds" in d for q in qs for d in q["detections"])
        per_rule = {"false_detections_over_240": sum(1 for q in qs for d in q["detections"]
                                                     if not d["correct"] and d["hits"] >= 240)}
        for rule in RULES:
            if rule != TODAY and not has_span:
                continue
            if not all(available(rule, q["detections"]) for q in qs):
                continue
            recalled = sum(1 for q in qs if not q["held_out"]
                           and any(d["correct"] and level(d, rule) == "confident" for d in q["detections"]))
            wrong = [cause(d) | {"query": q["asset"], "speed": q["speed_percent"]}
                     for q in qs for d in q["detections"]
                     if not d["correct"] and level(d, rule) == "confident"]
            changed = [cause(d) | {"query": q["asset"], "speed": q["speed_percent"], "correct": d["correct"],
                                   "today": level(d, TODAY), "variant": level(d, rule)}
                       for q in qs for d in q["detections"] if level(d, TODAY) != level(d, rule)]
            if rule == TODAY:
                check(f"{Path(path).name}: recalled {recalled} against {stored_recalled}",
                      recalled == stored_recalled, result["problems"])
                check(f"{Path(path).name}: wrong {len(wrong)} against {stored_wrong}",
                      len(wrong) == stored_wrong, result["problems"])
            per_rule[name(rule)] = {"indexed": sum(1 for q in qs if not q["held_out"]),
                                    "recalled": recalled, "wrong": wrong, "changed": changed}
        result["reports"][Path(path).name] = per_rule
    write(out, result)
    print(f"queries: reproduction problems: {len(result['problems'])}")
    for problem in result["problems"]:
        print("  PROBLEM", problem)
    for report_name, per_rule in result["reports"].items():
        cells = "; ".join(f"{k}: {v['recalled']}/{v['indexed']} wrong {len(v['wrong'])} changed {len(v['changed'])}"
                          for k, v in per_rule.items() if isinstance(v, dict))
        print(f"  {report_name}: false detections of 240 hits or more: "
              f"{per_rule['false_detections_over_240']}; {cells}")


# --- robust ---------------------------------------------------------------------

def robust(out, path):
    report = load(path)
    qs = report["queries"]
    # `RobustFound` keeps no `fitted` flag either (see `shared`).
    for q in qs:
        for d in q["found"]:
            d.setdefault("fitted", True)
    problems = []
    for q in qs:
        own = [d for d in q["found"] if d["role"] == "own"]
        other = [d for d in q["found"] if d["role"] == "other"]
        partner = [d for d in q["found"] if d["role"] == "partner"]
        for d in q["found"]:
            check(f"{q['condition']} {q['number']}: {d['asset']} level", d["level"] == level(d, TODAY), problems)
        check(f"{q['condition']} {q['number']}: confident",
              q["confident"] == any(level(d, TODAY) == "confident" for d in own), problems)
        check(f"{q['condition']} {q['number']}: wrong confident",
              q["wrong_confident"] == sum(level(d, TODAY) == "confident" for d in other), problems)
        check(f"{q['condition']} {q['number']}: wrong possible",
              q["wrong_possible"] == sum(level(d, TODAY) == "possible" for d in other), problems)
        check(f"{q['condition']} {q['number']}: partner",
              q["partner_confident"] == any(level(d, TODAY) == "confident" for d in partner), problems)
    result = {"report": Path(path).name, "problems": problems[:50], "problem_count": len(problems),
              "false_detections_over_240": sum(1 for q in qs for d in q["found"]
                                               if d["role"] == "other" and d["hits"] >= 240),
              "rules": {}}
    for rule in RULES:
        rows = defaultdict(Counter)
        changed, new_wrong = [], []
        for q in qs:
            key = q["condition"]
            own = [d for d in q["found"] if d["role"] == "own"]
            other = [d for d in q["found"] if d["role"] == "other"]
            if not q["held_out"]:
                rows[key]["indexed"] += 1
                rows[key]["confident"] += any(level(d, rule) == "confident" for d in own)
            rows[key]["wrong confident"] += sum(level(d, rule) == "confident" for d in other)
            for d in q["found"]:
                if level(d, TODAY) != level(d, rule):
                    changed.append(cause(d) | {"condition": key, "number": q["number"], "role": d["role"],
                                               "today": level(d, TODAY), "variant": level(d, rule)})
                    if d["role"] == "other" and level(d, rule) == "confident":
                        new_wrong.append(changed[-1])
        result["rules"][name(rule)] = {"conditions": {k: dict(v) for k, v in rows.items()},
                                       "confident": sum(v["confident"] for v in rows.values()),
                                       "wrong": sum(v["wrong confident"] for v in rows.values()),
                                       "new_wrong_confident": new_wrong, "changed": changed}
    write(out, result)
    print(f"robust {Path(path).name}: {len(qs)} queries; reproduction problems: {len(problems)}; "
          f"false detections of 240 hits or more: {result['false_detections_over_240']}")
    for rule_name, r in result["rules"].items():
        print(f"  {rule_name:>13}: own confident {r['confident']}, wrong {r['wrong']} "
              f"(new {len(r['new_wrong_confident'])}), changed {len(r['changed'])}")


# --- identity ---------------------------------------------------------------------

TIMINGS = {"wall_seconds", "search_seconds"}


def same(m, v, where, differences):
    """Whether `v` holds everything `m` holds: keys only in `v` (the
    variant's additions) and timings aside."""
    if isinstance(m, dict) and isinstance(v, dict):
        for key, value in m.items():
            if key in TIMINGS:
                continue
            if key not in v:
                differences.append(f"{where}.{key}: missing")
                continue
            same(value, v[key], f"{where}.{key}", differences)
    elif isinstance(m, list) and isinstance(v, list):
        if len(m) != len(v):
            differences.append(f"{where}: {len(m)} against {len(v)} items")
            return
        for i, (a, b) in enumerate(zip(m, v)):
            same(a, b, f"{where}[{i}]", differences)
    elif m != v:
        differences.append(f"{where}: {m!r} against {v!r}")


def identity(m_path, v_path):
    differences = []
    same(load(m_path), load(v_path), "", differences)
    print(f"identity {Path(m_path).name}: {'identical' if not differences else f'{len(differences)} differences'}")
    for difference in differences[:20]:
        print("  ", difference)
    return not differences


def main():
    args = sys.argv[1:]
    command = args.pop(0)
    if command == "grid":
        grid(*args)
    elif command == "mixes":
        mixes(args[0], args[1], args[2:])
    elif command == "scans":
        nas = args[0] == "--nas"
        if nas:
            args.pop(0)
        scans(nas, args[0], args[1], args[2:])
    elif command == "shared":
        shared(*args)
    elif command == "queries":
        queries(args[0], args[1:])
    elif command == "robust":
        robust(*args)
    elif command == "identity":
        sys.exit(0 if identity(*args) else 1)
    else:
        raise SystemExit(__doc__)


if __name__ == "__main__":
    main()
