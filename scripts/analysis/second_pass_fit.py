"""Session 63: a second-pass fit's reports against the default's, query by query.

Usage:
  second_pass_fit.py queries DEFAULT.json VARIANT.json
      sweep or recall reports (the same queries in the same order)
  second_pass_fit.py shared DEFAULT.json VARIANT.json
  second_pass_fit.py scans DEFAULT.json VARIANT.json
  second_pass_fit.py moved TODAY.json LIKE.json [QUERY...]
      recall under today's lists and under another index's
      (`--set-aside-like`), the same fit: the queries whose right answer
      moved, and the named queries' best right detections

Prints one JSON object of the registered scores (docs/brief-63.md): confident
right answers gained and lost, wrong confident detections added, hit ratios
for weak true candidates and negatives of 80 hits or more, per-query margins
and the fitted speed's error on a query's own file.
"""
import json
import statistics
import sys

FLOOR = 80
RULE = 240
WEAK = 3 * RULE


def load(path):
    with open(path) as f:
        return json.load(f)


def right(d):
    return d.get("correct") or d.get("same_title")


def wrong(d):
    return not right(d) and not d.get("partner")


def best(detections, keep):
    hits = [d["hits"] for d in detections if keep(d)]
    return max(hits) if hits else 0


def by_asset(detections):
    out = {}
    for d in detections:
        if d["hits"] > out.get(d["asset"], {"hits": -1})["hits"]:
            out[d["asset"]] = d
    return out


def confident_by_asset(detections):
    return by_asset([d for d in detections if d["level"] == "confident"])


