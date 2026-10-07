# Session 4 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` blocked (with the reason). See `docs/brief-4.md`
for scope and rules.

## Setup

- [x] Brief, checklist; `CLAUDE.md` points to them
- [x] Release build; baseline `session-4-start` (`regress session-3-start`
  first: identical detections; binaries kept in `work/bin/s4-start/`)

## 1. Idle-machine measurements of today's code

- [x] Where search time goes: `identify` on the development mix profiled,
  time attributed to decoding, index build, posting scans, sorting hits,
  clustering offsets, chaining and the second pass; today's matcher and
  skip at 240
- [x] Peak memory and time at scale: development scan at 262, 8,122,
  26,462 assets and as close to 32,768 as memory allows; today's matcher
  and skip at 240
- [x] End-to-end `identify` at 262 assets: turntable only, both
  playbacks, skip at 240 with both
- [x] Indexing throughput into a scratch store under `work/variants/`;
  per track, per hour of audio, projected for 20,000 tracks

## 2. Search memory at scale

- [x] Merge neighbouring rungs' lines as each rung finishes; `regress`
  identical (commit d4d8fb1: merged a block of windows at a time once
  every rung has searched it, since a rung-by-rung merge cannot be exact;
  one hit buffer per worker)
- [x] Lines held (`memory --count-lines`) and peak memory against item 1
  at 8,122 and 26,462 assets (experiment 0029)

## 3. Chance lines in chains

- [x] Opt-in: linked lines from nearby rungs, or a stronger line across a
  gap (`--nearby-rungs`, `--strong-gaps`; commit afc731c)
- [x] Checks: `explain --windows` on the Clockwork remix's third window;
  the window grid
- [x] Full protocol under today's matcher and skip at 240; case for
  adoption

## 4. Speed that wanders within a play

- [x] Opt-in: the second pass fits the speed per stretch of a few windows
  (`--speed-per-stretch`; commit afc731c)
- [ ] Star Trails, generated mixes, wow; extra analysis cost
- [ ] Full protocol; case for adoption

## 5. Peaks across frequency bands

- [ ] True hits per 1,000 postings scanned, by anchor band (offline)
- [ ] Variant profiles with peak budgets spread across bands, each in its
  own store under `work/variants/`
- [ ] Recall under the robust conditions, the development mix's kept
  hashes, postings, margins, index size

## 6. Detection boundaries

- [x] Opt-in trimming of weak chain ends (`--trim-ends`; commit afc731c)
- [x] Against the generated mixes' exact boundaries: true coverage lost
  beside the gain

## 7. The edge of the ladder

- [ ] Recall from ±8.2% to ±9%; the cost of extra rungs

## 8. Small engineering

- [x] Library identifier in the peak store; existing stores adopt their
  library without re-extraction (commit 028ee90)
- [x] Harness option adding a second library's peak records to the index;
  tested with a scratch library (commit b383281)
- [x] `doctor` counts only files within the track length range against
  the 32,768-asset limit (commit 9247e5f)
- [x] Shared material in the report: a possible play inside a confident
  play of another recording shown as "shares material with" (commit
  1dc6baa)

## Wrap-up

- [ ] README, roadmap, calibration, status up to date
- [ ] Session 4 summary at the top of `docs/notes-for-owner.md`
