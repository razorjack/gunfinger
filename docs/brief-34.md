# Session 34 brief: unidentified time in tracklists, measured

Written by the agent for itself on 2026-10-21, so the plan survives
context compaction. Work autonomously and never stop to ask: where a
decision is needed, take the conservative option, write it down here and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-34-checklist.md` (progress).
3. The pre-registration, `docs/research/2026-10-21-gap-measures.md`
   (never change it after its commit; deviations go here).
4. The plan `docs/research/2026-10-20-unidentified-gaps.md`, experiment
   0154 and `scripts/analysis/gap_count.py`, the roadmap's "Time without
   a confident match".
5. `docs/session-notes.md` ("Session 32" and "Session 33"),
   `docs/status.md` (newest entries), ADRs 0006, 0017 and 0019,
   experiments 0020, 0067, 0100, 0107 and 0139, and this session's (0159
   and up).

`docs/brief-2.md` to `docs/brief-33.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 34 of Gunfinger: unidentified time in tracklists, measured. `docs/research/2026-10-20-unidentified-gaps.md` is the plan (our decision, 2026-10-20: marking the time a tracklist does not account for is worth doing). This session builds step 1 (the gaps) and step 2's first method (how many tracks a gap holds, from its length) in the harness only, and measures both on real detection edges: the development mixes with runs of consecutive tracks left out of the index at NAS scale, and generated mixes with exact truth. Experiment 0154 measured method 1 on the manifests' start times only; this session measures it where it would run, on detections.

You run unattended overnight: the user is asleep and cannot answer. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes to plan the next session; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM; default jobs 32. Use it: long, parallel, CPU-heavy runs are welcome, one search at a time so memory stays bounded (a NAS-scale search peaks at about 5-8 GB).

