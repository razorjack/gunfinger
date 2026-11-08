"""Session 84: robust's rows per arm, side by side: confident and wrong confident per condition.

gap_search_robust_rows.py LABEL=robust.json ...
"""
import json
import sys

arms = [(a.split("=", 1)[0], json.load(open(a.split("=", 1)[1]))) for a in sys.argv[1:]]
keys = [(r["condition"], r["speed"]) for r in arms[0][1]["rows"]]
print("condition speed indexed | " + " | ".join(f"{l} conf/wrong/wrongposs" for l, _ in arms))
totals = {l: [0, 0, 0] for l, _ in arms}
for k in keys:
    cells = []
    for label, report in arms:
        row = next(r for r in report["rows"] if (r["condition"], r["speed"]) == k)
        cells.append(f"{row['confident']}/{row['wrong_confident']}/{row['wrong_possible']}")
        t = totals[label]
        t[0] += row["confident"]; t[1] += row["wrong_confident"]; t[2] += row["wrong_possible"]
    print(f"{k[0]} {k[1]} {row['indexed']} | " + " | ".join(cells))
print("total | " + " | ".join(f"{l} {t[0]}/{t[1]}/{t[2]}" for l, t in totals.items()))
