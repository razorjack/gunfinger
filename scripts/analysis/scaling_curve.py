"""Chance against index size: for each reports directory (one library
size), the development scan and sweep seed 2026: indexed assets, false
candidates (all, 30 hits or more, 60 or more), the strongest false
candidate on audio not in the index, and the strongest unrelated one
(different artist and title, labelled by nas_calibrate.py).

usage: python3 -I scripts/analysis/scaling_curve.py TAGS MAP HAND_LABELS DIR...

Each DIR must hold sweep-seed-2026.json and scan-stakka-skynet-knowledge.json.
With OUT set, writes the rows as JSON there.
"""

import json
import os
import subprocess
import sys
import tempfile

HERE = os.path.dirname(os.path.abspath(__file__))


def load(path):
    with open(path) as file:
        return json.load(file)


def row(directory, tags, library_map, hand):
    sweep = load(os.path.join(directory, "sweep-seed-2026.json"))
    scan = load(os.path.join(directory, "scan-stakka-skynet-knowledge.json"))
    false = [detection for query in sweep["queries"] for detection in query["detections"]
             if not detection["correct"]]
    not_indexed = [detection["hits"] for query in sweep["queries"] if query["held_out"]
                   for detection in query["detections"]]
    scan_false = scan["score"]["false_candidates"]

    def strongest_unrelated(name):
        with tempfile.TemporaryDirectory() as only:
            os.symlink(os.path.abspath(os.path.join(directory, name)), os.path.join(only, name))
            output = os.path.join(only, "calibrate.json")
            subprocess.run([sys.executable, "-I", os.path.join(HERE, "nas_calibrate.py"), only, tags,
                            library_map, output, hand], check=True, capture_output=True)
            unrelated = load(output)["strongest"]["unrelated"]
        return unrelated and {"hits": unrelated["hits"], "query": unrelated["query_name"],
                              "found": unrelated["found_name"], "where": unrelated["where"]}

    return {
        "directory": directory,
        "indexed_assets": sweep["indexed_assets"],
        "sweep_false": len(false),
        "sweep_false_30": sum(1 for detection in false if detection["hits"] >= 30),
        "sweep_false_60": sum(1 for detection in false if detection["hits"] >= 60),
        "sweep_wrong": sum(1 for detection in false if detection["confident"]),
        "strongest_not_indexed": max(not_indexed, default=0),
        "scan_identified": scan["score"]["identified"],
        "scan_wrong": scan["score"]["wrong"],
        "scan_false_kept": len(scan_false),
        # The scan labels a false candidate by the track playing at its
        # middle, so shared material in a blend can show as unrelated.
        "sweep_unrelated": strongest_unrelated("sweep-seed-2026.json"),
        "scan_unrelated": strongest_unrelated("scan-stakka-skynet-knowledge.json"),
    }


def main():
    tags, library_map, hand = sys.argv[1:4]
    rows = [row(directory, tags, library_map, hand) for directory in sys.argv[4:]]
    none = {"hits": 0, "query": "", "found": ""}
    for entry in rows:
        sweep, scan = entry["sweep_unrelated"] or none, entry["scan_unrelated"] or none
        print(f"{entry['indexed_assets']:>6} assets: sweep false {entry['sweep_false']} "
              f"(30+: {entry['sweep_false_30']}, 60+: {entry['sweep_false_60']}, confident {entry['sweep_wrong']}); "
              f"not indexed {entry['strongest_not_indexed']}; scan {entry['scan_identified']}/11, "
              f"wrong {entry['scan_wrong']}")
        print(f"        unrelated: sweep {sweep['hits']} ({sweep['query']} ~ {sweep['found']}); "
              f"scan {scan['hits']} ({scan['query']} ~ {scan['found']})")
    if "OUT" in os.environ:
        with open(os.environ["OUT"], "w") as file:
            json.dump(rows, file, indent=1)


if __name__ == "__main__":
    main()
