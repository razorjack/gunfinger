"""Session 33: the Linux machine's NAS duplicate clusters against the Mac's
(experiment 0128, and the members 0120 and 0128 list as added).

0128's data file holds counts, the corpus groups, the members added since
0120, the gap, the borderline rows and the cut joins, not every cluster's
members. So the comparison is:

- counts: clusters, files, NAS files, pairs, pairs by verdict;
- the corpus partition (groups of corpus files sharing a cluster) against
  0128's 262-track groups;
- every member 0120 and 0128 list as added: is it here, in the cluster of
  the corpus file it joined there?
- the cut joins, the gap and the borderline rows (40-80% coverage, not
  joined), matched by their two files (a pair searched from both sides
  gives two rows and one pair);
- for this machine's members and pairs, whether a file's record is one of
  the 557 the Mac's store wrote after 0128's run (2026-10-17), from the
  Mac's record listing: a difference from 0128 that involves such a file
  can come from the larger store, one that does not can only come from
  the peaks (or from the queries).

Paths are compared after Unicode NFC normalization: the Mac's listings
hold 160 NAS paths decomposed (experiment 0153).

usage: python3 -I scripts/analysis/clusters_linux.py NEW DATA_0128 DATA_0120
       MAC_NAS_RECORDS VERDICTS OUT
"""

import collections
import csv
import datetime
import json
import sys
import unicodedata

PREFIX = "second-library/"
BORDERLINE = 0.4
NEW_RECORDS_FROM = datetime.datetime(2026, 10, 17).timestamp()


def nfc(text):
    return unicodedata.normalize("NFC", text)


def load(path):
    with open(path, encoding="utf-8") as file:
        return json.load(file)


def index_of(clusters):
    out = {}
    for number, members in enumerate(clusters["duplicates"]):
        for member in members:
            out[nfc(member)] = number
    return out


def corpus_partition(duplicates):
    groups = set()
    for members in duplicates:
        corpus = frozenset(nfc(m) for m in members if not m.startswith(PREFIX))
        if len(corpus) > 1:
            groups.add(corpus)
    return groups


def verdicts_of(path):
    out = {}
    with open(path, encoding="utf-8") as file:
        for number, line in enumerate(file, 1):
            if line.startswith("#") or not line.strip():
                continue
            verdict, a, b = line.rstrip("\n").split("\t")
            out[frozenset((nfc(a), nfc(b)))] = (verdict, number)
    return out


def names_only(value):
    """File names without their folders (AGENTS.md: evaluation JSON in
    docs holds file names only)."""
    if isinstance(value, dict):
        return {k: names_only(v) for k, v in value.items()}
    if isinstance(value, list):
        return [names_only(v) for v in value]
    if isinstance(value, str) and "/" in value and not value.startswith("line "):
        return value.rsplit("/", 1)[-1]
    return value


def key(pair):
    return frozenset((nfc(pair["query"]), nfc(pair["found"])))


