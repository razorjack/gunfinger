# Session 39 brief: scoring the printed tracklist, with double drops

Written by the agent for itself on 2026-10-23, so the plan survives
context compaction. Work autonomously and never stop to ask: where a
decision is needed, take the conservative option, write it down here and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-39-checklist.md` (progress).
3. ADR 0006, 0017 and 0019; experiments 0095, 0101, 0102, 0107 and this
   session's (0182 and up); the roadmap's "The printed tracklist" and
   "Possible plays inside a confident play of another recording".
4. `docs/session-notes.md` ("Session 35" to the end), `docs/status.md`
   (newest entries), `crates/gunfinger-eval/src/mixes.rs`,
   `tempo.rs`, `robust.rs` (`partner_confident`),
   `crates/gunfinger/src/export.rs` (`entries`), `report.rs`,
   `output.rs`, `crates/gunfinger-core/src/plays.rs`.

`docs/brief-2.md` to `docs/brief-38.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 39 of Gunfinger: score the tracklist `identify` prints, with double drops. The roadmap's "The printed tracklist" (a proposal from an external review): the harness scores detections, plays and seconds, but nothing scores the entries `identify` prints after grouping files (ADR 0017), choosing each entry's name, flagging disagreeing titles and leaving out possible plays inside a confident play (ADR 0019). ADR 0019 names a cost no stored report could measure: a track found only as possible inside a passage of a stronger track, as in a double drop, leaves the tracklist. Double drops are common in drum & bass, and the generated mixes only move from one track to the next with fades, bass swaps and (since session 35) beatmatched blends. This session adds double drops to the generator, renders mixes, runs the real CLI on them and scores what it prints. It is a measurement session: the product's rules and output do not change.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600. The search is faster since session 36 (0169): use the default jobs (32). Long mixes render at about 14 GiB per seed of 16; one heavy job at a time.

Time box: started about 00:30; aim to finish by about 04:00, because further sessions follow you tonight. If a step would run past that, scale it down (fewer seeds, 262 tracks only), record the decision in the brief and continue.

