"""The shared-material panel (session 20): recording families built from
the judgements by ear, split into a development and a validation half
by family with a fixed seed, and written as the input of `gunfinger-eval
shared`.

usage: shared_panel.py families <clusters.json> <tags.json> <out.json>
       shared_panel.py rip-pairs <families.json> <out.tsv>
       shared_panel.py join <families.json> <pairs report.json> <out.json>
       shared_panel.py pairs <families.json> <out.tsv>
       shared_panel.py freeze <families.json> <pairs report.json> <panel.json> <plan.json>

`families` reads the judgements (docs/pair-verdicts.txt's `different`
lines, the answer log's `shared` answers, and the two judged in the docs,
listed below), the NAS duplicate clusters (session 18's file, read only)
and the NAS store's tags (store_tags.py), and writes the families with
their split. Files whose tags give a family's titles but which no cluster
holds may be rips the clusters (made at 33,723 records) never saw:
`rip-pairs` lists each against the family's recordings for `gunfinger-eval
pair`, and `join` adds a file to a recording when one alignment at the
fitted speed covers 80% of the shorter file, the clustering rule
(`duplicate-clusters.json`'s criterion), from library audio alone. `pairs` writes one line per judged pair of recordings for
`gunfinger-eval pair`, both directions. `freeze` reads `pair`'s report,
takes each direction's shared passage (below) and writes the panel and
the development half's plan for `shared`.

Every path is a library path as the harness names it (corpus paths, and
`second-library/<path>` for the NAS). Nothing here reads audio.
"""
import hashlib
import json
import re
import sys
import unicodedata

SEED = 2026
PREFIX = "second-library/"
VERDICTS = "docs/pair-verdicts.txt"
ANSWERS = "work/audition/answers.jsonl"

# Judgements recorded in the docs rather than in the verdicts file or the
# answer log, with where.
DOCUMENTED = [
    ("Skynet & Stakka - Clockwork.mp3",
     "extra/a-stakka_and_skynet-clockwork_(stakka_rmx)-sour.mp3",
     "different, shares a passage: experiment 0026, ADR 0019"),
    ("Bad Company - China Cup.mp3", "Bad Company - The Nine.mp3",
     "different, share their drums: notes, after session 4"),
]

# 0076's other versions of a judged family, never a scenario's file.
OTHER_VERSIONS_FROM = "docs/experiments/data/0076-shared-material-nas.json"

# The five test-derived versions stay out (brief 20).
TEST_DERIVED = re.compile(
    r"terminal 2|orders|funktion|clear skyz|beachball", re.IGNORECASE)

# Each family's name, by one of its recordings' files, and the title
# words its tags are searched for (other versions and unclustered rips).
NAMES = {
    "DJ Trace - Sonar.m4a": ("sonar", [("trace", "sonar")]),
    "Bad Company - The Nine.mp3": ("the-nine", [("bad company", "the nine"), ("bad company", "china cup")]),
    "second-library/__full_scene/1999/1999-11/1999-11-29/ram-ram_trilogy_chapter_2-ramm23-1998-sour/01-ram_trilogy-mind_overload.mp3":
        ("mind-overload", [("ram trilogy", "mind overload")]),
    "extra/A. Pyro -- Time Is Broken.mp3": ("pyro", [("pyro", "time is broken"), ("pyro", "ignorance")]),
    "second-library/__full_scene/dnb_scene - 2000 - Part 1/2000-06/pwl_int-prizna_feat_demolition_man-nlbcdx18-1995-sour/02-prizna_feat_demolition_man-fire_(original_mix)-sour.mp3":
        ("fire", [("demolition man", "fire")]),
    "extra/01 - Ryme Tyme - T-Minus.mp3": ("t-minus", [("ryme tyme", "t minus")]),
    "Ed Rush & Optical - Dozer.mp3": ("dozer", [("ed rush", "dozer")]),
    "extra/APH39B-Aphrodite-2001-sour/a-aphrodite-playground_shades-sour.mp3":
        ("playground-shades", [("aphrodite", "playground shades"), ("critikal", "interlude")]),
    "second-library/__youtube_archivists/alien5ive/IDIOMA RECORDINGS [ IDIOMA 002 ： CAUSE 4 CONCERN - night gasp - ] drum and bass.opus":
        ("night-gasp", [("cause 4 concern", "night gasp"), ("ray keith", "dark soldier")]),
    "Ed Rush & Optical - Watermelon.mp3": ("watermelon", [("ed rush", "watermelon"), ("300 baud", "monotone")]),
    "extra/a. falcon - fractles.mp3": ("fractles", [("falcon", "fract"), ("ryme tyme", "we enter")]),
    "Skynet & Stakka - Clockwork.mp3": ("clockwork", [("stakka", "clockwork")]),
}


