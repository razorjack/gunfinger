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
- [x] 3. The containment rule (experiment 0101): passages, 10 s, bound
  none or 0.5, on the new mix at both sizes and its leave-outs: no wrong
  play to remove, no right play demoted; dropping each track's strongest
  file loses no right recording in any scan; every row of 0091 unchanged.
  Recommended: do not adopt
- [x] 4. The possible tier (experiment 0102): the listening answers of
  2026-10-14 in the roadmap and `docs/calibration.md`; the new mix shows
  3.6, 3.6, 2.7, 1.8 possible plays per hour at 60, 80, 100, 130 hits,
  all shared material; no true play only possible; no threshold changed
- [x] 5. Listening questions: none written. Track 17's slot holds no
  candidate (nothing at least possible; the strongest alignment 9 hits);
  `pair` settles the TI029 C side (neither the VIP nor the 1999
  original); no wrong confident play. Paths to judge by ear in the notes
- [x] 6. Search speed: profile; slot variants V2, V2b, V3 rejected
  (0103); kept the postings read on a second thread (0104) and peak
  records parsed from memory (0105): 3.1% less wall time per NAS search,
  `stats` 19% faster; reports and `regress` identical, memory +0.4%; and the
  index built on threads when the saved one is stale (0106): the first
  search after the store changes 53-55 s, not 116-130 s
- [x] 7. Read-only review by a subagent: no change to detections, the
  index or the saved-index file; fixed a dead panic mapping, two weak
  tests (buffer reuse, a batch's error paths), a stale doc comment,
  numbers in 0100-0106 and two scripts (status, step 7)

## Wrap-up

- [x] Notes "Session 18", status, roadmap ("More development mixes", the
  possible tier), ADR 0004's "Later"
- [x] `regress session-18-start` identical with the final build;
  `scripts/check.sh` green
- [x] The NAS store's revision at the end
- [x] `git diff docs/pair-verdicts.txt` empty and
  `work/audition/answers.jsonl` unchanged before every commit
- [x] Commit; the final message
