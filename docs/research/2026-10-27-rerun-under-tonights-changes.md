# The evaluation protocol again, under tonight's changes (2026-10-27)

**Status: pre-registration**, committed in session 57 before any of its
runs. It fixes which runs are made, which old report each is compared
with, and what counts as an explained difference and as a finding. It is
not edited after its commit; a deviation is written in
`docs/brief-57.md` with its reason, and the experiments say where they
depart from it.

## Why

Sessions 52-56 changed what the evaluations stand on:

1. **The prune** (session 52, experiment 0231): the NAS store lost the
   records of five ignored files, the three tracks of
   `knowledge-the_lowdown_vol_7-track_fix-1999-sour/` (Propaganda,
   Drumtrip, Kraken), the Space Cake `[CUT]` upload and the T-Minus
   vocal mix upload. 40,683 -> 40,678 records.
2. **The second suppression** (session 53, ADR 0021): a detection whose
   span overlaps a stronger one of the same file is dropped only when
   their fitted speeds are within 0.6%.
3. **Same-title credits** (session 54, ADR 0022): a confident detection
   of a file judged to carry the truth's title (`docs/same-titles.txt`:
   China Cup against ƆƐI3C's upload) is right and counted apart.
4. **Gap lines** (sessions 55 and 56, ADR 0023 and 0024): `identify` and
   `show` print stretches of 45 s or more without a confident match as
   "ID - ID" lines with an estimate of their tracks; JSON gains `gaps`
   and `gap_repeats`. The harness's reports do not change.

`regress` covers the sweep of seed 2026, the Stakka scan and its
leave-outs, and calibrate; both regressions were identical at the start
of session 57. Every run below last ran before at least one change.

## The runs and the old reports

Every old report below was written on 2026-10-25 or earlier, before all
four changes, unless a row says otherwise. Commands are the harness's
(`gunfinger-eval`, global options first) or the CLI's (`gunfinger`);
`$NAS` is the NAS peak store on the local disk. Where session 38 or 45
also saved a `--apart-in-speed 0.006` report (the variant ADR 0021
adopted), it is a second comparison: the new default should equal it
apart from the prune and same-title credits.

**Group 1, the shared panels' development halves, NAS scale**
(`--other-peaks-dir $NAS --panels work/s33/panels`):

| Run | Old report (session) |
|---|---|
| `shared docs/panels/shared-material-seed-2026-development.json` | `work/s44/w/reports/library-nas-dnb-peaks/` (44, the newest default); `work/reports/library-nas-dnb-peaks/` and its `variant-apart-0.006/` (38, 0179) |
| `shared docs/panels/shared-material-2-seed-2026-development.json` | `work/s45/w/reports/library-nas-dnb-peaks/` and its `variant-apart-0.006/` (45, 0209) |

