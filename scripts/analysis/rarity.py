"""Hash rarity against raw hits (session 85, brief 85's registration).

usage: rarity.py --shared <report.json> <panel.json> [--shared ...]
                 --recall <report.json> [--recall ...]
                 --clusters <clusters.json> <out.json>

Every detection that `gunfinger-eval --rarity` measured carries `rarity`:
the hits counted again (R), their weights summed (W, 1 / ln(e + list
length)), the hits outside the set-aside lists (K) and the median list
length. Roles as in `quantisation.py` (`shared_outcomes.py` with the
clusters): positives are the played recording's detections, negatives the
related recording's and the family's other versions; partner, unjudged and
other files are left out. `recall`'s right detections (own cluster or same
title) are true plays of real damage and brevity.

S1: AUC of W and of R, played against related. S2: the AUC of m = W / R
(and k = K / R) within bands of R, pooled over the bands by their pairs
(0.5: no information at equal hits). S3: S2 per family. S4: S2 for
recall's right detections against both panels' related ones; W*, the
weighted score that accepts as many related detections as R >= 240, and
what W >= W* accepts. S5: the recount within 5% of the detection's hits.
"""
import argparse
import bisect
import json
import os
import statistics
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from quantisation import panel_roles  # noqa: E402
from shared_outcomes import clusters_of, expanded, reroled, role  # noqa: E402

BANDS = [(30, 60), (60, 120), (120, 240), (240, 480), (480, 960), (960, 10**9)]
RULE_HITS = 240


def auc(positives, negatives):
    """P(positive > negative), ties half, by ranks; and the pairs."""
    if not positives or not negatives:
        return None, 0
    ordered = sorted(negatives)
    wins = 0.0
    for p in positives:
        below = bisect.bisect_left(ordered, p)
        equal = bisect.bisect_right(ordered, p) - below
        wins += below + 0.5 * equal
    pairs = len(positives) * len(negatives)
    return wins / pairs, pairs


def banded(pos, neg, measure):
    bands, wins, pairs = {}, 0.0, 0
    for lo, hi in BANDS:
        p = [d[measure] for d in pos if lo <= d["R"] < hi]
        n = [d[measure] for d in neg if lo <= d["R"] < hi]
        value, count = auc(p, n)
        bands[f"{lo}-{hi - 1 if hi < 10**9 else ''}"] = dict(
            n=[len(p), len(n)], auc=None if value is None else round(value, 3),
            median=[median(p), median(n)])
        if value is not None:
            wins += value * count
            pairs += count
    return dict(pooled=round(wins / pairs, 3) if pairs else None, pairs=pairs, bands=bands)


def median(values):
    values = list(values)
    return round(statistics.median(values), 4) if values else None


def measured(d, **extra):
    r = d.get("rarity")
    if not r or r["hits"] == 0:
        return None
    return dict(R=r["hits"], W=r["weighted"], m=r["weighted"] / r["hits"], k=r["kept"] / r["hits"],
                L=r["median_list"], hits=d["hits"], asset=d["asset"], **extra)


def shared_sets(report, panel, cluster_of, tag):
    roles = panel_roles(panel, report)
    recordings = expanded(report, cluster_of)
    played, related, unmeasured = [], [], 0
    for result in report["results"]:
        if result.get("error"):
            continue
        family, versions, unjudged, _ = roles[result["scenario"]]
        for d in result["detections"]:
            if d["role"] == "partner":
                continue
            r = role(dict(d, role=reroled(d, recordings.get(result["scenario"]))), versions, unjudged)
            if r not in ("played", "related", "version"):
                continue
            m = measured(d, panel=tag, family=family, scenario=result["scenario"], kind=result["kind"],
                         query=result["query"],
                         index=result["index"], role=r, confident=d["level"] == "confident")
            if m is None:
                unmeasured += d.get("fitted", False) and d["hits"] >= 30
                continue
            (played if r == "played" else related).append(m)
    return played, related, unmeasured


def recall_sets(report):
    right, wrong, weak = [], [], []
    for q in report["queries"]:
        for d in q["detections"]:
            m = measured(d, kind=q["kind"], family=q["family"], query_set=report["query_set"],
                         confident=d["confident"])
            if m is None:
                continue
            if d["correct"] or d.get("same_title"):
                right.append(m)
            elif d["confident"] and not d.get("partner"):
                wrong.append(m)
            elif not d.get("partner"):
                weak.append(m)
    return right, wrong, weak


def separation(pos, neg):
    if not pos or not neg:
        return None
    w, _ = auc([d["W"] for d in pos], [d["W"] for d in neg])
    r, _ = auc([d["R"] for d in pos], [d["R"] for d in neg])
    return dict(n=[len(pos), len(neg)], families=sorted({d["family"] for d in pos + neg}),
                auc_W=round(w, 3), auc_R=round(r, 3), gain=round(w - r, 3),
                at_equal_hits_m=banded(pos, neg, "m"), at_equal_hits_k=banded(pos, neg, "k"),
                median_m=[median(d["m"] for d in pos), median(d["m"] for d in neg)],
                median_L=[median(d["L"] for d in pos), median(d["L"] for d in neg)])


