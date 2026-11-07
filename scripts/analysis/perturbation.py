"""Session 81: a sweep's or grid's report under a perturbed index, against its base.

Usage:
  perturbation.py compare sweep|grid BASE.json PERTURBED.json
      the registered measures (docs/brief-81.md): survival, fitted-speed
      movement, hit movement and confident identity changes, by query
      length and by the base detection's distance to the rule
  perturbation.py fits sweep|grid DEFAULT.json VARIANT.json
      confident answers gained and lost, and wrong confident detections
      added, by the variant against the default on the same index

Only the corpus's files are followed from one report to the other: the
other library's files (`second-library/`) and copies (`padding-copy/`) may
be in one index only, so their detections are counted apart.
"""
import json
import statistics
import sys
from collections import defaultdict

RULE = 240
FLOOR = 10
SPEED_MOVE = 1e-4  # percentage points
HIT_MOVE = 0.05


def load(path):
    with open(path) as f:
        return json.load(f)


def followed(asset):
    return not asset.startswith(("second-library/", "padding-copy/"))


def hits_band(hits):
    if hits < 120:
        return "<120"
    if hits < RULE:
        return "120-239"
    if hits < 2 * RULE:
        return "240-479"
    return ">=480"


def windows_band(windows):
    return "<3 windows" if windows < 3 else ">=3 windows"


def length_band(seconds):
    return "10-20 s" if seconds <= 20 else "25-40 s"


class Report:
    """One report's queries as (key, length, detections, right, wrong)."""

    def __init__(self, kind, path):
        self.kind = kind
        self.data = load(path)
        self.queries = {}
        for query in self.data["queries"]:
            if kind == "sweep":
                key = (query["asset"], query["speed_percent"], query["start_seconds"])
                detections = [
                    {**d, "speed": d["speed_percent"]} for d in query["detections"]
                ]
                right = any(
                    d["confident"] and (d.get("correct") or d.get("same_title"))
                    for d in query["detections"]
                )
                wrong = sorted(
                    d["asset"]
                    for d in query["detections"]
                    if d["confident"] and not (d.get("correct") or d.get("same_title"))
                )
                length = 30.0
            else:
                key = (query["brief"], query["seconds"], query["offset_seconds"])
                detections = [{**d, "speed": 100.0 * d["speed"]} for d in query["matching"]]
                right = query["level"] == "confident"
                wrong = query["wrong_confident"]
                length = query["seconds"]
            self.queries[key] = (length, detections, right, wrong)


def strongest(detections):
    out = {}
    for d in detections:
        if d["hits"] > out.get(d["asset"], {"hits": -1})["hits"]:
            out[d["asset"]] = d
    return out


def summary(moves):
    speeds = [abs(m["speed"]) for m in moves]
    hits = [m["hits"] for m in moves]
    return {
        "n": len(moves),
        "speed_moved": sum(s > SPEED_MOVE for s in speeds),
        "speed_median": statistics.median(speeds) if speeds else None,
        "speed_largest": max(speeds) if speeds else None,
        "hits_moved": sum(h != 0 for h in hits),
        "hits_moved_5pct": sum(abs(m["ratio"] - 1) >= HIT_MOVE for m in moves),
        "hits_largest_change": max(hits, key=abs) if hits else None,
    }


def compare(kind, base_path, perturbed_path):
    base = Report(kind, base_path)
    perturbed = Report(kind, perturbed_path)
    if base.queries.keys() != perturbed.queries.keys() or base.data.get("briefs") != perturbed.data.get("briefs"):
        sys.exit(f"the reports hold different queries: {base_path}, {perturbed_path}")
    survival = defaultdict(lambda: [0, 0])
    moves = defaultdict(list)
    apart = {"base": 0, "perturbed": 0}
    identity = defaultdict(lambda: defaultdict(int))
    changed = []
    for key, (length, before, right_before, wrong_before) in base.queries.items():
        _, after, right_after, wrong_after = perturbed.queries[key]
        apart["base"] += sum(not followed(d["asset"]) for d in before)
        apart["perturbed"] += sum(not followed(d["asset"]) for d in after)
        old = {a: d for a, d in strongest(before).items() if followed(a) and d["hits"] >= FLOOR}
        new = strongest(after)
        band = length_band(length)
        for asset, d in old.items():
            strata = [
                ("all", "all"),
                ("length", band),
                ("hits", hits_band(d["hits"])),
                ("windows", windows_band(d["windows"])),
                ("length x hits", f"{band}, {hits_band(d['hits'])}"),
            ]
            for stratum in strata:
                survival[stratum][1] += 1
            if asset not in new:
                continue
            for stratum in strata:
                survival[stratum][0] += 1
            n = new[asset]
            move = {
                "speed": n["speed"] - d["speed"],
                "hits": n["hits"] - d["hits"],
                "ratio": n["hits"] / d["hits"],
            }
            for stratum in strata:
                moves[stratum].append(move)
        group = identity[band]
        group["queries"] += 1
        if right_before and not right_after:
            group["right_lost"] += 1
        if right_after and not right_before:
            group["right_gained"] += 1
        if kind == "sweep":
            group["wrong_added"] += len(set(wrong_after) - set(wrong_before))
            group["wrong_removed"] += len(set(wrong_before) - set(wrong_after))
        else:
            group["wrong_added"] += max(0, wrong_after - wrong_before)
            group["wrong_removed"] += max(0, wrong_before - wrong_after)
        if right_before != right_after or wrong_before != wrong_after:
            changed.append({"query": list(key), "right": [right_before, right_after],
                            "wrong": [wrong_before, wrong_after]})
    return {
        "base": base_path,
        "perturbed": perturbed_path,
        "indexed_assets": [base.data["indexed_assets"], perturbed.data["indexed_assets"]],
        "detections_of_files_apart": apart,
        "survival": {f"{s[0]}: {s[1]}": {"survived": v[0], "of": v[1]} for s, v in sorted(survival.items())},
        "moves": {f"{s[0]}: {s[1]}": summary(v) for s, v in sorted(moves.items())},
        "identity": {band: dict(v) for band, v in sorted(identity.items())},
        "identity_changes": sum(
            v["right_lost"] + v["right_gained"] + v["wrong_added"] + v["wrong_removed"]
            for v in identity.values()
        ),
        "changed_queries": changed,
    }


def fits(kind, default_path, variant_path):
    default = Report(kind, default_path)
    variant = Report(kind, variant_path)
    if default.queries.keys() != variant.queries.keys() or default.data.get("briefs") != variant.data.get("briefs"):
        sys.exit("the reports hold different queries")
    out = defaultdict(lambda: defaultdict(int))
    for key, (length, before, right_before, wrong_before) in default.queries.items():
        _, after, right_after, wrong_after = variant.queries[key]
        group = out[length_band(length)]
        group["right_gained"] += right_after and not right_before
        group["right_lost"] += right_before and not right_after
        if kind == "sweep":
            group["wrong_added"] += len(set(wrong_after) - set(wrong_before))
        else:
            group["wrong_added"] += max(0, wrong_after - wrong_before)
        old, new = strongest(before), strongest(after)
        for asset in old.keys() & new.keys():
            if new[asset]["hits"] != old[asset]["hits"]:
                group["detections_with_other_hits"] += 1
    return {band: dict(v) for band, v in sorted(out.items())}


if __name__ == "__main__":
    command, kind, a, b = sys.argv[1:5]
    result = compare(kind, a, b) if command == "compare" else fits(kind, a, b)
    json.dump(result, sys.stdout, indent=1)
    print()
