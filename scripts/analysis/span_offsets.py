"""Session 29 (experiments 0146 and 0147): today's confident rule and the
variants that replace its 3-window condition, on the shared panels' brief
passages placed at every offset of the 10 s window grid.

The report is a `gunfinger-eval shared` run of the variant build with
`S29_OFFSETS_ONLY` (queries `<scenario>-silence-<o>s-alone-<L>s`: the
first L s of the passage after o s of digital silence). A result is one
query against one index; a wrong answer is a file other than the played
recording's named confidently in a result (`span_rule.py shared`'s
scoring, `shared_outcomes.py`'s roles with the clusters' rips). A passage
is (scenario, L, index); its results are its offsets.

The fair form of criterion 1 (brief 29, step 3), both parts required:
  F1  the variant's wrong answers, summed over every passage and offset,
      are no more than today's rule's, on each panel;
  F2  no passage is wrong under the variant at any offset while today's
      rule is wrong at none of its offsets.
Brief 27's criterion 1 as written (C27), beside: no (result, file) is
confident and wrong under the variant that is not under today's rule.

usage: span_offsets.py CLUSTERS PANEL REPORT OUT
       span_offsets.py same OLD_REPORT REPORT   (every result of OLD, a run
                                                 of 0142, found in REPORT by
                                                 query and index, with the
                                                 same detections)

Refuses any path naming the held-out test set.
"""

import json
import re
import sys
from collections import Counter, defaultdict
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import shared_outcomes  # noqa: E402
import span_rule  # noqa: E402

NAME = re.compile(r"^(?P<scenario>.+)-silence-(?P<offset>\d+)s-alone-(?P<length>\d+)s$")


def wrong_files(found, rule):
    return sorted({d["asset"] for d in found
                   if span_rule.level(d, rule) == "confident" and d["role"] != "played"})


def played_confident(found, rule):
    return any(span_rule.level(d, rule) == "confident" and d["role"] == "played" for d in found)


DETECTION_FIELDS = ["asset", "role", "start_seconds", "end_seconds", "track_start_seconds",
                    "track_end_seconds", "speed", "windows", "hits", "level", "supported_seconds"]


def same(old_path, new_path):
    old, new = span_rule.load(old_path), span_rule.load(new_path)
    found = {(r["query"], r["index"]): r for r in new["results"]}
    differences, compared = [], 0
    for r in old["results"]:
        other = found.get((r["query"], r["index"]))
        if other is None:
            differences.append(f"{r['query']} {r['index']}: missing")
            continue
        compared += 1
        a = [[d.get(f) for f in DETECTION_FIELDS] for d in r["detections"]]
        b = [[d.get(f) for f in DETECTION_FIELDS] for d in other["detections"]]
        if a != b:
            differences.append(f"{r['query']} {r['index']}: detections differ")
    print(f"same {Path(old_path).name}: {compared} of {len(old['results'])} results found; "
          f"differences: {len(differences)}")
    for difference in differences[:20]:
        print("  DIFFERENT", difference)
    return not differences


