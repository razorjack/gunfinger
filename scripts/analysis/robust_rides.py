"""2026-10-22 (session 38): `robust` reports of the rides inside excerpts
under several matchers (experiment 0180). Per condition, the two base
speeds together: indexed excerpts confident on their own cluster, at least
possible, wrong confident detections, and the median hit retention
against the control; then, per excerpt, why a ridden excerpt is missed
(its best detection's windows and hits).

usage: python3 -I scripts/analysis/robust_rides.py LABEL=REPORT ...
"""

import collections
import json
import sys


def main():
    for arg in sys.argv[1:]:
        label, path = arg.split("=", 1)
        report = json.load(open(path))
        rows = collections.defaultdict(lambda: collections.Counter())
        retention = collections.defaultdict(list)
        for row in report["rows"]:
            r = rows[row["condition"]]
            for key in ("indexed", "confident", "possible_or_better", "wrong_confident", "wrong_possible"):
                r[key] += row[key]
            retention[row["condition"]].append(row["median_hit_retention"])
        print(f"{label} ({report['indexed_assets']} assets)")
        for condition, r in rows.items():
            print(
                f"  {condition:18} confident {r['confident']:3}/{r['indexed']}  possible+ {r['possible_or_better']:3}"
                f"  wrong confident {r['wrong_confident']}  wrong possible {r['wrong_possible']}"
                f"  retention {', '.join(f'{x:.2f}' for x in retention[condition])}"
            )
        missed = collections.Counter()
        for q in report["queries"]:
            if q["held_out"] or q["confident"] or not q["condition"].startswith("ride"):
                continue
            reason = "under 3 windows" if q["best_windows"] < 3 else "under 240 hits"
            missed[(q["condition"], reason)] += 1
        for (condition, reason), n in sorted(missed.items()):
            print(f"    missed {condition}: {reason} {n}")


if __name__ == "__main__":
    main()
