"""Proposal 5's outcome per direction (session 20, experiment 0112), by
the decision rule written in docs/brief-20.md before the diagnostic ran:
the related file R against the played recording's rips (the noise floor)
in audio, peaks and pairs over the shared passage, with the main case
(the played recording left out) from the `shared` report.

usage: shared_identifiability.py <identifiability report.json> <shared report.json>
                                 <outcomes.json> <out.json>

The outcomes file is shared_outcomes.py's. Spans are in seconds of the
query: in `beyond 30 s` from the passage's start and end, in `before 20 s`
from the passage's start.

Beside the rule's outcome (R's own measures, R itself wrong confident):
- where R is never confident but another file of the related recording
  is, the rule applied to the strongest such file (`by the confident
  file`);
- the rule with pairs per second of each file's aligned span instead of
  the passage (`pairs per span`), a sensitivity check, not the rule.
A rip with no detection is listed and left out of the floor.
"""
import json
import sys

MEASURES = {
    "audio": lambda found: found["audio_similarity"],
    "peaks": lambda found: found["peaks_share"] or 0.0,
    "pairs": lambda found: found["hits_per_second"],
}


def values(found, per_span=False):
    out = {name: measure(found) for name, measure in MEASURES.items()}
    if per_span:
        out["pairs"] = found["hits"] / (found["query_end_seconds"] - found["query_start_seconds"])
    return out


def floor_of(rips, per_span=False):
    measured = [values(c["found"], per_span) for c in rips if c["found"] is not None]
    return {m: min(r[m] for r in measured) for m in MEASURES} if measured else None


def classify(r, floor, confident):
    if r["audio"] >= floor["audio"]:
        return "1: no distinguishing information"
    if r["peaks"] >= floor["peaks"] and r["pairs"] >= floor["pairs"]:
        return "2: lost before peaks and pairs"
    if confident:
        return "3: survives, ignored by the evidence"
    return "separated today"


def strongest(rows, direction, query_prefix, role):
    hits = [r["strongest"].get(role, {}).get("hits", 0) for r in rows
            if r["direction"] == direction and r["index"] == "played left out"
            and r["query"].startswith(f"{direction}-{query_prefix}")]
    return max(hits, default=None)


def evidence(rows, direction):
    """The related recording's strongest hits in some queries of the
    played-left-out index."""
    out = {}
    for label, prefix in (("alone 30 s", "alone-30s"), ("beyond 20 s", "beyond-20s"),
                          ("before 20 s", "before-20s"), ("looped 240 s", "looped-240s"),
                          ("break 240 s", "break-5s-looped-240s")):
        out[label] = strongest(rows, direction, prefix, "related")
    return out


def related_detections(shared, direction, query):
    return [(result, d) for result in shared["results"]
            if result["scenario"] == direction and result["query"] == f"{direction}-{query}"
            and result["index"] == "played left out"
            for d in result["detections"] if d["role"] == "related"]


def spans(shared, direction):
    """Where the related recording's detections fall in the
    played-left-out index."""
    beyond = [(round(d["start_seconds"], 1), round(d["end_seconds"] - (result["seconds"] - 30), 1),
               d["hits"]) for result, d in related_detections(shared, direction, "beyond-30s")]
    before = [(round(d["start_seconds"] - 20, 1), round(d["end_seconds"] - 20, 1), d["hits"])
              for _, d in related_detections(shared, direction, "before-20s")]
    looped = [d["hits"] for _, d in related_detections(shared, direction, "looped-240s")]
    return {
        "beyond 30 s: start after the passage's start, end after its end, hits": sorted(beyond),
        "before 20 s: start and end after the passage's start, hits": sorted(before),
        "looped 240 s: detections, strongest, sum": [len(looped), max(looped, default=0), sum(looped)],
    }


def confident_kinds(shared, direction, file):
    """The kinds of query in which `file` is confident with the played
    recording left out."""
    return sorted({result["kind"] for result in shared["results"]
                   if result["scenario"] == direction and result["index"] == "played left out"
                   for d in result["detections"]
                   if d["asset"] == file and d["level"] == "confident"})


