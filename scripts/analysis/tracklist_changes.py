"""What a new build changes in the tracklists and cue sheets of saved
`identify` reports, rendered by `gunfinger show` with an old and a new
binary. Reports with the same bytes are rendered once.

usage: python3 -I scripts/analysis/tracklist_changes.py OLD_BIN NEW_BIN OUT.json REPORT...

For each distinct report: the number of tracklist entries and cue tracks
under each build, and the entries that changed (an old entry gone or
renamed, a new one added), with the plays behind them: asset, start, end,
the mix time at which the track would have started, and the tags.
Refuses any path naming the held-out test set.
"""

import hashlib
import json
import re
import subprocess
import sys
from pathlib import Path

TEST_SET = ("ed-rush-optical-essential-mix", "EM_1999_02_07")


def render(binary, report, fmt):
    result = subprocess.run(
        [binary, "--color", "never", "show", report, "-f", fmt],
        capture_output=True,
        text=True,
        check=True,
    )
    return result.stdout


def entry_lines(tracklist):
    """Each entry without its number, so a renumbering is not a change."""
    return [re.sub(r"^\s*\d+\.\s+", "", line) for line in tracklist.splitlines() if line.strip()]


def cue_tracks(cue):
    return sum(1 for line in cue.splitlines() if line.startswith("  TRACK "))


def plays_summary(report):
    plays = []
    for number, play in enumerate(report["plays"], 1):
        tags = play.get("tags") or {}
        plays.append(
            {
                "play": number,
                "asset": play["asset"],
                "confidence": play["confidence"],
                "hits": play["hits"],
                "start": round(play["start_seconds"], 2),
                "end": round(play["end_seconds"], 2),
                "track_started_at": round(
                    play["start_seconds"] - play.get("track_start_seconds", 0.0) / play["speed"], 2
                ),
                "artist": tags.get("artist"),
                "title": tags.get("title"),
            }
        )
    return plays


def main():
    old_bin, new_bin, out = sys.argv[1:4]
    reports = sys.argv[4:]
    for path in reports:
        if any(name in path for name in TEST_SET):
            sys.exit(f"refusing {path}: the held-out test set is never read")
    seen = {}
    for path in reports:
        digest = hashlib.sha256(Path(path).read_bytes()).hexdigest()
        seen.setdefault(digest, []).append(path)
    results = []
    for digest, paths in seen.items():
        report = paths[0]
        old_list = render(old_bin, report, "tracklist")
        new_list = render(new_bin, report, "tracklist")
        old_entries, new_entries = entry_lines(old_list), entry_lines(new_list)
        removed = [line for line in old_entries if line not in new_entries]
        added = [line for line in new_entries if line not in old_entries]
        data = json.loads(Path(report).read_text())
        results.append(
            {
                "reports": paths,
                "mix": Path(data["query"]["path"]).name,
                "library": data.get("library"),
                "playback": data["query"].get("playback"),
                "plays": len(data["plays"]),
                "tagged_plays": sum(1 for play in data["plays"] if play.get("tags")),
                "entries_old": len(old_entries),
                "entries_new": len(new_entries),
                "cue_tracks_old": cue_tracks(render(old_bin, report, "cue")),
                "cue_tracks_new": cue_tracks(render(new_bin, report, "cue")),
                "removed": removed,
                "added": added,
                "play_details": plays_summary(data) if removed or added else [],
            }
        )
    Path(out).write_text(json.dumps(results, indent=1, ensure_ascii=False) + "\n")
    changed = [result for result in results if result["removed"] or result["added"]]
    print(f"{len(reports)} reports, {len(results)} distinct, {len(changed)} with a changed tracklist")
    for result in results:
        print(
            f"{result['entries_old']:>3} -> {result['entries_new']:>3} entries, "
            f"cue {result['cue_tracks_old']:>2} -> {result['cue_tracks_new']:>2}, "
            f"{result['tagged_plays']:>2}/{result['plays']:>2} plays tagged, "
            f"{result['playback']}, {result['mix'][:30]}, {result['library'][-26:]}: "
            f"{', '.join(result['reports'])}"
        )
        for line in result["removed"]:
            print(f"      - {line}")
        for line in result["added"]:
            print(f"      + {line}")


main()
