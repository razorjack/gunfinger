"""Session 66: the harmonic view's pilot, scored as brief 66's pre-registration says.

Reads `shared --harmonic` reports. A case is a detection measured in the
views (20 hits or more). Positives: the played recording's files (role
`played`) and the blend partner's (`partner`, which plays in the blend and
damaged queries); negatives: the related recording's (`related`); `other`
apart. Scores: each view's excess share (share found minus the mean of the
four chance shifts), hits, and the peak verifier's excess share beside. A
view with fewer than 50 reference peaks in the span abstains: kept, never
rejected, left out of the AUC.

    harmonic_views.py develop OUT.json REPORT...   thresholds frozen into OUT.json
    harmonic_views.py frozen THRESHOLDS.json OUT.json REPORT...

Both print the tables and write a summary (families and numbers only).
"""
import json
import math
import sys

MEASURES = ["harmonic", "denser", "percussive", "hits", "verifier"]
VIEWS = {"harmonic", "denser", "percussive"}
MIN_REFERENCE_PEAKS = 50
BANDS = [(20, 59), (60, 239), (240, 10**9)]
LOOPS = {"looped", "break"}


def family(scenario):
    return scenario.rsplit("-", 1)[0]


def cases(paths):
    out = []
    for path in paths:
        report = json.load(open(path))
        for result in report["results"]:
            for d in result["detections"]:
                views = d.get("views")
                if views is None:
                    continue
                role = d["role"]
                label = {"played": "positive", "partner": "positive", "related": "negative"}.get(role, "other")
                case = {
                    "family": family(result["scenario"]),
                    "scenario": result["scenario"],
                    "kind": result["kind"],
                    "query": result["query"],
                    "index": result["index"],
                    "role": role,
                    "asset": d["asset"],
                    "label": label,
                    "hits": d["hits"],
                    "level": d["level"],
                }
                for view in VIEWS:
                    v = views[view]
                    case[view] = None if v["reference_peaks"] < MIN_REFERENCE_PEAKS else v["excess_share"]
                    case[view + "_share"] = v["share"]
                verified = d.get("verified")
                if verified:
                    shifted = [c["share"] for c in verified["shifted"]]
                    case["verifier"] = verified["aligned"]["share"] - sum(shifted) / len(shifted)
                else:
                    case["verifier"] = None
                case["hits_measure"] = d["hits"]
                out.append(case)
    return out


def value(case, measure):
    return case["hits_measure"] if measure == "hits" else case[measure]


def hit_matched(all_cases):
    negatives = [c for c in all_cases if c["label"] == "negative"]
    if not negatives:
        return [], negatives
    low = min(c["hits"] for c in negatives)
    high = max(c["hits"] for c in negatives)
    positives = [c for c in all_cases if c["label"] == "positive" and low <= c["hits"] <= high]
    return positives, negatives


def thresholds(negatives):
    out = {}
    for measure in MEASURES:
        values = sorted(v for v in (value(c, measure) for c in negatives) if v is not None)
        if not values:
            out[measure] = {"rate_0": None, "rate_10": None}
            continue
        rank = max(math.ceil(0.9 * len(values)) - 1, 0)
        out[measure] = {"rate_0": values[-1], "rate_10": values[rank]}
    return out


def kept(case, measure, threshold):
    v = value(case, measure)
    return v is None or threshold is None or v > threshold


def auc(positives, negatives, measure):
    p = [value(c, measure) for c in positives if value(c, measure) is not None]
    n = [value(c, measure) for c in negatives if value(c, measure) is not None]
    if not p or not n:
        return None
    wins = sum((x > y) + 0.5 * (x == y) for x in p for y in n)
    return wins / (len(p) * len(n))


def near_identical(all_cases):
    """Negatives whose D excess reaches the 5th percentile of the other-rip positives'."""
    rips = sorted(
        c["denser"]
        for c in all_cases
        if c["label"] == "positive" and c["role"] == "played" and c["index"] == "source left out" and c["denser"] is not None
    )
    if not rips:
        return None, set()
    floor = rips[max(math.ceil(0.05 * len(rips)) - 1, 0)]
    marked = {id(c) for c in all_cases if c["label"] == "negative" and c["denser"] is not None and c["denser"] >= floor}
    return floor, marked


def retained(positives, measure, threshold):
    return sum(kept(c, measure, threshold) for c in positives)


