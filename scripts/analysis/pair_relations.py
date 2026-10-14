"""Summarise a `gunfinger-eval pair` report: for each pair, the strongest
alignment over every search (peaks and audio, each ladder, the fitted
speed) and what it says about the two files.

- same recording: one alignment covers at least 80% of the shorter file
  (the clustering rule, `clusters::MIN_COVERAGE`);
- shared material: an alignment of 30 hits or more (`RELATED_HITS`, the
  census of related recordings, experiment 0015);
- nothing shared found: every alignment below 30 hits. Shared material
  can still hide there: by ear, Fractles and We Enter share
  material at 20 hits (2026-10-14).

usage: pair_relations.py <pairs report> [<kinds.tsv>]
The kinds file, if given, has one line per pair in the same order:
position, kind, first path, second path.
"""
import json
import sys

SAME_COVERAGE = 0.8
RELATED_HITS = 30


def strongest(pair):
    best = None
    for search in pair["searches"]:
        for alignment in search["alignments"]:
            key = (alignment["hits"], alignment["coverage"])
            if best is None or key > best[0]:
                best = (key, search["name"], alignment)
    return best


def widest(pair):
    return max(
        (a["coverage"] for s in pair["searches"] for a in s["alignments"]),
        default=0.0,
    )


def relation(pair):
    best = strongest(pair)
    if widest(pair) >= SAME_COVERAGE:
        return "same recording"
    if best and best[2]["hits"] >= RELATED_HITS:
        return "shared material"
    return "nothing shared found"


def short(path):
    return path.split("/")[-1]


def main():
    report = json.load(open(sys.argv[1]))
    kinds = []
    if len(sys.argv) > 2:
        kinds = [line.rstrip("\n").split("\t") for line in open(sys.argv[2])]
    for number, pair in enumerate(report):
        best = strongest(pair)
        label = " ".join(kinds[number][:2]) if kinds else str(number + 1)
        print(f"{label}: {short(pair['query'])} ~ {short(pair['found'])}")
        if best is None:
            print("    no alignment; nothing shared found")
            continue
        _, name, a = best
        lock = " key-locked" if a["key_locked"] else ""
        print(
            f"    {relation(pair)}: strongest {a['hits']} hits in {a['windows']} windows"
            f" ({name}), query {a['query_start_seconds']:.0f}-{a['query_end_seconds']:.0f} s,"
            f" found {a['found_start_seconds']:.0f}-{a['found_end_seconds']:.0f} s,"
            f" speed {a['speed']:.4f}{lock}; widest coverage {widest(pair):.0%}"
        )


main()
