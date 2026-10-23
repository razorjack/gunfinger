"""2026-10-23 (session 42): `robust` reports of the stereo faults under the
query channel choices (experiments 0194 and after), at one scale.

Per condition and choice, over the 80 indexed queries: confident and
possible or better on the excerpt's own cluster, split by the excerpt's
width (its `stereo-control` side-to-mid ratio, the mean of its two
speeds: "mono or nearly mono" at -20 dB or less, else "stereo"); the median
hit retention against the same query's `stereo-control` under `downmix`;
over all 100 queries, wrong confident and possible detections, the
strongest wrong hits, held-out queries with a confident detection, and
wrong detections a fault or choice raises (an asset on a query that
`stereo-control` under `downmix` does not detect at that level); and the
share of queries whose side is louder than their mid (the trigger).
`either` is the union of the `left` and `right` reports, query by query;
two exploratory unions follow, not pre-registered: `downmix+side` and
`downmix+either` (the downmix searched first and the other choices
beside it).

usage: python3 -I scripts/analysis/robust_stereo.py OUT.json CHOICE=REPORT ...
"""

import json
import statistics
import sys

CONTROL = "stereo-control"
NARROW_DB = -20.0


def key(query):
    return (query["condition"], query["speed"], query["number"])


def stereo_queries(report):
    return {key(q): q for q in report["queries"] if q.get("stereo")}


def widths(queries):
    """The excerpt's width: the mean of its two speeds' control ratios."""
    ratios = {}
    for (condition, _, number), q in queries.items():
        if condition == CONTROL:
            ratios.setdefault(number, []).append(q["stereo"]["side_to_mid_db"])
    return {number: statistics.mean(values) for number, values in ratios.items()}


def either(*choices):
    """Search each choice and keep every detection: per query, the union.
    A wrong asset found at both levels counts at the higher one."""
    union = {}
    for k, first in choices[0].items():
        here = [choice[k] for choice in choices]
        wrong = {}
        for q in here:
            for w in q["stereo"]["wrong"]:
                if wrong.get(w["asset"], {}).get("level") != "confident":
                    wrong[w["asset"]] = w
        union[k] = dict(
            first,
            best_hits=max(q["best_hits"] for q in here),
            confident=any(q["confident"] for q in here),
            possible=any(q["possible"] for q in here),
            wrong_confident=sum(1 for w in wrong.values() if w["level"] == "confident"),
            wrong_possible=sum(1 for w in wrong.values() if w["level"] == "possible"),
            strongest_wrong_hits=max(q["strongest_wrong_hits"] for q in here),
            stereo=dict(first["stereo"], wrong=list(wrong.values())),
        )
    return union


def wrong_set(query, level):
    return {w["asset"] for w in query["stereo"]["wrong"] if w["level"] == level}


def summarise(choices):
    base = choices["downmix"]
    width = widths(base)
    rows = []
    conditions = sorted({k[0] for k in base}, key=lambda c: (c != CONTROL, c))
    for condition in conditions:
        for choice, queries in choices.items():
            here = [q for k, q in queries.items() if k[0] == condition]
            indexed = [q for q in here if not q["held_out"]]
            narrow = [q for q in indexed if width[q["number"]] <= NARROW_DB]
            stereo = [q for q in indexed if width[q["number"]] > NARROW_DB]
            control = {k: q for k, q in base.items() if k[0] == CONTROL}
            retention = [
                q["best_hits"] / max(control[(CONTROL, q["speed"], q["number"])]["best_hits"], 1)
                for q in indexed
            ]
            raised = {"confident": 0, "possible": 0}
            raised_assets = set()
            for q in here:
                c = control[(CONTROL, q["speed"], q["number"])]
                for level in raised:
                    new = wrong_set(q, level) - wrong_set(c, "confident") - wrong_set(c, "possible")
                    raised[level] += len(new)
                    raised_assets |= new
            rows.append(
                dict(
                    condition=condition,
                    choice=choice,
                    indexed=len(indexed),
                    confident=sum(q["confident"] for q in indexed),
                    possible_or_better=sum(q["possible"] for q in indexed),
                    narrow=len(narrow),
                    narrow_confident=sum(q["confident"] for q in narrow),
                    stereo=len(stereo),
                    stereo_confident=sum(q["confident"] for q in stereo),
                    median_retention=statistics.median(retention) if retention else 0.0,
                    wrong_confident=sum(q["wrong_confident"] for q in here),
                    wrong_possible=sum(q["wrong_possible"] for q in here),
                    strongest_wrong=max((q["strongest_wrong_hits"] for q in here), default=0),
                    held_out_confident=sum(1 for q in here if q["held_out"] and q["wrong_confident"]),
                    raised_confident=raised["confident"],
                    raised_possible=raised["possible"],
                    raised_assets=sorted(raised_assets),
                    side_louder=sum(1 for q in here if q["stereo"]["side_to_mid_db"] > 0.0),
                    queries=len(here),
                )
            )
    return dict(widths={str(n): w for n, w in sorted(width.items())}, rows=rows)


def main():
    out = sys.argv[1]
    choices = {}
    for arg in sys.argv[2:]:
        choice, path = arg.split("=", 1)
        choices[choice] = stereo_queries(json.load(open(path)))
    if "left" in choices and "right" in choices:
        choices["either"] = either(choices["left"], choices["right"])
        if "side" in choices:
            choices["downmix+side"] = either(choices["downmix"], choices["side"])
        choices["downmix+either"] = either(choices["downmix"], choices["left"], choices["right"])
    summary = summarise(choices)
    json.dump(summary, open(out, "w"), indent=1, ensure_ascii=False)
    narrow = sum(1 for w in summary["widths"].values() if w <= NARROW_DB)
    print(f"excerpts {len(summary['widths'])}, mono or nearly mono {narrow}")
    print(
        f"{'condition':24} {'choice':14} {'conf':>5} {'poss+':>5} {'mono+':>7} {'stereo':>7} {'ret':>5}"
        f" {'w.conf':>6} {'w.poss':>6} {'w.max':>5} {'ho.c':>4} {'r.conf':>6} {'r.poss':>6} {'S>M':>4}"
    )
    for r in summary["rows"]:
        print(
            f"{r['condition']:24} {r['choice']:14} {r['confident']:5} {r['possible_or_better']:5}"
            f" {r['narrow_confident']:3}/{r['narrow']:<3} {r['stereo_confident']:3}/{r['stereo']:<3}"
            f" {r['median_retention']:5.2f} {r['wrong_confident']:6} {r['wrong_possible']:6}"
            f" {r['strongest_wrong']:5} {r['held_out_confident']:4} {r['raised_confident']:6}"
            f" {r['raised_possible']:6} {r['side_louder']:4}"
        )


if __name__ == "__main__":
    main()
