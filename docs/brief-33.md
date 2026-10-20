# Session 33 brief: the NAS-scale harness on the Linux machine

Written by the agent for itself on 2026-10-21, so the plan survives
context compaction. Work autonomously and never stop to ask: where a
decision is needed, take the conservative option, write it down here and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-33-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 28" to the end), `docs/status.md`
   (sessions 18, 23, 25, 28, 30 and 32), `docs/brief-nas.md`,
   `docs/nas-plan.md`, ADR 0014 and ADR 0020.
4. Experiments 0066, 0067, 0100, 0120, 0128, 0139, 0143-0145,
   0152-0154, and this session's (0155 and up).

`docs/brief-2.md` to `docs/brief-32.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them. This prompt changes one: session 32's "rebuilds no NAS
cluster" no longer holds; this session builds the Linux machine's own.

## The prompt, verbatim

```text
Start session 33 of Gunfinger: build the NAS-scale harness on the Linux machine (the library map, the NAS duplicate clusters, a NAS baseline), then run the compute-heavy NAS-scale measurements again here and compare them with the Mac's: the hardest recall conditions (session 28, experiments 0144 and 0145), the development scans with leave-outs, and per-second scores. Session 32 brought Gunfinger up on Linux and built this machine's own NAS peak store, but the library map and the NAS clusters exist only on the Mac, and nothing is transferred from the Mac (our decision), so no NAS-scale harness run works here yet. Later sessions tonight (unidentified time in tracklists, `docs/research/2026-10-20-unidentified-gaps.md`) need NAS-scale leave-outs, which need these clusters.

