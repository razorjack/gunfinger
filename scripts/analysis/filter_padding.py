"""Session 50: the fullest-list filter under exact copies (docs/brief-50.md).

  copied panel PANEL CLUSTERS OUT
      every file of the clusters of the panel's indexed sources, one per line
  copied share FULLEST SEED SHARE OUT
      a seeded share of the files of a `fullest` report (or a traced sweep
      report's census), one per line
  compare BASE PADDED [COPIED]
      a traced sweep report against the traced level-0 report: the lists set
      aside, and per query the candidates and answers (JSON to stdout)
"""

import json
import random
import statistics
import sys

COPY_PREFIX = "padding-copy/"
POSSIBLE_HITS = 60


def original(path):
    if path.startswith(COPY_PREFIX):
        return path[len(COPY_PREFIX):].split("/", 1)[1]
    return path


def panel_files(panel, duplicates):
    files = set()
    for draw in panel["draws"]:
        if draw["held_out"]:
            continue
        files.add(draw["asset"])
        for members in duplicates:
            if draw["asset"] in members:
                files.update(members)
    return sorted(files)


def share_files(paths, seed, share):
    paths = sorted(paths)
    return sorted(random.Random(seed).sample(paths, round(share * len(paths))))


def census_of(report):
    return report["set_aside"]["census"] if "set_aside" in report else report


def key(query):
    return (query["asset"], query["start_seconds"], query["speed_percent"])


def own(query):
    """The query's own-cluster evidence: first-pass candidate hits (None when
    no candidate) and whether an own-cluster answer is confident."""
    hits = [c["hits"] for c in query["candidates"] if c["correct"]]
    answers = [d["hits"] for d in query["detections"] if d["correct"] and d["confident"]]
    return {
        "candidate": max(hits) if hits else None,
        "confident": bool(answers),
        "answer_hits": max(answers) if answers else None,
        "candidates": len(query["candidates"]),
        "wrong_confident": sum(1 for d in query["detections"] if d["confident"] and not d["correct"]),
        "wrong_possible": sum(
            1 for d in query["detections"]
            if not d["correct"] and not d["confident"] and d["hits"] >= POSSIBLE_HITS
        ),
        # Copies of one file count once.
        "wrong_candidate_files": len({original(c["asset"]) for c in query["candidates"] if not c["correct"]}),
        "wrong_possible_files": len({
            original(d["asset"]) for d in query["detections"]
            if not d["correct"] and not d["confident"] and d["hits"] >= POSSIBLE_HITS
        }),
    }


def lists_moved(base, padded):
    before, after = set(base["set_aside"]["hashes"]), set(padded["set_aside"]["hashes"])
    return {"in": len(after - before), "out": len(before - after), "kept": len(before & after)}


def shares(census, copied):
    """Postings set aside for copies, their originals and the other files."""
    groups = {"copies": [0, 0], "originals": [0, 0], "rest": [0, 0]}
    for asset in census["assets"]:
        path = asset["path"]
        group = "copies" if path.startswith(COPY_PREFIX) else "originals" if path in copied else "rest"
        groups[group][0] += asset["skipped"]
        groups[group][1] += asset["postings"]
    return {name: round(skipped / postings, 4) if postings else None
            for name, (skipped, postings) in groups.items()}


