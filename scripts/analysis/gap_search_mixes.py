"""Session 84: generated mixes played at the edge or key-locked, per arm.

gap_search_mixes.py edge|key-locked REF=report.json ARM=report.json ...
The first arm is the reference for new wrong answers.
Plays: indexed (not held out). Changed plays: |speed - 1| >= 0.08 (edge) or key-locked.
"""
import json
import sys

BINS = [(8.0, 8.4), (8.4, 8.8), (8.8, 9.2), (9.2, 9.6), (9.6, 10.01)]


def load(path):
    with open(path) as f:
        return json.load(f)


def summary(report, mode):
    rows = {"changed": {}, "other": [0, 0, 0], "bins": {b: [0, 0, 0] for b in BINS},
            "wrong": set(), "wrong_possible": 0, "held_out": 0}
    levels = {"confident": 0, "possible": 1, "weak": 2, "none": 2}
    for mix in report["mixes"]:
        name = mix["mix"]["name"]
        for play in mix["score"]["plays"]:
            t = play["truth"]
            if t["held_out"]:
                rows["held_out"] += 1
                continue
            level = levels[play["level"]]
            away = abs(t["speed"] - 1.0) * 100
            changed = t.get("key_locked", False) if mode == "key-locked" else away >= 8.0 - 1e-9
            if changed:
                rows["changed"].setdefault("all", [0, 0, 0])[level] += 1
                if mode == "edge":
                    for b in BINS:
                        if b[0] <= away + 1e-9 < b[1]:
                            rows["bins"][b][level] += 1
                else:
                    key = "<=1%" if away <= 1.0 else ">1%"
                    rows["changed"].setdefault(key, [0, 0, 0])[level] += 1
            else:
                rows["other"][level] += 1
        for w in mix["score"]["wrong_confident"]:
            rows["wrong"].add((name, w["asset"], round(w["start_seconds"])))
        rows["wrong_possible"] += len(mix["score"]["wrong_possible"])
    return rows


def main():
    mode, *arms = sys.argv[1:]
    summaries = []
    for arm in arms:
        label, path = arm.split("=", 1)
        summaries.append((label, summary(load(path), mode)))
    ref_wrong = {(m, a) for m, a, _ in summaries[0][1]["wrong"]}
    out = {}
    for label, s in summaries:
        new = sorted({(m, a) for m, a, _ in s["wrong"]} - ref_wrong)
        out[label] = {
            "changed (confident, possible, weak or none)": s["changed"],
            "edge bins": {f"{b[0]}-{min(b[1], 10.0)}": v for b, v in s["bins"].items()} if mode == "edge" else None,
            "other plays (confident, possible, weak or none)": s["other"],
            "held-out plays": s["held_out"],
            "wrong confident": sorted(s["wrong"]),
            "new wrong confident against the first arm": new,
            "wrong possible": s["wrong_possible"],
        }
    print(json.dumps(out, indent=1, ensure_ascii=False))


if __name__ == "__main__":
    main()