def paired(played, related):
    """Exploratory (not registered): in each query and index holding both,
    the strongest played detection's m against the strongest related one's."""
    def strongest(ds):
        out = {}
        for d in ds:
            key = (d["panel"], d["scenario"], d["query"], d["index"])
            if key not in out or d["R"] > out[key]["R"]:
                out[key] = d
        return out
    p, r = strongest(played), strongest(related)
    both = sorted(set(p) & set(r))
    higher = sum(p[k]["m"] > r[k]["m"] for k in both)
    families = sorted({k[1].rsplit("-", 1)[0] for k in both})
    return dict(searches=len(both), played_rarer=higher, share=round(higher / len(both), 3) if both else None,
                by_family={f: [sum(p[k]["m"] > r[k]["m"] for k in both if k[1].rsplit("-", 1)[0] == f),
                               sum(1 for k in both if k[1].rsplit("-", 1)[0] == f)] for f in families})


def recount(ds):
    within = sum(abs(d["R"] - d["hits"]) <= 0.05 * d["hits"] for d in ds)
    return dict(n=len(ds), within_5_percent=within, share=round(within / len(ds), 3) if ds else None,
                median_R_over_hits=median(d["R"] / d["hits"] for d in ds))


def accepted_like_the_rule(related, others):
    """W*, the weighted score accepting as many related detections as R >= 240."""
    n = sum(d["R"] >= RULE_HITS for d in related)
    ws = sorted((d["W"] for d in related), reverse=True)
    if n == 0:
        return None
    w_star = ws[n - 1]
    return dict(related_at_240=n, w_star=round(w_star, 2),
                related_at_w_star=sum(d["W"] >= w_star for d in related),
                **{name: dict(at_240=sum(d["R"] >= RULE_HITS for d in ds),
                              at_w_star=sum(d["W"] >= w_star for d in ds), n=len(ds))
                   for name, ds in others.items()})


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("out")
    parser.add_argument("--shared", nargs=2, action="append", default=[])
    parser.add_argument("--recall", action="append", default=[])
    parser.add_argument("--clusters", required=True)
    args = parser.parse_args()
    cluster_of = clusters_of(args.clusters)
    out, all_played, all_related = {"panels": {}}, [], []
    for number, (report_path, panel_path) in enumerate(args.shared, 1):
        report = json.load(open(report_path, encoding="utf-8"))
        panel = json.load(open(panel_path, encoding="utf-8"))
        played, related, unmeasured = shared_sets(report, panel, cluster_of, number)
        all_played += played
        all_related += related
        families = sorted({d["family"] for d in played + related})
        out["panels"][number] = dict(
            report=os.path.basename(report_path), unmeasured_fitted_30=unmeasured,
            all=separation(played, related),
            by_index={i: separation([d for d in played if d["index"] == i], [d for d in related if d["index"] == i])
                      for i in ("everything", "played left out", "source left out")},
            break_queries=separation([d for d in played if d["kind"] == "break"],
                                     [d for d in related if d["kind"] == "break"]),
            by_family={f: separation([d for d in played if d["family"] == f],
                                     [d for d in related if d["family"] == f]) for f in families},
            recount=dict(played=recount(played), related=recount(related)),
            paired=paired(played, related))
    out["pooled"] = separation(all_played, all_related)
    right, wrong, weak = [], [], []
    for path in args.recall:
        r, w, k = recall_sets(json.load(open(path, encoding="utf-8")))
        right += r
        wrong += w
        weak += k
    if right:
        for d in right:
            d["family"] = "recall:" + d["family"]
        out["recall"] = dict(
            right_against_related=separation(right, all_related),
            by_query_set={s: separation([d for d in right if d["query_set"] == s], all_related)
                          for s in sorted({d["query_set"] for d in right})},
            recount=dict(right=recount(right), wrong=recount(wrong)),
            wrong=[dict(asset=os.path.basename(d["asset"]), R=d["R"], m=round(d["m"], 4)) for d in wrong],
            median_m_right_confident=median(d["m"] for d in right if d["confident"]),
            # Wrong detections below confident, against the right ones at
            # equal hits (session 85's review; not registered).
            weak_wrong=dict(n=len(weak), median_m=median(d["m"] for d in weak),
                            right_against_weak_wrong=banded(right, weak, "m")["pooled"]))
    out["rule_counterpart"] = accepted_like_the_rule(
        all_related, {"panels_played": all_played, "recall_right": right})
    families = [(n, f, v) for n, p in out["panels"].items() for f, v in p["by_family"].items()]
    out["families_above_half"] = dict(
        total=len(families),
        above=sum(1 for *_, v in families if v and v["at_equal_hits_m"]["pooled"] is not None
                  and v["at_equal_hits_m"]["pooled"] > 0.5),
        each={f"{n}:{f}": (v and v["at_equal_hits_m"]["pooled"]) for n, f, v in families})
    json.dump(out, open(args.out, "w"), indent=1)


if __name__ == "__main__":
    main()
