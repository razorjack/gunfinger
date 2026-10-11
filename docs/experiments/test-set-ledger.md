# Test-set ledger: `ed-rush-optical-essential-mix`

Every evaluation of the held-out test set, in order. At most five in total.

## Evaluation 1: first run at `poc-freeze-1`

- Code: tag `poc-freeze-1` (commit 45d0ac8). Nothing changed since the
  freeze.
- Command: `target/release/gunfinger-eval scan ed-rush-optical-essential-mix`
- Why: the first and verdict-giving run.
- Result: **15/17 referenced tracks identified (88.2%), 0 wrong
  identifications. Pass.** 1:58:11 of audio scanned in 52.8 s.

Output, verbatim:

```text
ed-rush-optical-essential-mix: 15/17 referenced tracks identified, 0 wrong identifications; 1:58:11 of audio scanned in 52.8 s
  absent  1. Optical & Ryme Tyme - Headhunters
  found   2. Bad Company - The Nine  best 762 hits/26 windows at +0.65% 2:23-6:37
  absent  3. Ram Trilogy - Terminal 1
  absent  4. Jonny L - The Bells
  found   5. Ed Rush & Optical - Fixation  best 789 hits/30 windows at +2.73% 13:10-18:43
  absent  6. Ram Trilogy - Mind Overload
  found   7. DJ Trace - Sonar  best 729 hits/28 windows at +4.54% 23:01-27:49
  found   8. Krust - Warhead (Ram Trilogy Remix)  best 854 hits/29 windows at +2.42% 26:30-31:32
  absent  9. Matrix - Airhead
  found  10. Bad Company - The Pulse  best 913 hits/20 windows at +1.40% 36:17-39:29
  found  11. Ed Rush & Optical - Bacteria  best 493 hits/18 windows at +4.76% 39:13-42:01
  absent 12. DJ Phantasy & DJ Probe - Orders (DJ Reality Remix)
  found  13. Ryme Tyme - Payback Pt. 1  best 1233 hits/29 windows at +3.23% 47:01-52:15
  found  14. Roni Size / Reprazent - Watching Windows (DJ Die Remix)  best 458 hits/19 windows at +5.34% 51:50-54:58
  absent 15. Matrix - Asylum
  MISSED 16. Ed Rush & Optical - Funktion  best 18 hits/2 windows at -1.87% 1:01:45-1:01:54
  found  17. Bad Company - China Cup  best 371 hits/18 windows at +0.40% 1:02:23-1:06:08
  absent 18. Ram Trilogy - Iron Lung
  absent 19. Optical - Newoptic
  found  20. Ed Rush & Optical - Dozer  best 1784 hits/31 windows at +3.07% 1:13:25-1:18:27
  absent 21. DJ Die - Clear Skyz
  MISSED 22. Fibre Optix - Sin  best 84 hits/2 windows at +5.43% 1:22:21-1:22:39
  absent 23. Usual Suspects - Killa Beez
  absent 24. Ram Trilogy - Chase Scene
  found  25. Ed Rush & Optical - Watermelon  best 714 hits/18 windows at +3.66% 1:32:20-1:35:24
  absent 26. Ram Trilogy - System Error (Y2K)
  found  27. Ed Rush & Optical - Gas Mask  best 2010 hits/42 windows at +0.40% 1:39:06-1:46:19
  absent 28. Infinite (Optical & Fierce) - Beachball
  found  29. Ed Rush, Optical & Fierce - Alien Girl  best 1059 hits/27 windows at +5.65% 1:50:00-1:55:19
  found  30. Optical - Slip Thru  best 868 hits/20 windows at +2.41% 1:54:20-1:57:44
  found  31. Ed Rush & Optical - Sick Note  best 209 hits/6 windows at +3.81% 1:57:24-1:58:10
```

Diagnosis, from the same report (no further run):

- No confident detection is wrong. The strongest detection that matched no
  track had 24 hits in 3 windows (weakest credited: 209 hits, Sick Note, the
  last track; the file ends 27 s after its listed start, and the detection
  covers the last 46 s).
- Track 22, Fibre Optix "Sin": two chains of 84 hits each (2 and 4 windows)
  at a consistent +5.4% inside its slot, 82:22 to 84:29. The record was found
  but the evidence stays below the rule (200 hits and 3 windows in one
  chain); the two chains are 34 s apart, beyond the 2-window gap a chain may
  bridge.
