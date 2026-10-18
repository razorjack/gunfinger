"""Checks that every NAS file named in the verdicts file and in the given
set manifests (`second-library/<path>`) exists under the library root with
exactly that name: each path component must appear byte for byte in its
folder's listing, so a share that matches names regardless of case or
Unicode form does not hide a difference. Reads the library only.

usage: reference_paths.py <library root> <pair-verdicts.txt> [<tracklist.toml> ...]
"""
import os, sys, tomllib, unicodedata

PREFIX = "second-library/"


def references(verdicts, manifests):
    found = {}
    with open(verdicts, encoding="utf-8") as lines:
        for number, line in enumerate(lines, 1):
            if line.startswith("#") or not line.strip():
                continue
            for field in line.rstrip("\n").split("\t")[1:]:
                if field.startswith(PREFIX):
                    found.setdefault(field[len(PREFIX):], []).append(f"{verdicts}:{number}")
    for manifest in manifests:
        with open(manifest, "rb") as file:
            for track in tomllib.load(file).get("track", []):
                for reference in track.get("reference", []):
                    if reference.startswith(PREFIX):
                        found.setdefault(reference[len(PREFIX):], []).append(
                            f"{manifest}: track {track.get('position')}")
    return found


listings = {}


def listing(folder):
    if folder not in listings:
        try:
            listings[folder] = set(os.listdir(folder))
        except OSError:
            listings[folder] = None
    return listings[folder]


def check(root, path):
    folder = root
    for part in path.split("/"):
        names = listing(folder)
        if names is None:
            return f"folder cannot be listed: {folder}"
        if part not in names:
            near = [name for name in names
                    if unicodedata.normalize("NFC", name).casefold()
                    == unicodedata.normalize("NFC", part).casefold()]
            return f"no entry {part!r} in {folder}" + (f"; listed as {near!r}" if near else "")
        folder = os.path.join(folder, part)
    return None


root, verdicts, manifests = sys.argv[1], sys.argv[2], sys.argv[3:]
found = references(verdicts, manifests)
missing = 0
for path in sorted(found):
    problem = check(root, path)
    if problem:
        missing += 1
        print(f"MISSING {path}\n  {problem}\n  named in {', '.join(found[path])}")
print(f"{len(found)} paths named, {len(found) - missing} found byte for byte, {missing} missing")
