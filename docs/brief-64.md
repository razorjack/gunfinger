# Session 64 brief: the four ignored NAS uploads pruned, new NAS-scale baselines, and the evening's answers carried

Written on 2026-10-30, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-64-checklist.md` (progress).
3. `docs/brief-52.md` (the last prune: its procedure), ADR 0009 and
   0022, experiment 0231, and the newest four entries of
   `docs/status.md` (among them "the listening of the 2026-10-30 pack").
4. `docs/session-notes.md` ("Session 52", "Session 58", "Session 63");
   this session's experiment (0254).

`docs/brief-2.md` to `docs/brief-63.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 64 of Gunfinger: prune the four files the NAS ignore file now leaves out, record new NAS-scale baselines, and carry the evening's listening answers into the documents that depend on them. After session 58's pack (answered 2026-10-30; docs/status.md, "2026-10-30: the listening of session 58's pack"), the user judged four uploads to hold more than one recording, and at the user's request four patterns were added to the NAS `.gunfingerignore` (docs/status.md, "2026-10-30: four uploads ignored on the NAS, and a listening pack"): `__youtube_archivists/i-witness-dnb/Bad Company - Spider (Optiv & BTK Remix) [CUT].m4a`, `__youtube_archivists/i-witness-dnb/Break - Wine [CUT].m4a`, `__youtube_archivists/i-witness-dnb/500 Mills (Counterstrike) - Vaccine.m4a` and `__youtube_archivists/3zb-gangstep-dnb/OMR - Bad Vibes.opus`. The NAS store still holds their peak records, so every `--other-peaks-dir` run and the saved index still search them. The user approved pruning them on 2026-10-30. This session prunes them as session 52 pruned five files, rebuilds what depends on the store, explains every change against the old baseline, and records new baselines that the next sessions tonight compare against. The product's code does not change.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).

Time box: started about 21:05; aim to finish within about 1.5 hours, because other sessions follow you tonight. If a step would run past that, scale it down, record the decision in the brief and continue.

