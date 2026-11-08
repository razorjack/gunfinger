"""Scores `gunfinger-eval break-search` reports as retrieval with many relevant
holders (brief 86's registration, R2; experiment 0296).

    python3 -I scripts/analysis/break_retrieval.py CASES.json CLUSTERS.json OUT.json \
        LADDER=REPORT.json [LADDER=REPORT.json ...] [--same-titles FILE] \
        [--unrelated FILE] [--old-clusters CLUSTERS.json]

Each listed holder (own and others merged, ranked by hits, the query file left
out) is *relevant* (in the query file's cluster; an expected file or one of
the case's own files, or in their clusters; or paired with one of these by
`--same-titles`), *unrelated* (judged by ear not to share the passage:
`--unrelated`, tab-separated query file and holder, followed through the
clusters) or *unjudged*. Per ladder, mode and group: precision at 5, 10 and 20
over judged holders, the judged and relevant shares of the first K,
expected-holder recall (listed at all and in the first 20), the share of
listed holders unrelated, and the null: the strongest holder outside the
query's cluster (N1's 95th percentile against the found expected files'
median hits), with the same null under `--old-clusters` for contrast."""

import json
import math
import statistics
import sys

POSITIVE = ("A", "B", "C", "D", "E")
NEGATIVE = ("N1", "N2")
KS = (5, 10, 20)
REACH_SECONDS = 60.0


def percentile(values, share):
    """The value at `share` of the sorted values (nearest rank)."""
    if not values:
        return None
    ordered = sorted(values)
    rank = max(1, math.ceil(share * len(ordered)))
    return ordered[min(rank, len(ordered)) - 1]


def median(values):
    return statistics.median(values) if values else None


def cluster_index(clusters):
    """Each clustered file's members (a frozenset), from a clusters file."""
    index = {}
    for members in clusters["duplicates"]:
        group = frozenset(members)
        for member in members:
            index[member] = group
    return index


def cluster_of(index, file):
    return index.get(file, frozenset([file]))


def read_pairs(path, keyword=None):
    """Tab-separated pairs; with `keyword`, lines starting with it."""
    pairs = []
    if not path:
        return pairs
    for line in open(path):
        if not line.strip() or line.startswith("#"):
            continue
        fields = line.rstrip("\n").split("\t")
        if keyword is not None:
            if fields[0] != keyword:
                continue
            fields = fields[1:]
        pairs.append((fields[0], fields[1]))
    return pairs


def partners(pairs, files):
    """The files `pairs` pair with any of `files`, either way round."""
    found = set()
    for a, b in pairs:
        if a in files:
            found.add(b)
        if b in files:
            found.add(a)
    return found


def relevant_set(case, index, same_titles):
    seeds = {case["file"], *case.get("expected", []), *case.get("own_extra", [])}
    seeds |= partners(same_titles, seeds)
    relevant = set()
    for file in seeds:
        relevant |= cluster_of(index, file)
    return relevant


def unrelated_set(case, index, unrelated):
    own = cluster_of(index, case["file"])
    judged = {b for a, b in unrelated if a in own} | {a for a, b in unrelated if b in own}
    found = set()
    for file in judged:
        found |= cluster_of(index, file)
    return found


def holders(searched):
    """Own and other holders merged, strongest first, the query file left out."""
    merged = [h for h in searched["own"] + searched["others"] if h["file"] != searched["file"]]
    merged.sort(key=lambda h: (-h["hits"], h["file"]))
    return merged


def label(file, relevant, unrelated):
    if file in relevant:
        return "relevant"
    if file in unrelated:
        return "unrelated"
    return "unjudged"


def at_k(labels, k):
    first = labels[:k]
    rel = first.count("relevant")
    unrel = first.count("unrelated")
    n = len(first)
    return {
        "listed": n,
        "relevant": rel,
        "unrelated": unrel,
        "unjudged": n - rel - unrel,
        "precision_judged": rel / (rel + unrel) if rel + unrel else None,
        "judged_share": (rel + unrel) / n if n else None,
        "relevant_share": rel / n if n else None,
    }


