"""Session 67: the check at indexing over a census of the other library
(experiment 0262). Reads a `stereo-width --other` report and, optionally,
an earlier census's report whose files it should contain, and writes: the
files the check repairs (`repaired`) by kind of source and weighted to the
population (Wilson's interval per kind, combined by weight, as
`stereo_census.py`), split into plainly inverted (side 10 dB or more above
the mid, so narrow or narrower once repaired) and uncertain; the files
inverted only partway (an unflagged file with a 30 s window at -0.5 or
less at lag 0); the correlations at lag 0 below 0.3; and the files new
since the earlier census apart. File names only.
Usage: polarity_census.py REPORT OUT.json [EARLIER_REPORT]"""
import json, math, sys

Z = 1.96
PLAINLY_DB = 10.0


def wilson(k, n):
    if n == 0:
        return (0.0, 0.0, 1.0)
    p = k / n
    centre = (p + Z * Z / (2 * n)) / (1 + Z * Z / n)
    half = Z * math.sqrt(p * (1 - p) / n + Z * Z / (4 * n * n)) / (1 + Z * Z / n)
    return (p, max(0.0, centre - half), min(1.0, centre + half))


def weighted(files, population, test):
    total = sum(population.values())
    row = {"by_kind": {}, "share": 0.0, "low": 0.0, "high": 0.0}
    for kind, count in population.items():
        drawn = [f for f in files if f["kind"] == kind]
        k = sum(1 for f in drawn if test(f))
        p, lo, hi = wilson(k, len(drawn))
        row["by_kind"][kind] = k
        for key, value in (("share", p), ("low", lo), ("high", hi)):
            row[key] += value * count / total
    row["files_estimated"] = round(row["share"] * total)
    return row


def entry(f):
    return {"asset": f["asset"], "kind": f["kind"], "seconds": round(f["seconds"]),
            "side_to_mid_db": round(f["side_to_mid_db"], 1),
            "correlation": round(f["channels"]["correlation"], 3),
            "strongest_lag": f["channels"]["strongest_lag"],
            "strongest_correlation": round(f["channels"]["strongest_correlation"], 3),
            "windows": f["windows"], "inverted_windows": f["inverted_windows"]}


def summary(files, population):
    repaired = lambda f: f["repaired"]
    return {
        "files": len(files),
        "by_kind": {kind: sum(f["kind"] == kind for f in files) for kind in population},
        "repaired": weighted(files, population, repaired),
        "plainly_inverted": weighted(
            files, population, lambda f: repaired(f) and f["side_to_mid_db"] >= PLAINLY_DB),
        "uncertain": weighted(
            files, population, lambda f: repaired(f) and f["side_to_mid_db"] < PLAINLY_DB),
        "partly_inverted": weighted(
            files, population, lambda f: not repaired(f) and f["inverted_windows"] > 0),
    }


report = json.load(open(sys.argv[1]))
population = {k["kind"]: k["files"] for k in report["population_by_kind"]}
files = report["files"]
out = {"population": sum(population.values()), "population_by_kind": population,
       "all": summary(files, population)}
if len(sys.argv) > 3:
    earlier = {f["asset"] for f in json.load(open(sys.argv[3]))["files"]}
    paths = {f["asset"] for f in files}
    out["earlier_files"] = len(earlier)
    out["earlier_files_missing"] = sorted(earlier - paths)
    out["new"] = summary([f for f in files if f["asset"] not in earlier], population)
out["repaired"] = [entry(f) for f in sorted(files, key=lambda f: f["channels"]["correlation"])
                   if f["repaired"]]
out["partly_inverted"] = [entry(f) for f in files if not f["repaired"] and f["inverted_windows"] > 0]
out["correlation_below_0_3"] = [entry(f) for f in sorted(files, key=lambda f: f["channels"]["correlation"])
                                if f["channels"]["correlation"] < 0.3]
json.dump(out, open(sys.argv[2], "w"), indent=1)
for part in ("all", "new"):
    if part not in out:
        continue
    print(f"{part}: {out[part]['files']} files {out[part]['by_kind']}")
    for key in ("repaired", "plainly_inverted", "uncertain", "partly_inverted"):
        row = out[part][key]
        print(f"  {key:<17} {row['by_kind']} share {100 * row['share']:.2f}% "
              f"({100 * row['low']:.2f}-{100 * row['high']:.2f}%), about {row['files_estimated']} files")
if "earlier_files" in out:
    print(f"earlier census files contained: {out['earlier_files'] - len(out['earlier_files_missing'])} of {out['earlier_files']}")
for f in out["repaired"]:
    print(f"  repaired {f['correlation']:>7} {f['side_to_mid_db']:>6} dB  {f['inverted_windows']}/{f['windows']}  {f['kind']:<16} {f['asset'][:80]}")
print(f"partly inverted, not repaired: {len(out['partly_inverted'])}")
for f in out["partly_inverted"][:30]:
    print(f"  partly {f['correlation']:>7} {f['side_to_mid_db']:>6} dB  {f['inverted_windows']}/{f['windows']}  {f['asset'][:80]}")
