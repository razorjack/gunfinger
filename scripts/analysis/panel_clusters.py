"""The shared-material panel's recordings under a clusters file (session
23): for each recording of one half, the files the clusters put with its
files beyond the panel's list (rips the panel does not name), and any
cluster that holds files of two recordings of one family.

usage: python3 -I scripts/analysis/panel_clusters.py PANEL CLUSTERS HALF OUT
"""

import json
import sys


def load(path):
    with open(path, encoding="utf-8") as file:
        return json.load(file)


def main():
    panel_path, clusters_path, half, out_path = sys.argv[1:]
    panel, clusters = load(panel_path), load(clusters_path)
    cluster = {}
    for members in clusters["duplicates"]:
        for member in members:
            cluster[member] = set(members)
    rows = []
    for family in panel["families"]:
        if family["half"] != half:
            continue
        for number, recording in enumerate(family["recordings"]):
            listed = set(recording)
            joined = set().union(*(cluster.get(f, {f}) for f in recording))
            others = [set(r) for n, r in enumerate(family["recordings"]) if n != number]
            rows.append({
                "family": family["name"],
                "recording": sorted(recording)[0],
                "files": len(listed),
                "added": sorted(joined - listed),
                "joins_another_recording": any(joined & other for other in others),
            })
    with open(out_path, "w", encoding="utf-8") as file:
        json.dump(rows, file, indent=1, ensure_ascii=False)
    for row in rows:
        print(f"{row['family']:<18} {row['files']:>2} files, +{len(row['added'])}"
              f"{'  JOINS ANOTHER RECORDING' if row['joins_another_recording'] else ''}  {row['recording'][-60:]}")
        for added in row["added"]:
            print(f"    + {added}")


if __name__ == "__main__":
    main()
