# Session 35 brief: unidentified time in tracklists, continued

Written by the agent for itself on 2026-10-21, so the plan survives
context compaction. Work autonomously and never stop to ask: where a
decision is needed, take the conservative option, write it down here and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-35-checklist.md` (progress).
3. The pre-registration of this session,
   `docs/research/2026-10-21-gap-tests.md` (never change it after its
   commit; deviations go here), and session 34's,
   `docs/research/2026-10-21-gap-measures.md`.
4. `docs/session-notes.md` ("Session 34"), `docs/status.md` (newest
   entries), experiments 0159-0162 and this session's (0163 and up), the
   plan `docs/research/2026-10-20-unidentified-gaps.md`.

`docs/brief-2.md` to `docs/brief-34.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 35 of Gunfinger: unidentified time in tracklists, continued. Session 34 (experiments 0159-0162) found that 30 s is the smallest gap length meeting the pre-registered criteria, that a gap's length gives its missed tracks' count as a range but not as a count (method 1), and that a gap matched against itself (method 2, `gunfinger-eval self-match`, stretches at 10 hits) counts the records of DJ-length plays better: 95/80/69/54% exact at k = 1-4 on long generated mixes, fixed before that test. This session tests method 2 where it is expected to fail and combines the two methods, all in the harness, then writes down the options for printing gaps with the evidence, for our decision.

You run unattended: the user is asleep and cannot answer. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM. Use `--jobs 16` for searches (faster than 32 here, experiment 0158). Long mixes render at 13-14 GiB per seed; run one heavy job at a time.

