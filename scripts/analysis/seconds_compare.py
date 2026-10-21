"""Session 33: `gunfinger-eval seconds` outputs of the NAS scans against
experiment 0139's (the Mac, session 27), report by report and margin by
margin, under today's rule ("3 windows"): every count of seconds and every
track's start and end error.

usage: seconds_compare.py DATA_0139 SECONDS_DIR [OUT.json]
SECONDS_DIR holds `<report stem>-margin-<m>.json` (work/seconds/library-<store>/).
"""
import json
import os
import sys

RULE = "3 windows"


def bounds(boundaries):
    """Start and end errors rounded to 0.01 s, as 0139's data file holds them."""
    out = []
    for b in boundaries:
        if isinstance(b, dict):
            b = [b["start_error_seconds"], b["end_error_seconds"]]
        out.append([None if v is None else round(v, 2) for v in b])
    return out
mac = json.load(open(sys.argv[1]))
here_dir = sys.argv[2]
out = {}
for key, rules in sorted(mac.items()):
    if "library-nas-dnb-peaks/" not in key:
        continue
    name = key.rsplit("/", 1)[-1]
    path = os.path.join(here_dir, name)
    if "ed-rush" in name:
        raise SystemExit(f"refused: {name} names the held-out test set")
    if not os.path.exists(path):
        print(f"{name}: not here")
        continue
    here = {s["rule"]: s["score"] for s in json.load(open(path))}
    m, h = rules[RULE], here[RULE]
    same_seconds = m["seconds"] == h["seconds"]
    mb, hb = bounds(m["boundaries"]), bounds(h["boundaries"])
    same_bounds = mb == hb
    out[name] = {"seconds": {"mac": m["seconds"], "here": h["seconds"]},
                 "boundaries_equal": same_bounds,
                 "boundaries": {"mac": mb, "here": hb}}
    print(f"{name}: seconds {'equal' if same_seconds else 'differ'}, boundaries "
          f"{'equal' if same_bounds else 'differ'}")
    if not same_seconds:
        for k in m["seconds"]:
            if m["seconds"][k] != h["seconds"].get(k):
                print(f"  {k}: {m['seconds'][k]} -> {h['seconds'].get(k)}")
    if len(mb) != len(hb):
        print(f"  tracks: {len(mb)} -> {len(hb)}")
    if not same_bounds:
        for i, (a, b) in enumerate(zip(mb, hb)):
            if a != b:
                print(f"  track {i + 1}: {a} -> {b}")
if len(sys.argv) > 3:
    with open(sys.argv[3], "w", encoding="utf-8") as file:
        json.dump(out, file, indent=1)
