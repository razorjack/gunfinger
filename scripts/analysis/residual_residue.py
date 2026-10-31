"""Session 70 (brief 70): summarise `mixes --kind drops --residual` reports
with several timings and fixes (`--residual-timing`, `--residual-fix`).

For each timing and fix: the level each arm gives each record B inside a
record A found confidently (as `residual_pilot.py`), wrong answers per arm,
what the mask leaves of A by kind of reference, and the evidence rule of
step 3 at the bars 20, 40 and 60. With --diagnose, the measures of step 1
for the cases whose reference is another rip, split by whether A stays
confident after masking (true time map, no fix).

Usage: python3 -I scripts/analysis/residual_residue.py REPORT... [--json OUT]
       [--diagnose] [--probe]
--probe runs ffprobe on each other-rip reference (codec, bit rate) and
needs the corpus at corpus/library and the NAS at /mnt/atlas/Music/dnb.
"""
import json
import statistics
import subprocess
import sys
from collections import Counter, defaultdict

LEVELS = ["none", "weak", "possible", "confident"]
BARS = [20, 40, 60]
SCENARIOS = ["quiet", "brief", "unrelated-drop", "pair-drop"]
NAS_PREFIX = "second-library/"


def at_least(level, floor):
    return LEVELS.index(level) >= LEVELS.index(floor)


def inside_blocks(plays):
    """(A index, B index, scenario) for each B inside the A it joins."""
    for index, a in enumerate(plays[:-1]):
        b = plays[index + 1]
        if (
            a.get("side") == "first"
            and b.get("side") == "second"
            and a.get("scenario") == b.get("scenario")
            and b["end_seconds"] <= a["end_seconds"]
        ):
            yield index, index + 1, a["scenario"]


def source(masking):
    """The played file, a rip in the corpus, or a file of the other library."""
    if masking["reference_kind"] != "other-rip":
        return masking["reference_kind"]
    return "nas-file" if masking["reference"].startswith(NAS_PREFIX) else "corpus-rip"


def evidence_of(arm, found):
    """The plain hits stored for a detection of the arm (0 if none)."""
    for item in arm.get("evidence", []):
        if item["asset"] == found["asset"] and abs(item["start_seconds"] - found["start_seconds"]) < 1e-6:
            return item["plain_hits"]
    return 0


def level_under_rule(arm, found, level, bar):
    """A detection's level once the evidence rule drops it under `bar`."""
    if found is None or not at_least(level, "possible"):
        return level
    return level if evidence_of(arm, found) >= bar else "weak"


def summarise(paths):
    out = {}
    for path in paths:
        report = json.load(open(path))
        seed = report["seed"]
        mixes = {result["mix"]["name"]: result for result in report["mixes"]}
        for case in report["residual"]["cases"]:
            result = mixes[case["mix"]]
            plays = result["mix"]["plays"]
            plain = {arm["name"]: arm for arm in case["arms"]}
            for masking in case["maskings"]:
                for fit in masking["fits"]:
                    key = (masking["timing"], fit["fix"])
                    entry = out.setdefault(key, new_entry())
                    arms = dict(plain)
                    arms.update({arm["name"]: arm for arm in fit["arms"]})
                    count_case(entry, seed, case, masking, fit, arms, plays, result)
    return out


def new_entry():
    return {
        "rows": [],
        "wrong": defaultdict(Counter),
        "wrong_items": defaultdict(list),
        "seen": defaultdict(set),
        "residue": Counter(),
        "cases": Counter(),
    }


def count_case(entry, seed, case, masking, fit, arms, plays, result):
    kind = source(masking)
    entry["cases"][kind] += 1
    for name, arm in arms.items():
        for level, key in [("confident", "wrong_confident"), ("possible", "wrong_possible")]:
            for found in arm[key]:
                seen = (seed, case["mix"], found["asset"], round(found["start_seconds"]))
                if level == "confident":
                    if seen in entry["seen"][name]:
                        continue
                    entry["seen"][name].add(seen)
                    entry["wrong_items"][name].append(
                        {
                            "seed": seed,
                            "mix": case["mix"],
                            "asset": found["asset"],
                            "start": round(found["start_seconds"], 1),
                            "hits": found["hits"],
                            "plain_hits": evidence_of(arm, found) if name.startswith("masked") else None,
                        }
                    )
                entry["wrong"][name][level] += 1
                if name.startswith("masked"):
                    for bar in BARS:
                        if evidence_of(arm, found) >= bar:
                            entry["wrong"][f"{name} rule {bar}"][level] += 1
    masked_a = next(p for p in arms["masked"]["plays"] if p["play"] == case["play"])
    entry["residue"][f"{kind} {masked_a['level']}"] += 1
    for a, b, scenario in inside_blocks(plays):
        if a != case["play"]:
            continue
        row = {
            "seed": seed,
            "mix": case["mix"],
            "scenario": scenario,
            "b": plays[b]["asset"],
            "reference": kind,
            "whole_mix": result["score"]["plays"][b]["level"],
        }
        for name, arm in arms.items():
            found = next((p for p in arm["plays"] if p["play"] == b), None)
            level = found["level"] if found else "none"
            row[name] = level
            best = (found or {}).get("best")
            row[name + "_hits"] = best and best["hits"]
            if name.startswith("masked"):
                row[name + "_plain_hits"] = evidence_of(arm, best) if best else 0
                for bar in BARS:
                    row[f"{name} rule {bar}"] = level_under_rule(arm, best, level, bar)
        entry["rows"].append(row)


