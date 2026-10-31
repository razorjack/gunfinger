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

## Pre-registration (step 1; written at 00:09-00:14, before any harmonic-view run)

**Question.** For a candidate the unchanged search finds, at the
alignment and speed it finds, does the candidate's sustained material
(its harmonic part) match the query's better for a true play than for a
related version that shares the played recording's drums, at similar
hit counts, and better than denser peaks of the whole spectrum do?

**Where the cases come from.** `gunfinger-eval shared` (the
shared-material scenarios: a passage two judged-different recordings
share, queries cut from the played file, searched with everything
indexed, with the played recording left out and, as a positive control,
with the played file alone left out while another rip stays), with a new
opt-in option `--harmonic`. Every detection of at least 20 hits is a
case, placed by the line through its ends as the search reports it
(file time = `track_start + slope x (t - start)`, the verifier's
`Line`). The view never searches: no offset, speed or line is fitted.
With `--harmonic` the scenarios also get three damaged queries, the
positives at lower hit counts: `killed-alone-30s` and
`killed-beyond-20s` (the 30 s alone and 20 s beyond queries through a
DJ bass kill, FFmpeg `highpass=f=250,highpass=f=250`, 24 dB an octave
below 250 Hz), and `damaged` (robust's combined damage from the
passage's start: wow, the plan's blend partner beatmatched at -6 dB,
broadcast processing, AAC 64 kbit/s, at turntable speed 1.03, so the
view's render at the detection's speed is exercised). The default
queries are as today.

**Plans** (written into `work/s66/plans/`, not committed panels; the
first and second panels' development halves, their scenarios copied
unchanged, and the judged pairs, each direction a scenario, the passage
the strongest alignment of `pair` at the fitted speed, the panels'
rule; the blend partner Decoy at 60 s as in both panels):

