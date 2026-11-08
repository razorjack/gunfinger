"""Compares library-wide clusters with the clusters in use (brief 86's
registration, R1; experiment 0295).

    python3 -I scripts/analysis/library_clusters.py NEW.json OLD.json MAP.json OUT.json

(a) the old same-recording joins and old clusters the new clusters keep
together (a corpus file's copy in the other library, which the index stands
for by the corpus file, is named by its corpus file); (b) new clusters joining
two or more old clusters; (c) new clusters without a corpus file: count,
files, sizes, the largest; (d) sparse joins (under a tenth of the median hits
per second, verdicts by ear not counted) and borderline pairs (40-80%
coverage, not joined); (e) cut links. Writes the summary as JSON."""

import json
import statistics
import sys

PREFIX = "second-library/"
SPARSE_SHARE = 0.1
BORDERLINE = 0.4


def renamed(map_file):
    """Each copy's name in the other library -> the corpus file it copies."""
    return {PREFIX + c["stands_for"]: c["corpus"] for c in map_file.get("copies", [])}


def index_of(duplicates, rename=None):
    rename = rename or {}
    index = {}
    for number, members in enumerate(duplicates):
        for member in members:
            index[rename.get(member, member)] = number
    return index


def hits_per_second(pair):
    span = pair["query_end_seconds"] - pair["query_start_seconds"]
    return pair["hits"] / span if span > 0 else 0.0


def summarise(new, old, map_file):
    rename = renamed(map_file)
    new_index = index_of(new["duplicates"])
    is_corpus = lambda f: not f.startswith(PREFIX)
    old_joins = {(rename.get(p["query"], p["query"]), rename.get(p["found"], p["found"]))
                 for p in old["pairs"] if p["same_recording"]}
    old_joins = {tuple(sorted(j)) for j in old_joins if j[0] != j[1]}
    kept = [j for j in old_joins if j[0] in new_index and new_index.get(j[0]) == new_index.get(j[1])]
    missed = sorted(set(old_joins) - set(kept))
    old_whole = []
    for members in old["duplicates"]:
        names = {rename.get(m, m) for m in members}
        ids = {new_index.get(m) for m in names}
        old_whole.append(len(ids) == 1 and None not in ids)
    old_index = index_of(old["duplicates"], rename)
    merging = []
    for members in new["duplicates"]:
        olds = {old_index[m] for m in members if m in old_index}
        if len(olds) > 1:
            merging.append({"members": len(members), "old_clusters": len(olds),
                            "corpus": sorted(m for m in members if is_corpus(m))[:6]})
    without_corpus = [m for m in new["duplicates"] if not any(is_corpus(x) for x in m)]
    sizes = sorted((len(m) for m in new["duplicates"]), reverse=True)
    joins = [p for p in new["pairs"] if p["same_recording"]]
    densities = sorted(hits_per_second(p) for p in joins)
    med = densities[len(densities) // 2] if densities else 0.0
    sparse = [p for p in joins if p.get("aural_verdict") is None and hits_per_second(p) < SPARSE_SHARE * med]
    # Each pair is usually measured from both files; count unordered joins.
    unordered = lambda ps: {tuple(sorted((p["query"], p["found"]))) for p in ps}
    borderline = [p for p in new["pairs"] if BORDERLINE <= p["coverage"] and not p["same_recording"]]
    largest = [{"size": len(m), "corpus": sorted(x for x in m if is_corpus(x))[:3], "first": m[:3]}
               for m in sorted(new["duplicates"], key=len, reverse=True)[:8]]
    return {
        "new": {"clusters": len(new["duplicates"]), "files": sum(sizes),
                "with_corpus_file": len(new["duplicates"]) - len(without_corpus),
                "sizes_top": sizes[:15], "size_median": statistics.median(sizes) if sizes else None,
                "size_2": sum(s == 2 for s in sizes), "size_3_5": sum(3 <= s <= 5 for s in sizes),
                "size_6_20": sum(6 <= s <= 20 for s in sizes), "size_over_20": sum(s > 20 for s in sizes),
                "largest": largest},
        "old": {"clusters": len(old["duplicates"]), "files": sum(len(m) for m in old["duplicates"]),
                "joins": len(old_joins)},
        "a_old_joins_kept": len(kept), "a_old_joins_share": len(kept) / len(old_joins) if old_joins else None,
        "a_old_joins_missed": [list(j) for j in missed],
        "a_old_clusters_whole": sum(old_whole),
        "b_merging": merging,
        "c_without_corpus": {"clusters": len(without_corpus), "files": sum(len(m) for m in without_corpus)},
        "d_joins": len(unordered(joins)), "d_median_hits_per_second": med,
        "d_sparse": len(unordered(sparse)),
        "d_sparse_share": len(unordered(sparse)) / len(unordered(joins)) if joins else None,
        "d_sparse_list": sorted(([p["query"], p["found"], round(p["coverage"], 3), p["hits"],
                                  round(hits_per_second(p), 2)] for p in sparse), key=lambda r: r[4])[:40],
        "d_borderline": len(unordered(borderline)),
        "d_borderline_list": sorted(([p["query"], p["found"], round(p["coverage"], 3), p["hits"]]
                                     for p in borderline), key=lambda r: -r[2])[:40],
        "e_cut_links": new.get("cut_links", []),
    }


def main(argv):
    new, old, map_file = (json.load(open(p)) for p in argv[1:4])
    summary = summarise(new, old, map_file)
    small = {k: v for k, v in summary.items() if not k.endswith("_list") and k not in ("a_old_joins_missed", "b_merging")}
    print(json.dumps(small, indent=1))
    print("merging:", len(summary["b_merging"]), "missed old joins:", len(summary["a_old_joins_missed"]))
    json.dump(summary, open(argv[4], "w"), indent=1, ensure_ascii=False)


if __name__ == "__main__":
    main(sys.argv)
