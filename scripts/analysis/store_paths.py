"""Session 58: list path and duration of every peak record in a store (read only)."""
import os, struct, sys
def string(data, at):
    (length,) = struct.unpack_from("<H", data, at)
    return data[at + 2:at + 2 + length].decode("utf-8"), at + 2 + length
store = sys.argv[1]
for name in sorted(os.listdir(store)):
    if not name.endswith(".peaks"):
        continue
    with open(os.path.join(store, name), "rb") as f:
        data = f.read(4096)
    a, at = string(data, 10)
    b, at = string(data, at)
    at += 20
    (duration,) = struct.unpack_from("<d", data, at)
    print(f"{duration:.1f}\t{a}\t{b}")
