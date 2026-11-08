"""Query count against family count for the experiments behind the
decisions (brief 87, experiment 0297).

    python3 -I scripts/analysis/family_reread.py KEY.json DATA_DIR OUT.json

For each experiment, the items its conclusion counts (queries, plays,
passages or negatives, as its text names them) are read from its saved data
in DATA_DIR (docs/experiments/data) and grouped by family: the relation and
recording families of the key `gunfinger-eval families` writes, looked up by
path or by file name (the data holds file names only), or the family labels
the data itself carries. Prints items, families and the largest family's
share of the items."""

import collections
import json
import os
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from family_counts import by_name, family, load_key, nfc  # noqa: E402


def load(data_dir, name):
    with open(os.path.join(data_dir, name), encoding="utf-8") as f:
        return json.load(f)


def files_at(value, *path):
    """The values under `path` (keys; lists are walked)."""
    if not path:
        if isinstance(value, list):
            return [x for item in value for x in files_at(item)]
        return [value] if isinstance(value, str) else []
    if isinstance(value, list):
        return [x for item in value for x in files_at(item, *path)]
    if isinstance(value, dict) and path[0] in value:
        return files_at(value[path[0]], *path[1:])
    return []


def items(data_dir):
    """(experiment, decision, what, files or labels, kind) for each row."""
    rows = []
    add = lambda exp, decision, what, values, kind="file": rows.append((exp, decision, what, values, kind))

    d = load(data_dir, "0102-possible-tier-third-mix.json")
    add("0092, 0100, 0102", "ADR 0019", "possible lines of the three development mixes",
        files_at(d["scans"], "plays", "asset") + files_at(d["new mix"], "plays", "asset"))
    d = load(data_dir, "0091-containment-rule.json")["demoted at passage, tolerance 10 s, no ratio bound"]
    add("0091", "containment", "design: 0076 passages, played file indexed (by played file)",
        files_at(d["0076 passages, played file indexed"], "play"))
    add("0091", "containment", "test: new judged pairs, played file indexed (by played file)",
        files_at(d["new judged pairs, played file indexed"], "play"))
    d = load(data_dir, "0107-possible-plays-inside-a-confident-play.json")
    for part in ("sweeps (NAS)", "recall (NAS)"):
        add("0107", "ADR 0019", f"{part}: possible plays left out (by confident play's file)",
            files_at(d[part], "left out", "play"))
    d = load(data_dir, "0175-where-a-ride-loses-a-side.json")
    for seed in ("3521", "3522"):
        add("0175", "ADR 0021", f"seed {seed}: lost sides, no detection",
            files_at(d["seeds"][seed]["no_detection"], "lost_sides", "asset"))
    d = load(data_dir, "0188-false-detections-measures.json")
    add("0188", "ADR 0022", "false confident detections (by query file)",
        [row["query"] for row in d for _ in range(row["confident"])])
    d = load(data_dir, "0230-other-copies-seeds-2027-2028.json")
    add("0230", "ADR 0022", "seed 2027, L5: new wrong confident",
        files_at(d["2027"]["ladders"]["L5"], "new_wrong_confident"))
    d = load(data_dir, "0232-second-suppression-by-default.json")
    add("0232", "ADR 0021", "NAS 2003 scan: plays added", files_at(d["nas"]["scan-razorjack-2003-03-29"], "added", "asset"))

    plays = load(data_dir, "0116-shared-mixes.json")["plays"]
    left_out = [p for p in plays if p["run"] == "played left out"]
    add("0116", "shared rules", "wrong related, played left out (by played)",
        [p["played"].rsplit("-", 1)[0] for p in left_out if p["outcome"] == "wrong related"], "label")
    add("0116", "shared rules", "true drawn plays, played left out",
        [p["asset"] for p in left_out if p["role"] == "drawn" and p["outcome"] == "right"])
    plays = load(data_dir, "0118-shared-mixes-rips.json")["plays"]
    left_out = [p for p in plays if p["run"] == "played left out"]
    add("0118", "shared rules", "wrong related, played left out (by played)",
        [p["played"].rsplit("-", 1)[0] for p in left_out if p["outcome"] == "wrong related"], "label")
    add("0118", "shared rules", "true plays through other rips, played left out",
        [p["asset"] for p in left_out if p["role"] == "drawn" and p["outcome"] == "right"])
    plays = load(data_dir, "0124-version-verifier-calibration.json")["plays"]
    add("0124", "shared rules", "negatives", [p["family"] for p in plays if p["label"] == "negative"], "label")
    add("0124", "shared rules", "positives", [p["family"] for p in plays if p["label"] == "positive"], "label")
    plays = load(data_dir, "0134-version-verifier-2-evaluation.json")["plays"]
    add("0134", "shared rules", "E1: true plays, development mixes", [p["file"] for p in plays if p["label"] == "true"])
    add("0134", "shared rules", "E2: wrong related (by played)",
        [p["played"].rsplit("-", 1)[0] for p in plays if p["label"] == "wrong related"], "label")
    for exp, name, prefix in (("0215", "0215-marked-features-development.json", "features-dev"),
                              ("0216", "0216-marked-features-test.json", "features-test")):
        d = load(data_dir, name)
        for part in sorted(d):
            if part.startswith(prefix + "-nas-2") or part.startswith(prefix + "-262-"):
                shared = [p for p in d[part] if p["class"] == "shared"]
                add(exp, "shared rules", f"{part[len(prefix) + 1:]}: shared plays (by marked file)",
                    [p["asset"] for p in shared])
                add(exp, "shared rules", f"{part[len(prefix) + 1:]}: unrelated playing (by marked file)",
                    [p["asset"] for p in d[part] if p["class"] == "playing"])

    for exp, name, part in (("0255", "0255-residual-oracle-development.json", "over-1"),
                            ("0256", "0256-residual-oracle-test-seeds.json", "summary"),
                            ("0257", "0257-residual-oracle-nas-scale.json", "summary"),
                            ("0258", "0258-residual-oracle-nas-test-seeds.json", "summary")):
        d = load(data_dir, name)
        add(exp, "0255-0275", "rows (by quiet record b)", files_at(d[part]["rows"], "b"))
    d = load(data_dir, "0263-repaired-records-recovery-and-controls.json")
    add("0263", "0255-0275", "flagged files", files_at(d, "flagged", "asset"))
    d = load(data_dir, "0265-counterexample-replays.json")
    add("0265", "0255-0275", "minimal recipes (by target)", files_at(d, "minimal", "base", "target", "asset"))
    for exp, name in (("0264", "0264-counterexample-classes.json"), ("0266", "0266-counterexample-classes-rerun.json")):
        d = load(data_dir, name)
        add(exp, "0255-0275", "NAS wrong answers (by wrong file)",
            files_at(d["nas"]["classes"], "examples", "outcome", "wrong", "asset"))
    return rows


