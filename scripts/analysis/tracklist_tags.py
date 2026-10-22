"""Session 39: which recordings of a peak store have tags fit for scoring the
printed tracklist.

A recording (a duplicate cluster, or a file in none) qualifies when every
file of it has an artist and a title tag, the files' artists and titles
compare equal as the CLI compares them (letters and digits in lower case,
`report::compared_text`), and each file's name holds every word of that
artist and title. Reads the store's `.tags` and `.peaks` headers (layout in
`store.rs`); writes the qualifying files as JSON (`assets`) with a count per
reason a recording fails.

    python3 -I scripts/analysis/tracklist_tags.py STORE CLUSTERS.json OUT.json
"""
import json
import os
import re
import struct
import sys


def read_string(data, at):
    (length,) = struct.unpack_from("<H", data, at)
    at += 2
    return data[at:at + length].decode("utf-8"), at + length


def tags_of(path):
    data = open(path, "rb").read()
    assert data[:8] == b"GUNFTAGS", path
    at = 10
    source, at = read_string(data, at)
    at += 8 + 8 + 4
    fields = []
    for _ in range(3):
        present = data[at]
        at += 1
        if present:
            value, at = read_string(data, at)
            fields.append(value)
        else:
            fields.append(None)
    return source, fields[0], fields[1]


def words(text):
    return [word.lower() for word in re.split(r"[^\w]|_", text or "") if word]


def compared(text):
    found = words(text)
    return " ".join(found) if found else None


def main(store, clusters_path, out):
    tags = {}
    for name in os.listdir(store):
        if name.endswith(".tags"):
            source, artist, title = tags_of(os.path.join(store, name))
            tags[source] = (artist, title)
    clusters = json.load(open(clusters_path))["duplicates"]
    clustered = {path for cluster in clusters for path in cluster}
    recordings = [cluster for cluster in clusters if all(p in tags for p in cluster)]
    recordings += [[path] for path in sorted(tags) if path not in clustered]
    reasons = {}
    assets = []
    for recording in recordings:
        names = set()
        reason = None
        for path in recording:
            artist, title = tags[path]
            if not compared(artist) or not compared(title):
                reason = "a file without an artist or title tag"
                break
            names.add((compared(artist), compared(title)))
            in_name = set(words(os.path.splitext(os.path.basename(path))[0]))
            if not set(words(artist) + words(title)) <= in_name:
                reason = "a file name without the tagged artist and title"
        if reason is None and len(names) > 1:
            reason = "files tagged with different artists or titles"
        if reason:
            reasons[reason] = reasons.get(reason, 0) + 1
        else:
            assets += recording
    summary = {
        "recordings": len(recordings),
        "qualifying_recordings": len(recordings) - sum(reasons.values()),
        "qualifying_recordings_of_two_files_or_more": sum(
            1 for r in recordings if len(r) > 1 and all(p in assets for p in r)),
        "failing": reasons,
    }
    json.dump({"summary": summary, "assets": sorted(assets)}, open(out, "w"), indent=1)
    print(json.dumps(summary, indent=1))


if __name__ == "__main__":
    main(*sys.argv[1:])
