"""The unused evidence of 0112 against true plays (session 20, step 4's
follow-up in docs/brief-20.md): hits per second of a detection's span and
the verifier's aligned share, for every confident detection credited to a
referenced track in the development sets' verified scans, against the
wrong confident detections of the shared-material panel's main case.

usage: shared_true_plays.py <shared report.json> <outcomes.json> <out.json> <verified scan.json>...

For each measure: the largest threshold that keeps at least 95% of the
true plays, and the share of wrong plays below it. No rule is adopted.
"""
import json
import math
import sys

KEEP = 0.95


def density(detection):
    return detection["hits"] / (detection["end_seconds"] - detection["start_seconds"])


def share(detection):
    verified = detection.get("verified")
    return verified["aligned"]["share"] if verified else None


MEASURES = {"hits per second of the span": density, "verifier's aligned share": share}


def true_plays(scan_paths):
    plays = []
    for path in scan_paths:
        report = json.load(open(path, encoding="utf-8"))
        for track in report["score"]["tracks"]:
            for detection in track.get("credited", []):
                if detection.get("confident"):
                    plays.append({"set": report["set"], "track": track["label"].strip(),
                                  "asset": detection["asset"], "hits": detection["hits"],
                                  **{name: measure(detection) for name, measure in MEASURES.items()}})
    return plays


def wrong_plays(shared_path, outcomes_path):
    shared = json.load(open(shared_path, encoding="utf-8"))
    rows = json.load(open(outcomes_path, encoding="utf-8"))["rows"]
    wanted = {(r["query"], r["index"]): r for r in rows
              if r["outcome"] == "wrong confident" and r["index"] == "played left out"}
    plays = []
    for result in shared["results"]:
        row = wanted.get((result["query"], result["index"]))
        if row is None:
            continue
        detection = next(d for d in result["detections"]
                         if d["asset"] == row["wrong_file"] and d["hits"] == row["wrong_hits"])
        plays.append({"direction": row["direction"], "query": row["query"], "role": row["wrong_role"],
                      "asset": detection["asset"], "hits": detection["hits"],
                      **{name: measure(detection) for name, measure in MEASURES.items()}})
    return plays


def threshold(values):
    """The largest value that keeps at least KEEP of `values` at or above it."""
    ordered = sorted(values)
    below = len(ordered) - math.ceil(KEEP * len(ordered))
    return ordered[below]


def main(shared_path, outcomes_path, out_path, *scan_paths):
    true = true_plays(scan_paths)
    wrong = wrong_plays(shared_path, outcomes_path)
    summary = {}
    for name in MEASURES:
        values = [p[name] for p in true if p[name] is not None]
        wrong_values = [p[name] for p in wrong if p[name] is not None]
        cut = threshold(values)
        kept = sum(v >= cut for v in values)
        rejected = sum(v < cut for v in wrong_values)
        summary[name] = {
            "true plays": len(values), "threshold": cut, "true kept": kept,
            "wrong plays": len(wrong_values), "wrong below": rejected,
            "true range": [min(values), max(values)], "wrong range": [min(wrong_values), max(wrong_values)],
        }
        print(f"{name}: threshold {cut:.3f} keeps {kept} of {len(values)} true plays; "
              f"{rejected} of {len(wrong_values)} wrong plays below it "
              f"(true {min(values):.2f}-{max(values):.2f}, wrong {min(wrong_values):.2f}-{max(wrong_values):.2f})")
    by_set = {}
    for p in true:
        by_set.setdefault(p["set"], []).append(p)
    for name_of_set, plays in by_set.items():
        for name in MEASURES:
            values = sorted(p[name] for p in plays if p[name] is not None)
            print(f"  {name_of_set}: {name} {len(values)} plays, "
                  f"min {values[0]:.2f}, median {values[len(values) // 2]:.2f}")
    json.dump({"keep": KEEP, "summary": summary, "true": true, "wrong": wrong},
              open(out_path, "w", encoding="utf-8"), indent=1, ensure_ascii=False)


if __name__ == "__main__":
    main(*sys.argv[1:])
