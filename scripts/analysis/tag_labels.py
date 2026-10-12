"""How two library files are related by their tags: another rip by name,
a version of the same title, the same artist, or unrelated. Uploads tag
artists and remixers in their own ways, so this is a hint, not evidence.

`label(query, found, tags, stands_for)`: `tags` maps a NAS store path to
its artist and title (`store_tags.py`); `stands_for` maps a corpus file to
the NAS copy it stands for (`library-map.json`). A path such as
` 3. Artist - Title` (a manifest label) is read as artist and title.
"""

import os
import re

PREFIX = "second-library/"
SPLIT = re.compile(r"\s*(?:&|,|\+|/|\band\b|\bfeat\.?|\bft\.?|\bvs\.?|\bx\b)\s*")
NEUTRAL = re.compile(r"\b(remaster(ed)?|original( mix)?|hq audio|drum and bass)\b")
VERSION = re.compile(r"\b(vip|remix|rmx|mix|edit|dub|bootleg|refix|rework|cut|dubstyle)\b")


def name(path, tags, stands_for):
    label = re.match(r"\s*\d+\.\s+(.*)", path)
    if label:
        artist, _, title = label.group(1).partition(" - ")
        return artist, title
    other = path[len(PREFIX):] if path.startswith(PREFIX) else stands_for.get(path)
    fields = tags.get(other or "", {})
    if "artist" in fields and "title" in fields:
        return fields["artist"], fields["title"]
    stem = os.path.splitext(os.path.basename(path))[0].replace("_", " ")
    artist, _, title = stem.partition(" - ")
    return (artist, title) if title else ("", stem)


def artists(text):
    return {p.strip() for p in SPLIT.split(text.lower()) if len(p.strip()) > 1}


def key(title, words):
    title = words.sub(" ", NEUTRAL.sub(" ", title.lower()))
    return re.sub(r"[^a-z0-9]+", "", title)


def base(title):
    return key(re.sub(r"[\(\[].*?[\)\]]", " ", title), VERSION)


def words_in(full, part):
    return part and part in re.sub(r"[^a-z0-9]+", "", full.lower())


def label(query, found, tags, stands_for):
    (qa, qt), (fa, ft) = name(query, tags, stands_for), name(found, tags, stands_for)
    same_artist = bool(artists(qa) & artists(fa)) or words_in(fa + ft, re.sub(r"[^a-z0-9]+", "", qa.lower()))
    qk, fk = key(qt, re.compile(r"$^")), key(ft, re.compile(r"$^"))
    if qk and (qk == fk or (same_artist and qk in fk and VERSION.search(ft.lower()) is None)):
        return "same name"
    if base(qt) and (base(qt) == base(ft) or base(qt) in key(ft, re.compile(r"$^"))):
        return "version"
    if same_artist:
        return "same artist"
    return "unrelated"