def compare(base, padded, copied=frozenset()):
    base_queries = {key(q): q for q in base["queries"]}
    rows = []
    for query in padded["queries"]:
        before, after = own(base_queries[key(query)]), own(query)
        rows.append({
            "query": list(key(query)),
            "held_out": query["held_out"],
            "copied": query["asset"] in copied,
            "before": before,
            "after": after,
        })
    indexed = [r for r in rows if not r["held_out"]]
    held_out = [r for r in rows if r["held_out"]]
    ratios = [r["after"]["candidate"] / r["before"]["candidate"]
              for r in indexed if r["before"]["candidate"] and r["after"]["candidate"]]
    lowest = sorted(indexed, key=lambda r: (r["after"]["candidate"] or 0) / (r["before"]["candidate"] or 1))
    census = census_of(padded)
    return {
        "cut": census["shortest_skipped"],
        "lists_set_aside": census["skipped_lists"],
        "postings_set_aside": round(census["skipped_postings"] / census["postings"], 4),
        "assets": len(census["assets"]),
        "moved": lists_moved(base, padded),
        "set_aside_share": shares(census, copied),
        "u1_lost_confident": [r["query"] for r in indexed if r["before"]["confident"] and not r["after"]["confident"]],
        "u2_lost_candidate": [r["query"] for r in indexed
                              if r["before"]["candidate"] is not None and r["after"]["candidate"] is None],
        "u3_wrong_confident": sum(r["after"]["wrong_confident"] for r in rows),
        "u3_wrong_confident_before": sum(r["before"]["wrong_confident"] for r in rows),
        "u3_held_out_confident": sum(1 for r in held_out if r["after"]["wrong_confident"]),
        "confident": sum(1 for r in indexed if r["after"]["confident"]),
        "confident_before": sum(1 for r in indexed if r["before"]["confident"]),
        "indexed": len(indexed),
        "ratio_min": round(min(ratios), 3) if ratios else None,
        "ratio_median": round(statistics.median(ratios), 3) if ratios else None,
        "ratio_median_copied": median([r for r in indexed if r["copied"]]),
        "ratio_median_not_copied": median([r for r in indexed if not r["copied"]]),
        "ratio_below_half": sum(1 for x in ratios if x < 0.5),
        "lowest": [{"query": r["query"], "before": r["before"]["candidate"], "after": r["after"]["candidate"]}
                   for r in lowest[:5]],
        "answer_hits_median_ratio": round(statistics.median(
            r["after"]["answer_hits"] / r["before"]["answer_hits"]
            for r in indexed if r["after"]["answer_hits"] and r["before"]["answer_hits"]), 4),
        "candidates_median": statistics.median(r["after"]["candidates"] for r in rows),
        "candidates_median_before": statistics.median(r["before"]["candidates"] for r in rows),
        "wrong_candidate_files_median": statistics.median(r["after"]["wrong_candidate_files"] for r in rows),
        "wrong_candidate_files_median_before": statistics.median(
            r["before"]["wrong_candidate_files"] for r in rows),
        "wrong_possible_files": sum(r["after"]["wrong_possible_files"] for r in rows),
        "wrong_possible_files_before": sum(r["before"]["wrong_possible_files"] for r in rows),
        "wrong_possible": sum(r["after"]["wrong_possible"] for r in rows),
        "wrong_possible_before": sum(r["before"]["wrong_possible"] for r in rows),
    }


def median(rows):
    ratios = [r["after"]["candidate"] / r["before"]["candidate"]
              for r in rows if r["before"]["candidate"] and r["after"]["candidate"]]
    return round(statistics.median(ratios), 3) if ratios else None


def load(path):
    with open(path) as file:
        return json.load(file)


def write_lines(path, lines):
    with open(path, "w") as file:
        file.writelines(line + "\n" for line in lines)


def main(args):
    if args[:2] == ["copied", "panel"]:
        files = panel_files(load(args[2]), load(args[3])["duplicates"])
        write_lines(args[4], files)
        print(len(files), "files")
    elif args[:2] == ["copied", "share"]:
        paths = [asset["path"] for asset in census_of(load(args[2]))["assets"]]
        files = share_files(paths, int(args[3]), float(args[4]))
        write_lines(args[5], files)
        print(len(files), "of", len(paths), "files")
    elif args[0] == "compare":
        copied = set()
        if len(args) > 3:
            with open(args[3]) as file:
                copied = {line.strip() for line in file if line.strip()}
        print(json.dumps(compare(load(args[1]), load(args[2]), copied), indent=1))
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main(sys.argv[1:])