**Group 2, the drops of sessions 39 and 48** (the harness at 262 tracks
with `--count 16 --other-root /mnt/atlas/Music/dnb` and session 39's
`--tagged` lists; the CLI with `--saved-index off` on the same renders,
at 262 tracks on the store-only copies of sessions 39 and 48, at NAS
scale `--peaks-dir $NAS identify --store-only --exclude-from` the
held-out recordings' NAS files):

| Run | Old report (session) |
|---|---|
| `mixes --kind drops --seed 2026`, `2028` | `work/s45/w0/reports/` and its `variant-apart-0.006/` (45, 0210; = session 39's) |
| `mixes --kind drops --other-pairs --seed 2026`, `2028` | `work/reports/` (39, 0185) |
| `--ladder turntable --verify mixes --kind drops --seed 2032`, `2034` | `work/s48/w/reports/ladder-turntable/verified/` (48, 0216) |
| `--ladder turntable mixes --kind drops --other-pairs --seed 2032`, `2034` | `work/s48/w/reports/ladder-turntable/` (48) |
| `--ladder turntable mixes --kind drops --aligned-drops --seed 2026`, `2028`, `2032`, `2034` | the same (48, 0217) |
| CLI, 262 tracks: drops 2026, 2028; 2032, 2034; aligned 2026-2034 | `work/s45/cli-262-default/` (45); `work/s48/cli-262/`, `cli-262-aligned/` (48) |
| CLI, NAS scale: drops 2026, 2028; 2032, 2034; other pairs 2026, 2028; 2032, 2034; aligned 2026-2034 | `work/s45/cli-nas-default/` (45); `work/s48/cli-nas/`; `work/s39/cli-nas-other/` (39, made with the NAS configuration, so its index held the library's listed files: after the prune the same records); `work/s48/cli-nas-other/`, `cli-nas-aligned/` (48) |

Session 48's turntable `--verify` reruns of seeds 2026 and 2028 play
the same mixes as session 39's and are not run again.

**Group 3, `recall` and the other development scans:**

| Run | Old report (session) |
|---|---|
| `--other-peaks-dir $NAS --panels work/s33/panels recall` with `--queries clean`, `damaged`, `brief`, `brief-damaged`, each with and without `--other-rips-only` (8 runs) | `work/reports/library-nas-dnb-peaks/recall-seed-2026*.json` (33, 0156) |
| `--other-peaks-dir $NAS scan razorjack-2003-03-29` plain, `--leave-out 3`, `--leave-out 12`; `scan nocny-trans-vertical-2002-08-23` plain, `--leave-out 3 --seed 2026`, `2027`, `2028`, `--leave-out 17` (8 runs) | `work/reports/library-nas-dnb-peaks/` (33, 34) and its `variant-apart-0.006/` (38) |
| `scan` of the same two mixes at 262 tracks | `work/reports/` (34) |

**Group 4, sweeps, `robust`, `grid`:**

| Run | Old report (session) |
|---|---|
| `--other-peaks-dir $NAS sweep --seed 2027`, `2028` | `work/s40/sweeps-nas/` (40, before all changes); `work/s54/wseeds-start/` (54, before ADR 0022 only) |
| `sweep --seed 2027`, `2028` at 262 tracks | `work/s54/wseeds-start/` (54, before ADR 0022 only) |
| `sweep --seed 2029`, both scales | none on this machine: also run with `--overlap-by-spans` in the same build |
| `calibrate` over seeds 2026-2029 and the Stakka scans, both scales | session 54's over 2026-2028 (0233) and `docs/calibration.md` |
| `robust --seed 2026 --only pitch-ride-2pct,ride-1pct-at-15s,ride-2pct-at-15s,ride-3pct-at-15s,ride-2pct-at-24s`, both scales | `work/reports/` and `library-nas-dnb-peaks/` with their `variant-apart-0.006/` (38, 0180) |
| `grid`, 262 tracks | none on this machine (the Mac, session 29 or earlier): also run with `--overlap-by-spans` in the same build |

**Group 5, long and ridden generated mixes, 262 tracks, on the saved
renders** (`--count 16`):

| Run | Old report (session) |
|---|---|
| `mixes --seed 2026`; `mixes --kind long --seed 2026` | `work/reports/` and `variant-apart-0.006/` (37, 38) |
| `mixes --kind rides --seed 3801` with `--ride-change 0.03` or `0.01,0.02` and `--ride-seconds 2,5` or `10,20` (4 runs) | `work/reports/` and `variant-apart-0.006/` (38, 0179) |
| `mixes --kind rides --seed 3741`, `3742` | `work/reports/` (37, 0178) |

Not run, by design: `shared-mixes` (no full report on this machine;
session 21 ran it on the Mac); `robust` without `--only` (no saved full
report here); mixes whose renders were deleted (plain and long seeds
2027-2029 and 7, long 3531-3532, rides 3521-3522); the shared and
beatmatched seeds 3501-3513, which session 56 ran after every change;
the rides dose of seed 3701, made by trace builds; NAS-scale generated
mixes, of which none was saved.

All runs use `work/bin/s57-start/` (= `s56-final`, master), 32 jobs,
in scratch work directories under `work/s57/` (never `work/reports/`),
with copies of the clusters file and the library map, the saved NAS
index linked, and earlier renders copied by reflink so that a run
whose plan is unchanged reads the same audio. The old reports are read,
never written. The held-out halves of the shared panels (their
validation halves) stay unsearched: only the `*-development.json` panel
files are passed to `shared`.

## What is compared

For each run, against its old report: the scored counts (identified,
right, wrong confident, possible, same-title credits, recall, per
level); every confident detection in one report and not the other; and
the weak and possible detections that differ, counted. For the shared
panels and drops, ADR 0021's residual risk is counted explicitly
(`work/s57/residual.py`): every detection whose span overlaps a
stronger one of its file more than 0.6% away in speed (one the old rule
dropped), by hits (under 60, 60-239, 240 or more) and, in `shared`, by
role; any that reaches 240 hits is listed by name.

## What counts as explained

A difference is explained when it is traced, detection by detection, to
one of tonight's changes:

- **the prune**: a detection of one of the five pruned files is gone
  (and nothing else changes in its query or mix, or what changes follows
  from its absence, such as a weaker candidate of another file now
  listed); an index holds up to five fewer assets;
- **a kept segment more than 0.6% apart**: a detection overlaps a
  stronger one of its file more than 0.6% away in speed (added), or one
  is gone because it overlaps, within 0.6%, such a newly kept detection
  (removed; ADR 0021's consequences);
- **a same-title credit**: a confident detection of a file that
  `docs/same-titles.txt` credits to the truth's title is now right and
  counted apart, where it was wrong;
- **a gap line**: in a printed tracklist, an "ID - ID" line, its
  estimate, or a table row or JSON field that ADR 0023 or 0024 adds.

Where the old report also predates an earlier documented change (for
example a report made before session 52's harness fix of
`identifiability`), a difference traced to that change is listed as
such, by name, and still counts as a finding below if it touches a
confident detection or a level.

## What counts as a finding

Anything not explained above, and in particular:

- a new wrong confident detection (a confident detection of a file not
  playing, or not of the truth's cluster or title), also an added
  segment of a file the old report already named wrongly;
- a lost confident identification (a track, play, excerpt or query
  confident on its truth before and not now);
- a level change (a play, track or query going between confident,
  possible and unmatched) that no explained difference accounts for;
- a run that fails, or whose query, mix or excerpt counts differ from
  the old report's for a reason other than the prune.

A finding does not change the product (brief 57, step 4): it is
described, reproduced by the smallest command, and put first in the
handoff.

## Order and time

The runs go in the prompt's order and stop where time runs out (no new
run after about 14:30); what was not run is recorded. Each group's
experiment (at most 40 lines) is written before the next group's runs
start: shared panels; drops; recall and scans; sweeps, `robust` and
`grid`; long and ridden mixes. Within a group, runs may go in two
queues at once, so wall times are not comparable with the old runs';
detections do not depend on the load. A group starts only if its runs
can end by about 10:15, so that the review and wrap-up end near the
time box (10:45); otherwise it is recorded as not run.