def summarise(rows, key):
    names = by_name(key)
    out = []
    for exp, decision, what, values, kind in rows:
        if kind == "label":
            groups = {"label": collections.Counter(values)}
        else:
            values = [nfc(v) for v in values]
            groups = {tier: collections.Counter(family(key, names, v, tier)[0] for v in values)
                      for tier in ("recording", "relation")}
        main = groups["label" if kind == "label" else "relation"]
        out.append({
            "experiment": exp,
            "decision": decision,
            "items": what,
            "count": len(values),
            "families": len(main),
            "recording_families": len(groups["recording"]) if kind != "label" else None,
            "largest_share": round(max(main.values()) / len(values), 3) if values else None,
            "by": kind,
        })
    return out


def main():
    key_path, data_dir, out = sys.argv[1:4]
    summary = summarise(items(data_dir), load_key(key_path))
    for row in summary:
        rec = "" if row["recording_families"] is None else f" (recording {row['recording_families']})"
        print(f"{row['experiment']} {row['decision']:<13} {row['items']:<62} {row['count']:>5} items, "
              f"{row['families']:>3} families{rec}, largest {row['largest_share']}")
    with open(out, "w", encoding="utf-8") as f:
        json.dump(summary, f, indent=1, ensure_ascii=False)


if __name__ == "__main__":
    main()
