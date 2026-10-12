"""What the verdicts by ear change in the clusters: for each verdict, whether
its two files share a cluster before (OLD) and after (NEW), as joined,
split, confirmed or not honoured; then every cluster whose members changed.

usage: python3 -I scripts/analysis/verdicts_applied.py NEW OLD VERDICTS FIRST_LINE

Verdicts from line FIRST_LINE on are the new ones; earlier lines are listed
only when their standing changed.
"""

import json
import sys


def load(path):
    with open(path) as file:
        return json.load(file)


def index(clusters):
    found = {}
    for number, members in enumerate(clusters["duplicates"]):
        for member in members:
            found[member] = number
    return found


def together(where, a, b):
    return a in where and b in where and where[a] == where[b]


def main():
    new_file, old_file, verdicts_file, first = sys.argv[1:]
    new, old = load(new_file), load(old_file)
    new_at, old_at = index(new), index(old)
    with open(verdicts_file) as file:
        lines = list(enumerate(file, 1))
    counts, older = {}, {}
    for number, line in lines:
        if line.startswith("#") or not line.strip():
            continue
        verdict, a, b = line.rstrip("\n").split("\t")
        before, after = together(old_at, a, b), together(new_at, a, b)
        same = verdict == "same"
        if after != same:
            standing = "NOT HONOURED"
        elif before == after:
            standing = "confirmed"
        else:
            standing = "joined" if same else "split"
        if number >= int(first) or standing != "confirmed":
            counted = counts if number >= int(first) else older
            counted[standing] = counted.get(standing, 0) + 1
            print(f"line {number}: {verdict}: {standing} (together before: {before}, after: {after})")
            print(f"    {a}\n    {b}")
    print("new verdicts:", counts)
    print("older verdicts whose standing is not confirmed:", older)
    old_sets = {frozenset(members) for members in old["duplicates"]}
    new_sets = {frozenset(members) for members in new["duplicates"]}
    gone = old_sets - new_sets
    came = new_sets - old_sets
    print(f"clusters: {len(old_sets)} -> {len(new_sets)}; {len(gone)} changed or gone, {len(came)} changed or new")
    for members in sorted(came, key=lambda m: sorted(m)[0]):
        before = {frozenset(o) for o in old_sets if o & members}
        print(f"  now ({len(members)}): {sorted(members)}")
        for o in before:
            print(f"    was ({len(o)}): gained {sorted(members - o)}, lost {sorted(o - members)}")
    for members in gone:
        if not any(members & n for n in came):
            print(f"  gone: {sorted(members)}")
    print("cut links:", len(old.get("cut_links", [])), "->", len(new.get("cut_links", [])))


if __name__ == "__main__":
    main()