def main(clusters_path, panel_path, report_path, out_path):
    panel = span_rule.load(panel_path)
    report = span_rule.load(report_path)
    recordings = shared_outcomes.expanded(report, shared_outcomes.clusters_of(clusters_path))
    roles = shared_outcomes.roles_of(panel)
    problems, results = [], []
    for r in report["results"]:
        match = NAME.match(r["query"])
        if match is None:
            problems.append(f"{r['query']}: not a silence-led query")
            continue
        if r.get("error"):
            problems.append(f"{r['query']} {r['index']}: not rendered ({r['error']})")
            continue
        _, versions, unjudged, _ = roles[r["scenario"]]
        found = []
        for d in r["detections"]:
            # `SharedFound` keeps no `fitted` flag; every detection that can
            # reach a rule under the default matcher was fitted (brief 27).
            d = dict(d, fitted=True)
            if d["level"] != span_rule.level(d, span_rule.TODAY):
                problems.append(f"{r['query']} {r['index']}: {d['asset']} stored {d['level']}")
            if d["role"] == "partner":
                continue
            role = shared_outcomes.role(
                dict(d, role=shared_outcomes.reroled(d, recordings.get(r["scenario"]))),
                versions, unjudged)
            found.append(dict(d, role=role))
        results.append({"scenario": r["scenario"], "length": int(match["length"]),
                        "offset": int(match["offset"]), "index": r["index"],
                        "query": r["query"], "found": found})

    offsets = sorted({r["offset"] for r in results})
    out = {"report": Path(report_path).name, "problems": problems, "results": len(results),
           "offsets": offsets, "rules": {}}
    passages = defaultdict(list)
    for r in results:
        passages[(r["scenario"], r["length"], r["index"])].append(r)
    today_wrong = {id(r): wrong_files(r["found"], span_rule.TODAY) for r in results}
    today_level = {(id(r), i): span_rule.level(d, span_rule.TODAY)
                   for r in results for i, d in enumerate(r["found"])}
    for rule in span_rule.RULES:
        if not all(span_rule.available(rule, r["found"]) for r in results):
            continue
        wrong = {id(r): wrong_files(r["found"], rule) for r in results}
        by_offset = Counter()
        by_index = Counter()
        right_by_offset = Counter()
        for r in results:
            by_offset[r["offset"]] += len(wrong[id(r)])
            by_index[r["index"]] += len(wrong[id(r)])
            if r["index"] != "played left out" and played_confident(r["found"], rule):
                right_by_offset[r["offset"]] += 1
        new_wrong = [{"query": r["query"], "index": r["index"], "file": f,
                      "detections": [span_rule.cause(d) for d in r["found"] if d["asset"] == f]}
                     for r in results for f in wrong[id(r)] if f not in today_wrong[id(r)]]
        f2 = []
        grid_dependence = []
        for (scenario, length, index), cell in sorted(passages.items()):
            today_at = sorted(r["offset"] for r in cell if today_wrong[id(r)])
            rule_at = sorted(r["offset"] for r in cell if wrong[id(r)])
            if today_at or rule_at:
                grid_dependence.append({"scenario": scenario, "length": length, "index": index,
                                        "offsets": len(cell), "today_wrong_at": today_at,
                                        "rule_wrong_at": rule_at,
                                        "files": sorted({f for r in cell for f in wrong[id(r)]})})
            if rule_at and not today_at:
                f2.append({"scenario": scenario, "length": length, "index": index,
                           "wrong_at": rule_at,
                           "files": sorted({f for r in cell for f in wrong[id(r)]})})
        changes = Counter()
        for r in results:
            for i, d in enumerate(r["found"]):
                before, after = today_level[(id(r), i)], span_rule.level(d, rule)
                if before != after:
                    side = "right" if d["role"] == "played" else "wrong"
                    changes[f"{side} {before} -> {after}"] += 1
        out["rules"][span_rule.name(rule)] = {
            "wrong": sum(by_offset.values()),
            "wrong_by_offset": {o: by_offset[o] for o in offsets},
            "wrong_by_index": dict(sorted(by_index.items())),
            "right_confident": sum(right_by_offset.values()),
            "right_confident_by_offset": {o: right_by_offset[o] for o in offsets},
            "new_wrong_c27": new_wrong,
            "f2_passages": f2,
            "grid_dependence": grid_dependence,
            "level_changes": dict(sorted(changes.items())),
        }
    today_total = out["rules"][span_rule.name(span_rule.TODAY)]["wrong"]
    for rule_name, r in out["rules"].items():
        r["f1"] = r["wrong"] <= today_total
        r["f2"] = not r["f2_passages"]
        r["c27"] = not r["new_wrong_c27"]
    span_rule.write(out_path, out)
    print(f"{Path(report_path).name}: {len(results)} results, {len(passages)} passages, "
          f"offsets {offsets}; reproduction problems: {len(problems)}")
    for rule_name, r in out["rules"].items():
        print(f"  {rule_name:>13}: wrong {r['wrong']:>3} by offset "
              f"{list(r['wrong_by_offset'].values())}; right {r['right_confident']:>3}; "
              f"F1 {'yes' if r['f1'] else 'no'}, F2 {'yes' if r['f2'] else 'no'} "
              f"({len(r['f2_passages'])}), C27 {'yes' if r['c27'] else 'no'} "
              f"({len(r['new_wrong_c27'])})")


if __name__ == "__main__":
    if sys.argv[1] == "same":
        sys.exit(0 if same(*sys.argv[2:]) else 1)
    main(*sys.argv[1:])
