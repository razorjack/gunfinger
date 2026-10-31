# Session 67 brief: inverted rips in the store, and a check at indexing that could repair their records

Written on 2026-10-31, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file (its pre-registration above all), and
   `docs/session-67-checklist.md` (progress).
3. `docs/research/2026-10-30-stereo-power-decision.md`; the proposals
   note `docs/research/2026-10-29-matching-and-archive-proposals.md`
   (§1, "Evaluation and adoption boundaries"); the roadmap's "Stereo
   faults before the downmix"; `docs/brief-62.md`, `docs/brief-42.md`;
   ADR 0003 (the profile's invalidation rule).
4. `docs/session-notes.md` ("Session 42", "Session 61", "Session 62",
   "Session 66"); experiments 0194-0198, 0245-0250 and this session's
   (0262 and up).
5. `crates/gunfinger-eval/src/stereo.rs` (`channels`, `Class`,
   `build_store`), `commands/library.rs` (`stereo-width`,
   `variant-store`), `crates/gunfinger-core/src/decode.rs`.

`docs/brief-2.md` to `docs/brief-66.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 67 of Gunfinger: inverted rips in the store, and whether a check at indexing could repair their records. Session 62's census (experiment 0248) decoded a stratified sample of 2,000 NAS files and flagged about 0.4% as having one channel inverted (an estimated 166 files in the store) and one as delayed. FFmpeg mixes every file down to mono before peaks are picked (`-ac 1` in `decode.rs`), so an inverted rip's record holds mostly the side signal: session 42 measured that a fault in the indexed rip costs as much as one in the query (0197). On 2026-10-30 the user confirmed three flagged rips by ear (docs/status.md, "the listening of the 2026-10-30 pack"): Usual Suspects - Advent, Keltech - Axis (Low Level Mix) and Plasmic - The Weapon are inverted; We - Hello World is unsure. Session 62 recommended stopping the query-side stereo work, because power-summed queries lost confident answers on wide sources. This session measures the narrower record-side idea, which session 62 did not: at indexing, decode two channels, and only when a file's channels are strongly anti-correlated (a rule fixed from the census before any search), downmix with one channel's polarity reversed; every other file's record stays byte-identical. It is a measurement session: the front end, the store and the product do not change; adoption would need a profile decision and an ADR, which are ours.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).

Time box: started about 02:10; aim to finish within about 2.5 hours, because another session may follow you tonight. If a step would run past that, scale it down (the flagged files and a smaller census first), record the decision in the brief and continue.

Environment (sessions 33-66's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,674 records, revision `7210fc479da58376` (session 64 pruned four ignored uploads). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-54-linux` (262 tracks) and `session-64-linux-nas` (NAS scale; compare with it, not `session-54-linux-nas`).
- Build `work/bin/s66-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Since `45f92d5` (2026-10-28) its calibrate reads only the reports its baseline holds; still run variants and other seeds in a scratch work directory, never in the reports directory `regress` compares.
- The user restructured the code on 2026-10-28 without meaning to change behaviour (commits `b5a84e3` to `f94604a`: harness subcommands in modules, named results, `Search` passed through the core's search, enums for closed sets). Session 60 confirmed both regressions identical on a build of master after it.
- Since sessions 53-56: `SpansAtSpeed(0.006)` is the default second suppression (ADR 0021); same-title detections (`docs/same-titles.txt`) are scored right (ADR 0022); `identify` prints gaps as "ID - ID" lines with an estimate of their tracks (ADR 0023, 0024). The minimum span stays undecided. A change to an existing decision needs a new ADR (the next number is in `docs/adr/`).
- The research note `docs/research/2026-10-29-matching-and-archive-proposals.md` (2026-10-29) records six unmeasured proposals and its evaluation boundaries; any session working on one follows its "Evaluation and adoption boundaries".
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen. On the evening of 2026-10-30 the user answered 8 of 12 questions (docs/status.md, "the listening of the 2026-10-30 pack"): the four same-title candidates (Cool Rock, Tower Bass twice, Stalker) are different titles; Vintage Dread 2000 disc 1 is separate tracks; Advent, Keltech - Axis and Plasmic - The Weapon have an inverted channel, We - Hello World is unsure. The pitch rides (Star Trails, Into Music) and the Planet Dust cut stay unanswered. When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
- The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-42.md, docs/brief-62.md and the newest brief, docs/session-notes.md ("Session 42", "Session 61", "Session 62" and the newest session), docs/status.md (the newest entries), the stereo decision note `docs/research/2026-10-30-stereo-power-decision.md`, experiments 0194-0198 and 0245-0250, the roadmap's "Stereo faults before the downmix", ADR 0002 or whichever ADR fixes the front end and the profile (and `hash::design()`'s invalidation rule), session 62's census scripts and summary under work/s62/ (`census.sh`, `census-summary.json`), and the code of `decode.rs`, `robust.rs` (the `stereo-*` conditions) and how the harness builds a store from audio.
2. Write docs/brief-67.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-67-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s67/listening-digests-start.txt. Experiments number from 0262.
3. Build `work/bin/s67-start`; `regress session-54-linux` and `regress session-64-linux-nas` identical.

Steps:

1. Pre-registration, committed before any search: the detector (the census's measures, correlation and side-to-mid power, over the whole file or a fixed set of windows; one threshold chosen from session 42's synthetic faults and the census's distribution, without using the user's answers to tune it, which serve as a check); the repair (polarity-reversed downmix of the flagged file only; say what happens with a file inverted only partway); the false-positive question (on a larger seeded census than 0248's, for example 6,000 NAS files stratified as before, how many files the detector would change, and how many of those are wide-stereo recordings rather than faults: list them for later listening, never judge them by ear); the recovery test (queries that carry the recording's correct polarity: the flagged rips repaired and rendered into generated mixes and `robust`-style excerpts, searched against a scratch copy of the affected records built today's way and built repaired, plus other rips of the same recording where the library holds them); the controls (unflagged files must give byte-identical records; no new wrong confident answer); the scores (confident and possible recall on the flagged recordings, wrong confident answers, records changed); and what would justify an ADR proposal.
2. The detector and the repair as an opt-in harness option (harness only; `regress` identical with it off), tested: unit tests that an inverted channel is detected and repaired on fixed samples, that a mono or ordinary stereo file is left unchanged, and that the default path is untouched.
3. The runs: the census extension first (decoding only, read only, never on `EM_1999_02_07_-_Ed_Rush_Optical.mp3`, files the ignore file leaves out, or experiment 0065's list; kept off any timing run), then the recovery test with scratch stores under work/s67/ only. Never write `work/peaks` or the NAS store.
4. Experiments (at most 40 lines each, written before the next starts): the detector's census and false positives; the recovery and controls.
5. Update the stereo decision note with a "Session 67" section (what the record-side check recovers, what it would change in the store, how adoption would work: a profile bump or a per-record flag, the files re-indexed, the time, and a recommendation that is ours to take), the roadmap's "Stereo faults before the downmix" and the proposals note's §1.
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 67" section at the end of docs/session-notes.md with a handoff and the files for later listening; update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-54-linux` and `regress session-64-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: the detector's census and false positives, the recovery on the flagged recordings, the controls, the recommendation, and anything left.

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
- Scratch data goes under work/s67/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00 on 2026-10-31. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 67

- 2026-10-31, 02:08 by the Linux machine's clock (CET). The time box is
  about 2.5 hours: aim to finish by about 04:40. The soft deadline is
  11:00 on 2026-10-31; no new run starts after about 10:20.
- `master` at `762983b`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s67/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `c4d3da2e7e37...`, `work/audition/answers.jsonl` `3dc509cb1ec9...`.
- Experiments 0001-0261; this session numbers from 0262. ADRs
  0001-0024. Test set: 4 of 5 evaluations used (none in this session).
- The NAS store: 40,674 records, revision `7210fc479da58376`. Builds:
  `work/bin/s66-final/` (master). Baselines `session-54-linux` and
  `session-64-linux-nas`.
- Session 62's census (0248; `work/s62/census-summary.json`, report
  `work/s62/w/reports/library-nas-dnb-peaks/stereo-width-other-seed-2026.json`):
  2,000 files, seed 2026, 667/667/666 by kind (scene releases, uploads,
  elsewhere), its exclusions in `work/s62/census-exclude.txt`. Lag-0
  correlations there: four files at -0.95 to -0.76 (Usual Suspects -
  Advent, Plasmic - The Weapon, Keltech - Axis, We - Hello World), then
  nothing until -0.32 (the 5 ms delayed `sety/` file) and -0.27 (another
  We track, wide); DJ Sense - Let You Go, counted inverted by its
  strongest lag (69 samples), is +0.62 at lag 0.
- By ear (2026-10-30): Advent, Keltech - Axis and Plasmic - The Weapon
  are inverted in one channel; Hello World is unsure.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build and both regressions (tmux `s67start`) | 02:08-02:20 |
| 1 | pre-registration (below), committed before any search | 02:12-02:15 |
| 2 | the harness: the detector and the repair, opt-in (`stereo-width` reports the verdict; `variant-store --repair-inverted`), unit tests; `regress` identical with it off | 02:15-02:55 |
| 3 | census extension (6,000 files, decoding only); then the recovery test and the controls with scratch stores under `work/s67/` | 03:25-04:05 |
| 4 | experiments (0262 and up) | with the runs |
| 5 | decision note "Session 67"; the roadmap; the proposals note's §1 | 04:05-04:20 |
| 6-7 | review, notes, status, checklist, final checks | 04:20-04:50 |

## Pre-registration (step 1; written at 02:12-02:15, before any search)

**Question.** At indexing, can a check of a file's two channels find the
rips whose channel is inverted and repair their records, with every
other record byte-identical, recovering the recording in mixes and
adding no wrong confident answer?

**The detector (frozen).** Each file decoded whole as two channels at 8
kHz (`stereo::decode_stereo`, as the census). Measures over the whole
file, as 0248's: the balance, the correlation at lag 0 (`correlation`),
the strongest correlation within ±80 samples and its lag, and the side
against the mid. A file is repaired when all hold:

1. the balance is within ±20 dB (not one channel);
2. the correlation at lag 0 is -0.5 or less;
3. the strongest correlation within ±80 samples is at lag 0.

The side-to-mid ratio is reported, not tested: with a negative
correlation the side is always louder than the mid, so it adds nothing
to 2. The threshold is the census's (`INVERTED_CORRELATION`, brief 62),
not tuned again: on session 42's synthetic faults it takes 96 of 100
inverted renders and 10 of 10 of 0197's inverted copies, and no clean,
silent, noisy or delayed render (condition 3 leaves out the one 5 ms
render at -0.55); in 0248's 2,000 files no correlation lies between
-0.76 and -0.32. The rule differs from 0248's `inverted` class only in
using lag 0 (the repair reverses polarity at lag 0): DJ Sense - Let You
Go (+0.62 at lag 0) is not repaired. The user's answers are a check,
not an input: by the rule, Advent, Keltech - Axis and Plasmic - The
Weapon are repaired, and so is We - Hello World (-0.76).

**How sure a repair is, by measure only.** A repair swaps the mid and
the side, so the repaired file's side-to-mid ratio is today's negated.
A flagged file whose side is 10 dB or more above its mid becomes narrow
or narrower when repaired, like 93% of the census's files: counted
*plainly inverted*. One with the side 0-10 dB above the mid stays wide
either way and could be a wide recording: counted *uncertain* and listed
for listening (Hello World, 8.6 dB, would be the one in 0248). The -10
dB boundary is brief 42's width class.

**The repair.** Only for a flagged file: peaks picked from (L - R) /
sqrt(2) of the two-channel decode, the downmix with one channel's
polarity reversed, at the downmix's scale. Every other file is decoded
by the core's `decode` (`-ac 1`) as today, so its record is
byte-identical. A file inverted only partway is judged whole: it is
repaired only if the whole file crosses -0.5, and then its correct part
becomes the side and is lost. The census also measures the correlation
at lag 0 in 30 s windows (the last kept from 10 s) and lists unflagged
files with any window at -0.5 or less as partly inverted candidates,
for listening; they are not repaired.

**The census extension (false positives).** `stereo-width --other
--seed 2026 --sample 6000 --exclude work/s62/census-exclude.txt`, 16
jobs, decoding only, read only, alone on the machine. With the same
seed the draw of each kind is a prefix-extension of 0248's (2,000 per
kind instead of 667), so 0248's files are included and 4,000 are new;
both are reported. Scores: files repaired, by kind and weighted to the
store (Wilson bounds, as 0248); plainly inverted and uncertain among
them; partly inverted candidates; the decode rate (files and hours of
audio per minute), the check's cost at indexing.

**The recovery test.** F = the files the rule repairs in the extension
(at most 16: every plainly inverted one, then uncertain ones by
correlation). For each f in F, the correct-polarity audio is f with
its right channel negated (FFmpeg `pan`); queries are rendered from it
as mono MP3s at 128 kbit/s and 44.1 kHz:

- excerpts: 30 s at 20, 40, 60 and 80% of the file, each at speed 0.96,
  1.00 and 1.04 (turntable: `asetrate`, then `aresample`): 12 a file;
- a generated mix: a corpus track drawn with seed 2026 (90 s), then f
  at speed 1.02 from 25% of its length (150 s), then another drawn
  track (90 s), 10 s crossfades;
- other rips: the NAS files confident in at least 3 of f's 4 excerpts
  at speed 1.00 against today's store (audio-derived; at most 3 per f,
  by path), each cut at 30 and 70% at the three speeds: 6 a rip. They
  are listed for listening; agents do not judge them.

Two scratch stores in `work/s67/stores/`: T, a reflink copy of the NAS
store (today's records), and R, T with the records of F replaced by
repaired ones. Both are searched by the CLI (`identify --config
nas-dnb.toml --peaks-dir <store> --store-only --saved-index off --format
json`, every query in one run). The affected records are also built
today's way from scratch copies of the files (`gunfinger index`), which
must equal the NAS records byte for byte, and repaired by `variant-store
--repair-inverted`.

**The controls.** (a) `variant-store --repair-inverted` over the corpus
library: each record byte-identical to `work/peaks`'s unless flagged;
(b) the same over a scratch library of F and 40 unflagged census files
(the 20 lowest correlations among the unflagged, DJ Sense and the
delayed file among them, and 20 drawn with seed 2026): every unflagged
record byte-identical to the NAS record; (c) `regress
session-64-linux-nas` against R in a scratch work directory, and the
three development scans (named) against T and R: identical, apart from
detections of F's files, which are listed.

**Scores.** Confident and possible queries naming f, under T and under
R, for the excerpts, the mix plays and the other rips' queries (each
apart); wrong confident answers: a confident file outside f and its
other rips, and which (query, file) pairs are new under R; records
changed (in the census, in the control sample, weighted to the store).

**What would justify an ADR proposal (all of):**

1. f's excerpts confident in 10% or fewer under T and 90% or more
   under R, over the plainly inverted files, and each one's mix play
   confident under R;
2. where other rips exist, 90% or more of their queries name f
   confidently under R (10% or fewer under T); unmeasured if none
   exist (stated);
3. no new wrong confident (query, file) pair under R in any search;
4. every unflagged record byte-identical in (a) and (b), and (c)
   identical apart from F's files;
5. of the files repaired in the census, at least 3 in 4 plainly
   inverted; the rest listed for listening.

If 1-4 hold and 5 does not, no proposal: the uncertain repairs need
listening first. The ADR itself (a profile bump or a per-record flag)
is ours; this session writes none.

## Decisions

- **(02:11) Start build and regressions** (`work/s67/start.sh`, tmux
  `s67start`, logs in `work/s67/logs/`; `run.sh` and `same.py` are
  session 66's with the paths changed). The start build is
  byte-identical to `s66-final`.
- **(02:15) Pre-registration committed** (`faeadee`), before any search;
  the times in its heading were corrected afterwards (no other change).
- **(02:16) Both regressions with the start build identical** (0:51;
  3:54), every report equal to its baseline's apart from
  `wall_seconds`.
- **(02:17) The harness** (`stereo.rs`, `commands/library.rs`):
  `stereo::inverted` (the detector), `decode_repairing` and
  `reversed_downmix` (the repair), `Polarity` for `build_store`
  (`variant-store --repair-inverted`, only under `--front-end downmix`;
  its report lists the repaired files), and `stereo-width`'s per-file
  `repaired`, `windows` and `inverted_windows` (30 s windows at lag 0).
  Unit tests: the detector on fixed samples (an inverted channel either
  side; mono, narrow, one-channel, silent, wide at about -0.3 and an
  inverted channel 5 ms late left alone), the repaired downmix, windows
  of a file inverted from 30 s, and on FFmpeg-generated files that a
  mono and an ordinary stereo file are decoded by the core's `decode`
  exactly, that an inverted file is repaired (90% or more of the clean
  file's peaks against under 50% today) and that the default front end
  decodes it as today.
- **(02:18) The census extension started** (`work/s67/census.sh`, tmux
  `s67census`, the run build `work/bin/s67-run/`, 16 jobs, alone), then
  both regressions with the run build.
- **(02:20) Scratch stores** T and R: reflink copies of the NAS store in
  `work/s67/stores/{T,R}/nas-dnb-peaks/` (the NAS store only read).
  A repaired record keeps today's header (path, size, mtime, profile),
  so a saved index of today's records would pass for R's (ADR 0011's
  provenance holds the sources and the profile, not the peaks): the CLI
  searches with `--saved-index off`, and the harness's work directory
  for R (`work/s67/wr`) starts without a saved index, so that its first
  run builds one from R's records. This also bears on adoption (a
  per-record flag must change something the provenance holds).
- **(02:21) The search tool for the recovery test.** `identify` takes
  every query in one run (`--save-dir`, one JSON report a query), so the
  index is built once per store; queries are rendered by
  `work/s67/queries.py` (checked on a corpus file first: a 30 s excerpt
  and a 310 s mix, mono at 44.1 kHz).
- **(02:28) Census extension done** (02:18-02:28, 9:43; 0262). A
  departure from the pre-registration: the draw is not a superset of
  0248's, because session 64's prune of 4 uploads changed the uploads'
  shuffle and so the next kind's; 1,234 of 0248's 2,000 files are in it
  (all 667 scene releases), Keltech - Axis and Plasmic - The Weapon not.
  Decided (conservative): estimates use the union of both censuses
  (6,766 distinct files), and F is every file the rule repairs in either
  report (the rule applied to 0248's channel measures, which it holds):
  Paradox - Filmscape (new), Advent, Plasmic - The Weapon, Keltech -
  Axis (plainly inverted) and We - Hello World (uncertain). The control
  sample (b) takes its 40 unflagged files from the new census, as
  registered, plus DJ Sense - Let You Go from 0248's (0248's `inverted`
  class, left alone at lag 0), 46 files in all.
- **(02:31) 0262 written** before the recovery test's runs.
- **(02:33) Both regressions with the run build identical** (0:52;
  3:48; the option off), every report equal to its baseline's apart
  from `wall_seconds`. The recovery test and the controls started
  (`work/s67/recovery.sh`, tmux `s67recovery`).
