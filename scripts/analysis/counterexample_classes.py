"""Session 68: a `counterexamples` report's failures by class (experiments
0264-0265). Writes, per class (the failure's kind and its minimal
transformations' kinds): failures, distinct families, the families with
their counts, and the renders the minimizer spent; per sample, how many
held each kind of failure; and the sampled transformation counts against
the minimal ones. File names only.
Usage: counterexample_classes.py REPORT OUT.json"""
import json, sys
from collections import Counter, defaultdict

report = json.load(open(sys.argv[1]))
classes = defaultdict(list)
for counterexample in report["counterexamples"]:
    classes[counterexample["class"]].append(counterexample)


def kinds(transformations):
    return "+".join(t["kind"] for t in transformations) or "none"


summary = {
    "samples": report["samples"],
    "indexed_assets": report["indexed_assets"],
    "evaluations": report["evaluations"],
    "sampling_seconds": round(report["sampling_seconds"], 1),
    "minimizing_seconds": round(report["minimizing_seconds"], 1),
    "failures": len(report["counterexamples"]),
    "samples_failing": len({c["sample"] for c in report["counterexamples"]}),
    "held_out_targets": sum(s["recipe"]["base"]["target"]["held_out"] for s in report["sampled"]),
    "axis_joined": Counter(t["kind"] for s in report["sampled"] for t in s["recipe"]["transformations"]),
    "by_kind": Counter(c["failure"]["kind"] for c in report["counterexamples"]),
    "classes": [],
}
for name, members in sorted(classes.items(), key=lambda item: -len(item[1])):
    families = Counter(c["family"] for c in members)
    sampled_sizes = Counter(len(c["sampled"]["transformations"]) for c in members)
    summary["classes"].append({
        "class": name,
        "failures": len(members),
        "families": len(families),
        "renders": sum(c["evaluations"] for c in members),
        "sampled_sizes": dict(sorted(sampled_sizes.items())),
        "faults": dict(Counter(t["fault"] for c in members for t in c["minimal"]["transformations"] if t["kind"] == "channel-fault")),
        "by_family": dict(families.most_common()),
        "examples": [
            {"number": c["number"], "sampled": kinds(c["sampled"]["transformations"]),
             "minimal": c["minimal"]["transformations"], "outcome": c["minimal_outcome"]}
            for c in members[:3]
        ],
    })
json.dump(summary, open(sys.argv[2], "w"), indent=1, ensure_ascii=False)
for c in summary["classes"]:
    print(f'{c["class"]:<45} {c["failures"]:>4} failures {c["families"]:>3} families {c["renders"]:>5} renders')
print(json.dumps({k: summary[k] for k in ("samples", "failures", "samples_failing", "evaluations", "sampling_seconds", "minimizing_seconds", "by_kind", "axis_joined")}))
