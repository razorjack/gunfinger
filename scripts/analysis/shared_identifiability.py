"""Proposal 5's outcome per direction (session 20, experiment 0112), by
the decision rule written in docs/brief-20.md before the diagnostic ran:
the related file against the played recording's rips (the noise floor)
in audio, peaks and pairs over the shared passage, with the main case
(the played recording left out) from the `shared` outcomes.

usage: shared_identifiability.py <identifiability report.json> <outcomes.json> <out.json>

The outcomes file is shared_outcomes.py's.
"""
import json
import sys

MEASURES = {
    "audio": lambda found: found["audio_similarity"],
    "peaks": lambda found: found["peaks_share"],
    "pairs": lambda found: found["hits_per_second"],
}


def values(compared):
    found = compared["found"]
    if found is None:
        return {name: 0.0 for name in MEASURES}
    return {name: (measure(found) if measure(found) is not None else 0.0)
            for name, measure in MEASURES.items()}


def strongest(rows, direction, query_prefix, role):
    hits = [r["strongest"].get(role, {}).get("hits", 0) for r in rows
            if r["direction"] == direction and r["index"] == "played left out"
            and r["query"].startswith(f"{direction}-{query_prefix}")]
    return max(hits, default=None)


def evidence(rows, direction):
    """Where R's evidence falls, in the played-left-out index, and the
    played file's in `everything`."""
    out = {}
    for label, prefix in (("alone 30 s", "alone-30s"), ("beyond 20 s", "beyond-20s"),
                          ("before 20 s", "before-20s"), ("looped 240 s", "looped-240s"),
                          ("break 240 s", "break-5s-looped-240s")):
        out[label] = strongest(rows, direction, prefix, "related")
    return out


def main(report_path, outcomes_path, out_path):
    report = json.load(open(report_path, encoding="utf-8"))
    rows = json.load(open(outcomes_path, encoding="utf-8"))["rows"]
    rips = [values(c) for d in report["directions"] if not d["control"]
            for c in d["files"] if c["role"] == "rip"]
    pooled = {name: min(r[name] for r in rips) for name in MEASURES} if rips else None
    results = []
    for direction in report["directions"]:
        name = direction["direction"]
        related = next(c for c in direction["files"] if c["role"] == "related")
        own_rips = [values(c) for c in direction["files"] if c["role"] == "rip"]
        floor = ({m: min(r[m] for r in own_rips) for m in MEASURES} if own_rips else pooled)
        r = values(related)
        wrong = [row for row in rows if row["direction"] == name
                 and row["index"] == "played left out" and row["outcome"] == "wrong confident"
                 and row["wrong_role"] == "related"]
        if direction["control"]:
            outcome = "control"
        elif r["audio"] >= floor["audio"]:
            outcome = "1: no distinguishing information"
        elif r["peaks"] >= floor["peaks"] and r["pairs"] >= floor["pairs"]:
            outcome = "2: lost before peaks and pairs"
        elif wrong:
            outcome = "3: survives, ignored by the evidence"
        else:
            outcome = "separated today"
        results.append({
            "family": direction["family"], "direction": name, "control": direction["control"],
            "related": r, "floor": floor, "floor_from": "own rips" if own_rips else "pooled",
            "rips": own_rips, "related_found": related["found"],
            "wrong_confident_kinds": sorted({row["kind"] for row in wrong}),
            "evidence": evidence(rows, name), "outcome": outcome,
        })
        f = related["found"] or {}
        print(f"{name:<22} {outcome}")
        print(f"   R: audio {r['audio']:.2f} (chance {f.get('audio_chance', float('nan')):.2f}) "
              f"peaks {r['peaks']:.2f} (chance {f.get('peaks_chance') or float('nan'):.2f}) "
              f"pairs {r['pairs']:.2f}/s  |  floor ({'own' if own_rips else 'pooled'}): "
              f"audio {floor['audio']:.2f} peaks {floor['peaks']:.2f} pairs {floor['pairs']:.2f}/s")
        print(f"   wrong confident (played left out): {', '.join(results[-1]['wrong_confident_kinds']) or 'none'}"
              f"  |  R hits {results[-1]['evidence']}")
    json.dump({"pooled_floor": pooled, "directions": results},
              open(out_path, "w", encoding="utf-8"), indent=1, ensure_ascii=False)


if __name__ == "__main__":
    main(*sys.argv[1:])
