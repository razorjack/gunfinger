# Session 72 brief: a quiet track under an identified one, in `identify`, behind an option

Written on 2026-11-02, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file (its design above all), and `docs/session-72-checklist.md`
   (progress).
3. `docs/brief-70.md`, `docs/brief-65.md`; `docs/session-notes.md`
   ("Session 65", "Session 70"); `docs/status.md`, "2026-11-02: the
   user's answers on a quiet track under an identified one"; the
   proposals note's §4 and "Evaluation and adoption boundaries"; ADR
   0006, 0017, 0019; experiments 0255-0258 and 0269-0271.
4. The code: `crates/gunfinger-core/src/residual.rs` (moved from the
   harness this session), `crates/gunfinger/src/under.rs`,
   `crates/gunfinger-eval/src/residual.rs`, `mixes/residual.rs`.

`docs/brief-2.md` to `docs/brief-70.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 72 of Gunfinger: find a quiet track under an identified one, in `identify`, as an option that is off by default. Sessions 65 and 70 measured proposal 4 of `docs/research/2026-10-29-matching-and-archive-proposals.md` in the harness (experiments 0255-0258 and 0269-0271; `mixes --kind drops --residual`, `crates/gunfinger-eval/src/residual.rs`): once a play is confident, its file's audio is rendered at the play's time map, aligned on the mixture (*Align*), its gain fitted per band and its power removed from the spectrogram; the residual is searched again. On fresh test seeds at NAS scale, with A's time map from the whole-mix detection and another rip of A, it confirmed 61 of 64 quiet and brief partners (plain 41), 46 confidently (plain 15), with no wrong confident answer. On 2026-11-02 the user answered the questions an ADR needs (docs/status.md, "2026-11-02: the user's answers on a quiet track under an identified one"): on by default subject to its cost; today's acceptance rule (ADR 0006); the printing to be tried in use; related versions not printed at first; the cost measured before the default is decided; the best-aligned rip as the reference. This session builds it into the product behind an option, writes an ADR with status Proposed, and keeps the default unchanged; session 73 measures it to decide the default.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).

Time box: started about 04:30; aim to finish within about 3 hours (by about 07:30), because session 73 follows you tonight and must finish by 10:40. If a step would run past that, scale it down (keep the option, its tests and the ADR; leave CSV and cue details as recorded decisions), record the decision in the brief and continue.

Environment (sessions 33-70's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,674 records, revision `7210fc479da58376` (session 64 pruned four ignored uploads). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-54-linux` (262 tracks) and `session-64-linux-nas` (NAS scale; compare with it, not `session-54-linux-nas`).
- Build `work/bin/s70-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Since `45f92d5` (2026-10-28) its calibrate reads only the reports its baseline holds; still run variants and other seeds in a scratch work directory, never in the reports directory `regress` compares.
- The user restructured the code on 2026-10-28 without meaning to change behaviour (commits `b5a84e3` to `f94604a`: harness subcommands in modules, named results, `Search` passed through the core's search, enums for closed sets). Session 60 confirmed both regressions identical on a build of master after it.
- Since sessions 53-56: `SpansAtSpeed(0.006)` is the default second suppression (ADR 0021); same-title detections (`docs/same-titles.txt`) are scored right (ADR 0022); `identify` prints gaps as "ID - ID" lines with an estimate of their tracks (ADR 0023, 0024). The minimum span stays undecided. A change to an existing decision needs a new ADR (the next number is in `docs/adr/`).
- The research note `docs/research/2026-10-29-matching-and-archive-proposals.md` (2026-10-29) records six unmeasured proposals and its evaluation boundaries; any session working on one follows its "Evaluation and adoption boundaries".
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen. Every listening question asked so far has an answer (docs/status.md, the entries of 2026-10-31 and 2026-11-02), apart from the Planet Dust (DC Breaks Remix) cut (unsure). The user's decisions on a quiet track under an identified one are in docs/status.md, "2026-11-02: the user's answers on a quiet track under an identified one". When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
- The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.
- The repository root holds an untracked `yafc20261031.log` that is not this project's work: leave it alone, never commit or delete it; "`git status` clean" means clean apart from it.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-65.md, docs/brief-70.md and the newest brief, docs/session-notes.md ("Session 65", "Session 70" and the newest session), docs/status.md (the newest entries, the user's answers above in particular), the proposals note (§4 and "Evaluation and adoption boundaries"), ADR 0006, 0017, 0019, 0021 and 0022 and how ADRs are written here, experiments 0255-0258 and 0269-0271, and the code of the harness's residual (`crates/gunfinger-eval/src/residual.rs`, `mixes/residual.rs`), the core's search and plays (`search/`, `plays.rs`), decoding (`decode.rs`), the library and store (how `identify` reaches a matched file's audio, and what `--store-only` lacks), and the CLI's `identify` and its output formats.
2. Write docs/brief-72.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-72-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s72/listening-digests-start.txt. Experiments number from 0272.
3. Build `work/bin/s72-start`; `regress session-54-linux` and `regress session-64-linux-nas` identical.

Steps:

1. Design, recorded in the brief before code: the option's name (for example `identify --under`), where the step runs (after plays are grouped, for each confident play; the best-aligned rip of its file's duplicate cluster, as the user chose, decoded from the library); the alignment, gain fit and mask as session 70 froze them (*Align*, `over` 2), moved into the core rather than reimplemented, with the harness then calling the core's version so the measured code is the shipped code; the acceptance rule (today's, unchanged, applied to the residual's search); what is dropped (a residual detection of a file that shares material with the masked play, or of the masked file's own duplicate cluster: the product's reading of the user's "related versions not printed at first", since a related version of the quieter track cannot be told from the right recording; say so in the ADR); what `--store-only` does (the step does not run, and the output says why in one line); and the printing, a first option for the user to judge in use: a confident partner gets its own tracklist entry at its start time, marked as under the entry of the louder track (for example "under 6."); possible partners only in the table, CSV and JSON, as ADR 0019 does today; the cue sheet leaves partners out for now (a cue sheet cannot hold two tracks at once); JSON and CSV carry the partner's link to the masked play. Saved reports stay readable by `show`.
2. The implementation, tested: unit tests for the moved signal code (synthetic A over B: the mask removes A's peaks and keeps B's), for the selection of the reference and the drop rule, and for the printing in every format; an end-to-end test on a generated double drop if one is cheap. With the option off, every output and both regressions are byte-identical; the harness's residual reports from session 70's build and the new build agree on a development seed (the same counts), or every difference is explained.
3. A first look, not the measurement: run the option on one development mix at 262 tracks and NAS scale and on one generated drops mix, and record what the tracklist shows and the added time, for session 73.
4. ADR (next number in `docs/adr/`), status Proposed: the decision as built, the user's answers, what stays open (the default and the cost, decided after session 73; the printing, to be judged in use), and the consequences (reference audio required; the cost per confident play). Update the proposals note's §4, the roadmap and the README or CLI help where the option is documented.
5. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
6. Wrap-up: a "Session 72" section at the end of docs/session-notes.md with a handoff for session 73 (the option, how to run it, the first look); update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-54-linux` and `regress session-64-linux-nas` identical with the final build (option off), docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: the option as built, the printing, the tests, the first look, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The default behaviour does not change: the new quiet-track step is an option, off by default, and with it off every output and both regressions stay identical; no threshold, parameter, rule or default changes. The option, its output, its tests and an ADR with status Proposed are this session's product work, as the user decided on 2026-11-02. Harness changes are tested and keep `regress` identical.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
- Scratch data goes under work/s72/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00 on 2026-11-02. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 72

