"""2026-10-21 (session 35): how often matching a gap against itself merges
two consecutive held-out records into one stretch, from a `mixes` report
(the truth) and the `self-match` listing of the same report.

Two consecutive held-out plays inside one listed gap are a couple. A
couple is merged when one stretch (blocks at MIN_HITS hits spanning
MIN_BLOCK_SECONDS or more, overlapping ones merged, as gap_tests.py
counts them) overlaps each play's own part (the play without its
crossfades) by at least OWN_OVERLAP_SECONDS. Couples are split into
pairs of the shared-material panel (records known to share a passage)
and the others; the control's couples are all others.

usage: python3 -I scripts/analysis/shared_merges.py [--json OUT]
       REPORT LISTING [REPORT LISTING ...]
"""

import argparse
import json
from collections import defaultdict

MIN_HITS = 10
MIN_BLOCK_SECONDS = 30.0
OWN_OVERLAP_SECONDS = 15.0
PANEL = "work/s35/panels/shared-material-seed-2026.json"


def panel_pairs():
    with open(PANEL) as f:
        panel = json.load(f)
    return {
        (d["played"], d["related"])
        for family in panel["families"]
        for d in family["directions"]
        if d.get("passage") and not d["passage"]["control"]
    }


def stretches(gap):
    at = next(b for b in gap["blocks"] if b["min_hits"] == MIN_HITS)
    spans = sorted(s for s in at["spans"] if s[1] - s[0] >= MIN_BLOCK_SECONDS)
    out = []
    for start, end in spans:
        if out and start < out[-1][1]:
            out[-1][1] = max(out[-1][1], end)
        else:
            out.append([start, end])
    return [(gap["start_seconds"] + a, gap["start_seconds"] + b) for a, b in out]


def own(play):
    return (play["start_seconds"] + play["fade_in_seconds"], play["end_seconds"] - play["fade_out_seconds"])


def overlap(a, b):
    return max(0.0, min(a[1], b[1]) - max(a[0], b[0]))


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--json")
    parser.add_argument("files", nargs="+")
    args = parser.parse_args()
    pairs = panel_pairs()
    tally = defaultdict(lambda: {"couples": 0, "merged": 0})
    deficits = defaultdict(list)
    for report_path, listing_path in zip(args.files[::2], args.files[1::2]):
        with open(report_path) as f:
            report = json.load(f)
        with open(listing_path) as f:
            listing = json.load(f)
        plays_of = {m["mix"]["name"]: m["mix"]["plays"] for m in report["mixes"]}
        for gap in listing:
            plays = plays_of[gap["mix"]]
            found = stretches(gap)
            merged_here = 0
            for a, b in zip(plays, plays[1:]):
                if not (a["held_out"] and b["held_out"]):
                    continue
                inside = all(
                    overlap(own(p), (gap["start_seconds"], gap["end_seconds"])) >= OWN_OVERLAP_SECONDS
                    for p in (a, b)
                )
                if not inside:
                    continue
                kind = "panel pair" if (a["asset"], b["asset"]) in pairs else "other"
                merged = any(
                    overlap(s, own(a)) >= OWN_OVERLAP_SECONDS and overlap(s, own(b)) >= OWN_OVERLAP_SECONDS
                    for s in found
                )
                tally[kind]["couples"] += 1
                tally[kind]["merged"] += merged
                merged_here += merged
            if gap["missed_tracks"]:
                deficits[merged_here > 0].append(gap["missed_tracks"] - len(found))
    for kind, row in sorted(tally.items()):
        print(f"{kind}: {row['merged']} of {row['couples']} couples merged")
    for merged, values in sorted(deficits.items()):
        label = "gaps with a merged couple" if merged else "gaps without"
        if values:
            print(
                f"{label}: {len(values)}, truth minus stretches: mean {sum(values) / len(values):.2f}, "
                f"low {sum(v > 0 for v in values)}, exact {sum(v == 0 for v in values)}, high {sum(v < 0 for v in values)}"
            )
    if args.json:
        with open(args.json, "w") as f:
            json.dump({"couples": tally, "deficits": {str(k): v for k, v in deficits.items()}}, f, indent=1)


if __name__ == "__main__":
    main()