- Track 16, Ed Rush & Optical "Funktion": only chains of 8 to 18 hits, but
  most of them inside its slot (58:11 to 61:54) agree on a speed of about
  -1.8%. Something related was playing at that speed; the library recording
  barely aligns. Possibly a different version or mixdown was played.

No further test evaluations: the first run passed.

## Review of evaluation 1 by ear (2026-10-06)

The items listed for checking were checked by ear. No search was run; the
evaluation count stays at 1.

- **16. Funktion.** The version played is "Funktion (Remix)" from Planet V
  (V Recordings, 1999), not the original from the Funktion / Naked Lunch
  single (V026, 1998) that the library holds. Sources: the listening,
  and MixesDB and Dogs On Acid tracklists found in earlier research (not
  checked here). The remix is not in the library, so the slot is in effect
  absent, and the outcome is correct: the original stayed far below the rule
  (best 18 hits) while its remix played. We have since changed the
  manifest: track 16 is `title = "Funktion (Remix)"`, `reference = []`
  (`gunfinger-eval validate`: 31 tracks, 16 referenced, 15 absent). Under
  that manifest the evaluation-1 detections score 15/16 (93.8%) with 0 wrong.
  That figure is arithmetic on the stored report, not a new run.
- **22. Sin.** The broadcast faded Sin out, played a station insert and
  brought Sin back. The two chains at +5.4% are the record before and after
  the insert. Joined, they would hold 168 hits, still below the 200-hit rule,
  so the miss stands.
- **31. Sick Note.** Played solo for under 30 s at the end of the broadcast,
  after a blend out of Slip Thru. The detection (1:57:25 to 1:58:11) covers
  the blend and the solo part.
- **10. The Pulse.** A needle skip at 36:17. A skip moves the needle to
  another point of the record, so the position in the track jumps while mix
  time runs on. The hits move to a new line and a new chain starts, which is
  the split at 36:17. The report's first reading, a pitch adjustment, was
  wrong.

## Evaluation 2: plays and the possible tier

- Code: commit a58e7a9. Changes since evaluation 1: the elegance pass (no
  change in results), the correction of track 16 in the manifest
  (now "Funktion (Remix)", absent from the library), and plays with a
  possible tier (ADR 0006, experiment 0006). Search and the confident rule
  are unchanged.
- Commands: `target/release/gunfinger-eval scan ed-rush-optical-essential-mix`,
  then `target/release/gunfinger identify <mix> --library corpus/library` on
  the same build, as one evaluation.
- Why: the possible tier can only be checked on a set with brief plays, and
  this is the only one. Not blind for that tier: the evaluation-1 report,
  with Sin's 84-hit chains, was inspected before the decision.
- Result: **15/16 referenced tracks identified (93.8%), 0 wrong
  identifications**, with all 5,923 detections identical to evaluation 1.
  Possible tier: Sin found as possible (one play, two segments, 168 hits);
  no possible play matches no track. 1:58:11 scanned in 48.8 s.

Scan output, verbatim:

