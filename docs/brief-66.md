# Session 66 brief: does a harmonic view tell a true play from a related version that shares its drums? (proposal 5, a diagnostic pilot)

Written on 2026-10-31, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file (its pre-registration above all), and
   `docs/session-66-checklist.md` (progress).
3. `docs/research/2026-10-29-matching-and-archive-proposals.md` (§5,
   "Priorities and observations", "Evaluation and adoption
   boundaries"); `docs/research/2026-10-16-version-verification.md`;
   experiments 0054-0058, 0125-0127, 0133-0135, 0151 and 0208; ADR 0017
   and 0022; the newest four entries of `docs/status.md`.
4. `docs/session-notes.md` ("Session 26", "Session 30", "Session 65");
   this session's experiments (0259 and up).

`docs/brief-2.md` to `docs/brief-65.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 66 of Gunfinger: does a harmonic view tell a true play from a related version that shares its drums? A diagnostic pilot of proposal 5 of `docs/research/2026-10-29-matching-and-archive-proposals.md`. Related versions gain alongside true plays under every extra-evidence idea measured so far (tolerant pairs 0151, wider fan-out 0208, the peak verifier 0056 and 0058), and band-energy version verification lost true plays (0125-0127, 0133-0135). On 2026-10-30 the user confirmed by ear that four candidate pairs are different titles (docs/status.md, "the listening of the 2026-10-30 pack"): Aphrodite - Cool Rock Stuff against the Mobb Deep / Urban Takover upload (whose detections are the wrong confident answers left at NAS scale and calibrate's strongest false, 282), Tower Bass (Millennium Mix) against two other versions, and the two Stalker versions. With the earlier verdicts (Wikki Wikki Plate ~ Defjammer: one drum loop, different tracks; We R The Ones VIP ~ the Klute Remix; Konflict - Pendulum ~ the Paragliders remix) and the shared-material panels, these are judged hard negatives that share drum material with the played recording. The proposal: separate a candidate's query and reference into harmonic and percussive parts by median filtering across time and frequency (FitzGerald 2010, §2; no neural model), and ask whether a candidate found through drums also matches sustained material at the same alignment. This session measures that as a candidate-only diagnostic, against simply denser query peaks without separation. It is a measurement session: the index, the search and the product do not change.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).

Time box: started about 00:05; aim to finish within about 3 hours, because other sessions follow you tonight. If a step would run past that, scale it down (262 tracks and the judged pairs first, fewer damage conditions), record the decision in the brief and continue.

Environment (sessions 33-65's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,674 records, revision `7210fc479da58376` (session 64 pruned four ignored uploads). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-54-linux` (262 tracks) and `session-64-linux-nas` (NAS scale; compare with it, not `session-54-linux-nas`).
- Build `work/bin/s65-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Since `45f92d5` (2026-10-28) its calibrate reads only the reports its baseline holds; still run variants and other seeds in a scratch work directory, never in the reports directory `regress` compares.
- The user restructured the code on 2026-10-28 without meaning to change behaviour (commits `b5a84e3` to `f94604a`: harness subcommands in modules, named results, `Search` passed through the core's search, enums for closed sets). Session 60 confirmed both regressions identical on a build of master after it.
- Since sessions 53-56: `SpansAtSpeed(0.006)` is the default second suppression (ADR 0021); same-title detections (`docs/same-titles.txt`) are scored right (ADR 0022); `identify` prints gaps as "ID - ID" lines with an estimate of their tracks (ADR 0023, 0024). The minimum span stays undecided. A change to an existing decision needs a new ADR (the next number is in `docs/adr/`).
- The research note `docs/research/2026-10-29-matching-and-archive-proposals.md` (2026-10-29) records six unmeasured proposals and its evaluation boundaries; any session working on one follows its "Evaluation and adoption boundaries".
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen. On the evening of 2026-10-30 the user answered 8 of 12 questions (docs/status.md, "the listening of the 2026-10-30 pack"): the four same-title candidates (Cool Rock, Tower Bass twice, Stalker) are different titles; Vintage Dread 2000 disc 1 is separate tracks; Advent, Keltech - Axis and Plasmic - The Weapon have an inverted channel, We - Hello World is unsure. The pitch rides (Star Trails, Into Music) and the Planet Dust cut stay unanswered. When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
- The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-26.md and the newest brief, docs/session-notes.md ("Session 26", "Session 30" and the newest session), docs/status.md (the newest entries), the proposals note (§5, "Priorities and observations" and "Evaluation and adoption boundaries"), `docs/research/2026-10-16-version-verification.md`, ADR 0017 and 0022, experiments 0054-0058, 0125-0127, 0133-0135, 0151 and 0208, the panels in `docs/panels/` and their newest experiments, `docs/calibration.md`, and the code of the spectrogram and peak picking, `verifier.rs`, and how `pair`, `shared` and `robust` place a candidate's alignment.
2. Write docs/brief-66.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-66-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s66/listening-digests-start.txt. Experiments number from 0259.
3. Build `work/bin/s66-start`; `regress session-54-linux` and `regress session-64-linux-nas` identical.

Steps:

1. Pre-registration, committed before any harmonic-view run: the separation (median filter lengths across time and frequency, the mask rule, fixed before any data is seen and justified from the frame rate and the music's tempo range); the feature (for example peaks and pair hashes of the harmonic part, counted at the candidate's own alignment and speed only, so the view never searches); the positives (true plays at hit counts similar to the negatives': `robust`'s damaged conditions, other rips, generated blends, and bass kills where the generator allows, since a DJ's bass kill must not read as a contradiction); the negatives (the judged pairs above, the shared-material panels' related versions, calibrate's strongest false candidates at both scales), each case placed by the alignment today's search finds; the comparison arm (denser query peaks without separation, at the same alignment); the families split into a development part and a frozen part chosen before any run (a recording family is the played recording with its related versions; fresh seeds of one family are not independent); the score (true plays retained at a fixed wrong-answer rate, split by family, with the harmonic view, with denser peaks, and with neither); and what would justify a later evidence rule and an ADR (separation by the harmonic view where the denser arm does not, holding on the frozen families). Identical audible passages remain ambiguous by construction: say how such cases are scored.
2. The diagnostic, tested, harness only (an opt-in subcommand or option; `regress` identical with it off): unit tests on synthetic signals (a sustained tone over clicks: the harmonic part keeps the tone and drops the clicks, and the percussive part the reverse; the default path untouched).
3. The runs at 262 tracks and at NAS scale (reference audio is read from the corpus library or the NAS, read only; never write any store), under tmux with logs where long, development families first, the frozen families once, with the rule fixed.
4. Experiments (at most 40 lines each, written before the next starts): the development families against both arms; the frozen families; the cost (decoding reference audio, the separation, memory).
5. A short decision note (`docs/research/2026-10-31-harmonic-view-decision.md`): what the view separates and what it does not, the cost (including that a product use needs reference audio or a new stored field, each with its own ADR), the risks, and a recommendation that is ours to take. Update the roadmap's matching bullets and the proposals note's §5 with a pointer.
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 66" section at the end of docs/session-notes.md with a handoff; update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-54-linux` and `regress session-64-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: separation on the development and frozen families against the denser arm, the Cool Rock pair on its own line, the cost, the recommendation, and anything left.

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
- Scratch data goes under work/s66/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00 on 2026-10-31. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.

```

## State at the start of session 66

- 2026-10-31, 00:03 by the Linux machine's clock (CET). The time box is
  about 3 hours: aim to finish by about 03:05. The soft deadline is
  11:00 on 2026-10-31; no new run starts after about 10:20.
- `master` at `1772995`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s66/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `c4d3da2e7e37...`, `work/audition/answers.jsonl` `3dc509cb1ec9...`.
- Experiments 0001-0258; this session numbers from 0259. ADRs
  0001-0024. Test set: 4 of 5 evaluations used (none in this session).
- The NAS store: 40,674 records, revision `7210fc479da58376`. Builds:
  `work/bin/s65-final/` (master). Baselines `session-54-linux` and
  `session-64-linux-nas`.
- The shared-material panels: the first panel's development half
  (`docs/panels/shared-material-seed-2026-development.json`: Dozer,
  Fire, Playground Shades, Pyro, Sonar, Watermelon) and the second's
  (`shared-material-2-seed-2026-development.json`: Phantom Force, Star
  Trails, We R The Ones). Their validation halves stay unsearched
  (briefs 20-30, the proposals note's boundaries). Most of their files,
  and the related file of every judged pair, are NAS files.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build and both regressions (tmux `s66start`) | 00:03-00:25 |
| 1 | pre-registration (below), committed before any harmonic-view run | 00:25-00:45 |
| 2 | the harness: `shared --views`, unit tests on synthetic signals; `check.sh`; `regress` identical with it off | 00:45-01:40 |
| 3-4 | development families (0259); frozen families (0260); cost (0261) | 01:40-02:30 |
| 5 | decision note; roadmap; proposals note §5 | 02:30-02:45 |
| 6-7 | review, notes, status, checklist, final checks | 02:45-03:20 |

## Decisions

- **(00:06) Start build and regressions** (`work/s66/start.sh`, tmux
  `s66start`, logs in `work/s66/logs/`; `run.sh` and `same.py` are
  session 65's with the paths changed).
