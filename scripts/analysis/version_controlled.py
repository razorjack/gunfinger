"""Session 24, step 7: the external report's controlled test for the
version verifier (docs/brief-24.md, "Step 7"). For a panel direction
(played file P, related file R, P's other rip P2, the shared passage):
P's passage followed by N s of P's own material after it, audible,
attenuated (-12 dB) or cut away (the panel's blend partner at the
material's level); each query identified against R alone (the
negative) and P2 alone (the positive) with `gunfinger identify
--store-only --saved-index off` on a scratch store holding one record;
the plays written for `gunfinger-eval versions` as found and, as an
exploratory second list, with the longest segment's line extended over
the whole query.

usage:
  version_controlled.py prepare PLAN DIRECTION... OUT_DIR
  version_controlled.py report OUT_DIR

Reads the panel's files and the peak stores; writes only under OUT_DIR.
Nothing here reads the held-out test set.
"""

import json
import math
import shutil
import struct
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CORPUS = ROOT / "corpus/library"
CORPUS_STORE = ROOT / "work/peaks"
NAS = Path("/Volumes/atlas/Music/dnb")
NAS_STORE = Path.home() / ".local/share/gunfinger/nas-dnb-peaks"
PREFIX = "second-library/"
GUNFINGER = ROOT / "work/bin/s24-start/gunfinger"
PARTNER = ("Skynet & Stakka - Decoy.mp3", 60.0)
LENGTHS = (0, 5, 10, 20)
CONDITIONS = ("audible", "attenuated", "cut away")
ATTENUATION = 0.25  # -12 dB
RATE = 44100


def path(name):
    return NAS / name[len(PREFIX):] if name.startswith(PREFIX) else CORPUS / name


