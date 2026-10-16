"""Outcomes of a `gunfinger-eval shared` run on the shared-material panel
(session 20), by the oracle of docs/brief-20.md: each query against each
index is right confident, wrong confident, possible only or nothing, with
the strongest detection of each role.

usage: shared_outcomes.py [--clusters <clusters.json>] <panel.json> <shared report.json> <out.json>

Roles: the report's `played`, `related` and `partner`; its `other` split
with the panel into `version` (another recording or listed version of
the family), `unjudged` (a file whose tags give one of the family's
titles, relation unknown) and `other`. The blend partner's detections
are set aside: it plays in the query, so it is neither right nor wrong.
A query the harness could not render counts as `not rendered`.

With `--clusters` (session 23), each scenario's played and related
recordings also take in the duplicate clusters of that file's members, so
a rip the clusters of the run did not know is the played (or related)
recording: a confident detection of it is right, also with the played
recording left out, where the index kept it.
"""
import json
import sys
from collections import Counter, defaultdict

OUTCOMES = ["right confident", "wrong confident", "possible only", "nothing", "not rendered"]
KINDS = ["alone", "looped", "beyond", "before", "weak", "blend", "break"]
INDEXES = ["everything", "played left out", "source left out"]
RIGHT = {
    "everything": {"right confident"},
    "played left out": {"possible only", "nothing"},
    "source left out": {"right confident"},
}


def roles_of(panel):
    """For each direction name: (family, version files, unjudged files, control)."""
    out = {}
    for family in panel["families"]:
        recordings = family["recordings"]
        for direction in family["directions"]:
            own = {path for recording in recordings
                   if direction["played"] in recording or direction["related"] in recording
                   for path in recording}
            versions = {path for recording in recordings for path in recording} - own
            versions |= set(family["other_versions"])
            passage = direction["passage"] or {}
            out[direction["name"]] = (family["name"], versions,
                                      set(family["unjudged_same_title"]),
                                      passage.get("control", False))
    return out


def clusters_of(path):
    """A function from a file to the set of files of its cluster."""
    cluster = {}
    for members in json.load(open(path, encoding="utf-8"))["duplicates"]:
        for member in members:
            cluster[member] = frozenset(members)
    return lambda asset: cluster.get(asset, frozenset([asset]))


def expanded(report, cluster_of):
    """For each scenario: its played and related recordings with every
    member's cluster."""
    out = {}
    for scenario in report["scenarios"]:
        played = {scenario["played"], *scenario.get("played_recording", [])}
        related = {scenario["related"], *scenario.get("related_recording", [])}
        out[scenario["name"]] = (
            {f for member in played for f in cluster_of(member)},
            {f for member in related for f in cluster_of(member)},
        )
    return out


def reroled(detection, recordings):
    """The harness's role, with the clusters' rips of the two recordings."""
    if recordings is None or detection["role"] in ("played", "partner"):
        return detection["role"]
    played, related = recordings
    if detection["asset"] in played:
        return "played"
    if detection["asset"] in related:
        return "related"
    return detection["role"]


def role(detection, versions, unjudged):
    if detection["role"] != "other":
        return detection["role"]
    if detection["asset"] in versions:
        return "version"
    if detection["asset"] in unjudged:
        return "unjudged"
    return "other"


def outcome(result, versions, unjudged, recordings=None):
    if result.get("error"):
        return "not rendered", None, False, {}
    found = [dict(d, role=role(dict(d, role=reroled(d, recordings)), versions, unjudged))
             for d in result["detections"] if d["role"] != "partner"]
    confident = [d for d in found if d["level"] == "confident"]
    wrong = sorted((d for d in confident if d["role"] != "played"), key=lambda d: -d["hits"])
    strongest = {}
    for d in found:
        if d["role"] not in strongest or d["hits"] > strongest[d["role"]]["hits"]:
            strongest[d["role"]] = d
    summary = {r: {"asset": d["asset"], "hits": d["hits"], "windows": d["windows"],
                   "level": d["level"]} for r, d in strongest.items()}
    if wrong:
        played_too = any(d["role"] == "played" for d in confident)
        return "wrong confident", wrong[0], played_too, summary
    if confident:
        return "right confident", None, False, summary
    if any(d["level"] == "possible" for d in found):
        return "possible only", None, False, summary
    return "nothing", None, False, summary


def main(*args):
    args = list(args)
    recordings = {}
    if args[0] == "--clusters":
        clusters_path = args[1]
        args = args[2:]
    else:
        clusters_path = None
    panel_path, report_path, out_path = args
    panel = json.load(open(panel_path, encoding="utf-8"))
    report = json.load(open(report_path, encoding="utf-8"))
    if clusters_path:
        recordings = expanded(report, clusters_of(clusters_path))
    roles = roles_of(panel)
    rows = []
    for result in report["results"]:
        family, versions, unjudged, control = roles[result["scenario"]]
        name, wrong, played_too, strongest = outcome(
            result, versions, unjudged, recordings.get(result["scenario"]))
        rows.append({
            "family": family, "direction": result["scenario"], "control": control,
            "kind": result["kind"], "query": result["query"], "seconds": result["seconds"],
            "index": result["index"], "outcome": name,
            "right": name in RIGHT[result["index"]],
            "wrong_role": wrong["role"] if wrong else None,
            "wrong_file": wrong["asset"] if wrong else None,
            "wrong_hits": wrong["hits"] if wrong else None,
            "wrong_windows": wrong["windows"] if wrong else None,
            "played_also_confident": played_too,
            "strongest": strongest,
        })
    json.dump({"rows": rows}, open(out_path, "w", encoding="utf-8"), indent=1, ensure_ascii=False)
    for control in (False, True):
        chosen = [r for r in rows if r["control"] == control]
        if not chosen:
            continue
        print("controls" if control else "directions with a passage",
              f"({len({r['direction'] for r in chosen})} directions, {len(chosen)} queries)")
        print(f"  {'index':<16} {'kind':<7} {'n':>3} " + " ".join(f"{o[:13]:>13}" for o in OUTCOMES)
              + "  wrong by role")
        for index in INDEXES:
            for kind in KINDS:
                cell = [r for r in chosen if r["index"] == index and r["kind"] == kind]
                if not cell:
                    continue
                counts = Counter(r["outcome"] for r in cell)
                by_role = Counter(r["wrong_role"] for r in cell if r["wrong_role"])
                print(f"  {index:<16} {kind:<7} {len(cell):>3} "
                      + " ".join(f"{counts.get(o, 0):>13}" for o in OUTCOMES)
                      + "  " + ", ".join(f"{k} {v}" for k, v in sorted(by_role.items())))
    print("\nwrong confident, by direction and index:")
    grouped = defaultdict(list)
    for r in rows:
        if r["outcome"] == "wrong confident":
            grouped[(r["direction"], r["index"])].append(r)
    for (direction, index), cell in sorted(grouped.items()):
        files = sorted({(r["wrong_role"], r["wrong_file"]) for r in cell})
        queries = ", ".join(f"{r['query'].split(direction + '-')[1]} {r['wrong_hits']}/{r['wrong_windows']}"
                            + (" (played too)" if r["played_also_confident"] else "") for r in cell)
        print(f"  {direction} [{index}] {'; '.join(f'{role}: {f[-60:]}' for role, f in files)}")
        print(f"      {queries}")


if __name__ == "__main__":
    main(*sys.argv[1:])
