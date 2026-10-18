"""Every skip note of a peak store, one per line as tab-separated values:
source path, source size in bytes, source modification time (seconds),
reason (failed, too-long, too-short) and its message or limit.
Opens the store read-only.

usage: store_skips.py <peak store>
"""
import os, struct, sys


def text(data, at):
    (length,) = struct.unpack_from("<H", data, at)
    at += 2
    return data[at:at + length].decode("utf-8"), at + length


store = sys.argv[1]
for entry in sorted(os.listdir(store)):
    if not entry.endswith(".skip"):
        continue
    with open(os.path.join(store, entry), "rb") as source:
        data = source.read()
    assert data[:8] == b"GUNFSKIP", entry
    (version,) = struct.unpack_from("<H", data, 8)
    assert version == 1, entry
    path, at = text(data, 10)
    size, seconds, _nanos = struct.unpack_from("<QqI", data, at)
    at += 20
    kind = data[at]
    at += 1
    if kind == 0:
        detail, at = text(data, at)
        reason = "failed"
        detail = detail.replace("\t", " ").replace("\n", " ")
    else:
        (limit,) = struct.unpack_from("<d", data, at)
        reason = {1: "too-long", 2: "too-short"}[kind]
        detail = f"{limit:g}"
    print(f"{path}\t{size}\t{seconds}\t{reason}\t{detail}")
