# Test-set ledger: `ed-rush-optical-essential-mix`

Every evaluation of the held-out test set, in order. At most five in total.

## Evaluation 1: first run at `poc-freeze-1`

- Code: tag `poc-freeze-1` (commit 407cc93). Nothing changed since the
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

## Owner review of evaluation 1 (2026-10-06)

The owner listened to the items listed for checking. No search was run; the
evaluation count stays at 1.

- **16. Funktion.** The version played is "Funktion (Remix)" from Planet V
  (V Recordings, 1999), not the original from the Funktion / Naked Lunch
  single (V026, 1998) that the library holds. Sources: the owner's listening,
  and MixesDB and Dogs On Acid tracklists found in the owner's research (not
  checked here). The remix is not in the library, so the slot is in effect
  absent, and the outcome is correct: the original stayed far below the rule
  (best 18 hits) while its remix played. The owner has since changed the
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

- Code: commit e87b79f. Changes since evaluation 1: the elegance pass (no
  change in results), the owner's correction of track 16 in the manifest
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

- Code: commit da23e9d (session 7 start), release build saved as
  `work/bin/s7-start/`. Changes since evaluation 2 that affect detections:
  both playbacks, turntable and key lock, by default (the owner's decision
  of 2026-10-06, experiment 0018), and the fitted matcher by default (the
  owner's decision of session 6, ADR 0008, experiment 0047: the fullest 1%
  of posting lists skipped for candidates, the link rules, each candidate
  measured again at its fitted speed, confident at 240 hits in 3 windows,
  possible at 60). Changes without effect on these detections: sessions
  3-6's memory, report, store and display work (`regress` identical after
  each), and the clusters on the ±8% ladder (the 17 corpus clusters are
  unchanged, experiment 0049). The manifest is as in evaluation 2.
- Command: `work/bin/s7-start/gunfinger-eval scan ed-rush-optical-essential-mix`
  (log `work/logs/s7-test-evaluation-3.log`; report kept as
  `work/reports/evaluation-3-scan-ed-rush-optical-essential-mix.json`).
- Why: approved by the owner in the session 7 prompt, to measure the
  default matcher on the test mix, with the prediction that Sick Note (209
  hits under the single pass) becomes possible. Nothing is changed because
  of it.
- Result: **14/16 referenced tracks identified (87.5%), 0 wrong
  identifications. Pass.** Possible tier: Sin and Sick Note found as
  possible; no possible play matches no track. The strongest detection
  matching no track has 17 hits in 4 windows (evaluation 1: 24). 1:58:11
  scanned in 62.0 s on a machine in use (62.5 s wall, 539 s user CPU).
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

Evaluations used: 3 of 5.
