"""Audio at the track boundaries of each release folder, from the peak
records alone: for consecutive files (in name order) of one folder, the
peak density in the last second of one and the first second of the next,
as a share of each record's mean density. On a DJ-mixed CD one track
runs into the next, so both sides of a boundary are as dense as the
music; separate tracks usually end and start in silence or a fade. A
listening list for the owner's ignore file, not a rule. Opens the store
read-only.

usage: folder_edges.py <peak store> [min files per folder]
"""
import collections, os, struct, sys

FRAMES_PER_SECOND = 8000 / 128
EDGE = 1.0  # seconds


def string(data, at):
    (length,) = struct.unpack_from("<H", data, at)
    return data[at + 2:at + 2 + length].decode("utf-8"), at + 2 + length


def varint(data, at):
    value = shift = 0
    while True:
        byte = data[at]
        at += 1
        value |= (byte & 0x7F) << shift
        if byte < 0x80:
            return value, at
        shift += 7


def edges(data):
    """Peak densities of the first and last second against the mean."""
    _, at = string(data, 10)
    _, at = string(data, at)
    at += 20
    duration, count = struct.unpack_from("<dI", data, at)
    at += 12
    frames = []
    frame = 0
    for _ in range(count):
        delta, at = varint(data, at)
        frame += delta
        frames.append(frame)
        at += 4
    if count == 0 or duration < 4 * EDGE:
        return None
    mean = count / duration
    end = duration * FRAMES_PER_SECOND
    head = sum(1 for f in frames if f < EDGE * FRAMES_PER_SECOND) / EDGE
    tail = sum(1 for f in frames if f >= end - EDGE * FRAMES_PER_SECOND) / EDGE
    return head / mean, tail / mean


store = sys.argv[1]
minimum = int(sys.argv[2]) if len(sys.argv) > 2 else 4
folders = collections.defaultdict(list)
for entry in os.listdir(store):
    if not entry.endswith(".peaks"):
        continue
    with open(os.path.join(store, entry), "rb") as file:
        data = file.read()
    _, at = string(data, 10)
    path, _ = string(data, at)
    shares = edges(data)
    if shares:
        folder, _, name = path.rpartition("/")
        folders[folder].append((name, shares))
for folder, files in sorted(folders.items()):
    if len(files) < minimum:
        continue
    files.sort()
    boundaries = [(a[1][1], b[1][0]) for a, b in zip(files, files[1:])]
    joined = sum(1 for tail, head in boundaries if tail > 0.5 and head > 0.5)
    print(f"{joined / len(boundaries):.2f}\t{joined}\t{len(boundaries)}\t{folder}")
