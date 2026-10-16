"""The second shared-material panel's development half made ready for
its first run (session 25, step 4a), as session 20 made the first panel's
(experiment 0110). The frozen panel (docs/panels/shared-material-2-seed-2026.json)
is read and never changed; its validation half is not read beyond the
names of its files, which no step here aligns.

usage: shared_panel_2_development.py rips FROZEN_PANEL_2 FROZEN_PANEL_1 CLUSTERS TAGS OUT_FAMILIES OUT_TSV
       shared_panel_2_development.py join FAMILIES RIPS_REPORT OUT_FAMILIES
       shared_panel_2_development.py pairs FAMILIES OUT_TSV
       shared_panel_2_development.py freeze FAMILIES DIRECTIONS_REPORT OUT

`rips` takes the development families. Each recording takes in the
current duplicate clusters (CLUSTERS) of its files; two recordings of a
family that a cluster links stop the script. The unjudged files of a
family's titles are the frozen list and any file whose tags (TAGS, the
store's current tags) give the family's titles that no recording, no
family of either panel and no test-derived version holds. Each unjudged
file is listed against each recording's file that a judged pair names
(else its first), for `gunfinger-eval pair`. `join` adds an unjudged file
to a recording when one alignment at the fitted speed covers 80% of the
shorter file and only one recording is so covered: the clustering rule,
as shared_panel.py's `join`. `pairs` writes the judged pairs in both
directions for `pair`; `freeze` cuts each direction's passage by
shared_panel.py's rule and writes one file: the families with their
directions (for shared_outcomes.py, `other_versions` empty: this panel
lists none) and the development scenarios with the blend partner (for
`gunfinger-eval shared`, which reads only those).
"""
import hashlib
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import shared_panel  # noqa: E402
import shared_panel_2  # noqa: E402

PREFIX = "second-library/"


def load(path):
    with open(path, encoding="utf-8") as file:
        return json.load(file)


def dump(data, path):
    with open(path, "w", encoding="utf-8") as file:
        json.dump(data, file, indent=1, ensure_ascii=False)
        file.write("\n")


def rips(frozen_2_path, frozen_1_path, clusters_path, tags_path, families_path, tsv_path):
    frozen = load(frozen_2_path)
    panel_files = shared_panel_2.frozen_files(load(frozen_1_path))
    for family in frozen["families"]:
        for recording in family["recordings"]:
            panel_files |= set(recording)
        panel_files |= set(family["unjudged_same_title"])
    cluster_of = {}
    for members in load(clusters_path)["duplicates"]:
        for member in members:
            cluster_of[member] = set(members)
    tags = load(tags_path)
    words_of = {name: words for name, words in shared_panel_2.NAMES.values()}
    development = [f for f in frozen["families"] if f["half"] == "development"]
    for family in development:
        recordings = []
        added = []
        for recording in family["recordings"]:
            grown = set(recording)
            for path in recording:
                grown |= cluster_of.get(path, set())
            added += sorted(grown - set(recording))
            recordings.append(sorted(recording) + sorted(grown - set(recording)))
        for i, a in enumerate(recordings):
            for b in recordings[i + 1:]:
                if set(a) & set(b):
                    sys.exit(f"{family['name']}: the clusters link two recordings: {sorted(set(a) & set(b))}")
        in_recordings = {path for recording in recordings for path in recording}
        unjudged = [path for path in family["unjudged_same_title"] if path not in in_recordings]
        words = words_of[family["name"]]
        new = sorted(
            PREFIX + path for path, fields in tags.items()
            if PREFIX + path not in in_recordings and PREFIX + path not in panel_files
            and not shared_panel_2.TEST_DERIVED.search(shared_panel_2.normal(path))
            and any(shared_panel_2.normal(artist) in shared_panel_2.normal(fields.get("artist") or "")
                    and shared_panel_2.normal(title) in shared_panel_2.normal(fields.get("title") or "")
                    for artist, title in words))
        family["recordings"] = recordings
        family["added_by_clusters_in_session_25"] = added
        family["unjudged_same_title"] = sorted(set(unjudged) | set(new))
        family["unjudged_added_in_session_25"] = new
        print(f"{family['name']}: recordings {[len(r) for r in recordings]}, {len(added)} added by the "
              f"clusters, unjudged {len(family['unjudged_same_title'])} ({len(new)} new)")
    dump({"seed": frozen["seed"], "families": development}, families_path)
    with open(tsv_path, "w", encoding="utf-8") as out:
        for family in development:
            for path in family["unjudged_same_title"]:
                for recording in family["recordings"]:
                    out.write(f"{path}\t{shared_panel.representative(recording, family)}\n")


