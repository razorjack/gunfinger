"""The owner's listening list: pairs of files the clusters keep apart that
a search confuses (confident false detections in sweep and scan
reports), and the clusters' borderline pairs, with tags, hits, coverage
and where each was heard.

usage: python3 -I scripts/analysis/listening_list.py CLUSTERS TAGS MAP REPORT...

CLUSTERS is a duplicate-clusters.json with pairs; TAGS maps NAS library
paths to tags; MAP is library-map.json. With OUT set, writes the list as
JSON there.
"""

import collections
import json
import os
import sys

PREFIX = "second-library/"


def load(path):
    with open(path) as file:
        return json.load(file)


def main():
    clusters, tags, library_map = load(sys.argv[1]), load(sys.argv[2]), load(sys.argv[3])
    stands_for = {copy["corpus"]: copy["stands_for"] for copy in library_map["copies"]}

    def tagged(path):
        other = path[len(PREFIX):] if path.startswith(PREFIX) else stands_for.get(path)
        fields = tags.get(other or "", {})
        if "title" in fields:
            return f"{fields.get('artist', '?')} - {fields['title']}"
        return "(no tags)"

    coverage = {}
    for pair in clusters["pairs"]:
        key = tuple(sorted((pair["query"], pair["found"])))
        coverage[key] = max(coverage.get(key, 0.0), pair["coverage"])

    heard = collections.defaultdict(list)
    for path in sys.argv[4:]:
        report = load(path)
        name = os.path.basename(path)
        if "queries" in report:
            for query in report["queries"]:
                for detection in query["detections"]:
                    if detection["confident"] and not detection["correct"]:
                        key = tuple(sorted((query["asset"], detection["asset"])))
                        heard[key].append({"where": f"{name}: excerpt at {query['start_seconds']:.0f} s, "
                                                    f"{query['speed_percent']:+.0f}%",
                                           "hits": detection["hits"]})
        else:
            tracks = report["score"]["tracks"]
            for detection in report["score"]["wrong_identifications"]:
                # What was being played: the manifest slot holding the
                # detection's middle, by its credited file or its label.
                middle = (detection["start_seconds"] + detection["end_seconds"]) / 2
                slot = max((track for track in tracks if track["start_seconds"] <= middle),
                           key=lambda track: track["start_seconds"], default=tracks[0])
                played = slot["credited"][0]["asset"] if slot["credited"] else slot["label"]
                key = tuple(sorted((played, detection["asset"])))
                minutes, seconds = divmod(int(detection["start_seconds"]), 60)
                heard[key].append({"where": f"{name}: mix time {minutes}:{seconds:02d}",
                                   "hits": detection["hits"]})

    entries = []
    for key, items in heard.items():
        entries.append({"files": list(key), "tags": [tagged(path) for path in key],
                        "coverage": coverage.get(key), "confident_false": len(items),
                        "strongest_hits": max(item["hits"] for item in items),
                        "heard": sorted(items, key=lambda item: -item["hits"])[:3]})
    for pair in clusters["pairs"]:
        key = tuple(sorted((pair["query"], pair["found"])))
        if 0.4 <= pair["coverage"] < 0.8 and not pair["same_recording"] and key not in heard:
            heard[key] = []
            entries.append({"files": list(key), "tags": [tagged(path) for path in key],
                            "coverage": pair["coverage"], "confident_false": 0,
                            "strongest_hits": pair["hits"], "heard": []})
    entries.sort(key=lambda entry: (-entry["confident_false"], -(entry["coverage"] or 0)))
    for entry in entries:
        cover = f"{entry['coverage']:.0%}" if entry["coverage"] is not None else "not measured"
        print(f"{entry['confident_false']:>3} confident false, coverage {cover}, strongest {entry['strongest_hits']} hits")
        for path, tag in zip(entry["files"], entry["tags"]):
            print(f"      {tag}  [{path}]")
        for item in entry["heard"]:
            print(f"      heard: {item['where']} ({item['hits']} hits)")
    if "OUT" in os.environ:
        with open(os.environ["OUT"], "w") as file:
            json.dump(entries, file, indent=1)


if __name__ == "__main__":
    main()
