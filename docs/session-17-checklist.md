# Session 17 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-17.md`
for scope and rules.

## Setup

- [x] `git status` clean at `b7ddfb6`
- [x] Brief, checklist; `CLAUDE.md` points to them
- [x] Left alone every process this session did not start
- [x] Release build in `work/bin/s17-start/` (byte-identical to
  `s16-final`); `baseline session-17-start` from session 16's final
  `regress` reports, made by the same code
- [x] The NAS store at the start: 33,596 records, revision
  `f9f9bc706195a71b`, as expected (`work/s17/store-revision-start.txt`)

## Steps

- [x] 2. Search speed (experiments 0096-0098): grouping a window's hits a
  part of 4,096 assets at a time, taken from the posting lists (0096);
  peak picking without unpredictable branches (0097); record headers read
  on `jobs` threads and the mix decoded while the index loads (0098).
  The 2003 mix at NAS scale: 44.1-44.3 s, 377-382 s CPU, 4.33 GB against
  61.0-62.1 s, 482-487 s, 4.79 GB (A B B A, reports identical);
  `regress session-17-start` identical, 151.7 s against 200.8 s;
  `scripts/check.sh` green (346 tests). Rejected: two-pass grouping,
  run maxima, counting while taking, parts of 1,024 and 8,192
- [x] 3. Indexing speed (experiment 0099): first runs into scratch stores
  (corpus library 29.58 h; NAS folder `__youtube_archivists/error-bat`
  18.93 h, no 0065 candidate), A B B A: 4% and 3.5% faster with 0097's
  picker, every record and tag note identical. FFmpeg is 86% of the
  CPU (its options fixed); Gunfinger's own CPU 67 s -> 35 s on the
  corpus. No further change
- [x] 4a. Listening questions for Watermelon ~ 300 Baud - Monotone and
  Falcon - Fractles ~ Ryme Tyme - We Enter (shared material / chance /
  unsure): a new producer, `questions shared`, wrote
  `work/audition/shared/s17-shared.json`, outside the default queue; the
  page loaded both with a temporary verdicts file and answer log, nothing
  answered
- [x] 4b. The roadmap's "Behaviour at scale" and "The possible tier at
  NAS scale" and `docs/calibration.md` corrected: the strongest
  "unrelated" matches checked since session 10 were shared material, the
  strongest not checked 41 hits; no threshold changed
- [x] 5. Read-only review of the session's code by a subagent; fixes.
  Round 1 (`b7ddfb6..054e0e4`): no output change; fixed the grouping
  test (one part only), the saved-index loader (lists not checked for
  asset order; now refused at no measurable cost) and four comments;
  `scripts/check.sh` green (349 tests). Round 2 (`4471b5f`, `612b4d9`):
  no defect; three robustness gaps of `questions shared` and doc points
  fixed

## Wrap-up

- [ ] Notes "Session 17", status, roadmap
- [ ] `regress session-17-start` identical with the final build;
  `scripts/check.sh` green
- [ ] The NAS store's revision at the end
- [ ] `git diff docs/pair-verdicts.txt` empty and
  `work/audition/answers.jsonl` unchanged before every commit
- [ ] Commit; the final message
