# Session 82 brief: two records for a file with an inverted channel, a harness pilot

Written on 2026-11-07, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it), `AGENTS.md` and
   `docs/session-rules.md`.
2. This file, and `docs/session-82-checklist.md` (progress).
3. `docs/research/2026-10-30-stereo-power-decision.md` (its "Session
   67"), experiments 0248, 0262 and 0263, the roadmap's "Stereo faults
   before the downmix", `docs/decisions-queue.md` item 3, and the status
   entry "2026-10-31: the last answer, and the user's decisions and
   views" (`docs/history/status-2026-10-23-to-10-31.md`).
4. The code: `crates/gunfinger-eval/src/stereo.rs` (`inverted`,
   `decode_repairing`, `build_store`), `padding.rs` (how the harness
   builds an index of the corpus, the other library and its copies),
   `second_records.rs` (this session's variant) and `harness.rs`.

## The orchestrator's prompt, verbatim

> Start session 82 of Gunfinger: a harness pilot of the user's idea for rips with an inverted channel: index both the file as ripped and its repaired form (docs/status.md or its history file, "2026-10-31: the last answer, and the user's decisions and views"; docs/decisions-queue.md, item 3). Background: about 0.4% of NAS rips have one channel inverted, so the mono downmix cancels much of the centre (experiment 0248); session 67 (0262-0263) found that a polarity repair at indexing recovers the four plainly inverted recordings in full (0 of 48 to 48 of 48 excerpts, 0 to 4 of 4 mix plays, no new wrong answer) but loses all 48 queries of the audio as ripped, because a mix built from the inverted rip carries the cancelled downmix; its registered bar also failed on the set-aside cut's moves (0242's mechanism) and on the census's share. The external review of 2026-11-05 (`docs/research/2026-11-05-external-review.md`) adds: score the two forms apart, never sum their hits or let a chain mix them, include genuine wide stereo and the shared-material negatives, and use the heard-but-unflagged partial inversion of the 2026-10-31 listening as a control of the detector's recall. It is a measurement session: the product and the stores do not change.
>
> You run unattended: the user cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.
>
> The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).
>
> Time box: started about 21:45 on 2026-11-07; aim to finish within about 2.5 hours. If a step would run past that, scale it down (the four plainly inverted recordings and the controls at 262 tracks first), record the decision in the brief and continue.
>
> Environment (sessions 33-81's handoffs in docs/session-notes.md; check each):
> - Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
> - Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
> - NAS store: 40,666 records, revision `0bd3f0c23fabc698` (session 75 pruned the records of DJ sets and a mixed CD's track fix). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-78-linux` (262 tracks) and `session-78-linux-nas` (NAS scale), both with the 2003 and Nocny scans since session 78; `regress --strict` fails on any difference; a session that adds baselines names them in its handoff, and the next prompt says which to use.
> - Build `work/bin/s81-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
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
> 1. Read CLAUDE.md and follow its reading list, notes "Session 61", "Session 62", "Session 67" and the newest session, docs/research/2026-10-30-stereo-power-decision.md, the roadmap's "Stereo faults before the downmix", experiments 0245-0250 and 0262-0263, the status entries of 2026-10-31 (the user's views; the listening that heard a partial inversion), and the code of the polarity check and repair (`stereo-width`, `variant-store` or wherever session 67 put them), how the harness builds an in-memory index from audio, and how detections map to assets and clusters.
> 2. Write docs/brief-82.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-82-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s82/listening-digests-start.txt. Experiments number from 0288.
> 3. Build `work/bin/s82-start`; the start regressions follow docs/session-rules.md.
>
> Steps:
>
> 1. Pre-registration, committed before the scored runs: which files get a second record (the flag of session 67's check, its threshold fixed in advance; the partial inversion heard on 2026-10-31 as a control the flag may miss); how a second record maps to the file (one asset, two records; detections reported per record and merged at the file only by taking the stronger, never by summing hits or by joining chains across the two records); the queries (excerpts and generated mixes of the as-ripped audio and of correct-polarity audio of the same recordings, other rips where the store holds them, genuine wide-stereo files not flagged, the shared panels' development halves as negatives, the sweep at both scales); the scores (both query classes recovered, wrong confident answers, the possible tier's false candidates, the set-aside cut's moves counted and attributed with one check as docs/session-rules.md says, the posting cost of the second records, the time); and what would make it worth an ADR.
> 2. The variant, harness only, opt-in, tested (the mapping, the no-sum rule, the flag); `regress` identical with it off.
> 3. The runs at 262 tracks and NAS scale (never writing a store; reading the NAS only), under tmux with logs where long.
> 4. Experiments (at most 40 lines each, written before the next starts): recovery of both classes and the controls; the cost and the set-aside moves.
> 5. Update the stereo-power note's follow-up, the roadmap's item and docs/decisions-queue.md item 3 with the evidence and what an ADR would decide.
> 6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
> 7. Wrap-up: a "Session 82" section at the end of docs/session-notes.md with a handoff and any questions for listening; update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-78-linux` and `regress session-78-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: both query classes with two records, the controls, the cost, and anything left.
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
> - Scratch data goes under work/s82/; delete renders you no longer need.
> - Prose: no em dashes (spaced en dashes in prose, ASCII in code).
> - Commit after each step that changes tracked files.
> - Time: the user's soft deadline is 11:00 on 2026-11-08. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.

## State at the start

- `master` at `8252eb8`, `git status` clean (apart from the untracked
  `yafc20261031.log`). `work/bin/s82-start/`, built from `8252eb8`, is
  byte-identical to `s81-final` (`gunfinger` `f3ad06283ce8...`,
  `gunfinger-eval` `7db22b90766e...`). The store revisions are unchanged
  (NAS `0bd3f0c23fabc698`, corpus `526a3b978eb3e411`, read from a 10 s
  `identify` with each configuration). By `docs/session-rules.md` the
  start regressions are skipped; the final ones run.
- Baselines `session-78-linux` and `session-78-linux-nas`; NAS clusters
  `c4d3da2e7e37...` (284 clusters). Digests at the start:
  `work/s82/listening-digests-start.txt`.
- The harness has session 67's check (`stereo::inverted`) and repair
  (`variant-store --repair-inverted`, `decode_repairing`), which write a
  whole store of the library's files with the flagged ones repaired. No
  option indexes a second record of a file beside its first.
- 0263's arms: T (today's records) finds the four plainly inverted
  recordings in 0 of 48 correct-polarity excerpts and 48 of 48 as-ripped
  ones; R (the five records replaced) 48 of 48 and 0 of 48. Its queries
  were searched with `identify --store-only`; the census's flagged five
  are in `work/s67/f-files.txt`.

## Plan

1. Harness, opt-in and tested, `regress` identical with it off:
   - `--second-records DIR` (with `--other-peaks-dir`): every current
     record of the peak store DIR is a second record of the other
     library's file at the same path, indexed as its own asset
     `second-record/second-library/<path>` after every other record. A
     saved index with second records has its own file name.
   - `second_records::strongest`: at the file, the plays of its two
     records that overlap in query time are reduced to the stronger
     (the higher level, then more hits); hits are never added.
   - Scoring counts a second record as its file (`padding::original_path`).
   - `variant-store --repaired-only`: write only the records the check
     repairs, so that the flag alone decides which files get a second
     record.
   - `second-records QUERIES...`: search audio files against the index
     and report every play per record and the strongest per file.
2. The registration (below), committed before the scored runs.
3. The scratch library (F, P, W below, copied from the NAS with `cp -p`),
   its records today's way and `variant-store --repair-inverted
   --repaired-only`; the queries (`work/s82/queries.py`); the runs at NAS
   scale, then 262 tracks.
4. Experiments 0288 (recovery of both classes and the controls) and 0289
   (the cost and the set-aside moves), each written before the next starts.
5. The note, the roadmap, decisions queue item 3; the review; the wrap-up.

## Registration (committed before the scored runs)

**Which files get a second record.** The flag is session 67's check,
unchanged (`stereo::inverted`: both channels within 20 dB, correlation
at lag 0 of -0.5 or less, strongest there within ±80 samples at 8 kHz).
It runs through `variant-store --repair-inverted --repaired-only` over a
scratch library of the NAS files below (copied with `cp -p`, read only).
The rest of the NAS store is not checked: the pass over every file takes
about 66 minutes (0262) and changes no file outside the census, whose
6,766 files stand for the store as in 0262.

- F: the five files the union census flags (`work/s67/f-files.txt`):
  Paradox - Filmscape, Usual Suspects - Advent, Plasmic - The Weapon,
  Keltech - Axis (plainly inverted) and We - Hello World (uncertain,
  reported apart).
- P: We - Polynesian Inventor, heard inverted on 2026-10-31 but not
  flagged (correlation -0.27, 2 of 7 windows at -0.5 or less): the
  control of the detector's recall.
- W: 12 wide unflagged census files (side to mid above -10 dB,
  correlation at lag 0 above 0, no inverted window), a seeded draw
  (`work/s82/picks.py`, `work/s82/scratch-files.tsv`).
- Expected: second records for the five of F, none for P or W. Every
  first record stays as it is; the scratch store's records made today's
  way must equal the NAS store's byte for byte.

**How a second record maps to the file.** One file, two records: the
first is today's record (the downmix as ripped), the second the repaired
one, indexed as its own asset. Chains and plays form per asset, so no
chain joins the two. Detections are reported per record. At the file
they are merged only by taking the stronger (`second_records::strongest`):
of a file's plays from its two records that overlap in query time, the
one with the higher level, then more hits, is kept; a file's level is the
higher of its records' levels, and hits are never added.

**Arms.** T: today's index (the saved NAS index, or the 262-track index
with the scratch store as the other library). B: T with the second
records. R: B without F's first records (0263's repaired store, in the
same harness). Each arm is one run of `second-records` on the same
renders.

**Queries** (`work/s82/queries.py`, rendered afresh from the NAS audio as
0263's: mono MP3, 128 kb/s):

- F, correct polarity (right channel negated): 12 excerpts per file
  (30 s from 20, 40, 60 and 80% of the file at 0.96, 1.00 and 1.04) and
  one mix (150 s of the file at 1.02 between two drawn corpus tracks).
- F, as ripped: the same 12 excerpts and mix without the correction.
- P: one 30 s excerpt at 1.00 from each of its 30 s windows, as ripped
  and with the right channel negated (its inverted windows marked).
- W: 4 excerpts at 1.00 (20-80%) per file, as ripped and with the right
  channel negated.
- The sweep (seed 2026): under B at NAS scale against
  `session-78-linux-nas`'s sweep report, and once more with
  `--set-aside-like` today's saved index (the one check that attributes
  the cut's moves); at 262 tracks T and B with the scratch store.
- The shared panels' development halves (panels 1 and 2) at NAS scale,
  T (the start build) and B. Never their validation halves.
- Other rips: any other file confident on F's queries is reported (0263
  found none by audio).

**Scores**, per arm and class, at the file (after `strongest`) and per
record:

- S1 recovery: confident excerpts and mix plays of the true file, for the
  four plainly inverted recordings (Hello World apart).
- S2 wrong confident answers: a confident play of a file that is not the
  query's (the mixes' corpus tracks count as truth); for the sweep and
  the panels, the harness's own scoring.
- S3 the possible tier's false candidates: possible plays of other files.
- S4 the set-aside cut: the sweep queries that differ between B and the
  baseline, and how many `--set-aside-like` restores.
- S5 cost: the second records' postings (count, share of the index) and
  the index build's time and memory.
- S6 time: the search time per arm (a shared machine: not product
  timings).
- S7 controls: the flag on P and W; P's and W's confident counts under T
  and B.

**What would make it worth an ADR proposal** (all of):

- A1: under B, the four plainly inverted recordings are confident in at
  least 47 of 48 excerpts and 4 of 4 mix plays in each class (as ripped
  and correct polarity).
- A2: no new wrong confident answer under B against T in any query set
  (the plan's queries, the sweep, both panels), at the file and per
  record.
- A3: the possible tier's false candidates under B at most T's plus
  max(1, 1% of the set's queries), per set.
- A4: every NAS sweep query that differs between B and the baseline is
  restored by `--set-aside-like` (the count reported); no confident
  detection gained or lost on P, W, the sweep or the panels.
- A5: the flag gives second records to the five of F and to no file of P
  or W; the first records are byte-identical to the NAS store's.
- A6: the second records add at most 0.1% of the NAS index's postings.

Passing A1-A6 makes the per-record route worth an ADR proposal, which is
the user's decision (decisions queue item 3). A failed item is reported
as a failure; a revised rule would need a new registration and renders
the first run did not use. Hello World, P and W decide nothing alone.

## Decisions

- *(21:48)* Start regressions skipped: `s82-start` is byte-identical to
  `s81-final` and the store revisions are unchanged.
- *(21:51)* The panels' T arm (start build, no variant) started before
  this registration was committed, because it runs for an hour or more;
  its reports are not read until the registration is committed.
- *(21:55)* The flag is applied to the scratch library only (F, P, W),
  not to every NAS file (above).
- *(22:02)* The 262-track `regress --strict` (identical) ran with
  `s82-run`; a clippy fix that moves the index's time to the caller of
  `second_records::search` followed, and the runs use `s82-run2`. The
  final regressions run with the final build.
- *(22:02)* The queries are rendered from the scratch library's copies of
  the NAS files (`cp -p`; their records today's way equal the NAS
  store's), not from the NAS itself.
- *(22:05)* At 262 tracks the scratch store (`work/s82/c/today`, 18
  records) is the other library; the NAS map and clusters files stand in
  for its own, since none of its files is a copy of a corpus file.
- *(22:05)* `shared` and `scan` score a second record by its own name (not
  in any cluster), so a confident second record would count against the
  arm: the conservative reading.
- *(22:18)* Added `second-records --set-aside-like` (`b574f95`, build
  `s82-run3`): the registered check covers the sweep only, but at 262
  tracks 98 of 240 plan queries move between T and B, so the same one
  check is run for them and for the 262-track sweep. No bar item depends
  on it beyond A4's reading.
- *(22:20)* A slip: the NAS queue started the sweep (0289's run) at 22:13,
  before 0288 was committed (22:21); the sweep's results were not read
  until 0288 was committed.
