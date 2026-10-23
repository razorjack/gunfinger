"""Session 44 (experiment 0206): why reference pairs are lost, and what
pairing the query with more targets would recover. Reads the same
`recall --verify` and `shared` reports as `quantisation.py` (whose set
builders it uses), now with the pair count's `split` and `fan_outs`, and
a `robust --only combined --verify` report at 262 tracks.

usage: lost_pairs.py <out.json>
           --clean <recall clean> --damaged <recall damaged>
           [--other-rips <recall other rips>]
           --shared <shared report> <shared panel>  (repeatable)
           [--robust <robust report>]
           --clusters <clusters.json> --tags <tags.json> --map <library-map.json>
           [--fan-outs]

Without --fan-outs it prints the split and whether 0206's share is
reached; with it, also the fan-out scores (S1-S4), which 0206 opens only
when the share is reached.
"""
import argparse
import json
import os
import statistics
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import quantisation  # noqa: E402
from quantisation import auc, median, recall_sets, shared_sets  # noqa: E402
from shared_outcomes import clusters_of  # noqa: E402

RULE_HITS = 240
FAN_OUTS = (3, 5, 8)
E_BANDS = [(10, 30), (30, 60), (60, 120), (120, 240), (240, 10**9)]
KINDS = ("anchor_gone", "target_gone", "both_gone", "paired_same", "paired_near", "paired_far",
         "displaced_3", "displaced_4_5", "displaced_6_8", "displaced_later", "outside_zone")
# The registered groups of the lost pairs.
GROUPS = dict(
    endpoint_gone=("anchor_gone", "target_gone", "both_gone"),
    displaced=("displaced_3", "displaced_4_5", "displaced_6_8", "displaced_later"),
    displaced_3_to_8=("displaced_3", "displaced_4_5", "displaced_6_8"),
    changed_value=("paired_near", "paired_far"),
    paired_same_residual=("paired_same",),
    outside_zone=("outside_zone",),
)
session_30_measured = quantisation.measured


def measured(d, **extra):
    """`quantisation.measured` with the split and the fan-out counts."""
    m = session_30_measured(d, **extra)
    if m is None:
        return None
    return with_session_44(m, d["verified"])


def with_session_44(m, verified):
    pairs = verified["aligned"]["pairs"]
    m["split"] = pairs["split"]
    m["G"] = {f["fan_out"]: f["extra"] for f in pairs["fan_outs"]}
    m["Q_at"] = {f["fan_out"]: f["query"] for f in pairs["fan_outs"]}
    m["Gc"] = {F: statistics.mean(next(f["extra"] for f in s["pairs"]["fan_outs"] if f["fan_out"] == F)
                                  for s in verified["shifted"]) for F in m["G"]}
    return m


quantisation.measured = measured


def lost(m):
    """The detection's lost pairs (near and unmatched) by kind."""
    s = m["split"]
    return {k: s["near"][k] + s["unmatched"][k] for k in KINDS}


def split_summary(ds):
    """Pooled shares of the lost pairs by kind and group, the medians of
    the per-detection shares, the same over all reference pairs, and the
    check on exact pairs."""
    totals = {k: sum(lost(m)[k] for m in ds) for k in KINDS}
    in_n = {k: sum(m["split"]["near"][k] for m in ds) for k in KINDS}
    lost_all = sum(totals.values())
    reference = sum(m["R"] for m in ds)
    exact = sum(m["E"] for m in ds)
    exact_not_formed = sum(sum(m["split"]["exact"][k] for k in KINDS if k != "paired_same") for m in ds)

    def group(counts, keys):
        return sum(counts[k] for k in keys)

    per_detection = {g: median(group(lost(m), keys) / sum(lost(m).values())
                               for m in ds if sum(lost(m).values()))
                     for g, keys in GROUPS.items()}
    return dict(
        n=len(ds), reference=reference, exact=exact, lost=lost_all, near=sum(in_n.values()),
        exact_not_paired_same=exact_not_formed,
        lost_share={g: round(group(totals, keys) / lost_all, 4) if lost_all else None
                    for g, keys in GROUPS.items()},
        kind_share={k: round(totals[k] / lost_all, 4) if lost_all else None for k in KINDS},
        near_by_group={g: group(in_n, keys) for g, keys in GROUPS.items()},
        reference_share={g: round(group(totals, keys) / reference, 4) if reference else None
                         for g, keys in GROUPS.items()} | {"exact": round(exact / reference, 4) if reference else None},
        median_lost_share=per_detection,
        displaced_3_to_8_over_exact=round(group(totals, GROUPS["displaced_3_to_8"]) / exact, 4) if exact else None,
    )