def counts(rows, arm):
    levels = [row.get(arm) for row in rows if row.get(arm) is not None]
    if not levels:
        return None
    return (
        sum(at_least(level, "weak") for level in levels),
        sum(at_least(level, "possible") for level in levels),
        sum(level == "confident" for level in levels),
        len(levels),
    )


def table(rows, arms):
    print(f"    {'B inside A':<16}" + "".join(f"{arm:>22}" for arm in arms))
    for scenario in SCENARIOS + ["quiet+brief"]:
        if scenario == "quiet+brief":
            chosen = [row for row in rows if row["scenario"] in ("quiet", "brief")]
        else:
            chosen = [row for row in rows if row["scenario"] == scenario]
        if not chosen:
            continue
        cells = []
        for arm in arms:
            c = counts(chosen, arm)
            cells.append(f"{'-' if c is None else f'{c[0]}/{c[1]}/{c[2]} of {c[3]}':>22}")
        print(f"    {scenario:<16}" + "".join(cells))


def report(out):
    summary = {}
    for (timing, fix), entry in sorted(out.items()):
        rows = entry["rows"]
        print(f"\n== timing {timing}, fix {fix}; cases by reference {dict(entry['cases'])}")
        arms = ["plain", "denser", "masked"] + [f"masked rule {bar}" for bar in BARS]
        table(rows, arms)
        for kind in ["played", "corpus-rip", "nas-file"]:
            chosen = [row for row in rows if row["reference"] == kind]
            if chosen:
                print(f"  reference {kind}:")
                table(chosen, ["plain", "masked", "masked rule 40"])
        print("  wrong (confident deduplicated by mix, file, second / possible):")
        for name in sorted(entry["wrong"]):
            print(f"    {name:<36} {entry['wrong'][name]['confident']:>3} / {entry['wrong'][name]['possible']:>3}")
        for name in ["plain", "masked", "masked-without-partners"]:
            for item in entry["wrong_items"].get(name, []):
                print(f"    wrong confident in {name}: {item}")
        print("  A after masking by reference:", dict(sorted(entry["residue"].items())))
        newly = [
            row
            for row in rows
            if row["scenario"] in ("quiet", "brief")
            and at_least(row["masked"], "possible")
            and not at_least(row["plain"], "possible")
        ]
        kept = {bar: sum(at_least(row[f"masked rule {bar}"], "possible") for row in newly) for bar in BARS}
        with_plain = sum(at_least(row["plain"], "weak") for row in newly)
        print(
            f"  quiet and brief B newly confirmed (masked, not plain): {len(newly)};"
            f" with a plain detection {with_plain}; kept by the rule at 20/40/60: {kept}"
        )
        summary[f"{timing} {fix}"] = {
            "cases": dict(entry["cases"]),
            "rows": {
                scenario: {arm: counts([r for r in rows if r["scenario"] == scenario], arm) for arm in arms}
                for scenario in SCENARIOS
            },
            "quiet_brief": {arm: counts([r for r in rows if r["scenario"] in ("quiet", "brief")], arm) for arm in arms},
            "quiet_brief_by_reference": {
                kind: {
                    arm: counts([r for r in rows if r["scenario"] in ("quiet", "brief") and r["reference"] == kind], arm)
                    for arm in ["plain", "masked", "masked rule 20", "masked rule 40", "masked rule 60"]
                }
                for kind in ["played", "corpus-rip", "nas-file"]
            },
            "wrong": {name: dict(counter) for name, counter in entry["wrong"].items()},
            "wrong_confident_items": {name: items for name, items in entry["wrong_items"].items()},
            "residue": dict(entry["residue"]),
            "newly_confirmed": len(newly),
            "newly_with_plain_detection": with_plain,
            "newly_kept_by_rule": kept,
        }
    return summary


def median(values):
    values = [value for value in values if value is not None]
    return round(statistics.median(values), 2) if values else None


