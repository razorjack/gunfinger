# 0266: the counterexample budget again, the target's level as registered

**Question** (brief 68, 04:50). 0264 took the target's level from the scorer's strongest
detection by hits; registered was "no confident matching detection". What changes?
**Command.** `work/s68/{rerun,replays-b}.sh` (`s68-rerun`, `510fb6c`): the same seed and budgets. Data: `data/0266-counterexample-classes-rerun.json`.

| Class (failure: minimal transformations) | 262, 2,000 | NAS, 500 |
|---|--:|--:|
| gap: channel-fault | 85 (68 families) | 23 |
| gap: ride+channel-fault; ride+blend+channel-fault | 20; 20 | 6; 6 |
| gap: ride+blend; blend+channel-fault; blend | 15; 11; 1 | 3; 5; 0 |
| gap: other compounds with a fault (6 and 2 classes) | 7 | 2 |
| lost: blend; blend+channel-fault | 9; 11 | 4; 6 |
| lost: ride+blend; ride+blend+channel-fault; ride+channel-fault | 4; 4; 1 | 1; 0; 0 |
| wrong: none (3 families, as 0264) | 0 | 16 |
| Failures (17 and 11 classes) | 188 | 72 |

- Against 0264: at 262 tracks one sample is no longer a failure (#92, confident, 19.6 s
  uncovered), one `lost: ride+blend` recipe minimizes otherwise, the other 187 and all
  72 at NAS scale are unchanged; the 12 replayed recipes are 0265's and hold again.
- Minimal sizes at 262: 95 one transformation, 63 two, 28 three, 2 four. Rides in
  `gap: ride+blend` 0.58-2.93%, in `lost: ride+blend` 0.54-2.86%; 200 rides alone or
  with speed or a kill leave at most 15.6 s uncovered (ADR 0021 keeps both sides).
- The bounds are harsher than the generator's crossfade (the review): the blend has no
  fades and may cover the whole target, where `mixes::crossfade` fades both over at most
  half the shorter play; the kill lasts up to 60 s, a bass swap at most 30 s. All 28 lost
  failures with a blend overlap more than half the target (double drops, 0184's
  construction), as do 21 of 49 gaps with one; 5 minimal recipes hold a kill (7-58 s).
- The fault starts at the mix's middle, so in the target's second half: `gap:
  channel-fault` needs a target of 60.4 s or more; inverted targets fail 62 of 76 at
  60 s or more, 2 of 25 below. The minimizer drops in the draw's order, so where
  two minimal sets exist the later axes (the fault) stay.
- Cost: 262, 16:30, 29,475 CPU s (3,156 renders), 5.3 s and 157 CPU s per failure, 58 s
  per class; NAS, 17:10, 27,977 CPU s, 14.3 s and 389 CPU s per failure, 94 s per class.

**Conclusion.** 0264's classes stand (one failure fewer). Every lost or gap class needs a
channel fault or a blend (alone or with up to three more), the wrong answers none; no
class is outside the roadmap's entries alone, and the ride compounds (a ride's later side
under a blend or a fault) are in none as a pair. Criterion (b) holds; (a) only if a pair counts.