Environment (session 33's handoff, docs/session-notes.md "Session 33"; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, GNU time at `/usr/bin/time`. zsh does not split a command held in a variable: run command queues from `sh` scripts, as `work/s33/*.sh` do.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, 265 records). NAS store `~/.local/share/gunfinger/nas-dnb-peaks` (40,683 records, revision `56433d74d0919a08`), library `/mnt/atlas/Music/dnb` (mounted read-write by the user's decision; never written). The harness reaches it with `--other-peaks-dir ~/.local/share/gunfinger/nas-dnb-peaks`.
- NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json` (284 clusters, SHA-256 `37bc419d3d25...`) and the map beside it; saved harness index `work/indexes/library-nas-dnb-peaks.index` (current). Baselines `session-32-linux` (corpus) and `session-33-linux-nas` (NAS scale). Build `work/bin/s33-final/`.
- Use `--jobs 16` for searches: about 30% faster than the default 32 here (experiment 0158). At 16 jobs a NAS-scale leave-out scan takes about 52 s for Stakka; my 2003 mix and Nocny about 42 s and 75 s (estimates from 0157 and 0158). A scan writes over the report of the same set, count and seed, so slot-named leave-outs need report names of their own.
- `recall` and `clusters` need `--panels work/s33/panels` on this machine (the recall panel's one NFD path in NFC); this session should not need either.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-33.md, docs/session-notes.md ("Session 32" and "Session 33"), docs/status.md (newest entries), the plan `docs/research/2026-10-20-unidentified-gaps.md`, experiment 0154 and `scripts/analysis/gap_count.py`, the roadmap's "Time without a confident match", ADR 0006, ADR 0017, ADR 0019, experiments 0020, 0067, 0100, 0107, 0139, and the code of `scan.rs` (`draw_left_out`), `mixes.rs`, `seconds.rs` and the CLI's `plays` and `report.rs`.
2. Write docs/brief-34.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-34-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt in work/s34/listening-digests-start.txt; it must be the same at the end. Experiments number from 0159.
3. Build `work/bin/s34-start`; `regress session-32-linux` and `regress session-33-linux-nas` (with `--other-peaks-dir`) must be identical before any harness change is used.

Steps:

1. Pre-registration, committed before any count: the definitions (a gap is a stretch of the mix with no confident segment, computed from segments rather than play spans, so a gap inside a returning track counts; the mix's start and end count as edges; a possible play inside a gap does not close it, ADR 0006), the minimum lengths to compare (for example 10, 20, 30, 45, 60 and 90 s), the truth (which seconds an indexed track plays, which a left-out or held-out track plays, which are transition only), the measures (step 1: seconds of missed tracks inside listed gaps, listed gaps holding no missed track, missed tracks no gap reaches, by minimum length; step 2: estimate minus truth by number of missed tracks, how often the range misses, gaps holding no missed track), and the criteria for choosing the minimum length and for calling method 1 good enough to print as a range. Write the criteria before you see any number.

2. The harness, tested (`scripts/check.sh` green), harness only; the CLI and the core do not change:
   - A gap listing and its scoring for saved `scan`, `mixes` and `shared-mixes` reports (a `gaps` subcommand beside `seconds`, or an option of it), with method 1's count (the gap over the median start-to-start interval of the mix's found tracks, rounded, at least 1, with a range) and the truth above. Unit tests on fixtures.
   - A way to name the tracks left out of a scan: runs of consecutive manifest slots (for example `scan <set> --leave-out-slots 3,7-8,12-14`), leaving out each named track's whole cluster as `--leave-out` does, with the report naming the slots. Several runs separated by at least one kept track give several independent gaps from one scan; design the leave-out patterns so that every run of k = 1-4 consecutive slots with a kept track on each side appears at least once per set, in as few scans as possible, and write the design down before the runs.

3. The runs (detached under tmux, `/usr/bin/time -v`, own logs and exit lines, one search at a time):
   - The three development sets at NAS scale under the patterns of step 2, and each set with nothing left out (gaps with no missed track: blends, MCs, treated passages).
   - The same at 262 tracks, where many referenced tracks are not indexed at all (natural gaps): razorjack 7/12 and nocny 9/18 indexed.
   - Generated mixes (`mixes`, several seeds, at 262 tracks): their held-out plays are exact truth for step 1's minimum length against crossfades, cuts and short plays.

4. Experiments (at most 40 lines each, written before the next starts): step 1 (the minimum length, against the pre-registered criteria) and step 2 method 1 on detection edges (against 0154's start-time figures: exact 97/75/48/42% at k = 1-4). Say where the found tracks' median interval is biased (short plays missed more often), and what a gap with no missed track looks like.

5. Ahead, in this order, only when 1-4 are done:
   a. Generated mixes of long plays with runs of held-out tracks: the existing generator plays 20-60 s per track, far shorter than a DJ's 2-6 minutes, so method 1 cannot be judged on it. A generator option (or a new seeded mix kind) that plays tracks for DJ-like lengths with crossfades of 10-60 s and runs of k = 1-4 held-out tracks between indexed ones, with exact truth, rendered and searched at 262 tracks over many seeds (this machine renders in parallel). Score with step 1 and step 2 as above.
   b. A first look at step 2's second method, the gap searched against itself (the plan's "The gap against itself"), only as a measured prototype in the harness on this session's gaps: count blocks of self-matching moments and compare with the truth. The next session will continue it if you do not get there.

6. A read-only review of the session's commits and documents by a subagent; fix what it finds.

7. Wrap-up:
- A "Session 34" section at the end of docs/session-notes.md with a handoff (paths of the reports and scores, the commands, what the next session should do for method 2, and what an ADR for printing gaps would have to decide; drafting that ADR is ours, not this session's). Update docs/status.md, the checklist, the roadmap's "Time without a confident match" and the plan with the measured figures.
- Final checks: `scripts/check.sh` green, `regress session-32-linux` and `regress session-33-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, `git status` clean. Commit.
- Your final message: what was built, the results against the criteria in a few lines each, the times, the handoff paths and commands, and anything left. The orchestrating thread plans the next session from it.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- Long runs: the Bash tool kills a background task at its timeout. Every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line. After each run, check that its log ends with its exit line and holds no error.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches. Beware of globs over `corpus/sets/*`: name the three development sets explicitly.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild the NAS clusters.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change.
- The product does not change: no change to the CLI's output, the core, thresholds, parameters or rules. Whether gaps print by default or on request is our decision and needs an ADR; this session measures. Harness changes are tested and keep `regress` identical.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts, evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user runs among them).
- Scratch data goes under work/s34/; delete renders you no longer need, and keep the session's scratch under about 30 GB.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00. Start no new run after about 10:30; if a step is unfinished then, record where it stands and wrap up, so the orchestrating thread can launch a follow-up.
```

## State at the start of session 34

- 2026-10-21, 06:17. `master` at `121643c`, clean. Git identity `Jacek
  Galanciak <jacek.galanciak@gmail.com>`.
- The Linux machine as in session 33; load average 0.2 before the
  session's first run. FFmpeg 8.1.3.
- `docs/pair-verdicts.txt`: 82 lines, SHA-256 `ad0fc785d26a...`
  (`work/s34/listening-digests-start.txt`). No answer log in this clone.
- Start build `work/bin/s34-start/` (1.98.0): byte-identical to
  `work/bin/s33-final/`, as no code changed since. `regress
  session-32-linux` with it: identical (720 of 720 sweep queries, all 89,
  97 and 83 scan detections, the calibrate lines) in 68.5 s.
- Experiments 0001-0158; this session numbers from 0159. Test set: 4 of
  5 evaluations used (none in this session).
- The development manifests: Stakka 11 slots, all referenced; my 2003
  mix 12 slots, all referenced (5 references in `second-library/`, not in
  the 262-track corpus); Nocny 18 slots, slot 17 without a reference.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; brief, checklist, `CLAUDE.md`; digest; start build; both regressions | 06:17-06:40 |
| 1 | pre-registration `docs/research/2026-10-21-gap-measures.md` and the leave-out design, committed before any count | 06:40-07:10 |
| 2 | `scan --leave-out-slots`, `gaps` (listing, truth, method 1), tests, `check.sh`; both regressions with the new build | 07:10-08:20 |
| 3 | runs: 3 × 14 slot leave-outs and 3 plain scans at NAS scale; the same at 262 tracks; `mixes` at several seeds; `gaps` on all | 08:20-09:30 |
| 4 | experiments 0159 (step 1) and 0160 (step 2, method 1) | 09:30-10:00 |
| 5a | (ahead) long-play generated mixes with runs of held-out tracks | if time |
| 5b | (ahead) the gap against itself, a prototype | if time |
| 6 | read-only review by a subagent | 10:15 |
| 7 | notes, status, roadmap, plan, checklist; final checks | by 11:00 |

## Decisions

- **(06:25) The leave-out design** (`scripts/analysis/leave_out_design.py`).
  A run of slots s..s+k-1 needs slots s-1 and s+k kept, so as intervals
  [s, s+k] two runs fit one scan only if they do not overlap. The fewest
  scans is then the largest number of runs that cover one slot (interval
  graphs are perfect), 14 for each set (2 + 3 + 4 + 5 for k = 1-4 in the
  middle of the mix), and assigning runs in order of their first slot to
  the first scan they fit reaches it. 42 scans per scale, each run once.
  The first and last slots stay kept. Nocny's slot 17 has no reference,
  so it is missing in every scan (a natural one-track gap between 16 and
  18); its runs end at slot 15, so slot 16 stays as an indexed neighbour.
  At 262 tracks the same patterns run, and the slots not indexed there
  join the runs they touch; the truth counts what each gap holds.
- **(06:32) Both regressions identical with the start build**: corpus
  68.5 s; NAS scale 11:23, 5.4 GiB at peak (720 of 720 sweep queries,
  all 4,253, 4,244 and 4,212 scan detections, the calibrate lines).
- **(06:33) Pre-registration committed** (`e7122b1`) before any gap was
  counted. Covered time is the segments of confident plays (ADR 0013:
  a possible segment of a confident play is part of it, as `seconds`
  labels it); possible plays cover nothing. Range A (n-1 to n+1) is the
  primary range, range B (the quartiles) is reported beside it.
- **(06:34) Step 2 built.** `scan --leave-out-slots` (slots validated
  against the manifest; a slot without a reference, Nocny's 17, is
  accepted and leaves nothing out) and `gaps`; `seconds` and `gaps`
  share one report loader, and `seconds` writes byte-identical files
  and output before and after (checked on two NAS reports). The first
  look at `gaps` was on a stored session 33 report, after the
  pre-registration's commit.
- **(06:35) Run build `work/bin/s34-runs/`** (step 2's code), frozen for
  step 3's queue (`work/s34/runs.sh`, tmux `s34-runs`): both regressions
  again, the three plain scans and the 42 designed leave-outs at NAS
  scale with `--jobs 16`, the same at 262 tracks, then `mixes --count 16`
  at seeds 2026-2029 and 7. Only seeds with a committed sweep panel run,
  because `mixes` writes a new panel into `docs/panels/` for any other.
  `check.sh` ran beside the corpus regression (its 2:37 is slower for
  that).
- **(06:46) Step 5a prepared early and queued after step 3.** Long mixes
  (`mixes --kind long`): an indexed play, then runs of 1, 2, 3 and 4
  held-out tracks in a drawn order, each followed by 1 or 2 indexed
  plays; plays of 120-360 s (shortened to 85% of the track at its
  speed), crossfades of 10-60 s (at most half the shorter play), half of
  them bass swaps; speeds as the short mixes'. Five seeds with sweep
  panels, 16 mixes each (80 mixes, 320 runs). Build
  `work/bin/s34-long/`. The prompt puts step 5 after steps 1-4; its runs
  only use the machine while steps 3's analysis and 4's experiments are
  written, so they take nothing from 1-4. Its results are read only
  after 0159 and 0160 are written.
- **(06:49) Both regressions identical with the run build** (corpus
  2:37 beside `check.sh`, NAS 11:28, 5.5 GiB); step 3's scans started.
- **(06:50) Step 5b's prototype written before step 3 ended**
  (`self-match`, `self_match.rs`): each listed gap of a saved report is
  decoded from the mix, its pair hashes matched with its own at speed
  1.0 (lags of 8 s or more, 2-frame lag steps, hashes occurring more
  than 40 times in the gap left out), cells of 5 s joined where at least
  5, 10, 20 or 40 hits of one cell recur at one lag, and the groups of
  joined cells counted as blocks. Thresholds are compared, not chosen.
  Generated-mix reports now record their kind (`kind`, omitted for
  short mixes), so the harness finds a mix's render; `work/bin/s34-long/`
  was rebuilt with it before its queue started. It runs only after step
  3's scans, so the scans' times are not shared with it.
- **(07:31) Step 3's NAS-scale scans done** (06:49-07:30): 45 scans,
  all exit 0, no wrong identification in any; Stakka 52.9-53.6 s, my
  2003 mix 42.6-46.1 s, Nocny 65.9-68.9 s at 16 jobs, 4.8-5.0 GiB.
- **(07:35) A deviation: the single-run interval.** Each designed scan
  leaves out up to seven runs, so nearly every interval between found
  entries holds a gap; the registered interval (which skips those)
  leaves fewer than 3 intervals in most scans, and 90 of the 115 gaps
  holding 1-4 missed tracks get no estimate. That is the design's
  density (a third to half of each mix left out), not a property of a
  mix with one run missing. Beside the registered figures, which are
  reported as they are, `gap_scores.py` adds the single-run interval:
  the median of the same mix's plain scan's entry intervals that the
  gap does not overlap, with the gap's own detection edges, as if its
  run were the only one missing (the leave-out changes no other track's
  detections in these scans: every kept track is found in every scan).
  Criteria M1-M3 are judged on both, and the experiment says which.
- **(07:45) Self-match on the NAS gaps (5b), explored, then fixed for
  a test.** `self-match` on the 45 NAS reports (55 s, 0.7 GiB, 8 jobs,
  beside the 262-track scans). Of the counts compared
  (`self_match_scores.py`: blocks, blocks spanning 30 s or more, and
  stretches, where long blocks whose spans overlap are merged), the
  stretches at 10 hits per cell and lag read best. Chosen after seeing
  these gaps, so they are fixed now (stretches, 10 hits, blocks of 30 s
  or more) and tested unchanged on the long generated mixes, whose gaps
  this session has not looked at.
