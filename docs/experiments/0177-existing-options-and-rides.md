# 0177: what the existing options recover of a ridden play

**Question.** Session 37, step 3: do the harness's opt-in matching
options keep the side that 0175's rule drops, and at what cost?

**Method.** Two of 0176's conditions on seed 3701 (16 mixes, 194 ridden
indexed plays, renders reused): 1% over 10-20 s and 3% over 2-5 s. Each
searched by default, with `--speed-per-stretch`, `--trim-ends`,
`--single-pass`, `--extra-rungs 3`, and with step 4's variant
`--overlap-by-chains` (development data; its criteria were committed
first). User CPU from `/usr/bin/time` (decoding and searching 16 mixes,
one thread each, 262 tracks). `ride_dose.py`; a side counts with 240
hits in 3 windows, also for `--single-pass` (its rule is 200). Data
`data/0177-existing-options-and-rides.json`.

| 1% over 10-20 s; 3% over 2-5 s | No detection | 30 s+ uncovered | Own time uncovered | CPU |
|---|--:|--:|--:|--:|
| Default | 42; 91 | 70; 101 | 4,816; 7,239 s | 2,225; 2,223 s |
| `--speed-per-stretch` | 38; 92 | 75; 102 | 4,997; 7,256 s | +0.2%; 0% |
| `--trim-ends` | 84; 28 | 104; 30 | 8,658; 2,204 s | -0.4%; -0.3% |
| `--single-pass` | 0; 4 | 0; 8 | 7; 652 s | +10.8%; +10.8% |
| `--extra-rungs 3` | 42; 91 | 70; 101 | 4,816; 7,239 s | +15.0%; +15.1% |
| `--overlap-by-chains` | 0; 5 | 10; 6 | 742; 365 s | -0.4%; -0.3% |

- Of 326 and 347 sides of 30 s or more. No wrong confident detection
  and no ridden play without a confident one in any row.
- `--speed-per-stretch` refits stretches inside each side; it does not
  change the second pass's margin or the suppression. `--extra-rungs`
  adds rungs past ±8%, where no play here is. `--trim-ends` moves the
  facing ends back where the windows there are weak: it helps the short
  ramp and hurts the long one (not traced why).
- `--single-pass` has no second pass and no margin, so the two chains
  never overlap; it is the matcher ADR 0008 replaced (3.3 times the wall
  time at NAS scale, many more false candidates, 200 hits on the ladder).

**Conclusion.** Of the existing options only `--single-pass` keeps both
sides, and it reverts ADR 0008. The variant keeps them at no measurable
CPU (0.3-0.4% less, within noise). It leaves 10 and 6 sides; 0176's
trace found 10 and 7 lost at another stage than the second suppression.
