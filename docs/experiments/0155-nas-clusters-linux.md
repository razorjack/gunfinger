# 0155: the NAS library map, clusters and baseline on the Linux machine

**Question.** Session 33, steps 2-4: from this machine's NAS store (40,683
records), do map and clusters match the Mac's (0036; 0128, 40,126)? Session 18's
command, `--cut-sparsest`, the panel's NFD path in NFC; run 1 without
`--recall-panel` (it needs clusters first), then the full run 2 (brief, 01:24).

| | the Mac (0128) | the Linux machine |
|---|--:|--:|
| Map: corpus files with a NAS copy | 247 of 262 | 250 of 265; the same 15 without |
| Queries per round; clusters; files (NAS) | 370, 352, 3; 281; 963 (701) | 373, 386, 3; 284; 1,004 (739) |
| Pairs: unjudged, same, different; cut joins; corpus groups | 2,583, 28, 25; 2; the 17 | 2,843, 29, 31; 1; equal |
| Weakest same; strongest unjudged apart | 80.3% (Known Unknown); 74.2% (Stalker) | the same pair, 1,390 hits each; 78.0% |
| Borderline pairs (rows) | 44 (50) | 52 (58), the Mac's 44 among them |
| Wall; user CPU; peak | 32 min; -; 5.7 GB (10 cores) | 52:30; 94,387 s; 7.3 GiB (32 threads) |

- Each difference found has a named cause (0128's data holds counts, not
  members, so swapped members cannot be ruled out). +3 clusters, +3 corpus, +7
  NAS files: the 3 corpus records the Mac passes over and their rips. +31 NAS
  members: records the Mac stored on 2026-10-17, after 0128; 701 remain, the
  Mac's count. All 77 members 0120 and 0128 list as added sit with the same
  corpus file. Verdicts 77-82 (after 0128) judge PRO 012 A ~ the Revision: one
  cut remains (old-skool Sonar ~ the Revision, 100%, 301 hits), and Stalker
  leaves the gap. The harness prints 64 borderline rows: 6 more Sonar ~
  Revision rows at 89-99% that verdicts keep apart.
- Of the 44 shared borderline pairs, 40 have equal hits, 3 differ by 1-2 and
  wobble ~ Tower Bass moves 78 -> 88 (44.7% -> 56.4%): peaks, or the larger
  index's fullest lists, which the search sets aside (not separated). New,
  unjudged (listening questions): ƆƐI3C's China Cup upload ~ five China Cup
  rips (52-78%), Dilusion ~ an upload (73.1%), Hide U ~ a remix (44.6%).
- Per query: 4.1 s (run 1, partly under the earlier profile: 4.3 s), the Mac
  2.7 s (0128). User CPU (124 s a query) counts both threads of a core.

**Baseline** `session-33-linux-nas`: Stakka 11/11, leave-outs 8/11 and 0/11, 0
wrong, 3 possible plays matching no track, as 0067 (`devscore.py --nas` agrees).
Sweep 2026: 540/540, 7 wrong, one held-out query (APH027's Cool Rock Stuff
names the upload 0128 left apart at 41%, 257-282 hits; no Mac NAS sweep since).
`regress` identical (10:55). Index 115 s; sweep 9:00; scans 76-77 s; 5.0-5.5 GiB.

**Conclusion.** The clusters reproduce 0128 where inputs match (`clusters_linux.py`).
