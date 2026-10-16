"""Whether two harness reports of the same run (scan, sweep or mixes)
hold the same answers, as `regress` compares them, ignoring run times.

usage: python3 -I scripts/analysis/report_identity.py SAVED NEW

Prints `identical` or the first differing entries. Detections compare
field by field (search is deterministic, so identical means bit for bit
in the stored values). Refuses any path naming the held-out test set.
"""

import json
import sys

TEST_SET = "ed-rush-optical-essential-mix"
IGNORED = {"wall_seconds"}
SHOWN = 5


def load(path):
    if TEST_SET in path:
        raise SystemExit(f"refused: {path} names the held-out test set")
    with open(path) as file:
        return json.load(file)


def differences(saved, new, where="", found=None):
    """Paths at which the two JSON values differ, outside `IGNORED`."""
    found = [] if found is None else found
    if isinstance(saved, dict) and isinstance(new, dict):
        for key in sorted(set(saved) | set(new)):
            if key in IGNORED:
                continue
            if key not in saved or key not in new:
                found.append(f"{where}/{key}: only in {'new' if key in new else 'saved'}")
            else:
                differences(saved[key], new[key], f"{where}/{key}", found)
    elif isinstance(saved, list) and isinstance(new, list):
        if len(saved) != len(new):
            found.append(f"{where}: {len(saved)} entries saved, {len(new)} new")
        for number, (left, right) in enumerate(zip(saved, new)):
            differences(left, right, f"{where}[{number}]", found)
    elif saved != new:
        found.append(f"{where}: {saved!r} saved, {new!r} new")
    return found


def main():
    saved_path, new_path = sys.argv[1:3]
    found = differences(load(saved_path), load(new_path))
    if not found:
        print(f"{new_path}: identical")
        return
    print(f"{new_path}: {len(found)} differences")
    for line in found[:SHOWN]:
        print(f"  {line}")


if __name__ == "__main__":
    main()
