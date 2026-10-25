"""Session 49: a near-offset share that follows a copy whose speed drifts
(`docs/brief-49.md`, the pre-registration).

`measures` reads `pair --fine-rungs` reports and adds to session 40's
measures (`pair_offsets.measures_of`) the drift-following shares
`drift_W_T`: the seconds of the shorter file holding a supported hit of an
alignment, at the fitted speed or on a fine rung within W% of it, whose
line stays within T s of the main alignment's line over its span, over the
shorter file's length. `drift_0_T` uses the fitted speed alone.

`choose` picks (W, T) on session 40's populations by the registered order:
the fewest leave-one-out errors on the judged pairs (a), then the fewest
presumed same joins (b) below the threshold fitted on (a), then the
highest AUC on (a), then T = 10 s first, the smaller W, the smaller T.
Pairs named in the test verdicts are left out of every population.

`scores` gives the registered scores: on the development populations as
session 40's `scores`, on the test verdicts apart (also without the pairs
of SEEN, a pair file), and pooled with the judged pairs (a).

usage:
  drift_offsets.py measures OUT.json PAIRS_REPORT...
  drift_offsets.py choose POPULATIONS MEASURES TEST_VERDICTS OUT.json
  drift_offsets.py scores POPULATIONS MEASURES TEST_VERDICTS TEST_MEASURES CHOICE SEEN OUT.json
"""

import json
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from pair_offsets import (  # noqa: E402
    FITTED, SOURCE_KEYS, auc, best_threshold, key, leave_one_out, load,
    measures_of, name_of, says_same, search, verdicts_of)

FINE = "peaks, fine rung "
WIDTHS = (0.2, 0.4, 0.6, 0.8, 1.0)
NEARS = (2.0, 5.0, 10.0, 20.0)
# The measures the registered scores compare, beside the chosen one.
COMPARED = ["coverage", "supported", "near_10", "drift_0_10"]


def name(width, near):
    return f"drift_{width:g}_{near:g}"


def line_at(alignment, query_seconds):
    return alignment["offset_seconds"] + alignment["speed"] * query_seconds


def fine_rungs(report):
    """Each fine rung's relative speed in percent and its alignments."""
    return [(float(s["name"][len(FINE):].rstrip("%")), s["alignments"])
            for s in report["searches"] if s["name"].startswith(FINE)]


def drift_of(report, width, near):
    """The share of the shorter file holding a supported hit of an
    alignment of the fitted speed or a fine rung within `width` percent,
    whose line stays within `near` seconds of the main alignment's."""
    fitted = search(report, FITTED)
    if not fitted:
        return 0.0
    rungs = fine_rungs(report)
    if width > 0 and not rungs:
        raise ValueError(f"{report['query']}: a report without `pair --fine-rungs`")
    main = fitted[0]
    alignments = list(fitted) + [a for percent, found in rungs
                                 if abs(percent) <= width + 1e-9 for a in found]
    held = set()
    for a in alignments:
        ends = (a["query_start_seconds"], a["query_end_seconds"])
        if all(abs(line_at(a, q) - line_at(main, q)) <= near for q in ends):
            held.update(a.get("supported_seconds") or [])
    return len(held) / min(report["query_seconds"], report["found_seconds"])


def measures_with_drift(report):
    row = measures_of(report)
    for near in NEARS:
        row[name(0, near)] = drift_of(report, 0, near)
        for width in WIDTHS:
            row[name(width, near)] = drift_of(report, width, near)
    return row


def measures(arguments):
    out, reports = arguments[0], arguments[1:]
    rows = [measures_with_drift(report) for path in reports for report in load(path)]
    with open(out, "w") as file:
        json.dump(rows, file, indent=1, ensure_ascii=False)
    print(f"{len(rows)} pairs measured")


# The populations -------------------------------------------------------


def development(populations_file, measures_file, verdicts_file):
    """Session 40's labelled rows, each pair once, without the source
    pairs and without any pair the test verdicts name."""
    test = {key(a, b) for _, a, b in verdicts_of(verdicts_file)}
    labels = {key(p["query"], p["found"]): p for p in load(populations_file)["pairs"]}
    rows = {}
    for row in load(measures_file):
        k = key(row["query"], row["found"])
        label = labels.get(k)
        if label and k not in test and k not in SOURCE_KEYS:
            rows[k] = {**row, "population": label["population"], "label": label["label"]}
    rows = list(rows.values())

    def group(population, label):
        return [r for r in rows if r["population"] == population and r["label"] == label]

    return {
        "a same": group("a", "judged same"),
        "a different": group("a", "judged different"),
        "b": group("b", "presumed same"),
        "c": group("c", "panel different"),
        "d": group("d", "presumed different"),
    }


def labelled(rows, measure, same_label="judged same"):
    return [(r[measure], r["label"] == same_label) for r in rows]


# Choice ----------------------------------------------------------------


def candidates(groups):
    """Every (W, T) with its registered keys, best first."""
    judged = groups["a same"] + groups["a different"]
    found = []
    for near in NEARS:
        for width in WIDTHS:
            measure = name(width, near)
            pairs = labelled(judged, measure)
            threshold, errors = best_threshold(pairs, True)
            found.append({
                "measure": measure, "width": width, "near": near,
                "loo_errors": len(leave_one_out(pairs, True)),
                "b_below": sum(1 for r in groups["b"]
                               if not says_same(r[measure], threshold, True)),
                "auc_a": auc([r[measure] for r in groups["a same"]],
                             [r[measure] for r in groups["a different"]], True),
                "threshold": threshold, "errors": errors,
            })
    found.sort(key=lambda c: (c["loo_errors"], c["b_below"], -c["auc_a"],
                              c["near"] != 10.0, c["width"], c["near"]))
    return found


