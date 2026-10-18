"""Decodes each file of a store listing (store_records.py) with FFmpeg's
arguments from decode.rs (mono, 8 kHz, f32le) and prints, one per line as
tab-separated values: source path, sample count and a digest of the
samples. Run on two machines, the listings tell whether their FFmpeg builds
decode the same samples. Reads the library only.

usage: decoded_digests.py <library root> <store_records.py listing> [jobs]
"""
import hashlib, os, subprocess, sys
from concurrent.futures import ThreadPoolExecutor


def digest(root, path):
    command = ["ffmpeg", "-nostdin", "-v", "error", "-threads", "1",
               "-i", os.path.join(root, path),
               "-map", "0:a:0", "-vn", "-sn", "-dn", "-ac", "1", "-ar", "8000",
               "-f", "f32le", "pipe:1"]
    samples = subprocess.run(command, stdin=subprocess.DEVNULL, capture_output=True).stdout
    return f"{path}\t{len(samples) // 4}\t{hashlib.sha256(samples).hexdigest()[:16]}"


root, listing = sys.argv[1], sys.argv[2]
jobs = int(sys.argv[3]) if len(sys.argv) > 3 else os.cpu_count()
with open(listing, encoding="utf-8") as lines:
    paths = sorted(line.split("\t", 1)[0] for line in lines)
print(subprocess.run(["ffmpeg", "-version"], capture_output=True, text=True)
      .stdout.splitlines()[0], file=sys.stderr)
with ThreadPoolExecutor(jobs) as pool:
    for line in pool.map(lambda path: digest(root, path), paths):
        print(line)
