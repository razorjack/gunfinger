# Session 48 brief: telling a record playing from shared material

Written on 2026-10-25, so the plan survives
context compaction. Work autonomously and never stop to ask: where a
decision is needed, take the conservative option, write it down here and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-48-checklist.md` (progress).
3. ADR 0006, 0017 and 0019; experiments 0182-0185 and 0210, and this
   session's (0215 and up); the roadmap's "Possible plays inside a
   confident play of another recording", "A confidence statistic for
   shared material" and "The printed tracklist".
4. `docs/session-notes.md` ("Session 39", "Session 45", "Session 47"),
   `docs/status.md` (newest entries); the code of
   `crates/gunfinger/src/report.rs` (`mark_shared_material`,
   `same_recording`), `export.rs` (`entries`),
   `crates/gunfinger-eval/src/mixes/drops.rs`, `tempo.rs`, `pair.rs`,
   `verifier.rs`; `scripts/analysis/tracklist_score.py`,
   `tracklist_marked.py` and this session's `marked_features.py`.

`docs/brief-2.md` to `docs/brief-47.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 48 of Gunfinger: telling a record playing from shared material, among the plays the tracklist leaves out. ADR 0019 leaves out of the tracklist a possible play inside a passage of a confident play of another file (a "marked" play), because it is usually material a related version shares. Session 39 (experiments 0182-0185) measured its cost: of 43 records the search finds as possible inside a partner's passage, the tracklist prints 2 and leaves out 41 (double drops, quiet and brief records), and 0183 found that nothing the report holds (hits, length, the ratio to the partner) separates those from shared material (the Clockwork pair's 64 related plays). The roadmap's "A confidence statistic for shared material" names a hypothesis: evidence outside passages the library itself shows to be shared. An unrelated drop partner shares nothing with the confident play's file; a related version's segment maps, through the alignment between the two files, onto the moment the confident play is at; a record playing under its relative brings evidence from its own distinctive material. This session measures features of that kind on mixes with exact truth, and how often double drops whose drops are lined up in phase merge two records into one entry (session 39 matched tempo, not beat phase; DJs line drops up). It is a measurement session: the tracklist's rules do not change; a change is ours, with an ADR.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32). Rendering runs well in parallel here.

Time box: started about 02:40 (CET; the clocks went back an hour tonight); aim to finish within about 2.5 hours, because other sessions follow you tonight. If a step would run past that, scale it down (262 tracks first, fewer seeds), record the decision in the brief and continue.

