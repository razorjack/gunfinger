"""Session 59 (experiment 0242): the fullest-1% cut of a saved index's whole offsets table
(ADR 0014's file format), read only: non-empty lists, lists set aside, the
longest kept, and how many lists hold each length near the cut."""
import struct, sys
from array import array

OFFSETS = (1 << 21) + 1
for path in sys.argv[1:]:
    with open(path, "rb") as f:
        assert f.read(8) == b"GFINDEX\0"
        f.read(4)
        for _ in range(2):
            (n,) = struct.unpack("<H", f.read(2)); f.read(n)
        f.read(4 + 16)
        (count,) = struct.unpack("<I", f.read(4))
        for _ in range(count):
            (n,) = struct.unpack("<H", f.read(2)); f.read(n + 8 + 8 + 4 + 8)
        offsets = array("I"); offsets.frombytes(f.read(4 * OFFSETS))
    lengths = [offsets[i + 1] - offsets[i] for i in range(OFFSETS - 1)]
    values = sorted((v for v in lengths if v > 0), reverse=True)
    dropped = int(0.01 * len(values) + 0.5)  # as Rust's f64::round
    kept = values[dropped]
    aside = sum(1 for v in values if v > kept)
    near = {v: values.count(v) for v in range(kept - 2, kept + 3)}
    print(f"{path}: {count} assets, {len(values)} lists, dropped {dropped}, longest kept {kept}, set aside {aside}, lists by length near the cut {near}, hash 1286800: {lengths[1286800]}")
