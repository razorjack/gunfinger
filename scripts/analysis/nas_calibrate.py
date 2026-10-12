"""Calibrate's margins at NAS scale, split by what the false candidates are.

Collects the samples as `gunfinger-eval calibrate` does (each indexed sweep
excerpt's strongest correct detection and each referenced track's
strongest candidate; every incorrect sweep detection and each scan's kept
false candidates) and labels each false candidate from the tags against
the recording it was found in: the same artist and title (another rip the
clusters did not join), a version of the same title, the same artist, or
unrelated. The rules count every one as false; the split shows what the
margin would be if the same-name ones are confirmed as rips by ear.

A hand-labels file (JSON: "query|found" -> label) overrides the tag
labels; the confident false candidates are labelled by hand there, since
uploads tag artists and remixers in their own ways.

usage: nas_calibrate.py <reports dir> <tags.json> <library-map.json> <output.json>
                        [hand-labels.json] [development set]
"""
import collections, glob, json, os, sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import tag_labels  # noqa: E402

reports, tags_file, map_file, output = sys.argv[1:5]
hand = json.load(open(sys.argv[5])) if len(sys.argv) > 5 and os.path.exists(sys.argv[5]) else {}
development_set = sys.argv[6] if len(sys.argv) > 6 else "stakka-skynet-knowledge"
tags = json.load(open(tags_file))
stands_for = {c["corpus"]: c["stands_for"] for c in json.load(open(map_file))["copies"]}


def label(query, found):
    return tag_labels.label(query, found, tags, stands_for)


def name(path):
    return tag_labels.name(path, tags, stands_for)


identifying, false = [], []
for path in sorted(glob.glob(os.path.join(reports, "sweep-seed-*.json"))):
    sweep = json.load(open(path))
    for q in sweep["queries"]:
        where = f"sweep {sweep['seed']} {q['asset']} at {q['speed_percent']:+.0f}%"
        correct = [d["hits"] for d in q["detections"] if d["correct"]]
        if not q["held_out"]:
            identifying.append((max(correct, default=0), where))
        for d in q["detections"]:
            if not d["correct"]:
                false.append({"hits": d["hits"], "windows": d["windows"], "confident": d["confident"], "query": q["asset"],
                              "found": d["asset"], "where": where, "held_out": q["held_out"]})
for path in sorted(glob.glob(os.path.join(reports, f"scan-{development_set}*.json"))):
    scan = json.load(open(path))
    tracks = scan["score"]["tracks"]
    for t in tracks:
        if t["referenced"] and t["label"] not in scan["left_out_tracks"]:
            identifying.append(((t["strongest_candidate"] or {"hits": 0})["hits"], f"{os.path.basename(path)}: {t['label']}"))
    for d in scan["score"]["false_candidates"]:
        # What was being played: the manifest slot holding the detection's
        # middle, by its credited asset, or by its label when left out.
        middle = (d["start_seconds"] + d["end_seconds"]) / 2
        slot = max((t for t in tracks if t["start_seconds"] <= middle), key=lambda t: t["start_seconds"], default=tracks[0])
        played = slot["credited"][0]["asset"] if slot["credited"] else slot["label"]
        false.append({"hits": d["hits"], "windows": d["windows"], "confident": d["confident"], "query": played,
                      "found": d["asset"], "where": f"{os.path.basename(path)} at {d['start_seconds']:.0f} s", "held_out": False})

for f in false:
    f["label"] = hand.get(f"{f['query']}|{f['found']}") or label(f["query"], f["found"])
    f["query_name"], f["found_name"] = " - ".join(name(f["query"])), " - ".join(name(f["found"]))

weakest = min(identifying)
summary = {"reports": reports, "identifying": len(identifying), "false_candidates": len(false),
           "weakest_identifying": {"hits": weakest[0], "where": weakest[1]},
           "confident_false": collections.Counter(f["label"] for f in false if f["confident"]),
           "by_label": {}}
for group in ("same name", "version", "same artist", "unrelated"):
    members = sorted((f for f in false if f["label"] == group), key=lambda f: -f["hits"])
    summary["by_label"][group] = {"count": len(members), "confident": sum(f["confident"] for f in members),
                                  "at_60": sum(f["hits"] >= 60 for f in members),
                                  "strongest": members[:40]}
everything = sorted(false, key=lambda f: -f["hits"])
without_same_name = [f for f in everything if f["label"] != "same name"]
unrelated = [f for f in everything if f["label"] == "unrelated"]
held_out = [f for f in everything if f["held_out"]]
summary["strongest"] = {
    "all": everything[0] if everything else None,
    "without_same_name": without_same_name[0] if without_same_name else None,
    "unrelated": unrelated[0] if unrelated else None,
    "audio_not_in_the_index": held_out[0] if held_out else None,
    "audio_not_in_the_index_without_same_name": next((f for f in held_out if f["label"] != "same name"), None),
}
summary["margins"] = {k: (weakest[0] / v["hits"] if v else None) for k, v in summary["strongest"].items()}
summary["confident_false_list"] = [f for f in everything if f["confident"]]
summary["confident_false_pairs"] = sorted({(f["query"], f["found"], f["label"], f["query_name"], f["found_name"])
                                            for f in everything if f["confident"]})
json.dump(summary, open(output, "w"), indent=1, default=dict)
print(f"identifying {len(identifying)}, weakest {weakest}")
print("confident false by label:", dict(summary["confident_false"]))
for k, v in summary["strongest"].items():
    if v:
        print(f"strongest {k}: {v['hits']} ({v['label']}) {v['query_name']} ~ {v['found_name']} [{v['where']}]; margin {summary['margins'][k]:.2f}x")
for group, row in summary["by_label"].items():
    print(group, {k: row[k] for k in ("count", "confident", "at_60")})
