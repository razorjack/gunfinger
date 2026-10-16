"""Which files that `versions` and `identifiability` rendered lie on
another timeline when FFmpeg seeks than when they are decoded from the
start, as indexing decodes them (session 25, step 3c; experiment 0127
found Do Drop In's decoded audio 235 ms ahead of its timestamps).

For every file and every position a run rendered it from (rounded to 1 s),
2 s are rendered twice through the harness's chain at native speed: as
`render_samples` does (FFmpeg's `-ss` before `-i`) and as
`render_samples_as_indexed` does (decoded from the start, cut by sample
count). Where the two agree (correlation at lag 0 of 0.99 or more) the
position is unaffected; otherwise the lag that aligns them is found on a
1 kHz envelope and refined at 8 kHz. Positions are where a segment's
render starts: the reference's file time 1 s before the segment for
`versions` (and the mix's own time for the mixes, which the core's decoder
excerpts with a seek), the line's start in the file for `identifiability`.

usage: render_timeline.py OUT [--plays PLAYS.json]... [--identifiability REPORT.json]
       [--corpus DIR] [--other DIR] [--jobs N]
"""
import json
import math
import struct
import subprocess
import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

PREFIX = "second-library/"
RATE = 8000
SECONDS = 2.0
CHAIN = "aresample=44100,asetrate=44100,aresample=44100,aresample=8000"


def arguments(argv):
    out = Path(argv[0])
    plays, reports, jobs = [], [], 4
    corpus = Path("corpus/library")
    other = Path("/Volumes/atlas/Music/dnb")
    rest = iter(argv[1:])
    for flag in rest:
        value = next(rest)
        if flag == "--plays":
            plays.append(Path(value))
        elif flag == "--identifiability":
            reports.append(Path(value))
        elif flag == "--corpus":
            corpus = Path(value)
        elif flag == "--other":
            other = Path(value)
        elif flag == "--jobs":
            jobs = int(value)
        else:
            sys.exit(f"unknown option {flag}")
    return out, plays, reports, corpus, other, jobs


def path_of(name, corpus, other):
    if name.startswith("/") or name.startswith("work/"):
        return Path(name)
    if name.startswith(PREFIX):
        return other / name[len(PREFIX):]
    return corpus / name


def positions(plays_files, reports):
    """{(role, file name): {position rounded to 1 s}} for every render."""
    found = {}
    for plays_file in plays_files:
        for play in json.loads(plays_file.read_text())["plays"]:
            for segment in play["segments"]:
                file_at = segment["track_start_seconds"] - segment["speed"] * 1.0
                found.setdefault(("reference", play["reference"]), set()).add(
                    round(max(file_at, 0.0)))
                found.setdefault(("mix", play["mix"]), set()).add(
                    round(max(segment["start_seconds"] - 1.0, 0.0)))
    for report in reports:
        for direction in json.loads(report.read_text())["directions"]:
            for measured in direction["files"]:
                line = measured.get("found")
                if line:
                    found.setdefault(("reference", measured["file"]), set()).add(
                        round(max(line["file_start_seconds"], 0.0)))
    return found


def skip_bytes(path):
    """`Container::of`: the ID3 tag's length when a RIFF header follows it."""
    try:
        with open(path, "rb") as file:
            header = file.read(10)
            if header[:3] != b"ID3":
                return None
            size = 0
            for byte in header[6:10]:
                size = (size << 7) | (byte & 0x7F)
            tag = 10 + size + (10 if header[5] & 0x10 else 0)
            file.seek(tag)
            return tag if file.read(4) == b"RIFF" else None
    except OSError:
        return None