def record_name(name):
    """The store's file name for a source: FNV-1a 64 of its path."""
    relative = name[len(PREFIX):] if name.startswith(PREFIX) else name
    digest = 0xCBF29CE484222325
    for byte in relative.encode():
        digest = ((digest ^ byte) * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
    return f"{digest:016x}"


def scratch_store(name, directory):
    """A store holding `name`'s record alone (copied, never written back)."""
    directory.mkdir(parents=True, exist_ok=True)
    store = NAS_STORE if name.startswith(PREFIX) else CORPUS_STORE
    for extension in ("peaks", "tags"):
        source = store / f"{record_name(name)}.{extension}"
        if source.exists():
            shutil.copy(source, directory / source.name)
    if not (directory / f"{record_name(name)}.peaks").exists():
        raise SystemExit(f"no record of {name}")
    return directory


def decode(file, start, seconds):
    output = subprocess.run(
        ["ffmpeg", "-nostdin", "-v", "error", "-ss", f"{start:.3f}", "-i", str(file), "-t", f"{seconds:.3f}",
         "-ac", "1", "-ar", str(RATE), "-f", "f32le", "pipe:1"], capture_output=True, check=True).stdout
    return list(struct.unpack(f"<{len(output) // 4}f", output))


def rms(samples):
    return math.sqrt(sum(x * x for x in samples) / max(1, len(samples)))


def encode(samples, out):
    data = struct.pack(f"<{len(samples)}f", *samples)
    subprocess.run(["ffmpeg", "-nostdin", "-v", "error", "-y", "-f", "f32le", "-ar", str(RATE), "-ac", "1",
                    "-i", "pipe:0", "-c:a", "libmp3lame", "-b:a", "128k", str(out)], input=data, check=True)


def queries(direction, out_dir):
    """Each query's file and the passage's length in it."""
    played = path(direction["played"])
    start, end = direction["passage_start_seconds"], direction["passage_end_seconds"]
    passage = decode(played, start, end - start)
    made = []
    for length in LENGTHS:
        context = decode(played, end, length) if length else []
        for condition in CONDITIONS if length else ("audible",):
            if condition == "audible":
                tail = context
            elif condition == "attenuated":
                tail = [x * ATTENUATION for x in context]
            else:
                partner = decode(path(PARTNER[0]), PARTNER[1], length)
                scale = rms(context) / max(rms(partner), 1e-9)
                tail = [x * scale for x in partner]
            name = f"{direction['name']}-{length}s-{condition.replace(' ', '-')}"
            out = out_dir / f"{name}.mp3"
            encode(passage + tail, out)
            made.append({"name": name, "file": str(out), "length": length, "condition": condition,
                         "passage_seconds": len(passage) / RATE, "seconds": (len(passage) + len(tail)) / RATE})
    return made


def identify(query, store):
    output = subprocess.run(
        [str(GUNFINGER), "--config", str(store.parent / "empty.toml"), "identify", query, "--peaks-dir", str(store),
         "--store-only", "--saved-index", "off", "--playback", "turntable", "-f", "json"],
        capture_output=True, check=True, text=True).stdout
    return json.loads(output)["plays"]


def segment(s):
    return {"start_seconds": s["start_seconds"], "end_seconds": s["end_seconds"],
            "track_start_seconds": s["track_start_seconds"], "speed": s["speed"]}


def extended(segments, seconds):
    """The longest segment's line over the whole query."""
    longest = max(segments, key=lambda s: s["end_seconds"] - s["start_seconds"])
    offset = longest["track_start_seconds"] - longest["speed"] * longest["start_seconds"]
    start = max(0.0, -offset / longest["speed"])
    return [{"start_seconds": start, "end_seconds": seconds - 0.05,
             "track_start_seconds": offset + longest["speed"] * start, "speed": longest["speed"]}]


def prepare(plan_path, *arguments):
    *names, out_dir = arguments
    out_dir = Path(out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)
    (out_dir / "empty.toml").write_text("")
    plan = {s["name"]: s for s in json.load(open(plan_path, encoding="utf-8"))["scenarios"]}
    as_found, lines, found = [], [], []
    for name in names:
        direction = plan[name]
        rip = next(f for f in direction["played_recording"] if f != direction["played"])
        candidates = {"related": direction["related"], "rip": rip}
        stores = {role: scratch_store(file, out_dir / f"store-{name}-{role}") for role, file in candidates.items()}
        for query in queries(direction, out_dir):
            for role, file in candidates.items():
                plays = [p for p in identify(query["file"], stores[role])]
                meta = {"direction": name, "role": role, **{k: query[k] for k in
                        ("name", "length", "condition", "passage_seconds", "seconds")}}
                found.append({**meta, "plays": [{"confidence": p.get("confidence"),
                              "segments": [segment(s) for s in p["segments"]],
                              "hits": max(s.get("hits", 0) for s in p["segments"])} for p in plays]})
                if not plays:
                    continue
                best = max(plays, key=lambda p: max(s.get("hits", 0) for s in p["segments"]))
                segments = [segment(s) for s in best["segments"]]
                identifier = f"ctl|{query['name']}|{role}"
                entry = {"id": identifier, "mix": query["file"], "reference": file, "siblings": [],
                         "segments": segments, "meta": meta}
                as_found.append(entry)
                lines.append({**entry, "id": identifier + "|extended",
                              "segments": extended(segments, query["seconds"])})
    json.dump({"plays": as_found}, open(out_dir / "plays.json", "w"), indent=1, ensure_ascii=False)
    json.dump({"plays": lines}, open(out_dir / "plays-extended.json", "w"), indent=1, ensure_ascii=False)
    json.dump(found, open(out_dir / "found.json", "w"), indent=1, ensure_ascii=False)
    print(f"{len(as_found)} plays found of {len(found)} queries x candidates")


def context_states(verified, passage_seconds):
    counts = {"supported": 0, "contradicted": 0, "unobservable": 0}
    for timeline in verified["segments"]:
        for patch in timeline["patches"]:
            if patch["t"] - 1.0 > passage_seconds:
                counts[patch["state"]] += 1
    return counts


def report(out_dir):
    out_dir = Path(out_dir)
    found = json.load(open(out_dir / "found.json"))
    as_built = {v["id"]: v for v in json.load(open(out_dir / "verified.json"))}
    wide = {v["id"]: v for v in json.load(open(out_dir / "verified-extended.json"))}
    rows = []
    for entry in found:
        identifier = f"ctl|{entry['name']}|{entry['role']}"
        best = max(entry["plays"], key=lambda p: p["hits"], default=None)
        built = as_built.get(identifier)
        extended_ = wide.get(identifier + "|extended")
        rows.append({**{k: entry[k] for k in ("direction", "role", "length", "condition")},
                     "confidence": best and best["confidence"], "hits": best and best["hits"],
                     "s1": built and built["summaries"]["s1"],
                     "s1 extended": extended_ and extended_["summaries"]["s1"],
                     "added material": extended_ and context_states(extended_, entry["passage_seconds"])})
    json.dump(rows, open(out_dir / "report.json", "w"), indent=1)
    share = lambda v: "  -  " if v is None else f"{v:.3f}"
    for r in rows:
        added = r["added material"] or {}
        print(f"{r['direction']:<8} {r['role']:<7} {r['length']:>2}s {r['condition']:<10} "
              f"{str(r['confidence']):<10} {str(r['hits']):>5}  S1 {share(r['s1'])}  extended {share(r['s1 extended'])}  "
              f"added: S {added.get('supported', '-')} C {added.get('contradicted', '-')} U {added.get('unobservable', '-')}")


if __name__ == "__main__":
    command, *arguments = sys.argv[1:]
    {"prepare": prepare, "report": report}[command](*arguments)
