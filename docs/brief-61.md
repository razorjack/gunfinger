# Session 61 brief: stereo channels combined after their power is measured

Written on 2026-10-30, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-61-checklist.md` (progress).
3. `docs/research/2026-10-29-matching-and-archive-proposals.md` (§1 and
   "Evaluation and adoption boundaries"); the roadmap's "Stereo faults
   before the downmix"; `docs/brief-42.md`.
4. `docs/session-notes.md` ("Session 42", "Session 60"); experiments
   0194-0198 and this session's (0245 and up).
5. `crates/gunfinger-core/src/spectrogram.rs`, `peaks.rs`, `profile.rs`,
   `speed.rs`, `search.rs`, `search/refine.rs`;
   `crates/gunfinger-eval/src/stereo.rs`, `robust.rs`.

`docs/brief-2.md` to `docs/brief-60.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 61 of Gunfinger: combine stereo channels after measuring spectral power (proposal 1 of `docs/research/2026-10-29-matching-and-archive-proposals.md`). FFmpeg mixes every file down to mono before peaks are picked (`-ac 1` in `decode.rs`). Session 42 (experiments 0194-0198, the roadmap's "Stereo faults before the downmix") measured what that costs: one inverted channel loses all 80 of `robust`'s indexed stereo excerpts (the downmix keeps only the side), noise at 0 dB in one channel loses 51, a 5 ms delay 43; a fault in the indexed rip costs as much (0197), and over half a generated mix an inverted channel leaves 1 of 100 plays confident (0198). Choosing a channel lost clean matches; searching the downmix and both channels recovered most faults at three searches a query (0195). The proposal: compute an STFT per channel and pick peaks from the summed power `P(f,t) = |X_L(f,t)|^2 + |X_R(f,t)|^2`, with a fixed normalisation. Reversing either channel's polarity leaves that power unchanged for fixed decoded samples, so one search covers inversion without a fault detector; delay invariance is not claimed, and a noisy channel's noise stays. This session measures it as an opt-in variant: first whether power-summed query peaks still match today's mono reference peaks, then, with a small variant reference store, faults on either side. It is a measurement session: the front end and the product do not change; a change is ours, with an ADR.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).

Time box: started about 01:55 (CET); aim to finish within about 2.5 hours, because another session follows you tonight. If a step would run past that, scale it down (262 tracks only, fewer fault conditions), record the decision in the brief and continue.

Environment (sessions 33-60's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,678 records, revision `1fc6fe50293ad330`. NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `ce293fe6165e...`, 284 clusters. Baselines `session-54-linux` (262 tracks) and `session-54-linux-nas`.
- Build `work/bin/s60-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Since `45f92d5` (2026-10-28) its calibrate reads only the reports its baseline holds; still run variants and other seeds in a scratch work directory, never in the reports directory `regress` compares.
- The user restructured the code on 2026-10-28 without meaning to change behaviour (commits `b5a84e3` to `f94604a`: harness subcommands in modules, named results, `Search` passed through the core's search, enums for closed sets). Session 60 confirmed both regressions identical on a build of master after it.
- Since sessions 53-56: `SpansAtSpeed(0.006)` is the default second suppression (ADR 0021); same-title detections (`docs/same-titles.txt`) are scored right (ADR 0022); `identify` prints gaps as "ID - ID" lines with an estimate of their tracks (ADR 0023, 0024). The minimum span stays undecided. A change to an existing decision needs a new ADR (the next number is in `docs/adr/`).
- The research note `docs/research/2026-10-29-matching-and-archive-proposals.md` (2026-10-29) records six unmeasured proposals and its evaluation boundaries; any session working on one follows its "Evaluation and adoption boundaries".
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen. Pending listening (Star Trails, Into Music) stays for later. When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
- The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-42.md and the newest brief, docs/session-notes.md ("Session 42" and the newest session), docs/status.md (the newest entries), the proposals note (§1 and "Evaluation and adoption boundaries"), experiments 0194-0198, the roadmap's "Stereo faults before the downmix", ADR 0002 or whichever ADR fixes the front end and the profile (and `hash::design()`'s invalidation rule), and the code of `decode.rs`, the spectrogram and peak picking, the profile, `robust.rs` (the `stereo-*` conditions and `--query-channels`) and how the harness builds a store or an in-memory index from audio.
2. Write docs/brief-61.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-61-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s61/listening-digests-start.txt. Experiments number from 0245.
3. Build `work/bin/s61-start`; `regress session-54-linux` and `regress session-54-linux-nas` identical.

Steps:

1. Pre-registration, committed before any variant run: the variant (decoding two channels, an STFT per channel, peaks from summed power with a fixed normalisation chosen so that a mono source gives the same power scale as today's downmix, or the closest; say exactly which), its identity (a distinct profile or variant name, so its peaks never mix with today's store), the conditions (session 42's `stereo-*` faults on the query side and on the reference side, clean stereo controls, other rips, and the shared-material cases where the stereo panel allows), the two phases (A: query-only compatibility, power-summed query peaks against today's mono reference index; B: a small variant reference store at 262 tracks under work/s61/, built by the variant, with faults on either side, including a polarity change partway through a generated mix), the scores (confident and possible recall, wrong confident answers, peak and posting density, and full search cost including the second STFT), and what would make it worth an ADR proposal (fault recovery holding on clean and cross-rip controls with no new wrong confident answer).
2. The variant, tested, harness only (an opt-in front-end option; `regress` identical with it off): unit tests that inverting one channel leaves the variant's peaks unchanged on fixed samples, that a mono file gives the expected peaks, and that the default path is untouched.
3. The runs: phase A at 262 tracks and NAS scale (query side only, so the NAS store is only read), then phase B at 262 tracks. Never write `work/peaks` or the NAS store; variant stores live under work/s61/.
4. Experiments (at most 40 lines each, written before the next starts): phase A; phase B; cost and density.
5. A short decision note (`docs/research/2026-10-30-stereo-power-decision.md`): what the variant recovers, what it costs (including that adoption means a new profile and every reference rebuilt: the NAS store's 40,678 records decoded again, with an estimate of the time), the risks, and a recommendation that is ours to take. Update the roadmap's "Stereo faults before the downmix" and the proposals note's §1 with a pointer.
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 61" section at the end of docs/session-notes.md with a handoff; update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-54-linux` and `regress session-54-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: phase A's compatibility, phase B's recovery and controls, the cost, the recommendation, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's behaviour does not change: no threshold, parameter, rule, default, front end or output change; variants are opt-in, tested, and keep `regress` identical when off. Harness additions are tested and keep `regress` identical. Variant peak stores live under work/s61/ only.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
- Scratch data goes under work/s61/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 10:00. Start no new run after about 09:30; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 61

- 2026-10-30, 01:52 by the Linux machine's clock (CET). The time box is
  about 2.5 hours: aim to finish by about 04:25. The soft deadline is
  10:00; no new run starts after about 09:30.
- `master` at `af32482`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s61/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `ce293fe6165e...`, `work/audition/answers.jsonl` `4b83d2fc9a2e...`.
- Experiments 0001-0244; this session numbers from 0245. ADRs
  0001-0024. Test set: 4 of 5 evaluations used (none in this session).
- Start build `work/bin/s61-start/` from master (`cmp`-equal to
  `s60-final`). `regress session-54-linux` (0:51) and `regress
  session-54-linux-nas` (3:47) identical: 8 of 8 reports equal to their
  baselines' apart from `wall_seconds`.
- The front end today: `decode.rs` asks FFmpeg for one channel (`-ac 1`,
  each channel at 1/sqrt(2), measured in session 42); the STFT
  (`spectrogram::for_each_power_frame`: periodic Hann, 1,024 samples at
  8 kHz, hop 128) gives each bin's power `|X|^2 + 1e-20`; the picker
  (`peaks.rs`) takes local maxima of 25 frames by 25 bins above -10 dB
  and refines them. The search analyses the query's samples again for
  each rung (`speed.rs`, `search.rs`, `search/refine.rs`), so a
  front-end variant must reach the core's search, not only the store.
  The profile (`profile.rs`, ADR 0003) names every parameter that shapes
  the stored peaks; a record is current only under its profile's
  identifier.
- Measured (one 1-channel test tone): FFmpeg's `-ac 2` upmix of a
  one-channel file gives each channel the sample at 1/sqrt(2). Every
  one of the corpus's 265 files has two channels (ffprobe).

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build and regressions | 01:52-02:20 |
| 1 | pre-registration (below), committed with this brief | 02:20 |
| 2 | the variant: a `Signal` in the core's STFT and search, a profile field for its identity, `--front-end` in the harness, a store builder; tests | 02:20-03:05 |
| 3 | phase A (262 tracks, NAS scale), phase B (262 tracks), costs | 03:05-03:45 |
| 4 | experiments 0245 (phase A), 0246 (phase B), 0247 (cost and density) | with step 3 |
| 5 | decision note; roadmap and proposals note | 03:45-03:55 |
| 6-7 | review by a separate agent; notes, status, checklist, final checks | 03:55-04:25 |

### Pre-registration (committed before any variant run)

**The variant, `power-sum`.** Each file is decoded by FFmpeg as two
channels (`-ac 2 -ar 8000`, the first audio stream, the core's container
handling; `stereo::decode_stereo`, session 42's). For each STFT frame
(today's window, size and hop, and each rung's scaled ones in the
search) both channels are transformed and the bin's power is
`P = |X_L|^2 + |X_R|^2 + 1e-20`; today's picker, floor, neighbourhood and
refinement run on `P` unchanged. **Normalisation: none (a factor of 1).**
FFmpeg's downmix is `M = (L + R) / sqrt(2)`; with `S = (L - R) / sqrt(2)`
the map from `(L, R)` to `(M, S)` is orthogonal, so `P = |X_M|^2 +
|X_S|^2`: the variant's power is today's downmix power plus the side's
power. A mono source therefore gives today's power scale exactly in exact
arithmetic, both as a stereo file with equal channels (`S = 0`) and as a
one-channel file (FFmpeg's `-ac 2` gives each channel `x / sqrt(2)`, so
`P = |X|^2`, today's); in floating point the two differ by rounding. A
narrow source (side 10-20 dB below the mid, 0196) is 0.04-0.41 dB above
today's power; inverting either channel swaps `M` and `S` and leaves `P`
unchanged.

**Identity.** A profile field `channels` (`Downmix` today; `PowerSum`
for the variant). Today's profile keeps its identifier; the variant's
ends in ` channels=power-sum`, so its records are never current under
today's profile and the reverse. The harness option `--front-end
downmix|power-sum-queries|power-sum`: `downmix` (default) is today's
path; `power-sum-queries` analyses queries by the variant against
today's records (phase A); `power-sum` analyses queries by the variant
and loads records of the variant's profile (phase B, with `--peaks-dir`
naming a variant store under `work/s61/`). Variant stores are built by a
harness command from the corpus library (read only), for the files with
a current record in `work/peaks` (which is only read), never written to
`work/peaks` or the NAS store.

**Conditions.**

- `robust --seed 2026`: every default condition (mono renders, as
  today) and the nine `stereo-*` conditions (fault on the right channel,
  session 42), 40 indexed and 10 held-out draws at 0.95 and 1.03.
- Reference faults: session 42's re-encoded copies (`work/s42/ref/library/`,
  the first 10 indexed draws: unchanged, inverted, silent, 1 ms and 5 ms
  late) put in place of their library files in a 262-track store, one
  store per copy and per front end (the `downmix` stores by today's
  decode and profile); searched by `robust --only stereo-control` (80
  indexed queries: the 20 of those draws measure the fault, the other 60
  stay controls).
- Generated mixes: `mixes --seed 2026|2027|2028 --count 8 --stereo-fault
  stereo-control` and `stereo-inverted` (session 42's renders; the right
  channel inverted from half the mix's length, so the polarity changes
  partway through).
- Real mixes and other rips (cross-rip controls): the three development
  mixes (`stakka-skynet-knowledge`, `razorjack-2003-03-29`,
  `nocny-trans-vertical-2002-08-23`) scanned whole; at NAS scale
  `robust` and the scans also find the NAS's other rips of the corpus
  recordings, counted through the clusters.
- Shared-material cases: left out (conservative, for the time box);
  `shared` and the panels take no front-end option, and the result would
  need its own run of the panels.

**Phases.** A (query-only compatibility): `power-sum-queries` against
today's records, `robust` and the three scans at 262 tracks and at NAS
scale, the generated mixes at 262 tracks; each against the same runs
with `downmix` by the same build. B (the variant store): `power-sum` at
262 tracks for `robust`, the reference faults, the generated mixes and
the three scans, against `downmix`.

**Scores.** Confident and possible counts per condition (indexed
queries), wrong confident and wrong possible answers by name, median
hits against the control; mixes: confident plays in the first half,
across the middle and in the second half, wrong confident plays; scans:
confident plays, correct and wrong (each scan's own scoring). Density:
peaks per second of audio and postings of the 262-track store under each
front end. Cost: the Stakka scan at NAS scale, `downmix` and
`power-sum-queries` A B B A at 32 jobs (wall, user CPU, peak memory;
the scan's time includes decoding), and the time to build the 262-track
store under each front end, from which the NAS store's rebuild (40,678
records) is estimated.

**What would make it worth an ADR proposal** (phase B at 262 tracks,
against `downmix` by the same build): fault recovery, at least 76 of 80
`stereo-inverted` excerpts and 18 of 20 queries against an inverted
reference confident, and at least 90 of 100 plays in the inverted half
of the generated mixes; holding on the clean controls, every default
`robust` condition and `stereo-control` within 1 of 80 of today's, the
generated mixes' control within 2 confident plays, each development
scan's correct confident plays within 1 of today's; and no wrong
confident answer that `downmix` does not have, in any run of either
phase. Phase A decides nothing by itself; it measures whether today's
records could be searched by the variant's queries before a rebuild.

## Decisions

- **(01:55) Start checks.** Both regressions identical with
  `work/bin/s61-start/` (above).
- **(02:05) The variant in code** (harness option, core analysis):
  `spectrogram::Signal` (`Mono`, today's; `PowerSum { left, right }`)
  is what the STFT analyses; `peaks::extract_signal_peaks(_in)`,
  `Rung::signal_points` and `search::search_signal_with` take it, and
  every existing entry point that takes `&[f32]` wraps it in
  `Signal::Mono`, so the default path runs the same arithmetic.
  `Profile::POWER_SUM` (field `channels`) names the variant's records.
  In the harness, `--front-end` is a `Matching` option, so its reports
  go to `reports/variant-power-sum-queries/` and
  `reports/variant-power-sum/`; only `robust`, `scan`, `mixes` and the
  new `variant-store` take it, and `--verify` refuses it (the peak
  verifier analyses one channel). `variant-store --out DIR [--replace
  TSV]` writes records under the front end's profile for the library
  files with a current record in `work/peaks`, decoding a replacement
  file where the list names one; it refuses the library's own store.
- **(02:08) Scaled down for the time box.** `robust` runs all 38 default
  conditions and the 9 stereo ones at 262 tracks (two runs a front end:
  no `--only`, then `--only` the stereo set, each with `control`), but
  only the stereo set (with `control`) at NAS scale. Phase A runs in
  the scratch work directory `work/s61/w` (renders hard-linked from
  `work/s42/w/robust/`, `work/robust/` and `work/mixes/stereo-*`; the
  saved NAS index hard-linked; both clusters files and the library map
  copied); tmux `s61a`, `work/s61/phase-a.sh`. Phase A was started
  while the NAS regression with the run build was still running (it
  only slows both).
- **(02:15) Regressions with the variant off.** `work/bin/s61-run/`
  (the variant's code before `cargo fmt`, which changed layout only):
  `regress session-54-linux` (0:51) and `regress session-54-linux-nas`
  (6:39, beside phase A) identical, 8 of 8 reports. `cargo test
  --release` green (all suites).
- **(02:35) Phase A done** (02:08-02:34; 0245). Every wrong confident
  answer at NAS scale is the held-out Cool Rock Stuff excerpt (draw 41)
  as the Mobb Deep / Urban Takover upload, which the downmix also gives
  on the clean controls; the variant gives it in more fault rows only
  because it keeps that excerpt there. Read against the bar as "no new
  pair of query and file", which is how it was meant; recorded so the
  reading is visible.
- **(02:49) Phase B done** (02:35-02:47; 0246). Recovery meets the bar
  (80 of 80 inverted; 20 of 20 against an inverted rip; 94 of 100 plays
  in the inverted halves). The clean controls do not: 6 of 38 default
  `robust` conditions (mono renders) fall by 2-4 of 80, against a bar
  of 1. The registered bar for an ADR proposal is therefore not met.
- **(02:52) Cost** (tmux `s61cost`, 02:48-02:50; 0247): the Stakka scan
  at NAS scale 27.6-27.7 s with power-sum queries against 24.6-24.8 s,
  12% more wall time and 13% more CPU; peaks and postings 0.1% fewer;
  a 262-track build as fast, with 3 times the peak memory.
- **(02:56) Decision note** `docs/research/2026-10-30-stereo-power-decision.md`:
  not adopted; the recommendation (ours) is a pre-registered test of
  the query side alone. The roadmap's "Stereo faults before the
  downmix" and the proposals note's §1 point to it.
