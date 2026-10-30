"""Session 61: robust, mixes and scan reports of two front ends side by side
(experiments 0245-0247). Each argument after the output file is
`label=reports-dir`; a directory holds `robust-default.json` and
`robust-stereo.json` (renamed after each run), `mixes-stereo-*-seed-*.json`
and `scan-*.json`, any of them optional. Writes one JSON object keyed by
label, with file names only."""
import glob, json, os, sys
from collections import defaultdict


def robust(path):
    if not os.path.exists(path):
        return None
    report = json.load(open(path))
    rows = defaultdict(lambda: defaultdict(int))
    wrong = []
    for q in report["queries"]:
        row = rows[q["condition"]]
        if q["held_out"]:
            row["held_out_wrong_confident"] += q["wrong_confident"] > 0
            row["held_out_wrong_possible"] += q["wrong_possible"] > 0
        else:
            row["indexed"] += 1
            row["confident"] += q["confident"]
            row["possible_or_better"] += q["possible"] or q["confident"]
            row["wrong_confident"] += q["wrong_confident"] > 0
            row["wrong_possible"] += q["wrong_possible"] > 0
        if q["wrong_confident"]:
            names = [w["asset"] for w in (q.get("stereo") or {}).get("wrong", []) if w.get("level") == "confident"]
            wrong.append([q["condition"], q["speed"], q["number"], q["held_out"], names])
    hits = {(q["condition"], q["speed"], q["number"]): q["best_hits"] for q in report["queries"]}
    return {"rows": {c: dict(r) for c, r in rows.items()}, "wrong_confident": wrong, "hits": hits}


def mixes(directory):
    out = {}
    for path in sorted(glob.glob(os.path.join(directory, "mixes-stereo-*-seed-*.json"))):
        condition = os.path.basename(path)[len("mixes-"):].rsplit("-seed-", 1)[0]
        where = out.setdefault(condition, {"first": [0, 0], "across": [0, 0], "second": [0, 0], "wrong_confident": 0, "mixes": 0})
        for m in json.load(open(path))["mixes"]:
            where["mixes"] += 1
            half = m["mix"]["seconds"] / 2
            wrong = m["score"]["wrong_confident"]
            where["wrong_confident"] += wrong if isinstance(wrong, int) else len(wrong)
            for play in m["score"]["plays"]:
                t = play["truth"]
                if t["held_out"]:
                    continue
                part = "first" if t["end_seconds"] <= half else "second" if t["start_seconds"] >= half else "across"
                where[part][1] += 1
                where[part][0] += play["level"] == "confident"
    return out


def scans(directory):
    out = {}
    for path in sorted(glob.glob(os.path.join(directory, "scan-*.json"))):
        r = json.load(open(path))
        if r["left_out_tracks"]:
            continue
        s = r["score"]
        confident = [d for d in r["detections"] if d.get("confident", d.get("level") == "confident")]
        out[r["set"]] = {k: s[k] for k in ("referenced", "identified", "wrong", "possible")} | {
            "wall_seconds": r["wall_seconds"],
            "detections": len(r["detections"]),
        }
    return out


result = {}
for argument in sys.argv[2:]:
    label, directory = argument.split("=", 1)
    entry = {"mixes": mixes(directory), "scans": scans(directory)}
    for kind in ("default", "stereo"):
        entry[f"robust_{kind}"] = robust(os.path.join(directory, f"robust-{kind}.json"))
    result[label] = entry

labels = list(result)
for kind in ("robust_default", "robust_stereo"):
    present = [l for l in labels if result[l][kind]]
    if not present:
        continue
    print(kind, "confident / possible+ / wrong confident (indexed); held-out wrong confident:", " | ".join(present))
    conditions = list(result[present[0]][kind]["rows"])
    for c in conditions:
        cells = []
        for l in present:
            r = result[l][kind]["rows"].get(c, {})
            cells.append(f"{r.get('confident', 0)}/{r.get('possible_or_better', 0)}/{r.get('wrong_confident', 0)} h{r.get('held_out_wrong_confident', 0)}")
        print(f"  {c:<24}", " | ".join(cells))
for l in labels:
    for c, w in result[l]["mixes"].items():
        print("mixes", l, c, {k: f"{v[0]}/{v[1]}" for k, v in w.items() if isinstance(v, list)}, "wrong", w["wrong_confident"], "mixes", w["mixes"])
for l in labels:
    for s, v in result[l]["scans"].items():
        print("scan", l, s, v)
for entry in result.values():
    for kind in ("robust_default", "robust_stereo"):
        if entry[kind]:
            entry[kind]["hits"] = {"|".join(map(str, k)): v for k, v in entry[kind]["hits"].items()}
json.dump(result, open(sys.argv[1], "w"), indent=1)
