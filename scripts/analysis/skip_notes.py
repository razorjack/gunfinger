"""The skip notes of a peak store: path, reason and, for failures, the
message, one per line as tab-separated values. Opens the store read-only.

usage: skip_notes.py <peak store> [failed|too long|too short]
"""
import os, struct, sys

REASONS = ["failed", "too long", "too short"]


def text(data, at):
    (length,) = struct.unpack_from("<H", data, at)
    at += 2
    return data[at:at + length].decode("utf-8", "replace"), at + length


def note(data):
    assert data[:8] == b"GUNFSKIP"
    path, at = text(data, 10)
    at += 8 + 8 + 4
    reason = REASONS[data[at]]
    detail = text(data, at + 1)[0] if reason == "failed" else str(struct.unpack_from("<d", data, at + 1)[0])
    return path, reason, detail


store = sys.argv[1]
wanted = sys.argv[2] if len(sys.argv) > 2 else None
rows = []
for entry in os.listdir(store):
    if entry.endswith(".skip"):
        with open(os.path.join(store, entry), "rb") as file:
            path, reason, detail = note(file.read())
        if wanted is None or reason == wanted:
            rows.append((path, reason, detail.replace("\n", " ").replace("\t", " ")))
for row in sorted(rows):
    print("\t".join(row))