```text
ed-rush-optical-essential-mix: 15/16 referenced tracks identified, 0 wrong identifications; 1:58:11 of audio scanned in 48.8 s
possible tier: 1 more referenced tracks found as possible, 0 possible plays match no track
  absent    1. Optical & Ryme Tyme - Headhunters
  found     2. Bad Company - The Nine  best 762 hits/26 windows at +0.65% 2:23-6:37
  absent    3. Ram Trilogy - Terminal 1
  absent    4. Jonny L - The Bells
  found     5. Ed Rush & Optical - Fixation  best 789 hits/30 windows at +2.73% 13:10-18:43
  absent    6. Ram Trilogy - Mind Overload
  found     7. DJ Trace - Sonar  best 729 hits/28 windows at +4.54% 23:01-27:49
  found     8. Krust - Warhead (Ram Trilogy Remix)  best 854 hits/29 windows at +2.42% 26:30-31:32
  absent    9. Matrix - Airhead
  found    10. Bad Company - The Pulse  best 913 hits/20 windows at +1.40% 36:17-39:29
  found    11. Ed Rush & Optical - Bacteria  best 493 hits/18 windows at +4.76% 39:13-42:01
  absent   12. DJ Phantasy & DJ Probe - Orders (DJ Reality Remix)
  found    13. Ryme Tyme - Payback Pt. 1  best 1233 hits/29 windows at +3.23% 47:01-52:15
  found    14. Roni Size / Reprazent - Watching Windows (DJ Die Remix)  best 458 hits/19 windows at +5.34% 51:50-54:58
  absent   15. Matrix - Asylum
  absent   16. Ed Rush & Optical - Funktion (Remix)
  found    17. Bad Company - China Cup  best 371 hits/18 windows at +0.40% 1:02:23-1:06:08
  absent   18. Ram Trilogy - Iron Lung
  absent   19. Optical - Newoptic
  found    20. Ed Rush & Optical - Dozer  best 1784 hits/31 windows at +3.07% 1:13:25-1:18:27
  absent   21. DJ Die - Clear Skyz
  possible 22. Fibre Optix - Sin  best 84 hits/2 windows at +5.43% 1:22:21-1:22:39
  absent   23. Usual Suspects - Killa Beez
  absent   24. Ram Trilogy - Chase Scene
  found    25. Ed Rush & Optical - Watermelon  best 714 hits/18 windows at +3.66% 1:32:20-1:35:24
  absent   26. Ram Trilogy - System Error (Y2K)
  found    27. Ed Rush & Optical - Gas Mask  best 2010 hits/42 windows at +0.40% 1:39:06-1:46:19
  absent   28. Infinite (Optical & Fierce) - Beachball
  found    29. Ed Rush, Optical & Fierce - Alien Girl  best 1059 hits/27 windows at +5.65% 1:50:00-1:55:19
  found    30. Optical - Slip Thru  best 868 hits/20 windows at +2.41% 1:54:20-1:57:44
  found    31. Ed Rush & Optical - Sick Note  best 209 hits/6 windows at +3.81% 1:57:24-1:58:10
```

`identify` output, verbatim (trailing spaces removed). The Pulse is one play
per rip, split at the needle skip; Watching Windows includes its possible
first piece; Sin is one possible play of two segments:

```text
time                  speed  confidence   hits  asset
2:23-6:37            +0.65%  confident     762  Bad Company - The Nine.mp3
2:23-6:37            +0.65%  confident     747  extra/A- The_Nine.mp3
2:23-6:37            +0.65%  confident     762  extra/A-Bad_Company-The_Nine-dmz.mp3
13:10-18:43          +2.73%  confident     789  Ed Rush & Optical - Fixation.mp3
23:01-27:49          +4.54%  confident     729  DJ Trace - Sonar.m4a
26:30-31:32          +2.42%  confident     854  Krust - Warhead (Ram Trilogy Remix).m4a
34:50-39:29          +1.40%  confident    1206  extra/a-bad_company-pulse.mp3
  34:50-36:16        +1.24%  confident     293
  36:17-39:29        +1.40%  confident     913
34:51-39:28          +1.40%  confident    1214  Bad Company - The Pulse.mp3
  34:51-36:16        +1.24%  confident     313
  36:17-39:28        +1.40%  confident     901
39:13-42:01          +4.76%  confident     493  Ed Rush & Optical - Bacteria.mp3
47:01-52:15          +3.23%  confident    1233  Ryme Tyme - Payback Pt. 1.mp3
50:13-54:58          +5.34%  confident     536  Roni Size & Reprazent - Watching Windows (DJ Die Remix).m4a
  50:13-50:39        +5.40%  possible       78
  51:50-54:58        +5.34%  confident     458
1:02:23-1:06:08      +0.40%  confident     371  Bad Company - China Cup.mp3
1:02:23-1:06:08      +0.40%  confident     371  extra/02. Bad Company - China Cup.mp3
1:02:23-1:06:19      +0.40%  confident     368  extra/b-bad_company-china_cup.mp3
1:13:25-1:18:27      +3.07%  confident    1784  Ed Rush & Optical - Dozer.mp3
1:22:21-1:24:28      +5.43%  possible      168  Fibre Optix - Sin.m4a
  1:22:21-1:22:39    +5.43%  possible       84
  1:23:13-1:24:28    +5.37%  possible       84
1:32:20-1:35:24      +3.66%  confident     714  Ed Rush & Optical - Watermelon.mp3
1:39:06-1:46:19      +0.40%  confident    2010  Ed Rush & Optical - Gas Mask.m4a
1:50:00-1:55:19      +5.65%  confident    1059  Ed Rush, Optical & Fierce - Alien Girl.mp3
1:54:20-1:57:44      +2.41%  confident     868  Optical - Slip Thru.mp3
1:57:24-1:58:10      +3.81%  confident     209  Ed Rush & Optical - Sick Note.mp3
1:57:24-1:58:10      +3.81%  confident     209  extra/a-ed_rush_and_optical-sicknote-sour.mp3
```

