"""Session 84: for a mixes report, each mix's stretches the gap search covers (plays::group's rule)
and, for named plays, whether their time is covered and by which confident play.

gap_search_covered.py REPORT.json MIX START ...
"""
import json
import sys

POSSIBLE, CONFIDENT, WINDOWS, MAX_GAP = 60, 240, 3, 90.0


def plays(dets):
    segs = sorted((d for d in dets if d["hits"] >= POSSIBLE), key=lambda d: (d["asset"], d["start_seconds"]))
    out = []
    for d in segs:
        if out and out[-1]["asset"] == d["asset"] and d["start_seconds"] - max(s["end_seconds"] for s in out[-1]["segs"]) <= MAX_GAP:
            out[-1]["segs"].append(d)
        else:
            out.append({"asset": d["asset"], "segs": [d]})
    return out


r = json.load(open(sys.argv[1]))
args = sys.argv[2:]
for name, start in zip(args[::2], args[1::2]):
    m = next(m for m in r["mixes"] if m["mix"]["name"] == name)
    p = next(p for p in m["score"]["plays"] if abs(p["truth"]["start_seconds"] - float(start)) < 1)
    t = p["truth"]
    covering = []
    for play in plays(m["detections"]):
        if any(s["hits"] >= CONFIDENT and s["windows"] >= WINDOWS for s in play["segs"]):
            for s in play["segs"]:
                if s["start_seconds"] < t["end_seconds"] and t["start_seconds"] < s["end_seconds"]:
                    kind = "conf" if s["hits"] >= CONFIDENT and s["windows"] >= WINDOWS else "poss"
                    covering.append(f"{s['start_seconds']:.0f}-{s['end_seconds']:.0f} {kind} {s['hits']} {s['asset'][-40:]}")
    print(f"{name} truth {t['start_seconds']:.1f}-{t['end_seconds']:.1f} speed {t['speed']:.4f} {t['asset'][-40:]}")
    for c in covering:
        print("   covered by", c)
