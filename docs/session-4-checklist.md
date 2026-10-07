# Session 4 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` blocked (with the reason). See `docs/brief-4.md`
for scope and rules.

## Setup

- [x] Brief, checklist; `CLAUDE.md` points to them
- [ ] Release build; baseline `session-4-start`

## 1. Idle-machine measurements of today's code

- [ ] Where search time goes: `identify` on the development mix profiled,
  time attributed to decoding, index build, posting scans, sorting hits,
  clustering offsets, chaining and the second pass; today's matcher and
  skip at 240
- [ ] Peak memory and time at scale: development scan at 262, 8,122,
  26,462 assets and as close to 32,768 as memory allows; today's matcher
  and skip at 240
- [ ] End-to-end `identify` at 262 assets: turntable only, both
  playbacks, skip at 240 with both
- [ ] Indexing throughput into a scratch store under `work/variants/`;
  per track, per hour of audio, projected for 20,000 tracks

## 2. Search memory at scale

- [ ] Merge neighbouring rungs' lines as each rung finishes; `regress`
  identical
- [ ] Lines held (`memory --count-lines`) and peak memory against item 1
  at 8,122 and 26,462 assets

## 3. Chance lines in chains

- [ ] Opt-in: linked lines from nearby rungs, or a stronger line across a
  gap
- [ ] Checks: `explain --windows` on the Clockwork remix's third window;
  the window grid
- [ ] Full protocol under today's matcher and skip at 240; case for
  adoption

## 4. Speed that wanders within a play

- [ ] Opt-in: the second pass fits the speed per stretch of a few windows
- [ ] Star Trails, generated mixes, wow; extra analysis cost
- [ ] Full protocol; case for adoption

## 5. Peaks across frequency bands

- [ ] True hits per 1,000 postings scanned, by anchor band (offline)
- [ ] Variant profiles with peak budgets spread across bands, each in its
  own store under `work/variants/`
- [ ] Recall under the robust conditions, the development mix's kept
  hashes, postings, margins, index size

## 6. Detection boundaries

- [ ] Opt-in trimming of weak chain ends
- [ ] Against the generated mixes' exact boundaries: true coverage lost
  beside the gain

## 7. The edge of the ladder

- [ ] Recall from ±8.2% to ±9%; the cost of extra rungs

## 8. Small engineering

- [ ] Library identifier in the peak store; existing stores adopt their
  library without re-extraction
- [ ] Harness option adding a second library's peak records to the index;
  tested with a scratch library
- [ ] `doctor` counts only files within the track length range against
  the 32,768-asset limit
- [ ] Shared material in the report: a possible play inside a confident
  play of another recording shown as "shares material with"

## Wrap-up

- [ ] README, roadmap, calibration, status up to date
- [ ] Session 4 summary at the top of `docs/notes-for-owner.md`