def summary(all_cases, frozen_thresholds=None):
    positives, negatives = hit_matched(all_cases)
    limits = frozen_thresholds or thresholds(negatives)
    floor, marked = near_identical(all_cases)
    families = sorted({c["family"] for c in all_cases})
    out = {
        "cases": len(all_cases),
        "positives": sum(c["label"] == "positive" for c in all_cases),
        "positives_hit_matched": len(positives),
        "negatives": len(negatives),
        "others": sum(c["label"] == "other" for c in all_cases),
        "negative_hits": [min((c["hits"] for c in negatives), default=None), max((c["hits"] for c in negatives), default=None)],
        "abstained": {m: {"positives": sum(c[m] is None for c in positives), "negatives": sum(c[m] is None for c in negatives)} for m in VIEWS},
        "thresholds": limits,
        "near_identical_floor": floor,
        "near_identical_negatives": len(marked),
        "measures": {},
        "families": {},
    }
    killed = [c for c in positives if c["kind"] == "killed"]
    unlooped_p = [c for c in positives if c["kind"] not in LOOPS]
    unlooped_n = [c for c in negatives if c["kind"] not in LOOPS]
    distinct_n = [c for c in negatives if id(c) not in marked]
    for m in MEASURES:
        row = {"auc": auc(positives, negatives, m)}
        for rate in ("rate_0", "rate_10"):
            t = limits[m][rate]
            row[rate] = {
                "retained": retained(positives, m, t),
                "negatives_kept": sum(kept(c, m, t) for c in negatives),
                "killed_retained": retained(killed, m, t),
                "killed": len(killed),
            }
        row["auc_by_band"] = {
            f"{lo}-{hi if hi < 10**9 else ''}": auc(
                [c for c in positives if lo <= c["hits"] <= hi], [c for c in negatives if lo <= c["hits"] <= hi], m
            )
            for lo, hi in BANDS
        }
        row["auc_without_loops"] = auc(unlooped_p, unlooped_n, m)
        row["auc_without_near_identical"] = auc(positives, distinct_n, m)
        if frozen_thresholds is None:
            t = thresholds(distinct_n)[m]["rate_0"]
            row["rate_0_without_near_identical"] = {"threshold": t, "retained": retained(positives, m, t)}
        else:
            t = limits[m]["rate_0"]
            row["rate_0_without_near_identical"] = {"negatives_kept": sum(kept(c, m, t) for c in distinct_n), "of": len(distinct_n)}
        out["measures"][m] = row
    for f in families:
        fp = [c for c in positives if c["family"] == f]
        fn = [c for c in negatives if c["family"] == f]
        out["families"][f] = {
            "positives": len(fp),
            "negatives": len(fn),
            **{
                m: {
                    "retained_rate_0": retained(fp, m, limits[m]["rate_0"]),
                    "retained_rate_10": retained(fp, m, limits[m]["rate_10"]),
                    "negatives_kept_rate_0": sum(kept(c, m, limits[m]["rate_0"]) for c in fn),
                    "negatives_kept_rate_10": sum(kept(c, m, limits[m]["rate_10"]) for c in fn),
                    "auc": auc(fp, fn, m),
                }
                for m in MEASURES
            },
        }
    return out


def show(out, all_cases):
    f = lambda x: "-" if x is None else (f"{x:.3f}" if isinstance(x, float) else str(x))
    print(
        f"cases {out['cases']}: positives {out['positives']} ({out['positives_hit_matched']} at the negatives' hits "
        f"{out['negative_hits']}), negatives {out['negatives']}, other {out['others']}; "
        f"near-identical negatives {out['near_identical_negatives']} (D floor {f(out['near_identical_floor'])})"
    )
    print("abstained", out["abstained"])
    print(f"{'measure':<11} {'T0':>7} {'kept':>9} {'neg':>5} {'T10':>7} {'kept':>9} {'neg':>5} {'killed T0':>10} {'AUC':>6}  bands; no loops; no near-identical")
    n = out["positives_hit_matched"]
    for m, row in out["measures"].items():
        t = out["thresholds"][m]
        r0, r10 = row["rate_0"], row["rate_10"]
        print(
            f"{m:<11} {f(t['rate_0']):>7} {r0['retained']:>4}/{n:<4} {r0['negatives_kept']:>5} {f(t['rate_10']):>7} "
            f"{r10['retained']:>4}/{n:<4} {r10['negatives_kept']:>5} {r0['killed_retained']:>4}/{r0['killed']:<5} {f(row['auc']):>6}  "
            f"{' '.join(f(v) for v in row['auc_by_band'].values())}; {f(row['auc_without_loops'])}; {f(row['auc_without_near_identical'])} "
            f"{row['rate_0_without_near_identical']}"
        )
    print("by family (hit-matched positives retained at T0 / T10; negatives kept at T0 / T10; AUC):")
    for fam, row in out["families"].items():
        cells = " | ".join(
            f"{m[:4]} {row[m]['retained_rate_0']}/{row[m]['retained_rate_10']} ({row[m]['negatives_kept_rate_0']}/{row[m]['negatives_kept_rate_10']}) {f(row[m]['auc'])}"
            for m in ("harmonic", "denser", "hits")
        )
        print(f"  {fam:<18} +{row['positives']:<4} -{row['negatives']:<4} {cells}")


def cool_rock(all_cases, limits):
    rows = [c for c in all_cases if c["family"] == "cool-rock" and c["label"] != "positive"]
    rows.sort(key=lambda c: -c["hits"])
    print("cool-rock negatives and others (hits, level, H, D, P excess; kept by H, D at T0):")
    for c in rows[:40]:
        print(
            f"  {c['scenario']:<12} {c['query'][:32]:<32} {c['index'][:12]:<12} {c['role']:<8} {c['hits']:>5} {c['level']:<9} "
            + " ".join("-" if c[m] is None else f"{c[m]:.3f}" for m in ("harmonic", "denser", "percussive"))
            + f"  H {'kept' if kept(c, 'harmonic', limits['harmonic']['rate_0']) else 'rejected'}"
            + f"  D {'kept' if kept(c, 'denser', limits['denser']['rate_0']) else 'rejected'}"
        )


def main():
    mode = sys.argv[1]
    if mode == "develop":
        out_path, paths = sys.argv[2], sys.argv[3:]
        all_cases = cases(paths)
        out = summary(all_cases)
    elif mode == "frozen":
        limits = json.load(open(sys.argv[2]))["thresholds"]
        out_path, paths = sys.argv[3], sys.argv[4:]
        all_cases = cases(paths)
        out = summary(all_cases, limits)
        cool_rock(all_cases, limits)
    else:
        raise SystemExit(__doc__)
    show(out, all_cases)
    json.dump(out, open(out_path, "w"), indent=1)


if __name__ == "__main__":
    main()
