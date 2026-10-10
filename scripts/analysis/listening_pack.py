"""The owner's listening pack (roadmap, "Owner edits", the pair review):
pairs of library files to judge by ear, each with a sheet and short clips.

`select` chooses the items from a clusters report and the owner's verdicts:
every join `clusters --cut-sparsest` cut (`cut_links`: the sparsest link of
a chain between two files judged different), every borderline pair (40-80% coverage) between two clusters the owner has
not judged, every sparse join (under a tenth of the median join's hits per
second) the owner has not judged, and `--controls` joins drawn with
`--seed` among the rest. A pair counts as judged when a verdict names a
file of one cluster and a file of the other. Each pair of clusters is one
item, represented by its strongest pair; the others are listed with it.
Files without a current record (pruned) are left out. It writes the pairs
for `gunfinger-eval pair` and the items as JSON.

`render` writes, for each item, a folder under OUT with `sheet.txt` and
clips, from a `pair` report: around each place where the two files stop
lining up, and once inside the main alignment as a control, 8 s of the
first file then the same 8 s of the second at the first file's speed
(`*-a-then-b.mp3`), and the two aligned as a stereo clip, the first file
left and the second right (`*-stereo.mp3`). Then OUT/README.md lists the
items in the order to listen. Clips are rendered by FFmpeg from the files
under the corpus library and the NAS library.

usage:
  listening_pack.py select CLUSTERS VERDICTS STORE_PATHS OUT_PAIRS OUT_ITEMS
                    [--seed N] [--controls N] [--map MAP] [--known ITEMS...]
  listening_pack.py render ITEMS PAIRS_REPORT OUT [--corpus DIR] [--nas DIR]
  listening_pack.py readme OUT

`readme` writes OUT/README.md from every OUT/items-*.json that `render`
left: cut joins first, then borderline pairs (most hits first), then
sparse joins, then the control joins.

STORE_PATHS is a text file of the NAS store's record paths, one per line.
With --map (library-map.json), joins of identical copies are not drawn as
controls. With --known, items holding a pair of files another items file already
holds are left out (to render only the pairs a later run adds; cluster
numbers differ between runs, file pairs do not).
"""

import collections
import hashlib
import json
import os
import random
import subprocess
import sys

PREFIX = "second-library/"
BORDERLINE = 0.4
SPARSE_SHARE = 0.1
CLIP_SECONDS = 8.0
ORDER = {"cut join": 0, "borderline": 1, "sparse join": 2, "control join": 3}
RATE = 44100


def load(path):
    with open(path) as file:
        return json.load(file)


def verdicts_of(path):
    verdicts = {}
    with open(path) as file:
        for line in file:
            if line.startswith("#") or not line.strip():
                continue
            verdict, a, b = line.rstrip("\n").split("\t")
            verdicts[tuple(sorted((a, b)))] = verdict
    return verdicts


def hits_per_second(pair):
    span = pair["query_end_seconds"] - pair["query_start_seconds"]
    return pair["hits"] / span if span > 0 else 0.0