def ratios(values):
    if not values:
        return {"n": 0}
    values = sorted(values)
    return {
        "n": len(values),
        "median": round(statistics.median(values), 3),
        "p10": round(values[len(values) // 10], 3),
        "p90": round(values[min(len(values) - 1, (9 * len(values)) // 10)], 3),
        "largest": round(values[-1], 3),
        "smallest": round(values[0], 3),
    }


def key(q):
    return (q["asset"], round(q["start_seconds"], 3), q["speed_percent"], q.get("kind"))


def queries(default_path, variant_path):
    a, b = load(default_path)["queries"], load(variant_path)["queries"]
    assert len(a) == len(b), (len(a), len(b))
    out = {"queries": len(a), "gained": [], "lost": [], "wrong_added": [], "wrong_removed": 0}
    weak, negatives, margins, errors = [], [], [], {"default": [], "variant": []}
    for i, (qa, qb) in enumerate(zip(a, b)):
        assert key(qa) == key(qb), (i, key(qa), key(qb))
        da, db = qa["detections"], qb["detections"]
        right_a = any(d["confident"] and right(d) for d in da)
        right_b = any(d["confident"] and right(d) for d in db)
        label = f"{i} {qa['asset'].split('/')[-1]} {qa['speed_percent']:+}%"
        if qa.get("brief"):
            label += " brief"
        if right_b and not right_a:
            out["gained"].append(label + f" {best(da, right)}->{best(db, right)}")
        if right_a and not right_b:
            out["lost"].append(label + f" {best(da, right)}->{best(db, right)}")
        wrong_a = {d["asset"] for d in da if d["confident"] and wrong(d)}
        wrong_b = {d["asset"] for d in db if d["confident"] and wrong(d)}
        for asset in sorted(wrong_b - wrong_a):
            hits = by_asset(db)[asset]["hits"]
            out["wrong_added"].append(label + f" {asset.split('/')[-1]} {hits}")
        out["wrong_removed"] += len(wrong_a - wrong_b)
        va = by_asset(da)
        vb = by_asset(db)
        for asset, d in va.items():
            if d["hits"] < FLOOR:
                continue
            ratio = vb[asset]["hits"] / d["hits"] if asset in vb else 0.0
            if right(d) and d["hits"] < WEAK:
                weak.append(ratio)
            elif wrong(d):
                negatives.append(ratio)
        ra, wa = best(da, right), best(da, wrong)
        rb, wb = best(db, right), best(db, wrong)
        if ra >= FLOOR and wa > 0 and rb > 0 and wb > 0:
            margins.append((rb / wb) / (ra / wa))
        if not qa.get("held_out"):
            for name, ds in (("default", da), ("variant", db)):
                own = [d for d in ds if d["asset"] == qa["asset"] and d["hits"] >= FLOOR]
                if own:
                    top = max(own, key=lambda d: d["hits"])
                    errors[name].append(abs(top["speed_percent"] - qa["speed_percent"]))
    out["confident_right"] = {
        "default": sum(any(d["confident"] and right(d) for d in q["detections"]) for q in a),
        "variant": sum(any(d["confident"] and right(d) for d in q["detections"]) for q in b),
    }
    out["weak_true_ratio"] = ratios(weak)
    out["negative_ratio"] = ratios(negatives)
    out["negatives_over_rule"] = {
        "default": sum(1 for q in a for d in q["detections"] if wrong(d) and d["hits"] >= RULE),
        "variant": sum(1 for q in b for d in q["detections"] if wrong(d) and d["hits"] >= RULE),
    }
    out["strongest_wrong"] = {
        "default": max([best(q["detections"], wrong) for q in a] or [0]),
        "variant": max([best(q["detections"], wrong) for q in b] or [0]),
    }
    out["margin_change"] = ratios(margins)
    out["own_speed_error_percent"] = {
        name: {"n": len(v), "median": round(statistics.median(v), 4) if v else None,
               "p90": round(sorted(v)[(9 * len(v)) // 10], 4) if v else None}
        for name, v in errors.items()
    }
    return out


def shared(default_path, variant_path):
    a, b = load(default_path)["results"], load(variant_path)["results"]
    assert len(a) == len(b)
    out = {"results": len(a), "confident": {}, "newly_confident": [], "no_longer_confident": []}
    ratio = {"played": [], "related": [], "other": [], "partner": []}
    for role in ratio:
        out["confident"][role] = {
            "default": sum(d["level"] == "confident" and d["role"] == role for r in a for d in r["detections"]),
            "variant": sum(d["level"] == "confident" and d["role"] == role for r in b for d in r["detections"]),
        }
    for ra, rb in zip(a, b):
        assert (ra["query"], ra["index"]) == (rb["query"], rb["index"])
        va, vb = by_asset(ra["detections"]), by_asset(rb["detections"])
        label = f"{ra['query']} [{ra['index']}]"
        # An asset is confident when any of its detections is, not only its strongest.
        ca, cb = confident_by_asset(ra["detections"]), confident_by_asset(rb["detections"])
        for asset in sorted(set(cb) - set(ca)):
            d, before = cb[asset], va.get(asset)
            out["newly_confident"].append(
                f"{label} {d['role']} {asset.split('/')[-1]} {before['hits'] if before else 0}->{d['hits']}")
        for asset in sorted(set(ca) - set(cb)):
            d, after = ca[asset], vb.get(asset)
            out["no_longer_confident"].append(
                f"{label} {d['role']} {asset.split('/')[-1]} {d['hits']}->{after['hits'] if after else 0}")
        for asset, d in va.items():
            if d["hits"] >= FLOOR:
                ratio[d["role"]].append(vb[asset]["hits"] / d["hits"] if asset in vb else 0.0)
    out["ratio"] = {role: ratios(v) for role, v in ratio.items()}
    out["strongest_related"] = {
        "default": max([d["hits"] for r in a for d in r["detections"] if d["role"] == "related"] or [0]),
        "variant": max([d["hits"] for r in b for d in r["detections"] if d["role"] == "related"] or [0]),
    }
    return out


def scans(default_path, variant_path):
    a, b = load(default_path), load(variant_path)
    out = {name: {k: x["score"][k] for k in ("referenced", "identified", "wrong", "possible")}
           for name, x in (("default", a), ("variant", b))}
    changes = []
    for ta, tb in zip(a["score"]["tracks"], b["score"]["tracks"]):
        ha = max([c["hits"] for c in ta.get("credited", [])] or [0])
        hb = max([c["hits"] for c in tb.get("credited", [])] or [0])
        if ta["identified"] != tb["identified"] or ha != hb:
            changes.append(f"{ta['label'].strip()} {ta['identified']}->{tb['identified']} {ha}->{hb}")
    out["tracks_changed"] = changes
    confident = lambda x: sum(d["evidence"]["confident"] if "evidence" in d else d.get("confident", False) for d in x["detections"])
    out["confident_detections"] = {"default": confident(a), "variant": confident(b)}
    return out


def moved(today_path, like_path, *named):
    a, b = load(today_path)["queries"], load(like_path)["queries"]
    assert len(a) == len(b)
    out = {"queries": len(a), "right_moved": 0, "confident_changed": [], "detections_moved": 0, "named": {}}

    def top(q):
        own = [d for d in q["detections"] if right(d)]
        if not own:
            return None
        d = max(own, key=lambda d: d["hits"])
        return {"hits": d["hits"], "speed_percent": round(d["speed_percent"], 4), "confident": d["confident"]}

    for i, (qa, qb) in enumerate(zip(a, b)):
        assert key(qa) == key(qb)
        ta, tb = top(qa), top(qb)
        if qa["detections"] != qb["detections"]:
            out["detections_moved"] += 1
        if ta != tb:
            out["right_moved"] += 1
        if (ta or {}).get("confident") != (tb or {}).get("confident"):
            out["confident_changed"].append(f"{i} {qa['asset'].split('/')[-1]} {ta}->{tb}")
        if str(i) in named:
            out["named"][str(i)] = {"today": ta, "like": tb}
    return out


if __name__ == "__main__":
    mode, default_path, variant_path = sys.argv[1:4]
    if mode == "moved":
        result = moved(default_path, variant_path, *sys.argv[4:])
    else:
        result = {"queries": queries, "shared": shared, "scans": scans}[mode](default_path, variant_path)
    print(json.dumps(result, indent=1))
