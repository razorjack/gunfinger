"""Scores `gunfinger-eval break-search` reports against the cases file (brief
74's pre-registration).

    python3 -I scripts/analysis/break_search.py CASES.json REPORT.json [OUT.json]

Per mode (`all`, `filtered`) and group: positives found (an expected file the
index holds, listed), the rank of the strongest expected file among the other
files (own, own extra and expected files are not counted above it), the noise
of random passages and intros, the share of passage hashes set aside, the
share of the index listed, reach and cost. Prints a summary and writes it as
JSON with OUT."""

import json
import math
import statistics
import sys

GROUPS = ("A", "B", "C", "D")
NEGATIVE = ("N1", "N2")
REACH_SECONDS = 60.0


def percentile(values, share):
    """The value at `share` of the sorted values (nearest rank)."""
    if not values:
        return None
    ordered = sorted(values)
    rank = max(1, math.ceil(share * len(ordered)))
    return ordered[min(rank, len(ordered)) - 1]


def median(values):
    return statistics.median(values) if values else None


def score_positive(case, searched):
    """Found, rank and the other files above the best expected file, or
    None when the index holds none of the expected files."""
    missing = set(searched["expected_not_indexed"])
    expected = set(case["expected"]) - missing
    if not expected:
        return None
    apart = expected | set(case["own_extra"])
    above = 0
    for holder in searched["others"]:
        if holder["file"] in expected:
            return {"found": True, "rank": above + 1, "above": above, "hits": holder["hits"],
                    "expected_listed": sum(h["file"] in expected for h in searched["others"]),
                    "expected": len(expected)}
        if holder["file"] not in apart:
            above += 1
    return {"found": False, "rank": None, "above": above, "hits": None,
            "expected_listed": 0, "expected": len(expected)}


def strongest_other(case, searched):
    apart = set(case["own_extra"]) | set(case["expected"])
    for holder in searched["others"]:
        if holder["file"] not in apart:
            return holder["hits"]
    return 0


def summarise(cases, report):
    by_name = {case["name"]: case for case in cases}
    index_files = report["index_files"]
    summary = {"index_files": index_files, "index_seconds": report["index_seconds"], "modes": {}}
    for mode in ("all", "filtered"):
        rows = [(by_name[s["name"]], s) for s in report["searches"] if s["lists"] == mode]
        errors = [s["name"] for _, s in rows if s.get("error")]
        rows = [(c, s) for c, s in rows if not s.get("error")]
        out = {"errors": errors, "groups": {}}
        n1_top = [strongest_other(c, s) for c, s in rows if c["group"] == "N1"]
        threshold = percentile(n1_top, 0.95)
        out["null_95th_hits"] = threshold
        found_hits = []
        for group in GROUPS + NEGATIVE:
            members = [(c, s) for c, s in rows if c["group"] == group]
            if not members:
                continue
            g = {"queries": len(members)}
            listed = lambda s, at: dict((a, n) for a, n in s["listed_at"])[at]
            others_at = lambda s, at: sum(h["hits"] >= at for h in s["others"])
            g["others_20_median"] = median([others_at(s, 20) for _, s in members])
            g["others_60_median"] = median([others_at(s, 60) for _, s in members])
            g["others_20_max"] = max(others_at(s, 20) for _, s in members)
            g["share_of_index_20_max"] = max(listed(s, 20) for _, s in members) / index_files
            g["share_of_index_60_max"] = max(listed(s, 60) for _, s in members) / index_files
            g["set_aside_share_median"] = median(
                [s["set_aside_hashes"] / s["held_hashes"] for _, s in members if s["held_hashes"]])
            g["search_seconds_median"] = median([s["search_seconds"] for _, s in members])
            g["search_seconds_max"] = max(s["search_seconds"] for _, s in members)
            g["reach_seconds_median"] = median([s["reach_seconds"] for _, s in members])
            reached = [h for _, s in members for h in s["others"] if h.get("reach")]
            g["others_reached"] = len(reached)
            g["others_reaching_60s"] = sum(
                h["reach"]["query_end_seconds"] - h["reach"]["query_start_seconds"] >= REACH_SECONDS
                for h in reached)
            if group in NEGATIVE:
                tops = [strongest_other(c, s) for c, s in members]
                g["strongest_other_hits_median"] = median(tops)
                g["strongest_other_hits_max"] = max(tops)
                g["with_another_file_at_20"] = sum(t >= 20 for t in tops)
            else:
                scored = [(c["name"], score_positive(c, s)) for c, s in members]
                none = [n for n, x in scored if x is None]
                scored = [(n, x) for n, x in scored if x is not None]
                g["no_expected_indexed"] = none
                g["scored"] = len(scored)
                g["found"] = sum(x["found"] for _, x in scored)
                g["first"] = sum(x["rank"] == 1 for _, x in scored)
                g["top5"] = sum(x["found"] and x["rank"] <= 5 for _, x in scored)
                g["above_median"] = median([x["above"] for _, x in scored if x["found"]])
                g["missed"] = [n for n, x in scored if not x["found"]]
                g["ranks"] = {n: x["rank"] for n, x in scored}
                g["at_or_above_null"] = sum(
                    x["found"] and threshold is not None and x["hits"] >= threshold for _, x in scored)
                found_hits += [x["hits"] for _, x in scored if x["found"]]
            out["groups"][group] = g
        pooled = [g for k, g in out["groups"].items() if k in GROUPS]
        scored = sum(g["scored"] for g in pooled)
        out["positives"] = {
            "scored": scored,
            "found": sum(g["found"] for g in pooled),
            "first": sum(g["first"] for g in pooled),
            "top5": sum(g["top5"] for g in pooled),
            "found_hits_median": median(found_hits),
        }
        summary["modes"][mode] = out
    return summary


def main(argv):
    cases = json.load(open(argv[1]))
    report = json.load(open(argv[2]))
    summary = summarise(cases, report)
    for mode, out in summary["modes"].items():
        p = out["positives"]
        print(f"{mode}: positives {p['found']}/{p['scored']} found, {p['first']} first, "
              f"{p['top5']} in the first 5; expected hits median {p['found_hits_median']}; "
              f"N1 95th {out['null_95th_hits']}; errors {len(out['errors'])}")
        for group, g in out["groups"].items():
            line = {k: v for k, v in g.items() if k not in ("ranks", "missed", "no_expected_indexed")}
            print(f"  {group}: {json.dumps(line)}")
    if len(argv) > 3:
        json.dump(summary, open(argv[3], "w"), indent=1)


if __name__ == "__main__":
    main(sys.argv)
