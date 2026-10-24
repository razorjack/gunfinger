# Session 45 brief: the pitch-ride variant on data its threshold has not seen

Written by the agent for itself on 2026-10-24, so the plan survives
context compaction. Work autonomously and never stop to ask: where a
decision is needed, take the conservative option, write it down here and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-45-checklist.md` (progress).
3. `docs/brief-38.md` (criteria S1-S3, the runs and the decisions), the
   decision note `docs/research/2026-10-22-pitch-rides-decision.md`,
   experiments 0175-0185 and this session's (0209 and up); the roadmap's
   "Speed that wanders within a play" and "The printed tracklist"; ADR
   0006, 0008 and 0019.
4. `docs/session-notes.md` ("Session 37", "Session 38", "Session 39" and
   "Session 44"), `docs/status.md` (newest entries), the code of
   `crates/gunfinger-core/src/search.rs` (`strongest_per_moment`,
   `SecondOverlap`), `crates/gunfinger-eval/src/shared.rs`,
   `mixes/drops.rs`, and `scripts/analysis/variant_compare.py` and
   `tracklist_score.py`.

`docs/brief-2.md` to `docs/brief-44.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 45 of Gunfinger: the pitch-ride variant on data its threshold has not seen. Session 38 (experiments 0179-0181, `docs/research/2026-10-22-pitch-rides-decision.md`) found that the opt-in `--apart-in-speed 0.006` (`SecondOverlap::SpansAtSpeed`) keeps every side of a 1-3% pitch ride that today's second suppression drops, and changes no confident detection on the sweep, the development scans, the generated mixes or the first shared panel. But the first shared panel was also the threshold's development data. The decision note asks for one confirmation before an ADR: the second shared panel's development half (`docs/panels/shared-material-2-seed-2026-development.json`) at NAS scale, a default run and a variant run, with session 38's S2 and S3. This session runs that confirmation, adds the double-drop mixes session 39 built (two records playing at once, often at different speeds: where the variant could keep a partner's or a wrong file's extra segment), and leaves the decision ready for us. It is a measurement session: the default does not change; adoption is ours, with an ADR.

You run unattended: the user is away and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).

Time box: started about 21:15; aim to finish by about 23:30, because another session follows you tonight. If a step would run past that, scale it down (the panel first, fewer drop seeds), record the decision in the brief and continue.

