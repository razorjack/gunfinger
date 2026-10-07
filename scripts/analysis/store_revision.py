"""The library revision of a peak store's current records, as an index
built from the store alone (`identify --store-only`) would record it:
FNV-1a over each record's source path, NUL, size (u64), mtime seconds
(i64) and nanoseconds (u32), little-endian, in path order. Assumes every
record is current under one profile and inside the track length range.
Opens the store read-only.

usage: store_revision.py <peak store>
"""
import os, struct, sys

store = sys.argv[1]
sources = []
for entry in os.listdir(store):
    if not entry.endswith(".peaks"):
        continue
    with open(os.path.join(store, entry), "rb") as file:
        data = file.read(4096)
    (profile_length,) = struct.unpack_from("<H", data, 10)
    at = 12 + profile_length
    (path_length,) = struct.unpack_from("<H", data, at)
    at += 2
    path = data[at:at + path_length]
    size, seconds, nanos = struct.unpack_from("<QqI", data, at + path_length)
    sources.append((path, size, seconds, nanos))

sources.sort(key=lambda source: source[0])
digest = 0xCBF29CE484222325
for path, size, seconds, nanos in sources:
    for byte in path + b"\0" + struct.pack("<QqI", size, seconds, nanos):
        digest = ((digest ^ byte) * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
print(f"{len(sources)} records, revision {digest:016x}")
