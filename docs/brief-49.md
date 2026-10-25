# Session 49 brief: copies of one recording that drift, in pair alignments

Written on 2026-10-25, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-49-checklist.md` (progress).
3. `docs/brief-40.md` (session 40's pre-registration: the populations,
   the measures, the scores); experiments 0186-0188 and this session's
   (0219 and up); the roadmap's "The edge of the ladder", "Speed that
   wanders within a play", "The verdicts by ear on borderline pairs" and
   "Clustering precision".
4. `docs/session-notes.md` ("Session 40", "Session 47", "Session 48"),
   `docs/status.md` (newest entries, "after session 46" among them),
   `work/audition/listening-comments-2026-10-25.md`; the code of
   `crates/gunfinger-eval/src/pair.rs`, `clusters.rs` (`pair`,
   `at_fitted_speed`), `audition/placement.rs`, `web/timeline.js`,
   `web/player.js`, and `scripts/analysis/pair_offsets.py`.

`docs/brief-2.md` to `docs/brief-48.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 49 of Gunfinger: copies of one recording that drift, start late or sit near the ladder's edge, in pair alignments. The listening of 2026-10-25 (docs/status.md, "after session 46"; verbatim in `work/audition/listening-comments-2026-10-25.md`) judged Falcon - The Stand's "HD Audio & Speed Up" upload the same recording as the CITRUS 003 upload, with the user's reading: "one of them has longer silence at the end. This is why if they get speed matched, they diverge over time. If they were cut the moment the track actually ends, it would be a match". Its sheet (`work/listening-s40/disagreement-citrus-recordings-citrus-003-falcon-5262ac/sheet.txt`) has one main alignment at +7.78% (A 0:30-7:38) holding 0.72 of the file, no alignment over A's first 30 s, and a 26-hit alignment 221 s away. Session 40 (experiments 0186-0188) found that every offset-aware measure fails on same recordings whose speed drifts (Phoenix, Fractles, Coma) because one rung follows one speed, and that session 40's near-offset share (`near_10`) met its bar by one pair. The roadmap's "The edge of the ladder" notes Falcon - The Stand's cluster spans 11.5% in speed. This session finds out what actually happens in the Falcon pair, tests whether trailing silence or file length can move any fit or placement, builds a near-offset measure that follows drift, and scores it on the judged pairs, the ten verdicts of 2026-10-25 kept apart as data no measure was tuned on. It is a measurement session: the clusters' join rule, the product and the clusters file do not change.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).

Time box: started about 03:55 (CET; the clocks went back an hour tonight); aim to finish within about 2.5 hours, because another session follows you. If a step would run past that, scale it down, record the decision in the brief and continue.

