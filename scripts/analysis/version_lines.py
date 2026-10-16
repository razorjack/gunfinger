"""Session 24, exploratory: how far a play's reported line lies from the
line its own hashes give locally. For each play of a `versions` play
list whose id contains ID_PART, 25 s excerpts of the mix at POINTS
places in each segment are identified (`gunfinger identify --store-only
--saved-index off --playback turntable`) against a scratch store holding
the reference's record alone; each excerpt's line is compared with the
reported segment's line at the excerpt's middle, in mix milliseconds
(positive: the audio plays later than the reported line says).

usage:
  version_lines.py PLAYS ID_PART [POINTS] OUT_DIR

Reads the mixes, the reference records and the peak stores; writes only
under OUT_DIR (scratch stores, one excerpt at a time) and prints.
"""

import json
import shutil
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
CORPUS_STORE = ROOT / "work/peaks"
NAS_STORE = Path.home() / ".local/share/gunfinger/nas-dnb-peaks"
PREFIX = "second-library/"
GUNFINGER = ROOT / "work/bin/s24-start/gunfinger"
EXCERPT_SECONDS = 25.0


def record_name(name):
    """The store's file name for a source: FNV-1a 64 of its path."""
    relative = name[len(PREFIX):] if name.startswith(PREFIX) else name
    digest = 0xCBF29CE484222325
    for byte in relative.encode():
        digest = ((digest ^ byte) * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
    return f"{digest:016x}"


def scratch_store(name, directory):
    """A store holding `name`'s record alone (copied, never written back)."""
    if directory.exists():
        shutil.rmtree(directory)
    directory.mkdir(parents=True)
    store = NAS_STORE if name.startswith(PREFIX) else CORPUS_STORE
    for extension in ("peaks", "tags"):
        source = store / f"{record_name(name)}.{extension}"
        if source.exists():
            shutil.copy(source, directory / source.name)
    if not (directory / f"{record_name(name)}.peaks").exists():
        raise SystemExit(f"no record of {name}")
    return directory


def local_lines(mix, start, store, out_dir):
    excerpt = out_dir / "excerpt.wav"
    subprocess.run(["ffmpeg", "-nostdin", "-v", "error", "-y", "-ss", f"{start:.3f}", "-i", str(mix),
                    "-map", "0:a:0", "-ac", "1", "-t", f"{EXCERPT_SECONDS}", str(excerpt)], check=True)
    output = subprocess.run(
        [str(GUNFINGER), "--config", str(out_dir / "empty.toml"), "identify", str(excerpt), "--peaks-dir",
         str(store), "--store-only", "--saved-index", "off", "--playback", "turntable", "-f", "json"],
        capture_output=True, check=True, text=True).stdout
    return [s for p in json.loads(output)["plays"] for s in p["segments"]]


def file_time(segment, t):
    return segment["track_start_seconds"] + segment["speed"] * (t - segment["start_seconds"])


def main(plays_path, id_part, *rest):
    *points, out_dir = rest
    points = int(points[0]) if points else 3
    out_dir = Path(out_dir)
    out_dir.mkdir(parents=True, exist_ok=True)
    (out_dir / "empty.toml").write_text("")
    for play in json.load(open(plays_path, encoding="utf-8"))["plays"]:
        if id_part not in play["id"]:
            continue
        print(f"== {play['id']}")
        store = scratch_store(play["reference"], out_dir / "store")
        for segment in play["segments"]:
            first, last = segment["start_seconds"], segment["end_seconds"] - EXCERPT_SECONDS
            print(f"   segment {first:8.2f}-{segment['end_seconds']:8.2f} speed {segment['speed']:.5f}")
            for k in range(points if last > first else 0):
                start = first + (last - first) * k / max(1, points - 1)
                for found in local_lines(play["mix"], start, store, out_dir):
                    t = start + (found["start_seconds"] + found["end_seconds"]) / 2
                    found = {**found, "start_seconds": start + found["start_seconds"]}
                    offset = (file_time(segment, t) - file_time(found, t)) / segment["speed"]
                    print(f"     t {t:8.1f}  hits {found['hits']:5d}  speed {found['speed']:.5f}  "
                          f"audio minus reported line {1000 * offset:+7.1f} ms")


if __name__ == "__main__":
    main(*sys.argv[1:])