def render(path, start, as_indexed):
    command = ["ffmpeg", "-nostdin", "-v", "error"]
    chain = CHAIN
    if as_indexed:
        chain = f"asetpts=N/SR/TB,atrim=start={start:.6f},asetpts=PTS-STARTPTS,{CHAIN}"
    else:
        command += ["-ss", f"{start:.3f}"]
    tag = skip_bytes(path)
    if tag is not None:
        command += ["-skip_initial_bytes", str(tag)]
    command += ["-i", str(path), "-map", "0:a:0", "-ac", "1", "-af", chain,
                "-t", f"{SECONDS:.3f}", "-f", "f32le", "pipe:1"]
    done = subprocess.run(command, capture_output=True)
    if done.returncode != 0:
        raise RuntimeError(done.stderr.decode(errors="replace").strip()[:300])
    data = done.stdout
    return list(struct.unpack(f"<{len(data) // 4}f", data[: len(data) // 4 * 4]))


def correlation(a, b):
    n = min(len(a), len(b))
    if n < 100:
        return 0.0
    a, b = a[:n], b[:n]
    ma, mb = sum(a) / n, sum(b) / n
    sab = sum((x - ma) * (y - mb) for x, y in zip(a, b))
    saa = sum((x - ma) ** 2 for x in a)
    sbb = sum((y - mb) ** 2 for y in b)
    return sab / math.sqrt(saa * sbb) if saa > 0 and sbb > 0 else 0.0


def envelope(samples, step=8):
    return [sum(abs(x) for x in samples[i:i + step]) for i in range(0, len(samples) - step + 1, step)]


def lag_ms(sought, indexed, path, start):
    """Where the seek's audio lies on the decoded timeline, in ms from
    `start` (negative: earlier, as after dropped frames): the decoded
    render is taken again from 0.5 s earlier and 1 s longer, and the seek's
    render placed where it fits best."""
    early = max(start - 0.5, 0.0)
    wide = render_long(path, early)
    coarse_s, coarse_w = envelope(sought), envelope(wide)
    best, best_r = 0, -2.0
    for shift in range(0, max(len(coarse_w) - len(coarse_s), 0) + 1):
        r = correlation(coarse_s, coarse_w[shift:shift + len(coarse_s)])
        if r > best_r:
            best, best_r = shift, r
    fine, fine_r = best * 8, -2.0
    for shift in range(max(best * 8 - 16, 0), best * 8 + 17):
        r = correlation(sought, wide[shift:shift + len(sought)])
        if r > fine_r:
            fine, fine_r = shift, r
    return round((early + fine / RATE - start) * 1000.0, 1), round(fine_r, 4)


def render_long(path, start):
    command = ["ffmpeg", "-nostdin", "-v", "error"]
    tag = skip_bytes(path)
    if tag is not None:
        command += ["-skip_initial_bytes", str(tag)]
    chain = f"asetpts=N/SR/TB,atrim=start={start:.6f},asetpts=PTS-STARTPTS,{CHAIN}"
    command += ["-i", str(path), "-map", "0:a:0", "-ac", "1", "-af", chain,
                "-t", f"{SECONDS + 1.0:.3f}", "-f", "f32le", "pipe:1"]
    data = subprocess.run(command, capture_output=True, check=True).stdout
    return list(struct.unpack(f"<{len(data) // 4}f", data[: len(data) // 4 * 4]))


def check(item, corpus, other):
    (role, name), at = item
    path = path_of(name, corpus, other)
    rows = []
    for start in sorted(at):
        try:
            sought = render(path, float(start), as_indexed=False)
            indexed = render(path, float(start), as_indexed=True)
        except (RuntimeError, OSError) as error:
            rows.append({"start": start, "error": str(error)})
            continue
        # The first 0.1 s may differ where the decoder starts after a seek.
        r0 = correlation(sought[800:], indexed[800:])
        row = {"start": start, "r_at_lag_0": round(r0, 4)}
        if r0 < 0.99:
            row["seek_offset_ms"], row["r_at_lag"] = lag_ms(sought, indexed, path, float(start))
        rows.append(row)
    return {"role": role, "file": name, "positions": rows}


def main(argv):
    out, plays_files, reports, corpus, other, jobs = arguments(argv)
    found = positions(plays_files, reports)
    with ThreadPoolExecutor(jobs) as pool:
        results = list(pool.map(lambda item: check(item, corpus, other), sorted(found.items())))
    affected = [r for r in results if any("seek_offset_ms" in p for p in r["positions"])]
    errors = [r for r in results if any("error" in p for p in r["positions"])]
    summary = {
        "files": len(results),
        "positions": sum(len(r["positions"]) for r in results),
        "affected_files": [
            {"role": r["role"], "file": r["file"],
             "positions": [p for p in r["positions"] if "seek_offset_ms" in p]}
            for r in affected],
        "errors": [{"file": r["file"], "positions": [p for p in r["positions"] if "error" in p]} for r in errors],
        "inputs": [str(p) for p in plays_files] + [str(p) for p in reports],
        "results": results,
    }
    out.write_text(json.dumps(summary, indent=1) + "\n")
    print(f"{summary['files']} files, {summary['positions']} positions; "
          f"{len(affected)} files affected, {len(errors)} with errors")
    for r in affected:
        lags = sorted({p['seek_offset_ms'] for p in r['positions'] if 'seek_offset_ms' in p})
        print(f"  {r['role']} {r['file']}: the seek lands {lags} ms from the decoded timeline")


if __name__ == "__main__":
    main(sys.argv[1:])
