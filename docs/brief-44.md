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
