"""Looks up every path of another machine's store listings (store_records.py
and store_skips.py) under a library root on this machine, by exact name in
each folder's listing, then under another Unicode normalization; compares
sizes, and the skip notes' modification seconds, with stat; and counts the
modification times with fractions of a second and their resolution.
Reads the library only.

usage: listing_lookup.py <library root> <records listing> <skips listing>
"""
import os, sys, unicodedata
from collections import Counter
from concurrent.futures import ThreadPoolExecutor

root, records, skips = sys.argv[1:4]
mac = {}
for line in open(records, encoding="utf-8"):
    f = line.rstrip("\n").split("\t"); mac[f[0]] = ("record", int(f[5]), None)
for line in open(skips, encoding="utf-8"):
    f = line.rstrip("\n").split("\t"); mac[f[0]] = (f"skip:{f[3]}", int(f[1]), int(f[2]))

folders = sorted({os.path.dirname(p) for p in mac})
def listdir(folder):
    try: return folder, set(os.listdir(os.path.join(root, folder)))
    except OSError as e: return folder, e
with ThreadPoolExecutor(16) as pool:
    listings = dict(pool.map(listdir, folders))

def look(path):
    folder, name = os.path.dirname(path), os.path.basename(path)
    names = listings.get(folder)
    if isinstance(names, Exception) or names is None:
        # The folder itself may be listed under another form.
        return path, "folder missing", None
    if name in names:
        found = name; how = "exact"
    else:
        near = [n for n in names if unicodedata.normalize("NFC", n) == unicodedata.normalize("NFC", name)]
        if not near: return path, "missing", None
        found = near[0]; how = "other normalization"
    st = os.stat(os.path.join(root, folder, found))
    return path, how, (found, st.st_size, st.st_mtime_ns)
with ThreadPoolExecutor(16) as pool:
    results = list(pool.map(look, sorted(mac)))

how = Counter(r[1] for r in results)
print("Mac paths:", len(mac), dict(Counter(v[0] for v in mac.values())))
print("Mac paths not in NFC:", sum(unicodedata.normalize("NFC", p) != p for p in mac))
print("looked up on this mount:", dict(how))
found = [r for r in results if r[2]]
print("found names not in NFC:", sum(unicodedata.normalize("NFC", r[2][0]) != r[2][0] for r in found),
      "; not in NFD:", sum(unicodedata.normalize("NFD", r[2][0]) != r[2][0] for r in found))
print("size equal:", sum(mac[r[0]][1] == r[2][1] for r in found), "of", len(found))
skipped = [r for r in found if mac[r[0]][2] is not None]
print("skip notes: mtime seconds equal:", sum(mac[r[0]][2] == r[2][2] // 10**9 for r in skipped), "of", len(skipped))
print("mtimes with a fraction of a second:", sum(r[2][2] % 10**9 != 0 for r in found), "of", len(found))
print("fraction resolution (last nonzero digit of ns):", dict(Counter(len(str(r[2][2] % 10**9).rstrip('0')) for r in found if r[2][2] % 10**9)))
for r in results:
    if r[1] != "exact": print(" ", r[1], repr(r[0]), r[2][0] if r[2] else "")
