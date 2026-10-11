# 0072: Pre-release correctness regression

**Hypothesis.** Fixing report, cache and store lifecycle defects does not
change detections from unchanged library records. Play confidence must
match the maximum confidence of its independently qualifying segments.

**Setup.** Isolated worktree `fix/prerelease-audit` from `8038064`.
Copied the corpus peak store, seed-2026 render cache, duplicate clusters and
four standard reports into this worktree's `work/`. Original corpus read
only; no held-out set evaluated. Independent release binaries and outputs.

**Commands.** `cargo build --release --workspace -j 2`; preserve binaries
in `work/bin/`; `work/bin/baseline-eval baseline prerelease-start`;
`work/bin/baseline-eval --corpus ../gunfinger/corpus --jobs 4 regress
prerelease-start > work/logs/regress-base.log`.

**Baseline result.** 720/720 sweep queries identical; development and
leave-out scans have 87, 93 and 79 identical detections. Sweep recall 100%
at every speed, zero wrong; development 11/11, zero wrong. Calibration
unchanged (weakest identifying 680, strongest false 99).

**Final command.** Save verified reports with `baseline before-core-fixes`,
then `/usr/bin/time -p work/bin/final-eval --corpus ../gunfinger/corpus
--jobs 4 regress before-core-fixes` (release binary), log
`work/logs/regress-final.log`. Includes the core fixes and five optimizations.
The superseded candidate run was stopped after rendering, to include a
legacy-store migration fix in the final regression. Sample renders remained
byte-identical. The final run completed in 987.43 s wall time.

**Final result.** All 720 sweep queries and all 87 + 93 + 79 scan detections
identical. Recall, wrong answers and calibration unchanged from the baseline.
`CARGO_BUILD_JOBS=2 scripts/check.sh`: formatting, Clippy with warnings denied
and 255 tests pass. Original audit CLI reproductions pass, including offline
alias titles. No held-out evaluation; only this worktree's outputs changed.

**Conclusion.** Keep the fixes and measured optimizations. Unit tests cover
the corrected confidence selection; existing segment detections are unchanged.
