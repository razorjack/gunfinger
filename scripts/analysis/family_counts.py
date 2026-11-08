"""Distinct families behind an experiment's saved data (brief 87).

    python3 -I scripts/analysis/family_counts.py KEY.json DATA.json [DATA.json ...] [--out OUT.json]

Walks each JSON file and collects every string that names a library file (an
audio file name, not a mix), grouped by the path of keys it sits under (list
positions dropped, so `mixes.score.plays.truth.asset` gathers every play's
truth). For each path it prints the strings found, the distinct files and the
distinct families at each tier of the key that `gunfinger-eval families`
writes. Data in docs/experiments/data holds file names only, so a name the
key lacks is looked up by its last path component; a file name the key gives
to files of two families is counted as its own family and reported. Strings
under a key named `family` are counted as family labels. Which path holds an
experiment's queries or truths is read from the experiment's text; this
script only counts. Names are compared in Unicode NFC."""

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


def walk(value, path, found, labels):
    if isinstance(value, dict):
        for key, item in value.items():
            if key == "family" and isinstance(item, str):
                labels.setdefault(".".join(path + (key,)), []).append(item)
            else:
                walk(item, path + (key,), found, labels)
    elif isinstance(value, list):
        for item in value:
            walk(item, path, found, labels)
    elif isinstance(value, str) and is_library_file(value):
        found.setdefault(".".join(path) or "(top)", []).append(nfc(value))


def base(name):
    return name.rsplit("/", 1)[-1]


def by_name(key):
    """Each file name (last path component) with the families of the files
    of that name, at each tier."""
    names = {}
    for f, fams in key.items():
        entry = names.setdefault(base(f), {tier: set() for tier in TIERS})
        for tier in TIERS:
            entry[tier].add(fams[tier])
    return names


def family(key, names, name, tier):
    """The family of `name`, and whether its file name was ambiguous."""
    if name in key:
        return key[name][tier], False
    found = names.get(base(name))
    if found is None:
        return name, False
    if len(found[tier]) == 1:
        return next(iter(found[tier])), False
    return name, True


def count(key, data, names=None):
    names = names if names is not None else by_name(key)
    found, labels = {}, {}
    walk(data, (), found, labels)
    rows = {}
    for path, strings in sorted(found.items()):
        row = {"strings": len(strings), "files": len(set(strings))}
        for tier in TIERS:
            looked_up = [family(key, names, s, tier) for s in strings]
            row[tier] = len({fam for fam, _ in looked_up})
            if tier == "relation":
                row["ambiguous_names"] = len({s for s, (_, amb) in zip(strings, looked_up) if amb})
        rows[path] = row
    for path, values in sorted(labels.items()):
        rows[path] = {"strings": len(values), "labels": len(set(values))}
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
    names = by_name(key)
    result = {}
    for path in args[1:]:
        rows = count(key, json.load(open(path, encoding="utf-8")), names)
        result[path] = rows
        print(f"== {path}")
        for field, row in rows.items():
            if "labels" in row:
                print(f"  {field}: {row['strings']} strings, {row['labels']} family labels")
                continue
            print(
                f"  {field}: {row['strings']} strings, {row['files']} files, "
                f"{row['recording']} recording, {row['title']} title, {row['relation']} relation families"
                + (f" ({row['ambiguous_names']} ambiguous names)" if row["ambiguous_names"] else "")
            )
    if out:
        with open(out, "w", encoding="utf-8") as f:
            json.dump(result, f, indent=1, ensure_ascii=False)


if __name__ == "__main__":
    main()