Evaluations used: 2 of 5.

## Evaluation 3: the fitted matcher with both playbacks

- Code: commit da8dd4a (session 7 start), release build saved as
  `work/bin/s7-start/`. Changes since evaluation 2 that affect detections:
  both playbacks, turntable and key lock, by default (our decision
  of 2026-10-06, experiment 0018), and the fitted matcher by default (our
  decision of session 6, ADR 0008, experiment 0047: the fullest 1%
  of posting lists skipped for candidates, the link rules, each candidate
  measured again at its fitted speed, confident at 240 hits in 3 windows,
  possible at 60). Changes without effect on these detections: sessions
  3-6's memory, report, store and display work (`regress` identical after
  each), and the clusters on the ±8% ladder (the 17 corpus clusters are
  unchanged, experiment 0049). The manifest is as in evaluation 2.
- Command: `work/bin/s7-start/gunfinger-eval scan ed-rush-optical-essential-mix`
  (log `work/logs/s7-test-evaluation-3.log`; report kept as
  `work/reports/evaluation-3-scan-ed-rush-optical-essential-mix.json`).
- Why: approved in the session 7 prompt, to measure the
  default matcher on the test mix, with the prediction that Sick Note (209
  hits under the single pass) becomes possible. Nothing is changed because
  of it.
- Result: **14/16 referenced tracks identified (87.5%), 0 wrong
  identifications. Pass.** Possible tier: Sin and Sick Note found as
  possible; no possible play matches no track. The strongest detection
  matching no track has 17 hits in 4 windows (evaluation 1: 24). 1:58:11
  scanned in 62.0 s under other load (62.5 s wall, 539 s user CPU).
- As predicted, Sick Note is possible: 233 hits in 4 windows at +4.14%,
  below the rule of 240 (the single pass gave 209 hits, confident under its
  rule of 200). Sin gains (123 hits in 3 windows, 84 before) and stays
  possible.

Scan output, verbatim:

```text
ed-rush-optical-essential-mix: 14/16 referenced tracks identified, 0 wrong identifications; 1:58:11 of audio scanned in 62.0 s
possible tier: 2 more referenced tracks found as possible, 0 possible plays match no track
  absent    1. Optical & Ryme Tyme - Headhunters
  found     2. Bad Company - The Nine  best 760 hits/25 windows at +0.65% 2:24-6:26
  absent    3. Ram Trilogy - Terminal 1
  absent    4. Jonny L - The Bells
  found     5. Ed Rush & Optical - Fixation  best 737 hits/24 windows at +2.73% 14:00-17:57
  absent    6. Ram Trilogy - Mind Overload
  found     7. DJ Trace - Sonar  best 659 hits/21 windows at +4.55% 23:54-27:15
  found     8. Krust - Warhead (Ram Trilogy Remix)  best 825 hits/28 windows at +2.42% 26:52-31:26
  absent    9. Matrix - Airhead
  found    10. Bad Company - The Pulse  best 914 hits/20 windows at +1.40% 36:18-39:29
  found    11. Ed Rush & Optical - Bacteria  best 471 hits/17 windows at +4.76% 39:10-41:54
  absent   12. DJ Phantasy & DJ Probe - Orders (DJ Reality Remix)
  found    13. Ryme Tyme - Payback Pt. 1  best 855 hits/19 windows at +3.30% 47:15-50:19
  found    14. Roni Size / Reprazent - Watching Windows (DJ Die Remix)  best 462 hits/17 windows at +5.34% 51:50-54:36
  absent   15. Matrix - Asylum
  absent   16. Ed Rush & Optical - Funktion (Remix)
  found    17. Bad Company - China Cup  best 403 hits/18 windows at +0.40% 1:03:13-1:06:08
  absent   18. Ram Trilogy - Iron Lung
  absent   19. Optical - Newoptic
  found    20. Ed Rush & Optical - Dozer  best 1859 hits/29 windows at +3.07% 1:13:31-1:18:17
  absent   21. DJ Die - Clear Skyz
  possible 22. Fibre Optix - Sin  best 123 hits/3 windows at +5.44% 1:22:21-1:22:48
  absent   23. Usual Suspects - Killa Beez
  absent   24. Ram Trilogy - Chase Scene
  found    25. Ed Rush & Optical - Watermelon  best 714 hits/16 windows at +3.66% 1:32:31-1:35:08
  absent   26. Ram Trilogy - System Error (Y2K)
  found    27. Ed Rush & Optical - Gas Mask  best 1894 hits/38 windows at +0.40% 1:40:01-1:46:17
  absent   28. Infinite (Optical & Fierce) - Beachball
  found    29. Ed Rush, Optical & Fierce - Alien Girl  best 1201 hits/32 windows at +5.65% 1:49:52-1:55:04
  found    30. Optical - Slip Thru  best 803 hits/15 windows at +2.41% 1:55:25-1:57:44
  possible 31. Ed Rush & Optical - Sick Note  best 233 hits/4 windows at +4.14% 1:57:43-1:58:10
```

