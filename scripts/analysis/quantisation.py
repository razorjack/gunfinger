"""Session 30 (experiment 0149): what exact hash quantisation loses after
fitting. Reads `recall --verify` and `shared` reports whose verifier
counts carry `pairs` (exact E and near N pair hashes on the verifier's
alignment, aligned and at the four chance shifts) and the peaks found V,
builds 0149's sets and prints its measures M1-M4.

usage: quantisation.py <out.json>
           --clean <recall clean> --damaged <recall damaged>
           --other-rips <recall other rips>
           --shared <shared report> <shared panel>  (repeatable)
           [--shared-0055 <shared report>]
           --clusters <clusters.json> --tags <tags.json> --map <library-map.json>
           [--also-related <file name part>]...

Sets (fitted detections only):
- T(C), T(c), T(a): per observable searched query, its strongest
  detection of its own cluster in the clean, damaged and other-rips
  reports;
- F: the three recall reports' detections outside the own cluster, not
  the damage partner's, that `labels.py` calls unrelated to the source;
  R(r): those it calls related (same name, version, same artist);
- R: the shared reports' detections of the related recording (`related`
  and `version` roles, as `shared_outcomes.py` with the clusters); P: the
  played recording's; F(s): the rest, beside;
- R(0055), P(0055): the same in 0055's report.

With --also-related, F detections of files whose name holds one of the
given parts are reported beside as related (sets `F'` and `R(r)'`), for
a relation the tags do not show; the main sets keep `labels.py`'s.
"""
import argparse
import json
import os
import statistics
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from labels import label, name  # noqa: E402
from shared_outcomes import clusters_of, expanded, reroled, role, roles_of  # noqa: E402

HIT_BANDS = [(10, 60), (60, 120), (120, 240), (240, 10**9)]
E_BANDS = [(10, 30), (30, 60), (60, 120), (120, 240)]
RULE_HITS = 240


def measured(d, **extra):
    """A detection's counts, or None without the verifier's pairs."""
    verified = d.get("verified")
    if not d.get("fitted") or not verified or verified["aligned"].get("pairs") is None:
        return None
    aligned, shifted = verified["aligned"], verified["shifted"]
    mean = lambda values: sum(values) / len(values)
    return dict(
        asset=d["asset"], hits=d["hits"], windows=d["windows"],
        confident=d.get("confident", d.get("level") == "confident"),
        E=aligned["pairs"]["exact"], N=aligned["pairs"]["near"],
        V=aligned["found"], P=aligned["reference_peaks"],
        Q=aligned["pairs"]["query"], R=aligned["pairs"]["reference"],
        Ec=mean([s["pairs"]["exact"] for s in shifted]),
        Nc=mean([s["pairs"]["near"] for s in shifted]),
        Vc=mean([s["found"] for s in shifted]),
        **extra)


def strongest(detections):
    return max(detections, key=lambda d: d["hits"]) if detections else None


def recall_sets(report, condition, tags, stands_for):
    """T for the condition, keyed by (source, speed), and the false
    candidates split by label."""
    truth, unrelated, related = {}, [], []
    measured_count = unmeasured = 0
    for q in report["queries"]:
        observable = not q["held_out"] and not q.get("unobservable")
        own = []
        strongest_wrong = None
        for d in q["detections"]:
            m = measured(d, condition=condition, source=q["asset"], speed=q["speed_percent"])
            if d.get("fitted"):
                measured_count += m is not None
                unmeasured += m is None
            if m is None:
                continue
            if d["correct"]:
                if observable:
                    own.append(m)
                continue
            if d.get("partner"):
                continue
            kind = label(name(q["asset"], tags, stands_for), name(d["asset"], tags, stands_for))
            m["label"] = kind
            m["held_out_query"] = not observable
            (unrelated if kind == "unrelated" else related).append(m)
            if not observable and (strongest_wrong is None or m["hits"] > strongest_wrong["hits"]):
                strongest_wrong = m
        if strongest_wrong is not None:
            strongest_wrong["strongest_on_held_out"] = True
        best = strongest(own)
        if observable:
            truth[(q["asset"], q["speed_percent"])] = best
    return truth, unrelated, related, (measured_count, unmeasured)