def main(report_path, shared_path, outcomes_path, out_path):
    report = json.load(open(report_path, encoding="utf-8"))
    shared = json.load(open(shared_path, encoding="utf-8"))
    rows = json.load(open(outcomes_path, encoding="utf-8"))["rows"]
    every_rip = [c for d in report["directions"] for c in d["files"] if c["role"] == "rip"]
    pooled = floor_of(every_rip)
    pooled_per_span = floor_of(every_rip, per_span=True)
    results = []
    for direction in report["directions"]:
        name = direction["direction"]
        files = direction["files"]
        related = next(c for c in files if c["role"] == "related")
        own_rips = [c for c in files if c["role"] == "rip"]
        floor = floor_of(own_rips) or pooled
        floor_per_span = floor_of(own_rips, per_span=True) or pooled_per_span
        floor_from = "own rips" if floor_of(own_rips) else "pooled"
        found = related["found"]
        r = values(found) if found else None
        r_kinds = confident_kinds(shared, name, related["file"])
        result = {
            "family": direction["family"], "direction": name, "control": direction["control"],
            "related_file": related["file"], "related": r, "related_found": found,
            "floor": floor, "floor_from": floor_from,
            "rips": [{"file": c["file"], **(values(c["found"]) if c["found"] else {})}
                     for c in own_rips],
            "rips_not_found": [c["file"] for c in own_rips if c["found"] is None],
            "related_wrong_confident_kinds": r_kinds,
            "evidence": evidence(rows, name), "spans": spans(shared, name),
        }
        if direction["control"]:
            result["outcome"] = "control"
        elif r is None:
            result["outcome"] = "separated today" if not r_kinds else "R not measured"
        else:
            result["outcome"] = classify(r, floor, r_kinds)
            result["outcome pairs per span"] = classify(values(found, per_span=True),
                                                        floor_per_span, r_kinds)
            if not r_kinds:
                others = [(c, confident_kinds(shared, name, c["file"]))
                          for c in files if c["role"] == "related rip" and c["found"]]
                others = [(c, kinds) for c, kinds in others if kinds]
                if others:
                    c, kinds = max(others, key=lambda pair: pair[0]["found"]["hits"])
                    result["by the confident file"] = {
                        "file": c["file"], "measures": values(c["found"]), "kinds": kinds,
                        "outcome": classify(values(c["found"]), floor, kinds)}
        results.append(result)
        print(f"{name:<22} {result['outcome']}  |  pairs per span: {result.get('outcome pairs per span', '-')}")
        if r:
            print(f"   R: audio {r['audio']:.3f} (chance {found['audio_chance']:.2f}) "
                  f"peaks {r['peaks']:.3f} (chance {found['peaks_chance'] or float('nan'):.2f}) "
                  f"pairs {r['pairs']:.2f}/s  |  floor ({floor_from}): "
                  f"audio {floor['audio']:.3f} peaks {floor['peaks']:.3f} pairs {floor['pairs']:.2f}/s")
        print(f"   R wrong confident (played left out): {', '.join(r_kinds) or 'none'}"
              f"  |  related recording's hits {result['evidence']}")
        if "by the confident file" in result:
            other = result["by the confident file"]
            m = other["measures"]
            print(f"   by the confident file {other['file'][-60:]}: {other['outcome']} "
                  f"(audio {m['audio']:.3f} peaks {m['peaks']:.3f} pairs {m['pairs']:.2f}/s; "
                  f"{', '.join(other['kinds'])})")
        if result["rips_not_found"]:
            print(f"   rips with no detection: {result['rips_not_found']}")
    json.dump({"pooled_floor": pooled, "pooled_floor_per_span": pooled_per_span,
               "directions": results},
              open(out_path, "w", encoding="utf-8"), indent=1, ensure_ascii=False)


if __name__ == "__main__":
    main(*sys.argv[1:])
