"""Session 51 (roadmap, "The edge of the ladder"): recall and wrong answers
by relative speed for each ladder, from `sweep` reports (`--speeds edge`,
`--other-rips`, `--extra-rungs N`).

A query's relative speed is the rung that would meet the nearest copy of
its recording left in the index: an excerpt played at s finds a copy whose
time is k times the source's at k(1 + s), so the relative speed is
k(1 + s) - 1, nearest to 0 over the copies left in. The copies' times come
from the clusters' pairs, chained through the cluster (as
`relative_speed.py` does). With `--other-rips` the source and its
identical copies (library map) are not in the index; otherwise every
member of an indexed recording is.

Recall counts own-cluster detections: confident, or possible or better
(60 hits or more, the possible tier). A wrong answer is any other
detection: confident, or possible (60 hits or more). The targeted estimate
takes a query's default answers when the default finds anything
confident, otherwise the wider ladder's (brief 51's T_k).

usage:
  ladder_edge.py recall OUT.json CLUSTERS[,CLUSTERS...] MAP|- PANELS LABEL=REPORT[+REPORT...]...
  ladder_edge.py uncovered SCAN.json
"""

import collections
import json
import os
import sys

PREFIX = "second-library/"
POSSIBLE_HITS = 60
BINS = [(0, 8), (8, 8.4), (8.4, 8.8), (8.8, 9.2), (9.2, 9.6), (9.6, 10.0), (10.0, 10.4),
        (10.4, 10.8), (10.8, 11.2), (11.2, 12.4), (12.4, 100)]


def joins(pairs):
    """Each file's joins: the found file's time over the query's."""
    edges = collections.defaultdict(dict)
    for p in pairs:
        if p["same_recording"]:
            edges[p["query"]][p["found"]] = p["speed"]
            edges[p["found"]][p["query"]] = 1.0 / p["speed"]
    return edges


def times_from(source, edges):
    """Each file's time over the source's, through the chain of joins."""
    times, queue = {source: 1.0}, [source]
    while queue:
        here = queue.pop()
        for there, k in sorted(edges[here].items()):
            if there not in times:
                times[there] = times[here] * k
                queue.append(there)
    return times


def identical(asset, library_map):
    """The asset and the files whose records are identical to it."""
    found = {asset}
    copies = library_map["copies"] if library_map else []
    own = next((c for c in copies if c["corpus"] == asset), None)
    if own:
        for c in copies:
            if c["stands_for"] == own["stands_for"]:
                found.add(c["corpus"])
                found |= {PREFIX + path for path in [c["stands_for"], *c.get("also", [])]}
    return found


def left_out_by_other_rips(draws, cluster_of, library_map):
    """What `sweep --other-rips` leaves out besides the held-out
    recordings: each indexed draw's source and identical copies, when its
    recording keeps another copy."""
    out = set()
    for d in draws:
        if d["held_out"]:
            continue
        same = identical(d["asset"], library_map)
        if cluster_of.get(d["asset"], {d["asset"]}) - same:
            out |= same
    return out


def relative_percent(asset, speed_percent, cluster, left_out, edges):
    """The relative speed to the nearest copy left in, or None."""
    times = times_from(asset, edges)
    times.setdefault(asset, 1.0)
    relative = [times[m] * (1 + speed_percent / 100) - 1
                for m in cluster if m not in left_out and m in times]
    # Rounded so that a speed played on a bin's edge falls in its bin.
    return round(100 * min(relative, key=abs), 6) if relative else None


def query_rows(report, cluster_of, left_out, edges):
    rows = []
    for q in report["queries"]:
        own = [d for d in q["detections"] if d["correct"]]
        wrong = [d for d in q["detections"] if not d["correct"]]
        cluster = cluster_of.get(q["asset"], {q["asset"]})
        rows.append({
            "asset": q["asset"], "speed_percent": q["speed_percent"], "held_out": q["held_out"],
            "relative_percent": None if q["held_out"] else
            relative_percent(q["asset"], q["speed_percent"], cluster, left_out, edges),
            "confident": any(d["confident"] for d in own),
            "possible": any(d["confident"] or d["hits"] >= POSSIBLE_HITS for d in own),
            "best_hits": max((d["hits"] for d in own), default=0),
            "any_confident": any(d["confident"] for d in q["detections"]),
            "wrong_confident": sorted(d["asset"] for d in wrong if d["confident"]),
            "wrong_possible": sorted(d["asset"] for d in wrong
                                     if not d["confident"] and d["hits"] >= POSSIBLE_HITS),
            "strongest_wrong": max((d["hits"] for d in wrong), default=0),
        })
    return rows


def key(row):
    return (row["asset"], row["speed_percent"])


def targeted(default_rows, wider_rows):
    """Brief 51's T_k: the default's answers where it finds anything
    confident, otherwise the wider ladder's."""
    wider = {key(r): r for r in wider_rows}
    return [r if r["any_confident"] else wider[key(r)] for r in default_rows]


