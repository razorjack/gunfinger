"""Session 29 (experiments 0146 and 0148): today's confident rule and the
variants that replace its 3-window condition (`span_rule.py`'s rules) on
session 28's brief plays of the recall panel's development half at NAS
scale, and on the generated-mix plays only Panako names (0150).

usage:
  span_recall.py same S28_REPORT V_REPORT     the same queries, and today's
                                              detections identical
  span_recall.py count OUT V_REPORT           per rule: confident recall at
                                              15 and 20 s, by where the play
                                              starts in a 10 s window; wrong
                                              answers; held-out and
                                              unobservable queries confident
  span_recall.py panako OUT MIXES_REPORT CLUSTERS
                                              each of 0150's seven plays under
                                              each rule

A searched query is one of a source not held out and, under other rips
only, observable. A wrong answer is a detection outside the query's own
cluster that is not its partner's. Refuses any path naming the held-out
test set.
"""

import json
import math
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import devscore  # noqa: E402
import span_rule  # noqa: E402

QUERY_FIELDS = ["asset", "held_out", "speed_percent", "start_seconds", "kind", "family",
                "unobservable", "partner", "miss", "brief"]
DETECTION_FIELDS = ["asset", "correct", "confident", "partner", "windows", "hits",
                    "speed_percent", "fitted"]
# 0150, generated mixes of seed 2026: the plays Panako names and Gunfinger
# reports as possible, by mix and a word of the file name.
PANAKO_ONLY = [("mix-02", "kosmic"), ("mix-03", "fanfare"), ("mix-04", "movement"),
               ("mix-06", "again_and_again"), ("mix-07", "soul_sucker"),
               ("mix-09", "pulse"), ("mix-10", "dominion")]


def same(m_path, v_path):
    m, v = span_rule.load(m_path), span_rule.load(v_path)
    differences = []
    if len(m["queries"]) != len(v["queries"]):
        differences.append(f"{len(m['queries'])} queries against {len(v['queries'])}")
    for i, (a, b) in enumerate(zip(m["queries"], v["queries"])):
        for field in QUERY_FIELDS:
            if a.get(field) != b.get(field):
                differences.append(f"query {i} {field}: {a.get(field)!r} against {b.get(field)!r}")
        if len(a["detections"]) != len(b["detections"]):
            differences.append(f"query {i}: {len(a['detections'])} detections against "
                               f"{len(b['detections'])}")
            continue
        for j, (x, y) in enumerate(zip(a["detections"], b["detections"])):
            for field in DETECTION_FIELDS:
                if x.get(field) != y.get(field):
                    differences.append(f"query {i} detection {j} {field}: "
                                       f"{x.get(field)!r} against {y.get(field)!r}")
    print(f"same {Path(m_path).name} {Path(v_path).name}: {len(m['queries'])} queries, "
          f"{sum(len(q['detections']) for q in m['queries'])} detections; "
          f"differences: {len(differences)}")
    for difference in differences[:20]:
        print("  DIFFERENT", difference)
    return not differences


def searched(q):
    return not q["held_out"] and not q.get("unobservable", False)


