# 0010: key-locked rungs find key-locked playback

**Context.** Under key lock (tempo changes, pitch does not, as with a
CDJ's master tempo) the turntable ladder finds nothing confident from ±2%
on (experiment 0009).

**Hypothesis.** A rung that rescales time only (`points_at_tempo`: the STFT
hop shrinks by the tempo, window and bins stay) meets key-locked audio as a
turntable rung meets resampled audio, without new false matches.

**Change.** `speed::Rung::{Turntable, KeyLocked}`, `key_lock_ladder()` (the
same 41 tempos); detections record which kind of rung found most of their
hits. `gunfinger-eval --ladder turntable|key-lock|both`.

**Command.** `robust --ladder key-lock --only <key-lock conditions>`, then
`robust --ladder both` on all 32 conditions (seed 2026). Data:
`data/0010-robust-*-summary.json`.

**Results** (of 40 indexed excerpts; held-out: strongest false candidate).

| Condition | Turntable | Key-lock | Both: confident; median hits; held-out max |
|-----------|----------:|---------:|--------------------------------------------|
| control (0.95, 1.03) | 40, 40 | 0, 0 | 40, 40; 835, 877; 24, 17 |
| key lock -2%, +2% | 0, 0 | 40, 40 | 40, 40; 893, 973; 13, 16 |
| key lock -5%, +5% | 0, 0 | 40, 40 | 40, 40; 656, 791; 12, 10 |
| key lock +8% | 0 | 40 | 40; 731; 10 |

With both ladders every turntable condition of 0009 keeps its confident
and possible counts and its strongest wrong candidate per row. Own-track
hits are unchanged in 2,394 of 2,400 queries (6 off-ladder ones gain 6-8).
The strongest wrong candidate of single queries rose by 1-9 hits in 709
queries (more hypotheses, more chance), but no row maximum moved and the
held-out maximum stays 26. No wrong confident detection. Median tempo
error under key lock is at most 0.007% per row.

**Conclusion.** Key lock is solved at twice the search time (82 rungs
instead of 41; to be timed on an idle machine). The default stays the
turntable ladder (the brief is vinyl); `identify --playback both` (or
`key-lock`) is offered, its plays marked `(key lock)`. Making `both` the
default needs the full protocol with `--ladder both` and calibrate.