def probe(asset):
    if asset.startswith(NAS_PREFIX):
        path = "/mnt/atlas/Music/dnb/" + asset[len(NAS_PREFIX):]
    else:
        path = "corpus/library/" + asset
    try:
        output = subprocess.run(
            ["ffprobe", "-v", "error", "-select_streams", "a:0", "-show_entries",
             "stream=codec_name,bit_rate,sample_rate:format=bit_rate", "-of", "json", path],
            capture_output=True, text=True, timeout=60,
        ).stdout
        data = json.loads(output)
        stream = data["streams"][0]
        rate = stream.get("bit_rate") or data.get("format", {}).get("bit_rate")
        return stream.get("codec_name"), int(rate) // 1000 if rate else None
    except Exception:
        return None, None


def diagnose(paths, with_probe):
    """Step 1's measures, true time map, no fix, by reference and residue."""
    groups = defaultdict(list)
    for path in paths:
        report = json.load(open(path))
        for case in report["residual"]["cases"]:
            for masking in case["maskings"]:
                if masking["timing"] != "truth":
                    continue
                fits = {fit["fix"]: fit for fit in masking["fits"]}
                if "off" not in fits:
                    continue
                masked = next(arm for arm in fits["off"]["arms"] if arm["name"] == "masked")
                a_level = next(p for p in masked["plays"] if p["play"] == case["play"])["level"]
                after = {
                    fix: next(p for p in next(arm for arm in fit["arms"] if arm["name"] == "masked")["plays"]
                              if p["play"] == case["play"])["level"]
                    for fix, fit in fits.items()
                }
                lags = masking["lags"]
                blocks = lags["blocks"]
                spreads = [s for s in fits["off"]["gain_spread_db"] if s is not None]
                item = {
                    "mix": case["mix"],
                    "seed": report["seed"],
                    "reference": masking["reference"],
                    "rip_hits": (masking.get("rip_line") or {}).get("hits"),
                    "rip_speed_off_ppm": (masking.get("rip_line") or {}).get("speed") and
                    round(abs(masking["rip_line"]["speed"] - 1.0) * 1e6),
                    "blocks_used": f"{len(blocks)}/{lags['measured']}",
                    "abs_lag_ms": median([abs(b["lag_ms"]) for b in blocks]),
                    "lag_spread_ms": (max(b["lag_ms"] for b in blocks) - min(b["lag_ms"] for b in blocks)) if blocks else None,
                    "drift_ppm": round(lags["drift_ppm"]),
                    "correlation": median([b["correlation"] for b in blocks]),
                    "at_zero": median([b["at_zero"] for b in blocks]),
                    "gain_spread_db": median(spreads),
                    "reference_seek_ms": masking.get("reference_seek_ms"),
                    "played_seek_ms": case.get("played_seek_ms"),
                    "after": after,
                }
                if with_probe and masking["reference_kind"] == "other-rip":
                    item["codec"], item["kbps"] = probe(masking["reference"])
                groups[(source(masking), a_level == "confident")].append(item)
    measures = ["rip_hits", "rip_speed_off_ppm", "abs_lag_ms", "lag_spread_ms", "drift_ppm",
                "correlation", "at_zero", "gain_spread_db", "reference_seek_ms", "played_seek_ms"]
    summary = {}
    for (kind, stays), items in sorted(groups.items()):
        line = {m: median([i.get(m) for i in items]) for m in measures}
        line["n"] = len(items)
        line["abs_lag_over_20ms"] = sum(1 for i in items if (i["abs_lag_ms"] or 0) > 20)
        line["seek_off_over_5ms"] = sum(
            1 for i in items
            if abs(i.get("reference_seek_ms") or 0) > 5 or abs(i.get("played_seek_ms") or 0) > 5
        )
        line["after_fix"] = {
            fix: dict(Counter(i["after"].get(fix) for i in items)) for fix in ["off", "align", "align-fine"]
        }
        if with_probe:
            line["codecs"] = dict(Counter(f"{i.get('codec')} {i.get('kbps')}" for i in items).most_common(8))
            line["lossy_under_256"] = sum(
                1 for i in items if i.get("codec") in ("mp3", "aac", "vorbis", "opus") and (i.get("kbps") or 999) < 256
            )
        print(f"\n{kind}, A {'stays confident' if stays else 'not confident'} after masking:")
        print("  " + json.dumps(line))
        summary[f"{kind} {'stays' if stays else 'goes'}"] = line
    return summary, groups


def main():
    args = sys.argv[1:]
    out = None
    if "--json" in args:
        at = args.index("--json")
        out = args[at + 1]
        args = args[:at] + args[at + 2:]
    with_diagnosis = "--diagnose" in args
    with_probe = "--probe" in args
    args = [arg for arg in args if arg not in ("--diagnose", "--probe")]
    summary = {"by_timing_and_fix": report(summarise(args))}
    if with_diagnosis:
        summary["diagnosis"], _ = diagnose(args, with_probe)
    if out:
        json.dump(summary, open(out, "w"), indent=1)


main()
