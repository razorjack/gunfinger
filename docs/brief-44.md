# Session 44 brief: why reference pairs are lost under damage

Written by the agent for itself on 2026-10-23, so the plan survives
context compaction. Work autonomously and never stop to ask: where a
decision is needed, take the conservative option, write it down here and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-44-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 30" and "Session 43"),
   `docs/status.md` (newest entries), the roadmap's "Evidence beyond
   exact pair hashes" (all three steps) and "Recall beyond the corpus
   recordings", ADR 0001, experiments 0004, 0027, 0054-0059, 0149, 0151
   and this session's (0206 and up).
4. The code of `crates/gunfinger-eval/src/verifier.rs`,
   `verifier/pairs.rs`, `crates/gunfinger-core/src/hash.rs` and
   `search/refine.rs`; `scripts/analysis/quantisation.py`.

`docs/brief-2.md` to `docs/brief-43.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 44 of Gunfinger: why reference pairs are lost under damage, and whether pairing the query with more targets in the second pass recovers them. The roadmap's "Evidence beyond exact pair hashes", step 2's open part (a proposal from an external review, 2026-10-17): under combined damage the peak verifier finds a median 37% of the reference peaks while exact pairs (E) keep 9.6% of the clean exact pairs (session 30, experiments 0149 and 0151, `verifier/pairs.rs`). A reference pair can be lost because an endpoint is gone, because both endpoints are in the query but the target is displaced (another peak, from a blend for example, now comes first in the anchor's target zone; each anchor pairs with its next 2 peaks, ADR 0001), or because both are found with a changed quantised value (N). First split the lost pairs into these kinds. Only if displaced targets are a large share, pair the query with more targets (fan-out 3, 5 and 8) in the second pass alone, the index unchanged at fan-out 2 (experiment 0004 changed both), each reference pair counted once, and measure what that gains for the played recording against what it gains for unrelated candidates and related versions (step 2 predicts related versions gain as much as the played one; if they do, that settles it). It is a measurement session: the product, its rules and the index do not change.

You run unattended: the user is away and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).

Time box: started about 10:10, after the user's soft deadline of 11:00 was near; aim to finish within about 2.5 hours. Keep the scope to steps 1-4 and the wrap-up; if a step would run past that, scale it down (262 tracks first, fewer fan-outs), record the decision in the brief and continue.

Environment (sessions 33-43's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS store `~/.local/share/gunfinger/nas-dnb-peaks` (40,683 records), library `/mnt/atlas/Music/dnb` (read only), reached with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json` (SHA-256 `d9c85c4586ad...`, 284 clusters). Baselines `session-32-linux` and `session-33-linux-nas`. Build `work/bin/s43-final/` (master; since sessions 41 and 43 the search is about a third faster than in session 40: a NAS-scale development scan searches in about 26 s, `regress session-33-linux-nas` takes about 4 minutes). `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- Session 38's opt-in `--apart-in-speed` and session 37's `--overlap-by-chains` are variants, not defaults (adoption waits for an ADR, ours).
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans, sweep reports of seeds other than 2026 (calibrate reads every sweep report there), and reports of opt-in variants must not sit in a reports directory during `regress`.
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-43.md, docs/session-notes.md ("Session 30" and "Session 43"), docs/status.md (newest entries), the roadmap's "Evidence beyond exact pair hashes" (all three steps) and "Recall beyond the corpus recordings", ADR 0001, experiments 0004, 0027, 0054-0059, 0149 and 0151, and the code of `verifier.rs`, `verifier/pairs.rs`, `search/refine.rs`, `hash.rs` (pairing, target zone, fan-out) and `recall.rs`.
2. Write docs/brief-44.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-44-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt in work/s44/listening-digests-start.txt; it must be the same at the end. Experiments number from 0206.
3. Build `work/bin/s44-start`; `regress session-32-linux` and `regress session-33-linux-nas` identical.

Steps:

1. Pre-registration, committed before any count: the populations (as session 30's: true detections under combined damage and other rips only, unrelated candidates, related versions on the shared panels as in 0151), the split of lost reference pairs (endpoint gone; both endpoints found within the verifier's tolerance but the target displaced from the anchor's first two; both found and paired but with a changed quantised value), how each is decided exactly, what share of displaced targets would justify the fan-out step, the fan-outs (3, 5, 8, second pass only, each reference pair once), and the scores (pairs recovered for true detections, for unrelated candidates and for related versions; true detections that would cross 240; related or unrelated ones that would; CPU of the second pass) with the criterion that would make a fan-out worth a later experiment on the rules.
2. The harness, tested: the split as a diagnostic in `verifier/pairs.rs` or beside it, and the second-pass fan-out as a harness-only option (the CLI, the core's defaults and the index unchanged; `regress` identical).
3. The runs: at 262 tracks and at NAS scale, under tmux with logs if long.
4. Experiments (at most 40 lines each, written before the next starts): the split; then, only if the registered share is reached, the fan-outs against the criterion; otherwise say what the lost pairs are and what that means for step 3 (denser query peaks).
5. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
6. Wrap-up: a "Session 44" section at the end of docs/session-notes.md with a handoff and the questions it leaves for us; update docs/status.md, the checklist and the roadmap's bullet. Final checks: `scripts/check.sh` green, `regress session-32-linux` and `regress session-33-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: what was built, the split and the fan-out results in a few lines, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's behaviour does not change: no threshold, parameter, rule, default or output change; variants are opt-in, tested, and keep `regress` identical when off. Harness additions are tested and keep `regress` identical.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user runs among them).
- Scratch data goes under work/s44/ (your session number).
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline of 11:00 has nearly passed; this is the night's last session. Work through it as planned, start no optional extra work, do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 44