| Part | Family | Scenarios | Passage source |
|---|---|---|---|
| development | dozer, fire, pyro, sonar | first panel, development half | the panel |
| development | phantom-force, we-r-the-ones | second panel, development half | the panel |
| development | tower-bass | Millennium Mix ~ `a - aphrodite - tower bass.mp3` and ~ the JumpUpThrowDown version, both directions (4) | `work/listening-2026-10-30/pair.log` |
| development | wikki-wikki | Wikki Wikki Plate ~ Defjammer, both directions | `work/s66/logs/pair-judged.log` |
| frozen | playground-shades, watermelon | first panel, development half | the panel |
| frozen | star-trails | second panel, development half | the panel |
| frozen | cool-rock | Cool Rock Stuff ~ the Mobb Deep / Urban Takover upload, both directions | the pack's `pair.log` |
| frozen | stalker | APH029 ~ the album version, both directions | the pack's `pair.log` |
| frozen | konflict | Konflict - Pendulum ~ the Paragliders remix, both directions | `pair-judged.log` |
| frozen | clockwork | Clockwork ~ its Stakka remix (calibrate's strongest false at 262 tracks, 99-100 hits), both directions | `pair-judged.log` |

The split was chosen before any run: each part mixes panel families and
judged pairs; Fire, the largest family, develops; the Cool Rock pair
(calibrate's strongest false at NAS scale, 282) is frozen so that its
line is a clean test. Clockwork's family is in the first panel's
validation half: none of that half's scenarios is used; its scenario
here comes from `pair`, and it is used once, frozen, because the prompt
names calibrate's strongest false at both scales. A family's scenarios
(both directions, every query and index) are one unit: fresh queries of
one family are not independent.

**Scales.** NAS scale (`--other-peaks-dir`, the panels' files are NAS
files) is the primary run. At 262 tracks the same plans run beside
(scenarios whose played file is a NAS file are reported unrendered;
the only related pair indexed at 262 tracks is Clockwork's).

**The separation** (FitzGerald 2010, §2; fixed now, before any data).
The query and the candidate's reference are analysed with the
profile's STFT (8 kHz, 1,024-sample Hann window, hop 128: 16 ms frames,
7.8 Hz bins). For each bin the magnitude is median-filtered across time
over 25 frames (400 ms) and, for each frame, across frequency over 17
bins (133 Hz). At 160-180 BPM a beat lasts 333-375 ms (21-23 frames) and
a sixteenth 83-94 ms; a drum hit's energy, smeared by the 128 ms
window, covers about 4-8 frames, so a single hit occupies well under
the 13 of 25 frames a time median needs to pass it, while a held note,
pad or bass lasting a beat or more passes. A partial's main lobe covers
about 4 of 17 bins, so the frequency median rejects it, while a
broadband hit (snare, hats) covers the whole 17. Mask: FitzGerald's
soft mask with power 2, `W_h = H^2 / (H^2 + P^2)` from the
time-filtered `H` and frequency-filtered `P`; the harmonic part is the
complex STFT times `W_h`, the percussive part times `1 - W_h`, each
turned back into audio by overlap-add (session 65's resynthesis), so
that the core's unchanged peak picker reads it. Known limit: bass notes
whose partials lie closer than about 2 bins apart (below about 60 Hz)
are partly classed percussive.

**The candidate's reference.** The detection's file decoded as indexed
(the sample-counted timeline of its peak record; 0127) and played at
the line's slope as a turntable (a key-locked detection: key-locked at
that tempo), from 8 reference seconds before the span to 8 after, and
decoded at 8 kHz by the same FFmpeg path as the query (a WAV in the
render directory, deleted once decoded). Reference audio is read from
the corpus library or the NAS, read only.

**The feature and its arms**, each counted at the candidate's own
alignment only, query frame `f` against reference frame `f` of the
render (bins already scaled by the render's speed):

- **H, the harmonic view**: peaks of the harmonic part of the reference
  and of the query, the core's picker with an 8 x 8 neighbourhood
  (`neighbourhood_frames` = `neighbourhood_bins` = 8, the rest of the
  profile unchanged).
- **D, the comparison arm**: peaks of the whole (unseparated) reference
  and query with the same 8 x 8 picker: denser than today's 12 x 12 (as
  session 65's denser arm), the only difference from H being the
  separation.
- **P, the percussive part** (beside, deciding nothing): the same on
  the percussive part.
- **Neither**: the detection's hits (today's evidence), with the peak
  verifier's excess share (12 x 12, the stored peaks; `shared` measures
  it already) beside.

For each view: the reference's peaks inside the span, the share found
among the query's peaks of the same view within +-1 bin and +-1 frame
(the verifier's tolerance, 0054), and the same share with the reference
shifted -6.2, -5, +5 and +6.2 reference seconds (the verifier's chance
shifts). The score is the **excess share**: the share minus the mean of
the four shifted shares. A view with fewer than 50 reference peaks in
the span abstains (unobservable: little sustained content, a kill); an
abstaining case is counted apart and kept, never rejected.

**Positives and negatives.** Positive: a case of the played recording
(role `played`: the played file, or its other rip in the positive
control) or of the blend partner (role `partner`) in a query where it
plays. Negative: a case of the scenario's related recording (role
`related`): judged a different recording, sharing material with the
played one. Cases of any other file (role `other`) are reported apart
(not judged by ear; unnamed versions and chance among them). **At
similar hit counts**: the scores are taken over the positives whose hits
lie within the range of that part's negatives' hits (and at least 20),
pooled and per hit band (20-59, 60-239, 240 and more).

**Identical audible passages.** Where the related file's audio is as
close to the query as another rip of the played recording is, no view
can tell them apart by construction. A negative whose D excess share
reaches the 5th percentile of the same part's other-rip positives'
(index `source left out`, role `played`) is marked *near-identical*. It
stays a negative in the primary score; a secondary score leaves the
near-identical negatives out. The marking uses D, not H, and decides
nothing.

**The score.** For each measure (H, D, hits; P and the verifier
beside), on the development part: the threshold at a wrong-answer rate
of 0 (just above the strongest development negative) and of 10% (the
90th percentile of the development negatives); a case is retained when
its score is above the threshold or its view abstains. Reported: the
hit-matched positives retained at each threshold, pooled and by family;
the area under the ROC curve (positives against negatives, hit-matched),
pooled and per hit band; the killed positives retained apart. The
thresholds are frozen when experiment 0259 is written. On the frozen
part, with the development thresholds unchanged: negatives above the
threshold (the wrong-answer rate reached), hit-matched positives
retained, by family; the Cool Rock pair on its own line. The 262-track
run is reported beside.

**What would justify a later evidence rule and an ADR** (reading
rules, not adoption thresholds). All of: (1) on the development part,
at the rate-0 threshold, H retains at least 10 percentage points more of
the hit-matched positives than D, and H's pooled AUC exceeds D's by at
least 0.05; (2) on the frozen part, with the development thresholds, H's
wrong-answer rate is at most D's and at most 10%, and H retains more
hit-matched positives than D; (3) under the bass kill, H retains no
fewer than 10 percentage points below D's share of the killed positives.
Then the next step would be a registered evidence-rule experiment on
fresh families and real mixes; a product use would need reference audio
at identification time or a new stored field, each with its own ADR.
Otherwise the view is not proposed.

**Not done.** No index, search, rule or product change; the view is a
harness option (`regress` identical with it off). Real mixes are not
searched in this pilot (the three development mixes' true plays are
the binding positive control of any later rule, 0116, 0118, 0125).

## Decisions

- **(00:06) Start build and regressions** (`work/s66/start.sh`, tmux
  `s66start`, logs in `work/s66/logs/`; `run.sh` and `same.py` are
  session 65's with the paths changed).
- **(00:11) Pair alignments for three judged pairs** (`work/s66/pair.sh`,
  `pair` at NAS scale in the scratch work directory `work/s66/wn`):
  Wikki Wikki Plate ~ Defjammer 110.6-185.7 s (58 hits at the fitted
  speed), Konflict - Pendulum ~ the Paragliders remix 22.7-154.0 s (371
  hits, speed 0.9735), Clockwork ~ its remix 205.4-218.5 s (98 hits).
- **(00:13) The start build is byte-identical to `s65-final`; both
  regressions identical** (0:51; 3:48), every report equal to its
  baseline's apart from `wall_seconds`.
- **(00:14) Pre-registration committed** (above), before any harmonic
  view runs.
- **(00:16) The harness** (`shared --harmonic`; `harmonic.rs`; report
  `<plan>-harmonic.json`; the views' queries change the render key only
  with the option). The query's views are computed once per query, the
  reference's per detection. Unit tests on synthetic signals: a tone over
  clicks (the harmonic part keeps over 80% of the tone's power and under
  10% of the clicks', the percussive part the reverse), the parts adding
  up to the audio, identical audio found at the alignment and not at the
  shifts, and a shared drum track under other notes found in the
  percussive part only. A one-sample click has a flat spectrum, whose
  local maxima are set by whatever else plays, so that test's drums are
  a fixed 6 ms burst with a spectral shape of its own; its percussive
  bound is an excess share over 0.3 (0.46 measured; the notes' changes
  are transients that differ).
- **(00:20) Smoke run** (one development scenario, `wikki-wikki-1`, 262
  tracks, 0:38): the views measure every case. It showed that
  Defjammer is indexed under its corpus copy
  (`extra/APH029-.../01-aphrodite-defjammer-bpm.mp3`), so a plan naming
  the NAS path gives its detections the role `other`. **The plan names
  the corpus copy** (the panels' convention; the library map lists no
  other plan file with a corpus copy). The smoke report was deleted.
- **(00:20) A beside line, decided after the smoke run and before any
  development run:** in looped queries a detection's line through its
  ends can cross the loop's repetitions (the played file found 0.03 of
  its peaks over a 120 s loop, against 0.81 over 60 s), so every view
  misplaces there alike. The registered scores keep every kind; the
  AUC without the `looped` and `break` kinds is printed beside and
  decides nothing.
- **(00:23) Harness committed** (`7604a41`); the CLI binary is
  byte-identical to `s65-final`'s. Development runs (`work/s66/dev.sh`,
  tmux `s66dev`, `work/bin/s66-run`: NAS scale in `work/s66/wn`, then
  262 tracks in `work/s66/wc`) and the regressions with the harness
  build and `check.sh` (`work/s66/regress.sh`, tmux `s66reg`) started.
  The analysis: `scripts/analysis/harmonic_views.py`.
- **(00:34) Step 2 verified:** with `work/bin/s66-run` (the harness
  build, `--harmonic` off) `regress session-54-linux` (1:00) and
  `regress session-64-linux-nas` (4:15) identical, every report equal
  to its baseline's apart from `wall_seconds`; `scripts/check.sh` green
  (5:47, beside the development run).
- **(00:45) The NAS development run split in four.** One process ran
  at about 4.2 cores and 3.7 minutes a scenario (the long looped
  queries and their many candidates are measured one after another in
  one worker), which would have ended the development part near 02:00
  and the frozen part near 03:00. It was stopped after 22 minutes (6 of
  26 scenarios rendered, no report written) and restarted as four
  processes over four parts of the plan (`work/s66/plans/dev-a.json` to
  `dev-d.json`, every fourth scenario; tmux `s66deva` to `s66devd`),
  with the 262-track run beside (tmux `s66c262`). Each scenario is
  searched on its own indexes, so the split changes no result. The
  frozen plan is split the same way (`frozen-a.json` to `frozen-d.json`).
- **(01:21) Experiment 0259 written; thresholds frozen** (in
  `docs/experiments/data/0259-harmonic-view-development.json`, copied
  from `work/s66/thresholds-dev.json`). Reading rule (1) is not met on
  the development part (H +3.6 points over D at T0, AUC 0.579 against
  0.684); the frozen part runs once as registered.
