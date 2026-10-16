"""Session 23: NAS duplicate clusters rebuilt at a new store revision,
against the clusters in use before (step 2 of docs/brief-23.md).

- The corpus recordings: the partition of corpus files that the
  262-track clusters give must reappear in both NAS runs.
- Each new cluster against the old ones: members added and gone, with
  their reason; old clusters split, new clusters joining old ones.
- Members added are dated by their record (the time the store wrote it):
  `newer upload` between the old run and TONIGHT, `tonight` from TONIGHT
  on (the index run), `older` before the old run.
- The gap: the weakest same-recording pair and the strongest different
  pair that no verdict decides; borderline pairs (40-80% coverage, not
  joined), judged or not, new or seen in the old run.

usage: python3 -I scripts/analysis/clusters_rebuilt.py NEW OLD CLUSTERS_262
       STORE_RECORDS VERDICTS OLD_RUN TONIGHT OUT

OLD_RUN and TONIGHT are local times, YYYY-MM-DDTHH:MM:SS. STORE_RECORDS
is store_records.py's output for the store now.
"""

import collections
import csv
import datetime
import json
import sys

PREFIX = "second-library/"
BORDERLINE = 0.4


def load(path):
    with open(path, encoding="utf-8") as file:
        return json.load(file)


def index_of(clusters):
    out = {}
    for number, members in enumerate(clusters["duplicates"]):
        for member in members:
            out[member] = number
    return out


def corpus_partition(clusters):
    """The groups of corpus files that share a cluster (two or more)."""
    groups = set()
    for members in clusters["duplicates"]:
        corpus = frozenset(m for m in members if not m.startswith(PREFIX))
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
            out[frozenset((a, b))] = (verdict, number)
    return out