- 2026-11-02, 04:25 by the Linux machine's clock (CET). Aim to finish by
  about 07:30 (session 73 follows tonight); no new run after 10:20 in
  any case.
- `master` at `2701507`, clean apart from the untracked
  `yafc20261031.log`. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s72/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `c4d3da2e7e37...`, `work/audition/answers.jsonl` `3e99ebca589b...`
  (records added by the user's listening of 2026-10-31).
- Experiments 0001-0271; this session numbers from 0272. ADRs
  0001-0024; the next is 0025. Test set: 4 of 5 evaluations used (none
  in this session).
- The NAS store: 40,674 records, revision `7210fc479da58376`. Build
  `work/bin/s70-final/` (master). Baselines `session-54-linux` and
  `session-64-linux-nas`.
- No brief 71 exists: session 71 did not run in this checkout; the
  newest brief is 70.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build and both regressions (tmux `s72start`) | 04:25-04:45 |
| 1 | design, below, committed before code | 04:45 |
| 2 | core `residual` (moved), core `decode::Decoder::render`, CLI `under`, printing, tests; harness on the core's code; `check.sh`; both regressions and the harness's residual report identical | 04:45-06:15 |
| 3 | first look: one development mix at 262 tracks and NAS scale, one generated drops mix | 06:15-06:45 |
| 4 | ADR 0025 (Proposed), proposals note §4, roadmap, README and help | 06:45-07:05 |
| 5 | read-only review by a subagent; fixes | 07:05-07:20 |
| 6 | notes, status, checklist; final checks | 07:20-07:40 |

## Design (step 1, written before code)

**The option.** `gunfinger identify --under`, off by default. No
configuration-file key and no environment variable this session (the
default is decided after session 73; a key can follow the decision).
Help: also search under each confident track for a quieter one, by
taking that track's audio out of the mix; needs the library's audio.

**Where the step runs.** In `identify`, after the report is built (plays
grouped, same-audio plays merged, the marks of ADR 0019 and 0017
derived) and before the gaps' repeats. It runs once for each confident
tracklist entry (`export::entries`: the plays that are one track), not
for each play, so two rips of one recording found over the same passage
are taken out once.

**The reference: the best-aligned rip.** The product has no duplicate
clusters (they are a harness file made by `gunfinger-eval clusters`);
the files of A's recording that it knows are the entry's plays and
their same-audio files. Of those, the reference is the entry's strongest
confident play (the most hits): the file whose stored peaks the mixture
supports best, the product's reading of the user's "the best-aligned rip
of the louder track". Its audio is decoded from the library
(`catalog.root` joined with its path).

**The time map.** Each segment of the reference play places its file by
its own line: from `track_start_seconds`, at its speed and playback
(turntable or key lock), over the segment's span. The excerpt searched
is the play's span (first segment's start to last segment's end); in a
gap between segments the reference is silent, so nothing is taken out
there. This is session 70's detected timing (A's line from the
whole-mix detection), per segment instead of one line extended over the
truth's audible span, which the product does not know.

