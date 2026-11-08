# Session 88 brief: a set-aside threshold by posting count

Written on 2026-11-08, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it), `AGENTS.md` and
   `docs/session-rules.md`.
2. This file, and `docs/session-88-checklist.md` (progress).
3. ADR 0011 and 0014; experiments 0242, 0254, 0263, 0278; the external
   review (`docs/research/2026-11-05-external-review.md`).
4. The code: `crates/gunfinger-core/src/index.rs` (`SetAside`,
   `skipping_fullest`, `skipping_hashes`, `without_assets`),
   `crates/gunfinger-eval/src/padding.rs` (`whole_index`),
   `crates/gunfinger-eval/src/matching.rs`, `fullest.rs`.

## The prompt (verbatim)

> Start session 88 of Gunfinger: a prototype of a set-aside threshold by posting count, so that a small library change moves nothing. ADR 0014 sets aside the fullest 1% of the searched index's posting lists. The cut is a share, so every library change moves some lists across it: experiment 0242 (session 59) explained that this moves fitted speeds and weak candidates of untouched files, and sessions 64 and 75 (0254, 0278) traced the same mechanism after prunes of 4 and 8 files; 0263's registered bar failed on it too; runs that leave files out have cuts of their own. The external review of 2026-11-05 (`docs/research/2026-11-05-external-review.md`) proposes a posting-count threshold recorded with the saved index and re-derived only when the library grows by more than a stated share. This session builds that as an opt-in variant and measures what it moves. It is a measurement session: ADR 0014's rule stays the default; a change would need a new ADR, which is the user's decision.
>
> You run unattended: the user cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.
>
> The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).
>
> Time box: started about 08:28 on 2026-11-08; aim to finish within about 1.5 hours, and by 10:30 at the latest (the user's deadline is 11:00; this is the night's last session). If a step would run past that, scale it down (the replay of the two prunes at NAS scale first), record the decision in the brief and continue.
>
> Environment (sessions 33-87's handoffs in docs/session-notes.md; check each):
> - Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
> - Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
> - NAS store: 40,666 records, revision `0bd3f0c23fabc698` (session 75 pruned the records of DJ sets and a mixed CD's track fix). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-78-linux` (262 tracks) and `session-78-linux-nas` (NAS scale), both with the 2003 and Nocny scans since session 78; `regress --strict` fails on any difference; a session that adds baselines names them in its handoff, and the next prompt says which to use.
> - Build `work/bin/s87-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
> - `gunfinger-eval regress` prints what changed and returns success either way: read its output. Since `45f92d5` (2026-10-28) its calibrate reads only the reports its baseline holds; still run variants and other seeds in a scratch work directory, never in the reports directory `regress` compares.
> - The user restructured the code on 2026-10-28 without meaning to change behaviour (commits `b5a84e3` to `f94604a`: harness subcommands in modules, named results, `Search` passed through the core's search, enums for closed sets). Session 60 confirmed both regressions identical on a build of master after it.
> - Since sessions 53-56: `SpansAtSpeed(0.006)` is the default second suppression (ADR 0021); same-title detections (`docs/same-titles.txt`) are scored right (ADR 0022); `identify` prints gaps as "ID - ID" lines with an estimate of their tracks (ADR 0023, 0024). The minimum span stays undecided. A change to an existing decision needs a new ADR (the next number is in `docs/adr/`).
> - Since 2026-11-05: `docs/session-rules.md` holds the standing rules (the older briefs are history), `docs/decisions-queue.md` the decisions waiting for the user, and the roadmap begins with "The active queue" (`docs/research/2026-11-05-external-review.md` explains why). A session that leaves a decision for us adds it to the decisions queue. The six proposals of `docs/research/2026-10-29-matching-and-archive-proposals.md` have all been measured; a session working on one follows its "Evaluation and adoption boundaries".
> - `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
> - Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
> - Tonight (2026-11-07 to 11:00 on 2026-11-08) sessions run one after another, unattended; the user cannot decide or listen. Every listening question asked so far has an answer (docs/status.md, the entries of 2026-10-31 and 2026-11-02), apart from the Planet Dust (DC Breaks Remix) cut (unsure). The user's decisions on a quiet track under an identified one are in docs/status.md, "2026-11-02: the user's answers on a quiet track under an identified one". When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
> - The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.
> - The repository root holds an untracked `yafc20261031.log` that is not this project's work: leave it alone, never commit or delete it; "`git status` clean" means clean apart from it.
>
> First:
> 1. Read CLAUDE.md and follow its reading list, notes "Session 59", "Session 64", "Session 75" and the newest session, ADR 0011 and 0014, experiments 0242, 0254, 0263 and 0278, and the code of the set-aside lists, the saved index's header, `sweep --set-aside-like`, and how leave-out runs build their indexes.
> 2. Write docs/brief-88.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-88-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s88/listening-digests-start.txt. Experiments number from 0298.
> 3. Build `work/bin/s88-start`; the start regressions follow docs/session-rules.md.
>
> Steps:
>
> 1. Pre-registration, committed before the scored runs: the variant (the threshold, in postings per list, derived once from today's NAS index so that it sets aside the same lists as today's 1%; stored with the saved index; applied unchanged to leave-out runs and to an index after a prune; re-derived only past a stated growth); the replay (the prunes of sessions 64 and 75, re-created by leaving the pruned files out of an in-memory or scratch index, never by writing a store; the sweep and the Stakka scans at NAS scale; the leave-out scans); the scores (queries with identical detections against the pre-prune index under the variant, against today's rule; any confident change; what a growth of 5% and 20% would do, simulated with duplicate padding); and what would justify an ADR superseding part of ADR 0014.
> 2. The variant, opt-in, tested (the threshold derived and stored, the same lists at today's index); with it off, every output and both regressions byte-identical.
> 3. The runs at NAS scale (262 tracks too), under tmux with logs where long.
> 4. Experiment (at most 40 lines): what a fixed threshold moves after the two prunes and under growth.
> 5. Update the roadmap's item and its active queue item 11, docs/session-rules.md only if the result changes how prunes are checked (it does not change the default), and docs/decisions-queue.md with a row if an ADR is proposed.
> 6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
> 7. Wrap-up: a "Session 88" section at the end of docs/session-notes.md with a handoff; update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-78-linux` and `regress session-78-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: what the fixed threshold moves after a prune, under growth, and anything left.
>
> Rules for this session:
> - `docs/session-rules.md` binds as well; where it and this list differ, this list wins for this session.
> - No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
> - The product's behaviour does not change: no threshold, parameter, rule, default or output change; `identify --under` stays off by default (the default is ours, in ADR 0025). Harness changes and variants are tested, opt-in where they are variants, and keep `regress` identical.
> - Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
> - The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
> - The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
> - Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
> - AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
> - Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
> - Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
> - Scratch data goes under work/s88/; delete renders you no longer need.
> - Prose: no em dashes (spaced en dashes in prose, ASCII in code).
> - Commit after each step that changes tracked files.
> - Time: the user's soft deadline is 11:00 on 2026-11-08. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
## State at the start

- Master at `f37bf60`, clean (apart from `yafc20261031.log`).
- `work/bin/s88-start/` is byte-identical to `s87-final` (`gunfinger`
  `cf8393e36bad...`, `gunfinger-eval` `8577f809d8a1...`), and the store
  revisions are unchanged (corpus `526a3b978eb3e411`, NAS
  `0bd3f0c23fabc698`, from a 10 s `identify` of the Stakka mix with each
  configuration, `work/s88/rev-*.json`). By `docs/session-rules.md` the
  start regressions are skipped; the final ones run.
- Digests at the start (`work/s88/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, NAS clusters
  `c4d3da2e7e37...`, answer log `3e99ebca589b...`.
- Baselines `session-78-linux` and `session-78-linux-nas`. Builds
  `s87-final` and `s86-final`.
- The pre-prune saved indexes are kept as hard links: the index before
  session 64's prune (40,678 NAS records, 40,700 assets with the corpus's
  22 not stood for) is `work/s63/wd/indexes/library-nas-dnb-peaks.index`
  and that before session 75's (40,674 records, 40,696 assets) is
  `work/s75/indexes/pre-prune.index`. Today's is
  `work/indexes/library-nas-dnb-peaks.index` (40,666 records, 40,688
  assets).

## The session's design and registration (2026-11-08, 08:40)

Registered before any scored run. The product does not change.

**The variant.** Today's rule (ADR 0014, `Index::skipping_fullest` with
`SKIPPED_SHARE` 0.01) sets aside the fullest 1% of the non-empty lists of
each index a run searches: a share, re-cut per run, per leave-out. The
variant is a fixed number T: a list is set aside when it holds more than
T postings. T is derived once from today's whole NAS index as the longest
list today's 1% cut keeps, so that at today's index the variant sets aside
exactly today's lists (test). It is written to a sidecar file beside the
saved index, `<work>/indexes/library-nas-dnb-peaks.set-aside.json`
(`postings_per_list`, the posting count and asset count it was derived
from, the share and the growth limit), because a new field in the saved
index's header would change the file format of ADR 0011. T is applied
unchanged to every index a run searches: the whole index, an index less
a run's held-out files, and an index after a prune. T is re-derived only
when the whole index's posting count has grown by more than 10% since the
derivation (the growth limit, stated here in advance; the file's number
stays below that). Harness only: `gunfinger-eval set-aside-threshold`
derives and writes the sidecar, and `--set-aside-from FILE` applies it,
with reports under `variant-set-aside-above-T`. The default path is
unchanged, and the core does not change: the harness computes the hashes
and calls `Index::skipping_hashes`. Rule **R** is today's, **V** the
variant.

**The replay** (never a written store). The prunes of sessions 64 (4
files) and 75 (8 files) are re-created in memory from the pre-prune saved
index: `--replay-index A` loads `work/s63/wd/.../library-nas-dnb-peaks.index`
by its own provenance and `--replay-leave-out FILE` leaves out the named
files by path (the 4, or 4 + 8), giving three states searched with the
current build: **A** (before prune 64, 40,678 NAS records), **B** (A less
the 4, the state before prune 75) and **D** (A less the 12, today's
index). Two checks that the replay is the history: B equals
`work/s75/indexes/pre-prune.index` and D equals
`work/indexes/library-nas-dnb-peaks.index`, posting for posting (assets
and list lengths; printed as digests by `set-aside-threshold`). For each
state and each rule, the standard evaluation at NAS scale as `regress`
runs it (the sweep of 720 queries, the Stakka scan and its leave-outs of
3 and 11, the 2003 and Nocny scans), in a scratch work directory per arm.
At 262 tracks the same with a threshold derived from the corpus index
(`work/s88/cut-262.json`) and a prune simulated by leaving out 12 library
files drawn with a fixed seed, none of them referenced by a development
set's manifest.

**The scores.** Reports are compared with `work/s88/diff_arms.py`,
detection by detection, the left-out files' own detections set aside.
- S1, prune stability: for each prune (A to B, B to D), per rule, the
  number of sweep queries (of 720) whose detections equal the pre-prune
  arm's, and the same for each scan's confident detections and all its
  candidates. R is measured with the same build, so that 0254's 707 of
  720 and 0278's 689 of 720, from other builds, are repeated rather than
  assumed.
- S2, any confident change: a confident detection's file or level
  changed, gained or lost, at either rule, per prune (a fitted-speed or
  hits change of a confident detection is counted apart from a changed
  answer).
- S3, the leave-out scans and the sweep's own held-out files: T is the
  same number at every index, so they carry no cut of their own; the
  one-time difference between V and R at the same index is counted, per
  report, by queries and detections.
- S4, growth: exact copies of drawn records added with `sweep
  --exact-copies 1 --copied LIST` at 5% and at 20% of the whole index's
  assets (`padding-copy/1/` assets, seeded draw; about the same share of
  postings), sweep only (`--exact-copies` is a sweep option; scans have
  no such padding). Arms at each size: R, V with T unchanged, and at 20%
  also V re-derived (T read from the grown index, the 10% rule). Scores:
  the share of lists V sets aside against 1%, and the sweep queries whose
  detections differ between V and R at the same grown index, with any
  confident change named. Exact copies add postings to existing lists and
  no new hash; real growth also adds new hashes, which this simulation
  cannot show.

**What would justify an ADR superseding part of ADR 0014** (a proposal
only; defaults are the user's). All of:
1. At today's index V sets aside exactly the lists R does (test, and at
   NAS scale).
2. Over the two prune replays at NAS scale V changes at most half as many
   of the 1,440 sweep query comparisons as R does, with no confident
   change at V and none of a kind R does not show; every V change is
   traced to a list that lost postings of a pruned file and crossed T
   (the other prune mechanism remains, by design).
3. On the leave-out scans and the sweep's held-out index, V's one-time
   difference from R at the same index changes no confident answer of
   the development reports.
4. At 5% growth with T unchanged, V sets aside at most 1.5% of the lists
   and changes no confident answer against R on the grown sweep; at 20%,
   T re-derived, no confident answer differs from R on the grown sweep.

If any fails, it is reported as failed and no ADR is proposed for this
variant; a revised candidate would get a new registration and fresh
data. Whatever the result, ADR 0014's rule stays the default.

## Decisions

Conservative choices, as they are taken:

- 08:40 The prunes are replayed from the pre-prune saved index that
  session 63 left (40,678 records). That file also holds session 75's
  8 files' records (`sety/` DJ sets, a Kiss FM show, a Lowdown Vol 6 fix)
  as ordinary reference records; no scan reads them, and nothing is
  decoded or searched as a query from them. The queries stay the three
  development mixes and the sweep's excerpts; 0065's listed candidate
  mixes are not run.
- 08:42 The threshold from today's NAS index is T = 9,932 postings
  (`work/indexes/library-nas-dnb-peaks.set-aside.json`, 10,865 of
  1,086,456 lists, 1.000%). The replay is faithful: the index before
  session 64's prune less the 12 files has the same lists as today's, and
  less the 4 the same as the index before session 75's prune (digests in
  `work/s88/logs/replay-*.log`; A's 1% cut would be 9,938, B's 9,938,
  D's 9,932).
- 08:43 For the 262-track replay, 12 corpus files named by no
  development set's manifest and by no sweep panel, drawn with seed 88
  (`work/s88/prune-262.txt`).
- 09:35 The two growth threshold runs failed on a missing `library-map.json`
  in the scratch directory (the queue created it later); they and the
  20% re-derived arm are rerun in `work/s88/queue2.sh`. Off-regressions
  (`off-262`, `off-nas`, `--strict`) identical with `s88-run`.
