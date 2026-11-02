# Session 75 brief: the records of the files the three newest NAS ignore entries leave out pruned, and new NAS-scale baselines

Written on 2026-11-02, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-75-checklist.md` (progress).
3. `docs/brief-64.md` (the last prune: its procedure), ADR 0009 and
   0022, experiments 0231, 0242 and 0254, and the newest four entries of
   `docs/status.md` (among them "the listening of the 2026-10-31 packs"
   and "the last answer, and the user's decisions and views").
4. `docs/session-notes.md` ("Session 52", "Session 64", "Session 74");
   this session's experiment (0278).

`docs/brief-2.md` to `docs/brief-74.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 75 of Gunfinger: prune the records of the files the three newest NAS ignore entries leave out, and record new NAS-scale baselines. After the listening of 2026-10-31 (docs/status.md, "2026-10-31: the listening of the 2026-10-31 packs"), at the user's request three entries were added to the NAS `.gunfingerignore` (lines 77-91): `/sety/` (every file there is a DJ set), the folder of the Virus on Kiss FM show of 3 May 2000 (`/__full_scene/dnb_scene - 2000 - Part 1/2000-05/virus_on_kiss_fm-3rd_may_2000-sour/`) and the folder of the Lowdown Vol 6 track fix (`/__full_scene/1999/1999-09/1999-09-28/knowledge-the_lowdown_vol_6-track_fix-1999-sour/`). `prune` without `--yes` lists 107 files they leave out (`work/listening-2026-10-31/prune-dry-run.txt`): 8 peak records (both Lowdown Vol 6 fix tracks, part 1 of the Kiss FM show and five files of `sety/`), 99 skip notes and 8 tag notes. The NAS store still holds those records, so every `--other-peaks-dir` run and the saved index still search them. The user approved the prune on 2026-10-31 (docs/status.md, "2026-10-31: the last answer, and the user's decisions and views"). This session prunes them as sessions 52 and 64 did, explains every change against the old baseline, and records new baselines that the next sessions tonight compare against. The product's code does not change.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).