def normal(text):
    text = unicodedata.normalize("NFKC", text).lower()
    return re.sub(r"[^a-z0-9]+", " ", text).strip()


def judged_pairs():
    pairs = []
    for line in open(VERDICTS, encoding="utf-8"):
        if line.startswith("different\t"):
            _, a, b = line.rstrip("\n").split("\t")
            pairs.append((a, b, "different: docs/pair-verdicts.txt"))
    for line in open(ANSWERS, encoding="utf-8"):
        record = json.loads(line)
        if record.get("action") == "answer" and record.get("choice") == "shared":
            a, b = record["files"]
            pairs.append((a, b, f"shared: answer {record['seq']} of the answer log"))
    pairs.extend(DOCUMENTED)
    return pairs


def recording_of(path, clusters):
    for cluster in clusters:
        if path in cluster:
            return tuple(sorted(cluster))
    return (path,)


def families(clusters_path, tags_path, out_path):
    clusters = json.load(open(clusters_path, encoding="utf-8"))["duplicates"]
    tags = json.load(open(tags_path, encoding="utf-8"))
    pairs = judged_pairs()
    for a, b, _ in pairs:
        for path in (a, b):
            if TEST_DERIVED.search(path):
                sys.exit(f"a test-derived version among the judged pairs: {path}")
    # Recordings joined by judged pairs: union-find over recordings.
    parent = {}

    def find(recording):
        parent.setdefault(recording, recording)
        while parent[recording] != recording:
            recording = parent[recording]
        return recording

    recording_pairs = {}
    for a, b, why in pairs:
        ra, rb = recording_of(a, clusters), recording_of(b, clusters)
        parent[find(ra)] = find(rb)
        key = tuple(sorted([ra, rb]))
        recording_pairs.setdefault(key, {"files": (a, b), "why": []})["why"].append(why)
    grouped = {}
    for recording in list(parent):
        grouped.setdefault(find(recording), []).append(recording)
    other_versions = other_versions_of_0076()
    result = []
    for recordings in grouped.values():
        files = {path for recording in recordings for path in recording}
        name, title_words = next(
            (NAMES[path] for path in sorted(files) if path in NAMES), (None, []))
        if name is None:
            sys.exit(f"a family without a name: {sorted(files)[:3]}")
        versions = sorted(
            path for path in other_versions
            if path not in files and any(
                normal(word[1]) in normal(path) for word in title_words))
        unjudged = sorted(
            PREFIX + path for path, fields in tags.items()
            if PREFIX + path not in files and PREFIX + path not in versions
            and not TEST_DERIVED.search(path)
            and any(normal(artist) in normal(fields.get("artist") or "")
                    and normal(title) in normal(fields.get("title") or "")
                    for artist, title in title_words))
        family_pairs = [
            {"files": list(value["files"]), "why": value["why"]}
            for key, value in sorted(recording_pairs.items())
            if key[0] in recordings]
        result.append({
            "name": name,
            "recordings": [list(recording) for recording in sorted(recordings)],
            "pairs": family_pairs,
            "other_versions": versions,
            "unjudged_same_title": unjudged,
        })
    result.sort(key=lambda family: family["name"])
    order = sorted(result, key=lambda family: hashlib.sha256(
        f"{SEED}:{family['name']}".encode()).hexdigest())
    development = {family["name"] for family in order[:(len(order) + 1) // 2]}
    for family in result:
        family["half"] = "development" if family["name"] in development else "validation"
    json.dump({"seed": SEED, "families": result}, open(out_path, "w", encoding="utf-8"),
              indent=1, ensure_ascii=False)
    for family in result:
        print(f"{family['half']:<12} {family['name']:<18} recordings {len(family['recordings'])}"
              f" pairs {len(family['pairs'])} versions {len(family['other_versions'])}"
              f" unjudged {len(family['unjudged_same_title'])}")


def other_versions_of_0076():
    data = json.load(open(OTHER_VERSIONS_FROM, encoding="utf-8"))
    return {path for row in data["plan"] for path in (row["played"], row["related"])}


def rip_pairs(families_path, out_path):
    data = json.load(open(families_path, encoding="utf-8"))
    with open(out_path, "w", encoding="utf-8") as out:
        for family in data["families"]:
            for path in family["unjudged_same_title"]:
                for recording in family["recordings"]:
                    out.write(f"{path}\t{representative(recording, family)}\n")


def representative(recording, family):
    """The recording's file that a judged pair names, else its first."""
    named = {path for pair in family["pairs"] for path in pair["files"]}
    return next((path for path in recording if path in named), recording[0])


SAME_RECORDING_COVERAGE = 0.8
FITTED = "peaks, at the fitted speed"


def join(families_path, report_path, out_path):
    data = json.load(open(families_path, encoding="utf-8"))
    reports = json.load(open(report_path, encoding="utf-8"))
    for family in data["families"]:
        family["joined_in_session_20"] = []
        for path in list(family["unjudged_same_title"]):
            covering = []
            for index, recording in enumerate(family["recordings"]):
                found = representative(recording, family)
                report = next(r for r in reports if r["query"] == path and r["found"] == found)
                search = next((s for s in report["searches"] if s["name"] == FITTED), None)
                best = search["alignments"][0] if search and search["alignments"] else None
                if best and best["coverage"] >= SAME_RECORDING_COVERAGE:
                    covering.append((index, best["coverage"], best["hits"]))
            if len(covering) == 1:
                index, coverage, hits = covering[0]
                family["recordings"][index].append(path)
                family["unjudged_same_title"].remove(path)
                family["joined_in_session_20"].append(
                    {"file": path, "recording": representative(family["recordings"][index], family),
                     "coverage": round(coverage, 3), "hits": hits})
            elif len(covering) > 1:
                print(f"{family['name']}: {path} covers several recordings, left unjudged")
        for joined in family["joined_in_session_20"]:
            print(f"{family['name']}: joined {joined['file'][-70:]} at {joined['coverage']}")
    json.dump(data, open(out_path, "w", encoding="utf-8"), indent=1, ensure_ascii=False)


def pair_lines(families_path, out_path):
    data = json.load(open(families_path, encoding="utf-8"))
    with open(out_path, "w", encoding="utf-8") as out:
        for family in data["families"]:
            for pair in family["pairs"]:
                a, b = pair["files"]
                out.write(f"{a}\t{b}\n{b}\t{a}\n")


# A direction's shared passage: the strongest alignment of the search
# closest to `identify` that the direction has (the audio's second pass
# when the played file is in the corpus, else the stored peaks at the
# fitted speed), in the played file's time; when it holds fewer than
# PASSAGE_HITS, the key-locked ladder's (a relation the turntable search
# cannot follow). Below PASSAGE_HITS in both, the direction has no passage
# above chance: the two judged pairs the notes call chance spans reach at
# most 16 hits in any of `pair`'s searches. Such a direction stays in the
# panel as a control, cut from its strongest alignment.
SEARCHES = ["audio, both ladders, second pass", "peaks, at the fitted speed"]
KEY_LOCKED = "peaks, key-locked ladder"
PASSAGE_HITS = 25


def strongest(report, name):
    search = next((s for s in report["searches"] if s["name"] == name), None)
    if search is None or not search["alignments"]:
        return None
    return max(search["alignments"], key=lambda a: a["hits"])


def passage_of(report):
    name = next((n for n in SEARCHES if any(s["name"] == n for s in report["searches"])), None)
    best = strongest(report, name) if name else None
    if best is None or best["hits"] < PASSAGE_HITS:
        locked = strongest(report, KEY_LOCKED)
        if locked is not None and locked["hits"] >= PASSAGE_HITS:
            name, best = KEY_LOCKED, locked
    if best is None:
        candidates = [(strongest(report, s["name"]), s["name"]) for s in report["searches"]]
        candidates = [c for c in candidates if c[0] is not None]
        if not candidates:
            return None
        best, name = max(candidates, key=lambda c: c[0]["hits"])
    return {
        "search": name,
        "control": best["hits"] < PASSAGE_HITS,
        "start_seconds": round(best["query_start_seconds"], 1),
        "end_seconds": round(best["query_end_seconds"], 1),
        "related_start_seconds": round(best["found_start_seconds"], 1),
        "related_end_seconds": round(best["found_end_seconds"], 1),
        "speed": round(best["speed"], 4),
        "key_locked": best["key_locked"],
        "hits": best["hits"],
        "windows": best["windows"],
    }


# The blend partner: a corpus file of the first development set outside
# every family, played over the context of the blend query at equal level.
BLEND_PARTNER = {"file": "Skynet & Stakka - Decoy.mp3", "start_seconds": 60.0}


def freeze(families_path, report_path, panel_path, plan_path):
    data = json.load(open(families_path, encoding="utf-8"))
    reports = {(r["query"], r["found"]): r for r in json.load(open(report_path, encoding="utf-8"))}
    scenarios = []
    for family in data["families"]:
        family["directions"] = []
        for pair in family["pairs"]:
            a, b = pair["files"]
            for played, related in ((a, b), (b, a)):
                report = reports[(played, related)]
                passage = passage_of(report)
                direction = {
                    "name": f"{family['name']}-{len(family['directions']) + 1}",
                    "played": played,
                    "related": related,
                    "played_seconds": round(report["query_seconds"], 1),
                    "related_seconds": round(report["found_seconds"], 1),
                    "passage": passage,
                }
                family["directions"].append(direction)
                if family["half"] == "development" and passage is not None:
                    scenarios.append({
                        "name": direction["name"],
                        "played": played,
                        "related": related,
                        "passage_start_seconds": passage["start_seconds"],
                        "passage_end_seconds": passage["end_seconds"],
                        "played_recording": recording_with(played, family),
                        "related_recording": recording_with(related, family),
                    })
    data["passage_rule"] = (
        "the strongest alignment of `pair`'s audio second pass (corpus played files) or "
        f"stored peaks at the fitted speed, else the key-locked ladder's; below {PASSAGE_HITS} "
        "hits a control direction")
    data["blend_partner"] = BLEND_PARTNER
    json.dump(data, open(panel_path, "w", encoding="utf-8"), indent=1, ensure_ascii=False)
    json.dump({"blend_partner": BLEND_PARTNER, "scenarios": scenarios},
              open(plan_path, "w", encoding="utf-8"), indent=1, ensure_ascii=False)
    for family in data["families"]:
        for direction in family["directions"]:
            passage = direction["passage"] or {}
            print(f"{family['half'][:3]} {direction['name']:<20} {passage.get('start_seconds', '-'):>6}-"
                  f"{passage.get('end_seconds', '-'):<6} {passage.get('hits', 0):>5} hits"
                  f"{' control' if passage.get('control') else ''}  {passage.get('search', '')}")
    print(f"{len(scenarios)} development scenarios")


def recording_with(path, family):
    return next(recording for recording in family["recordings"] if path in recording)


if __name__ == "__main__":
    command, *arguments = sys.argv[1:]
    {"families": families, "rip-pairs": rip_pairs, "join": join, "pairs": pair_lines,
     "freeze": freeze}[command](*arguments)