Environment (sessions 33-48's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS store `~/.local/share/gunfinger/nas-dnb-peaks` (40,683 records), library `/mnt/atlas/Music/dnb` (read only; on Ethernet again tonight), reached with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: after the listening of 2026-10-25 (docs/status.md, "after session 46") SHA-256 `fdf7428a6322...`, the same 284 clusters as before. Baselines `session-32-linux` and `session-33-linux-nas`. Build `work/bin/s48-final/` (master). Since sessions 41, 43 and 46 a NAS-scale development scan searches in about 24 s and `regress session-33-linux-nas` takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans, sweep reports of seeds other than 2026 (calibrate reads every sweep report there), and reports of opt-in variants must not sit in a reports directory during `regress`; run variants in a scratch work directory.
- `pair` writes `reports/.../pairs-<file stem>.json`: give your pair files stems of their own so no earlier report is overwritten.
- Session 38's opt-in `--apart-in-speed` (confirmed by session 45; adoption ours, with an ADR) and session 37's `--overlap-by-chains` are variants, not defaults.
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen tonight. Pending listening (items 4, 5, 7 and 9 of session 40's pack, in `work/s47/questions/05-pack-s40.json`, session 39's questions, Star Trails, Into Music) stays for later.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-40.md and docs/brief-48.md, docs/session-notes.md ("Session 40", "Session 47" and "Session 48"), docs/status.md (newest entries, including "after session 46"), the roadmap's "The edge of the ladder", "Speed that wanders within a play", "The verdicts by ear on borderline pairs" and "Clustering precision", experiments 0033, 0049, 0057, 0059 and 0186-0188, `work/audition/listening-comments-2026-10-25.md`, the code of `pair.rs`, `clusters.rs` (coverage, supported time, the fitted speed), `search/refine.rs` and the `--speed-per-stretch` code, the listening page's placement (`audition/placement.rs`, `web/timeline.js`, `web/player.js`: how speed matching resamples and places lane B) and `scripts/analysis/pair_offsets.py`.
2. Write docs/brief-49.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-49-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt in work/s49/listening-digests-start.txt; it must be the same at the end. Experiments number from 0219.
3. Build `work/bin/s49-start`; `regress session-32-linux` and `regress session-33-linux-nas` identical.

Steps:

1. The Falcon pair, diagnosed (exploratory; say so): both files' lengths, leading and trailing silence (measured from decoded audio or the stored peaks), the alignments at every rung near +7.8% and with `--speed-per-stretch`, whether the speed is constant or drifts over the file, why A's first 30 s hold no alignment, and whether anything in `pair`, `clusters`, the sheet or the listening page uses a file's length or its end (so that trailing silence could move a fit or a placement). Write experiment 0219: what the user heard, what the evidence shows, and whether the user's reading holds, in plain terms.
2. Pre-registration, committed before the new measure is computed on any judged pair other than session 40's development pairs: a drift-following near-offset measure (for example the share of the shorter file whose aligned hits lie within T s of a slowly varying offset, fitted piecewise or by `--speed-per-stretch`), defined exactly, with its parameters chosen on session 40's populations only; the test data, the ten verdicts of 2026-10-25 (`git show dc4f7dc -- docs/pair-verdicts.txt`), which no measure has seen, reported apart and also pooled; the scores as session 40's (AUC, leave-one-out errors on the judged pairs, the pairs it gets wrong); and what would count as better than `near_10` and supported time.
3. The harness or a script, tested (harness only, `regress` identical), and the runs: `pair` on the populations and the new verdicts (the NAS on Ethernet).
4. Experiments (at most 40 lines each, written before the next starts): the measure against the registered scores; `near_10`, supported time and coverage on the ten new verdicts, as a first test on unseen judged pairs.
5. Ahead, only when 1-4 are done: how many of the NAS clusters span more than 4%, 6% and 8% in speed now, and which copies would fall out of the ladder for a deck at ±5%, from the clusters file and the stored peaks (no new clustering run); write it into the roadmap's "The edge of the ladder".
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 49" section at the end of docs/session-notes.md with a handoff and the questions it leaves for us; update docs/status.md, the checklist and the roadmap's bullets above. Final checks: `scripts/check.sh` green, `regress session-32-linux` and `regress session-33-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: the Falcon diagnosis in plain terms, the measure's scores, the ten new verdicts' scores, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's behaviour does not change: no threshold, parameter, rule, default or output change; variants are opt-in, tested, and keep `regress` identical when off. Harness additions are tested and keep `regress` identical.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
- Scratch data goes under work/s49/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:59. Start no new run after about 11:30; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 49

- 2026-10-25, 03:51 by the Linux machine's clock (CET). The time box
  is read as 2.5 hours from about 03:55: finish by about 06:25.
- `master` at `12d347d`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s49/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `d3bb1fe061f7...`, the NAS clusters file
  `fdf7428a6322...`, `work/audition/answers.jsonl` `78c3fb0e7ce0...`
  (recorded as well, though no step here touches it).
- Experiments 0001-0218; this session numbers from 0219. Test set: 4 of
  5 evaluations used (none in this session).
- Session 40's data: populations `work/s40/populations/s40-{a,b,c,d}.txt`
  and `populations.json`; reports
  `work/reports/library-nas-dnb-peaks/pairs-s40-{a,b,c,d,pack,source}.json`;
  measures and scores `work/s40/measures*.json`, `scores*.json`.
- The Falcon pair (one of the ten verdicts of 2026-10-25) was measured
  by session 40's pack run (`pairs-s40-pack.json`); its question as the
  user heard it is in `work/audition/questions/05-pack-s40.json`.
- Disk: 790 GB free; `work/mixes/` holds 23 GB. No render is planned.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build and both regressions (tmux `s49start`) | 03:51-04:05 |
| 1 | the Falcon pair: lengths, silence, drift (decoded audio), per-rung alignments (stored peaks), what uses a file's length; 0219 | 04:05-04:35 |
| 3a | `pair --fine-rungs` (per-rung searches around the fitted speed), tested; both regressions | 04:20-04:50 |
| 2 | pre-registration in this brief; commit | 04:50-05:00 |
| 3b | `pair --fine-rungs` on populations a-d and the ten verdicts | 05:00-05:15 |
| 4 | 0220 (the measure), 0221 (the ten verdicts) | 05:15-05:40 |
| 5 | the clusters' speed spans; roadmap | 05:40-05:55 |
| 6-7 | review, notes, status, roadmap, final checks | 05:55-06:25 |

The harness option comes before the pre-registration because step 1
needs the per-rung alignments; the measure is not computed on any judged
pair before the pre-registration is committed (Falcon's per-rung
alignments are looked at in step 1, so Falcon is declared seen).

## Decisions

- **(03:51) Start build and regressions** (`work/s49/start.sh`, tmux
  `s49start`, logs in `work/s49/logs/`; `run.sh` and `same.py` are
  session 48's with the paths changed).
- **(03:57) Start checks.** The start build is byte-identical to
  `work/bin/s48-final/` (cargo had nothing to rebuild). `regress
  session-32-linux` (0:50) and `regress session-33-linux-nas` (3:46)
  identical: every report identical to its baseline's apart from
  `wall_seconds` (`work/s49/same.py`).
- **(04:01) Harness, `pair --fine-rungs`** (opt-in; `pair::FineRungs`):
  beside the four searches of stored peaks, the query is searched on 20
  more rungs, the fitted speed times 1 + k × 0.1% for k = ±1 to ±10
  (`peaks, fine rung -1.0%` to `+1.0%`), each alignment with its
  supported seconds as at the fitted speed. Step 0.1%: every speed within
  1% is within 0.05% of a rung, where about 90% of hashes survive
  (experiment 0001). Without the option the report is unchanged: `pair`
  on session 40's source pairs gives a report identical to
  `pairs-s40-source.json`. A unit test builds a query whose last third
  plays 0.8% off and checks that the fine rungs hold it and the fitted
  speed does not. Run build `work/bin/s49-run/`.
- **(04:02) `--speed-per-stretch` in step 1, replaced.** It is an option
  of the second pass on decoded audio (`search/refine.rs`), and `pair`
  decodes only a corpus query; both Falcon files are NAS uploads. The
  fine rungs on stored peaks give the same information (which speed each
  stretch lines up at), so step 1 uses them, with an envelope lag measured
  on the decoded audio as a second, independent reading.
- **(04:04) Step 1, experiment 0219** (exploratory). The CITRUS 003
  upload's speed drifts (1.031 to 1.038 times the vinyl rip's over the
  track); the Speed Up upload is about 1.116 times the vinyl rip
  throughout. A ends with 4-6 s of silence that moves no fit and no
  placement. Falcon's per-rung alignments were read here, so Falcon is
  declared seen: the test scores are given with and without it.
- **(04:08) Three of the ten verdicts sit in session 40's presumed
  populations:** Dead Air ~ the RH 66 upload and Pyro - Time Is Broken's
  two rips in (b), Known Unknown - Drumtrip ~ Propaganda in (d). They
  were never judged pairs there, but the parameter choice below reads
  (b). Decision (conservative): the choice and the development scores
  use (b) and (d) without these three pairs, so nothing chosen saw the
  ten.
- **(04:10) Step 2, the pre-registration** (committed before the new
  measure is computed on any pair; only Falcon's per-rung alignments
  have been read, in step 1).

  *Data.* Development: session 40's populations (`work/s40/populations/`;
  (a) 60 judged pairs without the two source pairs, 22 same and 38
  different; (b) 98 presumed same less the two above; (c) 51 panel
  different; (d) 60 presumed different less the one above), searched
  again with `pair --fine-rungs` (`work/bin/s49-run`; stems
  `s49-{a,b,c,d}`). Test: the ten verdicts of 2026-10-25 (`git show
  dc4f7dc -- docs/pair-verdicts.txt`, `work/s49/new-verdicts.txt`), 6
  same and 4 different, in the verdict lines' order (first file the
  query), stem `s49-new`. Known before this registration: the status
  ("after session 46") says each of the ten lies on its verdict's side
  of `near_10`'s threshold (0.552), and the ten were picked by session
  40 as the pairs nearest that threshold, so they are not a random
  sample of judged pairs, and `near_10` makes no error on them.

  *The measure.* From one `pair --fine-rungs` report: F the search
  "peaks, at the fitted speed", M its first alignment (speed s_M, offset
  o_M: found time = o_M + s_M × query time), S the shorter file's length.
  For a width W, R(W) is F's alignments and those of every search
  "peaks, fine rung x%" with |x| ≤ W. An alignment a of R(W) (speed s_a,
  offset o_a, query span [q0, q1]) is *near* when its line stays within
  T s of M's over its span: |(o_a + s_a q) - (o_M + s_M q)| ≤ T at q =
  q0 and q = q1. `drift_W_T` is the number of distinct seconds in the
  union of the near alignments' `supported_seconds`, over S. With F empty
  it is 0. `drift_0_T` (F only, nearness by line) is reported beside it
  as the measure without fine rungs; it differs from `near_T` only in
  comparing lines instead of offsets at query time 0.

  *Parameters, chosen on the development populations only*: W in {0.2,
  0.4, 0.6, 0.8, 1.0}% and T in {2, 5, 10, 20} s. The chosen pair (W, T)
  has (1) the fewest leave-one-out errors on (a); among ties (2) the
  fewest (b) joins below the threshold fitted on all of (a); then (3)
  the highest AUC on (a); then (4) T = 10 s before the others, then the
  smaller W, then the smaller T. Its threshold is the one fitted on all
  of (a) (session 40's `best_threshold`), fixed before the test pairs'
  values are computed. Every other (W, T) is reported as found, not as
  confirmed.

  *Scores* (`scripts/analysis/drift_offsets.py`), for `coverage`,
  `supported`, `near_10`, `drift_0_10` and the chosen `drift_W_T`:
  - development, as 0186: AUC (a); AUC (b) ~ (a) different, (b) ~ (c),
    (b) ~ (d); the threshold fitted on (a), its errors and leave-one-out
    errors; (b) below, (c) and (d) above it, by name; the source pairs'
    sides;
  - test, the ten apart: AUC (6 same against 4 different), the errors
    at the threshold fitted on (a) and the pairs it gets wrong, with and
    without Falcon;
  - pooled, (a) and the ten (28 same, 42 different): AUC, the errors at
    the best threshold, leave-one-out errors and the pairs wrong.

  *Better than `near_10` and supported time* (worth a roadmap entry; a
  join rule stays ours, with an ADR): on the ten, at the thresholds
  fitted on (a), no more errors than either; pooled, fewer leave-one-out
  errors than both; and no (b) join below its threshold. Since `near_10`
  makes no error on the ten, the ten can at best tie it there; the
  pooled leave-one-out errors decide. The drift pairs of (a) (Phoenix,
  Fractles, Coma ~ Spraycan) and Falcon are listed with every measure's
  values as an exploratory reading.