Environment (sessions 33-63's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,678 records, revision `1fc6fe50293ad330`. NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `ce293fe6165e...`, 284 clusters. Baselines `session-54-linux` (262 tracks) and `session-54-linux-nas`.
- Build `work/bin/s63-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Since `45f92d5` (2026-10-28) its calibrate reads only the reports its baseline holds; still run variants and other seeds in a scratch work directory, never in the reports directory `regress` compares.
- The user restructured the code on 2026-10-28 without meaning to change behaviour (commits `b5a84e3` to `f94604a`: harness subcommands in modules, named results, `Search` passed through the core's search, enums for closed sets). Session 60 confirmed both regressions identical on a build of master after it.
- Since sessions 53-56: `SpansAtSpeed(0.006)` is the default second suppression (ADR 0021); same-title detections (`docs/same-titles.txt`) are scored right (ADR 0022); `identify` prints gaps as "ID - ID" lines with an estimate of their tracks (ADR 0023, 0024). The minimum span stays undecided. A change to an existing decision needs a new ADR (the next number is in `docs/adr/`).
- The research note `docs/research/2026-10-29-matching-and-archive-proposals.md` (2026-10-29) records six unmeasured proposals and its evaluation boundaries; any session working on one follows its "Evaluation and adoption boundaries".
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen. On the evening of 2026-10-30 the user answered 8 of 12 questions (docs/status.md, "the listening of the 2026-10-30 pack"): the four same-title candidates (Cool Rock, Tower Bass twice, Stalker) are different titles; Vintage Dread 2000 disc 1 is separate tracks; Advent, Keltech - Axis and Plasmic - The Weapon have an inverted channel, We - Hello World is unsure. The pitch rides (Star Trails, Into Music) and the Planet Dust cut stay unanswered. When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
- The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-52.md (the last prune: follow its procedure) and the newest brief, docs/brief-nas.md, docs/session-notes.md ("Session 52", "Session 58" and the newest session), docs/status.md (the newest four entries), ADR 0009 and 0022, experiment 0231 (what session 52's prune changed), and the code of the CLI's `prune` and `doctor` and of the harness's handling of panel entries that name a pruned file (session 52's fix).
2. Write docs/brief-64.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-64-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s64/listening-digests-start.txt. Experiments number from 0254.
3. Build `work/bin/s64-start`; `regress session-54-linux` and `regress session-54-linux-nas` identical before anything is pruned.

Steps:

1. Before pruning: `gunfinger --config ~/.config/gunfinger/nas-dnb.toml doctor` and `prune` (without `--yes`) must list exactly the four files above and nothing else; if they list anything else, stop this step, record it, and prune nothing. Save both outputs under work/s64/. Find where the four files appear today: the NAS clusters file, session 33's panels (`work/s33/panels`), the committed panels in `docs/panels/`, the saved reports of the baseline `session-54-linux-nas`, and the corpus store (`work/peaks`; it must not hold them, and is never written).
2. Prune: `gunfinger --config ~/.config/gunfinger/nas-dnb.toml prune --yes`, then `doctor` again: the store at 40,674 records with no ignored records left; record its revision before and after. Then `clusters --from-peaks --reuse-pairs` as session 52 ran it (this re-applies the verdicts and the ignore file to the saved pairs; it is not a rebuild): record the clusters file's SHA-256 and what changed. No full clusters rebuild.
3. The standard evaluation at NAS scale with the pruned store (the saved index is built again on the first search): the runs `regress session-54-linux-nas` compares. Compare with the old baseline and explain every difference by the four files (their plays, their queries, their places in a cluster or panel) or by the set-aside lists moving (experiment 0242's mechanism: the fullest 1% of the index is set aside per run, and a prune can move a list across that cut); a difference you can trace to neither is a finding to stop on and report, not to baseline.
4. Record `baseline session-64-linux-nas` with the final build. `regress session-54-linux` stays identical (the corpus side does not change). One experiment (at most 40 lines): what the prune changed in the NAS-scale reports, calibrate and the clusters, against `session-54-linux-nas`.
5. The evening's answers (docs/status.md, "2026-10-30: the listening of the 2026-10-30 pack"; read them there, never from the answer log's raw records beyond their choices): update the places that depend on them, without changing any verdict, manifest, panel or `docs/same-titles.txt`. The four same-title candidates are different titles: close the question in session 54's notes' pointer and the roadmap, and say in `docs/calibration.md` (if it names the Cool Rock pair) that the strongest false is a confirmed different version. Vintage Dread 2000 disc 1 is separate tracks: record in the roadmap's or session 58's pointer that its five edge parts are false transitions of `parts` (shared material between separate tracks), a negative for any later rule. Three of the census's inverted rips are confirmed by ear: add one line to the roadmap's "Stereo faults before the downmix" and the stereo decision note. Also update the places that name the NAS baseline, the store's record count or the clusters file's digest for the next sessions (docs/status.md, the handoff; older experiments and briefs stay as written).
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 64" section at the end of docs/session-notes.md with a handoff (the new baseline names, the store's record count and revision, the clusters file's digest); update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-54-linux` and `regress session-64-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, `git status` clean. Commit. Your final message: what was pruned, what changed in the reports and why, the new baseline names and digests (store revision, clusters SHA-256), and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's code and behaviour do not change: no threshold, parameter, rule, default or output change. Harness fixes are tested and keep `regress` identical apart from the prune's explained changes.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. This session runs `prune --yes` once with the NAS configuration (step 2, after step 1's check) and `clusters --from-peaks --reuse-pairs`; it never runs `index` with the NAS configuration and does not rebuild the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
- Scratch data goes under work/s64/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00 on 2026-10-31. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 64

- 2026-10-30, 20:56 by the Linux machine's clock (CET). The time box
  is about 1.5 hours: aim to finish by about 22:30. The soft deadline is
  11:00 on 2026-10-31; no new run starts after about 10:20.
- `master` at `c757ea0`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s64/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `ce293fe6165e...`, `work/audition/answers.jsonl` `3dc509cb1ec9...`.
- Experiments 0001-0253; this session numbers from 0254. ADRs
  0001-0024. Test set: 4 of 5 evaluations used (none in this session).
- The NAS store: 40,678 records, revision `1fc6fe50293ad330` (session
  52); the NAS `.gunfingerignore` gained four patterns on 2026-10-30
  (lines 70-76 with their comment; the file before in
  `work/listening-2026-10-30/gunfingerignore.before`).
- Builds: `work/bin/s63-final/` (master; the Rust code last changed in
  `383d91a`). Baselines `session-54-linux` and `session-54-linux-nas`.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build and both regressions (tmux `s64start`) | 20:56-21:10 |
| 1 | `doctor` and `prune` listings (tmux `s64list`, read only, beside the start regressions); where the four files appear | 20:57-21:15 |
| 2 | `prune --yes`, `doctor`, revision; `clusters --from-peaks --reuse-pairs` | 21:15-21:20 |
| 3 | `regress session-54-linux-nas` on the pruned store; every difference traced | 21:20-21:35 |
| 4 | `baseline session-64-linux-nas`; `regress session-54-linux`; experiment 0254 | 21:35-21:50 |
| 5 | the evening's answers in the documents; the places that name the baseline, the record count, the digest | 21:50-22:05 |
| 6-7 | review, notes, status, checklist, final checks | 22:05-22:30 |

## Decisions

- **(20:57) Start build and regressions** (`work/s64/start.sh`, tmux
  `s64start`, logs in `work/s64/logs/`; `run.sh` and `same.py` are
  session 63's with the paths changed). Cargo had nothing to rebuild:
  the start build is byte-identical to `work/bin/s63-final/`.
- **(20:57) Step 1's listings ran beside the start regressions**
  (`work/s64/step1-list.sh`, tmux `s64list`, with `work/bin/s63-final`'s
  `gunfinger`), as in session 52: `doctor` and `prune` without `--yes`
  only read the library and the store, and the regressions only read
  the store, so running them together changes no result (no timing is
  compared).
- **(21:02) Both start regressions identical** (`work/bin/s64-start`):
  `regress session-54-linux` (0:52) and `regress session-54-linux-nas`
  (3:53, 5.4 GiB peak), every report identical to its baseline's apart
  from `wall_seconds`.
- **(21:03) Step 1, the listings.** `doctor` (6:17, beside the NAS
  regression): 48 patterns leave out 627 audio files, lines 73-76 leave
  out one file each; 40,674 current records; one warning, 4 peak records
  and 4 tag notes of 4 files that the ignore file leaves out; no record
  of a gone file, no temporary file. `prune` without `--yes` lists
  exactly those 8 files: the peak record and tag note of each of the
  four uploads, nothing else. So the prune goes ahead. Outputs
  `work/s64/logs/doctor-before.log` and `prune-list.log`; the store
  before it: 40,678 records, revision `1fc6fe50293ad330`
  (`work/s64/store-revision-before.txt`).

  Where the four files appear (`work/s64/where.py`, output
  `work/s64/where-before.tsv`):
  - the NAS clusters file: one pair entry, the corpus Item.Code -
    Magnetics against the Break - Wine upload (9 hits, 20% coverage,
    `same_recording` false); no cluster holds any of the four;
  - session 33's panels, `docs/panels/` and the library map: none;
  - the baseline `session-54-linux-nas`: six sub-threshold candidates in
    `sweep-seed-2026.json` (query 244, Bulletproof - Scarred at -5%: the
    Spider upload, 7 hits; query 391, Bad Company - Pulse at 0%: Bad
    Vibes, 8 hits; queries 605, 606, 608 and 611, Item.Code - Magnetics
    at -3%, -1%, +1% and +8%: Break - Wine, 6-7 hits), 2 windows each;
    and one sub-threshold detection of the 500 Mills upload in each of
    the three Stakka scans (335.5-346.3 s, 7 hits, 2 windows);
  - the corpus store (`work/peaks`, 265 records, read only): none.
- **(21:04) Step 2 and step 3 queued together** (`work/s64/step2.sh`
  then `step3.sh`, tmux `s64prune`, `work/bin/s64-start`), after the NAS
  regression had finished reading the store.
- **(21:04) Step 2, the prune** (`work/s64/step2.sh`, tmux `s64prune`,
  `work/bin/s64-start`). `prune --yes` deleted the 8 files (0.2 MB).
  `doctor` after it: 40,674 current records, no warning of ignored
  records. The store went from 40,678 records, revision
  `1fc6fe50293ad330`, to 40,674, revision `7210fc479da58376`
  (`scripts/analysis/store_revision.py`); 40,674 tag notes and 1,926
  skip notes. Both saved NAS indexes (the CLI's in `~/.cache`, the
  harness's in `work/indexes/`) are out of date and are built again by
  the next search.
- **(21:04) `clusters --from-peaks --reuse-pairs`** (with `--panels
  work/s33/panels`, 0.2 s): 2,875 pairs of 2,876 clustered again, 1
  dropped (Item.Code - Magnetics against the Break - Wine upload, 9
  hits); the same 284 clusters (`duplicates` and the criterion
  unchanged, the other pair entries in the same order). The clusters
  file is now `c4d3da2e7e37...` (was `ce293fe6165e...`; both in
  `work/s64/`). The same four verdict lines as in session 52 name an
  ignored file and add nothing (lines 26, 45, 67 and 93); no verdict
  line names any of the four uploads.
- **(21:10) Step 3, the standard evaluation on the pruned store**
  (`work/s64/step3.sh`, tmux `s64prune`, `work/bin/s64-start`): `regress
  session-54-linux-nas` (5:41 with the saved index built again; 5.4 GiB
  peak). Scores and calibrate unchanged (11/11, 8/11, 0/11, 0 wrong;
  weakest identifying 704, strongest false 282, 7 false accepted, 95
  false candidates at 30 hits or more). Differences
  (`work/s64/diff.py`, output `work/s64/diff-pruned.txt`; the reports
  copied to `work/s64/pruned-reports/`):
  - the pruned files: the six sweep candidates and the three scan
    detections of step 1 are gone, nothing takes their place;
  - not of a pruned file: in the sweep, one weak candidate gains or
    loses a hit in queries 201 (Decoy at -1%, 6 to 7) and 698 (Konflict -
    RoadBlock at +1%, 7 to 6); in query 296 (Kemal - Animation at +8%)
    one weak candidate moves from +4.8% to +4.0% and two new ones appear
    (Falcon and Predator - Shadow Depth, 8 hits; S.P.Y - Need You, 6);
    and Recharger's
    fitted speed at four speeds (queries 495, 496, 499, 503; about 1e-6
    percentage points, hits unchanged; `per_speed`'s mean speed errors
    with it); in the Stakka scan, the three confident Clockwork
    detections' speed, by less than 1e-5 percentage points (hits
    unchanged).
- **(21:12) Tracing the second group to the lists set aside.** The
  pre-prune saved index (40,678 records) survives as session 63's hard
  links (`work/s63/wd/indexes/`, only read). Whole index
  (`work/s64/cut_moved.py`): 1,086,463 lists, cut 9,938 in both; two
  lists left the set aside (hash 1141698, 9,939 to 9,938 postings, and
  1212162, 9,940 to 9,938), each losing postings of a pruned file;
  `work/s64/list_holders.py` shows hash 1141698 holds 11 postings of
  each corpus Clockwork file and 14 of the Remastered upload, so the
  scan's first pass now reads them (by inference, the chain's fit moves
  with them). The sweep leaves its held-out files out and has a cut of
  its own; only `recall` had `--set-aside-like` (session 59).
- **(21:13) Decision: `sweep --set-aside-like FILE`** (harness only,
  opt-in, the conservative way to trace the sweep's changes instead of
  inferring them): as `recall`'s, `fullest::setting_aside_like` with the
  sweep's left-out files; the report's name ends in `-set-aside-like`
  and holds `set_aside_like` (left out of the JSON when absent, so
  default reports do not change); refused with `--rank-fullest-by` or
  another matcher. The line both commands print moves to
  `fullest::print_like`. One test (the option, its report name and the
  refused combination); `setting_aside_like` has its own. Not added to
  `scan`: the whole index's moved lists explain the scan by inference,
  and the leave-out scans change only by the pruned detection.
- **(21:17) The sweep under the old lists** (`work/s64/like.sh`, tmux
  `s64like`, `work/bin/s64-run`, scratch `work/s64/wlike`; 2:38): equal
  to the baseline less the pruned files' candidates, every query and
  `per_speed`. In the sweep's index one list left the set aside (hash
  851777, 9,919 to 9,871 postings, 48 of the four files) and one entered
  (1254102, 9,911 in both, as the shortest list set aside fell from
  9,912 to 9,911 postings). So every sweep difference traces to the four
  files or to the lists set aside, and the scan's Clockwork speeds to
  the lists by inference; nothing to stop on.
  Experiment 0254, data `docs/experiments/data/0254-prune-four-ignored.json`
  (`work/s64/data.py`).
- **(21:20) Step 4** (`work/s64/step4.sh`): `scripts/check.sh`, the
  final build `work/bin/s64-final`, `regress session-54-linux` and
  `regress session-54-linux-nas` (compared with step 3's reports), then
  `baseline session-64-linux-nas`. In the first queue (tmux `s64base`)
  `scripts/check.sh` failed at `cargo fmt --check` (the new test's
  layout), and the queue went on, since `step4.sh` does not stop on a
  failed step; it was stopped by hand during its NAS regression. After
  `cargo fmt` the queue ran again (tmux `s64base2`).
- **(21:28) Step 4** (`work/s64/step4.sh`, tmux `s64base2`, every run
  exit 0): `scripts/check.sh` green (4:55); `work/bin/s64-final` built
  (`gunfinger` byte-identical to the start build's; `gunfinger-eval`
  with `sweep --set-aside-like`, not byte-identical to `s64-run`'s,
  which was built before `cargo fmt` and the two fixtures' new field,
  both in test code only; why the binaries differ is not established);
  `regress
  session-54-linux` identical (0:51); `regress session-54-linux-nas`
  (3:46, the saved index loaded) gives step 3's reports, every one
  identical; `baseline session-64-linux-nas` copies them (identical to
  `work/s64/pruned-reports/`).
- **(21:30) Step 5.** The evening's answers: session 54's notes (the
  same-title candidates closed), session 58's notes (Vintage Dread 2000's
  disc 1, separate tracks), the roadmap (the same-title bullet, "Stereo
  faults before the downmix", and one sentence in "Library ignore
  file"), `docs/calibration.md` (the strongest false is a confirmed
  different version) and the stereo decision note (one sentence). No
  roadmap bullet covers `parts`, so the Vintage Dread answer goes to
  session 58's pointer only. `docs/calibration.md` has no NAS record
  count or baseline name, and no calibrate number moved. The places that
  name the baseline, the record count or the digest for the next
  sessions: `docs/status.md` (a session 64 entry) and the handoff; older
  experiments, briefs and status entries stay as written.
- **(21:29) The scripts' needle for Bad Vibes** also matched the
  unignored copy in `__youtube_archivists/unxplored-dnb/`; narrowed to
  the 3zb-gangstep-dnb folder in `where.py`, `diff.py` and `data.py`.
  Run again with outputs to `/tmp`, `where.py` and `diff.py` give the
  same lines as `where-before.tsv` (less the clusters pair now pruned)
  and `diff-pruned.txt` (no report compared held the other copy), and
  0254's data file is rewritten byte-identical.
- **(21:38) The review's findings** (a separate agent, read-only,
  21:31-21:38). Every number checked matches the evidence; no verdict,
  same-title judgment, panel or manifest changed; the rules were
  followed. Taken:
  1. 0254 had 42 lines: shortened to 40.
  2. The Stakka scan's cause is inferred (the scan was not run under the
     old lists), but several places stated it as fact: 0254, the notes,
     status and this brief now say so.
  3. The three sweep queries changed through the lists set aside were
     described as "one weak candidate each": each change is now stated
     (above, 21:10).
  4. The full check of the sweep under the old lists was not saved
     (`work/s64/diff-like.txt` was a traceback of `diff.py`, which looks
     for the default report names): `work/s64/like_check.py` compares
     every query (720 of 720 equal to the baseline less the pruned
     files' candidates; only `indexed_assets` and `set_aside_like`
     differ), and the data file records the count; `diff-like.txt` is
     deleted.
  5. Corrections here: the first step 4 queue, the binary difference,
     the needle entry's time and outputs.
  6. The roadmap's session 52 sentence is now in the past tense.
  7. The new test's name says what it checks
     (`set_aside_like_names_its_report_and_takes_no_ranking`); the
     sweep's refusal of every non-default matcher, stricter than
     `recall`'s, is recorded in the notes. Not taken: a test of that
     refusal, which needs a harness (`run_sweep`), as `--rank-fullest-by`'s
     has none either.
  8. "The 8th digit" replaced by the size of the move.
  9. Prose fixes in 0254 and the notes.
- **(21:39) Final checks** (`work/s64/final.sh`, tmux `s64final`, which
  stops at the first failed step): `scripts/check.sh`; the final build
  `work/bin/s64-final` (step 4's build kept as `work/bin/s64-step4`);
  `sweep --set-aside-like` again with the final build in
  `work/s64/wlike2`, compared with the `s64-run` report and checked by
  `like_check.py`; `regress session-54-linux` and `regress
  session-64-linux-nas`; the digests.
- **(21:51) Final checks** (every run exit 0): `scripts/check.sh` green
  (4:50, 664 tests); `work/bin/s64-final` (`gunfinger` byte-identical
  to the start build's); `sweep --set-aside-like` with it (2:27) gives
  the `s64-run` report, identical, and `like_check.py` finds 720 of 720
  queries equal; `regress session-54-linux` (0:52) and `regress
  session-64-linux-nas` (3:47) identical, every report equal to its
  baseline's apart from `wall_seconds`; the verdicts file and the answer
  log at their start digests, the clusters file `c4d3da2e7e37...`
  (`work/s64/listening-digests-end.txt`). Then `baseline
  session-64-linux-nas` saved again with the final build, from those
  reports (byte-identical to the first copy, kept in
  `work/s64/baseline-first/`).
