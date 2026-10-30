# Session 65 brief: can a quiet track be recovered once the track over it is identified? (proposal 4, the oracle first)

Written on 2026-10-30, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file (its pre-registration above all), and
   `docs/session-65-checklist.md` (progress).
3. `docs/research/2026-10-29-matching-and-archive-proposals.md` (§4,
   "Automated counterexample search", "Evaluation and adoption
   boundaries"); experiments 0182-0185, 0207, 0208, 0215-0218; ADR
   0006, 0017 and 0019; the newest four entries of `docs/status.md`.
4. `docs/session-notes.md` ("Session 39", "Session 48", "Session 64");
   this session's experiments (0255 and up).

`docs/brief-2.md` to `docs/brief-64.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 65 of Gunfinger: can a quiet track be recovered once the track over it is identified? A pilot of proposal 4 of `docs/research/2026-10-29-matching-and-archive-proposals.md`, oracle first. In double drops with a quiet partner, the partner is lost mainly by the search; brief partners are more often found as possible and then left out by ADR 0019 (experiment 0184): two different failures, scored apart. Lost peak endpoints dominate combined damage (0207), and wider query pairing helped related versions more than true plays (0208). The proposal: once track A is confidently identified, estimate its local time alignment and slowly varying gains across frequency bands, attenuate the spectrogram regions A explains (a spectral mask or a conservative power residual, not waveform subtraction), and search what remains for a quieter track B, inside covered passages too. This session tests the potential with the upper bound first: the true time map of A given by the generator, gains estimated from the mixture, against the plain mixture and against denser query peaks without masking. Only if the oracle shows recall worth having does it try estimated timing. It is a measurement session: the search and the product do not change.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32). Rendering runs well in parallel here.

Time box: started about 21:55; aim to finish within about 3 hours, because other sessions follow you tonight. If a step would run past that, scale it down (262 tracks only, the oracle only, fewer scenarios), record the decision in the brief and continue.

Environment (sessions 33-64's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,674 records, revision `7210fc479da58376` (session 64 pruned four ignored uploads). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-54-linux` (262 tracks) and `session-64-linux-nas` (NAS scale; compare with it, not `session-54-linux-nas`).
- Build `work/bin/s64-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Since `45f92d5` (2026-10-28) its calibrate reads only the reports its baseline holds; still run variants and other seeds in a scratch work directory, never in the reports directory `regress` compares.
- The user restructured the code on 2026-10-28 without meaning to change behaviour (commits `b5a84e3` to `f94604a`: harness subcommands in modules, named results, `Search` passed through the core's search, enums for closed sets). Session 60 confirmed both regressions identical on a build of master after it.
- Since sessions 53-56: `SpansAtSpeed(0.006)` is the default second suppression (ADR 0021); same-title detections (`docs/same-titles.txt`) are scored right (ADR 0022); `identify` prints gaps as "ID - ID" lines with an estimate of their tracks (ADR 0023, 0024). The minimum span stays undecided. A change to an existing decision needs a new ADR (the next number is in `docs/adr/`).
- The research note `docs/research/2026-10-29-matching-and-archive-proposals.md` (2026-10-29) records six unmeasured proposals and its evaluation boundaries; any session working on one follows its "Evaluation and adoption boundaries".
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen. On the evening of 2026-10-30 the user answered 8 of 12 questions (docs/status.md, "the listening of the 2026-10-30 pack"): the four same-title candidates (Cool Rock, Tower Bass twice, Stalker) are different titles; Vintage Dread 2000 disc 1 is separate tracks; Advent, Keltech - Axis and Plasmic - The Weapon have an inverted channel, We - Hello World is unsure. The pitch rides (Star Trails, Into Music) and the Planet Dust cut stay unanswered. When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
- The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-39.md, docs/brief-48.md and the newest brief, docs/session-notes.md ("Session 39", "Session 48" and the newest session), docs/status.md (the newest entries), the proposals note (§4, "Automated counterexample search" and "Evaluation and adoption boundaries"), ADR 0006, 0017 and 0019, experiments 0182-0185, 0207, 0208 and 0215-0218, the roadmap's "Possible plays inside a confident play of another recording", and the code of `mixes/drops.rs` (the generator, its truth: each play's source start, speed, gain and fades), the spectrogram and peak picking, and how the harness searches a rendered mix.
2. Write docs/brief-65.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-65-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s65/listening-digests-start.txt. Experiments number from 0255.
3. Build `work/bin/s65-start`; `regress session-54-linux` and `regress session-64-linux-nas` identical.

Steps:

1. Pre-registration, committed before any masked search: the mixes (generated double drops with exact truth, quiet partners at -9 dB and brief partners as in session 39, and controls: A alone, a held-out B, related versions, shared breaks, bass swaps; development seeds 2026 and 2028, two fresh test seeds that nothing is tuned on); A's reference (a different rip of A where the library holds one, so the mask does not lean on identical audio; say which records allow it); the mask (the oracle time map from the truth; per-band gains fitted from the mixture over the covered passage; a spectral attenuation rule fixed before the test seeds), the comparison arms (plain mixture, masked residual, denser query peaks without masking), the scores kept apart (B found as a candidate, B confirmed confident or possible in the residual, and whether the same evidence exists in the original mixture; absent records made confident by the residual counted as wrong), and what would justify trying estimated timing or an evidence rule (the proposal names that confirmation needs evidence in the original mixture).
2. The harness, tested: the masked residual as an opt-in query transform (harness only; `regress` identical with it off), unit tests on synthetic signals (a known A over a known B: the mask removes A's peaks and keeps B's where they do not overlap).
3. The runs at 262 tracks (and NAS scale for the oracle if time allows; never write the NAS store), under tmux with logs where long.
4. Experiments (at most 40 lines each, written before the next starts): the oracle against both arms on the development seeds; the frozen rule on the test seeds; estimated timing only if the oracle earns it.
5. A short note in the proposals note's §4 and the roadmap's "Possible plays inside a confident play of another recording": what the oracle shows, and whether the idea is worth a next step.
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 65" section at the end of docs/session-notes.md with a handoff; update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-54-linux` and `regress session-64-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: the oracle's recall against both arms, wrong answers, the test seeds, and anything left.

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
- Scratch data goes under work/s65/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00 on 2026-10-31. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 65

- 2026-10-30, 21:52 by the Linux machine's clock (CET). The time box is
  about 3 hours: aim to finish by about 00:55. The soft deadline is
  11:00 on 2026-10-31; no new run starts after about 10:20.
- `master` at `10ed89b`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s65/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `c4d3da2e7e37...`, `work/audition/answers.jsonl` `3dc509cb1ec9...`.
- Experiments 0001-0254; this session numbers from 0255. ADRs
  0001-0024. Test set: 4 of 5 evaluations used (none in this session).
- The NAS store: 40,674 records, revision `7210fc479da58376`. Builds:
  `work/bin/s64-final/` (master). Baselines `session-54-linux` and
  `session-64-linux-nas`.
- Session 39's drops renders of seeds 2026 and 2028 (16 mixes each,
  `--tagged work/s39/tags-262.json`) are in `work/mixes/drops-seed-*`;
  session 48 drew panels for seeds 2030-2035 in `work/s48/panels` and
  used 2032 and 2034 as its test seeds.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build and both regressions (tmux `s65start`) | 21:52-22:10 |
| 1 | pre-registration (below), committed before any masked search | 22:10 |
| 2 | the harness: `mixes --kind drops --residual`, unit tests on synthetic signals; `check.sh`; `regress` identical with it off | 22:10-23:05 |
| 3-4 | development seeds (0255); the frozen rule on the test seeds (0256); estimated timing only if earned | 23:05-00:10 |
| 5 | the proposals note's §4 and the roadmap | 00:10-00:20 |
| 6-7 | review, notes, status, checklist, final checks | 00:20-00:55 |

## Pre-registration (step 1; written before any masked search)

**Question.** In generated double drops with exact truth, once the
louder record A is identified confidently, does removing what A
explains from the spectrogram let the search find the quieter record B
playing under it, with no confident answer that is not playing? Upper
bound first: A's time map comes from the generator.

**Mixes.** `mixes --kind drops --tagged work/s39/tags-262.json
--count 16 --ladder turntable` (as experiments 0184 and 0216), 262
tracks: the corpus store less each seed's held-out recordings.
Development seeds 2026 and 2028 (session 39's renders, hard-linked into
the scratch work directory, not rendered again). Test seeds: the first
two seeds from 2036 up whose panel keeps both Clockwork records indexed
(session 48's condition for 2032 and 2034, which a session already
used); nothing is tuned on them.

**Cases.** Every play of an indexed record that the plain search of the
whole mix finds confidently is an A. Its excerpt is A's audible span in
the mix (fades included), cut from the decoded mix. The other plays
overlapping the excerpt are scored by the truth. The rows that matter:
B is the second record of a quiet (-9 dB), brief (20-40 s), unrelated
"inside" or pair-drop block and A the first. Every other A is a
control: a record alone (original, remix, returning, two rips), a
first or second record of a drop whose partner plays on, and the
transitions at each excerpt's edges (bass swaps among them, `join`).
Related versions: the Clockwork pair's records alone and in pair drops
(masking one must not make its relative confident). Shared breaks are
not built separately: any confident record not playing in a masked
excerpt is wrong, whatever it shares with A. Broadcast processing is
not in the drops kind and is not tested (the proposal lists it; left
for a later session). Held-out B: the masked excerpts of the quiet and
brief blocks are searched again with every such B's recording also
left out of the index; anything confident not playing there is wrong.

**A's reference.** The oracle renders A's contribution exactly as the
generator did (`play_audio`: speed, source start, fades, gain, bass
cuts), from a different rip of A where the 262-track clusters
(`work/reports/duplicate-clusters.json`) hold another indexed file of
A's recording, else from the played file itself. The rip's time map
to the played file comes from library audio only: its stored peaks
searched against the played file's (`search_peaks`), the strongest
detection's line. Results are reported apart for the two kinds of
reference.

**The mask.** STFT of the excerpt and of A's reference at the
profile's analysis (8 kHz, 1024-sample Hann window, hop 128). Gains:
per third-octave band from bin 5 to bin 500 and per block of 63 frames
(about 1 s), the 30th percentile of the mixture's power over A's, over
the band-block's points whose A power is at least its median; no gain
(nothing removed) where the block holds fewer than 16 such points.
Rule: each bin keeps the share `max(1 - over * g * P_A / P_mix,
0.001)` of its power (a conservative power residual; at least -30 dB),
`over` = 2 by default. The masked spectrum is turned back into audio
(overlap-add), which the unchanged search analyses on every rung: the
product's front end and search do not change. The one alternative on
the development seeds is `over` = 1. The rule for the test seeds is the
one that confirms more B on the development seeds with no wrong
confident answer in any control; on a tie, `over` = 2.

**Arms** (each on the same excerpts): P, the plain excerpt; D, denser
query peaks without masking (the query's peaks picked with a
neighbourhood of 8 x 8 instead of 12 x 12, about twice as many; the
index unchanged); M, the masked residual; MH, M searched without the
quiet and brief Bs' recordings.

**Scores, kept apart.** For each B: found as a candidate (a detection
of B's cluster overlapping B's play, two windows or more), confirmed
(possible or confident) and confident; for B confirmed in M, whether
the plain excerpt holds a detection of B (the same evidence in the
original mixture) and its hits. Wrong: confident (and possible)
detections matching no play, per arm, with absent records made
confident by the residual counted as wrong. A's own level in M (what
the mask leaves of A). Printed-tracklist recall is not measured: the
product has no residual, and a B under A stays marked by ADR 0019
whatever its level (a later rule's question).

**What would justify a next step.** Estimated timing (A's line from its
confident detection instead of the truth) is tried in this session
only if, on the development seeds, M confirms at least 8 more of the 64
quiet and brief Bs than both P and D, with no wrong confident answer in
M or MH, and at least half of the Bs it newly confirms also have a
detection in the plain excerpt. An evidence rule (confirmation needs
evidence in the original mixture, §4) is not designed here; the scores
above say whether one is worth designing. These are the session's
reading rules, not adoption thresholds (the proposals note sets none).

**Not done.** NAS scale only if time allows after the test seeds. No
product change; the residual is a harness option (`regress` identical
with it off).

## Decisions

- **(21:58) Start build and regressions** (`work/s65/start.sh`, tmux
  `s65start`, logs in `work/s65/logs/`; `run.sh` and `same.py` are
  session 64's with the paths changed). Cargo had nothing to rebuild:
  the start build is byte-identical to `work/bin/s64-final/`.
- **(22:05) The residual is turned back into audio.** The core analyses
  the query again for each turntable rung (`Rung::points_in` resamples
  the audio), so a masked spectrogram cannot be handed to the search as
  peaks; the harness resynthesises the masked STFT by overlap-add and
  searches the audio with the unchanged search. The harness takes
  `realfft` (the core's FFT crate, the same version) for it.
- **(22:05) The ladder is the turntable ladder** (as 0184 and 0216; the
  drops kind plays every record on a turntable).
- **(22:00) The pre-registered floor is dropped before any masked
  search.** The synthetic test (a known A over a known B) showed that a
  bin keeping at least -30 dB of its power keeps A's pattern of peaks
  wherever nothing else plays within the picker's neighbourhood: A's
  bursts, 30 dB down, were still local maxima above the floor. A bin
  that A explains entirely now falls to silence, below the peak picker's
  floor (`max(1 - over * g * P_A / P_mix, 0)`). A guard was added with
  it: a band and block whose reference power is under 1e-6 of the
  band's mean over the excerpt gets no gain (A silent there, so the
  ratio would measure what else plays).
- **(22:07) The harness** (`mixes --kind drops --residual
  [--residual-over F] [--residual-timing truth|detected]`;
  `crates/gunfinger-eval/src/residual.rs` for the mask,
  `mixes/residual.rs` for the cases). The report gains `residual`
  (left out of the JSON when absent, so default reports do not change)
  and goes to `mixes-drops-seed-<seed>-residual-<timing>-over-<over>.json`.
  The mixture's excerpt and A's reference are both decoded at 8 kHz by
  the same FFmpeg path (A's render written as a WAV for a moment). The
  denser arm picks the query's peaks in an 8 x 8 neighbourhood. The
  held-out-B arm runs for A of a quiet or brief block only. Smoke test
  on one mix of seed 2026 (1:33): 15 cases, 5 with another rip as
  reference.
- **(22:10) Test seeds 2036 and 2039.** Panels of seeds 2036-2041 drawn
  with `--count 0` into `work/s65/panels` (a copy of session 48's):
  2036, 2039, 2040 and 2041 keep both Clockwork records indexed, 2037
  and 2038 hold one out. Nothing is run on 2036 or 2039 until the rule
  is frozen.
- **(22:20) The development seeds** (`work/s65/dev.sh`, tmux `s65dev`,
  `work/bin/s65-run`, scratch `work/s65/wd` with session 39's renders
  hard-linked; 2:38-2:44 per run): experiment 0255. `over` 2 confirms 62
  of the 64 quiet and brief Bs (P 41, D 39) with no wrong confident
  answer in M or MH; `over` 1 confirms 55. **Rule frozen for the test
  seeds: `over` 2.** Only 6 of the 21 Bs newly confirmed have a plain
  detection, under the registered half, so **estimated timing is not
  tried in this session** (the registered reading; it stays the next
  step to measure). Only 2 of the 64 quiet and brief As had another
  rip, so those rows rest on identical audio; another rip as reference
  left A confident in 20 of 132 cases.
- **(22:31) The test seeds** (`work/s65/test.sh 2`, tmux `s65test`,
  scratch `work/s65/wt`, rendered there; 3:29 and 3:03): experiment
  0256. M confirms all 64 quiet and brief Bs (P 36, D 40), 48
  confidently (P 12), with no wrong confident answer in any arm or in
  MH; M's wrong possibles 0 (P 196). Estimated timing stays untried
  (the rule was read on the development seeds).
- **(22:32) A slip, without effect:** the NAS-scale queue (tmux
  `s65nas`) was started a minute before 0256 was written; both runs
  failed at once while drawing the mixes (below) and produced nothing,
  and the queue ran again only after 0256 was written.
- **(22:33) NAS scale for the oracle, on the development seeds.** At
  NAS scale the drops kind could not draw its mixes: `draw_rips` needs
  every file of a cluster among the corpus records, and the NAS clusters
  hold NAS files ("too few tracks"). The drops kind now draws with the
  corpus clusters at every scale (`Options::corpus_clusters`; at 262
  tracks they are the clusters the run loads, so nothing changes there,
  and no other kind or regression report is affected); scoring keeps the
  run's clusters. The residual's other rip may now be a NAS file, its
  peaks read from the NAS store (`OtherLibrary::record`), so far more As
  get a reference that is not the played audio. The renders are copied,
  not hard-linked, into `work/s65/wn`, so that a plan that differed could
  not write over session 39's files. `work/s65/nas.sh 2026 2028`, tmux
  `s65nas2`, `work/bin/s65-run2`.
- **(22:37) Two fixes during the NAS runs.** The other library's
  records name their files without the `second-library/` prefix, so the
  first NAS run rendered a rip from the corpus root and failed; the rip
  now keeps its cluster's name. One restart (tmux `s65nas3`) ran for a
  few seconds with the old binary before it was stopped. The temporary
  WAVs' render receipts are removed with the scratch directory after
  the cases (no result changes).
- **(23:12) NAS scale** (`work/s65/nas.sh 2026 2028`, tmux `s65nas4`,
  11:29 and 12:19, 14.2 GB peak): experiment 0257. All 32 plans equal
  session 39's. Precision holds (M and MH 0 wrong confident; P and D 2,
  the Kemal - Animation remix upload over the original, as in 0182).
  With another rip as reference (403 of 440 cases) the mask leaves A
  confident in 115 and the quiet and brief Bs confirmed fall to 50 of
  64 (P 40), confident 32 (P 8). Not run: the test seeds at NAS scale
  and estimated timing.
