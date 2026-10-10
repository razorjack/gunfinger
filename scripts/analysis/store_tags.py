"""The tags a peak store holds, as JSON: each record's source path with
its artist, title and album, for the analysis scripts that name files by
their tags. Opens the store read-only.

usage: store_tags.py <peak store> <output.json>
"""
import json, os, struct, sys

store, output = sys.argv[1], sys.argv[2]


def string(data, at):
    (length,) = struct.unpack_from("<H", data, at)
    at += 2
    return data[at:at + length].decode("utf-8"), at + length


tags = {}
for entry in sorted(os.listdir(store)):
    if not entry.endswith(".tags"):
        continue
    with open(os.path.join(store, entry), "rb") as file:
        data = file.read()
    assert data[:8] == b"GUNFTAGS"
    path, at = string(data, 10)
    at += 20
    fields = {}
    for name in ("artist", "title", "album"):
        present = data[at]
        at += 1
        if present:
            fields[name], at = string(data, at)
    tags[path] = fields
with open(output, "w") as file:
    json.dump(tags, file, indent=0, ensure_ascii=False)
print(f"{len(tags)} tag notes")
