# 0018: key lock on by default

**Question.** The owner made both playbacks the default on 2026-10-06.
What does that cost in CPU time, and does the new default reproduce the
both-ladder reports of experiment 0016?

**Command.** `/usr/bin/time -l gunfinger identify <development mix>
--playback turntable|both`, plus two interleaved rounds of wall time. On
the new default, `gunfinger-eval regress key-lock-default` against the
both-ladder reports of 0016, then `calibrate`. Data:
`data/0018-key-lock-default.json`.

**Results.**

| Development mix, 56:09 | Turntable | Both |
|------------------------|-----------|------|
| CPU (user) time | 166.2 s | 260.6 s (1.57×) |
| Wall time, quiet round | 23.8 s | 34.7 s (1.46×) |
| Peak resident memory | 387 MB | 461 MB |
| Plays (confident and possible) | 14 | 14, none with key lock |

Three plays change. Side Effects starts at 21:30 instead of 21:07 (2,241
hits over 29 windows against 2,237 over 31); Bios-Fear gains 3 hits and a
window; the Clockwork remix's possible play gains 3 hits (94) and a window.

The new default reproduces experiment 0016 bit for bit: 720 of 720 sweep
queries and every detection of the development scan and both leave-outs
identical (6,114, 6,037, 5,736); `calibrate` gives weakest identifying
501, strongest false 97 (margin 5.16×), every false candidate of 30 hits
or more the Clockwork remix, audio not in the index 19.

The 1.3 times of experiment 0016 came from wall times on a machine with a
load average above 100; CPU time depends much less on other load. 82 rungs
cost less than twice 41 because decoding and the index build happen once
per run; how the rest divides was not profiled.

**Conclusions.** Both playbacks are the default in `identify`, `explain`
and the harness. Vinyl-only users set `playback = "turntable"` in the
configuration file (README, "Key lock and vinyl"); the owner's file does.
Turntable-only harness reports moved to `work/reports/ladder-turntable/`.
