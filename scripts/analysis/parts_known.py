"""Session 58 (experiment 0241): the pre-registration's check against the known cases. The
five pruned files' saved pairs in the clusters file before session 51's
ignore run, oriented so that the pruned file is A, judged by the rules of
`parts::assess` (re-implemented here; constants as in parts.rs)."""
import json, sys

EDGE, MIN_S, MIN_HITS, MAX_COV, REST, INSIDE, BEYOND, STRONG = 15, 20, 60, 0.8, 30, 30, 20, 100
P = "second-library/"
old = json.load(open("work/listening-2026-10-25/duplicate-clusters-before.json"))
seconds = {P + r[2]: float(r[0]) for r in (l.rstrip("\n").split("\t") for l in open("work/s58/store-paths.tsv"))}
pruned = {}
for line in open("work/s58/pruned-durations.tsv"):
    d, path = line.rstrip("\n").split("\t")
    pruned[P + path] = float(d)
seconds.update(pruned)
cluster = {m: ms[0] for ms in old["duplicates"] for m in ms}

def edge(p, L):
    n = p["end"] - p["start"]
    if not (n >= MIN_S and p["hits"] >= MIN_HITS and n < MAX_COV * L and L - n >= REST):
        return None
    if p["start"] <= EDGE and p["fstart"] >= INSIDE:
        return "start"
    if p["end"] >= L - EDGE and p["fsec"] - p["fend"] >= INSIDE:
        return "end"
    return None

def elsewhere(p, e):
    n = p["end"] - p["start"]
    further = p["end"] >= e["end"] + BEYOND if e["role"] == "start" else p["start"] <= e["start"] - BEYOND
    return n >= MIN_S and p["hits"] >= MIN_HITS and p["found"] != e["found"] and p["cluster"] != e["cluster"] and further

for a, L in pruned.items():
    parts = []
    for q in old["pairs"]:
        if q["query"] == a:
            s, e, fs, fe, f = q["query_start_seconds"], q["query_end_seconds"], q["found_start_seconds"], q["found_end_seconds"], q["found"]
        elif q["found"] == a:
            s, e, fs, fe, f = q["found_start_seconds"], q["found_end_seconds"], q["query_start_seconds"], q["query_end_seconds"], q["query"]
        else:
            continue
        parts.append(dict(found=f, cluster=cluster.get(f, f), start=s, end=e, fstart=fs, fend=fe, fsec=seconds.get(f, float("nan")), hits=q["hits"]))
    for p in parts:
        p["role"] = edge(p, L) or "other"
    edges = [p for p in parts if p["role"] != "other"]
    for p in parts:
        if p["role"] == "other" and any(elsewhere(p, e) for e in edges):
            p["role"] = "elsewhere"
    strength = max((p["hits"] for p in edges), default=None)
    worth = strength is not None and (any(p["role"] == "elsewhere" for p in parts) or strength >= STRONG)
    print(f"{a[len(P):]} ({L:.0f} s): {'candidate' if edges else 'not a candidate'}, strength {strength}, worth the ear {worth}, {len(parts)} saved pairs")
    for p in sorted(parts, key=lambda p: (p["start"], -p["hits"])):
        if p["hits"] >= 30 or p["role"] != "other":
            print(f"  {p['role']:9} A {p['start']:5.0f}-{p['end']:5.0f}  B {p['fstart']:5.0f}-{p['fend']:5.0f} of {p['fsec']:5.0f}  {p['hits']:5} hits  {p['found'][len(P):]}")
