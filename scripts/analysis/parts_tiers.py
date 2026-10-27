"""Session 58 (experiment 0241): the query tiers of the pre-registration, from the store's
record headers (work/s58/store-paths.tsv), and the leave-out list of
experiment 0065's candidate folders (docs/experiments/data/0065-nas-candidates.json).
Writes work/s58/tier-{1,2,3,4}.txt and work/s58/leave-out-0065.txt."""
import json

rows = [l.rstrip("\n").split("\t") for l in open("work/s58/store-paths.tsv")]
paths = sorted(r[2] for r in rows)
d = json.load(open("docs/experiments/data/0065-nas-candidates.json"))
folders = sorted(f["folder"] for f in d["folder_edges"] if f["share_joined"] >= 0.75)
too_long = sorted(t["path"] for t in d["too_long"])
with open("work/s58/leave-out-0065.txt", "w") as out:
    out.write("".join(f + "\n" for f in folders + too_long))
def left(p):
    return any(p.startswith(f + "/") for f in folders) or p in too_long
yt = "__youtube_archivists/"
iw = yt + "i-witness-dnb/"
tiers = {
    1: [p for p in paths if "[CUT]" in p],
    2: [p for p in paths if p.startswith(iw) and "[CUT]" not in p],
    3: [p for p in paths if not p.startswith(yt) and not left(p)],
    4: [p for p in paths if p.startswith(yt) and not p.startswith(iw)],
}
for n, tier in tiers.items():
    with open(f"work/s58/tier-{n}.txt", "w") as out:
        out.write("".join(p + "\n" for p in tier))
    print(n, len(tier))
