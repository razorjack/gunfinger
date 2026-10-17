"""Totals of `gunfinger-eval seconds` outputs per rule, over several files
(session 27): the shares of indexed seconds labelled correctly, possibly,
not at all (and inside a margin), mislabelled seconds, specificity, and
the start and end errors of identified tracks.

usage: seconds_summary.py [--prefix INDEX] OUTPUT.json...
`--prefix` keeps only mixes whose name starts with it (a shared-mixes
index, such as "played left out").
"""
import json
import statistics
import sys
from collections import defaultdict

args = sys.argv[1:]
prefix = None
if args and args[0] == "--prefix":
    prefix = args[1] + ": "
    args = args[2:]
totals = defaultdict(lambda: defaultdict(int))
errors = defaultdict(lambda: ([], []))
for path in args:
    if "ed-rush" in path:
        raise SystemExit(f"refused: {path} names the held-out test set")
    for scored in json.load(open(path)):
        if prefix and not scored["name"].startswith(prefix):
            continue
        for key, value in scored["score"]["seconds"].items():
            totals[scored["rule"]][key] += value
        for boundary in scored["score"]["boundaries"]:
            errors[scored["rule"]][0].append(boundary["start_error_seconds"])
            errors[scored["rule"]][1].append(boundary["end_error_seconds"])


def p90(values):
    ordered = sorted(abs(v) for v in values)
    return ordered[max(0, -(-9 * len(ordered) // 10) - 1)]


for rule, t in totals.items():
    indexed = t["correct"] + t["possible"] + t["unlabelled"] + t["unlabelled_in_margin"] + t["mislabelled"]
    unindexed = t["silent"] + t["mislabelled_unindexed"]
    share = lambda n, of: 100.0 * n / of if of else 0.0
    line = (f"{rule:>14}: indexed {indexed} s: correct {share(t['correct'], indexed):.1f}%, "
            f"possible {share(t['possible'], indexed):.1f}%, unlabelled {share(t['unlabelled'], indexed):.1f}%, "
            f"margin {share(t['unlabelled_in_margin'], indexed):.1f}%, mislabelled {t['mislabelled']} s; "
            f"not indexed {unindexed} s: specificity {share(t['silent'], unindexed):.1f}%, "
            f"mislabelled {t['mislabelled_unindexed']} s")
    starts, ends = errors[rule]
    if starts:
        line += (f"; {len(starts)} tracks: start {statistics.median(starts):+.1f} s (p90 |{p90(starts):.1f}|), "
                 f"end {statistics.median(ends):+.1f} s (p90 |{p90(ends):.1f}|)")
    print(line)
