# 0084: the pre-release fixes at NAS scale, after the merge

**Question.** The audit's branch (experiments 0077-0083) was checked at
262 tracks only, where 0083 counted about 20,000 hits in a 10-second
window, below the 50,000 at which its split sort (0080) starts. Does the
merged code give the same detections at NAS scale, and how long does it
take?

**Method.** Master before the merge (`9602a61`, release build kept in
`work/bin/pre-merge/`) and the merged tree (`work/bin/merged/`), one run
each on an idle machine, the pre-merge build first: `gunfinger --config
nas-dnb.toml identify razorjack-2003-03-29.mp3 --store-only --playback
both -f json` (experiment 0075's command; store `f9f9bc706195a71b`, 33,596
records, the saved index warm). `work/merge-check/compare.py` compares
every segment exactly. Reports in `work/merge-check/`.

| 2003 mix (46:10), NAS scale | Before | Merged |
|---|--:|--:|
| Wall | 220.1 s | 181.4 s |
| CPU | 1,903 s | 1,524 s |
| Peak memory | 4.78 GB | 4.78 GB |
| Plays; segments | 27; 37 | 27; 37, identical |

**Differences.** The intended ones only: the search settings end in
`play=max-segment` (ADR 0013), the query records the file's size and
modification time, and plays 4-7 (The Specialist's files) also list
the other titles of copies with identical audio (Sex Drive's scene
copy, the second INFRA012 rip). No play changes confidence.

`regress session-11-start` with the merged build: 720/720 sweep
queries and 87, 93 and 79 scan detections identical (260 s). Its
calibrate line (680 -> 658, 18 -> 46) is the one sessions 7 and 8
explained: it reads every report in `work/reports/`.

**Conclusions.** Same detections at NAS scale, and 18% less wall time
(20% less CPU) from the split sort and the shared key-locked analysis,
one run each.
