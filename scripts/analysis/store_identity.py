"""Whether this machine lists the files of a peak store's records as the
store recorded them (ADR 0020, "Open risks"). Two things may differ when
another system reaches the same library: the Unicode form of a path (a
record keyed by a decomposed name is not found under a precomposed one)
and the precision of modification times (a record whose time differs
looks out of date to `index`).

Counts the records whose paths have two Unicode forms, by the form
stored, and those whose modification time has a fraction of a second.
Then, if the library is reachable, looks up every record of the first
kind and a seeded sample of the others by listing their folders: found
under the stored bytes, only under the other form, or not at all, and
whether size and modification time match to the nanosecond. Opens the
store and the library read-only.

usage: store_identity.py <peak store> <library root> [sample size, default 20]
"""
import os, random, struct, sys, unicodedata

# Never read: the copy of the held-out test set's broadcast at the NAS root.
LEFT_OUT = ("EM_1999_02_07", "ed-rush-optical-essential-mix")


def string(data, at):
    (length,) = struct.unpack_from("<H", data, at)
    at += 2
    return data[at:at + length].decode("utf-8"), at + length


def source(data):
    """The record's path, size and modification time in nanoseconds."""
    _, at = string(data, 10)
    path, at = string(data, at)
    size, seconds, nanos = struct.unpack_from("<QqI", data, at)
    return path, size, seconds * 1_000_000_000 + nanos


def record(file):
    with open(file, "rb") as stored:
        data = stored.read(4096)
        if data[:8] != b"GUNFPEAK":
            return None
        try:
            return source(data)
        except (struct.error, UnicodeDecodeError):
            return source(data + stored.read())


def nfc(text):
    return unicodedata.normalize("NFC", text)


def nfd(text):
    return unicodedata.normalize("NFD", text)


listings = {}


def listed(folder):
    if folder not in listings:
        try:
            listings[folder] = set(os.listdir(folder))
        except OSError:
            listings[folder] = set()
    return listings[folder]


def find(root, path):
    """The file as this machine lists it, and how its name matched."""
    here, how = root, "exact"
    for part in path.split("/"):
        names = listed(here)
        if part in names:
            here = os.path.join(here, part)
            continue
        same = sorted(name for name in names if nfc(name) == nfc(part))
        if not same:
            return None, "missing"
        here, how = os.path.join(here, same[0]), "other form"
    return here, how


store, root = sys.argv[1], sys.argv[2]
sample_size = int(sys.argv[3]) if len(sys.argv) > 3 else 20

records = []
for entry in sorted(os.listdir(store)):
    if entry.endswith(".peaks"):
        found = record(os.path.join(store, entry))
        if found:
            records.append(found)

two_forms = [r for r in records if nfc(r[0]) != nfd(r[0])]
stored_nfd = sum(1 for r in two_forms if r[0] == nfd(r[0]))
stored_nfc = sum(1 for r in two_forms if r[0] == nfc(r[0]))
fractional = sum(1 for r in records if r[2] % 1_000_000_000)
print(f"{len(records)} records in {store}")
print(f"{len(two_forms)} paths with two Unicode forms: {stored_nfd} stored decomposed (NFD), "
      f"{stored_nfc} precomposed (NFC), {len(two_forms) - stored_nfd - stored_nfc} mixed")
print(f"{fractional} modification times with a fraction of a second")

if not os.path.isdir(root):
    print(f"the library at {root} is not reachable here; nothing looked up")
    sys.exit(0)


def allowed(chosen):
    return [r for r in chosen if not any(name in r[0] for name in LEFT_OUT)]


others = allowed(r for r in records if nfc(r[0]) == nfd(r[0]))
sample = random.Random(2026).sample(others, min(sample_size, len(others)))
looked_up = allowed(two_forms) + sample
matches = {"exact": 0, "other form": 0, "missing": 0}
same, differs = 0, []
for path, size, nanoseconds in looked_up:
    file, how = find(root, path)
    matches[how] += 1
    if file is None:
        continue
    try:
        status = os.stat(file)
    except OSError:
        matches[how] -= 1
        matches["missing"] += 1
        continue
    if status.st_size == size and status.st_mtime_ns == nanoseconds:
        same += 1
    else:
        differs.append((path, size, nanoseconds, status.st_size, status.st_mtime_ns))

print(f"looked up {len(looked_up)} files under {root} "
      f"({len(looked_up) - len(sample)} with two forms, {len(sample)} sampled):")
print(f"  found under the stored path: {matches['exact']}; only under the other Unicode form: "
      f"{matches['other form']}; not found: {matches['missing']}")
print(f"  size and modification time as recorded: {same}; different: {len(differs)}")
for path, size, recorded, listed_size, listed_time in differs[:5]:
    print(f"    {path}: recorded {size} bytes at {recorded} ns, listed {listed_size} bytes at {listed_time} ns")