def score_search(case, searched, index, same_titles, unrelated):
    relevant = relevant_set(case, index, same_titles)
    unrel = unrelated_set(case, index, unrelated) - relevant
    listed = holders(searched)
    labels = [label(h["file"], relevant, unrel) for h in listed]
    out = {"listed": len(listed), "unrelated": labels.count("unrelated"),
           "unjudged": labels.count("unjudged"), "at": {k: at_k(labels, k) for k in KS}}
    missing = set(searched["expected_not_indexed"])
    expected = [e for e in case.get("expected", []) if e not in missing]
    if expected:
        targets = set()
        for e in expected:
            targets |= cluster_of(index, e)
        own = cluster_of(index, case["file"])
        out["expected_joined_to_query"] = bool(targets & own)
        ranks = [i for i, h in enumerate(listed) if h["file"] in targets]
        out["expected_found"] = bool(ranks)
        out["expected_in_20"] = bool(ranks) and ranks[0] < 20
        out["expected_hits"] = listed[ranks[0]]["hits"] if ranks else None
        above = sum(1 for lab in labels[:ranks[0]] if lab != "relevant") if ranks else None
        out["expected_rank_among_others"] = above + 1 if ranks else None
    return out


def strongest_outside(case, searched, index):
    """The strongest holder outside the query's cluster and the case's own
    files, and whether any such holder reaches `REACH_SECONDS` of the query."""
    apart = set(cluster_of(index, case["file"]))
    for f in case.get("own_extra", []):
        apart |= cluster_of(index, f)
    outside = [h for h in holders(searched) if h["file"] not in apart]
    reaching = [h for h in outside if h.get("reach") and
                h["reach"]["query_end_seconds"] - h["reach"]["query_start_seconds"] >= REACH_SECONDS]
    return (outside[0]["hits"] if outside else 0), bool(reaching), (outside[0]["file"] if outside else None)


def summarise(cases, reports, index, same_titles, unrelated, old_index=None):
    by_name = {c["name"]: c for c in cases}
    summary = {}
    for ladder, report in reports.items():
        for mode in ("all", "filtered"):
            rows = [(by_name[s["name"]], s) for s in report["searches"]
                    if s["lists"] == mode and s["name"] in by_name and not s.get("error")]
            out = {"groups": {}, "errors": [s["name"] for s in report["searches"]
                                            if s["lists"] == mode and s.get("error")]}
            scored = {}
            for case, searched in rows:
                scored[case["name"]] = score_search(case, searched, index, same_titles, unrelated)
            for group in POSITIVE + NEGATIVE:
                members = [(c, s) for c, s in rows if c["group"] == group]
                if not members:
                    continue
                g = {"queries": len(members),
                     "recordings": len({min(cluster_of(index, c["file"])) for c, _ in members}),
                     "search_seconds_median": median([s["search_seconds"] for _, s in members])}
                g["listed_median"] = median([scored[c["name"]]["listed"] for c, _ in members])
                listed = sum(scored[c["name"]]["listed"] for c, _ in members)
                g["unrelated_share"] = (sum(scored[c["name"]]["unrelated"] for c, _ in members) / listed
                                        if listed else None)
                g["unjudged_share"] = (sum(scored[c["name"]]["unjudged"] for c, _ in members) / listed
                                       if listed else None)
                g["at"] = pooled_at([scored[c["name"]] for c, _ in members])
                if group in POSITIVE:
                    g.update(recall([scored[c["name"]] for c, _ in members]))
                    g["missed"] = [c["name"] for c, _ in members
                                   if scored[c["name"]].get("expected_found") is False]
                else:
                    tops = [strongest_outside(c, s, index) for c, s in members]
                    g["strongest_outside_median"] = median([t[0] for t in tops])
                    g["strongest_outside_max"] = max(t[0] for t in tops)
                    g["reaching_60s_outside"] = sum(t[1] for t in tops)
                    g["strongest_outside_95th"] = percentile([t[0] for t in tops], 0.95)
                    if old_index is not None:
                        old = [strongest_outside(c, s, old_index) for c, s in members]
                        g["old_clusters"] = {
                            "strongest_outside_95th": percentile([t[0] for t in old], 0.95),
                            "strongest_outside_median": median([t[0] for t in old]),
                            "reaching_60s_outside": sum(t[1] for t in old)}
                out["groups"][group] = g
            positives = [scored[c["name"]] for c, _ in rows if c["group"] in POSITIVE]
            out["positives"] = recall(positives)
            out["positives"]["at"] = pooled_at(positives)
            out["positives"]["recordings"] = len(
                {min(cluster_of(index, c["file"])) for c, _ in rows if c["group"] in POSITIVE})
            n1 = out["groups"].get("N1", {})
            out["null_T"] = n1.get("strongest_outside_95th")
            out["T_below_expected_median"] = (
                out["null_T"] is not None and out["positives"]["expected_hits_median"] is not None
                and out["null_T"] < out["positives"]["expected_hits_median"])
            summary[f"{ladder}/{mode}"] = out
    return summary


