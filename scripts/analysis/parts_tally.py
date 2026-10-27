"""Session 58 (experiment 0241): the experiment's tally and the listening pack's choice, by
the pre-registration's kinds (a guess from each candidate's parts and its
folder, for counting only). Reads work/s58/out/tier-*.json; writes
work/s58/pack-files.txt (at most 20 candidates worth the ear) and prints
the counts. Name- and folder-derived, so it lives outside the harness."""
import glob, json, collections

P = "second-library/"
YT = "__youtube_archivists/"
leave = [l.rstrip("\n") for l in open("work/s58/leave-out-0065.txt") if l.strip()]
controls = ["va-listen_up-hlcd9-2000-sour/"] + [f"/{f}/" for f in (
    "underfire_recordings-blazin-cd_rip-1999-sour", "ram_trilogy-molten_beats-1999-sour",
    "bad_company-inside_the_machine-vdbcrcd001-2000-sour", "21st_century_grooves-pivotal4cd-2000-sour",
    "ray_keith_presents_vintage_dread_2000-2000-sour", "va-tayla_presents_soul_survivors-nexuscd001-2000-sour",
    "va-essential_rewindz-mixed_by_grooverider-rhlpcd03-2000-sour")]

def in_0065(path):
    return any(path == f or path.startswith(f + "/") for f in leave)

def kind(c):
    path = c["file"][len(P):]
    roles = {p["role"] for p in c["parts"]}
    if not path.startswith(YT):
        return "mixed CD's track"
    if "[CUT]" in path or {"start", "end"} <= roles:
        return "cut from a mix"
    if "start" in roles:
        return "opening with another record"
    return "other"

reports = sorted(glob.glob("work/s58/out/tier-*.json"))
queries = 0
candidates = []
for name in reports:
    r = json.load(open(name))
    queries += r["queries"]
    for c in r["candidates"]:
        c["tier"] = name.split("/")[-1].split("-")[1].split(".")[0]
        candidates.append(c)
print(f"{len(reports)} reports, {queries} files searched, {len(candidates)} candidates, {sum(c['worth_the_ear'] for c in candidates)} worth the ear")
table = collections.Counter((kind(c), c["worth_the_ear"]) for c in candidates)
for k in ("cut from a mix", "mixed CD's track", "opening with another record", "other"):
    print(f"  {k}: {table[(k, True)] + table[(k, False)]} ({table[(k, True)]} worth the ear)")
cut = [c for c in candidates if "[CUT]" in c["file"]]
print(f"[CUT] candidates: {len(cut)} ({sum(c['worth_the_ear'] for c in cut)} worth the ear)")
both = [c for c in candidates if {"start", "end"} <= {p["role"] for p in c["parts"]}]
second = [c for c in candidates if any(p["role"] == "elsewhere" for p in c["parts"])]
print(f"edge parts at both ends: {len(both)}; with a second part elsewhere: {len(second)}")
ctrl = [c for c in candidates if any(s in c["file"] for s in controls)]
print(f"controls that are candidates: {len(ctrl)} ({sum(c['worth_the_ear'] for c in ctrl)} worth the ear)")
for c in ctrl:
    print("   ", c["strength"], c["worth_the_ear"], c["file"][len(P):])
touch = [c for c in candidates if any(in_0065(p["found"][len(P):]) for p in c["parts"] if p["role"] != "other")]
print(f"with an edge or second part in a 0065 folder: {len(touch)}")

pool = [c for c in candidates if c["worth_the_ear"] and c not in ctrl and c not in touch]
by_kind = collections.OrderedDict((k, []) for k in ("cut from a mix", "mixed CD's track", "opening with another record", "other"))
for c in pool:
    by_kind[kind(c)].append(c)
for k, cs in by_kind.items():
    cs.sort(key=lambda c: (k == "cut from a mix" and "[CUT]" not in c["file"], -c["strength"], c["file"]))
pack = []
while len(pack) < 20 and any(by_kind.values()):
    for cs in by_kind.values():
        if cs and len(pack) < 20:
            pack.append(cs.pop(0))
with open("work/s58/pack-files.txt", "w") as out:
    out.write("".join(c["file"] + "\n" for c in pack))
print(f"pack: {len(pack)}")
for c in pack:
    print(f"  {kind(c):28} {c['strength']:5} {c['file'][len(P):]}")

# Post hoc, for reading only (not pre-registered): whether a candidate's
# strongest edge part is of a file whose name shares a word of five or more
# letters with the candidate's (another version of the same tune, most
# likely), apart from common words.
import re
COMMON = {"remix", "bootleg", "original", "records", "recordings", "drum", "bass", "music", "version",
          "remaster", "remastered", "unreleased", "feat", "featuring", "dubplate", "company", "optical"}
def words(path):
    stem = path.rsplit("/", 1)[-1].rsplit(".", 1)[0].lower()
    return {w for w in re.findall(r"[a-z]{5,}", stem) if w not in COMMON}
def same_tune(c):
    edge = max((p for p in c["parts"] if p["role"] in ("start", "end")), key=lambda p: p["hits"])
    return bool(words(c["file"]) & words(edge["found"]))
st = [c for c in candidates if same_tune(c)]
print(f"post hoc: strongest edge part of a file sharing a title word: {len(st)} of {len(candidates)} ({sum(c['worth_the_ear'] for c in st)} worth the ear); in the pack: {sum(same_tune(c) for c in pack)}")
for c in sorted((c for c in candidates if not same_tune(c) and c["worth_the_ear"]), key=lambda c: -c["strength"]):
    edge = max((p for p in c["parts"] if p["role"] in ("start", "end")), key=lambda p: p["hits"])
    print(f"   different name: {kind(c):28} {c['strength']:5} {edge['role']:5} {edge['end_seconds']-edge['start_seconds']:4.0f} s  {c['file'][len(P):]}  <-  {edge['found'][len(P):]}")
different = sorted((c for c in candidates if not same_tune(c) and c["worth_the_ear"] and c not in ctrl and c not in touch), key=lambda c: -c["strength"])
with open("work/s58/different-name-files.txt", "w") as out:
    out.write("".join(c["file"] + "\n" for c in different))
print(f"post hoc list written: {len(different)} files (work/s58/different-name-files.txt)")

# Experiment 0241's data: every candidate with its kind, the post-hoc name
# split, and its edge parts and second parts (file names only).
data = [{"file": c["file"], "seconds": round(c["seconds"], 1), "strength": c["strength"],
         "worth_the_ear": c["worth_the_ear"], "kind": kind(c), "same_name_post_hoc": same_tune(c),
         "parts": [{k: (round(v, 2) if isinstance(v, float) else v) for k, v in p.items() if k != "cluster"}
                   for p in c["parts"] if p["role"] != "other"]}
        for c in candidates]
with open("docs/experiments/data/0241-parts-candidates.json", "w") as out:
    json.dump({"experiment": "0241", "candidates": data}, out, ensure_ascii=False, separators=(",", ":"))
