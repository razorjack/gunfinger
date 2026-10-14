# Session 19 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-19.md`
for scope and rules.

## Setup

- [x] `git status` clean at `d867557`
- [x] Brief, checklist; `CLAUDE.md` points to them
- [x] Release build in `work/bin/s19-start/`; `baseline session-19-start`
- [x] The NAS store at the start: 38,105 records, `b285da9984422fb5`, as
  expected; rustc 1.98.0, RealFFT 3.5.0, RustFFT 6.4.1
- [x] `doctor`: the CLI's saved index current for the store (the 2003 mix
  at NAS scale: 50.0 s, 412 s user, 4.82 GB, the index loaded)

## Steps

- [x] 2a. Integer phases per query point in `OffsetSlots`, guarded, with
  today's f64 filter outside the guard
- [x] 2b. 10 tests, 7 against `densest_clusters` (three fail when odd
  anchors' phases shift by 3 frames)
- [x] 2c. Counts: the same lines, +0.017% assets sorted; split profiles A B
  B A, contended and quieter: the stage about 13% cheaper against the
  analysis, below the 20% stop point
- [-] 2d. Not run: stopped at 2c. Code deleted (kept in
  `work/s19/lines.rs.p1`); experiment 0108
- [x] 3a. The lengths of four arms with factorisations, plus the fastest
  measured within 1 and 2 samples (`work/s19/fft-lengths.txt`)
- [x] 3b. Timing on 60 s of the 2003 mix, one thread; instrumented
  weights; 1,862 plans in 0.052 s per `identify`
- [x] 3c. Weighted first-pass turntable analysis: nearest-even -21.8%,
  factor-aware within 2 samples -36.5% (-30.8% with key-locked rungs);
  timing-only builds -5.4% to -11.1% wall; retrieval study described in
  the notes
- [-] 3d. Plan cache not tried: plans are 0.04% of the analysis
- [x] 3e. Experiment 0109
- [x] 4. Read-only review by a subagent: no output change; fixed the
  deleted code's rounding test (kept copy), 0108's framing and numbers,
  0109's load range and per-frame wording, the roadmap's contended label

## Wrap-up

- [x] Notes "Session 19", status, roadmap ("Where search time goes"), the
  research note (proposals 1 and 2 measured)
- [x] `regress session-19-start` identical with the final build;
  `scripts/check.sh` green (357 tests)
- [x] The NAS store's revision at the end: 38,105 records,
  `b285da9984422fb5`, unchanged
- [x] `git diff docs/pair-verdicts.txt` empty and
  `work/audition/answers.jsonl` unchanged before every commit
- [x] Commit; the final message
