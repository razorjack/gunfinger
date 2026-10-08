"""The names in each duplicate cluster, and the clusters whose members'
titles differ once case, punctuation and bracketed words are set aside:
candidates for a cluster that joins different recordings.

usage: python3 -I scripts/analysis/cluster_titles.py CLUSTERS TAGS MAP [OLD_CLUSTERS]

TAGS maps NAS library paths to {artist, title}; MAP is the harness's
library-map.json (corpus files that stand for NAS files). With
OLD_CLUSTERS, members that are new since then are marked. With OUT set,
writes the result as JSON there.
"""

import json
import os
import re
import sys

PREFIX = "second-library/"


def load(path):
    with open(path) as file:
        return json.load(file)


def main():
    clusters = load(sys.argv[1])
    tags = load(sys.argv[2])
    stands_for = {copy["corpus"]: copy["stands_for"] for copy in load(sys.argv[3])["copies"]}
    old = set()
    if len(sys.argv) > 4:
        old = {member for members in load(sys.argv[4])["duplicates"] for member in members}

    def name(path):
        other = path[len(PREFIX):] if path.startswith(PREFIX) else stands_for.get(path)
        fields = tags.get(other or "", {})
        if "title" in fields:
            return fields.get("artist", ""), fields["title"]
        stem = os.path.splitext(os.path.basename(path))[0].replace("_", " ")
        artist, _, title = stem.partition(" - ")
        return (artist, title) if title else ("", stem)

    def title_key(title):
        title = re.sub(r"[\(\[].*?[\)\]]", " ", title.lower())
        title = re.sub(r"\b(original mix|remastered|remaster|drum and bass|hq)\b", " ", title)
        return re.sub(r"[^a-z0-9]+", "", title)

    result = []
    for members in clusters["duplicates"]:
        names = [(member, *name(member)) for member in members]
        keys = {title_key(title) for _, _, title in names if title_key(title)}
        # One title contained in another ("nightlore" in "nightlore vip")
        # counts as the same; only titles that share nothing differ.
        distinct = [key for key in keys if not any(key != other and (key in other or other in key) for other in keys)]
        differs = len(distinct) > 1
        new = [member for member in members if member not in old]
        result.append({"members": len(members), "new": len(new), "titles_differ": differs,
                       "names": [{"path": path, "artist": artist, "title": title, "new": path not in old}
                                 for path, artist, title in names]})
    differing = [cluster for cluster in result if cluster["titles_differ"]]
    print(f"{len(result)} clusters, {sum(c['members'] for c in result)} files; "
          f"{sum(c['new'] for c in result)} members new; {len(differing)} with titles that differ, "
          f"{sum(1 for c in differing if c['new'])} of them with new members (listed)")
    for cluster in differing:
        if old and not cluster["new"]:
            continue
        print("--")
        for entry in cluster["names"]:
            mark = "new " if entry["new"] else "    "
            print(f"  {mark}{entry['artist']} - {entry['title']}  [{entry['path']}]")
    if "OUT" in os.environ:
        with open(os.environ["OUT"], "w") as file:
            json.dump(result, file, indent=1)


if __name__ == "__main__":
    main()
