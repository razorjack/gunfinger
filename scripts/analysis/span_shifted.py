"""Session 27, experiment 0141: a shared panel's short `alone` queries
against the same passage shifted off the window grid.

The shifted report comes from the variant worktree's harness with
`S27_SHIFTED_ONLY` set: each scenario's queries are up to 20 s of its
passage after 3, 5 and 7 s of the played file's own material, named
`<scenario>-before-<b>s-alone-<a>s`. For every `alone` query of at most
20 s in the main report (2 windows, never confident under today's rule)
this pairs its outcome under each rule with the shifted queries'
outcomes under today's rule, in the same index, and lists, for each
wrong confident answer a rule gives on the short file, whether today's
rule names the same file confidently in the shifted queries.

usage: span_shifted.py CLUSTERS PANEL MAIN_REPORT SHIFTED_REPORT [OUT.json]
"""
import json
import math
import sys
from collections import Counter
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import shared_outcomes  # noqa: E402
import span_rule  # noqa: E402

clusters_path, panel_path, main_path, shifted_path = sys.argv[1:5]
for path in sys.argv[1:]:
    if "ed-rush" in path:
        raise SystemExit(f"refused: {path} names the held-out test set")
panel = json.load(open(panel_path))
roles = shared_outcomes.roles_of(panel)
cluster_of = shared_outcomes.clusters_of(clusters_path)
reports = {}
for key, path in (("main", main_path), ("shifted", shifted_path)):
    report = json.load(open(path))
    for r in report["results"]:
        for d in r["detections"]:
            d.setdefault("fitted", True)
    reports[key] = (report, shared_outcomes.expanded(report, cluster_of))


def judged(key, result, rule):
    report, recordings = reports[key]
    family, versions, unjudged, control = roles[result["scenario"]]
    relevelled = dict(result, detections=[dict(d, level=span_rule.level(d, rule))
                                          for d in result["detections"]])
    outcome, wrong, _, _ = shared_outcomes.outcome(relevelled, versions, unjudged,
                                                   recordings.get(result["scenario"]))
    return outcome, wrong


shifted = {}
for r in reports["shifted"][0]["results"]:
    shifted.setdefault((r["scenario"], r["index"]), []).append(r)
pairs = {span_rule.name(rule): Counter() for rule in span_rule.RULES}
wrong_rows = []
problems = 0
for r in reports["main"][0]["results"]:
    if r["kind"] != "alone" or math.ceil(r["seconds"] / 10.0 - 1e-9) > 2:
        continue
    alone = f"{r['seconds']:.0f}s"
    partners = [s for s in shifted.get((r["scenario"], r["index"]), [])
                if s["query"].endswith(f"-alone-{alone}")]
    if not partners:
        continue
    today_shifted = [judged("shifted", s, span_rule.TODAY) for s in partners]
    for d in r["detections"]:
        problems += d["level"] != span_rule.level(d, span_rule.TODAY)
    for rule in span_rule.RULES:
        outcome, wrong = judged("main", r, rule)
        for shifted_outcome, _ in today_shifted:
            pairs[span_rule.name(rule)][(outcome, shifted_outcome)] += 1
        if wrong is not None:
            named = [s["query"] for s, (o, w) in zip(partners, today_shifted)
                     if any(d["asset"] == wrong["asset"] and span_rule.level(d, span_rule.TODAY) == "confident"
                            for d in s["detections"])]
            wrong_rows.append({"rule": span_rule.name(rule), "query": r["query"], "index": r["index"],
                               "wrong": wrong["asset"], "hits": wrong["hits"],
                               "shifted": len(partners), "named_confidently_today_in": named})

print(f"{Path(shifted_path).name}: reproduction problems on the short files {problems}")
for rule, counter in pairs.items():
    print(f"  {rule:>14} on the short file -> today's rule shifted:")
    for (outcome, shifted_outcome), n in sorted(counter.items()):
        print(f"      {n:4}  {outcome} -> {shifted_outcome}")
for row in wrong_rows:
    print(f"  {row['rule']:>14}: {row['query']} ({row['index']}) names {row['wrong'].split('/')[-1]}; "
          f"today's rule names it in {len(row['named_confidently_today_in'])} of {row['shifted']} shifted")
if len(sys.argv) > 5:
    json.dump({"main": Path(main_path).name, "shifted": Path(shifted_path).name,
               "reproduction_problems": problems,
               "pairs": {rule: [{"short": k[0], "shifted_today": k[1], "queries": n}
                                for k, n in sorted(c.items())] for rule, c in pairs.items()},
               "wrong": wrong_rows}, open(sys.argv[5], "w"), indent=1)