def choose(populations_file, measures_file, verdicts_file, out):
    found = candidates(development(populations_file, measures_file, verdicts_file))
    with open(out, "w") as file:
        json.dump({"chosen": found[0], "candidates": found}, file, indent=1)
    for c in found:
        print(f"{c['measure']:14} LOO {c['loo_errors']:2}  (b) below {c['b_below']}"
              f"  AUC (a) {c['auc_a']:.3f}  threshold {c['threshold']:.4g}")
    print("chosen:", found[0]["measure"])


# Scores ----------------------------------------------------------------


def test_rows(verdicts_file, measures_file):
    verdicts = {key(a, b): verdict for verdict, a, b in verdicts_of(verdicts_file)}
    rows = []
    for row in load(measures_file):
        verdict = verdicts.get(key(row["query"], row["found"]))
        if verdict:
            rows.append({**row, "label": f"judged {verdict}"})
    return rows


def at_threshold(rows, measure, threshold):
    """The errors of a threshold fixed in advance, and the pairs wrong."""
    wrong = [name_of(r) for r in rows
             if says_same(r[measure], threshold, True) != (r["label"] == "judged same")]
    return {"errors": len(wrong), "wrong": wrong}


def apart(rows, measure):
    same = [r[measure] for r in rows if r["label"] == "judged same"]
    different = [r[measure] for r in rows if r["label"] == "judged different"]
    return auc(same, different, True)


def scores(populations_file, measures_file, verdicts_file, test_measures, choice_file,
           seen_file, out):
    groups = development(populations_file, measures_file, verdicts_file)
    chosen = load(choice_file)["chosen"]["measure"]
    test = test_rows(verdicts_file, test_measures)
    with open(seen_file) as file:
        seen = {key(*line.rstrip("\n").split("\t")) for line in file if line.strip()}
    unseen = [r for r in test if key(r["query"], r["found"]) not in seen]
    judged = groups["a same"] + groups["a different"]
    pooled = judged + test
    result = {"chosen": chosen, "counts": {
        "a": [len(groups["a same"]), len(groups["a different"])],
        "b": len(groups["b"]), "c": len(groups["c"]), "d": len(groups["d"]),
        "test": [sum(r["label"] == "judged same" for r in test),
                 sum(r["label"] == "judged different" for r in test)],
        "test without seen": len(unseen)}, "measures": {}}
    for measure in COMPARED + [chosen]:
        pairs = labelled(judged, measure)
        threshold, errors = best_threshold(pairs, True)
        loo = leave_one_out(pairs, True)
        pooled_pairs = labelled(pooled, measure)
        pooled_threshold, pooled_errors = best_threshold(pooled_pairs, True)
        pooled_loo = leave_one_out(pooled_pairs, True)

        def against(group, as_same):
            return [name_of(r) for r in groups[group]
                    if says_same(r[measure], threshold, True) != as_same]

        result["measures"][measure] = {
            "development": {
                "auc_a": apart(judged, measure),
                "auc_b_a_different": auc([r[measure] for r in groups["b"]],
                                         [r[measure] for r in groups["a different"]], True),
                "auc_b_c": auc([r[measure] for r in groups["b"]],
                               [r[measure] for r in groups["c"]], True),
                "auc_b_d": auc([r[measure] for r in groups["b"]],
                               [r[measure] for r in groups["d"]], True),
                "threshold": threshold, "errors": errors, "loo_errors": len(loo),
                "loo_wrong": [name_of(judged[i]) for i in loo],
                "b_below": against("b", True), "c_above": against("c", False),
                "d_above": against("d", False),
            },
            "test": {
                "auc": apart(test, measure),
                **at_threshold(test, measure, threshold),
                "auc_without_seen": apart(unseen, measure),
                "errors_without_seen": at_threshold(unseen, measure, threshold)["errors"],
                "values": {name_of(r): [r["label"], r[measure]] for r in test},
            },
            "pooled": {
                "auc": apart(pooled, measure), "threshold": pooled_threshold,
                "errors": pooled_errors, "loo_errors": len(pooled_loo),
                "loo_wrong": [name_of(pooled[i]) for i in pooled_loo],
            },
            "judged_values": {name_of(r): [r["label"], r[measure]] for r in judged},
        }
    with open(out, "w") as file:
        json.dump(result, file, indent=1, ensure_ascii=False)
    print_scores(result)


def print_scores(result):
    print("counts:", json.dumps(result["counts"]))
    for measure, s in result["measures"].items():
        d, t, p = s["development"], s["test"], s["pooled"]
        print(f"{measure:14} dev AUC {d['auc_a']:.3f} b~a {d['auc_b_a_different']:.3f}"
              f" b~c {d['auc_b_c']:.3f} b~d {d['auc_b_d']:.3f} thr {d['threshold']:.4g}"
              f" err {d['errors']} LOO {d['loo_errors']} (b) below {len(d['b_below'])}"
              f" (c) above {len(d['c_above'])} (d) above {len(d['d_above'])}")
        print(f"{'':14} test AUC {t['auc']:.3f} errors {t['errors']} {t['wrong']}"
              f" | without seen AUC {t['auc_without_seen']:.3f}"
              f" errors {t['errors_without_seen']}")
        print(f"{'':14} pooled AUC {p['auc']:.3f} errors {p['errors']} LOO {p['loo_errors']}")


def main():
    command, arguments = sys.argv[1], sys.argv[2:]
    if command == "measures":
        measures(arguments)
    elif command == "choose":
        choose(*arguments[:4])
    elif command == "scores":
        scores(*arguments[:7])
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main()
