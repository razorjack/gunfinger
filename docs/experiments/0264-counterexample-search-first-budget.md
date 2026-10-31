# 0264: automated counterexample search, the first budget

**Question** (brief 68, registered in `7440c8e`). Within the generator's bounds (speed
±6%, a 1-3% ride, a bass kill and a blend of 10-60 s, brief 42's channel faults), which
minimal transformations lose an indexed target of 40-120 s, leave 30 s or more of a
found one uncovered, or make a confident wrong answer?
**Command.** `work/s68/{queue,nas}.sh`: `counterexamples --seed 2026 --samples 2000`;
with `--other-peaks-dir`, 500 samples (the same 500 bases). `scripts/analysis/counterexample_classes.py`;
data `data/0264-counterexample-classes.json`.

| Class (failure: minimal transformations) | 262, 2,000 | 262, first 500 | NAS, 500 |
|---|--:|--:|--:|
| gap: channel-fault | 85 (68 families) | 28 | 23 |
| gap: ride+channel-fault; ride+blend+channel-fault | 20; 20 | 5; 4 | 6; 6 |
| gap: ride+blend; blend+channel-fault; blend | 15; 11; 1 | 4; 3; 0 | 3; 5; 0 |
| gap: other compounds with a fault (6 and 2 classes) | 7 | 0 | 2 |
| lost: blend; blend+channel-fault | 9; 11 | 5; 5 | 4; 6 |
| lost: ride+blend; ride+blend+channel-fault; ride+channel-fault | 4; 4; 2 | 1; 0; 0 | 1; 0; 0 |
| wrong: none (the base alone) | 0 | 0 | 16 (3 families) |
| Failures (one per failing sample) | 189 | 55 | 72 |

- Alone (of 1,504 indexed targets), speed (42), a bass kill (55) and the base (61) leave
  at most 1.3 s uncovered, a ride (54) 15.2 s (200 rides without a blend or fault: none
  20 s or more); a blend (53) loses one. No minimal recipe is a speed or bass kill alone.
- Faults in minimal recipes: inverted 64 of 101 sampled, noise at -10 dB 57 of 93, each other one 4-9 of 74-101.
- `lost: blend`: targets of 40-48 s under a partner for the whole play
  (39-59 s, -3.6 to -0.1 dB), possible (8) or weak (1). `gap:
  ride+blend`: from the ride to the end, under a 30-60 s blend, 30-51 s
  uncovered; rides halved to 0.6-1.0% still fail.
- Wrong at NAS scale, all without a transformation: Bad Company - The
  Nine (Evol Intent VIP) over The Nine (13; 240-791 hits; the original
  held out in 6), Cool Rock Shook Ones over an Aphrodite file (3), Pyro -
  Ignorance over Pyro - Time Is Broken (1).
- Scales: 54 of the first 500's 55 lost or gap failures recur at NAS scale (2 new
  there), 43 minimal recipes identical; median 6 and 5 renders to minimize.
- Cost: 262, 16:38 and 29,760 CPU s (3,165 renders), 5.3 s and 157 CPU s per failure;
  NAS, 17:12 and 27,983 CPU s (860 renders), 14.3 s and 389 CPU s per failure, 9.6 GiB.

**Conclusion.** Every class needs a channel fault or a blend, alone or with one or two
more; speed, bass kills and rides alone keep the target. The wrong answers are related versions the base alone shows.
