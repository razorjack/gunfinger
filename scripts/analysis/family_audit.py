"""The split audit of brief 87: whether any family has members on both
sides of a panel's split, from the family keys `gunfinger-eval families`
writes and the panels alone. Searches nothing and reads no report.

    python3 -I scripts/analysis/family_audit.py PANELS_DIR IN_USE_KEY.json WITH_SCRATCH_KEY.json OUT.json

Panels: the recall panel (development sources against validation sources),
the two shared-material panels (development families against validation
families, and panel 1 against panel 2), the sweep panels and the
shared-mixes panels (what a held-out or left-out recording's family holds
beyond what the panel leaves out of the index). Names are compared in
Unicode NFC: the committed recall panel and its copy in work/s33/panels
spell one name in different normalisations."""

import glob
import json
import os
import sys
import unicodedata

TIERS = ("recording", "title", "relation")


def nfc(name):
    return unicodedata.normalize("NFC", name)


class Key:
    """A family key: each file's family at each tier; a file no join
    names is its own family."""

    def __init__(self, key):
        self.sources = key["sources"]
        self.files = {
            nfc(f): {tier: nfc(fams[tier]) for tier in TIERS}
            for f, fams in key["files"].items()
        }
        self.members = {tier: {} for tier in TIERS}
        for f, fams in self.files.items():
            for tier in TIERS:
                self.members[tier].setdefault(fams[tier], set()).add(f)

    def of(self, name, tier):
        name = nfc(name)
        return self.files.get(name, {}).get(tier, name)

    def family_members(self, name, tier):
        name = nfc(name)
        return self.members[tier].get(self.of(name, tier), {name})


def across(key, side_a, side_b):
    """Families at each tier with files on both sides. Each side maps a
    label (a source or a panel family) to its files."""
    found = {}
    for tier in TIERS:
        families_a = {}
        for label, files in side_a.items():
            for f in files:
                families_a.setdefault(key.of(f, tier), set()).add((label, nfc(f)))
        hits = []
        for label, files in side_b.items():
            for f in files:
                fam = key.of(f, tier)
                if fam in families_a:
                    hits.append(
                        {
                            "family": fam,
                            "a": sorted([list(x) for x in families_a[fam]]),
                            "b": [label, nfc(f)],
                        }
                    )
        found[tier] = hits
    return found


def within(key, side):
    """Families at each tier holding files of two labels of one side."""
    found = {}
    for tier in TIERS:
        labels = {}
        for label, files in side.items():
            for f in files:
                labels.setdefault(key.of(f, tier), set()).add(label)
        found[tier] = sorted(
            [fam, sorted(ls)] for fam, ls in labels.items() if len(ls) > 1
        )
    return found


def recall_audit(panel, key):
    dev = {s["asset"]: [s["asset"]] for s in panel["development"]["sources"]}
    val = {s["asset"]: [s["asset"]] for s in panel["validation"]["sources"]}
    dev_tags = {s["family"] for s in panel["development"]["sources"]}
    val_tags = {s["family"] for s in panel["validation"]["sources"]}
    return {
        "development_sources": len(dev),
        "validation_sources": len(val),
        "split_key_overlap": sorted(dev_tags & val_tags),
        "across": across(key, dev, val),
        "within_development": within(key, dev),
        "within_validation": within(key, val),
        "development_families": {
            tier: len({key.of(f, tier) for f in dev}) for tier in TIERS
        },
        "validation_families": {
            tier: len({key.of(f, tier) for f in val}) for tier in TIERS
        },
    }


def shared_family_files(family):
    files = set()
    for recording in family.get("recordings", []):
        files.update(recording)
    for field in ("other_versions", "unjudged_same_title"):
        for item in family.get(field, []):
            files.update(item if isinstance(item, list) else [item])
    for pair in family.get("pairs", []):
        files.update(pair.get("files", []))
    for direction in family.get("directions", []):
        files.update([direction["played"], direction["related"]])
    return files


def scenario_files(development):
    files = {}
    for scenario in development.get("scenarios", []):
        files.setdefault(scenario["name"].rsplit("-", 1)[0], set()).update(
            [scenario["played"], scenario["related"]]
            + scenario.get("played_recording", [])
            + scenario.get("related_recording", [])
        )
    return files


def shared_sides(panel, development):
    sides = {"development": {}, "validation": {}}
    for family in panel["families"]:
        sides[family["half"]][family["name"]] = shared_family_files(family)
    if development:
        for name, files in scenario_files(development).items():
            sides["development"].setdefault(name, set()).update(files)
    return sides


