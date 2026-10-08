"""How two library files are related, from their names: the same artist
and title (another rip), a version of the same title (remix, VIP), the
same artist, or unrelated. Shared by the analysis scripts.

`name(path, tags, stands_for)` reads a manifest label (" 1. Artist -
Title"), the NAS store's tags for a second-library file or a corpus file's
NAS copy, or else the file name.
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


def named_in(names, full):
    """Whether any artist in `names` (four letters or more) appears in `full`,
    as in scene file names such as `aa-kemal-mechanizm-sour`."""
    return any(len(n) >= 4 and words_in(full, re.sub(r"[^a-z0-9]+", "", n)) for n in names)


def label(query_name, found_name):
    (qa, qt), (fa, ft) = query_name, found_name
    same_artist = (bool(artists(qa) & artists(fa)) or words_in(fa + ft, re.sub(r"[^a-z0-9]+", "", qa.lower()))
                   or named_in(artists(qa), fa + ft) or named_in(artists(fa), qa + qt))
    nothing = re.compile(r"$^")
    qk, fk = key(qt, nothing), key(ft, nothing)
    if qk and (qk == fk or (same_artist and qk in fk and VERSION.search(ft.lower()) is None)):
        return "same name"
    if base(qt) and (base(qt) == base(ft) or base(qt) in key(ft, nothing)):
        return "version"
    if same_artist:
        return "same artist"
    return "unrelated"