def panel_roles(panel, report):
    """`shared_outcomes.roles_of` for a panel of directions. A panel that
    lists only each family's recordings (the second panel): a scenario's
    family is the one whose name begins the scenario's, and its recordings
    other than the scenario's played and related ones are versions."""
    if all("directions" in family for family in panel["families"]):
        return roles_of(panel)
    roles = {}
    for scenario in report["scenarios"]:
        family = next(f for f in panel["families"] if scenario["name"].startswith(f["name"] + "-"))
        own = {scenario["played"], scenario["related"], *scenario.get("played_recording", []),
               *scenario.get("related_recording", [])}
        versions = {path for recording in family["recordings"] if not own & set(recording)
                    for path in recording}
        roles[scenario["name"]] = (family["name"], versions, set(family["unjudged_same_title"]), False)
    return roles


def shared_sets(report, panel, cluster_of, tag):
    roles = panel_roles(panel, report)
    recordings = expanded(report, cluster_of)
    played, related, other, searched = [], [], [], []
    for result in report["results"]:
        if result.get("error"):
            continue
        searched.append((tag, result["query"], result["index"]))
        _, versions, unjudged, _ = roles[result["scenario"]] if result["scenario"] in roles else (None, set(), set(), None)
        for d in result["detections"]:
            if d["role"] == "partner":
                continue
            r = role(dict(d, role=reroled(d, recordings.get(result["scenario"]))), versions, unjudged)
            m = measured(d, scenario=result["scenario"], kind=result["kind"], index=result["index"],
                         query=result["query"], role=r, source=tag)
            if m is None:
                continue
            m["result"] = (tag, result["query"], result["index"])
            if r == "played":
                played.append(m)
            elif r in ("related", "version"):
                related.append(m)
            elif r == "other":
                other.append(m)
    return played, related, other, searched


def per_search(ds, searched):
    """Exploratory (not in 0149): per shared search and index, whether a
    related file is confident today, and whether one would be with E + N
    at 240 or more in 3 windows; the same for the played recording."""
    searches = {key: dict(index=key[2], related=[False, False], played=[False, False]) for key in searched}
    for m in ds:
        entry = searches[m["result"]]
        side = "played" if m["role"] == "played" else "related" if m["role"] in ("related", "version") else None
        if side is None:
            continue
        tolerant = m["confident"] or (m["E"] + m["N"] >= RULE_HITS and m["windows"] >= 3)
        entry[side][0] |= m["confident"]
        entry[side][1] |= tolerant
    out = {}
    for entry in searches.values():
        row = out.setdefault(entry["index"], dict(searches=0, related_today=0, related_tolerant=0,
                                                  related_new=0, played_today=0, played_tolerant=0))
        row["searches"] += 1
        row["related_today"] += entry["related"][0]
        row["related_tolerant"] += entry["related"][1]
        row["related_new"] += entry["related"][1] and not entry["related"][0]
        row["played_today"] += entry["played"][0]
        row["played_tolerant"] += entry["played"][1]
    return out


def median(values):
    values = list(values)
    return round(statistics.median(values), 3) if values else None


def ratios(m):
    E, N = m["E"], m["N"]
    if E == 0:
        return None
    return dict(n_e=N / E, gain=(E + N) / E, v_e=m["V"] / E, v_p=m["V"] / m["P"] if m["P"] else 0.0,
                excess_n_e=(N - m["Nc"]) / E, e_hits=E / m["hits"])


def summary(ds):
    rs = [r for r in map(ratios, ds) if r]
    return dict(
        n=len(ds), e_zero=len(ds) - len(rs),
        median_hits=median(m["hits"] for m in ds), median_E=median(m["E"] for m in ds),
        median_N=median(m["N"] for m in ds), median_Ec=median(m["Ec"] for m in ds),
        median_Nc=median(m["Nc"] for m in ds),
        **{f"median_{k}": median(r[k] for r in rs) for k in
           ("n_e", "gain", "v_e", "v_p", "excess_n_e", "e_hits")})


def by_band(ds, key="hits", bands=HIT_BANDS):
    return {f"{lo}-{hi - 1 if hi < 10**9 else ''}": summary([m for m in ds if lo <= m[key] < hi])
            for lo, hi in bands}