Time box: started about 08:38; aim to finish within about 1.5 hours, and by 10:40 at the latest (the user's soft deadline is 11:00); it is the night's last session. If a step would run past that, scale it down, record the decision in the brief and continue.

Environment (sessions 33-74's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,674 records, revision `7210fc479da58376` (session 64 pruned four ignored uploads). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-54-linux` (262 tracks) and `session-64-linux-nas` (NAS scale; compare with it, not `session-54-linux-nas`).
- Build `work/bin/s74-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Since `45f92d5` (2026-10-28) its calibrate reads only the reports its baseline holds; still run variants and other seeds in a scratch work directory, never in the reports directory `regress` compares.
- The user restructured the code on 2026-10-28 without meaning to change behaviour (commits `b5a84e3` to `f94604a`: harness subcommands in modules, named results, `Search` passed through the core's search, enums for closed sets). Session 60 confirmed both regressions identical on a build of master after it.
- Since sessions 53-56: `SpansAtSpeed(0.006)` is the default second suppression (ADR 0021); same-title detections (`docs/same-titles.txt`) are scored right (ADR 0022); `identify` prints gaps as "ID - ID" lines with an estimate of their tracks (ADR 0023, 0024). The minimum span stays undecided. A change to an existing decision needs a new ADR (the next number is in `docs/adr/`).
- The research note `docs/research/2026-10-29-matching-and-archive-proposals.md` (2026-10-29) records six unmeasured proposals and its evaluation boundaries; any session working on one follows its "Evaluation and adoption boundaries".
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen. Every listening question asked so far has an answer (docs/status.md, the entries of 2026-10-31 and 2026-11-02), apart from the Planet Dust (DC Breaks Remix) cut (unsure). The user's decisions on a quiet track under an identified one are in docs/status.md, "2026-11-02: the user's answers on a quiet track under an identified one". When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
- The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.
- The repository root holds an untracked `yafc20261031.log` that is not this project's work: leave it alone, never commit or delete it; "`git status` clean" means clean apart from it.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-64.md (the last prune: follow its procedure) and the newest brief, docs/brief-nas.md, docs/session-notes.md ("Session 52", "Session 64" and the newest session), docs/status.md (the newest four entries), ADR 0009 and 0022, experiments 0231 and 0254 (what the last two prunes changed), and the code of the CLI's `prune` and `doctor` and of the harness's handling of panel entries that name a pruned file (session 52's fix).
2. Write docs/brief-75.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-75-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s75/listening-digests-start.txt. Experiments number from 0278.
3. Build `work/bin/s75-start`; `regress session-54-linux` and `regress session-64-linux-nas` identical before anything is pruned.

Steps:

1. Before pruning: `gunfinger --config ~/.config/gunfinger/nas-dnb.toml doctor` and `prune` (without `--yes`) must list exactly the 107 files of the dry run above (8 peak records, 99 skip notes, 8 tag notes) and nothing else; if they list anything else, stop this step, record it, and prune nothing. Save both outputs under work/s75/. Find where the 8 recorded files appear today: the NAS clusters file, session 33's panels (`work/s33/panels`), the committed panels in `docs/panels/`, the saved reports of the baseline `session-64-linux-nas`, and the corpus store (`work/peaks`; it must not hold them, and is never written).
2. Prune: `gunfinger --config ~/.config/gunfinger/nas-dnb.toml prune --yes`, then `doctor` again: the store at 40,666 records with no ignored records left; record its revision before and after. Then `clusters --from-peaks --reuse-pairs` as sessions 52 and 64 ran it (this re-applies the verdicts and the ignore file to the saved pairs; it is not a rebuild): record the clusters file's SHA-256 and what changed. No full clusters rebuild.
3. The standard evaluation at NAS scale with the pruned store (the saved index is built again on the first search): the runs `regress session-64-linux-nas` compares. Compare with the old baseline and explain every difference by the 8 pruned records (their plays, their queries, their places in a cluster or panel) or by the set-aside lists moving (experiment 0242's mechanism: the fullest 1% of the index is set aside per run, and a prune can move a list across that cut); a difference you can trace to neither is a finding to stop on and report, not to baseline.
4. Record `baseline session-75-linux-nas` with the final build. `regress session-54-linux` stays identical (the corpus side does not change). One experiment (at most 40 lines): what the prune changed in the NAS-scale reports, calibrate and the clusters, against `session-64-linux-nas`.
5. Update the places that name the NAS baseline, the store's record count or the clusters file's digest for the next sessions (docs/status.md, the handoff; older experiments and briefs stay as written). Note in the roadmap's "Library ignore file" that DJ sets indexed as tracks (`sety/`, the Kiss FM show) are now left out, while uploads cut from sets stay in the library by the user's decision (docs/status.md, 2026-10-31).
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 75" section at the end of docs/session-notes.md with a handoff (the new baseline names, the store's record count and revision, the clusters file's digest); update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-54-linux` and `regress session-75-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, `git status` clean. Commit. Your final message: what was pruned, what changed in the reports and why, the new baseline names and digests (store revision, clusters SHA-256), and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's code and behaviour do not change: no threshold, parameter, rule, default or output change. Harness fixes are tested and keep `regress` identical apart from the prune's explained changes.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. This session runs `prune --yes` once with the NAS configuration (step 2, after step 1's check) and `clusters --from-peaks --reuse-pairs`; it never runs `index` with the NAS configuration and does not rebuild the NAS clusters. The prune deletes the store's record and notes of files under `sety/`, among them experiment 0065's `sety/vertical.mp3` if the store holds anything of it: deleting a record is not running on that mix, and nothing else touches it. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
- Scratch data goes under work/s75/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00 on 2026-11-02. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 75

- 2026-11-02, 08:35 by the Linux machine's clock (CET). The time box is
  about 1.5 hours: aim to finish by about 10:10, by 10:40 at the latest.
  The soft deadline is 11:00; no new run starts after about 10:20.
- `master` at `793d413`, clean (apart from the untracked
  `yafc20261031.log`, which is not this project's work). Git identity
  `Jacek Galanciak <jacek.galanciak@gmail.com>`.
- Digests (`work/s75/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `c4d3da2e7e37...`, `work/audition/answers.jsonl` `3e99ebca589b...`.
- Experiments 0001-0277; this session numbers from 0278. ADRs
  0001-0025. Test set: 4 of 5 evaluations used (none in this session).
- The NAS store: 40,674 records, revision `7210fc479da58376` (session
  64); the NAS `.gunfingerignore` gained three entries on 2026-10-31
  (lines 77-91 with their comments; the file before in
  `work/listening-2026-10-31/gunfingerignore.before`).
- Builds: `work/bin/s74-final/` (master; the product's code last changed
  in session 72). Baselines `session-54-linux` and `session-64-linux-nas`.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build and both regressions (tmux `s75start`) | 08:35-08:45 |
| 1 | `doctor` and `prune` listings (tmux `s75list`, read only, beside the start regressions); an independent count of the store's records under the three entries; where the 8 recorded files appear | 08:36-08:50 |
| 2 | `prune --yes`, `doctor`, revision; `clusters --from-peaks --reuse-pairs` | 08:50-08:55 |
| 3 | `regress session-64-linux-nas` on the pruned store; every difference traced; the sweep under the old lists (`sweep --set-aside-like`) | 08:55-09:20 |
| 4 | `baseline session-75-linux-nas`; `regress session-54-linux`; experiment 0278 | 09:20-09:40 |
| 5 | the places that name the baseline, the record count, the digest; the roadmap's "Library ignore file" | 09:40-09:50 |
| 6-7 | review, notes, status, checklist, final checks | 09:50-10:30 |

## Decisions

- **(08:36) Start build and regressions** (`work/s75/start.sh`, tmux
  `s75start`, logs in `work/s75/logs/`; `run.sh` and `same.py` are
  session 64's with the paths changed). Cargo had nothing to rebuild:
  the start build is byte-identical to `work/bin/s74-final/`. The
  scripts were started before this brief was committed, since they only
  read the store and write the reports `regress` compares.
- **(08:36) The pre-prune saved index kept** as a hard link,
  `work/s75/indexes/pre-prune.index` (40,674 records, only read), for
  `sweep --set-aside-like` in step 3, as session 64 used session 63's
  links. The harness writes a new index file when it builds one again,
  so a hard link keeps the old one (session 64's links survived its
  rebuild).
- **(08:36) Step 1's listings ran beside the start regressions**
  (`work/s75/step1-list.sh`, tmux `s75list`, with `work/bin/s74-final`'s
  `gunfinger`), as in sessions 52 and 64: `doctor` and `prune` without
  `--yes` only read the library and the store.
- **(08:40) Step 1, the listings.** `doctor` (0:06): 51 patterns leave
  out 734 audio files; line 80 (`/sety/`) leaves out 101, line 85 (the
  Kiss FM show) 4 and line 91 (the Lowdown Vol 6 track fix) 2; 40,666
  current records; one warning, 8 peak records, 99 skip notes and 8 tag
  notes of 107 files that the ignore file leaves out; no record of a
  gone file, no temporary file. `prune` without `--yes` (0:04) lists
  115 files in one group, "8 peak records, 99 skip notes and 8 tag notes
  of 107 files that .gunfingerignore leaves out", no group of gone files
  or temporary files; its first 20 lines equal the dry run of 2026-10-31
  (`work/listening-2026-10-31/prune-dry-run.txt`). `prune` prints only
  20 paths per group, so the rest were checked apart:
  `work/s75/ignored_in_store.py` reads the source path of every record
  and note in the store and finds 115 under the three entries (8 peak
  records, 99 skip notes, 8 tag notes, 107 files; listed in
  `work/s75/ignored-in-store-before.tsv`), the same counts as `prune`'s
  only group. So `prune --yes` deletes exactly those, and the prune goes
  ahead. Outputs `work/s75/logs/doctor-before.log` and `prune-list.log`;
  the store before it: 40,674 records, revision `7210fc479da58376`
  (`work/s75/store-revision-before.txt`).

  The 8 peak records: the Lowdown Vol 6 fix's Plastic (Future Cut and
  Marcus Intalex) and Outa Orbit (General Midi vs Lurch), part 1 of the
  Kiss FM show, and `sety/`'s `01-Mr._Orange_-_Classical_Mix.mp3`,
  `BASE CLUB - ( unity ).mp3`, `Base Club - In to the sky.mp3`,
  `Weekenders_1_-_Nocny_Trans.mp3` and `base_club_-__all_4_one.mp3`. Experiment 0065's `sety/vertical.mp3` has a
  skip note only (too long for a record), which the prune deletes.

  Where they appear (`work/s75/where.py`, which matches any path under
  the three entries; output `work/s75/where-before.tsv`):
  - the NAS clusters file and the library map: none (no cluster or pair
    entry);
  - session 33's panels, `docs/panels/`, `docs/same-titles.txt` and
    `docs/pair-verdicts.txt`: none;
  - the baseline `session-64-linux-nas`: six sub-threshold candidates in
    `sweep-seed-2026.json`, 2 windows each (queries 345-347, SKC -
    Recharger (Black Sun Empire Remix) at -1%, 0% and +1%: the Plastic,
    8, 6 and 7 hits; queries 407, 408 and 410, Ed Rush & Optical -
    Sicknote at -3%, -1% and +1%: Weekenders 1 - Nocny Trans, 7, 7 and 6
    hits); and one sub-threshold detection of the Plastic in each of the
    three Stakka scans (51:33-51:44, 6 hits, 2 windows);
  - the corpus store (`work/peaks`, read only, the same script): none.