**Rendering.** A new core function decodes a stretch of a file as a mix
played it (`decode::Decoder::render`): FFmpeg seeks to the segment's
file time and plays it through the harness's filter chain (`aresample`
to 44.1 kHz, then `asetrate` and `aresample` for a turntable, `atempo`
for key lock) down to the profile's rate, mono, under the stall timeout
(ADR 0018). The harness renders at 44.1 kHz, writes a WAV and decodes it
to the profile's rate; the product goes straight to the profile's rate.
The gains are fitted from the mixture, so the reference's level does not
matter.

**The mask, moved into the core.** The harness's residual, lags,
shift and gain fit (`crates/gunfinger-eval/src/residual.rs`) move to
`crates/gunfinger-core/src/residual.rs` unchanged in arithmetic. The
core takes a `Mask` (`over`, whether the reference is aligned, the gain
blocks' length); `Mask::UNDER` is session 70's frozen setting: *Align*,
`over` 2, blocks of 63 frames. The harness keeps its `Rule` and `Fix`
(clap and serde), maps them to a core `Mask` and calls the core, so the
measured code is the shipped code; a serde mirror of `Lags` stays in the
harness (the core has no serde). `harmonic.rs` uses the core's
`spectra` and `overlap_add`.

**The search and the acceptance rule.** The residual is searched with
the core's search, the same index, ladder and matcher options as the
whole mix; today's confidence rule (ADR 0006, ADR 0013) applies
unchanged. Weak detections are left out as `plays::group` does.

**What is dropped.** A residual detection is not reported when its
file:

1. is a file of the masked entry (its plays' files and their same-audio
   files);
2. shares material with the reference file by their stored peaks: the
   detection's file's stored peaks, searched against an index of the
   reference file alone on the request's ladder (`search_peaks`, as the
   harness's rip line), give a detection at the possible tier or above.
   ADR 0006 defines that tier as "this recording, or one sharing
   material with it", so this covers other rips of A (its duplicate
   cluster) and related versions with a passage in common. This is the
   product's reading of the user's "related versions not printed at
   first": a related version of the quieter track cannot be told from the
   right recording, and one of A is held back by this rule; tracks that
   share only a sampled break may be held back too (precision first);
