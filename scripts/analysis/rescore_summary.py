"""Session 23, step 3: (c) at 60 hits and session 21's exploratory cut
((c)+(a1) under 6.276 hits per second) on `shared_mixes.py score` outputs
under the old and the new clusters, with the bounds the saved reports
allow.

A play whose saved holder is now in its own recording has an unknown (c)
(`c unknown`): the report keeps no weaker holder. Such a play was marked
under the old clusters if its old row's (c) is 60 or more; its mark is
then removed at most. The new clusters only add files to recordings, so a
play unmarked under the old clusters stays unmarked.

usage: python3 -I scripts/analysis/rescore_summary.py OLD.json NEW.json
"""

import json
import sys

CUT = 6.276
MARK = 60


def load(path):
    with open(path, encoding="utf-8") as file:
        return json.load(file)


def samples(score):
    rows = score["rows"]
    true = [r for r in rows if r["outcome"] == "right" and (
        (r["run"] == "played left out" and r["role"] == "drawn")
        or (r["run"] == "source left out" and r["role"] == "played"))]
    wrong = [r for r in rows if r["run"] == "played left out" and r["outcome"] == "wrong related"]
    return {"true": true, "wrong related": wrong, "second": score["second"]}


def summary(plays):
    known = [p for p in plays if not p["values"].get("c unknown")]
    return {
        "plays": len(plays),
        "c marked": sum(p["values"]["c"] >= MARK for p in known),
        "c unknown": len(plays) - len(known),
        "cut rejects": sum(p["values"]["c+a1"] < CUT for p in known),
    }


def main():
    old_path, new_path = sys.argv[1:]
    out = {}
    for name, path in (("old", old_path), ("new", new_path)):
        out[name] = {kind: summary(plays) for kind, plays in samples(load(path)).items()}
    print(json.dumps(out, indent=1))


if __name__ == "__main__":
    main()
