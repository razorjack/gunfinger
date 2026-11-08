"""Session 84: plays of one mixes report confident under arm A but not B, with the default's level.

gap_search_mix_pairs.py L0.json A.json B.json
"""
import json
import sys

l0, a, b = (json.load(open(p)) for p in sys.argv[1:4])
for m0, ma, mb in zip(l0["mixes"], a["mixes"], b["mixes"]):
    for p0, pa, pb in zip(m0["score"]["plays"], ma["score"]["plays"], mb["score"]["plays"]):
        t = pa["truth"]
        if pa["level"] != pb["level"]:
            best = lambda p: (p["best"] or {}).get("hits") if p.get("best") else None
            print(f"{ma['mix']['name']} {t['start_seconds']:7.1f}-{t['end_seconds']:7.1f} speed {t['speed']:.4f} "
                  f"L0 {p0['level']}({best(p0)}) A {pa['level']}({best(pa)}) B {pb['level']}({best(pb)}) "
                  f"{t['asset'][-50:]}")