You run unattended overnight: the user is asleep and cannot answer. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes to plan the next session; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM. Naming this hardware in repository text is allowed from this session on (our decision, 2026-10-21; session 32's prompt had forbidden it). Give wall times and peak memory of the heavy runs: they are the first NAS-scale harness timings on this machine, and the Mac's (10 cores) are in status.md for comparison. The default jobs is 32.

Environment (from session 32; check each):
- Rust: rustup in `~/.cargo/bin`, not on the shell's PATH. Use `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds, as session 32 did (the Mac's version).
- FFmpeg 8.1.2 (RPM Fusion), Python 3, Node, tmux (no `screen`), GNU time at `/usr/bin/time`.
- The corpus configuration `~/.config/gunfinger/config.toml` (library `corpus/library`, store `work/peaks`, 265 records: 3 more than the Mac's 262, see experiment 0152). The NAS configuration `~/.config/gunfinger/nas-dnb.toml`: library `/mnt/atlas/Music/dnb` (cifs, SMB 3.1.1, systemd automount, mounted read-write by the user's decision; Gunfinger and this session never write to it), store `~/.local/share/gunfinger/nas-dnb-peaks` (40,683 records, 1,926 skip notes, revision `56433d74d0919a08`; same records as the Mac's, paths in NFC). The harness reaches the NAS store with `--other-peaks-dir ~/.local/share/gunfinger/nas-dnb-peaks`; its NAS reports go to `work/reports/library-nas-dnb-peaks/`.
- Baselines: `work/baselines/session-32-linux` (262/265-track corpus, this machine). Builds `work/bin/s32-final`.
- Known Unicode difference: the Mac's SMB client stored 160 NAS paths in NFD, Linux lists them in NFC. One source path of `docs/panels/recall-seed-2026.json` is NFD (session 32's notes, "Session 32"), so the recall panel names a file this store holds under NFC bytes.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-32.md, docs/session-notes.md ("Session 28" to the end), docs/status.md (sessions 18, 23, 25, 28, 30 and 32 entries for the commands and the Mac's timings), docs/brief-nas.md, docs/nas-plan.md, experiments 0066, 0067, 0100, 0120, 0128, 0139, 0143, 0144, 0145, 0152, 0153, 0154, and ADR 0014 and ADR 0020.
2. Write docs/brief-33.md (this prompt verbatim, the state at the start, the plan and every decision as you take it) and docs/session-33-checklist.md, point CLAUDE.md's reading list at them (brief 32 joins the older briefs; item 6 should no longer say that no NAS cluster is rebuilt), and commit. Record the SHA-256 of docs/pair-verdicts.txt in work/s33/listening-digests-start.txt; it must be the same at the end. Experiments number from 0155.
3. Build `work/bin/s33-start` from master; `regress session-32-linux` must be identical (record its time).

Steps (2-6 are the plan; 7 and 8 are planned ahead, because experiments on this machine finish sooner than estimated; start them when 2-6 are done):

2. The library map: `map-library` with `--other-peaks-dir`. Compare with the Mac's (247 of 262 corpus files have a copy on the NAS); the 3 corpus records the Mac does not have may add copies.

3. The NAS clusters: session 18's command (`clusters --from-peaks --manifest razorjack-2003-03-29 --manifest stakka-skynet-knowledge --manifest nocny-trans-vertical-2002-08-23 --recall-panel 2026`, see status.md session 18, step 2a) with `--cut-sparsest`, which sessions 23 and 25 decided for chains between files judged different by ear. Run detached under tmux with `/usr/bin/time -v`, its own log and an exit line. The recall panel's NFD path: do not edit the committed panel. If the harness refuses it, the conservative option is a copy of `docs/panels/` under work/s33/ in which only that path is converted to NFC, passed with `--panels`; record which path and why. Compare with experiment 0128's clusters (data/0128-nas-clusters-at-40126.json, file names only) cluster by cluster: added, gone, split or joined, cut joins, the gap (weakest same, strongest unjudged left apart) and borderline rows. Remember the store has 557 more records than 0128's, and peaks differ slightly between the machines (0153), so a difference has two possible causes; say which applies where you can tell. New borderline pairs and new chains are listening questions for the notes, never verdicts. If the run stops for a reason `--cut-sparsest` does not cover, take the conservative option, record it and continue with what can run.

4. A NAS baseline for this machine: `baseline session-33-linux-nas` with `--other-peaks-dir` under the new clusters, and check that a `regress` of it is identical. Score the stored NAS development reports with the scripts session 32 and earlier sessions used (`devscore.py --nas` or its successor) and compare with the README's 11/11, 12/12 and 17/17.

5. The hardest recall conditions again at NAS scale: session 28's `recall --other-rips-only --queries clean,damaged,brief,brief-damaged` and its combined runs (brief-28, status.md session 28, experiments 0143-0145 give the commands and pre-registered counts). Compare with 0144 and 0145 condition by condition (confident, possible or better, wrong confident, held-out queries confident, reasons of misses), and query by query where the data files allow. Recall the store holds 557 more records than session 28's (40,126), so the recall panel's sources are searched against a larger library. These are the heaviest runs: start them detached as soon as the clusters exist and write the other steps' documents while they run. One run at a time, so their times mean something.

6. The development scans at NAS scale with session 10/18's leave-outs (experiments 0067 and 0100), then `seconds` on those reports (0139: 90.7% of indexed seconds correct at NAS scale at a 30 s margin). Compare with the Mac's numbers.

7. (Ahead.) The shared-material panels at NAS scale under the new clusters (session 25's `shared` run, experiments 0122 and 0129; `shared_outcomes.py`), compared with the Mac's outcomes.

8. (Ahead.) Session 30's quantisation-loss runs at NAS scale (`recall --verify`, experiments 0149 and 0151), compared with the Mac's.

If all eight finish early, stop there: the next session will work on unidentified time, and its runs need this session's clusters and baseline.

An experiment file (at most 40 lines) for each of steps 3, 5, 6, 7 and 8 (2 and 4 may go in 3's), written before the next experiment starts.

9. A read-only review of the session's commits and documents by a subagent; fix what it finds.

10. Wrap-up:
- A "Session 33" section at the end of docs/session-notes.md, with a handoff for the next session: the clusters file's path and revision, the baseline names, the exact commands that run NAS-scale scans with leave-outs, `seconds` and `recall` on this machine, the measured times at 32 jobs, and anything that did not reproduce. Update docs/status.md, the checklist, the roadmap where a fact changed, and ADR 0020's findings only if this session found something new about the platforms.
- Final checks: `scripts/check.sh` green, `regress session-32-linux` identical with the final build, docs/pair-verdicts.txt unchanged, `git status` clean. Commit.
- Your final message: what ran, the results against the Mac's in a few lines each, the times, the handoff paths and commands, and anything left or blocked. The orchestrating thread plans the next session from it.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- Long runs: the Bash tool kills a background task at its timeout. Every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line. After each run, check that its log ends with its exit line and holds no error.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The development mixes are the only mixes this session searches. Beware of globs over `corpus/sets/*`: session 32's reviewer opened the held-out manifest that way.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change.
- This session tunes nothing: no threshold, parameter or rule changes because of a result. Code changes only where a harness run cannot run on Linux, tested, with `check.sh` green; a change that could alter detections on the Mac goes on a branch `s33-detection`, never merged (session 32's rule).
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts, evaluation JSON in docs holds file names only, no filename- or tag-derived logic in the product.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user runs among them).
- Scratch data goes under work/s33/; delete renders you no longer need, and keep the session's scratch under about 20 GB (the disk has 800 GB free, but stay tidy).
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
```

## State at the start of session 33

- 2026-10-21, 01:15. `master` at `87529a3`, clean. Git identity `Jacek
  Galanciak <jacek.galanciak@gmail.com>`.
- The Linux machine: Fedora Linux 44, AMD Ryzen 9 5950X (16 cores, 32
  threads), 64 GB RAM; load average under 0.2 before the session's
  first build. Rust 1.98.0 through rustup (`~/.cargo/bin` put on `PATH`
  per command).
- FFmpeg is 8.1.3 (RPM Fusion, installed 2026-10-18 at 23:00, after
  session 32's runs), not 8.1.2. The NAS store's records were made with
  8.1.2; renders made tonight (recall excerpts, damaged queries, shared
  queries) use 8.1.3.
- The NAS: `/mnt/atlas` mounted (cifs, SMB 3.1.1, `rw`, systemd
  automount), `Music/dnb` readable. The NAS store: 40,683 records,
  revision `56433d74d0919a08` (to be confirmed by `stats` or the harness).
- `work/` holds only session 32's data: `baselines/session-30-start` and
  `session-32-linux`, `bin/s32-start` and `s32-final`, the corpus reports
  and `work/s32/`. No `work/reports/library-nas-dnb-peaks/`: no library
  map, no NAS clusters, no saved harness index.
- Experiments 0001-0154; this session numbers from 0155. Test set: 4 of
  5 evaluations used (none in this session).
- `docs/pair-verdicts.txt`: 82 lines, SHA-256 `ad0fc785d26a...`
  (`work/s33/listening-digests-start.txt`). No answer log in this clone.
- Start build `work/bin/s33-start/` (1.98.0, 20 s): both binaries differ
  from `s32-final`'s, as expected, since `3401fa9` changed the install
  hints in `decode.rs` and `memory.rs`. `regress session-32-linux`:
  identical (720 of 720 sweep queries, all 89, 97 and 83 scan
  detections, the calibrate lines) in 68.7 s wall, 1,709 s user, 0.39
  GiB at peak. So FFmpeg 8.1.3 decodes and encodes the sweep's queries
  as 8.1.2 did, at least as far as detections show.

## The Mac's figures to compare with

From status.md and the experiments; the Mac (M1 Pro, 10 cores) often
shared its CPU with other sessions, so its times are a rough guide.

| Run | The Mac |
|---|---|
| `clusters --from-peaks`, session 18's queries | 1,554 s, 12,548 s CPU, 4.2 GB (session 18, 33,723 records); 32 min, 5.7 GB (0128, 40,126); 2 h 54 min (0120, under load) |
| Development scan at NAS scale | 307-315 s, 4.8 GB (0067, 32,441); 66 s search (0100); 112 s with the saved index (ADR 0014) |
| Leave-outs | 64-74 s each (0100) |
| `recall`, one condition | 6-12 min, 5.1 GB (0144, under Panako's load) |
| `regress` at 262/265 tracks | 153-156 s |

## Plan

| Step | What | Expected here |
|---|---|---|
| First | reading; brief, checklist, `CLAUDE.md`; digests; start build; `regress session-32-linux` | done |
| 2 | `map-library`; compare with 247 of 262 | minutes |
| 3 | `clusters --from-peaks` with the three manifests, `--recall-panel 2026`, `--cut-sparsest` (tmux, `time -v`); compare with 0128 | 10-30 min |
| 4 | `baseline session-33-linux-nas` with `--other-peaks-dir`; `regress` of it; `devscore.py --nas` | 10-20 min |
| 5 | `recall` conditions as session 28 ran them, one at a time (tmux) | 1-2 h |
| 6 | development scans of the three mixes at NAS scale, leave-outs as 0067 and 0100; `seconds` | under 1 h |
| 7 | (ahead) the shared panels' development halves at NAS scale | about 1 h |
| 8 | (ahead) `recall --verify` | about 1 h |
| 9 | read-only review by a subagent | |
| 10 | notes, status, checklist, final checks | |

Experiments: step 3 (with 2 and 4) 0155, step 5 0156, step 6 0157,
step 7 0158, step 8 0159, each written before the next experiment
starts.

Commands: `E="work/bin/s33-start/gunfinger-eval --other-peaks-dir
$HOME/.local/share/gunfinger/nas-dnb-peaks"`; reports in
`work/reports/library-nas-dnb-peaks/`.

## Decisions

(by time, as they are made)

- **(01:25) The recall panel's decomposed path.** The harness does not
  refuse the panel: `recall::Panel::for_seed` treats a source without a
  record as pruned, prints "left out of the recall panel: ... has no
  record any more" and searches the others. Development source 26
  (`second-library/__youtube_archivists/3zb-gangstep-dnb/No Bluff Sound
  - Coup De Tête (Vocals - Izm & Peanut Butterfly).opus`, indexed, not
  held out) is stored in NFD in the panel and in NFC in this machine's
  store, so it would be left out silently, and the clusters run and every
  recall condition would search 57 indexed sources where the Mac searched
  58. That would change what is compared. The conservative option of the
  prompt applies: `work/s33/panels/` is a copy of `docs/panels/` in
  which only that string is converted to NFC (no other panel file holds
  a non-NFC string), passed with `--panels` to every run that reads the
  recall panel (`clusters --recall-panel`, `recall`). Other runs keep
  the default `docs/panels/`; the sweep panels in the copy are
  byte-identical anyway. The committed panel does not change.