Environment (sessions 33-47's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS store `~/.local/share/gunfinger/nas-dnb-peaks` (40,683 records), library `/mnt/atlas/Music/dnb` (read only; on Ethernet again tonight), reached with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: after the listening of 2026-10-25 (docs/status.md, "after session 46") SHA-256 `fdf7428a6322...`, the same 284 clusters as before. Baselines `session-32-linux` and `session-33-linux-nas`. Build `work/bin/s47-final/` (master). Since sessions 41, 43 and 46 a NAS-scale development scan searches in about 24 s and `regress session-33-linux-nas` takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans, sweep reports of seeds other than 2026 (calibrate reads every sweep report there), and reports of opt-in variants must not sit in a reports directory during `regress`; run variants in a scratch work directory.
- Session 38's opt-in `--apart-in-speed` (confirmed by session 45; adoption ours, with an ADR) and session 37's `--overlap-by-chains` are variants, not defaults.
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen tonight. Pending listening (items 4, 5, 7 and 9 of session 40's pack, in `work/s47/questions/05-pack-s40.json`, session 39's questions, Star Trails, Into Music) stays for later.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-39.md and docs/brief-47.md, docs/session-notes.md ("Session 39", "Session 45" and "Session 47"), docs/status.md (newest entries), ADR 0006, 0017 and 0019, experiments 0026, 0076, 0091, 0102, 0107, 0182-0185 and 0210, the roadmap's "Possible plays inside a confident play of another recording", "A confidence statistic for shared material" and "The printed tracklist", and the code of `plays.rs`, the CLI's `export.rs` (`entries`, the marking), `mixes/drops.rs`, `tempo.rs`, `pair.rs`, `verifier.rs`, and the scripts `tracklist_score.py` and `tracklist_marked.py`.
2. Write docs/brief-48.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-48-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt in work/s48/listening-digests-start.txt; it must be the same at the end. Experiments number from 0215.
3. Build `work/bin/s48-start`; `regress session-32-linux` and `regress session-33-linux-nas` identical.

Steps:

1. Pre-registration, committed before any feature is computed on new mixes: the features (at least: whether the two files share material at all by the library's own evidence, from `pair` between the marked play's file and its partner's, or the clusters' related pairs; where the marked segment falls through that alignment relative to the confident play's position at the same moment; the marked play's hits outside the passages the two files share; the peak verifier's share on the marked passage; the speed relation to the partner), each defined exactly; the development data (session 39's renders or the same seeds rendered again: 2026 and 2028) and fresh test seeds that no feature is tuned on (at least two); the scores (marked plays that are records playing against those that are shared material, per scenario: separation, the share of playing records a rule would print and the share of shared material it would let into the tracklist); and what would count as a feature worth an ADR proposal. Also the merge measure: with drops lined up in beat phase, how often `same_recording` joins two different records into one entry, against session 39's tempo-only drops.
2. The harness, tested: beat-phase alignment for the drops kind (an opt-in option, so session 39's renders and reports stay reproducible), and the features in a script or the harness (harness only; `regress` identical).
3. The runs: the development seeds, then the test seeds, at 262 tracks and, if time allows, at NAS scale (a store-only copy without the held-out files, as session 39 did; never write the NAS store), under tmux with logs.
4. Experiments (at most 40 lines each, written before the next starts): the features on the development seeds; the registered rule on the test seeds; the phase-aligned merges.
5. Ahead, only when 1-4 are done: the same features on the development mixes' saved NAS-scale reports (the five marked plays of 0183's last paragraph), as a check that the rule would not print known shared material there.
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 48" section at the end of docs/session-notes.md with a handoff and the decision it leaves for us; update docs/status.md, the checklist and the roadmap's bullets above. Final checks: `scripts/check.sh` green, `regress session-32-linux` and `regress session-33-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: the features' separation on the test seeds, what a rule would print and let in, the merges, and anything left.

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
- Scratch data goes under work/s48/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:59. Start no new run after about 11:30; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 48

- 2026-10-25, 02:35 by the Linux machine's clock (CET). The time box
  is read as 2.5 hours from about 02:40: finish by about 05:10.
- `master` at `28615bf`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s48/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `d3bb1fe061f7...`, the NAS clusters file
  `fdf7428a6322...`, `work/audition/answers.jsonl` `78c3fb0e7ce0...`
  (recorded as well, though no step here touches it).
- Experiments 0001-0214; this session numbers from 0215. Test set: 4 of
  5 evaluations used (none in this session).
- The start build `work/bin/s48-start/` is `target/release` of
  `28615bf`; cargo had nothing to rebuild, and both binaries are
  byte-identical to `work/bin/s47-final/`'s.
- Session 39's data: renders `work/mixes/drops-seed-{2026,2028}` and
  `drops-other-pairs-seed-{2026,2028}`; harness reports
  `work/reports/mixes-drops-seed-*.json`,
  `work/reports/mixes-drops-other-pairs-seed-*.json`; the CLI's reports
  `work/s39/cli-262/`, `cli-nas/`, `cli-nas-other/`; store-only copies
  `work/s39/store-{2026,2028}/`, NAS exclude lists
  `work/s39/nas-exclude-{2026,2028}.txt`, tag lists
  `work/s39/tags-262.json` and `tags-other-pairs.json`, panels
  `work/s39/panels/`.
- Sweep panels exist for seeds 2026-2029 and 7. Of these, 2027 and 2029
  hold a Clockwork record out (as session 39 found), so the drops kind
  at 262 tracks has no pair there. A new seed's sweep panel is drawn
  on first use into the `--panels` directory; this session's runs use a
  copy, `work/s48/panels/`, so no committed panel changes.
- Disk: 794 GB free; `work/mixes/` holds 23 GB.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build and both regressions (tmux `s48start`) | 02:35-02:55 |
| 1 | pre-registration in this brief; commit | 02:55-03:10 |
| 2 | `--aligned-drops` (drop points, beat phase) with tests; `marked_features.py`; `check.sh`, both regressions | 03:10-03:50 |
| 3 | test seeds: panels, store copies, renders, CLI at 262 and NAS scale; `pair` and the verifier for every marked play; aligned renders and the CLI | 03:30-04:20 |
| 4 | experiments 0215 (development), 0216 (test), 0217 (merges) | 04:00-04:35 |
| 5 | the development mixes' NAS-scale reports, if 1-4 are done | 04:35 |
| 6-7 | review, notes, status, roadmap, final checks | 04:35-05:10 |

If the time box runs short: NAS scale for the test seeds' other pairs
goes first, then the aligned drops at NAS scale.

## Decisions

- **(02:36) Start build and regressions** (`work/s48/start.sh`, tmux
  `s48start`, logs in `work/s48/logs/`; `run.sh` and `same.py` are
  session 47's with the paths changed).
- **(02:41) Start checks.** `regress session-32-linux` (0:51) and
  `regress session-33-linux-nas` (3:45) identical: every report
  identical to its baseline's apart from `wall_seconds`
  (`work/s48/same.py`).
- **(02:43) Step 1, the pre-registration** (committed before any
  feature is computed on new mixes, and before any of session 39's
  marked plays is measured by these features).

  *Marked plays and their classes.* A marked play M is a play of a CLI
  report (`identify --save-dir`) with `shares_material_with`; its
  partner P is the play that names (`plays[play - 1]`). Classes, from
  the mix's truth as `tracklist_marked.py` labels them: *playing* (a
  truth play of M's recording overlaps M; split by the truth play's
  scenario: unrelated drop, quiet, brief, pair drop), *shared* (no
  such play, but a truth play of M's family overlaps M: a related
  version playing), *other* (neither; reported apart, in no rule's
  score). Counts are plays, as in 0183.

  *The library's evidence.* For each (M's file, P's file), `gunfinger-eval
  pair` with M's file as the query and P's as the found file (at NAS
  scale both as `second-library/<path>` with `--other-peaks-dir`),
  search "peaks, turntable ladder" (stored peaks, 0.92-1.08; the same
  search at both scales). Its alignments map M's file time q to P's
  file time `offset_seconds + speed × q`. An alignment *counts* when
  it has at least 60 hits (`MIN_POSSIBLE_HITS`, what a search would
  call possible). The shared spans of M's file are the counting
  alignments' query spans.

  *Features* (t: the midpoint of M's strongest segment, in mix time;
  m: M's file time at t, linear within that segment; p: P's file time
  at t, from P's segment holding t, else the nearest segment
  extrapolated at its own rate):
  - F1, shares at all: the strongest alignment's hits (0 when none).
    Beside it, F1c: whether the clusters file (262-track or NAS, by
    scale) lists the two files as a pair, either way (yes or no).
  - F2, where M falls through the alignment: the least `|offset +
    speed × m - p|` over the counting alignments whose query span,
    widened by 10 s on each side, holds m; none when no alignment
    qualifies.
  - F3, evidence outside the shared spans: for each of M's segments,
    its hits times the share of its file span (track start to end)
    outside the shared spans, summed (apportioned hits); beside it the
    file seconds outside.
  - F4, the peak verifier's share: the harness (`mixes --kind drops
    --verify --ladder turntable` on the same renders, 262 tracks only,
    since `mixes` indexes the corpus store only) detection of M's file
    overlapping M's strongest segment most; F4 is its
    `verified.aligned.share`, beside it the mean `shifted` share
    (chance) and their difference.
  - F5, the speed relation: r = P's speed / M's speed (each play's
    `speed_percent`, the strongest segment's); with F2's alignment,
    `|r / speed - 1|`, else `|r - 1|`.

  *Rules, fixed now* (a marked play the rule calls *playing* would be
  printed): R1: F1 < 60 (no counting alignment). R2: F2 > 10 s or none.
  R3: F3 >= 60 apportioned hits. F4 and F5 have no threshold chosen in
  advance: theirs is set on the development seeds as the value that
  prints the most playing plays while letting in no shared one, and is
  then fixed for the test seeds. R1-R3 are reported with their
  development-optimal thresholds too, but judged at the fixed ones.

  *Data.* Development: session 39's renders and CLI reports, seeds
  2026 and 2028 (`cli-262`, `cli-nas`, `cli-nas-other`). Test: the first
  two seeds from 2030 up whose sweep panel keeps both Clockwork records
  indexed (needed for the pair at 262 tracks), drawn into
  `work/s48/panels/`; rendered with `mixes --kind drops --tagged
  work/s39/tags-262.json` (and `--other-pairs --tagged
  work/s39/tags-other-pairs.json` for NAS scale), searched by the CLI
  as session 39 did: 262 tracks with a store-only copy less the seed's
  held-out files, NAS scale with the NAS configuration, `--saved-index
  off` and `--exclude-from` the NAS files of every NAS cluster holding a
  held-out corpus file (`tracklist_score.py store` and `exclude`). No
  feature or threshold is changed after the test seeds are measured.

  *Scores*, per scale and seed group (development, test): per feature
  the class ranges (min, median, max) by scenario; the separation as the
  AUC of playing against shared (the chance that a random playing play
  is on the rule's printing side of a random shared play, ties counted
  half); per rule the share of playing plays printed (by scenario) and
  the share of shared plays let in.

  *Worth an ADR proposal:* a rule whose threshold was fixed before the
  test seeds that on the test seeds, at every scale searched, prints at
  least 80% of the playing marked plays of the unrelated scenarios
  (unrelated drop inside, quiet, brief) and lets in no shared play, and
  in step 5 lets in none of the development mixes' five marked plays at
  NAS scale (shared material by 0102 and 0107). A rule meeting the print
  bar while letting in at most 2 shared plays per scale is "promising,
  more data first". Pair drops (a record under its relative) are
  reported, not held to the bar: the pair drop aligns the two passages,
  so the evidence there may be the same relation of audio (0183).

  *The merge measure.* `mixes --kind drops --aligned-drops` (step 2):
  in both unrelated drops of a mix, each record's strongest drop it can
  play at that moment (the largest rise of loudness, snapped to the
  beat) is placed at the same mix time, so the drops coincide in beat
  phase; everything else (records, overlays, speeds, the other blocks)
  is drawn as without the option. Seeds 2026 and 2028 and the two test
  seeds, the CLI at 262 tracks (NAS scale if time allows). Scores: per
  aligned unrelated drop block, whether both records' plays are in one
  printed entry (`tracklist_score.py`'s merged across families),
  against the same seeds' tempo-only drops (session 39: 1 of 64 at 262
  tracks); beside it the share of blocks whose drops were placed and
  the share whose tracks would have started within 5 s by the truth
  (`start - source_start / speed`, what `same_recording` compares).
  Worth a roadmap entry (session 39's P3 bar): 5% or more of the
  aligned blocks merged.

  *The drop point* (harness, fixed now, not tuned on any feature): each
  record's audio at native speed, mono at the render rate; loudness in
  dB per 0.5 s frame (RMS); the rise at t is the mean loudness over
  [t, t + 8 s) less that over [t - 8 s, t). Within a window of file
  times, the drop is the t of the largest rise, moved to the beat: the
  position within half a beat period of t at which the onset envelope
  summed over the 16 beats from there is largest (the beat period of
  `beat_periods`). The first record's window is the file times it can
  be at when both reach full level (its play's start, length and speed,
  the overlay's solo time and fades); the second's those from which its
  play, entering with its fade, fits in its track. Where a window is
  empty or a record has no beat period, the block keeps session 39's
  placement and its plays carry no drop time.
- **(02:52) Step 2, the harness.** `mixes --kind drops --aligned-drops`
  (`tempo::Structure`, `drops::line_up_drops`, `drops::structures`):
  for the two unrelated double drops of a mix only (the pair drop keeps
  its passages aligned, the quiet and brief records are not drops); the
  report and renders are labelled `aligned-drops`; the option conflicts
  with `--other-pairs`. Plays gain `drop_seconds` (left out of the JSON
  when unset), so the default's truth files and reports are unchanged
  and session 39's renders stay valid. The draw consumes the same random
  numbers with and without the option, so the records, overlays and
  speeds of every block are the same as the tempo-only mixes of the
  same seed (a test checks it). Decisions while writing it: the onset
  envelope sees an onset up to one STFT window early (23 ms at the
  render rate), alike for every record, so it cancels between two
  records; the rise is found to half a second and the snap looks within
  half a beat, so on music with equally strong beats the drop can land
  one beat from the rise's true beat (the test allows one beat); the
  drop is still on a beat, so the two records stay in beat phase. The
  structures are computed only with the option, from each long record's
  whole audio at native speed. Features: `scripts/analysis/marked_features.py`
  (`pairs`, `features`, `summary`, `known` for step 5, `merges`), with
  `test_marked_features.py`. The merge command gives session 39's
  tempo-only drops 0 of 32 (2026) and 1 of 32 (2028) at 262 tracks, as
  0182 found.
- **(02:52) NAS scale through the store.** The CLI at NAS scale reads
  the NAS store's own records (`--peaks-dir` the NAS store,
  `identify --store-only --saved-index off --exclude-from`), as session
  45 did, instead of listing the NAS library; session 45 found its plays
  equal to session 39's in all 64 reports. Nothing is written to the NAS
  store and no saved index is written.
- **(02:58) Step 2 checks.** `scripts/check.sh` green (3:50; the first
  run stopped at `cargo fmt --check` and was run again after `cargo
  fmt`); `regress session-32-linux` (0:50) and `regress
  session-33-linux-nas` (3:45) identical with the run build
  `work/bin/s48-run/` (its `gunfinger` byte-identical to the start
  build's).
- **(03:03) Test seeds 2032 and 2034**, by the registered rule: the
  sweep panels drawn into `work/s48/panels/` (`mixes --count 0` with a
  scratch work directory) hold the Clockwork original out for 2030 and
  2031, the remix for 2033; 2032 and 2034 keep both. Development
  features were computed first (`work/s48/features-dev-*.json`,
  summaries `work/s48/summary-dev-*.txt`); no feature, rule or
  threshold changed after them. One reporting fix before the test
  seeds: the development-optimal threshold of F2 now counts a play
  without a near alignment as printed, as R2 does (the first summary
  skipped those plays).
- **(03:05) F4 and F5 thresholds, from the development seeds** (the
  registered way: the most playing plays printed with no shared play let
  in). F4 reads the wrong way round (AUC 0.14 for "higher is playing" at
  262 tracks: shared material scores higher), and its best bar prints 2
  of 41: R4 is F4 < 0.172 (262 tracks only). F5's bar is 0.003442 at
  262 tracks and 0.003386 at NAS scale; R5 takes the larger, F5 >
  0.00344, so that it lets in no development shared play at either
  scale. Both are now fixed in `marked_features.py` (`RULES`).
- **(03:12) Aligned drops, development seeds** (`work/s48/aligned.sh
  2026 2028`, renders 2:02 and 1:28, the CLI 1:03 per seed at 262
  tracks): 63 of 64 unrelated drops lined up (one block had no window);
  none merged, and in none would the tracks have started within 5 s
  (the least difference 10.7 s; medians 116.9 and 120.6 s, against
  103.4 and 96.8 s tempo-only). Observed: the first record's drop lies 44-378 s into its
  file (it must be at least the solo time and the fade in), while the
  second's strongest rise often lies near its start (8-25 s in 13 of 63
  blocks: the music coming in after a quiet intro), so the two records
  are rarely at one place in their tracks. A model in which DJs line up
  drops of the same rank (both first drops) is not measured; noted for
  the handoff.
- **(03:36) Steps 3 and 4 done.** Test queue (`work/s48/test.sh 2032
  2034`, tmux `s48test`) 03:02-03:19; aligned drops for the test seeds
  03:19-03:23; the aligned renders at NAS scale (`aligned-nas.sh`, tmux
  `s48alnas`) 03:22-03:32. The test queue's `tracklist_score.py score`
  lines failed (they need `show`'s printed tracklists, which the queue
  did not write); the registered scores do not use them, so they were
  not run again. Experiments 0215 (development), 0216 (test seeds) and
  0217 (aligned merges) written in that order.
- **(03:39) Step 5, experiment 0218.** The five marked plays of the
  development mixes' NAS-scale reports (session 36's `id-*-s36-step1-32`
  reports; the `s35-final` ones mark the same plays): R1 leaves all
  five out; R2 prints two and R3 one. The pair searches ran at 02:59
  (before 0216 was written) but were read only now.
- **(03:52) The review's findings** (a separate agent, read-only,
  03:37-03:44). No defect in `line_up_drops`, the feature definitions,
  the pre-registration's order or the seed choice. Fixed: R4's and R5's
  bars (03:05) were rounded the wrong way (0.172 and 0.00344 against the
  exact development extremes 0.171746 and 0.0034421), so each lets in 1
  of 64 development shared plays at 262 tracks; the bars stay as they
  were fixed before the test seeds, and with the exact bars the test
  results are the same; 0216 now gives R5's registered reading
  ("promising, more data first"); numbers corrected (the test seeds'
  pair counts 27 and 99; 0217's drop positions and medians; 0215's NAS
  ranges); R1-R3 at their development-optimal bars added to 0215 and
  0216; `marked_features.py`'s AUC leaves out unmeasured values (only
  F2's missing value counts as printed), the classes skip held-out truth
  plays as `tracklist_marked.py` does (no label changes: the development
  features computed again are identical), and a docstring; this brief's
  first line. F1c (the clusters file's pairs) is false for every play
  at 262 tracks because that file lists duplicates only; it is used in
  no verdict.
