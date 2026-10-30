"""Session 62: the fault census of the other library (experiment 0248).
Reads a `stereo-width --other` report and writes, per class, the count in
each kind of source and the share of the whole population, each kind
weighted by its share of the population, with a 95% interval (stratified;
Wilson's interval per kind, combined by weight). Also lists every file
flagged one channel, inverted or delayed, with its measures. File names
only. Usage: stereo_census.py REPORT OUT.json"""
import json, math, sys

CLASSES = ["one-channel", "inverted", "delayed", "mono", "nearly-mono", "narrow", "wide"]
FLAGGED = {"one-channel", "inverted", "delayed"}
Z = 1.96


def wilson(k, n):
    if n == 0:
        return (0.0, 0.0, 1.0)
    p = k / n
    centre = (p + Z * Z / (2 * n)) / (1 + Z * Z / n)
    half = Z * math.sqrt(p * (1 - p) / n + Z * Z / (4 * n * n)) / (1 + Z * Z / n)
    return (p, max(0.0, centre - half), min(1.0, centre + half))


report = json.load(open(sys.argv[1]))
population = {k["kind"]: k["files"] for k in report["population_by_kind"]}
total = sum(population.values())
sampled = {kind: [f for f in report["files"] if f["kind"] == kind] for kind in population}
out = {"population": total, "population_by_kind": population,
       "sampled_by_kind": {k: len(v) for k, v in sampled.items()}, "classes": {}}
for cls in CLASSES:
    row = {"by_kind": {}, "share": 0.0, "low": 0.0, "high": 0.0}
    for kind, files in sampled.items():
        k = sum(f["class"] == cls for f in files)
        p, lo, hi = wilson(k, len(files))
        w = population[kind] / total
        row["by_kind"][kind] = k
        row["share"] += w * p
        row["low"] += w * lo
        row["high"] += w * hi
    row["files_estimated"] = round(row["share"] * total)
    out["classes"][cls] = row
out["flagged"] = [
    {"asset": f["asset"], "kind": f["kind"], "class": f["class"], "seconds": round(f["seconds"]),
     "side_to_mid_db": round(f["side_to_mid_db"], 1), **{k: round(v, 3) for k, v in f["channels"].items()}}
    for f in report["files"] if f["class"] in FLAGGED
]
json.dump(out, open(sys.argv[2], "w"), indent=1)
for cls, row in out["classes"].items():
    print(f"{cls:<12} {row['by_kind']} share {100*row['share']:.2f}% ({100*row['low']:.2f}-{100*row['high']:.2f}%), about {row['files_estimated']} files")