Environment (sessions 33-44's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS store `~/.local/share/gunfinger/nas-dnb-peaks` (40,683 records), library `/mnt/atlas/Music/dnb` (read only), reached with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json` (SHA-256 `d9c85c4586ad...`, 284 clusters). Baselines `session-32-linux` and `session-33-linux-nas`. Build `work/bin/s44-final/` (master at `3c1ecd9`; since sessions 41 and 43 the search is about a third faster than in session 40: a NAS-scale development scan searches in about 26 s, `regress session-33-linux-nas` takes about 4 minutes). `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans, sweep reports of seeds other than 2026 (calibrate reads every sweep report there, session 40), and reports of opt-in variants must not sit in a reports directory during `regress`; run variants in a scratch work directory.
- Session 38's opt-in `--apart-in-speed` and session 37's `--overlap-by-chains` are variants, not defaults.
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is away for the evening and cannot listen to anything tonight. The pending listening (session 40's pack in `work/listening-s40/`, session 39's three questions, Star Trails and Into Music) stays for later; do not build new packs unless a step asks.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-38.md (its criteria S1-S3, the runs and the decisions) and docs/brief-44.md, docs/session-notes.md ("Session 37", "Session 38", "Session 39" and "Session 44"), docs/status.md (newest entries), the decision note `docs/research/2026-10-22-pitch-rides-decision.md`, experiments 0175-0181 and 0182-0185, the roadmap's "Speed that wanders within a play" and "The printed tracklist", ADR 0006, 0008 and 0019, and the code of `search.rs` (`strongest_per_moment`, `SecondOverlap`), `shared.rs`, `mixes/drops.rs` and the scripts `variant_compare.py` and `tracklist_score.py`.
2. Write docs/brief-45.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-45-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt in work/s45/listening-digests-start.txt; it must be the same at the end. Experiments number from 0209.
3. Build `work/bin/s45-start`; `regress session-32-linux` and `regress session-33-linux-nas` identical.

Steps:

1. Pre-registration, committed before any variant run on the new data: session 38's S2 and S3, word for word, applied to the second shared panel's development half at NAS scale (a default run and a variant run, compared query by query with `variant_compare.py shared`; wrong is a confident detection of role `other`, a wrong file's added segment counting as wrong) and to session 39's drop mixes (seeds 2026 and 2028, at 262 tracks and, if time allows, NAS scale; wrong is a confident detection of a file not playing then; also the printed tracklist under the variant scored with `tracklist_score.py` beside the default's: extra entries, entries lost). Say what result would confirm the variant, what would fail it, and that the second panel's validation half stays unseen.
2. The runs, default and variant, in scratch work directories so that no baseline or reports directory is overwritten, under tmux with logs: the panel first, then the drop mixes (render them again from the seeds if session 39 deleted its renders).
3. Experiments (at most 40 lines each, written before the next starts): the panel against S2 and S3, with every changed confident detection named; the drop mixes and their tracklists under the variant.
4. Update the decision note with what this session measured (a section "Session 45"), so that an ADR can be written from it. Do not write the ADR and do not change the default.
5. Ahead, only when 1-4 are done: the variant on session 35's `beatmatched` mixes (speed changes and two records at once), with the same criteria. Session 42's stereo-fault mixes are not needed.
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 45" section at the end of docs/session-notes.md with a handoff and the decision left for us; update docs/status.md, the checklist and the roadmap's "Speed that wanders within a play". Final checks: `scripts/check.sh` green, `regress session-32-linux` and `regress session-33-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: the panel's result against S2 and S3, the drops' result, whether the variant is confirmed, and anything left.

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
- Scratch data goes under work/s45/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 02:00. Start no new run after about 01:30; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

A note from the orchestrating thread, added at its request (21:14):

```text
Tonight the Linux machine is on a slow Wi-Fi link, not Ethernet. The NAS at `/mnt/atlas` is mounted but slow. The NAS peak store (`~/.local/share/gunfinger/nas-dnb-peaks`), the saved index and the corpus are local and unaffected. Keep reads of NAS audio to what a step needs:
- Render any query or mix from NAS files once, cache it under work/s45/, and reuse it for both the default and the variant run. Never decode the same NAS file twice.
- Prefer runs that need only stored peaks.
- Avoid commands that list or decode the whole NAS library. If `identify` with the NAS configuration lists the library, use the harness, or a store-only or scratch configuration instead.
- Expect NAS-touching steps to be slower than earlier sessions' times; do not read their wall time as a regression.
- If a run stalls on NAS I/O for more than a few minutes, scale it down (the drop mixes at 262 tracks only, corpus-sourced queries first), record the decision in the brief and continue.
- Timings that compare builds must not include NAS reads.
```

## State at the start of session 45

- 2026-10-24, 21:11. `master` at `3c1ecd9`, clean. Git identity
  `Jacek Galanciak <jacek.galanciak@gmail.com>`. No tmux server
  running; the machine had been up 6 minutes (load 0.4).
- `docs/pair-verdicts.txt`: SHA-256 `f4b1867956ba...`
  (`work/s45/listening-digests-start.txt`), equal to session 44's. The
  NAS clusters file: SHA-256 `d9c85c4586ad...`
  (`work/s45/clusters-digest-start.txt`).
- Experiments 0001-0208; this session numbers from 0209. Test set: 4 of
  5 evaluations used (none in this session).
- The second shared panel's development half (10 scenarios, 169
  queries, each searched against three indexes) was rendered by
  session 44 (`work/s44/w/shared/shared-material-2-seed-2026-development-f2be9a7f8ccdcf66/`,
  169 renders, each with its `.complete` marker; 177 MB). Neither
  overlap variant has run on this panel: its default report exists
  (session 44's R4) and no variant report.
- Session 39's drop renders are in place: `work/mixes/drops-seed-2026/`
  and `drops-seed-2028/` (16 mixes each, with their `.truth.json`
  plans), and its store-only copies `work/s39/store-{2026,2028}/` and
  NAS exclusion lists `work/s39/nas-exclude-{2026,2028}.txt`.
- `work/mixes/` holds 23 GB of earlier renders; 797 GB free.
- The NAS store holds no record whose tags name the NAS root's
  broadcast file (`EM_1999...`; grep of its `.tags` files).

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; start build and both regressions (tmux `s45start`); the variant's diagnostic CLI build (tmux `s45vbuild`); brief, checklist, `CLAUDE.md` committed | 21:11-21:40 |
| 1 | pre-registration in this brief, committed before any variant run on the new data | 21:40-21:50 |
| 2a | the panel: default run, then variant run, NAS scale, scratch work directory `work/s45/w` with session 44's renders hard-linked (tmux `s45panel`) | 21:50-22:35 |
| 3a | experiment 0209 (the panel) | 22:35-22:45 |
| 2b | the drops after 0209 is written: harness at 262 tracks (default, variant), the CLI's tracklists at 262 tracks and at NAS scale (default build, diagnostic build) (tmux `s45drops`) | 22:45-23:05 |
| 3b | experiment 0210 (the drops and their tracklists) | 23:05-23:15 |
| 4 | the decision note's "Session 45" | 23:15-23:25 |
| 5 | only if 1-4 are done and time is left | |
| 6-7 | review, notes, status, roadmap, final checks | 23:25-23:50 |

## Decisions

- **(21:16) Start checks** (`work/s45/start.sh`, tmux `s45start`): the
  start build into `work/bin/s45-start/` and both regressions, each
  report compared with its baseline apart from `wall_seconds`
  (`work/s45/same.py`, session 44's).
- **(21:18) The printed tracklist under the variant needs a diagnostic
  build.** `--apart-in-speed` is a harness option; the CLI, which groups
  plays, marks shared material (ADR 0019) and prints the tracklist, has
  no variant, and the harness's `mixes` report holds detections, not the
  CLI's plays. Adding an option to the CLI would change the product, so
  the tracklists under the variant come from a diagnostic build that is
  never committed: master's tree (`git archive 3c1ecd9`) in
  `work/s45/variant-src/` with one line added, `second_overlap:
  SecondOverlap::SpansAtSpeed(0.006)` in `Matcher::Fitted`'s options
  (`work/s45/variant-cli.diff`), built into
  `work/bin/s45-variant-cli/` (tmux `s45vbuild`). As session 37's
  diagnostic build (0175), it is a measuring instrument only. Its
  harness without options must give the same drops report as master's
  harness with `--apart-in-speed 0.006` (checked before its tracklists
  are read).
- **(21:18) No NAS audio for the panel.** `shared` renders a query only
  when its `.complete` marker is missing (`shared::render`), and the
  render directory's key follows the plan, so session 44's renders are
  hard-linked into the scratch work directory and both runs read only
  them, the saved index and the NAS store (local). Neither run is
  compared in wall time with an earlier session's.
- **(21:18) The drops at NAS scale without listing the NAS.** The
  harness's `mixes` has no `--other-peaks-dir`, so NAS scale for the
  drops is the CLI only. Session 39 ran `identify` with the NAS
  configuration, which lists `/mnt/atlas/Music/dnb`; tonight it runs on
  the NAS store's own records instead (`--peaks-dir` the NAS store,
  `--store-only`, `--exclude-from` session 39's lists, which also keeps
  the saved index unread and unwritten). That index can differ from
  session 39's by records the library would not list (ignored or gone
  files); default and variant use the same one, and session 39's
  NAS-scale numbers are not compared with tonight's. The renders are
  local; `mixes` probes the lengths of the first shared panel's pair
  files (`shared_pairs`, FFprobe, some on the NAS), a few header reads.
