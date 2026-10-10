"""What changed between two NAS-scale clusters runs, and why: each earlier
cluster with a corpus file against the cluster that holds that file now,
its members gained (judged the same by the owner, from folders indexed
since the earlier run, or other) and lost (no record now, judged
different, or other); clusters without a corpus file; the gap; the
borderline pairs and sparse joins the owner has not judged (a pair is
judged when a verdict names a file of each cluster); the joins
`--cut-sparsest` cut; and the clusters of a recall panel's development
sources.

usage: python3 -I scripts/analysis/clusters_change.py NEW OLD STORE_RECORDS
       VERDICTS PANEL SINCE OUT

STORE_RECORDS is store_records.py's output for the store now; SINCE is the
earliest date (YYYY-MM-DD) of records written after the earlier run.
"""

import collections
import csv
import datetime
import json
import sys

PREFIX = "second-library/"
BORDERLINE = 0.4
SPARSE_SHARE = 0.1


def load(path):
    with open(path) as file:
        return json.load(file)


def main():
    new_file, old_file, records_file, verdicts_file, panel_file, since, out = sys.argv[1:]
    new, old = load(new_file), load(old_file)
    written = {}
    with open(records_file) as file:
        for row in csv.reader(file, delimiter="\t"):
            written[PREFIX + row[0]] = datetime.date.fromtimestamp(float(row[4])).isoformat()
    verdicts = {}
    with open(verdicts_file) as file:
        for number, line in enumerate(file, 1):
            if line.startswith("#") or not line.strip():
                continue
            verdict, a, b = line.rstrip("\n").split("\t")
            verdicts[frozenset((a, b))] = (verdict, number)

    def has_record(path):
        return not path.startswith(PREFIX) or path in written

    def is_new(path):
        return path.startswith(PREFIX) and written.get(path, "") >= since

    def cluster_index(clusters):
        index = {}
        for number, members in enumerate(clusters["duplicates"]):
            for member in members:
                index[member] = number
        return index

    new_index, old_index = cluster_index(new), cluster_index(old)
    new_pairs = {frozenset((p["query"], p["found"])): p for p in new["pairs"]}

    def reason_gained(member, members):
        for other in members:
            verdict = verdicts.get(frozenset((member, other)))
            if verdict and verdict[0] == "same":
                return f"judged the same (line {verdict[1]})"
        if is_new(member):
            return "indexed since"
        return "other"

    def reason_lost(member):
        if not has_record(member):
            return "no record now (pruned)"
        if any(v[0] == "different" and member in pair for pair, v in verdicts.items()):
            return "judged different"
        return "other"

    changes, seen_new = [], set()
    for members in old["duplicates"]:
        corpus = [m for m in members if not m.startswith(PREFIX)]
        if not corpus:
            continue
        now_number = new_index.get(corpus[0])
        now = set(new["duplicates"][now_number]) if now_number is not None else {corpus[0]}
        if now_number is not None:
            seen_new.add(now_number)
        before = set(members)
        gained = sorted(now - before)
        lost = sorted(before - now)
        if gained or lost:
            changes.append({
                "corpus": corpus,
                "before": len(before),
                "now": len(now),
                "gained": [(m, reason_gained(m, now)) for m in gained],
                "lost": [(m, reason_lost(m)) for m in lost],
            })
    other_new = [m for n, m in enumerate(new["duplicates"]) if n not in seen_new]

    def gap(clusters):
        same = [p for p in clusters["pairs"] if p["same_recording"] and p.get("owner_verdict") is None]
        different = [p for p in clusters["pairs"]
                     if not p["same_recording"] and p.get("owner_verdict") is None]
        weakest = min(same, key=lambda p: p["coverage"], default=None)
        strongest = max(different, key=lambda p: p["coverage"], default=None)
        return weakest, strongest

    def judged(clusters, index, pair):
        key = lambda f: index.get(f, f)
        a, b = key(pair["query"]), key(pair["found"])
        for files in verdicts:
            x, y = tuple(files)
            if {key(x), key(y)} == {a, b}:
                return True
        return False

    def hits_per_second(pair):
        span = pair["query_end_seconds"] - pair["query_start_seconds"]
        return pair["hits"] / span if span > 0 else 0.0

    densities = sorted(hits_per_second(p) for p in new["pairs"] if p["same_recording"])
    median = densities[len(densities) // 2] if densities else 0.0
    cut = {frozenset((c["query"], c["found"])): c for c in new.get("cut_links", [])}
    borderline = [p for p in new["pairs"] if p["coverage"] >= BORDERLINE and not p["same_recording"]
                  and frozenset((p["query"], p["found"])) not in cut]
    old_borderline = {frozenset((p["query"], p["found"])) for p in old["pairs"]
                      if p["coverage"] >= BORDERLINE and not p["same_recording"]}
    sparse = [p for p in new["pairs"] if p["same_recording"] and p.get("owner_verdict") is None
              and hits_per_second(p) < SPARSE_SHARE * median]

    panel = load(panel_file)
    sources = []
    for source in panel["development"]["sources"]:
        number = new_index.get(source["asset"])
        members = new["duplicates"][number] if number is not None else [source["asset"]]
        sources.append({
            "asset": source["asset"],
            "kind": source["kind"],
            "held_out": source["held_out"],
            "cluster_size": len(members),
            "with_corpus_files": [m for m in members if not m.startswith(PREFIX)],
        })

    def files(clusters):
        members = {m for ms in clusters["duplicates"] for m in ms}
        return len(members), sum(1 for m in members if m.startswith(PREFIX))

    result = {
        "clusters": {"old": len(old["duplicates"]), "new": len(new["duplicates"])},
        "files_in_clusters": {"old": files(old), "new": files(new)},
        "pairs": {"old": len(old["pairs"]), "new": len(new["pairs"])},
        "gap": {name: [round(p["coverage"], 3) if p else None for p in gap(c)]
                for name, c in (("old", old), ("new", new))},
        "gap_pairs_new": [(p["query"], p["found"], round(p["coverage"], 3), p["hits"])
                          for p in gap(new) if p],
        "changes": changes,
        "clusters_without_corpus_files": other_new,
        "median_join_hits_per_second": round(median, 1),
        "borderline": [{
            "query": p["query"], "found": p["found"], "coverage": round(p["coverage"], 3),
            "hits": p["hits"], "judged": judged(new, new_index, p),
            "new": frozenset((p["query"], p["found"])) not in old_borderline,
        } for p in sorted(borderline, key=lambda p: -p["coverage"])],
        "cut_links": [{
            "query": p["query"], "found": p["found"], "coverage": round(p["coverage"], 3),
            "hits": p["hits"], "hits_per_second": round(hits_per_second(p), 2),
            "judged_different": cut[frozenset((p["query"], p["found"]))]["judged_different"],
        } for p in new["pairs"] if frozenset((p["query"], p["found"])) in cut],
        "sparse_joins": [{
            "query": p["query"], "found": p["found"], "coverage": round(p["coverage"], 3),
            "hits": p["hits"], "hits_per_second": round(hits_per_second(p), 2),
        } for p in sparse],
        "panel_sources": sources,
    }
    with open(out, "w") as file:
        json.dump(result, file, indent=1)

    print(f"clusters {result['clusters']}, files (all, NAS) {result['files_in_clusters']}, "
          f"pairs {result['pairs']}")
    print(f"gap (weakest same, strongest different, unjudged): {result['gap']}")
    reasons = collections.Counter()
    for change in changes:
        for _, why in change["gained"]:
            reasons["gained: " + why.split(" (")[0]] += 1
        for _, why in change["lost"]:
            reasons["lost: " + why.split(" (")[0]] += 1
    print(f"{len(changes)} corpus clusters changed: {dict(reasons)}")
    print(f"{len(other_new)} clusters without corpus files; {len(result['cut_links'])} joins cut")
    unjudged = [b for b in result["borderline"] if not b["judged"]]
    print(f"borderline {len(borderline)}, unjudged {len(unjudged)}, "
          f"new {sum(b['new'] for b in result['borderline'])}; sparse joins {len(sparse)} "
          f"(median {median:.1f} hits/s)")
    joined = [s for s in sources if s["cluster_size"] > 1]
    print(f"panel sources in a cluster: {len(joined)} of {len(sources)}; "
          f"with corpus files: {sum(1 for s in sources if s['with_corpus_files'])}")


if __name__ == "__main__":
    main()
