# Session 12 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-12.md`
for scope and rules.

## Setup

- [x] `git status` clean at `7ef893f`
- [x] Release build in `work/bin/s12-start/` (byte-identical to
  `work/bin/merged/`); `baseline session-12-start`
- [x] `regress session-12-start` with the start build: identical (720/720,
  87, 93, 79), the known calibrate line; 212.7 s
- [x] The NAS store at the start: 33,596 `.peaks`, 33,596 `.tags`, 1,582
  `.skip`; revision `f9f9bc706195a71b`, as expected; census
  `work/s12/census-start.json` (3,523.4 h)
- [x] The saved NAS index 3.8 GB
- [x] Brief, checklist; `CLAUDE.md` points to them (`3870fdd`)

## Items

- [x] 2. Profile the merged search (the 2003 mix at NAS scale, `sample`
  every 10 ms): 177.7 s, sorting 78.1% of the CPU (grouping by asset
  49.9%, each asset's offsets 28.2%); experiment 0085 (`2bcce3d`)
- [x] 3. A counting sort by asset, 8-byte found hits, no threshold;
  unit test against a comparison sort; `scripts/check.sh` green (4:49),
  `regress session-12-start` identical, the 2003 mix at NAS scale
  byte-identical in 85.7-92.1 s against 166.6-171.3 s, the same peak
  memory; experiment 0086 (`1345be7`)
- [x] 4. The harness uses the saved index (ADR 0014): one file per other
  store in `work/indexes/`, leave-outs filtered in place
  (`Index::without_assets`); tests (stale, damaged, truncated, changed
  store, leave-outs); checks pass; regress identical; the development
  scan at NAS scale 112 s loaded against 189 s built, leave-out 3 107
  against 180 s, reports identical; experiment 0087 (`de4577e`)
- [x] 5. `doctor` reports the store's saved index (size; current, current
  for `--store-only` only, out of date, unreadable or missing) and the
  folder's other files with their sizes, deleting nothing
  (`saved::examine` reads the header and length); tests; checks pass (260
  tests); regress identical; NAS: the saved index current

## Wrap-up

- [x] Experiments 0085-0088; notes "Session 12"; status; checklist;
  roadmap; README; AGENTS.md
- [x] Extra: `sweep --seed 2026` at NAS scale on the harness's saved
  index, byte-identical to session 11's report (641 s)
- [x] The NAS store's revision at the end: `f9f9bc706195a71b`, 33,596
  records, 1,582 skip notes
- [x] Commit; the final message
