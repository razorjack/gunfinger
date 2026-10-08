"""A NAS-scale clusters file against an earlier one: clusters, files,
same-recording pairs by speed, the gap between the weakest same-recording
pair and the strongest different one, and borderline pairs.

usage: python3 -I scripts/analysis/nas_clusters_summary.py NEW OLD

With OUT set, writes the summary as JSON there.
"""

import json
import os
import sys

PREFIX = "second-library/"


def load(path):
    with open(path) as file:
        return json.load(file)


def summary(clusters):
    pairs = clusters["pairs"]
    same = [pair for pair in pairs if pair["same_recording"]]
    different = [pair for pair in pairs if not pair["same_recording"]]
    files = {member for members in clusters["duplicates"] for member in members}
    other_files = {member for member in files if member.startswith(PREFIX)}
    fast = [pair for pair in same if abs(pair["speed"] - 1.0) > 0.0201]
    strongest_different = max(different, key=lambda pair: pair["coverage"], default=None)
    weakest_same = min(same, key=lambda pair: pair["coverage"], default=None)
    return {
        "clusters": len(clusters["duplicates"]),
        "files": len(files),
        "other_library_files": len(other_files),
        "pairs": len(pairs),
        "same_pairs": len(same),
        "same_pairs_beyond_2_percent": len(fast),
        "same_pair_speeds": [min((p["speed"] for p in same), default=None),
                             max((p["speed"] for p in same), default=None)],
        "weakest_same": weakest_same,
        "strongest_different": strongest_different,
        "borderline": sorted((p for p in different if p["coverage"] >= 0.4),
                             key=lambda pair: -pair["coverage"]),
        "related_below_20_percent": sum(1 for p in pairs if p["coverage"] < 0.2),
    }


def main():
    new, old = load(sys.argv[1]), load(sys.argv[2])
    result = {"new": summary(new), "old": summary(old)}
    old_files = {member for members in old["duplicates"] for member in members}
    new_files = {member for members in new["duplicates"] for member in members}
    result["files_added"] = sorted(new_files - old_files)
    result["files_dropped"] = sorted(old_files - new_files)
    for name in ("old", "new"):
        row = result[name]
        print(f"{name}: {row['clusters']} clusters, {row['files']} files "
              f"({row['other_library_files']} NAS), {row['pairs']} pairs, {row['same_pairs']} same "
              f"({row['same_pairs_beyond_2_percent']} beyond ±2%, speeds {row['same_pair_speeds']}), "
              f"{len(row['borderline'])} borderline, {row['related_below_20_percent']} related below 20%")
        weakest, strongest = row["weakest_same"], row["strongest_different"]
        if weakest:
            print(f"  weakest same {weakest['coverage']:.3f} ({weakest['hits']} hits, speed {weakest['speed']:.4f}): "
                  f"{weakest['query']} = {weakest['found']}")
        if strongest:
            print(f"  strongest different {strongest['coverage']:.3f} ({strongest['hits']} hits, speed "
                  f"{strongest['speed']:.4f}): {strongest['query']} ~ {strongest['found']}")
    print(f"files added {len(result['files_added'])}, dropped {len(result['files_dropped'])}")
    for path in result["files_dropped"]:
        print(f"  dropped: {path}")
    if "OUT" in os.environ:
        with open(os.environ["OUT"], "w") as file:
            json.dump(result, file, indent=1)


if __name__ == "__main__":
    main()
