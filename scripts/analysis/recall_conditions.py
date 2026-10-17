"""Summary of `gunfinger-eval recall` reports under the hardest conditions
(session 28, experiments 0143 and up): per report, confident and possible
recall of the observable indexed queries, wrong answers (confident and
possible, with their files), held-out and unobservable queries with a
confident answer, and every miss's reason; by speed, kind, brief length
and where a brief play starts on the 10 s window grid.

A detection is possible or better at 60 hits (both passes' possible
tier); a partner's detection (damaged queries) is neither right nor
wrong. Files are named without their folders (AGENTS.md: evaluation JSON
in docs holds file names only).

usage: recall_conditions.py [--json OUT] REPORT.json...
"""
import json
import sys
from collections import Counter, defaultdict

POSSIBLE_HITS = 60

args = sys.argv[1:]
out = None
if args and args[0] == "--json":
    out = args[1]
    args = args[2:]


def file_name(path):
    return path.rsplit("/", 1)[-1]


def own(query):
    return [d for d in query["detections"] if d["correct"]]


def wrong(query):
    return [d for d in query["detections"] if not d["correct"] and not d.get("partner")]


def at_least_possible(detection):
    return detection["confident"] or detection["hits"] >= POSSIBLE_HITS


def reason(query):
    """The miss's reason, with too few hits split at the possible tier."""
    miss = query.get("miss")
    if miss != "too-few-hits":
        return miss
    best = max(own(query), key=lambda d: d["hits"])
    return "too-few-hits, possible" if best["hits"] >= POSSIBLE_HITS else "too-few-hits, weak"


def tally(queries):
    indexed = [q for q in queries if not q["held_out"] and not q.get("unobservable")]
    confident = sum(any(d["confident"] for d in own(q)) for q in indexed)
    possible = sum(any(at_least_possible(d) for d in own(q)) for q in indexed)
    return {
        "indexed": len(indexed),
        "confident": confident,
        "possible_or_better": possible,
        "misses": dict(Counter(reason(q) for q in indexed if q.get("miss"))),
    }


summaries = {}
for path in args:
    if "ed-rush" in path:
        raise SystemExit(f"refused: {path} names the held-out test set")
    report = json.load(open(path))
    queries = report["queries"]
    name = path.rsplit("/", 1)[-1]
    held_out = [q for q in queries if q["held_out"]]
    unobservable = [q for q in queries if q.get("unobservable")]
    wrong_confident = []
    wrong_possible = []
    for q in queries:
        for d in wrong(q):
            entry = {
                "query": file_name(q["asset"]),
                "speed_percent": q["speed_percent"],
                "held_out": q["held_out"],
                "unobservable": bool(q.get("unobservable")),
                "found": file_name(d["asset"]),
                "hits": d["hits"],
                "windows": d["windows"],
            }
            if "brief" in q:
                entry["brief_seconds"] = q["brief"]["seconds"]
            if d["confident"]:
                wrong_confident.append(entry)
            elif d["hits"] >= POSSIBLE_HITS:
                wrong_possible.append(entry)
    by_speed = defaultdict(list)
    by_kind = defaultdict(list)
    by_length = defaultdict(list)
    by_phase = defaultdict(list)
    for q in queries:
        by_speed[q["speed_percent"]].append(q)
        by_kind[q["kind"]].append(q)
        if "brief" in q:
            length = q["brief"]["seconds"]
            by_length[length].append(q)
            phase = int(q["brief"]["starts_at_seconds"] % 10)
            by_phase[f"{length:.0f} s from {phase}-{phase + 1} s"].append(q)
    summary = {
        "report": name,
        "other_rips_only": report.get("other_rips_only", False),
        "query_set": report.get("query_set", "clean"),
        "indexed_assets": report["indexed_assets"],
        "left_out_records": report.get("left_out_records", 0),
        "all": tally(queries),
        "unobservable_queries": len(unobservable),
        "unobservable_sources": len({q["asset"] for q in unobservable}),
        "unobservable_confident": sum(any(d["confident"] for d in wrong(q)) for q in unobservable),
        "held_out_queries": len(held_out),
        "held_out_confident": sum(any(d["confident"] for d in wrong(q)) for q in held_out),
        "wrong_confident": wrong_confident,
        "wrong_possible_count": len(wrong_possible),
        "wrong_possible_over_200": [w for w in wrong_possible if w["hits"] >= 200],
        "strongest_wrong_possible": sorted(wrong_possible, key=lambda w: -w["hits"])[:10],
        "by_speed": {f"{speed:+.0f}": tally(qs) for speed, qs in sorted(by_speed.items())},
        "by_kind": {kind: tally(qs) for kind, qs in sorted(by_kind.items())},
        "by_length": {f"{length:.0f}": tally(qs) for length, qs in sorted(by_length.items())},
        "by_grid_phase": {key: tally(qs) for key, qs in sorted(by_phase.items())},
    }
    summaries[name] = summary
    a = summary["all"]
    print(
        f"{name}: indexed {a['indexed']}, confident {a['confident']} "
        f"({100 * a['confident'] / max(a['indexed'], 1):.1f}%), possible or better "
        f"{a['possible_or_better']} ({100 * a['possible_or_better'] / max(a['indexed'], 1):.1f}%); "
        f"wrong confident {len(wrong_confident)}, wrong possible {len(wrong_possible)}; "
        f"held out confident {summary['held_out_confident']} of {len(held_out)}; "
        f"unobservable {len(unobservable)} ({summary['unobservable_sources']} sources), "
        f"confident {summary['unobservable_confident']}; left out {summary['left_out_records']}"
    )
    print(f"  misses: {a['misses']}")
    for length, t in summary["by_length"].items():
        print(f"  {length} s: {t['confident']}/{t['indexed']} confident, {t['possible_or_better']} possible+, misses {t['misses']}")
    for kind, t in summary["by_kind"].items():
        print(f"  {kind}: {t['confident']}/{t['indexed']} confident, {t['possible_or_better']} possible+")
    print("  by speed: " + ", ".join(f"{s} {t['confident']}/{t['indexed']}" for s, t in summary["by_speed"].items()))
    for w in wrong_confident:
        print(f"  WRONG confident: {w}")
    for w in summary["wrong_possible_over_200"]:
        print(f"  wrong possible >= 200: {w}")

if out:
    with open(out, "w") as file:
        json.dump(summaries, file, indent=1)