def reaching(ds):
    under = [m for m in ds if m["hits"] < RULE_HITS]
    return dict(
        under_240=len(under),
        e_plus_n=sum(m["E"] + m["N"] >= RULE_HITS for m in under),
        hits_plus_n=sum(m["hits"] + m["N"] >= RULE_HITS for m in under),
        e_plus_excess_n=sum(m["E"] + m["N"] - m["Nc"] >= RULE_HITS for m in under),
        # Exploratory (not in 0149): of those reaching 240 with E + N, the
        # ones whose detection spans 3 windows, as the rule also needs.
        e_plus_n_in_3_windows=sum(m["E"] + m["N"] >= RULE_HITS and m["windows"] >= 3 for m in under),
        examples=sorted(
            ({"file": os.path.basename(m["asset"]), "hits": m["hits"], "E": m["E"], "N": m["N"],
              "Nc": m["Nc"], **({"scenario": m["scenario"], "query": m["query"], "index": m["index"]}
                                 if "scenario" in m else {"condition": m["condition"]})}
             for m in under if m["E"] + m["N"] >= RULE_HITS),
            key=lambda x: -(x["E"] + x["N"]))[:12])


def auc(positives, negatives):
    """Probability that a positive scores above a negative (ties count half),
    and the counts of pairs behind it."""
    if not positives or not negatives:
        return None, 0
    wins = 0.0
    for p in positives:
        for n in negatives:
            wins += 1.0 if p > n else 0.5 if p == n else 0.0
    return wins / (len(positives) * len(negatives)), len(positives) * len(negatives)


def separation(a, b):
    """AUC of each measure within E bands and pooled over the bands."""
    out = {}
    for measure in ("n_e", "excess_n_e", "v_p"):
        bands, pooled_wins, pooled_pairs = {}, 0.0, 0
        for lo, hi in E_BANDS:
            pa = [ratios(m)[measure] for m in a if lo <= m["E"] < hi]
            pb = [ratios(m)[measure] for m in b if lo <= m["E"] < hi]
            value, pairs = auc(pa, pb)
            bands[f"{lo}-{hi - 1}"] = dict(n=[len(pa), len(pb)], auc=None if value is None else round(value, 3))
            if value is not None:
                pooled_wins += value * pairs
                pooled_pairs += pairs
        out[measure] = dict(bands=bands, pooled=round(pooled_wins / pooled_pairs, 3) if pooled_pairs else None)
    return out


