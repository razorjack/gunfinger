"""Every peak record of a store, one per line as tab-separated values:
source path, duration in seconds, peak count, a digest of the peak bytes,
the modification time of the record file (when it was written) and the
source file's size in bytes.
Opens the store read-only.

usage: store_records.py <peak store>
"""
import hashlib, os, struct, sys


def string(data, at):
    (length,) = struct.unpack_from("<H", data, at)
    at += 2
    return data[at:at + length].decode("utf-8"), at + length


store = sys.argv[1]
for entry in sorted(os.listdir(store)):
    if not entry.endswith(".peaks"):
        continue
    file = os.path.join(store, entry)
    with open(file, "rb") as source:
        data = source.read()
    assert data[:8] == b"GUNFPEAK"
    _, at = string(data, 10)
    path, at = string(data, at)
    (size,) = struct.unpack_from("<Q", data, at)
    at += 20
    duration, count = struct.unpack_from("<dI", data, at)
    digest = hashlib.sha256(data[at + 8:]).hexdigest()[:16]
    written = os.stat(file).st_mtime
    print(f"{path}\t{duration:.3f}\t{count}\t{digest}\t{written:.0f}\t{size}")
