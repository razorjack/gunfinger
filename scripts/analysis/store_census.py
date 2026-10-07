"""Census of a peak store from its record headers, tag sidecars and skip
notes, without the library: folders, formats, lengths, tag coverage,
identical peak records and names on several files. Opens every file
read-only and writes nothing into the store.

usage: store_census.py <peak store> <output.json>
"""
import collections, hashlib, json, os, re, statistics, struct, sys

store, output = sys.argv[1], sys.argv[2]


def string(data, at):
    (length,) = struct.unpack_from("<H", data, at)
    at += 2
    return data[at:at + length].decode("utf-8"), at + length


def source(data, at):
    path, at = string(data, at)
    size, seconds, nanos = struct.unpack_from("<QqI", data, at)
    return {"path": path, "size": size, "mtime": seconds + nanos / 1e9}, at + 20


def record(data):
    assert data[:8] == b"GUNFPEAK"
    profile, at = string(data, 10)
    asset, at = source(data, at)
    duration, count = struct.unpack_from("<dI", data, at)
    peaks = hashlib.sha256(data[at + 8:]).hexdigest()
    return {**asset, "profile": profile, "duration": duration, "peaks": count, "digest": peaks}


def tags(data):
    assert data[:8] == b"GUNFTAGS"
    asset, at = source(data, 10)
    fields = {}
    for name in ("artist", "title", "album"):
        present = data[at]
        at += 1
        if present:
            fields[name], at = string(data, at)
    return asset, fields


def skip(data):
    assert data[:8] == b"GUNFSKIP"
    asset, at = source(data, 10)
    reason = ["failed", "too long", "too short"][data[at]]
    return {**asset, "reason": reason}


def folder(path):
    parts = path.split("/")
    if parts[0] == "__youtube_archivists" and len(parts) > 2:
        return "youtube/" + parts[1]
    return parts[0] if len(parts) > 1 else "(root)"


def name_key(fields):
    if "artist" not in fields or "title" not in fields:
        return None
    text = f"{fields['artist']} - {fields['title']}".lower()
    return re.sub(r"[^a-z0-9]+", " ", text).strip()


records, tag_notes, skips = {}, {}, []
for entry in sorted(os.listdir(store)):
    path = os.path.join(store, entry)
    with open(path, "rb") as file:
        data = file.read()
    if entry.endswith(".peaks"):
        item = record(data)
        records[item["path"]] = item
    elif entry.endswith(".tags"):
        asset, fields = tags(data)
        tag_notes[asset["path"]] = (asset, fields)
    elif entry.endswith(".skip"):
        skips.append(skip(data))

by_folder = collections.defaultdict(lambda: {"records": 0, "hours": 0.0, "artist_title": 0, "title_only": 0, "none": 0})
formats = collections.Counter()
coverage = collections.Counter()
names = collections.defaultdict(list)
untagged_or_stale = 0
for path, item in records.items():
    group = by_folder[folder(path)]
    group["records"] += 1
    group["hours"] += item["duration"] / 3600
    formats[os.path.splitext(path)[1].lower()] += 1
    note = tag_notes.get(path)
    current = note is not None and note[0]["size"] == item["size"] and abs(note[0]["mtime"] - item["mtime"]) < 1e-6
    fields = note[1] if current else {}
    untagged_or_stale += not current
    kind = "artist_title" if "artist" in fields and "title" in fields else "title_only" if "title" in fields else "none"
    group[kind] += 1
    coverage[kind] += 1
    key = name_key(fields)
    if key:
        names[key].append(path)

digests = collections.defaultdict(list)
for path, item in records.items():
    digests[(item["duration"], item["digest"])].append(path)
copies = sorted(sorted(paths) for paths in digests.values() if len(paths) > 1)

durations = [item["duration"] for item in records.values()]
shared = {key: paths for key, paths in names.items() if len(paths) > 1}
shared_across = sum(len({folder(p) for p in paths}) > 1 for paths in shared.values())
summary = {
    "records": len(records),
    "profiles": collections.Counter(item["profile"] for item in records.values()),
    "hours": sum(durations) / 3600,
    "duration_minutes": {
        "mean": statistics.mean(durations) / 60,
        "median": statistics.median(durations) / 60,
        "min": min(durations) / 60,
        "max": max(durations) / 60,
        "deciles": [d / 60 for d in statistics.quantiles(durations, n=10)],
    },
    "source_bytes": sum(item["size"] for item in records.values()),
    "record_bytes": sum(os.path.getsize(os.path.join(store, entry)) for entry in os.listdir(store) if entry.endswith(".peaks")),
    "peaks": sum(item["peaks"] for item in records.values()),
    "skip_notes": collections.Counter(item["reason"] for item in skips),
    "skip_bytes": {reason: sum(s["size"] for s in skips if s["reason"] == reason) for reason in ("failed", "too long", "too short")},
    "tag_coverage": coverage,
    "records_without_current_tags": untagged_or_stale,
    "formats": formats,
    "folders": dict(sorted(by_folder.items(), key=lambda kv: -kv[1]["records"])),
    "identical_peak_records": {
        "groups": len(copies),
        "files_beyond_the_first": sum(len(group) - 1 for group in copies),
        "groups_list": copies,
    },
    "names_on_several_files": {
        "names": len(shared),
        "files": sum(len(paths) for paths in shared.values()),
        "names_across_folders": shared_across,
        "largest": sorted(((len(p), k) for k, p in shared.items()), reverse=True)[:20],
    },
}
with open(output, "w") as out:
    json.dump(summary, out, indent=1, default=dict)
brief = {k: v for k, v in summary.items() if k not in ("folders", "identical_peak_records", "names_on_several_files")}
print(json.dumps(brief, indent=1, default=dict))
print("folders:")
for name, group in summary["folders"].items():
    print(f"  {name}: {group['records']} records, {group['hours']:.0f} h, artist+title {group['artist_title']}, title only {group['title_only']}, none {group['none']}")
print("identical peak records:", summary["identical_peak_records"]["groups"], "groups,", summary["identical_peak_records"]["files_beyond_the_first"], "files beyond the first")
print("names on several files:", {k: v for k, v in summary["names_on_several_files"].items() if k != "largest"})
