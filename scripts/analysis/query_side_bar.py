"""Session 62: the query side's bar (brief 62's pre-registration), query by
query, downmix against power-sum queries on the same rendered queries.
Usage: query_side_bar.py OUT.json DOWNMIX_DIR VARIANT_DIR. Each directory
holds the run's reports: `shared-material*-development.json`,
`recall-seed-2026*.json`, `mixes-stereo-*-seed-*.json`, `robust-stereo.json`
and `scan-*.json`, any of them optional. For each run: right confident
answers under each front end and those lost and won, by query; confident
(query, file) pairs of wrong files new under the variant; for `shared`,
confident detections by role and the related versions named. Session 40's
known pair (the Mobb Deep / Urban Takover upload) is counted on its own
line. File names only."""
import glob, json, os, sys
from collections import Counter

KNOWN = "Mobb Deep Vs Urban Takover - Cool Rock Shook Ones"
STEREO_KINDS = {"alone", "beyond", "before"}


def load(directory, name):
    path = os.path.join(directory, name)
    return json.load(open(path)) if os.path.exists(path) else None


def compare(right_d, right_v, wrong_d, wrong_v):
    """Sets of right query keys and of (query key, file) wrong pairs."""
    new_wrong = sorted(wrong_v - wrong_d)
    return {
        "right": [len(right_d), len(right_v)],
        "lost": sorted(map(str, right_d - right_v)),
        "won": len(right_v - right_d),
        "wrong": [len(wrong_d), len(wrong_v)],
        "new_wrong": [[str(k), f] for k, f in new_wrong if KNOWN not in f],
        "known_pair_new": [[str(k), f] for k, f in new_wrong if KNOWN in f],
        "known_pair": [sum(KNOWN in f for _, f in wrong_d), sum(KNOWN in f for _, f in wrong_v)],
    }


def shared(report):
    right, other, related, roles = set(), set(), set(), Counter()
    for r in report["results"]:
        key = (r["scenario"], r["query"], r["index"])
        mono = r["kind"] not in STEREO_KINDS
        for d in r["detections"]:
            if d["level"] != "confident":
                continue
            roles[(r["index"], d["role"], "mono" if mono else "stereo")] += 1
            if d["role"] == "played" and r["index"] != "played left out":
                right.add(key)
            elif d["role"] == "other":
                other.add((key, d["asset"]))
            elif d["role"] == "related":
                related.add((key, d["asset"]))
    return right, other, related, roles


def recall(report):
    right, wrong, tag = set(), set(), report["query_set"] + ("-other-rips" if report["other_rips_only"] else "")
    for n, q in enumerate(report["queries"]):
        key = (tag, n, q["asset"], q["speed_percent"])
        dets = q["detections"]
        if not q["held_out"] and not q.get("unobservable") and any(
            d["confident"] and (d["correct"] or d.get("same_title")) for d in dets):
            right.add(key)
        for d in dets:
            if d["confident"] and not d["correct"] and not d.get("same_title") and not d.get("partner"):
                wrong.add((key, d["asset"]))
    return right, wrong


def mixes(report, tag):
    right, wrong = set(), set()
    for m in report["mixes"]:
        half = m["mix"]["seconds"] / 2
        for n, play in enumerate(m["score"]["plays"]):
            t = play["truth"]
            if t["held_out"] or play["level"] != "confident":
                continue
            part = "first" if t["end_seconds"] <= half else "second" if t["start_seconds"] >= half else "across"
            right.add((tag, m["mix"]["name"], n, part))
        for w in m["score"]["wrong_confident"]:
            wrong.add(((tag, m["mix"]["name"]), w["asset"]))
    return right, wrong


def robust(report):
    right, wrong = set(), set()
    for q in report["queries"]:
        key = (q["condition"], q["speed"], q["number"])
        if not q["held_out"] and q["confident"]:
            right.add(key)
        if q["wrong_confident"]:
            names = [w["asset"] for w in (q.get("stereo") or {}).get("wrong", []) if w.get("level") == "confident"]
            for name in names or ["(mono query: no name kept)"]:
                wrong.add((key + (q["held_out"],), name))
    return right, wrong


def scan(report):
    s = report["score"]
    right = {t["label"] for t in s["tracks"] if t["identified"]}
    wrong = {(w.get("label", ""), w["asset"]) for w in s["wrong_identifications"]}
    return right, wrong


out_path, d_dir, v_dir = sys.argv[1:4]
out = {}
for path in sorted(glob.glob(os.path.join(d_dir, "shared-material*-development.json"))):
    name = os.path.basename(path)
    v = load(v_dir, name)
    if not v:
        continue
    rd, od, ld, cd = shared(json.load(open(path)))
    rv, ov, lv, cv = shared(v)
    entry = compare(rd, rv, od, ov)
    entry["related_pairs"] = [len(ld), len(lv)]
    entry["related_new"] = len(lv - ld)
    entry["related_gone"] = len(ld - lv)
    entry["roles"] = {"|".join(k): [cd.get(k, 0), cv.get(k, 0)] for k in sorted(set(cd) | set(cv))}
    out[name] = entry
for path in sorted(glob.glob(os.path.join(d_dir, "recall-seed-2026*.json"))):
    name = os.path.basename(path)
    v = load(v_dir, name)
    if v:
        rd, wd = recall(json.load(open(path)))
        rv, wv = recall(v)
        out[name] = compare(rd, rv, wd, wv)
for path in sorted(glob.glob(os.path.join(d_dir, "mixes-stereo-*-seed-*.json"))):
    name = os.path.basename(path)
    v = load(v_dir, name)
    if v:
        rd, wd = mixes(json.load(open(path)), name)
        rv, wv = mixes(v, name)
        entry = compare(rd, rv, wd, wv)
        entry["by_part"] = {p: [sum(k[3] == p for k in rd), sum(k[3] == p for k in rv)] for p in ("first", "across", "second")}
        out[name] = entry
for name in ["robust-stereo.json"] + sorted(os.path.basename(p) for p in glob.glob(os.path.join(d_dir, "scan-*.json"))):
    d, v = load(d_dir, name), load(v_dir, name)
    if d and v:
        f = robust if name.startswith("robust") else scan
        rd, wd = f(d)
        rv, wv = f(v)
        entry = compare(rd, rv, wd, wv)
        if f is robust:
            entry["by_condition"] = {c: [sum(k[0] == c for k in rd), sum(k[0] == c for k in rv)] for c in sorted({k[0] for k in rd | rv})}
        out[name] = entry
json.dump(out, open(out_path, "w"), indent=1)
for name, e in out.items():
    print(f"{name}: right {e['right']} lost {len(e['lost'])} won {e['won']} wrong {e['wrong']} new wrong {len(e['new_wrong'])} known pair {e['known_pair']} (new {len(e['known_pair_new'])})"
          + (f" related {e['related_pairs']} (+{e['related_new']} -{e['related_gone']})" if "related_pairs" in e else "")
          + (f" {e['by_part']}" if "by_part" in e else ""))
    if "by_condition" in e:
        print("   ", e["by_condition"])