def pooled_at(scored):
    pooled = {}
    for k in KS:
        rel = sum(s["at"][k]["relevant"] for s in scored)
        unrel = sum(s["at"][k]["unrelated"] for s in scored)
        n = sum(s["at"][k]["listed"] for s in scored)
        pooled[k] = {"listed": n, "relevant": rel, "unrelated": unrel,
                     "precision_judged": rel / (rel + unrel) if rel + unrel else None,
                     "judged_share": (rel + unrel) / n if n else None,
                     "relevant_share": rel / n if n else None}
    return pooled


def recall(scored):
    with_expected = [s for s in scored if "expected_found" in s]
    found = [s for s in with_expected if s["expected_found"]]
    return {"scored": len(with_expected), "found": len(found),
            "in_20": sum(s["expected_in_20"] for s in with_expected),
            "first": sum(s["expected_rank_among_others"] == 1 for s in found),
            "first_5": sum(s["expected_rank_among_others"] <= 5 for s in found),
            "joined_to_query": sum(s["expected_joined_to_query"] for s in with_expected),
            "expected_hits_median": median([s["expected_hits"] for s in found])}


def main(argv):
    cases_path, clusters_path, out_path = argv[1:4]
    reports, same_titles, unrelated, old = {}, [], [], None
    rest = argv[4:]
    i = 0
    while i < len(rest):
        arg = rest[i]
        if arg == "--same-titles":
            same_titles = read_pairs(rest[i + 1], "same-title")
            i += 2
        elif arg == "--unrelated":
            unrelated = read_pairs(rest[i + 1])
            i += 2
        elif arg == "--old-clusters":
            old = cluster_index(json.load(open(rest[i + 1])))
            i += 2
        else:
            ladder, path = arg.split("=", 1)
            reports[ladder] = json.load(open(path))
            i += 1
    cases = json.load(open(cases_path))
    index = cluster_index(json.load(open(clusters_path)))
    summary = summarise(cases, reports, index, same_titles, unrelated, old)
    for key, out in summary.items():
        p = out["positives"]
        at = p["at"]
        print(f"{key}: expected {p['found']}/{p['scored']} found, {p['in_20']} in 20, "
              f"{p['first']} first, {p['first_5']} in 5 among others; "
              f"P@10 judged {at[10]['precision_judged']}, judged share {at[10]['judged_share']:.3f}, "
              f"relevant share {at[10]['relevant_share']:.3f}; T {out['null_T']} vs expected median "
              f"{p['expected_hits_median']}")
    json.dump(summary, open(out_path, "w"), indent=1)


if __name__ == "__main__":
    main(sys.argv)