def select(arguments):
    clusters_file, verdicts_file, store_paths, out_pairs, out_items = arguments[:5]
    options = arguments[5:]
    seed = int(option(options, "--seed", "2026"))
    identical = set()
    if "--map" in options:
        for copies in load(option(options, "--map", ""))["copies"]:
            files = [copies["corpus"]] + [
                PREFIX + path for path in [copies["stands_for"], *copies.get("also", [])]
            ]
            identical.update(tuple(sorted((a, b))) for a in files for b in files if a != b)
    controls = int(option(options, "--controls", "10"))
    known = set()
    if "--known" in options:
        for path in options[options.index("--known") + 1:]:
            if path.startswith("--"):
                break
            known.update(pair_key(pair) for item in load(path) for pair in item["pairs"])

    clusters = load(clusters_file)
    with open(store_paths) as file:
        recorded = {PREFIX + line.rstrip("\n") for line in file if line.strip()}

    def exists(path):
        return not path.startswith(PREFIX) or path in recorded

    cluster_of = {}
    for number, members in enumerate(clusters["duplicates"]):
        for member in members:
            cluster_of[member] = f"cluster {number}"

    def key(path):
        return cluster_of.get(path, path)

    judged = {tuple(sorted((key(a), key(b)))) for (a, b) in verdicts_of(verdicts_file)}
    pairs = [p for p in clusters["pairs"] if exists(p["query"]) and exists(p["found"])]

    def unjudged(pair):
        return tuple(sorted((key(pair["query"]), key(pair["found"])))) not in judged

    densities = sorted(hits_per_second(p) for p in clusters["pairs"] if p["same_recording"])
    median = densities[len(densities) // 2] if densities else 0.0
    borderline = [p for p in pairs if p["coverage"] >= BORDERLINE and not p["same_recording"]]
    sparse = [
        p for p in pairs
        if p["same_recording"] and p.get("owner_verdict") is None
        and hits_per_second(p) < SPARSE_SHARE * median
    ]
    cut = {pair_key({"query": c["query"], "found": c["found"]}) for c in clusters.get("cut_links", [])}
    cut_pairs = [p for p in pairs if pair_key(p) in cut]
    borderline = [p for p in borderline if pair_key(p) not in cut]
    items = []
    for kind, chosen in (("cut join", cut_pairs), ("borderline", borderline), ("sparse join", sparse)):
        groups = collections.defaultdict(list)
        for pair in chosen:
            if kind == "cut join" or unjudged(pair):
                groups[tuple(sorted((key(pair["query"]), key(pair["found"]))))].append(pair)
        for group, members in groups.items():
            members.sort(key=lambda p: (-p["coverage"], -p["hits"]))
            items.append(item(kind, group, members))
    taken = {pair_key(p) for i in items for p in i["pairs"]}
    joins = sorted(
        (p for p in pairs if p["same_recording"] and p.get("owner_verdict") is None
         and pair_key(p) not in taken and pair_key(p) not in identical),
        key=lambda p: (p["query"], p["found"]),
    )
    for pair in random.Random(seed).sample(joins, min(controls, len(joins))):
        group = (key(pair["query"]), key(pair["found"]))
        items.append(item("control join", group, [pair]))
    items = [i for i in items if not any(pair_key(p) in known for p in i["pairs"])]
    items.sort(key=lambda i: (ORDER[i["kind"]], -i["pairs"][0]["hits"]))
    for number, entry in enumerate(items, 1):
        entry["number"] = number
    with open(out_pairs, "w") as file:
        for entry in items:
            first = entry["pairs"][0]
            file.write(f"{first['query']}\t{first['found']}\n")
    with open(out_items, "w") as file:
        json.dump(items, file, indent=1)
    counts = collections.Counter(i["kind"] for i in items)
    print(f"median join {median:.1f} hits/s; items: {dict(counts)}")


def item(kind, group, members):
    return {
        "kind": kind,
        "clusters": list(group),
        "pairs": [
            {name: pair.get(name) for name in (
                "query", "found", "coverage", "speed", "hits", "query_start_seconds",
                "query_end_seconds", "found_start_seconds", "found_end_seconds",
                "same_recording", "owner_verdict")}
            for pair in members
        ],
    }


def pair_key(pair):
    return tuple(sorted((pair["query"], pair["found"])))


def option(options, name, default):
    return options[options.index(name) + 1] if name in options else default


def main():
    command, arguments = sys.argv[1], sys.argv[2:]
    if command == "select":
        select(arguments)
    elif command == "render":
        render(arguments)
    elif command == "readme":
        readme(arguments[0])
    else:
        sys.exit(__doc__)


def readme(out):
    items = []
    for name in sorted(os.listdir(out)):
        if name.startswith("items-") and name.endswith(".json"):
            items += [i for i in load(os.path.join(out, name)) if "folder" in i]
    items.sort(key=lambda i: (ORDER[i["kind"]], -i["pairs"][0]["hits"]))
    counts = collections.Counter(i["kind"] for i in items)
    lines = [
        "# Listening pack",
        "",
        f"{len(items)} pairs of library files to judge by ear: "
        f"{counts['cut join']} cut joins (joined, but cut by `clusters --cut-sparsest` "
        "because the join chained two files you judged different; the harness's guess "
        f"at the wrong link), {counts['borderline']} borderline pairs (one alignment covers 40-80% of the "
        f"shorter file), {counts['sparse join']} sparse joins (joined on little "
        f"evidence) and {counts['control join']} control joins (joined, drawn by seed, "
        "to hear what the same recording sounds like here). None has a verdict yet.",
        "",
        "Each folder holds `sheet.txt` (both full paths, the coverage, supported",
        "time, every alignment with its times and where the files stop lining up)",
        "and clips: `N-...-a-then-b.mp3` plays 8 s of A, then the same moment of B",
        "at A's speed; `N-...-stereo.mp3` plays the two aligned, A left and B right.",
        "Clip 1 lies inside the main alignment, as a control; the others sit where",
        "the files stop lining up. A verdict goes into `docs/pair-verdicts.txt` as",
        "the sheet's last line shows: `same` or `different`, then the two paths,",
        "tab-separated.",
        "",
        "In the order to listen:",
        "",
    ]
    for number, entry in enumerate(items, 1):
        first = entry["pairs"][0]
        name = lambda path: os.path.basename(path)
        lines.append(
            f"{number}. `{os.path.basename(entry['folder'])}/` ({entry['kind']}, "
            f"{100 * first['coverage']:.0f}% coverage, {first['hits']} hits): "
            f"{name(first['query'])} ~ {name(first['found'])}")
    lines.append("")
    with open(os.path.join(out, "README.md"), "w") as file:
        file.write("\n".join(lines))
    print(f"README.md: {len(items)} items {dict(counts)}")


FITTED = "peaks, at the fitted speed"
EDGE = 10.0
MAX_PLACES = 4


def render(arguments):
    items_file, report_file, out = arguments[:3]
    options = arguments[3:]
    corpus = option(options, "--corpus", os.path.expanduser(
        "~/Projects/OpenSource/gunfinger/corpus/library"))
    nas = option(options, "--nas", "/Volumes/atlas/Music/dnb")
    items = load(items_file)
    reports = {(r["query"], r["found"]): r for r in load(report_file)}
    os.makedirs(out, exist_ok=True)
    for entry in items:
        first = entry["pairs"][0]
        report = reports.get((first["query"], first["found"]))
        if report is None:
            print(f"item {entry['number']}: no pair report for {first['query']} ~ {first['found']}")
            continue
        folder = os.path.join(out, folder_name(entry))
        os.makedirs(folder, exist_ok=True)
        files = {name: full_path(name, corpus, nas) for name in (first["query"], first["found"])}
        sheet, clips = describe(entry, report, files)
        for clip in clips:
            render_clip(clip, files[first["query"]], files[first["found"]], folder)
        with open(os.path.join(folder, "sheet.txt"), "w") as file:
            file.write(sheet)
        entry["folder"] = folder
        entry["clips"] = [clip["name"] for clip in clips]
    with open(os.path.join(out, f"items-{os.path.basename(items_file)}"), "w") as file:
        json.dump(items, file, indent=1)


def folder_name(entry):
    first = entry["pairs"][0]
    stem = os.path.splitext(os.path.basename(first["found"]))[0]
    slug = "".join(c if c.isalnum() else "-" for c in stem.lower()).strip("-")
    while "--" in slug:
        slug = slug.replace("--", "-")
    digest = hashlib.sha1(f"{first['query']}\t{first['found']}".encode()).hexdigest()[:6]
    return f"{entry['kind'].replace(' ', '-')}-{slug[:36].strip('-')}-{digest}"


def full_path(name, corpus, nas):
    if name.startswith(PREFIX):
        return os.path.join(nas, name[len(PREFIX):])
    return os.path.join(corpus, name)


def clock(seconds):
    tenths = round(max(seconds, 0.0) * 10)
    return f"{tenths // 600}:{tenths % 600 / 10:04.1f}"


def describe(entry, report, files):
    first = entry["pairs"][0]
    a_seconds, b_seconds = report["query_seconds"], report["found_seconds"]
    fitted = next((s for s in report["searches"] if s["name"] == FITTED), None)
    alignments = fitted["alignments"] if fitted else []
    lines = [
        f"{entry['kind'].capitalize()}",
        "",
        f"A: {files[first['query']]}",
        f"   ({clock(a_seconds)})",
        f"B: {files[first['found']]}",
        f"   ({clock(b_seconds)})",
        "",
        f"Clusters' alignment: coverage {100 * first['coverage']:.1f}% of the shorter file, "
        f"{first['hits']} hits, A at {first['speed']:.4f} times B's speed "
        f"({100 * (first['speed'] - 1):+.2f}%); "
        f"A {clock(first['query_start_seconds'])}-{clock(first['query_end_seconds'])}, "
        f"B {clock(first['found_start_seconds'])}-{clock(first['found_end_seconds'])}",
    ]
    supported = sum(a.get("supported") or 0.0 for a in alignments)
    lines.append(
        f"Supported time at the fitted speed: {supported:.2f} of the shorter file "
        "(seconds holding an aligned hit, summed over the alignments)")
    lines += ["", "Alignments at the fitted speed (B time = offset + speed x A time):"]
    for number, a in enumerate(alignments, 1):
        lines.append(
            f"  {number}. A {clock(a['query_start_seconds'])}-{clock(a['query_end_seconds'])}"
            f" = B {clock(a['found_start_seconds'])}-{clock(a['found_end_seconds'])},"
            f" speed {a['speed']:.4f}, offset {a['offset_seconds']:+.1f} s,"
            f" {a['hits']} hits in {a['windows']} windows, supported {a.get('supported') or 0:.2f}")
    if not alignments:
        lines.append("  none: searched alone at the fitted speed, A finds nothing of B")
    clips, places = [], []
    if alignments:
        main = alignments[0]
        places = stop_places(alignments, a_seconds, b_seconds)
        lines += ["", "Where the files stop lining up (in A's time; B placed by alignment 1):"]
        lines += [f"  {clock(t)}: {why}" for t, why in places] or [
            "  nowhere inside both files: alignment 1 runs from end to end"]
        middle = (main["query_start_seconds"] + main["query_end_seconds"]) / 2
        wanted = [("control", middle, "inside alignment 1, as a control")]
        wanted += [(f"at-{clock(t).replace(':', 'm').replace('.', 's')}", t, why)
                   for t, why in places]
        for number, (label, t, why) in enumerate(wanted, 1):
            a_start = min(max(t - CLIP_SECONDS / 2, 0.0), max(a_seconds - CLIP_SECONDS, 0.0))
            b_start = main["offset_seconds"] + main["speed"] * a_start
            if b_start < 0 or b_start + main["speed"] * CLIP_SECONDS > b_seconds:
                lines.append(f"  (no clip at {clock(t)}: B has no audio there under alignment 1)")
                continue
            clips.append({
                "name": f"{number}-{label}",
                "a_start": a_start,
                "b_start": b_start,
                "speed": main["speed"],
                "why": why,
            })
    lines += ["", "Clips (8 s each; B resampled to A's speed, pitch and tempo together):"]
    for clip in clips:
        lines.append(
            f"  {clip['name']}-a-then-b.mp3 and {clip['name']}-stereo.mp3: "
            f"A {clock(clip['a_start'])}-{clock(clip['a_start'] + CLIP_SECONDS)}, "
            f"B {clock(clip['b_start'])}-{clock(clip['b_start'] + clip['speed'] * CLIP_SECONDS)}"
            f" ({clip['why']})")
    lines.append("  -a-then-b: A, then the same moment of B; -stereo: A left, B right")
    others = entry["pairs"][1:]
    if others:
        lines += ["", "Other pairs between the same two clusters:"]
        lines += [f"  {p['query']} ~ {p['found']} ({100 * p['coverage']:.1f}%, {p['hits']} hits)"
                  for p in others]
    lines += [
        "",
        "To record a verdict, a line in docs/pair-verdicts.txt (tabs between the fields):",
        f"  same|different\t{first['query']}\t{first['found']}",
        "",
    ]
    return "\n".join(lines), clips


def stop_places(alignments, a_seconds, b_seconds):
    """Ends of the alignments that lie inside both files, in A's time: where
    one alignment stops and the files no longer line up at its offset.
    The strongest alignments' ends come first; places closer than EDGE to
    one already listed are merged."""
    places = []
    for number, a in enumerate(alignments, 1):
        ends = [
            (a["query_start_seconds"], a["found_start_seconds"], "starts"),
            (a["query_end_seconds"], a["found_end_seconds"], "ends"),
        ]
        for t, b, verb in ends:
            inside = EDGE < t < a_seconds - EDGE and EDGE < b < b_seconds - EDGE
            if inside and all(abs(t - other) >= EDGE for other, _ in places):
                places.append((t, f"alignment {number} {verb} (B {clock(b)}, offset "
                                  f"{a['offset_seconds']:+.1f} s, {a['hits']} hits)"))
            if len(places) == MAX_PLACES:
                return sorted(places)
    return sorted(places)


def render_clip(clip, a_file, b_file, folder):
    speed = clip["speed"]
    b_play = (f"aresample={RATE},asetrate={RATE * speed:.0f},aresample={RATE},"
              f"atrim=0:{CLIP_SECONDS}")
    inputs = [
        "-ss", f"{clip['a_start']:.3f}", "-t", f"{CLIP_SECONDS}", "-i", a_file,
        "-ss", f"{clip['b_start']:.3f}", "-t", f"{speed * CLIP_SECONDS + 0.5:.3f}", "-i", b_file,
    ]
    graphs = {
        "a-then-b": (f"[0:a]aresample={RATE},aformat=channel_layouts=stereo[a];"
                     f"[1:a]{b_play},aformat=channel_layouts=stereo[b];"
                     "[a][b]concat=n=2:v=0:a=1[out]"),
        "stereo": (f"[0:a]aresample={RATE},aformat=channel_layouts=mono[a];"
                   f"[1:a]{b_play},aformat=channel_layouts=mono[b];"
                   "[a][b]join=inputs=2:channel_layout=stereo:map=0.0-FL|1.0-FR[out]"),
    }
    for kind, graph in graphs.items():
        output = os.path.join(folder, f"{clip['name']}-{kind}.mp3")
        if os.path.exists(output):
            continue
        command = ["nice", "-n", "15", "ffmpeg", "-nostdin", "-v", "error", "-y", *inputs,
                   "-filter_complex", graph, "-map", "[out]", "-c:a", "libmp3lame",
                   "-b:a", "192k", output]
        result = subprocess.run(command, capture_output=True, text=True)
        if result.returncode != 0:
            print(f"ffmpeg failed for {output}: {result.stderr.strip()[:300]}")


if __name__ == "__main__":
    main()