3. has a confident play in the whole-mix report that overlaps the
   detection or lies within `plays::group`'s 90 s of it: the tracklist
   already names that track there (the common double drop, in which the
   quieter track goes on alone). Its earlier start is not shown.

The kept detections of each entry are grouped into plays
(`plays::group`, `merge_same_audio`); a partner play of a file that
another entry's partner play overlaps is kept once, the one with more
hits.

**`--store-only`.** The step does not run, and `identify` prints one
line on stderr saying why (the library's audio is needed to take a track
out). A reference file that cannot be decoded is passed over with one
warning line; the other entries still run.

**The report.** `Report::under: Option<Vec<FoundPlay>>`: absent when
the step did not run, an empty list when it ran and found nothing.
Each partner is a `FoundPlay` with `under: {play, asset}` (the masked
entry's reference play, numbered from 1 in `plays` as ADR 0019's
`shares_material_with` numbers them). `SearchSettings::under` holds the
step's design string when it ran (absent otherwise), so `--save-dir`
searches again when a saved report was made with the other setting.
Both new fields are skipped when absent, so with the option off every
output is byte-identical. Older reports load (serde defaults); `show`
renders the partners of a saved report.

**The printing, a first option to judge in use.**

- Tracklist: a confident partner gets its own numbered entry at its
  start time, marked `(under N.)`, N being the tracklist number of the
  louder track's entry. Possible partners stay out (ADR 0019).
- Table (`--format human`): every partner as a row in time order with
  "under <file> (play N)", as ADR 0019's "shares material with" reads.
- CSV: when the step ran, a last column `under_play` (the masked play's
  number) and one row per partner; without the step the header and rows
  are as today.
- JSON: `under` as above.
- Cue sheet: partners left out (a cue sheet cannot hold two tracks at
  once). Timeline: partners left out this session (recorded decision;
  the timeline draws `plays`).
- Gaps: unchanged; a partner lies inside a confident play, which covers
  its time already.

