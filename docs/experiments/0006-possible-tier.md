# 0006: calibrating a possible tier

**Hypothesis.** Unrelated recordings produce far fewer aligned hits than a
real but brief play or a remix of the playing record. A threshold in that
gap can show useful evidence below the confident rule without showing
unrelated recordings.

**Change.** Plays (segments of one asset with gaps up to 90 s) and
`Confidence::Possible` at `MIN_POSSIBLE_HITS = 60` (ADR 0006). Search and the
confident rule unchanged. Calibration data only: the sweep (seed 2026), the
development scan and its leave-outs; test-set reports are not read.

**Command.** `gunfinger-eval scan stakka-skynet-knowledge [--leave-out 3|11
--seed 2026]`, `gunfinger-eval calibrate`; logs `work/logs/plays-*.log`.

**Results.**

- Audio not in the index (180 held-out sweep queries, 2,260 detections):
  strongest 19 hits.
- False candidates at 30 hits or more: 18, all the Stakka remix of
  Clockwork against the original (75 to 95 hits in the sweep; 91, 45 and 35
  in each development scan). Strongest unrelated: 28 hits (sweep, one
  Aphrodite track against another).
- Real sub-threshold pieces of played tracks in the development mix: at most
  8 hits. Every track there plays in full; there are no brief plays.
- Threshold 60: 2.14× the strongest unrelated false candidate, 3.16× the
  strongest detection on audio not in the index.
- Development scan: 11/11, 0 wrong, detections identical to before; 0 tracks
  found only as possible; 1 possible play matching no track, the Stakka
  remix of Clockwork (20:22 to 20:39, 91 hits) while the original plays.
  Same in both leave-outs, whose speeds differ from older reports by at
  most 4.4e-16 (rewritten speed fit, experiment 0007).
- `identify` on the development mix: 19 plays, the remix the only possible
  one.

**Conclusion.** On the calibration data the tier shows no unrelated
recording, only a remix of the playing record. Its benefit cannot be
measured on the development set; the test set is the check (ledger,
evaluation 2).