def in_bin(value, low, high):
    return value is not None and low <= abs(value) < high


def by_bin(rows, field):
    table = []
    for low, high in BINS:
        cell = [r for r in rows if not r["held_out"] and in_bin(r[field], low, high)]
        table.append({"bin": f"{low}-{high}", "queries": len(cell),
                      "confident": sum(r["confident"] for r in cell),
                      "possible": sum(r["possible"] for r in cell)})
    return table


def by_speed(rows):
    table = []
    for speed in sorted({r["speed_percent"] for r in rows}):
        cell = [r for r in rows if r["speed_percent"] == speed]
        indexed = [r for r in cell if not r["held_out"]]
        table.append({"speed_percent": speed, "queries": len(indexed),
                      "confident": sum(r["confident"] for r in indexed),
                      "possible": sum(r["possible"] for r in indexed),
                      "wrong_confident": sum(len(r["wrong_confident"]) for r in cell),
                      "wrong_possible": sum(len(r["wrong_possible"]) for r in cell)})
    return table


def wrong_answers(rows, field):
    return {(r["asset"], r["speed_percent"], found) for r in rows for found in r[field]}


def summary(rows, default_rows):
    new = wrong_answers(rows, "wrong_confident") - wrong_answers(default_rows, "wrong_confident")
    return {
        "queries": sum(not r["held_out"] for r in rows),
        "confident": sum(r["confident"] for r in rows if not r["held_out"]),
        "possible": sum(r["possible"] for r in rows if not r["held_out"]),
        "wrong_confident": sum(len(r["wrong_confident"]) for r in rows),
        "wrong_possible": sum(len(r["wrong_possible"]) for r in rows),
        "held_out_confident": sum(r["any_confident"] for r in rows if r["held_out"]),
        "new_wrong_confident": sorted(new),
        "strongest_wrong": max((r["strongest_wrong"] for r in rows), default=0),
        "by_speed": by_speed(rows),
        "by_played_speed_bin": by_bin(rows, "speed_percent"),
        "by_relative_speed_bin": by_bin(rows, "relative_percent"),
    }


def clusters_and_pairs(paths):
    """The recordings of the first clusters file (the scale's own) and the
    pairs of every file: the corpus clusters' pairs add speeds between
    corpus files, but their smaller clusters must not stand for the
    larger library's."""
    cluster_of, pairs = {}, []
    for number, path in enumerate(paths):
        with open(path) as file:
            clusters = json.load(file)
        pairs += clusters["pairs"]
        if number == 0:
            for members in clusters["duplicates"]:
                for member in members:
                    cluster_of[member] = set(members)
    return cluster_of, pairs


def recall(out, cluster_files, map_file, panels, labelled):
    cluster_of, pairs = clusters_and_pairs(cluster_files.split(","))
    edges = joins(pairs)
    library_map = None
    if map_file != "-":
        with open(map_file) as file:
            library_map = json.load(file)
    results, rows_of = {}, {}
    for label, paths in (item.split("=", 1) for item in labelled):
        rows_of[label] = []
        for path in paths.split("+"):
            with open(path) as file:
                report = json.load(file)
            left_out = set()
            if "-other-rips" in os.path.basename(path):
                with open(os.path.join(panels, f"sweep-seed-{report['seed']}.json")) as file:
                    draws = json.load(file)["draws"]
                left_out = left_out_by_other_rips(draws, cluster_of, library_map)
            rows_of[label] += query_rows(report, cluster_of, left_out, edges)
    default = next(iter(rows_of))
    for label, rows in rows_of.items():
        results[label] = summary(rows, rows_of[default])
        if label != default:
            results[f"T({label})"] = summary(targeted(rows_of[default], rows), rows_of[default])
    with open(out, "w") as file:
        json.dump({"default": default, "ladders": results}, file, indent=1, ensure_ascii=False)
    for label, s in results.items():
        cells = " ".join(f"{c['bin']}:{c['confident']}/{c['queries']}"
                         for c in s["by_relative_speed_bin"] if c["queries"])
        print(f"{label}: {s['confident']}/{s['queries']} confident, {s['possible']} possible, "
              f"wrong {s['wrong_confident']} (new {len(s['new_wrong_confident'])}), "
              f"held out confident {s['held_out_confident']}; {cells}")


def uncovered(scan):
    """The share of the mix no confident detection covers."""
    spans = sorted((d["start_seconds"], d["end_seconds"]) for d in scan["detections"]
                   if d["confident"])
    covered, end = 0.0, 0.0
    for start, stop in spans:
        start = max(start, end)
        if stop > start:
            covered += stop - start
            end = stop
    return 1.0 - covered / scan["duration_seconds"]


def main():
    command, *args = sys.argv[1:]
    if command == "recall":
        out, cluster_files, map_file, panels, *labelled = args
        recall(out, cluster_files, map_file, panels, labelled)
    elif command == "uncovered":
        with open(args[0]) as file:
            print(f"{uncovered(json.load(file)):.4f}")
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main()