def main():
    new_path, data_0128, data_0120, mac_records, verdicts_path, out_path = sys.argv[1:]
    new, mac, mac_0120 = load(new_path), load(data_0128), load(data_0120)
    verdicts = verdicts_of(verdicts_path)
    index = index_of(new)

    newer = set()
    with open(mac_records, encoding="utf-8") as file:
        for row in csv.reader(file, delimiter="\t"):
            if float(row[4]) >= NEW_RECORDS_FROM:
                newer.add(PREFIX + nfc(row[0]))

    def is_newer(path):
        return nfc(path) in newer

    members = {nfc(m) for ms in new["duplicates"] for m in ms}
    counts = {
        "clusters": [mac["clusters"]["new"], len(new["duplicates"])],
        "files": [mac["files"]["new"]["files"], len(members)],
        "nas_files": [mac["files"]["new"]["nas"], sum(m.startswith(PREFIX) for m in members)],
        "pairs": [mac["pairs"]["new"], len(new["pairs"])],
        "pairs_by_verdict": [mac["pairs_by_verdict_new"],
                             {str(k): v for k, v in collections.Counter(
                                 p.get("aural_verdict") for p in new["pairs"]).items()}],
        "nas_members_newer": sum(1 for m in members if is_newer(m)),
    }

    mac_groups = {frozenset(nfc(m) for m in g) for g in mac["corpus"]["262"]}
    here_groups = corpus_partition(new["duplicates"])
    corpus = {
        "mac_groups": len(mac_groups),
        "here_groups": len(here_groups),
        "equal": mac_groups == here_groups,
        "here_only": sorted(sorted(g) for g in here_groups - mac_groups),
        "mac_only": sorted(sorted(g) for g in mac_groups - here_groups),
    }

    listed = []
    for source, rows in (("0120", mac_0120["added"]), ("0128", mac["added"])):
        for row in rows:
            member = nfc(row["member"])
            with_file = nfc(row["with"][0]) if row["with"] else None
            here = index.get(member)
            listed.append({
                "listed_in": source, "member": member, "with": with_file,
                "here": "absent" if here is None else (
                    "with the same file" if with_file is None or index.get(with_file) == here
                    else "in another cluster"),
                "newer_record": is_newer(member),
            })

    cut_here = {frozenset((nfc(c["query"]), nfc(c["found"]))) for c in new.get("cut_links", [])}
    cut_mac = {frozenset((nfc(c["query"]), nfc(c["found"]))) for c in mac["cut_links"]}
    cuts = {
        "mac": len(cut_mac), "here": len(cut_here), "both": len(cut_mac & cut_here),
        "here_only": [sorted(c) for c in cut_here - cut_mac],
        "mac_only": [sorted(c) for c in cut_mac - cut_here],
        "here_rows": new.get("cut_links", []),
    }

    def gap(clusters):
        cluster = index_of(clusters)
        apart = lambda p: cluster.get(nfc(p["query"])) is None or \
            cluster[nfc(p["query"])] != cluster.get(nfc(p["found"]))
        same = [p for p in clusters["pairs"] if p["same_recording"] and p.get("aural_verdict") is None]
        cut = {frozenset((nfc(c["query"]), nfc(c["found"]))) for c in clusters.get("cut_links", [])}
        different = [p for p in clusters["pairs"]
                     if not p["same_recording"] and p.get("aural_verdict") is None and apart(p)
                     and key(p) not in cut]
        weakest = min(same, key=lambda p: p["coverage"], default=None)
        strongest = max(different, key=lambda p: p["coverage"], default=None)
        return [{k: p[k] for k in ("query", "found", "coverage", "hits", "speed")} if p else None
                for p in (weakest, strongest)]

    def judged(pair):
        k = lambda f: index.get(nfc(f), nfc(f))
        a, b = k(pair["query"]), k(pair["found"])
        for files, (verdict, number) in verdicts.items():
            x, y = tuple(files)
            if {k(x), k(y)} == {a, b}:
                return f"{verdict} (line {number})"
        return None

    here_border = {key(p): p for p in new["pairs"]
                   if BORDERLINE <= p["coverage"] < 0.8 and not p["same_recording"]}
    mac_border = {key(p): p for p in mac["borderline"]}
    here_pairs = {}
    for p in new["pairs"]:
        here_pairs.setdefault(key(p), []).append(p)
    rows = []
    for k in sorted(set(here_border) | set(mac_border), key=lambda k: sorted(k)):
        h, m = here_border.get(k), mac_border.get(k)
        other = None
        if h is None:
            found = here_pairs.get(k, [])
            other = [{"coverage": round(p["coverage"], 3), "hits": p["hits"],
                      "same_recording": p["same_recording"]} for p in found] or "no pair"
        rows.append({
            "files": sorted(k),
            "mac": None if m is None else {"coverage": m["coverage"], "hits": m["hits"],
                                           "judged": m["judged"]},
            "here": None if h is None else {"coverage": round(h["coverage"], 3), "hits": h["hits"],
                                            "judged": judged(h)},
            "here_otherwise": other,
            "newer_record": any(is_newer(f) for f in k),
        })

    result = {
        "counts": counts,
        "corpus": corpus,
        "listed_added": listed,
        "listed_added_summary": collections.Counter(r["here"] for r in listed),
        "cut_links": cuts,
        "gap": {"mac": mac["gap"]["new"], "here": gap(new)},
        "borderline": rows,
        "borderline_summary": {
            "mac_rows": len(mac["borderline"]),
            "here_rows": sum(1 for p in new["pairs"]
                             if BORDERLINE <= p["coverage"] < 0.8 and not p["same_recording"]),
            "mac": len(mac_border), "here": len(here_border),
            "both": len(set(mac_border) & set(here_border)),
            "here_only": sum(1 for r in rows if r["mac"] is None),
            "here_only_newer_record": sum(1 for r in rows if r["mac"] is None and r["newer_record"]),
            "here_only_unjudged": sum(1 for r in rows if r["mac"] is None and not r["here"]["judged"]),
            "mac_only": sum(1 for r in rows if r["here"] is None),
        },
    }
    with open(out_path, "w", encoding="utf-8") as file:
        json.dump(names_only(result), file, indent=1, ensure_ascii=False)

    print("counts (Mac, here):", json.dumps(counts, ensure_ascii=False))
    print("corpus groups:", {k: corpus[k] for k in ("mac_groups", "here_groups", "equal")})
    for g in corpus["here_only"]:
        print("  here only:", g)
    for g in corpus["mac_only"]:
        print("  Mac only:", g)
    print("members 0120 and 0128 list as added:", dict(result["listed_added_summary"]))
    for r in listed:
        if r["here"] != "with the same file":
            print("  ", r["here"], r["listed_in"], r["member"])
    print("cut joins:", {k: cuts[k] for k in ("mac", "here", "both")})
    for c in cuts["here_only"]:
        print("  here only:", c)
    for c in cuts["mac_only"]:
        print("  Mac only:", c)
    print("gap (weakest same, strongest unjudged apart):")
    for side in ("mac", "here"):
        for p in result["gap"][side]:
            print(f"  {side}: {p['coverage']:.4f} {p['hits']} {p['query'].rsplit('/', 1)[-1]} ~ "
                  f"{p['found'].rsplit('/', 1)[-1]}" if p else f"  {side}: none")
    print("borderline:", result["borderline_summary"])
    for r in rows:
        if r["mac"] is None or r["here"] is None:
            names = " ~ ".join(f.rsplit("/", 1)[-1] for f in r["files"])
            print(f"  {'here only' if r['mac'] is None else 'Mac only'}"
                  f"{' (newer record)' if r['newer_record'] else ''}: {names}; "
                  f"Mac {r['mac']}; here {r['here'] or r['here_otherwise']}")


main()