def main():
    new_path, old_path, c262_path, records_path, verdicts_path, old_run, tonight, out_path = sys.argv[1:]
    new, old, c262 = load(new_path), load(old_path), load(c262_path)
    old_run = datetime.datetime.fromisoformat(old_run).timestamp()
    tonight = datetime.datetime.fromisoformat(tonight).timestamp()
    written = {}
    with open(records_path, encoding="utf-8") as file:
        for row in csv.reader(file, delimiter="\t"):
            written[PREFIX + row[0]] = float(row[4])
    verdicts = verdicts_of(verdicts_path)
    new_index, old_index = index_of(new), index_of(old)

    def dated(path):
        if not path.startswith(PREFIX):
            return "corpus file"
        when = written.get(path)
        if when is None:
            return "no record now"
        if when >= tonight:
            return "tonight"
        if when > old_run:
            return "newer upload"
        return "older"

    def verdict_with(member, others, kind):
        for other in others:
            verdict = verdicts.get(frozenset((member, other)))
            if verdict and verdict[0] == kind:
                return f"line {verdict[1]}"
        return None

    corpus = {
        "262": sorted(sorted(g) for g in corpus_partition(c262)),
        "old equal to 262": corpus_partition(old) == corpus_partition(c262),
        "new equal to 262": corpus_partition(new) == corpus_partition(c262),
        "new only": sorted(sorted(g) for g in corpus_partition(new) - corpus_partition(c262)),
        "262 only": sorted(sorted(g) for g in corpus_partition(c262) - corpus_partition(new)),
    }

    added, gone, splits, joins = [], [], [], []
    for number, members in enumerate(new["duplicates"]):
        olds = sorted({old_index[m] for m in members if m in old_index})
        before = {m for o in olds for m in old["duplicates"][o]}
        for member in members:
            if member not in old_index:
                same = verdict_with(member, members, "same")
                added.append({"member": member, "cluster": number, "dated": dated(member),
                              "same_verdict": same, "with": sorted(before)[:1]})
        if len(olds) > 1:
            joins.append({"cluster": number, "old_clusters": olds, "members": sorted(members)})
    for number, members in enumerate(old["duplicates"]):
        news = sorted({new_index[m] for m in members if m in new_index})
        if len(news) > 1:
            splits.append({"old_cluster": number, "new_clusters": news,
                           "parts": [sorted(m for m in members if new_index.get(m) == n) for n in news],
                           "outside": sorted(m for m in members if m not in new_index)})
        for member in members:
            if member in new_index:
                continue
            if dated(member) == "no record now":
                why = "no record now"
            elif verdict_with(member, members, "different"):
                why = "judged different (" + verdict_with(member, members, "different") + ")"
            else:
                why = "other"
            gone.append({"member": member, "old_cluster": number, "why": why,
                         "old_members": len(members)})

    def gap(clusters):
        same = [p for p in clusters["pairs"] if p["same_recording"] and p.get("aural_verdict") is None]
        different = [p for p in clusters["pairs"] if not p["same_recording"] and p.get("aural_verdict") is None]
        weakest = min(same, key=lambda p: p["coverage"], default=None)
        strongest = max(different, key=lambda p: p["coverage"], default=None)
        return [{k: p[k] for k in ("query", "found", "coverage", "hits", "speed")} if p else None
                for p in (weakest, strongest)]

    def judged(index, pair):
        key = lambda f: index.get(f, f)
        a, b = key(pair["query"]), key(pair["found"])
        for files, (verdict, number) in verdicts.items():
            x, y = tuple(files)
            if {key(x), key(y)} == {a, b}:
                return f"{verdict} (line {number})"
        return None

    def borderline(clusters):
        return [p for p in clusters["pairs"]
                if BORDERLINE <= p["coverage"] < 0.8 and not p["same_recording"]]

    old_borderline = {frozenset((p["query"], p["found"])) for p in borderline(old)}
    border = [{
        "query": p["query"], "found": p["found"], "coverage": round(p["coverage"], 3),
        "hits": p["hits"], "speed": round(p["speed"], 4), "judged": judged(new_index, p),
        "new": frozenset((p["query"], p["found"])) not in old_borderline,
        "dated": [dated(p["query"]), dated(p["found"])],
    } for p in sorted(borderline(new), key=lambda p: -p["coverage"])]

    def files(clusters):
        members = {m for ms in clusters["duplicates"] for m in ms}
        return {"files": len(members), "nas": sum(m.startswith(PREFIX) for m in members)}

    verdict_pairs = collections.Counter(p.get("aural_verdict") for p in new["pairs"])
    result = {
        "clusters": {"old": len(old["duplicates"]), "new": len(new["duplicates"])},
        "files": {"old": files(old), "new": files(new)},
        "pairs": {"old": len(old["pairs"]), "new": len(new["pairs"])},
        "pairs_by_verdict_new": {str(k): v for k, v in verdict_pairs.items()},
        "corpus": corpus,
        "added": added,
        "added_by_date": collections.Counter(a["dated"] for a in added),
        "gone": gone,
        "splits": splits,
        "joins": joins,
        "new_clusters_without_old_members": [
            sorted(ms) for ms in new["duplicates"] if not any(m in old_index for m in ms)],
        "gap": {"old": gap(old), "new": gap(new)},
        "borderline": border,
    }
    with open(out_path, "w", encoding="utf-8") as file:
        json.dump(result, file, indent=1, ensure_ascii=False)

    print(f"clusters {result['clusters']}, files {result['files']}, pairs {result['pairs']}")
    print(f"corpus partition: 262 groups {len(corpus['262'])}; old equal {corpus['old equal to 262']}, "
          f"new equal {corpus['new equal to 262']}")
    for group in corpus["new only"]:
        print("  new only:", group)
    for group in corpus["262 only"]:
        print("  262 only:", group)
    print(f"added {len(added)}: {dict(result['added_by_date'])}")
    print(f"gone {len(gone)}: {dict(collections.Counter(g['why'].split(' (')[0] for g in gone))}")
    for g in gone:
        print(f"  gone: {g['member']} ({g['why']}, old cluster of {g['old_members']})")
    print(f"splits {len(splits)}, joins {len(joins)}, "
          f"new clusters without old members {len(result['new_clusters_without_old_members'])}")
    for s in splits:
        print("  split:", [len(p) for p in s["parts"]], [p[0][-70:] for p in s["parts"]])
    for j in joins:
        print("  join:", j["old_clusters"], [m[-70:] for m in j["members"][:4]])
    for name in ("old", "new"):
        weakest, strongest = result["gap"][name]
        print(f"gap {name}: weakest same {weakest and round(weakest['coverage'], 3)} "
              f"({weakest and weakest['found'][-60:]}), strongest different "
              f"{strongest and round(strongest['coverage'], 3)} ({strongest and strongest['found'][-60:]})")
    unjudged = [b for b in border if not b["judged"]]
    print(f"borderline {len(border)} (new {sum(b['new'] for b in border)}), unjudged {len(unjudged)}, "
          f"unjudged and new {sum(b['new'] for b in unjudged)}")
    for b in unjudged:
        print(f"  {b['coverage']:.3f} {b['hits']:>5} {'new' if b['new'] else 'old'} "
              f"{b['query'][-55:]} ~ {b['found'][-55:]}")


if __name__ == "__main__":
    main()
