"""Session 65 (brief 65): summarise `mixes --kind drops --residual` reports.

For every record B inside a record A that the search found confidently
(the second record of a quiet, brief, unrelated or pair block that the first
plays on after), the level each arm gives B in A's excerpt; the wrong
answers of every arm over all cases; what the mask leaves of A.

Usage: python3 -I scripts/analysis/residual_pilot.py REPORT... [--json OUT]
"""
import json
import sys
from collections import Counter, defaultdict

LEVELS = ["none", "weak", "possible", "confident"]
ARMS = ["plain", "denser", "masked", "masked-without-partners"]


def at_least(level, floor):
    return LEVELS.index(level) >= LEVELS.index(floor)


def inside_blocks(plays):
    """(A index, B index, scenario) for each B inside the A it joins."""
    for index, a in enumerate(plays[:-1]):
        b = plays[index + 1]
        if (
            a.get("side") == "first"
            and b.get("side") == "second"
            and a.get("scenario") == b.get("scenario")
            and b["end_seconds"] <= a["end_seconds"]
        ):
            yield index, index + 1, a["scenario"]


def summarise(paths):
    rows = []  # one per B inside its A
    wrong = {arm: Counter() for arm in ARMS}
    wrong_seen = {arm: set() for arm in ARMS}
    residue = Counter()
    cases_by_kind = Counter()
    for path in paths:
        report = json.load(open(path))
        seed = report["seed"]
        mixes = {result["mix"]["name"]: result for result in report["mixes"]}
        for case in report["residual"]["cases"]:
            result = mixes[case["mix"]]
            plays = result["mix"]["plays"]
            cases_by_kind[case["reference_kind"]] += 1
            arms = {arm["name"]: arm for arm in case["arms"]}
            for name, arm in arms.items():
                for level, key in [("confident", "wrong_confident"), ("possible", "wrong_possible")]:
                    for found in arm[key]:
                        seen = (seed, case["mix"], found["asset"], round(found["start_seconds"]))
                        if seen in wrong_seen[name] and level == "confident":
                            continue
                        if level == "confident":
                            wrong_seen[name].add(seen)
                        wrong[name][level] += 1
            masked_a = next(p for p in arms["masked"]["plays"] if p["play"] == case["play"])
            residue[masked_a["level"]] += 1
            for a, b, scenario in inside_blocks(plays):
                if a != case["play"]:
                    continue
                whole = result["score"]["plays"][b]
                row = {
                    "seed": seed,
                    "mix": case["mix"],
                    "scenario": scenario,
                    "a": plays[a]["asset"],
                    "b": plays[b]["asset"],
                    "b_seconds": round(plays[b]["end_seconds"] - plays[b]["start_seconds"], 1),
                    "reference_kind": case["reference_kind"],
                    "whole_mix": whole["level"],
                }
                for name, arm in arms.items():
                    found = next((p for p in arm["plays"] if p["play"] == b), None)
                    row[name] = found["level"] if found else "none"
                    row[name + "_hits"] = (found or {}).get("best") and found["best"]["hits"]
                rows.append(row)
    return rows, wrong, residue, cases_by_kind


def table(rows, scenarios):
    print(f"{'B inside A':<22} {'n':>3}  " + "  ".join(f"{arm:>24}" for arm in ARMS))
    print(f"{'':<22} {'':>3}  " + "  ".join(f"{'cand/conf/confident':>24}" for _ in ARMS))
    for scenario in scenarios:
        chosen = [row for row in rows if row["scenario"] == scenario]
        if not chosen:
            continue
        cells = []
        for arm in ARMS:
            levels = [row.get(arm) for row in chosen if row.get(arm) is not None]
            if not levels:
                cells.append(f"{'-':>24}")
                continue
            cand = sum(at_least(level, "weak") for level in levels)
            conf = sum(at_least(level, "possible") for level in levels)
            sure = sum(level == "confident" for level in levels)
            cells.append(f"{f'{cand}/{conf}/{sure} of {len(levels)}':>24}")
        print(f"{scenario:<22} {len(chosen):>3}  " + "  ".join(cells))


def main():
    args = sys.argv[1:]
    out = None
    if "--json" in args:
        at = args.index("--json")
        out = args[at + 1]
        args = args[:at] + args[at + 2 :]
    rows, wrong, residue, kinds = summarise(args)
    scenarios = ["quiet", "brief", "unrelated-drop", "pair-drop"]
    print("cases by reference:", dict(kinds))
    table(rows, scenarios)
    for kind in ["played", "other-rip"]:
        chosen = [row for row in rows if row["reference_kind"] == kind]
        print(f"\nreference {kind}:")
        table(chosen, scenarios)
    print("\nwrong answers over all cases (confident deduplicated by mix, file, second):")
    for arm in ARMS:
        print(f"  {arm:<24} confident {wrong[arm]['confident']:>3}  possible {wrong[arm]['possible']:>3}")
    print("what the mask leaves of A (A's level in the masked arm):", dict(residue))
    newly = [
        row
        for row in rows
        if row["scenario"] in ("quiet", "brief")
        and at_least(row["masked"], "possible")
        and not at_least(row["plain"], "possible")
    ]
    with_evidence = sum(at_least(row["plain"], "weak") for row in newly)
    print(
        f"quiet and brief B confirmed in masked, not in plain: {len(newly)};"
        f" with a plain detection (two windows or more): {with_evidence}"
    )
    if out:
        json.dump(
            {"rows": rows, "wrong": {arm: dict(wrong[arm]) for arm in ARMS}, "residue": dict(residue), "cases": dict(kinds)},
            open(out, "w"),
            indent=1,
        )


main()