def robust_sets(report):
    """T(0): per query its strongest detection of its own recording; W(0):
    the other detections with 10 hits or more that are not the blend
    partner, unlabelled."""
    truth, wrong = [], []
    for q in report["queries"]:
        own = []
        for d in q.get("verified") or []:
            v = d.get("verified")
            if not v or v["aligned"].get("pairs") is None:
                continue
            p = v["aligned"]["pairs"]
            m = dict(asset=d["asset"], hits=d["hits"], windows=d["windows"],
                     confident=d["level"] == "confident", E=p["exact"], N=p["near"],
                     R=p["reference"], Q=p["query"], V=v["aligned"]["found"],
                     P=v["aligned"]["reference_peaks"], role=d["role"])
            with_session_44(m, v)
            if d["role"] == "own":
                own.append(m)
            elif d["role"] != "partner" and d["hits"] >= 10:
                wrong.append(m)
        if own:
            truth.append(max(own, key=lambda m: m["hits"]))
    return truth, wrong


def fan_out_summary(ds, F):
    rs = [m for m in ds if m["E"] > 0]
    pooled_e = sum(m["E"] for m in rs)
    return dict(
        n=len(ds), median_E=median(m["E"] for m in ds),
        median_g_over_e=median(m["G"][F] / m["E"] for m in rs),
        pooled_g_over_e=round(sum(m["G"][F] for m in rs) / pooled_e, 4) if pooled_e else None,
        median_G=median(m["G"][F] for m in ds), median_Gc=median(m["Gc"][F] for m in ds),
        query_pairs_ratio=round(sum(m["Q_at"][F] for m in ds) / sum(m["Q"] for m in ds), 3) if ds else None,
    )


def by_e_band(ds, F):
    return {f"{lo}-{hi - 1 if hi < 10**9 else ''}": fan_out_summary([m for m in ds if lo <= m["E"] < hi], F)
            for lo, hi in E_BANDS}


def reaching(ds, F):
    under = [m for m in ds if m["hits"] < RULE_HITS]
    over = [m for m in under if m["hits"] + m["G"][F] >= RULE_HITS and m["windows"] >= 3]
    return dict(under_240=len(under), reach_240_in_3_windows=len(over),
                examples=sorted(({"file": os.path.basename(m["asset"]), "hits": m["hits"], "G": m["G"][F]}
                                 for m in over), key=lambda x: -(x["hits"] + x["G"]))[:8])


def separation(a, b, F):
    bands, wins, pairs = {}, 0.0, 0
    for lo, hi in E_BANDS[:4]:
        pa = [m["G"][F] / m["E"] for m in a if lo <= m["E"] < hi]
        pb = [m["G"][F] / m["E"] for m in b if lo <= m["E"] < hi]
        value, n = auc(pa, pb)
        bands[f"{lo}-{hi - 1}"] = dict(n=[len(pa), len(pb)], auc=None if value is None else round(value, 3))
        if value is not None:
            wins += value * n
            pairs += n
    return dict(bands=bands, pooled=round(wins / pairs, 3) if pairs else None)


