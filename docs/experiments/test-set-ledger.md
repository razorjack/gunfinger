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