## Manifest change before evaluation 4 (2026-10-10)

No search was run; the evaluation count stays at 3. NAS files were found
by title in the NAS store's tags (no audio was compared), and each
candidate was judged by ear against the broadcast. On
request the manifest now references one NAS file for each match, written
`second-library/<path>` (backup:
`work/s11-prep/ed-rush-optical-essential-mix.tracklist.toml.before-2026-10-10`;
`corpus/` is not in git, so this section is its history).

- Matched, now referenced at NAS scale: 6 Mind Overload, 9 Airhead,
  15 Asylum, 18 Iron Lung, 23 Killa Beez, 24 Chase Scene, 26 System Error
  (Y2K).
- Still absent, with a related file on the NAS that was heard as
  another version: 3 Terminal 1 (the NAS has Terminal 2, a different
  track), 12 Orders (DJ Reality Remix) (the NAS's DJ Reality RMX and
  Distortion Mix share a sample with the record played but are other
  remixes; Gemini names the version played "DJ Reality Remix 3",
  unreleased, which is not checked here), 16 Funktion (Remix) (the NAS's
  Marky & Bungle remix), 21 Clear Skyz (the Break remix and the 1996 VIP),
  28 Beachball (the Black Barrel bootleg).

`gunfinger-eval validate`: with the NAS store, 31 tracks, 23 referenced
(29 reference files), 8 absent; at 262 tracks, 31 tracks, 16 referenced
(22 reference files), 15 absent, with the 7 NAS references set aside. At
262 tracks the scoring is therefore unchanged, and while `regress` stays
identical a fourth evaluation there would repeat evaluation 3. The test set
has never been scanned at NAS scale. There, tracks 12, 16, 21 and 28 test
whether a related version on the NAS produces a wrong confident detection.

Evaluations used: 3 of 5.

## Evaluation 4: the test mix at NAS scale

- Code: commit 1afd8cc, release build `work/bin/s11-code/`, the binary
  of this session's NAS baseline (experiment 0073) and recall panel
  (0074). Since evaluation 3 (da8dd4a), `regress` has been identical in
  detections after every change, among them the 16 frame and 16 asset
  bits of a posting (ADR 0010), the library ignore file, the clusters'
  verdicts and `--cut-sparsest`, and session 11's saved index (not used
  by the harness), other titles and pair review. At 262 tracks a run
  would repeat evaluation 3; at NAS scale the index also holds the NAS
  store's records.
- Store: `~/.local/share/gunfinger/nas-dnb-peaks`, revision
  `f9f9bc706195a71b` (33,596 records), read only.
- Clusters: `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`
  of session 11 (experiment 0071, 267 clusters), with the test manifest's references
  among the queries, the verdicts by ear and `--cut-sparsest`.
- Manifest: as in "Manifest change before evaluation 4". `validate` at
  NAS scale: 31 tracks, 23 referenced (29 reference files), 8 absent; at
  262 tracks: 16 referenced (22 files), 15 absent, the 7 NAS references
  set aside (`work/s11/validate-test-nas.txt`, `validate-test-262.txt`).
- Command: `work/bin/s11-code/gunfinger-eval --other-peaks-dir
  ~/.local/share/gunfinger/nas-dnb-peaks scan ed-rush-optical-essential-mix`
  (log `work/logs/s11-evaluation-4.log`; report moved to
  `work/reports/evaluation-4-scan-ed-rush-optical-essential-mix.json`, out
  of the NAS report folder, so that no later `calibrate` reads it).
