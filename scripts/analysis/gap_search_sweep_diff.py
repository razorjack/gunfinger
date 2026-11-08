"""Session 84: what an arm's sweep changes against a reference sweep, query by query.

gap_search_sweep_diff.py REF.json ARM.json: queries whose confident or possible (60+ hits) answers differ;
new confident answers (correct or wrong), lost confident answers, new wrong possible answers.
"""
import json
import sys

POSSIBLE = 60


def key(q):
    return (q["asset"], q["held_out"], round(q["start_seconds"], 3), q["speed_percent"])


def answers(q, confident):
    return {(d["asset"], d["correct"]) for d in q["detections"]
            if (d["confident"] if confident else (not d["confident"] and d["hits"] >= POSSIBLE))}


ref = {key(q): q for q in json.load(open(sys.argv[1]))["queries"]}
arm = {key(q): q for q in json.load(open(sys.argv[2]))["queries"]}
assert ref.keys() == arm.keys(), "different queries"
counts = dict(gained_correct=0, lost_correct=0, new_wrong_confident=0, new_wrong_possible=0,
              lost_wrong_possible=0, queries_changed_confident=0, recalled_ref=0, recalled_arm=0)
lines = []
for k in sorted(ref, key=str):
    a, b = ref[k], arm[k]
    ca, cb = answers(a, True), answers(b, True)
    ra, rb = any(c for _, c in ca), any(c for _, c in cb)
    counts["recalled_ref"] += ra
    counts["recalled_arm"] += rb
    if ca != cb:
        counts["queries_changed_confident"] += 1
    if rb and not ra:
        counts["gained_correct"] += 1
    if ra and not rb:
        counts["lost_correct"] += 1
        lines.append(f"LOST {k}")
    for asset, correct in cb - ca:
        if not correct:
            counts["new_wrong_confident"] += 1
            best = max(d["hits"] for d in b["detections"] if d["asset"] == asset and d["confident"])
            lines.append(f"NEW WRONG {k} -> {asset} ({best} hits)")
    pa, pb = answers(a, False), answers(b, False)
    counts["new_wrong_possible"] += len({x for x in pb - pa if not x[1]})
    counts["lost_wrong_possible"] += len({x for x in pa - pb if not x[1]})
print(json.dumps(counts))
print("\n".join(lines))
