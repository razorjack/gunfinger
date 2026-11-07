# Session 78 brief: render receipts, the harness's FFmpeg watchdog and `regress` over all three development mixes

Written on 2026-11-07, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it), `AGENTS.md` and
   `docs/session-rules.md`.
2. This file, and `docs/session-78-checklist.md` (progress).
3. `docs/research/2026-11-05-external-review.md`, "Harness findings,
   confirmed in the code", items 3-5 (why this session runs), and
   `docs/adr/0018-*.md` (the stall timeout).
4. The code: `crates/gunfinger-eval/src/render.rs`, its cache users
   (`sweep::render_all`, `loss.rs`, `recall::render_damaged`,
   `shared::render`, `robust.rs`'s queries, `mixes::render_once`),
   `stereo::decode_stereo`, `crates/gunfinger-core/src/decode/watch.rs`,
   `regress.rs` and `commands/evaluation.rs`.

## The prompt, verbatim

```text
Start session 78 of Gunfinger: the rest of the harness fixes on the roadmap's active queue, item 1. The external review of 2026-11-05 (`docs/research/2026-11-05-external-review.md`, "Harness findings, confirmed in the code", items 3-5) found, by reading the code: (3) `render.rs` accepts a cached render when its size matches its completion receipt, and sweep render names hold only the draw and the speed, so after a change of panel or of source audio at the same path a cached render can be reused against different truth (generated mixes compare their serialized plan, but not the sources' identity, the renderer or the `fault` argument); (4) the harness calls FFmpeg with `.status()` and `.output()` and not through the core's watched child (`crates/gunfinger-core/src/decode/watch.rs`, ADR 0018), so a stalled render, for example a reference rendered from the NAS, can take a whole unattended session; (5) `regress` covers only the Stakka mix (`regress::report_names`), while the 2003 and Nocny development mixes, which hold most of the development tracks and the only real version plays, run outside it. Session 77 fixed generated-mix scoring and `regress`'s checks (its notes and handoff). This session fixes the three above in the harness; the product does not change.

You run unattended: the user cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).

Time box: started about 14:05 on 2026-11-07; aim to finish within about 2.5 hours, because more sessions follow you tonight. If a step would run past that, scale it down (keep the watchdog and the regress coverage; leave the render fingerprint's reuse across sessions as a recorded next step), record the decision in the brief and continue.

Environment (sessions 33-77's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,666 records, revision `0bd3f0c23fabc698` (session 75 pruned the records of DJ sets and a mixed CD's track fix). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-54-linux` (262 tracks) and `session-75-linux-nas` (NAS scale); a session that adds baselines names them in its handoff, and the next prompt says which to use.
- Build `work/bin/s77-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Since `45f92d5` (2026-10-28) its calibrate reads only the reports its baseline holds; still run variants and other seeds in a scratch work directory, never in the reports directory `regress` compares.
- The user restructured the code on 2026-10-28 without meaning to change behaviour (commits `b5a84e3` to `f94604a`: harness subcommands in modules, named results, `Search` passed through the core's search, enums for closed sets). Session 60 confirmed both regressions identical on a build of master after it.
- Since sessions 53-56: `SpansAtSpeed(0.006)` is the default second suppression (ADR 0021); same-title detections (`docs/same-titles.txt`) are scored right (ADR 0022); `identify` prints gaps as "ID - ID" lines with an estimate of their tracks (ADR 0023, 0024). The minimum span stays undecided. A change to an existing decision needs a new ADR (the next number is in `docs/adr/`).
- Since 2026-11-05: `docs/session-rules.md` holds the standing rules (the older briefs are history), `docs/decisions-queue.md` the decisions waiting for the user, and the roadmap begins with "The active queue" (`docs/research/2026-11-05-external-review.md` explains why). A session that leaves a decision for us adds it to the decisions queue. The six proposals of `docs/research/2026-10-29-matching-and-archive-proposals.md` have all been measured; a session working on one follows its "Evaluation and adoption boundaries".
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- Tonight (2026-11-07 to 11:00 on 2026-11-08) sessions run one after another, unattended; the user cannot decide or listen. Every listening question asked so far has an answer (docs/status.md, the entries of 2026-10-31 and 2026-11-02), apart from the Planet Dust (DC Breaks Remix) cut (unsure). The user's decisions on a quiet track under an identified one are in docs/status.md, "2026-11-02: the user's answers on a quiet track under an identified one". When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
- The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.
- The repository root holds an untracked `yafc20261031.log` that is not this project's work: leave it alone, never commit or delete it; "`git status` clean" means clean apart from it.

First:
1. Read CLAUDE.md and follow its reading list (the index at the top of `docs/session-notes.md` says where each session's section lives), notes "Session 76" and "Session 77", the review note, ADR 0018, the code of `crates/gunfinger-eval/src/render.rs`, the sweep's and the generated mixes' render paths (`sweep.rs`, `mixes.rs`), the residual's reference render, `crates/gunfinger-core/src/decode/watch.rs`, `regress.rs`, `commands/evaluation.rs`, and how the 2003 and Nocny scans are run today (session 73's notes ran them through the CLI; the sets are `razorjack-2003-03-29` and `nocny-trans-vertical-2002-08-23`, whose manifest is never edited).
2. Write docs/brief-78.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-78-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s78/listening-digests-start.txt. Experiments number from 0280.
3. Build `work/bin/s78-start`; the start regressions follow docs/session-rules.md.

Steps:

1. Design, in the brief before code: the render recipe (source identity: path, size, modification time and, where cheap, a content digest of the decoded or stored audio; excerpt; playback and speed; encoding; faults; the renderer's version, including FFmpeg's), where it is stored (the completion receipt), and what happens on a mismatch (render afresh, never reuse); whether identical recipes may be reused across session directories (only if the receipt matches in full). The watchdog: which harness FFmpeg calls it covers, how a silent encode to file reports progress (for example `-progress pipe:` or `-stats_period`), and the timeouts. The regress coverage: the report names for the 2003 and Nocny scans at 262 tracks and at NAS scale (with `--other-peaks-dir`), and their leave-outs if any (none unless a reason is recorded).
2. The implementation, tested: a unit test where a changed source, a changed fault and a changed renderer version each refuse the cached render; a test of the watchdog on a child that stalls and on one that progresses without writing to standard error (on Linux; say in the brief what remains untested on the Mac); tests that `regress` names the new reports and that a baseline without them is reported as incomplete rather than failed.
3. Runs: with the final build, `regress session-54-linux` and `regress session-75-linux-nas` identical for the reports they hold; then save new baselines `session-78-linux` and `session-78-linux-nas` that include the 2003 and Nocny scans, and check that the new scans' scores equal the last recorded ones (the development mixes: 11/11, 12/12 and 17/17 at both scales, 0 wrong; experiment 0232 and session 73's runs), every difference explained. Renders that the new receipts refuse are rendered again; record how many and how long it took.
4. Experiment (at most 40 lines, only if a run changed a figure or refused renders that earlier experiments used): what the new receipts refused and why.
5. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
6. Wrap-up: a "Session 78" section at the end of docs/session-notes.md with a handoff (the new baselines and what they hold; how long a NAS regress takes now); update docs/status.md, the roadmap's active queue (item 1's state) and the checklist. Final checks: `scripts/check.sh` green, `regress session-78-linux` and `regress session-78-linux-nas` identical with the final build (and `--strict`), docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: the render receipts, the watchdog, the new baselines and their scores, and anything left.

Rules for this session:
- `docs/session-rules.md` binds as well; where it and this list differ, this list wins for this session.
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's behaviour does not change: no threshold, parameter, rule, default or output change; `identify --under` stays off by default (the default is ours, in ADR 0025). Harness changes and variants are tested, opt-in where they are variants, and keep `regress` identical.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
- Scratch data goes under work/s78/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00 on 2026-11-08. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## The state at the start (14:05-14:20)

- Master at `6bcf143` (session 77's wrap-up); `git status` clean apart
  from `yafc20261031.log`.
- `work/bin/s78-start/` is byte-identical to `work/bin/s77-final/`
  (`gunfinger` `46fdc5ea...`, `gunfinger-eval` `12a2d542...`), and the
  stores are unchanged (`scripts/analysis/store_revision.py`: corpus 265
  records at `526a3b978eb3e411`, NAS 40,666 at `0bd3f0c23fabc698`), so
  the start regressions are skipped (`docs/session-rules.md`).
- Start digests in `work/s78/listening-digests-start.txt`:
  `docs/pair-verdicts.txt` `51dd1bba...`, the NAS clusters file
  `c4d3da2e...`, the answer log `3e99ebca...`.
- Baselines `session-54-linux` and `session-75-linux-nas`, each with
  session 77's `provenance.json`. The sweep's renders
  (`work/sweep/seed-2026/`, 720) were made on 2026-10-18 at 19:03, before
  FFmpeg 8.1.3 was installed (23:00 that day). One of them rendered again
  with 8.1.3 differs from the cached file in one byte of the ID3 tag (the
  encoder's version) and decodes to identical samples
  (`work/s78/rerender/`).
- The last recorded scores of the 2003 and Nocny scans: 12/12 and 17/17,
  0 wrong, at both scales (experiment 0232; session 73 ran them through
  `gunfinger-eval scan --set ...`). Their saved reports are from
  2026-10-21 (`work/reports/` and `work/reports/library-nas-dnb-peaks/`).

## Step 1: the design (written before the code)

### The render recipe

A render is reused only when its receipt holds the same recipe as the
render about to be made. The recipe (`render::Recipe`) has three parts:

- **Sources**: each file the render reads, with its path as the harness
  passes it, its size in bytes and its modification time in nanoseconds.
  The metadata is read under the core's `watch::within` and the stall
  timeout, so a hung share fails the render instead of the run.
  *Decision:* no content digest. Hashing would read every source in full
  on each check of a cached render, from the NAS for the references and
  the recall panel, and size with modification time catch a file
  replaced or retagged at the same path. A file replaced by other audio
  of the same size with its modification time preserved is not detected;
  this is recorded as a limitation.
- **Steps**: everything else the caller decides that changes the audio,
  written by the caller with Rust's `{:?}` (floats round-trip exactly):
  the excerpt's start and length, the playback and speed, the filter
  chain, a partner and its start, faults, seeds and the encoding. A
  generated mix's steps are its serialized plan and its stereo fault
  (the plan alone was compared before).
- **Renderer**: `render::RENDERER`, a harness constant bumped by hand
  whenever a change to the harness's rendering code changes what the
  same recipe renders, and FFmpeg's `-version` line, asked once per run
  under the watchdog. *Decision:* the harness build's digest is not part
  of the recipe. It would refuse every render after every build,
  including refactors, and re-render generated mixes of up to 80 minutes
  each time; the hand-bumped constant is the documented rule instead.

**Where it is stored.** The completion receipt beside the render
(`<name>.<ext>.complete`), which held only the size, now holds JSON:
`{"bytes": N, "recipe": {...}}`, written after the render is installed.
Old receipts (a size only) do not parse as a recipe and are refused, so
every cached render is rendered once more on its next use.

**On a mismatch** (another recipe, another size, a receipt that is
missing, unreadable or of the old form): the receipt is removed and the
render is made afresh, installed over the old file as before (a
temporary file renamed into place), and the new receipt written. Nothing
is reused in part.

**Across session directories.** *Decision:* not this session. Renders
stay in the directory each run names, and a receipt is compared only
with the render beside it. Reuse across directories (a store keyed by
the recipe's digest, used only when the receipt matches in full) is a
recorded next step: the time box asks for it to be left first, and not
sharing is the conservative option.

**Callers.** One function, `render::once(output, &recipe, render)`,
used by every cache: the sweep's renders (`sweep::render_all`, also
`recall`'s clean renders), `loss`'s clean control,
`recall::render_damaged`, `shared::render`, `robust`'s queries and
`mixes::render_once` (generated mixes, `shared-mixes`,
`counterexamples`).

### The watchdog

**Covered**: every FFmpeg run of the harness outside test fixtures, all
through `gunfinger_core::decode::watch`:

- samples to a pipe: `render_pcm` (every `render_samples`,
  `render_samples_as_indexed` and `render_stereo_samples`, which include
  the residual's and the version verifier's reference renders) and
  `stereo::decode_stereo`, under `watch::run`, where the samples on
  stdout are the heartbeat, as in the core's decode;
- encodes to a file: `render_excerpt` and `encode_pcm`;
- `ffmpeg -version` for the recipe;
- `Container::of(source)`, the first bytes of the source that
  `render_pcm` and `decode_stereo` read in-process, under `watch::within`.

The listening tool already runs under the watchdog. Test fixtures that
generate tones with FFmpeg are not covered.

**How an encode to a file reports progress.** With `-v error` such an
encode writes nothing. `-progress pipe:1 -nostats` makes FFmpeg write a
block of `key=value` lines to stdout every `-stats_period` (0.5 s by
default). Measured on FFmpeg 8.1.3: 3 s of raw `f32le` written into a
FIFO that is then held open without data, encoded with `ffmpeg -nostdin
-v error -nostats -progress pipe:1 -stats_period 1 -f f32le -ar 44100
-ac 1 -i in.fifo -c:a libmp3lame out.mp3`. FFmpeg went on writing a
block every second while its input stalled, each with
`out_time_us=2822290`, until the writer closed the FIFO. A heartbeat
on any output would therefore never stop a stalled encode. The
heartbeat of an encode is a block whose `out_time_us` or `total_size`
has advanced.

This needs one addition to the core: `watch::run` counts every read as
a beat, and the reader cannot hold one back. *Decision:* add
`watch::run_reporting`, the same run in which stdout counts only when
the reader calls `Heartbeat::beat` (stderr still beats on data, as
before). `watch::run` and every product call are unchanged; the product
does not change. ADR 0018's decision (a stall timeout counted as
silence, 120 s) is unchanged and extended to the harness, so no new ADR.

**The timeout.** `gunfinger_core::decode::STALL`, 120 s, for the reasons
ADR 0018 gives (a NAS spinning up, an SMB client reconnecting). A
stalled render fails with a message naming the output, the program and
the timeout; every caller already returns a render's error, so a run
fails after two minutes without progress instead of holding the
session. The killed child and its reader threads end when the share
answers, as in the core.

**Tests.** Stand-in shell scripts, as `decode::watch`'s tests: an encode
that stalls (stopped after a short timeout), an encode that progresses
for longer than the timeout while writing nothing to stderr (finishes),
an encode whose progress repeats without advancing (stopped), and a
render to a pipe that stalls. They run on Linux; the Mac runs the same
`cfg(unix)` code, but nothing ran there this session (untested on the
Mac).

### The regress coverage

- `regress::report_names` adds `scan-razorjack-2003-03-29.json` and
  `scan-nocny-trans-vertical-2002-08-23.json`, the whole-set scans with
  nothing left out, after the Stakka reports. The same names serve both
  scales: at NAS scale `--other-peaks-dir` writes them under
  `work/reports/library-nas-dnb-peaks/`, beside the Stakka reports.
- *Leave-outs:* none for the two new sets (no reason was found to add
  one: the leave-outs of Stakka already test the left-out path, and the
  saved leave-outs of the other two have no baseline role).
- When `--set` names one of the two, it is not repeated.
- A baseline that lacks a report `regress` now names is **incomplete**:
  `regress` prints "NAME: incomplete: the baseline has no such report"
  (wording as built), compares what
  the baseline holds, and names the missing reports in its last line.
  Without `--strict` it returns success as before; *decision:* with
  `--strict` an incomplete baseline fails, because strict means every
  result was compared.
- The calibration comparison keeps reading only the `--set`'s reports,
  as `calibrate --set` does, so old baselines stay comparable.
- `baseline NAME` saves every named report, the two new ones included.

## Plan

1. The design above (done, committed with this brief).
2. The implementation with its tests, `scripts/check.sh` green, commit.
3. Runs with the build of step 2 (`work/bin/s78-final/` when nothing
   changes after it): `regress session-54-linux` and
   `regress session-75-linux-nas`, identical for the reports they hold
   and reported as incomplete; the sweep's renders refused and rendered
   again (counted and timed); then `baseline session-78-linux` and
   `baseline session-78-linux-nas`, and the new scans' scores against
   0232.
4. An experiment only if a figure moved or refused renders that earlier
   experiments used.
5. A read-only review by a subagent; fixes.
6. Wrap-up and final checks.

## Decisions taken during the session

(Added as they are taken, with the time.)

- **14:10, the core's watcher.** `watch::run_reporting` and
  `Heartbeat::beat` were added as designed. `watch::run` behaves as
  before, but the `gunfinger` binary is no longer byte-identical to
  `s77-final`'s (`5c0e2c66...` against `46fdc5ea...`), because the
  watcher's reader now holds the beat mode. No product behaviour
  changed; both regressions run the core's search and decode.
- **14:12, the recipe type.** `render::Recipe` keeps its name although
  `counterexamples` has a `Recipe` of its own (a transformation recipe);
  `robust` names the render's as `crate::render::Recipe`. A stalled read
  or render now reports "could not <what>: no progress for 120 s, so it
  was stopped", for the source's metadata, its first bytes, a pipe and an
  encode alike.
- **14:14, shared material.** A `shared` query lists its partner as a
  source only for the kinds that play it (damaged, blend), so that a
  partner in the larger library is not needed for the other kinds, as
  before.
- **14:15, `ffplay`.** `pair-review`'s playback (`ffplay`, run while the
  listener listens) stays outside the watchdog: it is the listening
  tool's playback, not a render, and agents never run it.
- **14:16, the last line of `regress`.** It is unchanged when the
  baseline is complete ("every report equal to its baseline's apart from
  timing"). For a baseline that is incomplete and otherwise identical it
  reads "every report the baseline holds equal to its baseline's apart
  from timing; incomplete: the baseline has no ..."; a changed one ends
  with the same "; incomplete: ..." clause. `baseline --provenance-only` skips reports the baseline
  was saved without.
- **14:20, the first corpus regress** (`work/bin/s78-run/`,
  `work/s78/logs/regress-54-run.log`): the four reports
  `session-54-linux` holds are identical, the two new scans are reported
  as incomplete, exit 0, 1:23 wall. All 720 sweep renders were refused
  (old receipts) and rendered again in that run; the new receipts hold
  the recipe.
- **14:21, the scores to compare with.** 0232's 12/12 and 17/17 are the
  NAS-scale figures. At 262 tracks the corpus holds 7 of the 2003 mix's
  tracks and 9 of Nocny's, and the last recorded figures are 7/7 and 9/9,
  0 wrong (0238). The first corpus run gives 7/7 and 9/9, 0 wrong. The
  regress run overwrote the 262-track reports of 2026-10-21; the NAS
  ones were copied to `work/s78/old-reports/nas/` before the NAS run.
- **14:25, a test that relied on old receipts.** `recall`'s
  `a_source_without_a_record_is_left_out_and_the_others_keep_their_order`
  wrote size-only receipts for sources that did not exist; the recipe
  reads each source's metadata, so the first `scripts/check.sh` failed on
  it (`work/s78/logs/check-1.log`). The test now creates the kept sources
  and writes receipts with their recipes through
  `render::excerpt_recipe`, which `excerpt_once` also uses.
- **14:34, the NAS regress** (`s78-final`,
  `work/s78/logs/regress-75-nas-1.log`): the four held reports identical,
  the two new scans incomplete, 4:49 wall (session 77: 3:47), 5.5 GiB.
  The new NAS scans give 12/12 and 17/17, 0 wrong. Against the saved
  reports of 2026-10-21, the only confident difference is the Into Music
  segment that ADR 0021 added (0232); the weak ones (6-20 hits) are of
  files the NAS store gained or lost since (session 75's prune removed
  the `track_fix` files among them).
- **14:36, re-rendering cost.** The corpus regress with every render
  cached took 1:05, against 1:23 when the 720 sweep renders were refused:
  about 18 s to render them again (by difference). The NAS regress reused
  the same renders (`work/sweep/seed-2026/` serves both scales).
- **14:37, new baselines** `session-78-linux` and `session-78-linux-nas`,
  saved by `s78-final` right after its regress runs, each with six reports
  (the sweep, the three development scans, Stakka's two leave-outs) and
  a provenance file.
- **14:40, experiment 0280.** Written because the receipts refused renders
  that every regress-based experiment used; no figure moved. 40,514
  older receipts under `work/` are refused on their next use.
- **14:55, the review** (a read-only subagent; step 5). Its findings and
  what was done:
  1. `harmonic::reference` removed `<name>.wav.complete` after its
     encode with `?`; `encode` writes no receipt any more, so every call
     failed and `shared --harmonic` would have recorded an error for each
     detection (no test calls it; nothing ran it this session). Fixed: it
     removes only the WAV, as `mixes/residual.rs` does.
  2. Report counts: the old baselines hold four reports, the new ones six
     (the brief and 0280 said five and seven). Corrected.
  3. The brief's wording of `regress`'s lines did not match the code.
     Corrected above.
  4. `--strict` on an incomplete baseline said the reports differ. It now
     names the missing reports and says to save a new baseline.
  5. `robust`'s recipe listed the partner and the speech file for every
     condition. It now lists the partner only for the conditions that
     play one and the speech file only for voice-over and insert.
  6. The renderer kept only the first line of `ffmpeg -version`, which
     does not change with the libraries (0280 found the MP3's tag holding
     libavformat's version). It now also holds every `lib...` line of
     that output. LAME's and Opus's own versions are not printed there:
     a limitation. This refuses the 720 sweep renders once more on the
     next run (about 18 s).
- **15:05, the final queue** (`work/s78/final.sh`, final build
  `gunfinger-eval` `8e01bb7a...`, `gunfinger` unchanged at
  `5c0e2c66...`): `regress session-78-linux` and
  `regress session-78-linux-nas` identical, also with `--strict` (1:24
  and 1:05; 4:59 and 5:00). The renderer's new library lines refused the
  720 sweep renders once more, in the first corpus run. The first
  `scripts/check.sh` failed on ADR 0020's "Text file busy" race in
  `decode::watch`'s stand-in tests; it was run again. The baselines were
  kept: the final build reproduces their reports, and the provenance
  prints only another build.
