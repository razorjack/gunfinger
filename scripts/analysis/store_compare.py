"""Compares two peak stores' listings, as written by store_records.py and
store_skips.py (for example the Mac's and the Linux machine's): paths on
one side only, raw and after Unicode NFC normalization; for paths on both,
equal and different durations, peak counts, peak digests and sizes; and
skip notes that differ, with both sides' messages.

usage: store_compare.py <records A> <skips A> <records B> <skips B>
                        [--names A B] [--json FILE]
"""
import json, sys, unicodedata
from collections import Counter


def read_tsv(file, width):
    rows = {}
    with open(file, encoding="utf-8") as lines:
        for line in lines:
            fields = line.rstrip("\n").split("\t", width - 1)
            assert len(fields) == width, line
            rows[fields[0]] = fields[1:]
    return rows


def nfc(path):
    return unicodedata.normalize("NFC", path)


def by_nfc(paths):
    keyed = {}
    for path in paths:
        keyed.setdefault(nfc(path), []).append(path)
    collisions = {key: found for key, found in keyed.items() if len(found) > 1}
    return {key: found[0] for key, found in keyed.items()}, collisions


def status(path, records, skips):
    if path in records:
        duration, count, _, _, _ = records[path]
        return f"record ({duration} s, {count} peaks)"
    if path in skips:
        _, _, reason, detail = skips[path]
        return f"{reason}: {detail}"
    return "absent"


options = sys.argv[1:]
names = ["A", "B"]
json_out = None
if "--names" in options:
    at = options.index("--names")
    names = options[at + 1:at + 3]
    del options[at:at + 3]
if "--json" in options:
    at = options.index("--json")
    json_out = options[at + 1]
    del options[at:at + 2]
records_a, skips_a, records_b, skips_b = (
    read_tsv(options[0], 6), read_tsv(options[1], 5),
    read_tsv(options[2], 6), read_tsv(options[3], 5))
a, b = names
summary = {"names": names}

# Records, raw paths.
raw_both = records_a.keys() & records_b.keys()
summary["records"] = {a: len(records_a), b: len(records_b), "both raw": len(raw_both),
                      f"only {a} raw": len(records_a.keys() - records_b.keys()),
                      f"only {b} raw": len(records_b.keys() - records_a.keys())}

# Records, NFC paths.
keyed_a, collisions_a = by_nfc(records_a)
keyed_b, collisions_b = by_nfc(records_b)
not_nfc_a = sorted(p for p in records_a if nfc(p) != p)
not_nfc_b = sorted(p for p in records_b if nfc(p) != p)
both = sorted(keyed_a.keys() & keyed_b.keys())
normalization_only = [key for key in both if keyed_a[key] != keyed_b[key]]
only_a = sorted(keyed_a[key] for key in keyed_a.keys() - keyed_b.keys())
only_b = sorted(keyed_b[key] for key in keyed_b.keys() - keyed_a.keys())
summary["records"].update({
    f"{a} paths not in NFC": len(not_nfc_a), f"{b} paths not in NFC": len(not_nfc_b),
    f"{a} NFC collisions": len(collisions_a), f"{b} NFC collisions": len(collisions_b),
    "both after NFC": len(both), "differ only in normalization": len(normalization_only),
    f"only {a} after NFC": len(only_a), f"only {b} after NFC": len(only_b)})

fields = {"duration": 0, "peak count": 1, "peak digest": 2, "size": 4}
compared = {name: Counter() for name in fields}
different = []
count_deltas = Counter()
duration_deltas = []
for key in both:
    left, right = records_a[keyed_a[key]], records_b[keyed_b[key]]
    differs = []
    for name, column in fields.items():
        equal = left[column] == right[column]
        compared[name]["equal" if equal else "different"] += 1
        if not equal:
            differs.append(name)
    if differs:
        different.append({"path": keyed_b[key], "differs": differs,
                          a: [left[0], left[1], left[2]], b: [right[0], right[1], right[2]]})
    if left[1] != right[1]:
        count_deltas[int(right[1]) - int(left[1])] += 1
    if left[0] != right[0]:
        duration_deltas.append(round(float(right[0]) - float(left[0]), 3))
summary["on both"] = {name: dict(counts) for name, counts in compared.items()}
summary["on both"]["peak count change, B - A"] = dict(sorted(count_deltas.items()))
summary["on both"]["duration change, B - A"] = dict(sorted(Counter(duration_deltas).items()))

# Skip notes, after NFC.
skeyed_a, _ = by_nfc(skips_a)
skeyed_b, _ = by_nfc(skips_b)
reasons = Counter()
one_side = []
for key in sorted(skeyed_a.keys() | skeyed_b.keys()):
    path_a = skeyed_a.get(key) or keyed_a.get(key)
    path_b = skeyed_b.get(key) or keyed_b.get(key)
    left = status(path_a, records_a, skips_a) if path_a else "absent"
    right = status(path_b, records_b, skips_b) if path_b else "absent"
    reason_a = left.split(":")[0] if not left.startswith("record") else "record"
    reason_b = right.split(":")[0] if not right.startswith("record") else "record"
    reasons[f"{reason_a} / {reason_b}"] += 1
    if reason_a != reason_b and "failed" in (reason_a, reason_b):
        one_side.append({"path": path_b or path_a, a: left, b: right})
summary["skip notes"] = {a: len(skips_a), b: len(skips_b),
                         f"{a} by reason": dict(Counter(v[2] for v in skips_a.values())),
                         f"{b} by reason": dict(Counter(v[2] for v in skips_b.values())),
                         f"status {a} / {b}": dict(sorted(reasons.items()))}
summary["failed on one side only"] = one_side

print(json.dumps(summary, indent=1, ensure_ascii=False))
if json_out:
    with open(json_out, "w", encoding="utf-8") as out:
        json.dump({"summary": summary, "different": different, f"only {a}": only_a,
                   f"only {b}": only_b, "differ only in normalization":
                   [[keyed_a[k], keyed_b[k]] for k in normalization_only]},
                  out, indent=1, ensure_ascii=False)
