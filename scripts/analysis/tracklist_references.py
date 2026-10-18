"""Checks each confident play of a saved `identify -f json` report against
the set's manifest: the slots it overlaps (from 90 s before a track's
start to 90 s after the next one's, as the harness scores) and whether its
file, or a file with the same audio, is one of those slots' references.
Corpus references count through their copies in the searched store: files
whose peak record (duration, peak count, peak digest) is identical, which
holds when both stores were made on one machine. Plays that match no
reference are listed with their tags, to be read by title; the NAS
clusters the harness credits other rips through are not used.

usage (from the repository root): tracklist_references.py <set name>
    <report.json> <store_records.py listing of the corpus store>
    <store_records.py listing of the searched store>
"""
import json, sys, tomllib
from collections import defaultdict

s, report, corpus_records, nas_records = sys.argv[1:5]
manifest = tomllib.load(open(f"corpus/sets/{s}/tracklist.toml", "rb"))
r = json.load(open(report))
end = r["query"]["duration_seconds"]
def secs(t):
    parts = [float(x) for x in t.split(":")]
    v = 0
    for x in parts: v = v * 60 + x
    return v
key = lambda f: tuple(f[1:4])  # duration, count, digest
corpus = {l.split("\t")[0]: key(l.rstrip("\n").split("\t")) for l in open(corpus_records, encoding="utf-8")}
by_key = defaultdict(set)
for l in open(nas_records, encoding="utf-8"):
    f = l.rstrip("\n").split("\t"); by_key[key(f)].add(f[0])
tracks = manifest["track"]
slots = []
for i, t in enumerate(tracks):
    start = secs(t["start"]); nxt = secs(tracks[i + 1]["start"]) if i + 1 < len(tracks) else end
    refs = set()
    for ref in t.get("reference", []):
        if ref.startswith("second-library/"): refs.add(ref[len("second-library/"):])
        else: refs |= by_key.get(corpus.get(ref), set())
    slots.append((t["position"], f"{t['artist']} - {t['title']}", start - 90, nxt + 90, refs, len(t.get("reference", []))))
identified = defaultdict(list); unmatched = []
for n, p in enumerate(r["plays"], 1):
    if p["confidence"] != "confident": continue
    files = {p["asset"]} | {a for a in p.get("same_audio", [])}
    over = [sl for sl in slots if p["start_seconds"] < sl[3] and p["end_seconds"] > sl[2]]
    hit = [sl for sl in over if files & sl[4]]
    if hit:
        for sl in hit: identified[sl[0]].append(n)
    else:
        unmatched.append((n, p, [sl[0] for sl in over]))
referenced = [sl for sl in slots if sl[5]]
print(f"{s}: {sum(1 for sl in referenced if identified[sl[0]])}/{len(referenced)} referenced slots with a confident play of a reference copy")
for sl in referenced:
    if not identified[sl[0]]: print(f"  slot {sl[0]} {sl[1]}: no confident play of a reference copy (refs mapped to {len(sl[4])} NAS files)")
for n, p, over in unmatched:
    t = p.get("tags") or {}
    print(f"  play {n} not a reference of an overlapping slot {over}: {p['start_seconds']/60:.2f}-{p['end_seconds']/60:.2f} min, {p['hits']} hits, shares material with {p.get('shares_material_with')}, tags {t.get('artist')} - {t.get('title')}\n    {p['asset']}")