def join(families_path, report_path, out_path):
    data = load(families_path)
    reports = load(report_path)
    for family in data["families"]:
        family["joined_in_session_25"] = []
        for path in list(family["unjudged_same_title"]):
            covering = []
            for index, recording in enumerate(family["recordings"]):
                found = shared_panel.representative(recording, family)
                report = next(r for r in reports if r["query"] == path and r["found"] == found)
                search = next((s for s in report["searches"] if s["name"] == shared_panel.FITTED), None)
                best = search["alignments"][0] if search and search["alignments"] else None
                if best and best["coverage"] >= shared_panel.SAME_RECORDING_COVERAGE:
                    covering.append((index, best["coverage"], best["hits"]))
            if len(covering) == 1:
                index, coverage, hits = covering[0]
                family["recordings"][index].append(path)
                family["unjudged_same_title"].remove(path)
                family["joined_in_session_25"].append(
                    {"file": path,
                     "recording": shared_panel.representative(family["recordings"][index], family),
                     "coverage": round(coverage, 3), "hits": hits})
            elif len(covering) > 1:
                print(f"{family['name']}: {path} covers several recordings, left unjudged")
        for joined in family["joined_in_session_25"]:
            print(f"{family['name']}: joined {joined['file'][-70:]} at {joined['coverage']}")
    dump(data, out_path)


def pairs(families_path, out_path):
    shared_panel.pair_lines(families_path, out_path)


def freeze(families_path, report_path, out_path):
    data = load(families_path)
    reports = {(r["query"], r["found"]): r for r in load(report_path)}
    scenarios = []
    for family in data["families"]:
        family["other_versions"] = []
        family["directions"] = []
        for pair in family["pairs"]:
            a, b = pair["files"]
            for played, related in ((a, b), (b, a)):
                report = reports[(played, related)]
                passage = shared_panel.passage_of(report)
                direction = {
                    "name": f"{family['name']}-{len(family['directions']) + 1}",
                    "played": played,
                    "related": related,
                    "played_seconds": round(report["query_seconds"], 1),
                    "related_seconds": round(report["found_seconds"], 1),
                    "passage": passage,
                }
                family["directions"].append(direction)
                if passage is not None:
                    scenarios.append({
                        "name": direction["name"],
                        "played": played,
                        "related": related,
                        "passage_start_seconds": passage["start_seconds"],
                        "passage_end_seconds": passage["end_seconds"],
                        "played_recording": shared_panel.recording_with(played, family),
                        "related_recording": shared_panel.recording_with(related, family),
                    })
    out = {
        "seed": data["seed"],
        "panel": "docs/panels/shared-material-2-seed-2026.json",
        "half": "development",
        "status": ("session 25, step 4a: the frozen panel's development half with its rips "
                   "joined and its passages cut, written before any search; the frozen file "
                   "is unchanged and its validation half stays unsearched"),
        "rule": " ".join(__doc__.split("\n\n")[2].split()),
        "passage_rule": (
            "the strongest alignment of `pair`'s audio second pass (corpus played files) or "
            f"stored peaks at the fitted speed, else the key-locked ladder's; below "
            f"{shared_panel.PASSAGE_HITS} hits a control direction"),
        "families": data["families"],
        "blend_partner": shared_panel.BLEND_PARTNER,
        "scenarios": scenarios,
    }
    dump(out, out_path)
    for family in data["families"]:
        for direction in family["directions"]:
            passage = direction["passage"] or {}
            print(f"{direction['name']:<20} {passage.get('start_seconds', '-'):>6}-"
                  f"{passage.get('end_seconds', '-'):<6} {passage.get('hits', 0):>5} hits"
                  f"{' control' if passage.get('control') else ''}  {passage.get('search', '')}")
    print(f"{len(scenarios)} development scenarios; plan digest input "
          f"{hashlib.sha256(json.dumps(scenarios, sort_keys=True).encode()).hexdigest()[:16]}")


if __name__ == "__main__":
    command, *arguments = sys.argv[1:]
    {"rips": rips, "join": join, "pairs": pairs, "freeze": freeze}[command](*arguments)
