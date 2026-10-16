"""Two shared-material panels' outcomes side by side (session 25, step
4b), from shared_outcomes.py's rows, in the layout of experiment 0111:
for the directions with a passage, the main case (`played left out`) by
kind of query; the other indexes' summaries; the controls' strongest
related file; and every wrong confident title by role and file.

usage: shared_side_by_side.py NAME=OUTCOMES.json... OUT.json
"""
import json
import sys
from collections import Counter, defaultdict

KINDS = ["alone", "looped", "beyond", "before", "weak", "blend", "break"]
OUTCOMES = ["wrong confident", "possible only", "nothing", "right confident", "not rendered"]


def summary(rows):
    passage = [r for r in rows if not r["control"]]
    main = [r for r in passage if r["index"] == "played left out"]
    everything = [r for r in passage if r["index"] == "everything"]
    source = [r for r in passage if r["index"] == "source left out"]
    by_kind = {}
    for kind in KINDS:
        cell = [r for r in main if r["kind"] == kind]
        if cell:
            counts = Counter(r["outcome"] for r in cell)
            by_kind[kind] = {"queries": len(cell), **{o: counts.get(o, 0) for o in OUTCOMES if counts.get(o)}}
    totals = Counter(r["outcome"] for r in main)
    wrong_titles = defaultdict(list)
    for r in main:
        if r["outcome"] == "wrong confident":
            wrong_titles[(r["direction"], r["wrong_role"], r["wrong_file"])].append(r["wrong_hits"])
    rip_confident = [r for r in source if r["outcome"] == "right confident"
                     or (r["outcome"] == "wrong confident" and r["played_also_confident"])]
    controls = [r for r in rows if r["control"]]
    control_related = max((r["strongest"].get("related", {}).get("hits", 0) for r in controls), default=None)
    return {
        "directions": len({r["direction"] for r in passage}),
        "control_directions": len({r["direction"] for r in controls}),
        "played left out": {
            "queries": len(main),
            **{o: totals.get(o, 0) for o in OUTCOMES},
            "directions with a wrong confident query": len({r["direction"] for r in main if r["outcome"] == "wrong confident"}),
            "wrong by role": dict(Counter(r["wrong_role"] for r in main if r["wrong_role"])),
            "by kind": by_kind,
        },
        "everything": {
            "queries": len(everything),
            "wrong confident": sum(r["outcome"] == "wrong confident" for r in everything),
            "wrong beside the played file's confident play": sum(
                r["outcome"] == "wrong confident" and r["played_also_confident"] for r in everything),
            "right confident": sum(r["outcome"] == "right confident" for r in everything),
        },
        "source left out": {
            "directions": len({r["direction"] for r in source}),
            "queries": len(source),
            "rip confident": len(rip_confident),
            "rip confident with a wrong title too": sum(r["outcome"] == "wrong confident" for r in rip_confident),
        },
        "controls": {"queries": len(controls), "strongest related hits": control_related,
                     "wrong confident": sum(r["outcome"] == "wrong confident" for r in controls)},
        "wrong titles, played left out": [
            {"direction": d, "role": role, "file": f, "queries": len(h), "hits": [min(h), max(h)]}
            for (d, role, f), h in sorted(wrong_titles.items())],
    }


def main(argv):
    *named, out = argv
    result = {}
    for item in named:
        name, path = item.split("=", 1)
        result[name] = summary(json.load(open(path, encoding="utf-8"))["rows"])
    with open(out, "w", encoding="utf-8") as file:
        json.dump(result, file, indent=1, ensure_ascii=False)
    names = list(result)
    print(f"{'played left out':<28}" + "".join(f"{n:>22}" for n in names))
    for key in ["queries"] + OUTCOMES + ["directions with a wrong confident query"]:
        print(f"  {key:<26}" + "".join(f"{str(result[n]['played left out'].get(key, '')):>22}" for n in names))
    for kind in KINDS:
        cells = []
        for n in names:
            c = result[n]["played left out"]["by kind"].get(kind)
            cells.append(f"{c.get('wrong confident', 0)}/{c['queries']}" if c else "-")
        print(f"  wrong/queries {kind:<12}" + "".join(f"{c:>22}" for c in cells))
    for section in ["everything", "source left out", "controls"]:
        for key in result[names[0]][section]:
            print(f"{section + ': ' + key:<60}" + "".join(f"{str(result[n][section].get(key)):>10}" for n in names))
    for n in names:
        print(f"\n{n}: wrong titles with the played recording left out")
        for w in result[n]["wrong titles, played left out"]:
            print(f"  {w['direction']:<20} {w['role']:<9} {w['queries']:>2} queries, {w['hits'][0]}-{w['hits'][1]} hits  {w['file'][-70:]}")


if __name__ == "__main__":
    main(sys.argv[1:])