- Why: the session 11 prompt. The test set has never been scanned
  at NAS scale; seven tracks are now referenced there, and five absent
  tracks have another version on the NAS. Run once; nothing changes
  because of the result, and it is not rerun.

Prediction, from evaluation 3 and the clusters alone (no audio of the test
mix read):

Basis. Evaluation 3 at 262 tracks: 14/16, Sin and Sick Note possible, 0
wrong, nothing above 17 hits matching no track (Funktion's original 14
hits in slot 16). At NAS scale the development mixes kept their tracks
and some gained hits through better copies (experiment 0067: 0 wrong,
3 possible plays matching no track on the development mix; unrelated
chance 63-70 hits). The clusters (experiment 0071) join each reference
with its copies; a confident detection of any file outside the slot's
cluster counts as wrong. In the sweeps, Evol Intent's VIP of The Nine
reached 9-24% of The Nine's own hits on the same excerpt (167-220 hits
against 795-955).

| Track | Prediction |
|---|---|
| 1, 4, 19 (absent) | nothing confident |
| 2 The Nine | identified (760 hits at 262); the VIP at 70-180 hits, perhaps possible, not confident |
| 3 Terminal 1 (absent; Terminal 2 on the NAS) | nothing confident; most likely nothing possible |
| 5, 10, 13, 14, 17, 25, 27, 29, 30 | identified, as in evaluation 3 |
| 6 Mind Overload (new) | identified; **one wrong confident detection** of Molten Beats' `111-ram_trilogy-mind_overload-sour.mp3`, an edit of the same recording that covers 79.3% of the reference (3,321 hits), just under the 80% join |
| 7 Sonar | identified; the cut revision (467 hits over the whole pair, 1.3 hits/s) far below 60 |
| 8 Warhead | identified; its pruned mixed copy no longer relates it to Side Effects |
| 9, 15, 23, 24, 26 (new) | identified (slots 3.1-4.9 min; Killa Beez's Optical & Fierce remix files share about 10%, far below the rule) |
| 11 Bacteria; 20 Dozer | identified; Space Cake and the Notjamie bootleg about 1% of a copy's hits, far below 60 |
| 12 Orders (absent) | nothing confident from the DJ Reality RMX or Distortion Mix |
| 16 Funktion (Remix) (absent) | nothing confident from the Marky & Bungle remix |
| 18 Iron Lung (new) | identified (2.9 min slot, the shortest new one) |
| 21 Clear Skyz (absent) | nothing confident from the Break remix or the 1996 VIP; a possible play would not surprise |
| 22 Sin | possible, as in evaluation 3 (a 27 s match) |
| 28 Beachball (absent) | nothing confident from the Black Barrel bootleg, the likeliest of the five to give a possible play |
| 31 Sick Note | possible (233 hits at 262, the mix ends 27 s into it); a NAS copy could take it past 240 |

In all: **21/23 referenced tracks identified, 1 wrong identification
(Mind Overload's Molten Beats edit), 2 possible (Sin, Sick Note), and a
few possible plays matching no track** (related versions and NAS-scale
chance). No wrong confident detection on tracks 3, 12, 16, 21 or 28.

Run: 06:52:11-07:03:46 on 2026-10-11, once, exit 0 (694 s wall, 5,590 s
CPU, 5.47 GB; 1:58:11 scanned in 595.2 s, under other
load). Items 5 and 6 matched experiments 0067 and 0068 (0073,
0074), so the run went ahead.

- Result: **21/23 referenced tracks identified (91.3%), 2 wrong
  identifications.** Possible tier: Sin and Sick Note found as possible;
  one possible play matches no track (The Nine's ChaseR bootleg, 81
  hits).
- Against the prediction: the 21 identified tracks, the two possible
  ones and Molten Beats' Mind Overload edit as a wrong identification
  (921 hits in 22 windows, 18:54-22:26, beside the reference's 848) came
  as predicted. Not predicted: **The Nine's Evol Intent VIP as a second
  wrong identification, 489 hits in 16 windows at 2:30-5:07**, beside
  The Nine's 868. The prediction took the sweeps' 9-24% of the
  original's hits; here the VIP reached 56%. By ear, the VIP is a
  different recording. The sweeps play 30 s excerpts; here the VIP
  matched 157 s of the original's play, so it shares the original's
  material over a longer passage than any sweep excerpt covers (an
  inference; item 8c, planned before this run, measures that passage).
  "A few" possible plays matching no track: one.
- Tracks 3, 12, 16, 21 and 28: no confident and no possible detection
  from the related versions. Their strongest detections anywhere in the
  mix: Terminal 2 20 hits (in slot 3), the Orders mixes 7, the Marky &
  Bungle Funktion 6, the Clear Skyz Break remix 6 (the 1996 VIP none),
  the Black Barrel Beachball 45 hits in 4 windows (in slot 28).
- Against evaluation 3: the 14 tracks found then are found again, with
  hits within a few percent or higher through NAS copies (The Pulse 914
  to 1,040, China Cup 403 to 436, Alien Girl 1,201 to 1,279); the seven
  NAS references are all found (Asylum the weakest, 309 hits in 16
  windows). Sin weakens to 75 hits in 2 windows (123 in 3); Sick Note
  stays at 232. Wrong identifications 0 to 2, both files the clusters
  keep apart from the reference: an edit at 79.3% coverage, just under
  the 80% join (listening pack item 2), and a VIP judged different.
- Nothing changes because of this result. If pack
  item 2 is judged `same` by ear, the Mind Overload edit joins the reference's cluster
  and the score is recomputed from the stored report, not run again.

Scan output, verbatim:

```text
ed-rush-optical-essential-mix: 21/23 referenced tracks identified, 2 wrong identifications; 1:58:11 of audio scanned in 595.2 s
possible tier: 2 more referenced tracks found as possible, 1 possible plays match no track
  absent    1. Optical & Ryme Tyme - Headhunters
  found     2. Bad Company - The Nine  best 868 hits/23 windows at +0.48% 2:23-6:07
           credited through cluster: second-library/BC_Recordings-Bad_Company-BCRUK001-2001-sour/A-Bad_Company-The_Nine-sour.mp3
           credited through cluster: second-library/__full_scene/dnb_scene - 2000 - Part 1/2000-04/va-drum_and_bass_assassins-dbascd01-2000-sour/210-bad_company-the_nine-sour.mp3
           credited through cluster: second-library/__youtube_archivists/alien5ive/BC RECORDINGS [ BCRUK 001 ： BAD COMPANY - the nine - ] drum and bass.m4a
           credited through cluster: second-library/bad_company-fear_lp/02 bad company-the nine.mp3
  absent    3. Ram Trilogy - Terminal 1
  absent    4. Jonny L - The Bells
  found     5. Ed Rush & Optical - Fixation  best 733 hits/24 windows at +2.73% 14:00-17:57
  found     6. Ram Trilogy - Mind Overload  best 848 hits/28 windows at +2.32% 18:01-22:33
  found     7. DJ Trace - Sonar  best 658 hits/21 windows at +4.55% 23:54-27:15
           credited through cluster: second-library/__full_scene/dnb_scene - 2000 - Part 2/2000-08/trace-sonar-pro012-2000-sour/a-trace-sonar-pro012-sour.mp3
  found     8. Krust - Warhead (Ram Trilogy Remix)  best 803 hits/25 windows at +2.42% 26:52-30:58
  found     9. Matrix - Airhead  best 1039 hits/25 windows at +4.77% 31:01-35:07
  found    10. Bad Company - The Pulse  best 1040 hits/20 windows at +1.31% 36:18-39:29
           credited through cluster: second-library/__youtube_archivists/alien5ive/PROTOTYPE RECORDINGS [ PRO 001 UK ： BAD COMPANY - the pulse - ] drum and bass.opus
           credited through cluster: second-library/bad_company-fear_lp/01 bad company-the pulse.mp3
  found    11. Ed Rush & Optical - Bacteria  best 484 hits/17 windows at +4.76% 39:10-41:54
           credited through cluster: second-library/__full_scene/1999/1999-08/1999-08-20/virus-ed_rush_and_optical-vrs005-1999-sour/aa-ed_rush_and_optical-bacteria-sour.mp3
           credited through cluster: second-library/__youtube_archivists/alien5ive/VIRUS RECORDINS [ VRS 005 ： ED RUSH + OPTICAL - bacteria - ] drum and bass.m4a
  absent   12. DJ Phantasy & DJ Probe - Orders (DJ Reality Remix)
  found    13. Ryme Tyme - Payback Pt. 1  best 855 hits/19 windows at +3.30% 47:15-50:19
           credited through cluster: second-library/__full_scene/1999/1999-11/1999-11-13/dsci4-ryme_tyme-dsci402-1999-sour/a-ryme_tyme-payback-sour.mp3
  found    14. Roni Size / Reprazent - Watching Windows (DJ Die Remix)  best 479 hits/19 windows at +5.34% 51:50-54:55
  found    15. Matrix - Asylum  best 309 hits/16 windows at +6.39% 55:01-57:38
  absent   16. Ed Rush & Optical - Funktion (Remix)
  found    17. Bad Company - China Cup  best 436 hits/20 windows at +0.28% 1:02:51-1:06:07
           credited through cluster: second-library/__youtube_archivists/alien5ive/PROTOTYPE RECORDINGS [ PRO 001 UK ： BAD COMPANY - china cup - ] drum and bass.opus
           credited through cluster: second-library/bad_company-fear_lp/07 bad company-china cup.mp3
  found    18. Ram Trilogy - Iron Lung  best 572 hits/18 windows at +4.03% 1:06:07-1:09:08
           credited through cluster: second-library/__full_scene/1999/1999-11/1999-11-21/ram_trilogy-molten_beats-1999-sour/108-ram_trilogy-iron_lung-sour.mp3
  absent   19. Optical - Newoptic
  found    20. Ed Rush & Optical - Dozer  best 1814 hits/30 windows at +3.07% 1:13:24-1:18:16
  absent   21. DJ Die - Clear Skyz
  possible 22. Fibre Optix - Sin  best 75 hits/2 windows at +5.54% 1:22:31-1:22:41
  found    23. Usual Suspects - Killa Beez  best 1794 hits/31 windows at +3.03% 1:24:36-1:29:37
  found    24. Ram Trilogy - Chase Scene  best 670 hits/22 windows at +3.51% 1:29:20-1:32:51
  found    25. Ed Rush & Optical - Watermelon  best 714 hits/16 windows at +3.66% 1:32:31-1:35:08
  found    26. Ram Trilogy - System Error (Y2K)  best 1454 hits/31 windows at +2.28% 1:35:01-1:40:11
           credited through cluster: second-library/__full_scene/1999/1999-11/1999-11-21/ram_trilogy-molten_beats-1999-sour/112-ram_trilogy-system_error_(y2k)-sour.mp3
  found    27. Ed Rush & Optical - Gas Mask  best 1921 hits/38 windows at +3.62% 1:39:51-1:46:07
           credited through cluster: second-library/__full_scene/1999/1999-08/1999-08-20/virus-ed_rush_and_optical-vrs005-1999-sour/a-ed_rush_and_optical-gas_mask-sour.mp3
  absent   28. Infinite (Optical & Fierce) - Beachball
  found    29. Ed Rush, Optical & Fierce - Alien Girl  best 1279 hits/25 windows at +5.44% 1:49:51-1:53:57
           credited through cluster: second-library/__full_scene/dnb_scene - 2000 - Part 2/2000-08/prototype-ed_rush_and_optical_and_fierce-pro014-1998-sour/aa-ed_rush_and_optical_and_fierce-alien_girl-sour.mp3
           credited through cluster: second-library/__youtube_archivists/i-witness-dnb/Ed Rush, Optical & Fierce - Alien Girl.m4a
  found    30. Optical - Slip Thru  best 838 hits/15 windows at +2.41% 1:55:25-1:57:44
           credited through cluster: second-library/__youtube_archivists/dfect-dnb/Optical - Slip Thru.m4a
  possible 31. Ed Rush & Optical - Sick Note  best 232 hits/4 windows at +4.13% 1:57:43-1:58:10
  WRONG  second-library/__full_scene/1999/1999-11/1999-11-21/ram_trilogy-molten_beats-1999-sour/111-ram_trilogy-mind_overload-sour.mp3 18:54-22:26 +2.23% 921 hits/22 windows
  WRONG  second-library/__youtube_archivists/i-witness-dnb/Bad Company - The Nine (Evol Intent VIP).m4a 2:30-5:07 +0.22% 489 hits/16 windows
  POSSIBLE, NO TRACK  second-library/__youtube_archivists/i-witness-dnb/Bad Company - The Nine (ChaseR Bootleg).opus 2:52-3:16 81 hits in 1 segments
```

Evaluations used: 4 of 5.
