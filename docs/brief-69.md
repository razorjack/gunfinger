# Session 69 brief: recurring unidentified passages across a batch of mixes

Written on 2026-10-31, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file (its pre-registration above all), and
   `docs/session-69-checklist.md` (progress).
3. The proposals note `docs/research/2026-10-29-matching-and-archive-proposals.md`
   (§6, "Evaluation and adoption boundaries"); `docs/brief-35.md`;
   ADR 0017, 0023 and 0024.
4. `docs/session-notes.md` ("Session 34", "Session 35", "Session 58",
   "Session 68"); experiments 0063, 0159-0166, 0185, 0241 and this
   session's (0267 and up); the roadmap's "Time without a confident
   match".
5. `crates/gunfinger-core/src/repeats.rs` (the gap self-match),
   `crates/gunfinger-eval/src/mixes.rs` (`PlannedPlay`, `render`,
   `crossfade`, `search_mix`) and this session's module,
   `crates/gunfinger-eval/src/passages.rs`.

`docs/brief-2.md` to `docs/brief-68.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 69 of Gunfinger: can a batch of mixes find its own recurring unidentified passages? A pilot of proposal 6 of `docs/research/2026-10-29-matching-and-archive-proposals.md`. An archive could report "the unidentified passage at 23:14 also matches audio at 51:06 in another mix" even when no clean reference is indexed, and one occurrence may be much clearer than another. The note names the two details that matter: opposite deck settings can exceed the ladder's relative reach (1.08 / 0.92 is about +17.4%, before rip differences), so a prototype needs an explicit relative-speed range and its cost; and direct passage matches must not propagate titles through chains of partial overlap (mixed-CD tracks and uploads holding parts of other records, 0063, 0185, 0241, and Vintage Dread 2000's separate tracks that `parts` flagged, session 64). The gap self-match is precedent for working without an indexed title, but shared material made it merge records (0163). This session measures passage retrieval across generated mixes with known construction, scoring a shared snippet apart from any whole-track claim, which is not an output of this pilot. It is a measurement session: the product does not change.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32). Rendering runs well in parallel here.

Time box: started about 05:40; aim to finish within about 3 hours, and by 10:40 at the latest (the user's soft deadline is 11:00). If a step would run past that, scale it down (fewer mixes, one speed range, 262 tracks' recordings only), record the decision in the brief and continue.

Environment (sessions 33-68's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,674 records, revision `7210fc479da58376` (session 64 pruned four ignored uploads). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-54-linux` (262 tracks) and `session-64-linux-nas` (NAS scale; compare with it, not `session-54-linux-nas`).
- Build `work/bin/s68-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Since `45f92d5` (2026-10-28) its calibrate reads only the reports its baseline holds; still run variants and other seeds in a scratch work directory, never in the reports directory `regress` compares.
- The user restructured the code on 2026-10-28 without meaning to change behaviour (commits `b5a84e3` to `f94604a`: harness subcommands in modules, named results, `Search` passed through the core's search, enums for closed sets). Session 60 confirmed both regressions identical on a build of master after it.
- Since sessions 53-56: `SpansAtSpeed(0.006)` is the default second suppression (ADR 0021); same-title detections (`docs/same-titles.txt`) are scored right (ADR 0022); `identify` prints gaps as "ID - ID" lines with an estimate of their tracks (ADR 0023, 0024). The minimum span stays undecided. A change to an existing decision needs a new ADR (the next number is in `docs/adr/`).
- The research note `docs/research/2026-10-29-matching-and-archive-proposals.md` (2026-10-29) records six unmeasured proposals and its evaluation boundaries; any session working on one follows its "Evaluation and adoption boundaries".
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen. On the evening of 2026-10-30 the user answered 8 of 12 questions (docs/status.md, "the listening of the 2026-10-30 pack"): the four same-title candidates (Cool Rock, Tower Bass twice, Stalker) are different titles; Vintage Dread 2000 disc 1 is separate tracks; Advent, Keltech - Axis and Plasmic - The Weapon have an inverted channel, We - Hello World is unsure. The pitch rides (Star Trails, Into Music) and the Planet Dust cut stay unanswered. When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
- The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-35.md and the newest brief, docs/session-notes.md ("Session 34", "Session 35", "Session 58" and the newest session), docs/status.md (the newest entries), the proposals note (§6 and "Evaluation and adoption boundaries"), ADR 0017, 0023 and 0024, experiments 0063, 0159-0166 (the gaps and the self-match), 0185 and 0241, the roadmap's "Time without a confident match", and the code of the gaps self-match, the mix generators (`mixes/`) and how the harness builds an in-memory index from audio.
2. Write docs/brief-69.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-69-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s69/listening-digests-start.txt. Experiments number from 0267.
3. Build `work/bin/s69-start`; `regress session-54-linux` and `regress session-64-linux-nas` identical.

Steps:

1. Pre-registration, committed before any batch run: the batch (generated mixes in which held-out recordings, left out of the index, occur several times with different partners, cue points, EQ and speeds, including opposite deck settings; controls: non-overlapping excerpts of one source, unrelated records sharing a break, related versions, a repeated radio insert); the matching (the mixes' unidentified spans, or all of each mix, indexed temporarily in memory and searched against each other with an explicit relative-speed range, its cost stated; never as library tracks); the scores (passage retrieval against the construction: the same source's overlapping passages found, with their times; non-overlapping excerpts of one source not linked; shared breaks and related versions counted as wrong links; whole-track claims not made); the no-propagation rule (links reported pairwise with their supported spans, never merged through chains); and what would make the capability worth a design note and an ADR.
2. The prototype, tested, harness only (an opt-in subcommand; `regress` identical): unit tests for the pairwise report (no transitive merge) and for the relative-speed range.
3. The runs, at 262 tracks' recordings first (held-out ones left out of the index), under tmux with logs where long; renders deleted when no longer needed.
4. Experiments (at most 40 lines each, written before the next starts): retrieval and wrong links; the cost and the speed range.
5. A short note in the proposals note's §6 and the roadmap's "Time without a confident match": what the pilot finds, its cost, the failure cases, and whether it is worth a next step.
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 69" section at the end of docs/session-notes.md with a handoff; update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-54-linux` and `regress session-64-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: passage retrieval, wrong links, the cost, and anything left.

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
- Scratch data goes under work/s69/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00 on 2026-10-31. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 69

- 2026-10-31, 05:37 by the Linux machine's clock (CET). The time box is
  about 3 hours: aim to finish by about 08:40, at the latest by 10:40;
  no new run starts after about 10:20.
- `master` at `c4310c3`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s69/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `c4d3da2e7e37...`, `work/audition/answers.jsonl` `3dc509cb1ec9...`.
- Experiments 0001-0266; this session numbers from 0267. ADRs
  0001-0024. Test set: 4 of 5 evaluations used (none in this session).
- The NAS store: 40,674 records, revision `7210fc479da58376`. Builds:
  `work/bin/s68-final/` (master). Baselines `session-54-linux` and
  `session-64-linux-nas`.
- What exists: the gap self-match (`gunfinger_core::repeats`) matches a
  gap's pair hashes against its own at lags of 8 s or more, at speed 1.0
  (and, in the harness, other ratios); it counts records and links no
  two places of different recordings. The harness builds indexes from
  peak records (`Index::build`); a posting addresses at most 17:28 of
  one record (`index::addressable_length`), so a mix is indexed in
  pieces. The global `--extra-rungs N` adds N rungs of 0.4% past each
  end of the ladder (0.92-1.08, 41 rungs).

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build and both regressions (tmux `s69start`) | 05:37-06:05 |
| 2 | the prototype `passages` (harness only, opt-in), unit tests for the pairwise report and the relative-speed range; `regress` identical | 06:05-07:15 |
| 1 | pre-registration (below), committed before any batch run | written 06:00, committed with or before the prototype |
| 3 | the batches at 262 tracks (tmux) | 07:15-07:45 |
| 4 | experiments (0267 and up) | with the runs |
| 5 | the proposals note's §6 and the roadmap | 07:45-08:00 |
| 6-7 | review, notes, status, checklist, final checks | 08:00-08:45 |

The prototype is written before the pre-registration's commit only
where the pre-registration names its exact rules; no registered batch
runs before the commit. Smoke runs on seed 6900 (two or three mixes)
test the code and are not reported as results.

## Pre-registration (step 1; written at 05:38-05:45, before any batch run)

**Question.** Within one run over a batch of generated mixes, can the
mixes' unidentified passages be matched against each other, so that a
held-out recording's occurrences in different mixes are linked with
their times, at different speeds (opposite deck settings included),
partners, cue points and bass swaps, while non-overlapping excerpts of
one source, records sharing a passage and related versions are not
linked, and no link propagates through a chain?

**The batch** (`gunfinger-eval passages`, `passages::draw_batch`; two
batches, seeds 6901 and 6902; 12 mixes each). Library: the 262 corpus
records. Everything drawn below as a subject, a control or the insert
has every file of its cluster left out of the index (`sweep_index`'s
`left_out`, beside seed 2026's held-out clusters); no recording of the
panel's validation half (Clockwork, Fractles, Mind Overload, Night
Gasp, T-Minus, The Nine and their files) plays in any role.

- *Subjects*: 16 recordings drawn from the corpus clusters whose first
  file lasts at least 330 s, none of a panel family. Subject i occurs
  in mixes i mod 12, (i + 4) mod 12 and (i + 8) mod 12, so each mix
  holds 4 subjects' occurrences. Each occurrence plays a file of the
  subject's cluster drawn at random (rips may differ). Occurrence 0
  plays at a turntable speed drawn in 1.04-1.08, occurrence 1 in
  0.92-0.96 (opposite deck settings: relative speed 1.083-1.174),
  occurrence 2 in 0.94-1.06. Mix-time lengths: 60-150 s (occurrences 0
  and 1 of a control subject: 60-100 s). Cue points: occurrence 0
  starts in the track at 10-35% of its length (control subjects:
  8-20%); occurrence 1 at occurrence 0's start ±30 s (at least 5% in),
  so the two overlap in the source. Occurrence 2: for even i, from
  occurrence 0's start -30 s to +60 s (usually overlapping); for odd i
  (the control subjects), from 15 s after the later of occurrences 0
  and 1 ends in the source, shortened to end by 97% of the track and
  never under 40 s, so it shares no source time with them. A control
  subject whose third occurrence cannot fit is replaced by the next
  draw.
- *Partners*: indexed corpus records (outside the left-out and
  held-out clusters), distinct clusters within a mix, drawn per mix
  (`mixes::draw_unused`), 60-150 s each, starting in the track as the
  generator's `drawn_play` places a play, at 0.94-1.06.
- *Order and joins*: per mix the held-out items (subject occurrences
  and controls) in a drawn order, with a partner before, between and
  after them; consecutive plays joined by the long mixes' crossfade
  (`mixes::crossfade`: 10-60 s, at most half the shorter play, half of
  them with a bass swap). EQ is the generator's bass swap only.
- *Records sharing a passage* (the panel's development half, with the
  passage the panel records): related versions, Dozer and its Notjamie
  bootleg, Fire (original mix) and its Rogue Unit remix, Sonar and its
  Mark System revision; different tunes sharing a passage, Pyro - Time
  Is Broken and Ignorance, Watermelon and 300 Baud - Monotone (Mobiuz &
  Cycom Remix). Pair j's two records play in mixes 2j and 2j + 5 (mod
  12), each from 20 s before its side of the passage (or its start) for
  the passage's length plus 40 s (at least 60 s), at 0.94-1.06. The
  NAS files among them are read from the NAS library for rendering only
  (`--other-library`); nothing at the NAS root plays. The panel does
  not separate "related" from "shares a break"; the labels above follow
  the panel's names and verdicts.
- *A repeated insert*: a 12 s excerpt from the middle of one drawn
  corpus record (at least 120 s long, of no other role), at native
  speed and 0 dB with 0.5 s fades, laid over the middle of one subject
  occurrence in each of mixes 0, 2, 4, 6, 8 and 10: identical audio at
  one speed, as a radio station's jingle repeats. It stands in for a
  radio insert; speech is not rendered.

**The matching.** Each mix is rendered as `mixes` renders it (128
kbit/s MP3, no channel fault), decoded as the default front end
decodes a query, and searched against the 262-track index with
today's defaults on one thread. Its *unidentified spans* are the
stretches of 30 s or more that no confident detection covers
(`gaps::uncovered`; 30 s, 0159's shortest length that reaches every
missed DJ-length track). Two modes:

- `gaps`: the pieces are the unidentified spans;
- `whole`: the pieces are the whole mixes.

A piece longer than 600 s is cut into pieces of 600 s overlapping by
60 s. Every piece of every mix is indexed temporarily in memory (its
peaks from the decoded mix, at speed 1.0, as a record of the harness's
own name, never a library track; the matcher's index transform as the
product's). Each piece is then searched as a query against that index
(the matcher's search and defaults), and detections of pieces of its
own mix are dropped. Two relative-speed ranges: ±8% (the product's
ladder, 41 rungs) and ±18% (`--relative-speed-range 0.18`: 25 extra
rungs past each end, 91 rungs, 0.82-1.18, which covers 1.08/0.92 =
1.174 and its inverse 0.852). The cost of each (elapsed time, CPU,
peak memory; renders, decoding, the 262-track search, the pieces'
index and the pairwise search apart) is reported.

**The report (the no-propagation rule).** A link is one confident
detection under the frozen rule: the query's mix and span, the
reference's mix and span (mix times), the relative speed, hits and
windows. Links are reported pairwise. Two links between the same two
mixes whose spans overlap in both mixes are one correspondence, spans
united (the two directions of one match); nothing else is merged, so a
correspondence of A with B and one of B with C never make one of A with
C, and no connected component of links is formed or named. No title
and no whole-track claim is reported.

**The scores** (`passages::score`, against the construction). A
correspondence is assigned, in this order, to the first of:

1. *right*: a play in each mix of the same recording (one cluster)
   overlaps the correspondence's span there by 10 s or more, and at the
   middle of the overlapped part the source positions the two plays
   give (the link mapping one mix's time to the other's linearly over
   its spans) differ by 10 s or less;
2. *insert*: both spans lie at least half within plays of the insert
   (a right passage link, counted apart);
3. *one source, another passage*: plays of the same recording overlap
   both spans by 10 s or more but their positions differ by more than
   10 s (wrong; the control subjects' pairs counted apart);
4. *shared passage*: the plays overlapping the spans are the two
   records of one shared-material pair (wrong, by its label: related
   version or different tunes);
5. *other*: anything else (wrong).

Retrieval: a *positive* is two plays in different mixes of the same
recording whose source spans overlap by 30 s or more; it is found when
a right correspondence is assigned to it. Recall is reported for the
subjects' positives, by relative speed (up to 8%, above 8%), by mode
and range; with the times of found ones: the correspondence's span in
each mix against the true overlap there (median start and end error,
and the share of the overlap covered). Partners recurring across mixes
(whole mode) and the insert are reported apart. The negatives: the
control subjects' non-overlapping pairs (16 per batch), each linked or
not; the five shared-material pairs; and every pair of plays of
different recordings (an "other" link). Whole-track claims are not an
output.

**What would make it worth a design note and an ADR.** A design note
(not an ADR: this pilot changes no product behaviour) is worth writing
if, on both batches, in gaps mode at ±18%: (1) at least 80% of the
subjects' positives are found, including at least 70% of those above
8% relative speed; (2) no "other" correspondence, and at most 2 of the
16 control subjects' non-overlapping pairs linked per batch; and (3)
every shared-material link's span lies within the panel's passage
±15 s in both mixes (a passage claim, not a track claim). Shared-
material links are reported as wrong links and counted; their number
does not decide (1)-(3), because no audio evidence tells a shared
passage from a recurring one (the proposals note). The ±18% range is
reported as affordable if its pairwise search costs at most 3 times
the ±8% range's CPU. An ADR would follow only a later decision to put
such links in the product's output.

## Decisions

- **(05:38) Start build and regressions** (`work/s69/start.sh`, tmux
  `s69start`, logs in `work/s69/logs/`; `run.sh` and `same.py` are
  session 68's with the paths changed). The start build is
  byte-identical to `s68-final`. Both regressions identical (0:51;
  4:01), every report equal to its baseline's apart from
  `wall_seconds`.
- **(05:45) Pre-registration committed** (`084d659`), before the
  prototype and before any batch run.
- **(05:45) No NAS-scale batch.** The prompt asks for 262 tracks
  first; at NAS scale the subjects' other rips would have to be left
  out through `clusters::find_around`, which the time box does not
  allow. The NAS is read only to render the shared-material pairs.
- **(05:47-05:55) The prototype** (`passages.rs`, the subcommand in
  `commands/mixes.rs`; `mixes::crossfade` made visible to the harness,
  unchanged). The NAS root option is `--other-root`, the name `mixes`
  already uses, not `--other-library`. A smoke run on seed 6900 (not a
  result) took 1:17 and 2.4 GiB at 32 workers and showed the code
  classifying as designed; it also showed that some subject plays lie
  wholly inside their partners' crossfades, so gaps mode cannot reach
  them (a property of the registered construction; nothing changed).
  Each correspondence's reach past the panel's passage is the larger of
  its two ends' (the registered "within the passage ±15 s").
- **(05:55) Eight more batches.** A batch costs under 2 minutes, so
  seeds 6903-6910 run as well, after the two registered ones, with the
  same build and rules. They are reported beside seeds 6901 and 6902;
  the bar is read on 6901 and 6902 as registered, and the pool of ten
  is reported apart.
- **(05:55) The queue** (`work/s69/queue.sh`, tmux `s69queue`): the
  run build `work/bin/s69-run/` from `ebdb991` (its `gunfinger`
  byte-identical to `s68-final`'s), both regressions (identical, 0:51
  and 3:53, every report equal to its baseline's apart from
  `wall_seconds`), the ten batches (1:15-1:22 each), then
  `scripts/check.sh`.
- **(06:14) Results.** By the registered bar the pilot fails (1): in
  gaps mode at ±18%, 30 of 60 subjects' positives on seeds 6901-6902
  (23 of 43 above 8%); (2) and (3) hold; the ±18% range costs 2.33 times
  the ±8%'s thread seconds. Experiments 0267 (`8911e52`) and 0268
  (`357baf5`). Whole mode, reported beside as registered, finds 48 of
  60 (266 of 312 over ten batches); it decides nothing.
- **(06:16) The proposals note's §6 and the roadmap's "Time without a
  confident match"** gained the pilot's result and the next step it
  suggests (whole mixes, archive-scale cost); no design note and no ADR.
- **(06:17-06:24) The review** (a read-only subagent) found no product
  change and no error in the batch draw or the scoring, and confirmed
  most numbers. Its findings and what was done:
  (1) the 77 plays without unidentified time are not all under
  crossfades: 28 lie inside their neighbours' crossfades, 49 leave at
  most 21 s between them, under the 30 s minimum (corrected in 0267 and
  the proposals note; the next step now names a shorter minimum too);
  (2) the 12 s insert cannot pass the rule's windows or be a positive
  (30 s), so as built it could not test bridging: a limit of the
  registered design, now stated in 0267 and the proposals note;
  (3) the record's own repeats are an inference (marked);
  (4) deviations not recorded until now: partners' positives were
  registered as reported apart (row added to 0267); rendering,
  decoding and the 262-track searches are timed as one figure, and each
  mix is decoded twice (stated in 0268); the shared-material records'
  clusters are left out of the index too (as "controls" in the
  registration's left-out sentence); subjects are drawn from files of
  at least 330 s, and their occurrences fit the shortest file of the
  cluster, not the cluster's first file; the unidentified spans come
  from confident detections and 30 s, not the product's gaps (plays'
  segments, 45 s), now worded so in the roadmap;
  (5) the shared links' reach past their passage is 0.2 s (gaps) and
  12.3 s (whole), not "within" (corrected);
  (6) `CLAUDE.md` named brief 68 for the rules (fixed);
  (7) the speed fields' conventions were ambiguous (documented; numbers
  unaffected);
  (8) the pooled median share is 58%, the render figure 28.0 s, memory
  in GiB, the batches' length 2.6-2.85 h (corrected);
  (9) the cost inference about query length was unsupported (replaced
  by what was measured);
  (10) house style: the control flag is an enum (`Excerpts`),
  `pairwise` returns a named struct, `draw_batch` is split into
  `subjects`, `shared_items`, `partner`, `joined` and `laid_insert`,
  the pieces' lookup returns an error instead of panicking, and a
  failed deletion of the renders is an error;
  (11) `correspondences` depends on the links' order within one pair of
  mixes (documented; no fixed point, as registered);
  (12) the speed test's fixture is 60 s instead of 90 s.
  The refactor changes no draw or score: the final build's report of
  seed 6901 is compared with the run build's below.
- **(06:27-06:41) Final checks** (`work/s69/final.sh`, tmux
  `s69final`): `work/bin/s69-final/` from `e9dda8a`; seed 6901 again
  gives the run build's report apart from timings and the flag's name;
  both regressions identical (0:52; 3:59); `scripts/check.sh` green
  (7:36); the digests unchanged. Session closed.