def per_search(ds, searched, F):
    """Per shared search and index: a related file confident today, and with
    hits + G(F) at 240 or more over 3 windows."""
    searches = {key: [False, False] for key in searched}
    for m in ds:
        if m["role"] not in ("related", "version"):
            continue
        entry = searches[m["result"]]
        entry[0] |= m["confident"]
        entry[1] |= m["confident"] or (m["hits"] + m["G"][F] >= RULE_HITS and m["windows"] >= 3)
    out = {}
    for key, (today, wider) in searches.items():
        row = out.setdefault(key[2], dict(searches=0, related_today=0, related_with_fan_out=0))
        row["searches"] += 1
        row["related_today"] += today
        row["related_with_fan_out"] += wider
    return out


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("out")
    parser.add_argument("--clean", required=True)
    parser.add_argument("--damaged", required=True)
    parser.add_argument("--other-rips")
    parser.add_argument("--shared", nargs=2, action="append", default=[])
    parser.add_argument("--robust")
    parser.add_argument("--clusters", required=True)
    parser.add_argument("--tags", required=True)
    parser.add_argument("--map", required=True)
    parser.add_argument("--fan-outs", action="store_true")
    args = parser.parse_args()
    tags = json.load(open(args.tags))
    stands_for = {c["corpus"]: c["stands_for"] for c in json.load(open(args.map))["copies"]}
    cluster_of = clusters_of(args.clusters)

    sets, unrelated, related = {}, [], []
    for condition, path in (("C", args.clean), ("c", args.damaged), ("a", args.other_rips)):
        if not path:
            continue
        truth, u, r, _ = recall_sets(json.load(open(path)), condition, tags, stands_for)
        sets[f"T({condition})"] = [m for m in truth.values() if m is not None]
        unrelated += u
        related += r
    sets["F"], sets["R(r)"] = unrelated, related
    played, rel, searched = [], [], []
    for report_path, panel_path in args.shared:
        p, r, _, s_ = shared_sets(json.load(open(report_path)), json.load(open(panel_path)), cluster_of,
                                  os.path.basename(report_path))
        played += p
        rel += r
        searched += s_
    sets["P"], sets["R"] = played, rel
    sets["R not confident"] = [m for m in rel if not m["confident"]]
    if args.robust:
        sets["T(0)"], sets["W(0)"] = robust_sets(json.load(open(args.robust)))

    splits = {name: split_summary(ds) for name, ds in sets.items() if ds}
    t = splits["T(c)"]
    opened = (t["lost_share"]["displaced_3_to_8"] >= 0.15 or t["displaced_3_to_8_over_exact"] >= 0.25)
    out = dict(split=splits, share_reached=opened,
               criterion=dict(displaced_3_to_8_over_lost=t["lost_share"]["displaced_3_to_8"],
                              displaced_3_to_8_over_exact=t["displaced_3_to_8_over_exact"]))
    for name, s in splits.items():
        print(f"{name:16} n {s['n']:5} R {s['reference']:8} E {s['exact']:8} lost {s['lost']:8} "
              f"(N {s['near']:7}) exact-not-same {s['exact_not_paired_same']}")
        print(f"{'':16} lost shares {s['lost_share']}")
        print(f"{'':16} of R {s['reference_share']}")
        print(f"{'':16} median lost shares {s['median_lost_share']}")
        print(f"{'':16} kinds {s['kind_share']}  N by group {s['near_by_group']}")
    print("share reached:", opened, out["criterion"])

    if args.fan_outs:
        out["fan_outs"] = {}
        for F in FAN_OUTS:
            block = dict(
                s1={name: dict(all=fan_out_summary(ds, F), by_e=by_e_band(ds, F)) for name, ds in sets.items() if ds},
                s2={name: reaching(ds, F) for name, ds in sets.items() if ds and name not in ("P", "T(C)")},
                per_search=per_search(played + rel, searched, F),
                s3={f"{a}~{b}": separation(sets[a], sets[b], F) for a, b in
                    (("P", "R"), ("T(c)", "R(r)"), ("T(c)", "F")) if sets.get(a) and sets.get(b)},
            )
            out["fan_outs"][F] = block
            print(f"--- fan-out {F}")
            for name, s in block["s1"].items():
                print(f"S1 {name:16} {s['all']}")
                for band, b in s["by_e"].items():
                    if b["n"]:
                        print(f"     E {band:8} n {b['n']:5} med G/E {b['median_g_over_e']} pooled {b['pooled_g_over_e']}")
            for name, r in block["s2"].items():
                print(f"S2 {name:16} {r}")
            print("per search", block["per_search"])
            for pair, s in block["s3"].items():
                print(f"S3 {pair:12} pooled {s['pooled']} {s['bands']}")
    json.dump(out, open(args.out, "w"), indent=1)


if __name__ == "__main__":
    main()