def shared_audit(sides, key):
    return {
        "families": {half: sorted(side) for half, side in sides.items()},
        "files": {half: sum(len(v) for v in side.values()) for half, side in sides.items()},
        "across": across(key, sides["development"], sides["validation"]),
    }


def left_out_audit(groups, in_use, key):
    """For each group of files left out of an index together, the files
    its family holds at the recording and title tiers that neither the
    group nor the clusters in use leave out."""
    found = []
    for label, files in groups.items():
        left_out = set()
        for f in files:
            left_out |= in_use.family_members(f, "recording")
            left_out.add(nfc(f))
        for tier in ("recording", "title"):
            family = set()
            for f in files:
                family |= key.family_members(f, tier)
            extra = sorted(family - left_out)
            if extra:
                found.append({"left_out": label, "tier": tier, "still_indexed": extra})
    return found


def sweep_audit(panel, in_use, key):
    held = {a: [a] for a in panel["held_out"]}
    draws_held = {d["asset"]: [d["asset"]] for d in panel["draws"] if d["held_out"]}
    draws_indexed = {d["asset"]: [d["asset"]] for d in panel["draws"] if not d["held_out"]}
    return {
        "held_out_files": len(held),
        "beyond_left_out": left_out_audit(held, in_use, key),
        "held_out_draws_against_indexed_draws": across(key, draws_held, draws_indexed),
    }


def shared_mixes_audit(panel, in_use, key):
    groups = {p["name"]: p["recording"] for p in panel["played"]}
    return {"beyond_left_out": left_out_audit(groups, in_use, key)}


def count(found):
    return {tier: len(hits) for tier, hits in found.items()}


def audit(panels_dir, in_use, key):
    load = lambda name: json.load(open(os.path.join(panels_dir, name), encoding="utf-8"))
    result = {"sources": key.sources}
    result["recall"] = recall_audit(load("recall-seed-2026.json"), key)
    p1 = shared_sides(
        load("shared-material-seed-2026.json"),
        load("shared-material-seed-2026-development.json"),
    )
    p2 = shared_sides(
        load("shared-material-2-seed-2026.json"),
        load("shared-material-2-seed-2026-development.json"),
    )
    result["shared_1"] = shared_audit(p1, key)
    result["shared_2"] = shared_audit(p2, key)
    both = lambda sides: {**sides["development"], **sides["validation"]}
    result["shared_1_against_2"] = across(key, both(p1), both(p2))
    result["sweep"] = {
        os.path.basename(path): sweep_audit(json.load(open(path, encoding="utf-8")), in_use, key)
        for path in sorted(glob.glob(os.path.join(panels_dir, "sweep-seed-*.json")))
    }
    result["shared_mixes"] = {
        name: shared_mixes_audit(load(name), in_use, key)
        for name in ("shared-mixes-s21.json", "shared-mixes-s21-rips.json")
    }
    return result


def print_audit(name, result):
    print(f"== {name}")
    r = result["recall"]
    print(
        f"recall: {r['development_sources']} + {r['validation_sources']} sources; split key overlap {len(r['split_key_overlap'])}; "
        f"across {count(r['across'])}; within development {count(r['within_development'])}; within validation {count(r['within_validation'])}; "
        f"families development {r['development_families']}, validation {r['validation_families']}"
    )
    for panel in ("shared_1", "shared_2"):
        s = result[panel]
        print(f"{panel}: files {s['files']}; across {count(s['across'])}")
    print(f"shared_1_against_2: {count(result['shared_1_against_2'])}")
    for seed, s in result["sweep"].items():
        tiers = [x["tier"] for x in s["beyond_left_out"]]
        print(
            f"{seed}: held out {s['held_out_files']}; beyond left out: recording {tiers.count('recording')}, title {tiers.count('title')}; "
            f"held-out draws against indexed draws {count(s['held_out_draws_against_indexed_draws'])}"
        )
    for name, s in result["shared_mixes"].items():
        tiers = [x["tier"] for x in s["beyond_left_out"]]
        print(f"{name}: beyond left out: recording {tiers.count('recording')}, title {tiers.count('title')}")


def main():
    panels_dir, in_use_path, scratch_path, out = sys.argv[1:5]
    in_use = Key(json.load(open(in_use_path, encoding="utf-8")))
    scratch = Key(json.load(open(scratch_path, encoding="utf-8")))
    results = {"in_use": audit(panels_dir, in_use, in_use), "with_scratch": audit(panels_dir, in_use, scratch)}
    for name, result in results.items():
        print_audit(name, result)
    with open(out, "w", encoding="utf-8") as f:
        json.dump(results, f, indent=1, ensure_ascii=False)


if __name__ == "__main__":
    main()
