# Session 18 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-18.md`
for scope and rules.

## Setup

- [x] `git status` clean at `300e008`
- [x] Brief, checklist; `CLAUDE.md` points to them
- [x] Release build in `work/bin/s18-start/` (byte-identical to
  `s17-final`); `baseline session-18-start` from session 17's final
  `regress` reports, made by the same code
- [x] The NAS store at the start: 33,723 records, revision
  `c835c5531c325274`, as expected (`work/s18/store-revision-start.txt`)
- [x] Digests of `docs/pair-verdicts.txt` and `work/audition/answers.jsonl`
  in `work/s18/listening-digests-start.txt`

## Steps

- [x] 2a. `clusters --from-peaks` with the three manifests and recall
  panel 2026 (1,554 s): 268 -> 271 clusters; `pair` on 40 pairs: the
  TI029 C rips are neither the VIP nor the 1999 original; SUICIDE004's
  "We R The Ones 3" is the VIP; alien5ive's "FUZE 29 cannibal" is the
  "Sonic - Change" uploads' recording
- [x] 2b. 262 tracks 9/9, 0 wrong; NAS 17/17, 0 wrong, 4 possible plays
  (all shared material); leave-outs (seeds 2026-2028) 14/17, 0 wrong;
  `identify` (turntable and both) 21 entries; track 17's slot empty
- [x] 2c. The fire-and-forget view: every track has a confident entry
  with a right file, 16 of 17 named right; no confident entry shows only
  a wrong title
- [x] 2d. Experiment 0100
- [ ] 3. The containment rule on the new reports and every stored
  report; recommendation; experiment
- [ ] 4. The possible tier: the listening answers in the roadmap and
  `docs/calibration.md`; possible plays per hour at 60-130; true plays
  only possible
- [ ] 5. Listening questions in `work/audition/s18/`; the page checked
  with temporary files
- [ ] 6. Search speed (if time remains)
- [ ] 7. Read-only review by a subagent; fixes

## Wrap-up

- [ ] Notes "Session 18", status, roadmap ("More development mixes", the
  possible tier), ADR 0004's "Later"
- [ ] `regress session-18-start` identical with the final build;
  `scripts/check.sh` green
- [ ] The NAS store's revision at the end
- [ ] `git diff docs/pair-verdicts.txt` empty and
  `work/audition/answers.jsonl` unchanged before every commit
- [ ] Commit; the final message