**Tests.** The moved signal tests (A over B: the mask removes A's peaks
and keeps B's; alignment); the reference's choice; each drop rule; the
printing in every format, with and without partners, and a saved report
read back; an end-to-end test on a synthetic double drop through the
binary if a level can be found at which the plain search misses B.

## Decisions

- **(04:25-04:35) Start build and regressions** (`work/s72/start.sh`,
  tmux `s72start`, logs in `work/s72/logs/`; `run.sh` and `same.py` are
  session 70's with the paths changed). Cargo had nothing to rebuild:
  the start build is byte-identical to `work/bin/s70-final/`; `regress
  session-54-linux` (0:51) and `regress session-64-linux-nas` (4:10)
  identical, every report equal to its baseline's.
- **(04:36) The core's `Mask` instead of the harness's `Rule` and
  `Fix`.** The core has no serde or clap, so it takes a plain `Mask`
  (`over`, `Alignment::{AsPlaced, Aligned}`, `block_frames`) and the
  harness maps its `Fix` onto it (`Fix::mask`, tested: `Align` is
  `Mask::UNDER`). `FINE_BLOCK_FRAMES` (32) stays in the harness, the only
  user. The harness keeps a serde mirror of `Lags` and `BlockLag`
  (`From` the core's). `Report`'s JSON layout is unchanged.
- **(04:39) The harness on the core's code gives session 70's report
  exactly**: `work/s72/smoke-harness.sh` (seed 2026, one mix, both
  timings, three fixes, 262 tracks; 3:57) against
  `work/s70/smoke/reports/...-fix-off-align-align-fine.json`: identical
  apart from `wall_seconds` (`same.py`).
- **(04:40) Store fallback.** The step needs the library's files, so it
  does not run whenever the catalog lists the store's own records:
  `--store-only`, no library given, or a library that cannot be read
  (`Indexable::reads_library`). The line on stderr is an info line, as
  the store fallback's own line is.
- **(04:41) `plays::MAX_GAP_SECONDS` made public** for the drop rule that
  a confident play of the same file within 90 s names the track already;
  its value and use in `plays::group` are unchanged.
- **(04:45) Names of plays found under others come from their stored
  tags**: `output::render`'s name lookup reads `plays_and_under()`, the
  plays and the found plays in time order (the plays keep their order,
  so without the step nothing changes).
- **(04:50) The end-to-end test** (`a_quiet_track_under_a_confident_one_is_found_once_the_louder_is_taken_out`):
  a probe at -6, -9, -12, -15 and -18 dB found the quiet track only as
  possible in the plain search (181 to 98 hits, left out of the
  tracklist by ADR 0019) and confident under the louder one (798 to 536
  hits). The test uses -12 dB and checks the tracklist with and without
  the option, the JSON, `show` of the saved report and `--store-only`.
  It takes about 40 s in a debug build. The probe file was deleted.
- **(04:53-05:00) With `--under` off, every output is byte-identical**
  (`work/s72/off.sh`, build `work/bin/s72-run/`): `identify --format
  json` by `s70-final` and by the run build, on the three development
  mixes at 262 tracks and NAS scale, give the same bytes; `show` of each
  report in every format (tracklist, human, timeline, CSV, cue, JSON) by
  both builds gives the same bytes: 42 of 42 identical. `regress
  session-54-linux` (1:06) and `regress session-64-linux-nas` (5:22,
  beside `off.sh`) with the run build identical, every report equal to
  its baseline's. `scripts/check.sh` is run at the end.
- **(05:00-05:04) First look, first pass** (`work/s72/look.sh`, run
  build): the step works on real and generated mixes, and showed two
  gaps in the design, fixed before the first look was run again. (1) At
  NAS scale the quiet partner of generated mix-00 was found through two
  files (`Original Mix` and an `Extended Mix` upload, lining up), and the
  tracklist printed them as two entries: plays found under others are
  now grouped into entries by the tracklist's own rule
  (`export::entries_of`, the code of `entries`). (2) A found play can be
  another upload of a track already confident over the same passage (the
  Gridlok and Echo Chamber partners at NAS scale, found as possible
  through other uploads): a found play that is one track with a
  confident play of the recording by ADR 0017's rule (`same_recording`
  or `same_tagged_track`) is now dropped (`under::named_as_one_track`),
  a fourth drop rule. The script's loop variable collided with
  `run.sh`'s global `name`, which garbled its output names (not the
  runs); fixed.
- **(05:06-05:09) First look** (`work/s72/look.sh` again, build
  `work/bin/s72-run2/`): experiment 0272. Stakka: nothing found under a
  confident play at either scale. Generated mix-00: the quiet partner
  (Muffler - Nutcracker) found confidently under entry 5 at both scales,
  one entry (two files at NAS scale, `titles disagree:`). The step took
  15.9-32.9 s for 11-17 confident entries.
- **(05:18) `scripts/check.sh` green** on `09014c6` (7:59).
- **(05:25) The review's findings** (a separate agent, read-only,
  05:11-05:17). It found no path that changes output with the option
  off, the moved residual code equal in arithmetic, `Decoder::render`'s
  arguments and the time map right, and the `(under N.)` number right.
  Taken: CSV rows no longer give play numbers once found plays are rows,
  so with the step CSV gains a `play` column beside `under_play`; the
  third and fourth drop rules compared files and overlapping passages,
  so a quieter track found through one file and going on alone through
  another upload would have been listed twice: the fourth rule now drops
  a found play that lines up with or is tagged like a confident play
  overlapping it or within 90 s (ADR 0017's tests without the overlap
  condition); validation rejects a play of `plays` marked `under` and a
  found play without a valid `under`, with its own messages; the
  shared-material check is built only when a detection needs it, and
  the warning says that nothing is reported under the play; the
  name lookup chains the two lists instead of collecting them; the
  store-only line reads plainly; 0272's facts (the plain search had the
  partner as possible, 207 hits; its span 11:08-12:27; the NAS entry
  starts at 11:07); ADR 0025 no longer quotes the user and cites the
  status entry of 2026-10-31 for the user's view on related versions;
  the proposals note names the fourth rule. Not taken: AGENTS.md's module
  map (it names neither the core's `residual` nor the CLI's `under`; left
  for the user, as session 70 left the harness's `residual`); a test of
  `--under` with `--start` (the arithmetic was checked by reading).
