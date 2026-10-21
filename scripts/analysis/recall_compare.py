"""Session 33: recall summaries of the Linux machine against the Mac's
(experiments 0144 and 0145), condition by condition and within each
condition by speed, kind, brief length and grid phase.

Both sides are `recall_conditions.py --json` summaries; the Mac's are the
data files of 0144 (single conditions) and 0145 (`conditions`, combined).
The Mac's data files hold no single query, so the finest comparison is a
breakdown's cell: indexed queries, confident, possible or better, and the
misses by reason.

usage: recall_compare.py DATA_0144 DATA_0145 HERE.json [OUT.json]
"""
import json
import sys

FIELDS = ("indexed", "confident", "possible_or_better")
SCALARS = ("indexed_assets", "left_out_records", "unobservable_queries", "unobservable_sources",
           "unobservable_confident", "held_out_queries", "held_out_confident",
           "wrong_possible_count")
BREAKDOWNS = ("by_speed", "by_kind", "by_length", "by_grid_phase")


def cell(entry):
    return [entry.get(f) for f in FIELDS] + [entry.get("misses", {})]


mac = json.load(open(sys.argv[1]))
mac.update(json.load(open(sys.argv[2]))["conditions"])
here = json.load(open(sys.argv[3]))
out = {}
for report in sorted(set(mac) | set(here)):
    m, h = mac.get(report), here.get(report)
    if m is None or h is None:
        print(f"{report}: only {'here' if m is None else 'on the Mac'}")
        continue
    row = {
        "all": {"mac": cell(m["all"]), "here": cell(h["all"])},
        "scalars": {k: [m.get(k), h.get(k)] for k in SCALARS},
        "wrong_confident": [len(m["wrong_confident"]), len(h["wrong_confident"])],
        "wrong_possible_over_200": [len(m["wrong_possible_over_200"]),
                                    len(h["wrong_possible_over_200"])],
        "strongest_wrong_possible": [m["strongest_wrong_possible"][:1],
                                     h["strongest_wrong_possible"][:1]],
        "cells_differing": {},
    }
    for breakdown in BREAKDOWNS:
        keys = set(m.get(breakdown, {})) | set(h.get(breakdown, {}))
        for key in sorted(keys):
            a = cell(m.get(breakdown, {}).get(key, {}))
            b = cell(h.get(breakdown, {}).get(key, {}))
            if a != b:
                row["cells_differing"][f"{breakdown} {key}"] = {"mac": a, "here": b}
    out[report] = row
    same = row["all"]["mac"] == row["all"]["here"]
    print(f"{report}: {'same' if same else 'differs'}")
    print(f"  Mac  {row['all']['mac']}")
    print(f"  here {row['all']['here']}")
    for key, (a, b) in row["scalars"].items():
        if a != b:
            print(f"  {key}: {a} -> {b}")
    print(f"  wrong confident {row['wrong_confident']}, wrong possible over 200 "
          f"{row['wrong_possible_over_200']}")
    for side, strongest in zip(("Mac", "here"), row["strongest_wrong_possible"]):
        for s in strongest:
            print(f"  strongest wrong possible, {side}: {s['found']} for {s['query']} "
                  f"at {s['speed_percent']}%, {s['hits']} hits")
    for key, cells in row["cells_differing"].items():
        print(f"  {key}: {cells['mac']} -> {cells['here']}")
if len(sys.argv) > 4:
    with open(sys.argv[4], "w", encoding="utf-8") as file:
        json.dump(out, file, indent=1, ensure_ascii=False)
