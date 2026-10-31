# Session 70 brief: proposal 4's next step: the other-rip residue, detected timing, an evidence rule

Written on 2026-10-31, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file (its pre-registration above all), and
   `docs/session-70-checklist.md` (progress).
3. `docs/brief-65.md`; `docs/session-notes.md` ("Session 65"); the
   proposals note's §4 and "Evaluation and adoption boundaries";
   experiments 0127 and 0255-0258; ADR 0019; this session's experiments
   (0269 and up).
4. The code: `crates/gunfinger-eval/src/residual.rs`,
   `crates/gunfinger-eval/src/mixes/residual.rs`, the `--residual-*`
   options in `commands/mixes.rs`.

`docs/brief-2.md` to `docs/brief-69.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 70 of Gunfinger: proposal 4's next step, as session 65's handoff orders it. Session 65 (experiments 0255-0258, notes "Session 65") measured the masked residual on generated double drops with the generator's time map: with A's own audio as the reference it recovered nearly every quiet and brief partner (confirmed 41 to 62 of 64, confident 8 to 47 at 262 tracks, 0 wrong); with another rip of A, the case of a real mix, at NAS scale the mask left A confident in about a quarter of the cases (15-19% of corpus rips, a third of NAS files) and the partners rose less (confirmed 40 to 50 and 35 to 55, confident 8 to 32 and 12 to 34), with one wrong confident answer (the Cool Rock upload, a confirmed different title). Its pre-registered reading did not try estimated timing, because only 6 of 21 newly confirmed partners had a plain detection on the development seeds. The orchestrating thread takes the next step anyway, as a decision recorded here and in the brief, because the other-rip gains at NAS scale are the largest recall gain measured tonight: first, why another rip leaves A in the residual; then `--residual-timing detected` (A's time map from the whole-mix search's detection, not the truth); then whether a confirmation rule that needs evidence in the original mixture keeps the gain and the wrong answers out. Each part has its own pre-registration. It is a measurement session: the search and the product do not change.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32). Rendering runs well in parallel here.

Time box: started about 06:50; finish by 10:40 at the latest (the user's soft deadline is 11:00); this is the night's last session. If a step would run past that, scale it down in this order: drop the evidence rule, then the NAS-scale test seeds, then keep the residue diagnosis and detected timing at 262 tracks; record the decision in the brief and continue.

Environment (sessions 33-69's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,674 records, revision `7210fc479da58376` (session 64 pruned four ignored uploads). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-54-linux` (262 tracks) and `session-64-linux-nas` (NAS scale; compare with it, not `session-54-linux-nas`).
- Build `work/bin/s69-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Since `45f92d5` (2026-10-28) its calibrate reads only the reports its baseline holds; still run variants and other seeds in a scratch work directory, never in the reports directory `regress` compares.
- The user restructured the code on 2026-10-28 without meaning to change behaviour (commits `b5a84e3` to `f94604a`: harness subcommands in modules, named results, `Search` passed through the core's search, enums for closed sets). Session 60 confirmed both regressions identical on a build of master after it.
- Since sessions 53-56: `SpansAtSpeed(0.006)` is the default second suppression (ADR 0021); same-title detections (`docs/same-titles.txt`) are scored right (ADR 0022); `identify` prints gaps as "ID - ID" lines with an estimate of their tracks (ADR 0023, 0024). The minimum span stays undecided. A change to an existing decision needs a new ADR (the next number is in `docs/adr/`).
- The research note `docs/research/2026-10-29-matching-and-archive-proposals.md` (2026-10-29) records six unmeasured proposals and its evaluation boundaries; any session working on one follows its "Evaluation and adoption boundaries".
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen. On the evening of 2026-10-30 the user answered 8 of 12 questions (docs/status.md, "the listening of the 2026-10-30 pack"): the four same-title candidates (Cool Rock, Tower Bass twice, Stalker) are different titles; Vintage Dread 2000 disc 1 is separate tracks; Advent, Keltech - Axis and Plasmic - The Weapon have an inverted channel, We - Hello World is unsure. The pitch rides (Star Trails, Into Music) and the Planet Dust cut stay unanswered. When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
- The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-65.md (all of it: the pre-registration and the decisions), docs/session-notes.md ("Session 65", "Session 66" and the newest session), docs/status.md (the newest entries), the proposals note (§4 and "Evaluation and adoption boundaries"), experiments 0127, 0182-0185 and 0255-0258, ADR 0019, and the code of `crates/gunfinger-eval/src/residual.rs`, `mixes/residual.rs` and the `--residual-*` options.
2. Write docs/brief-70.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-70-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s70/listening-digests-start.txt. Experiments number from 0269.
3. Build `work/bin/s70-start`; `regress session-54-linux` and `regress session-64-linux-nas` identical.

Steps:

1. The residue, diagnosed (pre-registered before its runs): for the cases where A stays confident after masking with another rip, measure what differs from the played-file cases (the alignment's hits and speed between the reference and the played file, gain-fit residuals per band, the render's seek against the stored peaks' timeline as in 0127, lossy uploads against scene rips), and test at most two fixes fixed in advance (for example re-fitting A's time map against the reference by the stored peaks' alignment, or a per-band time-varying gain), on the development seeds only.
2. Detected timing (pre-registered before its runs): A's time map from the whole-mix search's confident detection (its segments' offsets and speeds) against the truth's, on the development seeds 2026 and 2028, with the reference rule of 0257 (another rip where the store holds one), at 262 tracks and NAS scale; scores as in 0255-0258 (B confirmed and confident, wrong confident, A left confident), kept apart from session 65's oracle numbers.
3. The evidence rule (pre-registered before its runs, if time allows): a partner found in the residual counts only if the original mixture's excerpt also holds a detection of it at the same alignment above a fixed lower bar; frozen on the development seeds, then run once on the unused test seeds 2040 and 2041 (both keep the Clockwork records indexed), with the Cool Rock case and other related versions on their own lines.
4. Experiments (at most 40 lines each, written before the next starts): the residue; detected timing; the evidence rule on the test seeds.
5. Update the proposals note's §4 and the roadmap's "Possible plays inside a confident play of another recording" with what the three steps show and what a product version would need (reference audio at search time, which `--store-only` lacks; an ADR for any new evidence rule).
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 70" section at the end of docs/session-notes.md with a handoff; update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-54-linux` and `regress session-64-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: the residue's cause, detected timing against the oracle, the evidence rule on the test seeds, and anything left.

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
- Scratch data goes under work/s70/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00 on 2026-10-31. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 70

- 2026-10-31, 06:43 by the Linux machine's clock (CET). Finish by
  10:40; no new run starts after about 10:20.
- `master` at `4929028`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s70/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `c4d3da2e7e37...`, `work/audition/answers.jsonl` `3dc509cb1ec9...`.
- Experiments 0001-0268; this session numbers from 0269. ADRs
  0001-0024. Test set: 4 of 5 evaluations used (none in this session).
- The NAS store: 40,674 records, revision `7210fc479da58376`. Build
  `work/bin/s69-final/` (master). Baselines `session-54-linux` and
  `session-64-linux-nas`.
- Session 65's renders: development seeds 2026 and 2028 in
  `work/mixes/drops-seed-*` (session 39's) and copied in `work/s65/wn`;
  test seeds 2036 and 2039 in `work/s65/wt/mixes/`. Panels of seeds
  2030-2041 in `work/s65/panels`; 2040 and 2041 keep both Clockwork
  records indexed and have never been rendered or searched.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build and both regressions (tmux `s70start`) | 06:43-07:15 |
| Pre-registration | below, all three steps, committed before any run | 07:15 |
| Harness | the two fixes, the diagnostics, several timings and fixes in one run, the plain evidence per masked detection; tests; `check.sh` | 07:15-08:05 |
| 1-2 | development seeds 2026 and 2028, 262 tracks and NAS scale, one run per seed and scale with every timing and fix (tmux) | 08:05-09:05 |
| 4 | experiments 0269 (the residue) and 0270 (detected timing) | 09:05-09:25 |
| 3 | the test seeds 2040 and 2041 (renders, NAS scale and 262 tracks) and experiment 0271 | 09:25-10:10 |
| 5-7 | the proposals note, the roadmap, review, notes, status, final checks | 10:10-10:40 |

To save time, steps 1 and 2 share their runs: each run of a seed
searches every case's plain and denser excerpts once and the masked
residual for each timing (truth, detected) and each fix (none and the
two below). Both pre-registrations are written before those runs.

## Pre-registration (written before any run of this session)

### Common to the three steps

**Mixes and cases.** As session 65 (brief 65's pre-registration):
`mixes --kind drops --tagged work/s39/tags-262.json --count 16 --ladder
turntable --residual`, `over` 2. Every play the whole-mix search finds
confidently is an A; its audible span (by the truth) is the excerpt. The
plain (P) and denser (D) arms are searched once per case; the masked
residual (M) and, for A of a quiet or brief block, M without the
partners' recordings (MH) are searched once for each timing and fix.
Development seeds 2026 and 2028 (session 39's renders, copied, not
rendered again), at 262 tracks and at NAS scale. Test seeds 2040 and
2041, used only in step 3, after the rule is frozen.

**A's reference (the rule of 0257).** The first other indexed file of
A's recording (its cluster at the run's scale: a corpus rip, or at NAS
scale a NAS file) whose stored peaks align with the file the time map
is on; else that file itself. Rows are reported apart by reference: the
played file, a corpus rip, a NAS file.

**Scores** (as 0255-0258, kept apart): for each quiet and brief B
inside its A, found as a candidate, confirmed (possible or confident)
and confident; wrong confident and wrong possible answers per arm
(deduplicated by mix, file and second, as `residual_pilot.py`); A's
level after masking (the residue). The unrelated and pair drops' Bs are
reported beside. Session 65's numbers are not mixed with these.

### Step 1: the residue

**Question.** When the reference is another rip, why does A stay
confident after masking in 15-19% of the corpus rips' cases and a third
of the NAS files' (0255-0258), and does a fix fixed now remove it?

**Measured per case and timing** (stored in the report, compared
between cases where A stays confident and where it does not, and
against the played-file cases):

- The rip's alignment with the file the time map is on (stored peaks
  against stored peaks, `rip_line`): hits, windows, speed.
- The timing error left: the lag between the reference and the excerpt,
  measured on the log power of the third-octave bands in blocks of 512
  frames (8.2 s) every 256 frames, at lags of up to 25 frames (400 ms)
  either way: each block's best lag and its correlation, the
  correlation at lag 0, and a Theil-Sen line through the blocks' lags
  (offset and drift in parts per million).
- The gain fit: for each band and block with a gain, the spread (70th
  minus 30th percentile, in dB) of the mixture's power over the
  reference's at A's stronger points; the median per band.
- The seek: for the reference file and for the played file, the offset
  between a render by seek (`render_samples`, as the mix and the
  reference are rendered) and a render from the decoded samples
  (`render_samples_as_indexed`, the stored peaks' timeline) at the
  render's start (0127's Do Drop In case), by cross-correlation of 2 s
  at up to 0.5 s either way.
- The reference file's codec, bit rate and sample rate (`ffprobe`, in
  the analysis script): lossy uploads against scene rips.

**The two fixes, fixed now.**

- *Align*: the reference is shifted onto the mixture before the gains
  are fitted. The lags above are measured per block; blocks whose best
  correlation is under 0.3 or where the reference is silent are
  dropped; a Theil-Sen line is fitted through the rest; each kept
  block's lag is then refined within 3 frames of the line (the best
  frame, then a parabola through it and its neighbours); the time map
  between block centres is linear in the refined lags (the line beyond
  the first and last), and the reference sample at mixture sample `n`
  is the one at `n - lag(n)`. With no kept block nothing is shifted.
- *Align, finer gains*: *Align*, and gains per block of 32 frames (0.5
  s) instead of 63.

**Reading (development seeds only).** The cause is named from the
measures above, as an inference where it is one. A fix counts as
removing the residue if, at NAS scale with the true time map, it
reduces the cases in which another rip leaves A confident by at least
half (0257's rows: 115 of 403 at NAS scale), with no more wrong
confident answers in M or MH than without it, and without fewer quiet
and brief Bs confirmed. Of the two, the frozen fix is the one leaving
fewer such cases; on a difference of under 5 cases, *Align* (the
simpler). If neither counts, steps 2 and 3 use no fix.

### Step 2: detected timing

**Question.** With A's time map from the whole-mix search's
detection instead of the generator, how much of the oracle's gain is
left?

**Timing.** A's strongest confident detection matching its play in the
whole-mix search gives the file it names, the file time at the
detection's start and its speed; the line is extended over A's audible
span. No fades, level or bass cuts (the band gains follow them). The
reference is then chosen by the rule above with this file as the one
the time map is on, so where the store holds another rip, the reference
is the same file as under the true time map, placed through the
detected line.

**Arms and reading.** M and MH with detected timing, with no fix and
with each fix, against the true time map with the same fix, in the same
run. The primary row is detected timing with the fix frozen in step 1
(no fix if none was frozen). Reported: the quiet and brief Bs confirmed
and confident; wrong confident and possible answers; the residue; the
lag the Align measure finds under each timing. No threshold decides
anything here: the result says how much estimated timing keeps of the
oracle's gain, at 262 tracks and NAS scale.

### Step 3: the evidence rule (if time allows)

**The rule.** A detection in a masked arm counts (at its level) only if
the original mixture's excerpt also holds evidence of it at the same
alignment: the plain excerpt, over the masked detection's span plus 5 s
either way, searched against an index of that detection's file alone on
the rungs within 0.6% of its speed, gives a detection that overlaps it
in mix time, whose speed is within 0.6% and whose file time at the
overlap's middle is within 1 s of the masked detection's; its hits are
the evidence. A detection whose evidence is under the bar `H` is
dropped. The masked arms store this evidence for every detection at
possible or above.

**Frozen on the development seeds** (NAS scale, the primary timing and
fix of step 2): `H` is the largest of 20, 40 and 60 that keeps at
least three quarters of the quiet and brief Bs that the masked residual
confirms and the plain excerpt does not; if none keeps that many, 20.
The rule is not tuned further.

**Test seeds, once.** Seeds 2040 and 2041 (never rendered or searched;
both keep the Clockwork records indexed), rendered for this run, at NAS
scale (and at 262 tracks if time allows), with the true and detected
timings, no fix and the frozen fix. Reported with and without the rule:
the quiet and brief Bs confirmed and confident, wrong confident and
possible answers, and on their own lines the Cool Rock pair and any
other related version (the Clockwork pair, the Kemal and Paragliders
cases of 0257-0258) that appears.

**Not decided here.** No product change, no ADR. A rule that holds on
the test seeds is a candidate for a later ADR, not a decision.

## Decisions

