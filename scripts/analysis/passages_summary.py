"""Session 69: `passages` reports summarised by run (mode and relative-speed
range), pooled over the reports given, and checked against brief 69's
registered bar (experiments 0267 and up). Writes per seed and pooled:
the subjects' positives found (all, up to 8% relative speed, above),
their times, the correspondences by verdict, the control subjects' pairs
linked, the shared-material pairs linked and how far their spans reach
past the panel's passage, the insert's and the partners' positives, the
held-out plays' share inside unidentified spans, and the cost.
Numbers and roles only; no file names.
Usage: passages_summary.py OUT.json REPORT..."""
import json, sys
from statistics import median

out, paths = sys.argv[1], sys.argv[2:]


def role(r):
    return r["role"]


def tally(runs):
    positives = [p for run in runs for p in run["score"]["positives"]]
    subjects = [p for p in positives if role(p["roles"][0]) == "subject"]
    near = [p for p in subjects if abs(p["relative_speed"] - 1) <= 0.08 and abs(1 / p["relative_speed"] - 1) <= 0.08]
    far = [p for p in subjects if p not in near]
    found = [p for p in subjects if p["found"]]
    verdicts = {}
    for run in runs:
        for c in run["score"]["correspondences"]:
            verdicts[c["verdict"]] = verdicts.get(c["verdict"], 0) + 1
    shared = [c for run in runs for c in run["score"]["correspondences"] if c["verdict"] == "shared-passage"]
    other = [p for p in positives if role(p["roles"][0]) != "subject"]

    def share(ps):
        return [sum(p["found"] for p in ps), len(ps)]

    def med(values):
        return round(median(values), 1) if values else None

    return {
        "subjects": share(subjects),
        "subjects_up_to_8_percent": share(near),
        "subjects_above_8_percent": share(far),
        "start_late_seconds_median": med([x for p in found for x in p["start_late_seconds"]]),
        "end_early_seconds_median": med([x for p in found for x in p["end_early_seconds"]]),
        "covered_share_median": med([p["covered_share"] for p in found]),
        "verdicts": verdicts,
        "control_pairs_linked": [sum(c["linked"] for run in runs for c in run["score"]["control_pairs"]),
                                 sum(len(run["score"]["control_pairs"]) for run in runs)],
        "shared_pairs_linked": [sum(sum(run["score"]["shared_linked"]) for run in runs),
                                sum(len(run["score"]["shared_linked"]) for run in runs)],
        "shared_most_outside_passage_seconds": round(max((max(c["outside_passage_seconds"]) for c in shared), default=0), 1),
        "insert_positives": share([p for p in other if role(p["roles"][0]) == "insert"]),
        "partner_positives": share([p for p in other if role(p["roles"][0]) == "partner"]),
        "pieces": sum(run["pieces"] for run in runs),
        "piece_seconds": round(sum(run["piece_seconds"] for run in runs)),
        "index_seconds": round(sum(run["index_seconds"] for run in runs), 1),
        "search_seconds": round(sum(run["search_seconds"] for run in runs), 1),
        "search_thread_seconds": round(sum(run["search_thread_seconds"] for run in runs), 1),
    }


def bar(gaps18, gaps8_cost):
    s, far = gaps18["subjects"], gaps18["subjects_above_8_percent"]
    return {
        "recall_80": s[0] >= 0.8 * s[1],
        "above_8_percent_70": far[0] >= 0.7 * far[1],
        "no_other": gaps18["verdicts"].get("other", 0) == 0,
        "control_pairs_at_most_2": gaps18["control_pairs_linked"][0] <= 2,
        "shared_within_15_s": gaps18["shared_most_outside_passage_seconds"] <= 15,
        "cost_at_most_3x": gaps18["search_thread_seconds"] <= 3 * gaps8_cost,
    }


reports = [json.load(open(path)) for path in paths]
keys = sorted({(run["mode"], run["relative_speed_range"]) for r in reports for run in r["runs"]})
summary = {"seeds": {}, "pooled": {}}
for r in reports:
    by = {(run["mode"], run["relative_speed_range"]): run for run in r["runs"]}
    held = [c for s in r["library_searches"] for _, c in s["held_out_covered"]]
    seed = {
        "mixes": len(r["batch"]["mixes"]),
        "mix_seconds": round(sum(m["seconds"] for m in r["batch"]["mixes"])),
        "render_and_search_seconds": round(r["render_and_search_seconds"], 1),
        "library_wrong": sum(s["wrong"] for s in r["library_searches"]),
        "held_out_share_unidentified_median": round(median(held), 2),
        "held_out_plays_without_unidentified_time": [sum(c == 0 for c in held), len(held)],
        "runs": {f"{m} {g}": tally([by[(m, g)]]) for m, g in keys},
    }
    if ("gaps", 0.18) in by and ("gaps", 0.08) in by:
        seed["bar"] = bar(seed["runs"]["gaps 0.18"], seed["runs"]["gaps 0.08"]["search_thread_seconds"])
    summary["seeds"][r["seed"]] = seed
for m, g in keys:
    summary["pooled"][f"{m} {g}"] = tally([run for r in reports for run in r["runs"] if (run["mode"], run["relative_speed_range"]) == (m, g)])
json.dump(summary, open(out, "w"), indent=1)
for seed, s in summary["seeds"].items():
    print(seed, "bar:", s.get("bar"))
for key, t in summary["pooled"].items():
    print(key, t["subjects"], t["subjects_up_to_8_percent"], t["subjects_above_8_percent"], t["verdicts"], "controls", t["control_pairs_linked"], "shared", t["shared_pairs_linked"], t["shared_most_outside_passage_seconds"], "thread s", t["search_thread_seconds"])
