"""Summarises `gunfinger-eval shared` (work/reports/shared-material.json):
per query and index, the strongest detection of the played recording, the
related one and any other, with hits, windows, level and the peak
verifier's share and support against their chance level (the mean over
the shifted references).

usage: shared_summary.py <shared-material.json> <output.json>
"""
import json
import statistics
import sys

report = json.load(open(sys.argv[1]))
rows = []
for result in report["results"]:
    row = {k: result[k] for k in ("scenario", "kind", "query", "seconds", "index")}
    for role in ("played", "related", "other"):
        found = [d for d in result["detections"] if d["role"] == role]
        if not found:
            row[role] = None
            continue
        d = max(found, key=lambda d: d["hits"])
        v = d.get("verified")
        cell = {"hits": d["hits"], "windows": d["windows"], "level": d["level"], "segments": len(found),
                "asset": d["asset"]}
        if v:
            cell.update({"share": round(v["aligned"]["share"], 3), "support": round(v["aligned"]["support"], 3),
                         "chance_share": round(statistics.mean(s["share"] for s in v["shifted"]), 3),
                         "chance_support": round(statistics.mean(s["support"] for s in v["shifted"]), 3),
                         "reference_peaks": v["aligned"]["reference_peaks"]})
        row[role] = cell
    rows.append(row)
json.dump({"scenarios": report["scenarios"], "rows": rows}, open(sys.argv[2], "w"), indent=1)


def show(cell):
    if not cell:
        return "-"
    text = f"{cell['hits']}/{cell['windows']}w {cell['level'][:4]}"
    if "share" in cell:
        text += f" sh {cell['share']:.2f}({cell['chance_share']:.2f}) su {cell['support']:.2f}({cell['chance_support']:.2f})"
    return text + (f" x{cell['segments']}" if cell["segments"] > 1 else "")


for row in rows:
    print(f"{row['query']:<26} {row['index']:<16} played {show(row['played']):<44} related {show(row['related']):<44} other {show(row['other'])}")
