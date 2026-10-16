"""A proposed second shared-material panel (session 23, step 6b): the
recording families of the `different` verdicts by ear that the frozen
panel (docs/panels/shared-material-seed-2026.json) does not hold, split
into a development and a validation half by family with a fixed seed,
before any search.

usage: shared_panel_2.py FROZEN_PANEL CLUSTERS TAGS STORE_RECORDS OUT

A verdict joins the new panel when neither of its files belongs to a
family of the frozen panel (its recordings, other versions and unjudged
files of the family's title, both halves), when both files have a current
record (STORE_RECORDS, store_records.py's output; corpus files always do)
and when neither is one of the five test-derived versions. Recordings are
the duplicate clusters of CLUSTERS; files whose tags give a family's
titles outside its recordings are listed as unjudged, as in the frozen
panel. The split orders family names by SHA-256 of "<seed>:<name>" and
puts the first half (rounded up) in development, the frozen panel's rule.
Passages are not cut here: `pair` and shared_panel.py's passage rule
give them in the session that runs the development half, so no file of
the new panel is searched or aligned now. Each family lists the tracks of
the three development mixes whose references include one of its files
(`in_development_mixes`): those files are development data already,
whichever half the family falls in.
"""
import csv
import hashlib
import json
import re
import sys
import unicodedata
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import devscore  # noqa: E402

SEED = 2026
PREFIX = "second-library/"
VERDICTS = "docs/pair-verdicts.txt"
DEVELOPMENT_SETS = ("stakka-skynet-knowledge", "razorjack-2003-03-29",
                    "nocny-trans-vertical-2002-08-23")
TEST_DERIVED = re.compile(
    r"terminal 2|orders|funktion|clear skyz|beachball", re.IGNORECASE)

# Each new family's name, by one of its files, and the title words its
# tags are searched for (unjudged files of the same title).
NAMES = {
    "second-library/blade-the_kraft-bld002-2000-sour/aa-the_kraft-stab_2_def-sour.mp3":
        ("stab-2-def", [("kraft", "stab 2 def"), ("mampi swift", "2000")]),
    "second-library/__full_scene/dnb_scene - 2000 - Part 1/2000-04/va-drum_and_bass_assassins-dbascd01-2000-sour/209-usual_suspects-killa_bees-sour.mp3":
        ("killa-bees", [("usual suspects", "killa bees")]),
    "second-library/__youtube_archivists/i-witness-dnb/Digital & Spirit - Phantom Force.m4a":
        ("phantom-force", [("digital", "phantom"), ("rift", "meltdown")]),
    "second-library/__youtube_archivists/i-witness-dnb/Kemal & Rob Data - Star Trails (Synergy Remix).m4a":
        ("star-trails", [("kemal", "star trails"), ("konflict", "star trails")]),
    "second-library/__youtube_archivists/i-witness-dnb/Klute - We R The Ones VIP.m4a":
        ("we-r-the-ones", [("klute", "we r the ones")]),
    "second-library/__youtube_archivists/i-witness-dnb/Eddy Woo - Tsunami.m4a":
        ("tsunami", [("eddy woo", "tsunami")]),
}


def normal(text):
    text = unicodedata.normalize("NFKC", text).lower()
    return re.sub(r"[^a-z0-9]+", " ", text).strip()


def load(path):
    with open(path, encoding="utf-8") as file:
        return json.load(file)


def frozen_files(panel):
    files = set()
    for family in panel["families"]:
        for recording in family["recordings"]:
            files |= set(recording)
        files |= set(family["other_versions"]) | set(family["unjudged_same_title"])
    return files


def main():
    frozen_path, clusters_path, tags_path, records_path, out_path = sys.argv[1:]
    frozen = frozen_files(load(frozen_path))
    clusters = load(clusters_path)["duplicates"]
    tags = load(tags_path)
    with open(records_path, encoding="utf-8") as file:
        recorded = {PREFIX + row[0] for row in csv.reader(file, delimiter="\t")}
    cluster_of = {}
    for members in clusters:
        for member in members:
            cluster_of[member] = tuple(sorted(members))

    def recording_of(path):
        return cluster_of.get(path, (path,))

    pairs, left_out = [], []
    for number, line in enumerate(open(VERDICTS, encoding="utf-8"), 1):
        if not line.startswith("different\t"):
            continue
        _, a, b = line.rstrip("\n").split("\t")
        files = {f for path in (a, b) for f in recording_of(path)}
        if files & frozen:
            left_out.append((number, "a family of the frozen panel"))
        elif any(p.startswith(PREFIX) and p not in recorded for p in (a, b)):
            left_out.append((number, "a file without a current record"))
        elif any(TEST_DERIVED.search(p) for p in files):
            left_out.append((number, "a test-derived version"))
        else:
            pairs.append((a, b, number))

    parent = {}

    def find(recording):
        parent.setdefault(recording, recording)
        while parent[recording] != recording:
            recording = parent[recording]
        return recording

    for a, b, _ in pairs:
        parent[find(recording_of(a))] = find(recording_of(b))
    grouped = {}
    for recording in list(parent):
        grouped.setdefault(find(recording), []).append(recording)
    played = [(name, track) for name in DEVELOPMENT_SETS
              for track in devscore.load_set(name, True)]
    families = []
    for recordings in grouped.values():
        files = {path for recording in recordings for path in recording}
        name, words = next((NAMES[p] for p in sorted(files) if p in NAMES), (None, []))
        if name is None:
            sys.exit(f"a family without a name: {sorted(files)[:3]}")
        unjudged = sorted(
            PREFIX + path for path, fields in tags.items()
            if PREFIX + path not in files and PREFIX + path not in frozen
            and not TEST_DERIVED.search(path)
            and any(normal(artist) in normal(fields.get("artist") or "")
                    and normal(title) in normal(fields.get("title") or "")
                    for artist, title in words))
        families.append({
            "name": name,
            "recordings": [list(r) for r in sorted(recordings)],
            "pairs": [{"files": [a, b], "why": f"different: docs/pair-verdicts.txt, line {n}"}
                      for a, b, n in pairs if recording_of(a) in recordings],
            "unjudged_same_title": unjudged,
            "in_development_mixes": [
                f"{name}: {track['label'].strip()}" for name, track in played
                if set(track["references"]) & (files | set(unjudged))],
        })
    families.sort(key=lambda family: family["name"])
    order = sorted(families, key=lambda family: hashlib.sha256(
        f"{SEED}:{family['name']}".encode()).hexdigest())
    development = {family["name"] for family in order[:(len(order) + 1) // 2]}
    for family in families:
        family["half"] = "development" if family["name"] in development else "validation"
    panel = {
        "seed": SEED,
        "status": "proposed in session 23; frozen before any search; passages not cut yet",
        "rule": " ".join(__doc__.split("\n\n")[2:]).replace("\n", " ").strip(),
        "families": families,
        "verdicts_left_out": [{"line": n, "why": why} for n, why in left_out],
    }
    with open(out_path, "w", encoding="utf-8") as file:
        json.dump(panel, file, indent=1, ensure_ascii=False)
    for family in families:
        print(f"{family['half']:<12} {family['name']:<14} recordings {len(family['recordings'])} "
              f"files {sum(len(r) for r in family['recordings'])} pairs {len(family['pairs'])} "
              f"unjudged {len(family['unjudged_same_title'])} "
              f"in development mixes {len(family['in_development_mixes'])}")
    print(f"verdicts left out: {len(left_out)}")


if __name__ == "__main__":
    main()
