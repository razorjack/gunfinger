"""Distinct families behind an experiment's saved data (brief 87).

    python3 -I scripts/analysis/family_counts.py KEY.json DATA.json [DATA.json ...] [--out OUT.json]

Walks each JSON file and collects every string that names a library file (an
audio file name, not a mix), grouped by the path of keys it sits under (list
positions dropped, so `mixes.score.plays.truth.asset` gathers every play's
truth). For each path it prints the strings found, the distinct files and the
distinct families at the recording and relation tiers of the key that
`gunfinger-eval families` writes. Which path holds an experiment's queries or
truths is read from the experiment's text; this script only counts. Names are
compared in Unicode NFC."""

import json
import re
import sys
import unicodedata

AUDIO = re.compile(r"\.(mp3|m4a|opus|ogg|flac|wav|aiff?)$", re.IGNORECASE)
TIERS = ("recording", "title", "relation")


def nfc(name):
    return unicodedata.normalize("NFC", name)


def is_library_file(text):
    return bool(AUDIO.search(text)) and not text.startswith(("corpus/", "/")) and "/sets/" not in text


def walk(value, path, found):
    if isinstance(value, dict):
        for key, item in value.items():
            walk(item, path + (key,), found)
    elif isinstance(value, list):
        for item in value:
            walk(item, path, found)
    elif isinstance(value, str) and is_library_file(value):
        found.setdefault(".".join(path) or "(top)", []).append(nfc(value))


def families(key, names, tier):
    return {key.get(name, {}).get(tier, name) for name in names}


def count(key, data):
    found = {}
    walk(data, (), found)
    rows = {}
    for path, names in sorted(found.items()):
        rows[path] = {
            "strings": len(names),
            "files": len(set(names)),
            **{tier: len(families(key, names, tier)) for tier in TIERS},
        }
    return rows


def load_key(path):
    files = json.load(open(path, encoding="utf-8"))["files"]
    return {nfc(f): {tier: nfc(v[tier]) for tier in TIERS} for f, v in files.items()}


def main():
    args = sys.argv[1:]
    out = None
    if "--out" in args:
        at = args.index("--out")
        out = args[at + 1]
        args = args[:at] + args[at + 2 :]
    key = load_key(args[0])
    result = {}
    for path in args[1:]:
        rows = count(key, json.load(open(path, encoding="utf-8")))
        result[path] = rows
        print(f"== {path}")
        for field, row in rows.items():
            print(
                f"  {field}: {row['strings']} strings, {row['files']} files, "
                f"{row['recording']} recording, {row['title']} title, {row['relation']} relation families"
            )
    if out:
        with open(out, "w", encoding="utf-8") as f:
            json.dump(result, f, indent=1, ensure_ascii=False)


if __name__ == "__main__":
    main()
