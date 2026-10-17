# Session 28 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-28.md`
for scope and rules.

## Setup

- [x] `git status` clean at `c0b3172`
- [x] The NAS store at the start: 40,126 records, `fed26c76b8fe89ea`;
  listening files recorded (`work/s28/listening-digests-start.txt`)
- [x] Brief, checklist; `CLAUDE.md` points to them; committed (`1861afe`)
- [x] Release build in `work/bin/s28-start/` (byte-identical to
  `s27-final`); `baseline session-28-start` (copies reports, 0.23
  s)

## Steps

- [x] 2. The held-arrow bug: cause confirmed, every path that starts or
  reschedules a source, the invariant; minimal fix; a `node --test` test
  that fails before and passes after; `scripts/check.sh` runs it when
  Node 22.7 or newer is installed; the real page in headless Chrome (own server, /tmp
  copies, deleted); committed
- [x] 3. Visual refresh: screenshots before; CSS properties, type scale,
  contrast (WCAG AA), queue states, focus, lanes; screenshots after
  (`work/s28/screens/`); the step 2 test again; committed
- [x] 4a. Pre-registration (brief, experiment 0143); the clusters checked
  against the six verdicts (all six agree, no rebuild); committed before
  any count
- [x] 4b. Harness changes with unit tests; committed before the runs
  (`0c0905d`)
- [x] 4c. The runs at NAS scale under `screen`; experiment files;
  committed (single conditions 0144, combinations 0145: the three pairs
  and all three together; 03:57-05:11)
- [x] 5. Session 24b's merge when ready: ready at 05:40 (the
  orchestrating thread's note; checked: tree clean at `bad25f9`, wrap-up
  commits, no Panako process); diff since the last merge only docs;
  merged at `31ba92a` (one conflict in the roadmap, resolved as noted)

## Wrap-up

- [ ] 6. Read-only review by a subagent (by 09:30); fixes committed
- [~] Notes "Session 28" with the handoff, status, checklist, roadmap
  (written at 05:30 before the review; 24b's merge and the checks to add)
- [ ] `regress session-28-start` identical with the final build
- [ ] `scripts/check.sh` green with rustup's 1.98.0 first on PATH
  (`RUSTUP_TOOLCHAIN=1.98.0`)
- [ ] `target/release/gunfinger-eval` built from the final commit
- [ ] The NAS store's revision at the end
- [ ] `docs/pair-verdicts.txt` and `work/audition/answers.jsonl` unchanged
- [ ] Commit; the final message