def count(out, path):
    report = span_rule.load(path)
    qs = report["queries"]
    problems = [f"{q['asset']} {q['speed_percent']}: {d['asset']} stored confident {d['confident']}"
                for q in qs for d in q["detections"]
                if d["confident"] != (span_rule.level(d, span_rule.TODAY) == "confident")]
    result = {"report": Path(path).name, "problems": problems,
              "searched": dict(Counter(q["brief"]["seconds"] for q in qs if searched(q))),
              "held_out_or_unobservable": dict(Counter(q["brief"]["seconds"] for q in qs
                                                       if not searched(q))),
              "rules": {}}
    for rule in span_rule.RULES:
        if not all(span_rule.available(rule, q["detections"]) for q in qs):
            continue
        if rule[0] == "span" and not all("start_seconds" in d for q in qs for d in q["detections"]):
            continue
        rows = {}
        recalled = Counter()
        by_position = defaultdict(Counter)
        position_queries = defaultdict(Counter)
        wrong_confident, wrong_possible, others_confident, changed = [], 0, [], Counter()
        for q in qs:
            length = q["brief"]["seconds"]
            levels = [(d, span_rule.level(d, rule)) for d in q["detections"]]
            if searched(q):
                position = math.floor(q["brief"]["starts_at_seconds"] % 10.0)
                position_queries[length][position] += 1
                if any(d["correct"] and lv == "confident" for d, lv in levels):
                    recalled[length] += 1
                    by_position[length][position] += 1
            elif any(lv == "confident" and not d.get("partner", False) for d, lv in levels):
                others_confident.append({"asset": q["asset"], "speed": q["speed_percent"],
                                         "seconds": length})
            for d, lv in levels:
                if d["correct"] or d.get("partner", False):
                    pass
                elif lv == "confident":
                    wrong_confident.append(span_rule.cause(d) | {"query": q["asset"],
                                                                 "speed": q["speed_percent"],
                                                                 "seconds": length})
                elif lv == "possible":
                    wrong_possible += 1
                before = span_rule.level(d, span_rule.TODAY)
                if before != lv:
                    side = "right" if d["correct"] else ("partner" if d.get("partner") else "wrong")
                    changed[f"{side} {before} -> {lv}"] += 1
        rows["recalled"] = {str(k): v for k, v in sorted(recalled.items())}
        rows["recalled_by_position"] = {
            str(length): {str(p): [by_position[length][p], position_queries[length][p]]
                          for p in range(10)}
            for length in sorted(position_queries)}
        rows["wrong_confident"] = wrong_confident
        rows["wrong_possible_only"] = wrong_possible
        rows["held_out_or_unobservable_confident"] = others_confident
        rows["level_changes"] = dict(sorted(changed.items()))
        result["rules"][span_rule.name(rule)] = rows
    span_rule.write(out, result)
    print(f"count {Path(path).name}: searched {result['searched']}, held out or unobservable "
          f"{result['held_out_or_unobservable']}; reproduction problems: {len(problems)}")
    for rule_name, rows in result["rules"].items():
        positions = "; ".join(
            f"{length} s by second of the window "
            + " ".join(f"{hit}/{n}" for hit, n in cells.values())
            for length, cells in rows["recalled_by_position"].items())
        print(f"  {rule_name:>13}: recalled {rows['recalled']}; wrong confident "
              f"{len(rows['wrong_confident'])}, possible only {rows['wrong_possible_only']}; "
              f"held out or unobservable confident {len(rows['held_out_or_unobservable_confident'])}")
        print(f"  {'':>13}  {positions}")


def panako(out, path, clusters):
    report = span_rule.load(path)
    cluster_of = devscore.clusters_of(clusters)
    mixes = {m["mix"]["name"]: m for m in report["mixes"]}
    problems, plays = [], []
    for name, word in PANAKO_ONLY:
        m = mixes[name]
        found = [i for i, p in enumerate(m["mix"]["plays"]) if word in p["asset"].lower()]
        if len(found) != 1:
            problems.append(f"{name} {word}: {len(found)} plays")
            continue
        i = found[0]
        play = m["mix"]["plays"][i]
        today, _, _ = span_rule.mix_score(m["mix"], m["detections"], cluster_of, span_rule.TODAY)
        if today[i] != m["score"]["plays"][i]["level"]:
            problems.append(f"{name} {word}: today {today[i]} against stored "
                            f"{m['score']['plays'][i]['level']}")
        accepted = cluster_of(play["asset"])
        matching = [d for d in m["detections"] if d["asset"] in accepted
                    and d["start_seconds"] < play["end_seconds"]
                    and play["start_seconds"] < d["end_seconds"]]
        best = max(matching, key=lambda d: d["hits"])
        levels = {}
        for rule in span_rule.RULES:
            if span_rule.available(rule, m["detections"]):
                levels[span_rule.name(rule)] = span_rule.mix_score(
                    m["mix"], m["detections"], cluster_of, rule)[0][i]
        plays.append({"mix": name, "file": Path(play["asset"]).name,
                      "play_seconds": round(play["end_seconds"] - play["start_seconds"], 1),
                      "best": span_rule.cause(best), "levels": levels})
    span_rule.write(out, {"report": Path(path).name, "problems": problems, "plays": plays})
    print(f"panako {Path(path).name}: reproduction problems: {len(problems)}")
    for problem in problems:
        print("  PROBLEM", problem)
    for p in plays:
        confident = [rule for rule, level in p["levels"].items() if level == "confident"]
        print(f"  {p['mix']} {p['file'][:50]:<50} {p['play_seconds']:>5} s: hits {p['best']['hits']}, "
              f"windows {p['best']['windows']}, span {p['best'].get('span')}, supported "
              f"{p['best'].get('supported')}; confident under {', '.join(confident) or 'none'}")


def main():
    command, *args = sys.argv[1:]
    for arg in args:
        devscore.refuse_test_set(arg)
    if command == "same":
        sys.exit(0 if same(*args) else 1)
    elif command == "count":
        count(*args)
    elif command == "panako":
        panako(*args)
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main()