def recovered(truth, clean):
    """M2: per source and speed, E and E + N over the clean own record's E."""
    rows = []
    for key, m in truth.items():
        c = clean.get(key)
        if m is None or c is None or c["E"] == 0:
            continue
        rows.append((m["E"] / c["E"], (m["E"] + m["N"]) / c["E"], m["hits"] / c["hits"]))
    return dict(n=len(rows), median_e_over_clean_e=median(r[0] for r in rows),
                median_e_plus_n_over_clean_e=median(r[1] for r in rows),
                median_hits_over_clean_hits=median(r[2] for r in rows))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("out")
    parser.add_argument("--clean", required=True)
    parser.add_argument("--damaged", required=True)
    parser.add_argument("--other-rips")
    parser.add_argument("--shared", nargs=2, action="append", default=[])
    parser.add_argument("--shared-0055")
    parser.add_argument("--clusters", required=True)
    parser.add_argument("--tags", required=True)
    parser.add_argument("--map", required=True)
    parser.add_argument("--also-related", action="append", default=[])
    args = parser.parse_args()
    tags = json.load(open(args.tags))
    stands_for = {c["corpus"]: c["stands_for"] for c in json.load(open(args.map))["copies"]}
    cluster_of = clusters_of(args.clusters)

    sets, coverage, truth = {}, {}, {}
    false_unrelated, false_related = [], []
    for condition, path in (("C", args.clean), ("c", args.damaged), ("a", args.other_rips)):
        if not path:
            continue
        t, u, r, cov = recall_sets(json.load(open(path)), condition, tags, stands_for)
        truth[condition] = t
        coverage[condition] = dict(fitted_measured=cov[0], fitted_unmeasured=cov[1],
                                   queries=len(t), with_truth=sum(v is not None for v in t.values()))
        sets[f"T({condition})"] = [m for m in t.values() if m is not None]
        false_unrelated += u
        false_related += r
    sets["F"] = false_unrelated
    sets["F wrong possible"] = [m for m in false_unrelated if m["hits"] >= 60]
    sets["F strongest on held-out"] = [m for m in false_unrelated if m.get("strongest_on_held_out")]
    sets["R(r)"] = false_related
    # Breakdowns by condition (exploratory, not in 0149).
    for condition in ("C", "c", "a"):
        for name_, ds in (("F", false_unrelated), ("R(r)", false_related)):
            subset = [m for m in ds if m["condition"] == condition]
            if subset:
                sets[f"{name_} in {condition}"] = subset
    if args.also_related:
        given = lambda m: any(part in os.path.basename(m["asset"]) for part in args.also_related)
        sets["F'"] = [m for m in false_unrelated if not given(m)]
        sets["R(r)'"] = false_related + [m for m in false_unrelated if given(m)]
    played, related, other, searched = [], [], [], []
    for report_path, panel_path in args.shared:
        p, r, o, s_ = shared_sets(json.load(open(report_path)), json.load(open(panel_path)), cluster_of,
                                  os.path.basename(report_path))
        played += p
        related += r
        other += o
        searched += s_
    sets["P"], sets["R"], sets["F(s)"] = played, related, other
    searches = per_search(played + related, searched)
    sets["R confident"] = [m for m in related if m["confident"]]
    if args.shared_0055:
        report = json.load(open(args.shared_0055))
        p5, r5 = [], []
        for result in report["results"]:
            for d in result["detections"]:
                m = measured(d, scenario=result["scenario"], kind=result["kind"], index=result["index"],
                             query=result["query"], role=d["role"])
                if m and d["role"] == "played":
                    p5.append(m)
                elif m and d["role"] == "related":
                    r5.append(m)
        sets["P(0055)"], sets["R(0055)"] = p5, r5

    out = dict(
        coverage=coverage,
        m1={name_: dict(all=summary(ds), by_hits=by_band(ds)) for name_, ds in sets.items()},
        m2={c: recovered(truth[c], truth["C"]) for c in ("c", "a") if c in truth and "C" in truth},
        m3={name_: reaching(ds) for name_, ds in sets.items() if name_ != "P"},
        per_search=searches,
        m4={f"{a}~{b}": separation(sets[a], sets[b]) for a, b in
            (("T(c)", "F"), ("T(a)", "F"), ("T(c)", "R"), ("P", "R"), ("T(C)", "F"))
            if a in sets and b in sets},
    )
    json.dump(out, open(args.out, "w"), indent=1)

    print("coverage", json.dumps(coverage))
    print(f"{'set':26} {'n':>5} {'E=0':>4} {'hits':>6} {'E':>6} {'N':>6} {'Ec':>6} {'Nc':>6} "
          f"{'N/E':>6} {'gain':>6} {'xsN/E':>6} {'V/E':>6} {'V/P':>6} {'E/hit':>6}")
    for name_, block in out["m1"].items():
        for band, s in [("all", block["all"])] + list(block["by_hits"].items()):
            if not s["n"]:
                continue
            fmt = lambda v: "" if v is None else f"{v:.2f}" if isinstance(v, float) else str(v)
            print(f"{(name_ + ' ' + band):26} {s['n']:>5} {s['e_zero']:>4} {fmt(s['median_hits']):>6} "
                  f"{fmt(s['median_E']):>6} {fmt(s['median_N']):>6} {fmt(s['median_Ec']):>6} "
                  f"{fmt(s['median_Nc']):>6} {fmt(s['median_n_e']):>6} {fmt(s['median_gain']):>6} "
                  f"{fmt(s['median_excess_n_e']):>6} {fmt(s['median_v_e']):>6} {fmt(s['median_v_p']):>6} "
                  f"{fmt(s['median_e_hits']):>6}")
    print("M2", json.dumps(out["m2"]))
    for index, row in searches.items():
        print("per search", index, row)
    for name_, r in out["m3"].items():
        print("M3", name_, {k: v for k, v in r.items() if k != "examples"})
    for pair, block in out["m4"].items():
        print("M4", pair, {m: (b["pooled"], {k: v["auc"] for k, v in b["bands"].items()},
                               {k: v["n"] for k, v in b["bands"].items()}) for m, b in block.items()})


if __name__ == "__main__":
    main()
