"""Session 59 (experiment 0242): compare two recall reports query by query (by position), each
detection as (asset, confident, windows, hits, speed); detections of the files
named in --drop (one path per line) are set aside first. Prints the queries
that differ and a count."""
import json, sys

args = sys.argv[1:]
drop = set()
if args[0] == "--drop":
    drop = {line.strip() for line in open(args[1]) if line.strip()}
    args = args[2:]
a, b = (json.load(open(path))["queries"] for path in args)
assert len(a) == len(b), (len(a), len(b))


def key(query):
    return [
        (d["asset"], d["confident"], d["windows"], d["hits"], round(d["speed_percent"], 9))
        for d in query["detections"]
        if d["asset"] not in drop
    ]


differ = [i for i, (x, y) in enumerate(zip(a, b)) if key(x) != key(y)]
dropped = sum(1 for q in a for d in q["detections"] if d["asset"] in drop)
for i in differ[:20]:
    x, y = key(a[i]), key(b[i])
    print(f"query {i}: {a[i]['asset'].split('/')[-1]} at {a[i]['speed_percent']:+}%")
    for d in x:
        if d not in y:
            print("   only first: ", d[0].split("/")[-1], d[1:])
    for d in y:
        if d not in x:
            print("   only second:", d[0].split("/")[-1], d[1:])
print(f"{len(differ)} of {len(a)} queries differ; {dropped} detections of dropped files in the first")