Environment (sessions 33-38's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS store `~/.local/share/gunfinger/nas-dnb-peaks` (40,683 records), library `/mnt/atlas/Music/dnb` (read only), reached with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: since the listening of 2026-10-22 (docs/status.md, "after session 38") SHA-256 `d9c85c4586ad...`, 284 clusters, Dilusion's upload joined to its cluster; `regress session-33-linux-nas` is identical with it (5:56). Baselines `session-32-linux` and `session-33-linux-nas`. Build `work/bin/s38-final/` (master). `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- Session 38's opt-in `--apart-in-speed` and session 37's `--overlap-by-chains` are variants, not defaults; their adoption waits for an ADR (ours).
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans must not sit in a reports directory during `regress`.
- Renders from earlier sessions sit in `work/mixes/` (about 21 GB); leave them, and keep your own renders' total under about 40 GB.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/brief-38.md, docs/session-notes.md ("Session 35" to the end), docs/status.md (newest entries), the roadmap's "The printed tracklist" and "Possible plays inside a confident play of another recording", ADR 0006, 0017 and 0019, experiments 0095, 0101, 0102 and 0107, `robust`'s blend conditions (`partner_confident`), and the code of `mixes.rs` (every `--kind`, `tempo.rs`'s beatmatching), the CLI's `export.rs` (`entries`), `report.rs`, `output/` and `plays.rs`.
2. Write docs/brief-39.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-39-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt in work/s39/listening-digests-start.txt; it must be the same at the end. Experiments number from 0182.
3. Build `work/bin/s39-start`; `regress session-32-linux` and `regress session-33-linux-nas` identical.

Steps:

1. Pre-registration, committed before any count: the scenarios, the truth each mix carries (which recordings play when, which files of each recording are indexed, their tags), the scores (per mix: extra entries; played tracks missing; two played tracks merged into one entry; matching files missing from an entry; disagreeing titles not flagged; beside today's confident-only score, by recording family as well as by entry), and what would count as a problem worth a roadmap entry. Tags: use recordings whose files' artist and title tags agree across their cluster and with the file names (checked by a script, no listening), so that a mislabelled file does not hide a grouping error; say how many qualify.

2. The generator: a double drop (two records at about equal level for 30-90 s, beatmatched with `tempo.rs`, entering and leaving with the usual blends), tested, and a mix kind with these scenarios: an original alone; its remix alone; both together in a double drop (shared-material pairs from the panels in `docs/panels/`, as session 35's `--kind shared` draws them); two unrelated records together; a quiet (for example -9 dB) or brief (20-40 s) record inside a stronger one; a returning record; several rips of one recording played at different times. Exact truth in the report. Harness only; `regress` identical.

3. The runs: render the mixes (several seeds, enough for each scenario to occur at least about 30 times), search them the way the user does, with the CLI's `identify -f json` and its tracklist (`gunfinger show`), at 262 tracks (the corpus configuration, with the sweep's held-out clusters left out of the index by a separate peak store or store-only copy, so that held-out records are truly unknown; never write to `work/peaks`) and, if time allows, at NAS scale with a store-only copy or a scratch configuration (never index or write the NAS store). Score with a script against the truth.

4. Experiments (at most 40 lines each, written before the next starts): what the printed tracklist gets right and wrong in each scenario, with the double drop's partner the main question (ADR 0019's cost: how often a played record found only as possible inside its partner's passage leaves the tracklist, and whether the table or JSON still shows it), and where entries merge or split wrongly.

5. Ahead, only when 1-4 are done: the same scenarios scored at the detection level by the harness (`mixes`'s own scoring and `seconds`), to tell what the search loses from what grouping and printing lose.

6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.

7. Wrap-up: a "Session 39" section at the end of docs/session-notes.md with a handoff and the questions it leaves for us (a change to what the tracklist prints is ours to decide, with an ADR); update docs/status.md, the checklist and the roadmap's "The printed tracklist". Final checks: `scripts/check.sh` green, `regress session-32-linux` and `regress session-33-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, `git status` clean. Commit. Your final message: what was built, the scores per scenario in a few lines, the problems found, and anything left.

Rules for this session:
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- The product's behaviour does not change: no threshold, parameter, rule, default or output change; variants are opt-in, tested, and keep `regress` identical when off. Harness additions are tested and keep `regress` identical.
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user runs among them).
- Scratch data goes under work/s39/ (your session number) and renders under `work/mixes/`; delete your renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00. Start no new run after about 10:30; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 39

- 2026-10-23, 00:21. `master` at `6665077`, clean. Git identity `Jacek
  Galanciak <jacek.galanciak@gmail.com>`. No tmux server running.
- `docs/pair-verdicts.txt`: SHA-256 `f4b1867956ba...`
  (`work/s39/listening-digests-start.txt`), the file after the listening
  of 2026-10-22 (six lines more than session 38's `ad0fc785d26a...`).
- Experiments 0001-0181; this session numbers from 0182. Test set: 4 of
  5 evaluations used (none in this session).
- `work/mixes/`: 21 GB of earlier sessions' renders (left alone). Disk:
  803 GB free.
- The corpus store `work/peaks` holds 265 records with tags (`.tags`
  beside each record), so `identify` reports at 262 tracks carry tags
  (session 16's reports did not).
- `mixes` builds its index from the corpus store only; it has no
  `--other-peaks-dir`. Of the first shared panel's pairs, only
  Clockwork's (the original and the Stakka remix) has both records in
  the corpus library; the others need files of the NAS.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digest; brief; start build and both regressions (tmux `s39start`) | 00:21-00:45 |
| 1 | tag check (`tracklist_tags.py`); the pre-registration in this brief; commit | 00:45-01:15 |
| 2 | `mixes --kind drops` (double drops, overlays, scenarios), tests, `regress` identical | 01:15-02:15 |
| 3 | render and search (harness), store-only copy without the held-out records, CLI `identify -f json` and `show`; the scoring script | 02:15-03:00 |
| 4 | experiments | 03:00-03:30 |
| 5 | detection level (`mixes`'s scores, `seconds`), if 1-4 are done | 03:30-03:45 |
| 6-7 | review, notes, final checks | 03:45-04:30 |

NAS scale only if the 262-track work is done by about 03:00.

## Decisions

- **(00:26) Start build and regressions** (`work/s39/start.sh`, tmux
  `s39start`), while the brief and the tag check are written.