Environment (sessions 33 and 34's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts, as `work/s34/*.sh` do.
- NAS store `~/.local/share/gunfinger/nas-dnb-peaks` (40,683 records), reached with `--other-peaks-dir`; NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`; saved index `work/indexes/library-nas-dnb-peaks.index`. Baselines `session-32-linux` and `session-33-linux-nas`. Build `work/bin/s34-final/`.
- Session 34's reports, listings and scores: see its handoff. Slot leave-out scans must not stay in a reports directory during `regress` (they change `calibrate`'s counts); move new ones out as session 34 did.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-34.md, docs/session-notes.md ("Session 33" and "Session 34"), docs/status.md (newest entries), `docs/research/2026-10-20-unidentified-gaps.md`, `docs/research/2026-10-21-gap-measures.md`, experiments 0154 and 0159-0162, the roadmap's "Time without a confident match" and "The filter and famous breaks", ADR 0006, 0017 and 0019, the panels in `docs/panels/` and the experiments that made them (shared material: 0110-0118, 0122, 0129), and the code of `self_match.rs`, `mixes.rs` (`--kind long`), `robust.rs`'s beatmatched blends and `tempo.rs`.
2. Write docs/brief-35.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-35-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt in work/s35/listening-digests-start.txt; it must be the same at the end. Experiments number from 0163.
3. Build `work/bin/s35-start`; `regress session-32-linux` and `regress session-33-linux-nas` identical before any harness change is used.

Steps:

1. Pre-registration, committed before any count: for each test below, what is measured, on which data, and the criteria, including the rule of the combined estimate (step 4), fixed before the seeds it is tested on are rendered. Seeds used by session 34 (2026-2029, 7) are development data; the tests use new seeds.

2. Method 2's main risk, records that share material: long mixes whose held-out runs hold records known to share passages, beatmatched. Draw the held-out runs' records from pairs or groups that share material (the shared-material panels' played and related files, the library map's or clusters' shared passages, `holders`), and beatmatch each blend to the outgoing record's tempo with `tempo.rs` as `robust`'s beatmatched blends do, so that shared breaks line up as a DJ would play them. Score the self-match count and method 1 against the truth by k, beside a control of the same generator without shared material (new seeds). If shared breaks merge records, say how often and by how much.

3. A gap inside a found track: session 34 found 12 gaps at NAS scale with no missed track, inside a found track, where the found track's own repeats make stretches. Test telling them apart by matching the gap's blocks against the confident neighbours' records (the indexed files of the plays before and after): blocks whose material is the neighbour's do not count. Measure on session 34's NAS and 262-track gaps (development) and on the long mixes of step 2 (test): counts of 0 where nothing is missed, and missed records not discounted by mistake.

4. A combined estimate: the self-match count limited to method 1's range (range A or B, as registered), with step 3's discount if it passes. Fixed before the test seeds, then scored on them and on the development gaps, against each method alone: exact, within one, off by two or more, and gaps holding no missed track.

5. Pitch rides: session 34 matched at speed 1.0 only. Render long mixes whose plays change speed within the play (a pitch ride of 1-3%, as `robust` does) and measure whether the self-match count drops; if it does, try the neighbouring rungs and measure the cost.

An experiment file (at most 40 lines) for each of steps 2-5, written before the next experiment starts.

6. A proposal for printing gaps: `docs/research/2026-10-21-printing-gaps.md` (not an ADR: the decision is ours), with the options and the evidence for each choice session 34's handoff lists (default or on request; 30 or 60 s; the mix's start and end; a gap with a possible line inside it, ADR 0006 and 0019; a count or not, as a range, from which method, and what prints without an estimate; the JSON, CSV and cue fields; wording that does not claim the matched time is fully explained), with a recommendation marked as such and examples of the output from this session's real reports.

7. Ahead, only when 1-6 are done: a prototype of the printed gaps on a branch `s35-gaps-output` (never merged; master's product does not change), rendering the proposal's recommended option in `identify`'s tracklist and JSON, with tests, so that we can run it on a development mix. `scripts/check.sh` green on the branch. Say in the notes how to try it.

8. A read-only review of the session's commits and documents by a subagent; fix what it finds.

9. Wrap-up: a "Session 35" section at the end of docs/session-notes.md with a handoff; update docs/status.md, the checklist, the roadmap's "Time without a confident match" and the plan. Final checks on master: `scripts/check.sh` green, `regress session-32-linux` and `regress session-33-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, `git status` clean. Commit. Your final message: what was built and measured, the results against the criteria in a few lines each, the proposal's recommendation, and anything left.

Rules for this session:
- No push. Commit on master (and `s35-gaps-output` only as step 7 says) with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild the NAS clusters.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change.
- Master's product does not change: no change to the CLI's output, the core, thresholds, parameters or rules on master. Harness changes are tested and keep `regress` identical.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts, evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user runs among them).
- Scratch data goes under work/s35/; delete renders you no longer need (session 34's `work/mixes/` renders, 5.8 GB, may be deleted once nothing needs them), keep the session's scratch under about 30 GB.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00. Start no new run or step after about 11:15; finish what is running, then do steps 8-9. If steps are left, list them in the handoff.
```

## State at the start of session 35

- 2026-10-21, 08:52. `master` at `611449d`, clean. Git identity `Jacek
  Galanciak <jacek.galanciak@gmail.com>`. Load average 0.8 (session 34's
  runs had just ended).
- `docs/pair-verdicts.txt`: 82 lines, SHA-256 `ad0fc785d26a...`
  (`work/s35/listening-digests-start.txt`). No answer log in this clone.
- Start build `work/bin/s35-start/` (1.98.0): both binaries
  byte-identical to `work/bin/s34-final/`, as no code changed since.
- Experiments 0001-0162; this session numbers from 0163. Test set: 4 of
  5 evaluations used (none in this session).
- Session 34's generated mixes (seeds 2026-2029 and 7, short and long)
  are development data now. Their renders are in `work/mixes/` (5.8 GB).

## Plan

The session has about two hours before the soft deadline, so the plan
keeps each test small: two new seeds per condition, 16 mixes each.

| Step | What | Expected |
|---|---|---|
| First | reading; brief, checklist, `CLAUDE.md`; digest; start build; both regressions | 08:52-09:15 |
| 2-5 code | `mixes --kind beatmatched`, `shared`, `rides`; `self-match --neighbours` and `--ratios`; tests | 09:15-09:50 |
| 1 | pre-registration `docs/research/2026-10-21-gap-tests.md` (after exploring only development data), committed before any test seed is rendered | by 10:00 |
| 2-5 runs | test seeds rendered and searched, one at a time; `gaps`, `self-match` | 10:00-10:35 |
| 2-5 | experiments 0163-0166 | 10:35-10:55 |
| 6 | the proposal `docs/research/2026-10-21-printing-gaps.md` | 10:55-11:10 |
| 7 | (ahead) the prototype branch, only if 1-6 are done | if time |
| 8 | read-only review by a subagent | 11:10 |
| 9 | notes, status, roadmap, plan, checklist; final checks | by 11:30 |

## Decisions

- **(09:10) New seeds without new committed panels.** `mixes` writes a
  sweep panel for a seed without one into the panels directory; the
  global `--panels work/s35/panels` (a copy of `docs/panels/`) keeps the
  new seeds' panels out of `docs/panels/`.
