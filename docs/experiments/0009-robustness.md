# 0009: robustness of candidate A on transformed excerpts

**Question.** What survives what clubs, DJs and broadcasts do to a record,
and does any transform create false identifications?

**Setup.** `gunfinger-eval robust --seed 2026` (harness of commit bc2412b,
frozen rule): the sweep's index, its first 40 indexed and 10 held-out 30 s
excerpts, at 0.95 and 1.03 unless the condition sets the speed; 32
conditions (`crates/gunfinger-eval/src/robust.rs`). Data:
`data/0009-robust-seed-2026-summary.json`.

**Results** (of 40 per speed; retention: median best hits against control).

| Condition | Confident | Possible+ | Retention | Misses by cause |
|-----------|-----------|-----------|----------:|-----------------|
| control, ±12 dB EQ, low-pass 2 kHz, telephone, MP3 32-64k, AAC 48k, Opus 24k, voice-over, 8 s speech insert, 2% pitch ride | 40 | 40 | 41-113% | none |
| low-pass 800 Hz, high-pass 1 kHz | 40 | 40 | 41-74% | none |
| echo; pink noise at 10 dB SNR | 37-39 | 40 | 36-41% | hits < 200 |
| clipping (+12 dB); blend at -6 dB | 32-36 | 40 | 31-41% | hits < 200 |
| blend at 0 dB | 13-14 (partner 11) | 32-33 | 13-15% | hits < 200 |
| pink noise at 0 dB SNR | 0 | 14 | 6% | hits < 200 |
| needle skip 2 s mid-excerpt | 0-1 | 40 | 51-53% | windows < 3 |
| speed ±10, ±12, ±16% (off the ladder) | 0 | 0 | 0% | no line |
| key lock ±2% (tempo only) | 0 | 35 | 13-15% | hits and windows |
| key lock ±5%, +8% | 0 | 0 | 2% | no chain |

**False matches.** No wrong confident detection under any condition. The
strongest false candidate on held-out excerpts had 26 hits (possible needs
60); on indexed excerpts, 33 (blend at 0 dB). Every wrong possible (one per
row) came from the `Skynet & Stakka - Clockwork` excerpt matching its Stakka
remix, which shares material: the case the possible tier is meant to show.

**Conclusions.** (1) The null holds under every transform: no threshold
change is needed for broadcast or club damage. (2) Failures are lost
evidence, never false evidence; a 30 s excerpt with 30-40% retention falls
under 200 hits, a multi-minute play would not. (3) Key lock defeats the
turntable ladder from about 1%: frequencies stay put while time stretches,
so no rung fits; a tempo-only ladder is the next experiment. (4) Nothing is
found beyond ±8%, as designed. (5) Needle-skip misses come from 30 s
excerpts (halves of 2 windows); the test mix's real skip was found.
