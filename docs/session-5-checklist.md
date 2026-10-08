# Session 5 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-5.md`
for scope and rules. NAS plan steps in brackets.

## Setup

- [x] Brief, checklist; `CLAUDE.md` points to them
- [x] Release build; `regress session-4-start` identical; baseline
  `session-5-start` (binaries kept in `work/bin/s5-start/`)
- [x] The NAS store's file counts, digest and library revision
  (`d709390272e41475`) recorded at the start (`docs/status.md`)

## Items

- [x] 1. The store as indexed [1, 2]: throughput over Wi-Fi against
  experiment 0028; composition (channels and folders, formats, lengths,
  tag coverage, exact copies, names on several files); `doctor` and
  `stats` on the store alone (experiment 0035)
- [x] 2. The content map [3]: `map-library`; corpus files with no copy
  or several (experiment 0036)
- [x] 3. Clusters of the corpus recordings at NAS scale [4]: do the 17
  corpus clusters reappear; further rips, coverage, hits; borderline
  pairs for the owner (experiment 0037)
- [x] 4. Related recordings at NAS scale [5]: pairs of 30 hits or more;
  possible tier, 200, 240; same artist or label against unrelated; China
  Cup and The Nine; Fresh & Vegas - Mekon if present (experiment 0038;
  labels from tags and artists, not record labels: YouTube paths name
  none)
- [x] 5. The development set at NAS scale [6], both matchers: scan,
  leave-outs 3 and 11; identified, wrong, strongest false and margin,
  possible plays, wall, CPU, peak memory (experiment 0039)
- [x] 6. Sweeps at NAS scale [7]: seeds 2026-2029 under skip at 240,
  2026 and 2027 under today's matcher; `calibrate` for each; strongest
  false candidates named (experiments 0040, 0041)
- [x] 7. The proxy against reality [8]: `memory` at NAS scale under
  `time -l`, both matchers, against experiments 0012, 0019, 0027-0029
  (experiment 0042)
- [x] 8. Posting lists and famous breaks [9]: `stats` figures; records
  holding the fullest 1%; false candidates per track with and without
  skip (experiment 0043)
- [x] 9. Identifying without the NAS [10]: store-only index build time
  and memory; `identify --store-only` on the development mix with names
  from the tags (experiment 0044; internal disk only, no external disk
  attached)
- [x] 10. Wrap-up: session 5 summary in the notes; NAS figures in
  calibration as measurements; roadmap, status, NAS checklist and plan

## Only if time remains

- [-] 11. Today's matcher, sweep seeds 2028 and 2029, calibrate again:
  not run; each takes 66-69 minutes at NAS scale, and the link rules
  (item 12) were queued first
- [-] 12. Link rules at NAS scale: development scan and sweep 2026, both
  matchers. Done in part (experiment 0045): the scan under both matchers
  and sweep 2026 under skip at 240; today's sweep with the rules not run
  (66-69 minutes, past the session's end)
- [-] 13. A scaling curve on real records (seeded subsets of the other
  library): not run, no time left; 0042 compares the two ends (262 and
  26,914 assets) and the proxy instead

## At the end

- [x] The NAS store's file counts, digest and library revision checked
  again at 02:04: 26,890 records, 26,890 tags, 1,549 skip notes,
  `library.txt`; digest `7055ad8e...65a70` and revision
  `d709390272e41475`, as at the start
