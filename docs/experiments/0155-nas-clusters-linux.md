# 0155: the NAS library map, clusters and baseline on the Linux machine

**Question.** Session 33, steps 2-4: from this machine's NAS store (40,683
records, `56433d74d0919a08`), do the map and clusters match the Mac's (0036;
0128 at 40,126), and what baseline follows? `work/bin/s33-start`; session 18's
command, `--cut-sparsest`, the panel copy with its NFD path in NFC (brief,
01:25); run 1 without `--recall-panel`, which needs a clusters file first, then
the full run 2 (brief, 01:24). `clusters_linux.py`; `data/0155-*.json`.

| | the Mac (0128) | the Linux machine |
|---|--:|--:|
| Map: corpus files with a NAS copy | 247 of 262 | 250 of 265; the same 15 without |
| Queries per round; clusters; files (NAS) | 370, 352, 3; 281; 963 (701) | 373, 386, 3; 284; 1,004 (739) |
| Pairs: unjudged, same, different | 2,583, 28, 25 | 2,843, 29, 31 |
| Corpus groups (the 262-track clusters' 17); cut joins | equal; 2 | equal; 1 |
| Weakest same; strongest unjudged apart | 80.3% (Known Unknown); 74.2% (Stalker) | the same pair, 1,390 hits each; 78.0% |
| Borderline pairs (rows) | 44 (50) | 52 (58), the Mac's 44 among them |
| Wall; user CPU; peak | 32 min; -; 5.7 GB (10 cores) | 52:30; 94,387 s; 7.3 GiB (32 threads) |

- Every difference has a named cause. +3 clusters, +3 corpus and +7 NAS files:
  the 3 corpus records the Mac passes over (Dropzone, Hide U, Dilusion) and
  their rips. +31 NAS members: records the Mac also stored on 2026-10-17, after
  0128; the remaining 701 equal the Mac's count. All 77 members 0120 and 0128
  list as added sit with the same corpus file. Verdicts 77-82 (after 0128) judge
  PRO 012 A ~ the Revision, so one cut remains (old-skool Sonar ~ the Revision,
  100%, 301 hits), and Stalker leaves the gap (now China Cup ~ a newer upload).
- Peaks: of the 44 shared borderline pairs, 40 have equal hits, 3 differ by
  1-2, wobble ~ Tower Bass moves 78 -> 88 hits (44.7% -> 56.4%).
- New borderline pairs, unjudged, listening questions: ƆƐI3C's China Cup
  upload ~ five China Cup rips (52-78%, a newer record); corpus Dilusion ~ an
  upload (73.1%); Hide U ~ its Outrage & Markee Ledge Remix (44.6%, 103 hits).
- Speed: 4.1 s and 124 s of user CPU per query (run 2), against the Mac's 2.7 s
  (0128) and 19 s of CPU (session 18). Run 1 (partly before the power profile
  changed): 4.3 s.

**Baseline** `session-33-linux-nas` (0067 at 32,441 beside): Stakka 11/11,
leave-out 3 8/11, 11 0/11, 0 wrong, 3 possible plays matching no track, as
0067; `devscore.py --nas` agrees. Sweep 2026: 540/540, 7 wrong, all one held-out
query (APH027's Cool Rock Stuff naming the upload 0128 left apart at 41%,
257-282 hits; no Mac NAS sweep since that upload). `regress` identical (10:55).
Index built in 115 s; sweep 9:00; a scan or leave-out 76-77 s; 5.1-5.5 GiB.

**Conclusion.** The clusters reproduce 0128 wherever the inputs match; peaks
change one borderline pair. Clustering is 1.5 times slower per query than on
the Mac, while a NAS scan takes 76 s.
