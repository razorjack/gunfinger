"""Summarise `clusters --from-peaks --other-peaks-dir` against the corpus
clusters: further rips of the corpus recordings, borderline pairs, and the
census of related recordings (pairs of different recordings with 30 hits
or more), each labelled from the tags as versions of one title, the same
artist, or neither.

usage: nas_pairs.py <reports dir of the other library> <corpus clusters>
                    <tags.json: other library path -> tags> <output.json>
"""
import collections, json, os, re, sys

reports, corpus_clusters_file, tags_file, output = sys.argv[1:5]
PREFIX = "second-library/"
clusters = json.load(open(os.path.join(reports, "duplicate-clusters.json")))
corpus_clusters = json.load(open(corpus_clusters_file))
library_map = json.load(open(os.path.join(reports, "library-map.json")))
tags = json.load(open(tags_file))

stands_for = {copy["corpus"]: copy["stands_for"] for copy in library_map["copies"]}
copies = set(stands_for.values())
cluster_of = {}
for number, members in enumerate(clusters["duplicates"]):
    for member in members:
        cluster_of[member] = number


def recording(path):
    return cluster_of.get(path, path)


def other_path(path):
    if path.startswith(PREFIX):
        return path[len(PREFIX):]
    return stands_for.get(path)


def name(path):
    fields = tags.get(other_path(path) or "", {})
    if "artist" in fields and "title" in fields:
        return fields["artist"], fields["title"]
    stem = os.path.splitext(os.path.basename(path))[0]
    artist, _, title = stem.partition(" - ")
    return (artist, title) if title else ("", stem)


ARTIST_SPLIT = re.compile(r"\s*(?:&|,|\+|/|\band\b|\bfeat\.?|\bft\.?|\bfeaturing\b|\bvs\.?|\bx\b|\bpresents\b|\bmeets\b)\s*")
VERSION_WORDS = re.compile(r"\b(vip|remix|rmx|mix|edit|dub|original|remaster(ed)?|version|bootleg|refix|rework|cut|clip|instrumental|dubstyle|remastered|digital)\b")


def artists(artist):
    return {part.strip() for part in ARTIST_SPLIT.split(artist.lower()) if len(part.strip()) > 1}


def base_title(title):
    title = re.sub(r"[\(\[].*?[\)\]]", " ", title.lower())
    title = VERSION_WORDS.sub(" ", title)
    return re.sub(r"[^a-z0-9]+", " ", title).strip()


def label(a, b):
    (artist_a, title_a), (artist_b, title_b) = name(a), name(b)
    if base_title(title_a) and base_title(title_a) == base_title(title_b):
        return "versions of one title"
    if artists(artist_a) & artists(artist_b):
        return "same artist"
    return "neither"


def row(pair):
    return {
        "query": pair["query"],
        "found": pair["found"],
        "query_name": " - ".join(name(pair["query"])),
        "found_name": " - ".join(name(pair["found"])),
        "coverage": round(pair["coverage"], 3),
        "hits": pair["hits"],
        "speed": round(pair["speed"], 4),
        "shared_seconds": round(pair["query_end_seconds"] - pair["query_start_seconds"], 1),
    }


pairs = clusters["pairs"]
further = [p for p in pairs if p["same_recording"] and other_path(p["found"]) not in copies]
borderline = [p for p in pairs if not p["same_recording"] and p["coverage"] >= 0.4]
different = [p for p in pairs if not p["same_recording"] and recording(p["query"]) != recording(p["found"])]

strongest = {}
for pair in different:
    key = frozenset((recording(pair["query"]), recording(pair["found"])))
    if key not in strongest or pair["hits"] > strongest[key]["hits"]:
        strongest[key] = pair
census = sorted(strongest.values(), key=lambda p: -p["hits"])
related = [p for p in census if p["hits"] >= 30]

def band(hits):
    return "240+" if hits >= 240 else "200-239" if hits >= 200 else "60-199" if hits >= 60 else "30-59"

table = collections.Counter((label(p["query"], p["found"]), band(p["hits"])) for p in related)
corpus_members = sum(len(m) for m in corpus_clusters["duplicates"])
summary = {
    "pairs_kept": len(pairs),
    "clusters": len(clusters["duplicates"]),
    "corpus_clusters": len(corpus_clusters["duplicates"]),
    "corpus_cluster_members": corpus_members,
    "further_rips": {"files": len({p["found"] for p in further}), "pairs": [row(p) for p in further]},
    "borderline": [row(p) for p in borderline],
    "related": {
        "pairs_of_recordings": len(related),
        "by_label_and_hits": {f"{l} | {b}": n for (l, b), n in sorted(table.items())},
        "pairs": [{**row(p), "label": label(p["query"], p["found"])} for p in related],
    },
}
json.dump(summary, open(output, "w"), indent=1)
print(f"{len(pairs)} pairs kept; {len(clusters['duplicates'])} clusters with duplicates")
print(f"further rips: {summary['further_rips']['files']} files")
print(f"borderline (40-80%, not same recording): {len(borderline)}")
print(f"related recordings (30 hits or more, strongest pair per two recordings): {len(related)}")
for key, count in summary["related"]["by_label_and_hits"].items():
    print(f"  {key}: {count}")
for pair in summary["related"]["pairs"][:25]:
    print(f"  {pair['hits']:5d} {pair['coverage']:.2f} {pair['shared_seconds']:6.1f}s  {pair['label']:22s} {pair['query_name']}  ~  {pair['found_name']}")