- 2026-10-23, 10:05. `master` at `76b6859`, clean. Git identity
  `Jacek Galanciak <jacek.galanciak@gmail.com>`. No tmux server
  running; load average falling from session 43's runs (0.27 over a
  minute at 10:08).
- `docs/pair-verdicts.txt`: SHA-256 `f4b1867956ba...`
  (`work/s44/listening-digests-start.txt`). The NAS clusters file:
  SHA-256 `d9c85c4586ad...` (`work/s44/clusters-digest-start.txt`).
- Experiments 0001-0205; this session numbers from 0206. Test set: 4 of
  5 evaluations used (none in this session).
- The start build `work/bin/s44-start/` is byte-identical to
  `s43-final` (cargo had nothing to rebuild).
- The pair count today (`verifier/pairs.rs`, session 30): on the
  verifier's alignment, the query's pairs (fan-out 2, anchors in the
  detection's span) against the record's own pairs in the aligned
  passage; E with the same hash, then N with each of the three
  components at most one step away; each pair once; the reference
  anchor within ±1 frame. Nothing follows a reference pair to its peaks:
  why a pair is lost is not measured.
- Session 30's reports and scripts (`work/s30/`) were on the Mac and are
  not on this machine; neither is 0055's scenario file
  (`work/s7/shared-material.json`). The recall excerpts and damaged
  renders of session 33 are in `work/recall/`, the first shared panel's
  renders in `work/shared/`.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; start build and both regressions (tmux `s44start`); brief, checklist, `CLAUDE.md` committed | 10:05-10:30 |
| 1 | pre-registration in this brief and experiment 0206, committed before any count | 10:30-10:45 |
| 2 | the split and the fan-out counts in `verifier/pairs.rs`, tested; `regress` identical | 10:45-11:25 |
| 3 | the runs: 262 tracks (`robust --only combined`), NAS scale (recall clean and damaged, other rips, both shared panels), tmux `s44runs` | 11:25-12:10 |
| 4 | experiments 0207 (the split) and 0208 (the fan-outs, if the registered share is reached) | 12:10-12:25 |
| 5-6 | review, notes, status, roadmap, final checks | 12:25-12:40 |

## Decisions

- **(10:08) Start build and checks** (`work/s44/start.sh`, tmux
  `s44start`): the start build into `work/bin/s44-start/`, then
  `regress session-32-linux` and `regress session-33-linux-nas`, each
  report compared with its baseline apart from `wall_seconds`
  (`work/s44/same.py`, session 43's).
- **(10:13) Start checks.** `regress session-32-linux` (1:08) and
  `regress session-33-linux-nas` (4:05) identical with
  `work/bin/s44-start/`, each report identical to its baseline's apart
  from `wall_seconds`.
- **(10:13) The pre-registration's times.** Its heading said 10:20-10:35;
  it was written at 10:09-10:12 and committed at 10:12 (`d685f25`),
  before the harness code and any count.
- **(10:17) The harness** (`verifier/pairs.rs`, `verifier.rs`): the split
  and the fan-out counts are part of `--verify`'s pair count, as session
  30's N is, with no flag of their own: `--verify` is already an opt-in
  diagnostic of the harness, the CLI binary is byte-identical to the
  start build's, and nothing outside `--verify` changes. The fan-out
  counts are made in the same runs as the split (to fit the time box)
  and are analysed only if the registered share is reached
  (`scripts/analysis/lost_pairs.py` prints them only with
  `--fan-outs`).
- **(10:19) Session 30's counts unchanged.** `--verify robust --only
  combined` at 262 tracks with the start build and with the harness
  build (`work/bin/s44-harness/`): the reports are identical apart from
  the new `split` and `fan_outs` fields and the search times
  (`work/s44/same_counts.py`). One look at the first detection's new
  fields (a control excerpt at 1,437 hits, its exact pairs all "paired,
  same hash"; first written here, wrongly, as a combined-damage excerpt)
  checked their shape; no set was tallied.
- **(10:19) The runs start** (`work/s44/runs.sh`, tmux `s44runs`,
  scratch work directory `work/s44/w`) while the harness build's
  regressions run (tmux `s44check`): results are deterministic, and no
  time from this session is reported as a speed.
- **(10:26) The registered share is reached** on R1c alone (T(c) needs
  only that report; `lost_pairs.py` run with R1c in place of the clean
  report too, split only): in T(c) (511 detections) the displaced pairs
  at ranks 3-8 are 4.5% of the lost pairs, under the 15%, but equal 89%
  of T(c)'s E, over the 25%. The fan-out counts are therefore analysed
  once every run has finished, against the registered criterion.
- **(10:29) Harness checks.** `cargo test -p gunfinger-eval` (290
  tests, 5 new for the split and fan-outs, one for the verifier's found
  peaks), clippy and formatting clean; `regress session-32-linux` (1:54)
  and `regress session-33-linux-nas` (8:25, both beside the runs)
  identical with `work/bin/s44-harness/`, every report identical to its
  baseline's apart from `wall_seconds`.
- **(11:00) The shared panels' roles** come from the full panel files
  (`docs/panels/shared-material-seed-2026.json`,
  `shared-material-2-seed-2026.json`), which hold the families
  `quantisation.py` reads; the development files are what the runs
  searched. R3 took 26:41, mostly waiting on reads of the NAS store's
  records for the verifier; no shrink was needed.
- **(11:25) Experiment 0207** (the split) written and committed before
  the fan-out counts were opened (`lost_pairs.py --fan-outs`).
- **(11:30) Experiment 0208** (the fan-outs): criterion (a) not met at any
  fan-out (median G/E 0.30 at fan-out 5, but 4.1% of T(c) under 240
  reach it, not 10%); (b) not met (related versions gain more than the
  played recording in every E band, pooled AUC 0.24-0.27); (c) met. No
  experiment on the rules follows. The per-search count uses the "played
  left out" index, as registered; with everything indexed it gives the
  same numbers (114 today) because the related and version detections
  in the shared reports are the same in both indexes (the review
  checked: only weak detections of other files differ).
- **(11:40) The review** (a read-only agent, 11:22-11:29) found the code
  correct and these points, all fixed: T(0) and W(0) mixed the control
  and combined queries of `robust --only combined` (which also runs the
  control); `lost_pairs.py` now keeps the combined queries only, and
  0207's T(0) row is 80 detections (0.875 endpoint gone, as T(c)), its
  fan-out counts in 0208 unchanged; 0207's sentence on the pairs that
  keep both peaks contradicted its data (now the shares among them);
  every expectation in 0207 is marked wrong; "chance agreements" marked
  as probable; 0208's CPU estimate (2%, 6%, 11%) and its AUCs said to be
  pooled over E 10-239, as in 0149; the per-search note above; the first
  detection's condition; the split test now covers ranks 5 and 9 and the
  fan-out test a near hash at an extra fan-out; `CLAUDE.md`'s line
  width. Two choices of the analysis not in 0206 are recorded here: W(0)
  counts detections with 10 hits or more (those the second pass
  measures), and S3's AUCs are pooled over the E bands 10-239 only, as
  in 0149 (in the band of 240 and more R's median gain is also above
  P's).

## Step 1: pre-registration (written at 10:09-10:12, committed before any count)

Everything below is fixed before the diagnostic is written or any count
is made. Anything computed later and not named here is exploratory and
decides nothing. Nothing here proposes a rule.

### Where it is counted

On the verifier's alignment for each fitted detection (`verifier.rs`,
unchanged: the line through the detection's first and last aligned
hits, each 10 s window moved by the mean of its densest 2-frame run of
the asset's hash hits; the query analysed at the detection's speed and
playback over its span plus 1 s either side), with session 30's pair
count unchanged (`verifier/pairs.rs`): reference pairs are the record's
own pairs (fan-out 2, as the index forms them) whose anchor lies in the
aligned passage; E and N as in 0149. Nothing is looked up in the index.
The split is counted at the alignment only (shift 0); the fan-out
counts also at the verifier's four chance shifts.

### The split, per reference pair

Each reference pair (anchor peak a, target peak b of the record) gets a
status from session 30's matching (exact, near or unmatched) and a kind
from its peaks. A reference peak is *found* where the verifier finds it:
a query peak whose place on the alignment (the line, moved by the
window's offset, the window taken at the reference peak's own frame,
the last window beyond the span) lies within ±1 frame and ±1 bin
(`TOLERANCE`). Every such query peak is a candidate for it.

1. **Endpoint gone**: a or b has no candidate. Counted apart as anchor
   gone, target gone, both gone.
2. Both found. For each candidate pair (qa for a, qb for b): the rank
   of qb among qa's targets in the query (`hash::targets`, the target
   zone, nearest in time first; rank 1 is the first target), or none
   when qb is not in qa's zone. The pair's kind is the best over the
   candidate pairs, in this order:
   - **paired**: some rank is 1 or 2, so the query forms the pair. Then
     the query pair's hash against the reference pair's: **same hash**,
     **near** (each component at most one step away, as N), or **far**
     (some component two steps or more). Near and far are the prompt's
     *changed quantised value*.
   - **displaced**: both in qa's zone, but qb's best rank is 3 or more:
     counted by rank as 3, 4-5, 6-8 and 9 or later (what fan-outs 3, 5
     and 8 reach).
   - **outside the zone**: no candidate qb lies in a candidate qa's zone
     (qb at or before qa's frame, more than 63 frames after it, or more
     than 63 bins away).
3. *Lost* means not counted in E: the reference pairs in the passage
   minus E. The split reports each kind for exact, near and unmatched
   pairs apart; the lost pairs are the near and unmatched ones. Exact
   pairs whose kind is not "paired, same hash" are reported as a check.
   Lost pairs of kind "paired, same hash" (the query forms the pair with
   the same hash but E did not take it: its anchor more than a frame
   off, or the query pair taken by another reference pair) are a
   residual, reported apart.

The target of a pair at the passage's end can lie up to 63 frames
beyond it, about 1 s, the edge of the query's analysis margin; such a
target may count as gone (reported, not corrected).

### The fan-outs

Fan-outs 3, 5 and 8, on the verifier's analysis only (the second
pass's stand-in: E is 0.98-1.00 of a true detection's hits, 0151); the
index, the record's pairs and the core stay at fan-out 2. For fan-out F:
the query's fan-out 2 pairs are matched first, exactly as for E; then
the extra pairs (each anchor's targets 3 to F, anchors in the span, in
anchor order) are matched with the same hash against the reference
pairs E left, with the same ±1 frame and nearest-first rule; each
reference pair counts once. **G(F)** is the number of extra
correspondences; E + G(F) is the exact count at fan-out F. No near
matching at extra fan-outs. Chance: G(F) at the four shifts.

### The populations

Build: this session's harness on master (`work/bin/s44-*`), default
matcher and rule, the NAS clusters in use, `--panels` a copy of
`work/s33/panels`, scratch work directory `work/s44/w` (the saved index
and session 33's recall renders hard-linked). Runs one at a time, under
tmux with logs:

- R1c `--verify recall --queries damaged`, R1C `--verify recall
  --queries clean` (702 queries each), R2 `--verify recall
  --other-rips-only --queries clean`, R3 and R4 `shared` on both shared
  panels' development halves (40,683 records); R0 `--verify robust
  --only combined` at 262 tracks (80 excerpts). 0055's scenario file is
  not on the Linux machine, so session 30's R5 is not run.
- Sets as 0151's (`quantisation.py`'s builders, fitted detections):
  T(c), T(C), T(a) (per observable query its strongest own-cluster
  detection), F (unrelated wrong candidates in R1c, R1C, R2, by
  `labels.py` with the NAS store's tags), R(r) (related wrong candidates
  there), P (the played recording on the shared panels), R (related
  recordings and versions there; confident ones also apart). At 262
  tracks: T(0) the excerpt's own recording under combined damage, and
  W(0) the other wrong detections that are not the blend partner,
  unlabelled.

### The share that opens the fan-out step

In T(c), pooled over its detections: the displaced pairs at ranks 3-8
(those fan-out 8 can reach) are at least **15% of the lost pairs**, or
equal at least **25% of T(c)'s E**. If neither holds, the fan-out
counts (made in the same runs, to fit the time box) are not analysed:
the experiment says what the lost pairs are and what that means for
step 3, and the G(F) medians appear on one line marked exploratory.

### The scores (only if the share is reached)

- S1: per set, the median and pooled G(F)/E, by E band (10-29, 30-59,
  60-119, 120-239, 240 and more), and G(F)'s chance level.
- S2: detections under 240 hits that reach 240 with hits + G(F) over 3
  windows or more: T(c), T(a), F, R(r), R not confident; per shared
  search with the played recording left out, a related file confident
  today and with hits + G(F).
- S3: separation at equal E: AUC of G(F)/E within the E bands, pooled,
  for P~R, T(c)~R(r) and T(c)~F.
- S4: CPU: the query's pairs at fan-out F over those at 2 (the second
  pass looks up each query pair once), with the second pass's share of
  a scan's CPU (0205: the first pass is 96-97%); an estimate, not a
  timing.

### The criterion for a later experiment on the rules

A fan-out is worth an experiment on the rules only if all hold, at the
smallest F that meets (a):

- (a) T(c)'s median G(F)/E is at least 0.25, and at least 10% of T(c)'s
  detections under 240 reach 240 with hits + G(F) over 3 windows;
- (b) related versions gain at most half as much as the played
  recording: in every E band where P and R have 10 detections or more,
  R's median G(F)/E is at most half of P's, and the pooled AUC of
  G(F)/E for P~R is at least 0.75;
- (c) no F detection reaches 240 with hits + G(F).

If R gains as much as P in some band (step 2's prediction), that
settles it for the fan-out: no experiment on the rules.

### Expected, written before any count

Endpoint gone is most of the lost pairs under damage (the verifier finds
a median 37% of the peaks, so most pairs lose an endpoint): 60-80%;
displaced 5-15%; changed value 10-25%. On clean audio displaced under
5%. If the share is reached: T(c)'s G(8)/E 0.1-0.4, and R gaining about
as much as P.

### Shrinks

R2 first, then R4. R1c, R1C, R3 and R0 do not shrink.
