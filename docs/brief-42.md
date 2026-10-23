# Session 42 brief: stereo faults before the downmix, measured

Written by the agent for itself on 2026-10-23, so the plan survives
context compaction. Work autonomously and never stop to ask: where a
decision is needed, take the conservative option, write it down here and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-42-checklist.md` (progress).
3. `docs/session-notes.md` ("Session 40" and "Session 41"),
   `docs/status.md` (newest entries), the roadmap's "Stereo faults
   before the downmix" and "Recall beyond the corpus recordings",
   ADR 0020, experiments 0025, 0027, 0180 and this session's (0194 and
   up).
4. The code of `crates/gunfinger-core/src/decode.rs` (the FFmpeg
   arguments), `crates/gunfinger-eval/src/robust.rs` and `render.rs`.

`docs/brief-2.md` to `docs/brief-41.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 42 of Gunfinger: stereo faults before the downmix, measured. The roadmap's "Stereo faults before the downmix" (a proposal from an external review, 2026-10-17): FFmpeg mixes every file down to mono (`-ac 1` in `decode.rs`) before peaks are picked, and none of `robust`'s conditions is a stereo fault. Old tape and vinyl transfers, and the rips in the NAS collection, can carry each of them: one channel's polarity inverted (the downmix cancels what the channels share: centred bass, kick, often the vocal), one channel silent or noisy, a delay between the channels (the downmix is comb-filtered). This session renders such faults from development material, first as queries, measures what each costs today, and compares today's downmix with left-only, right-only and other channel choices as a diagnostic. It is a measurement session: the decoder and the product do not change; a fallback would need its own experiment and an ADR (ours).

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32). Rendering runs in parallel well here.

Time box: started about 06:15; aim to finish by about 10:00. If a step would run past that, scale it down (fewer levels, 262 tracks first), record the decision in the brief and continue.

Environment (sessions 33-41's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS store `~/.local/share/gunfinger/nas-dnb-peaks` (40,683 records), library `/mnt/atlas/Music/dnb` (read only), reached with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: since the listening of 2026-10-22 (docs/status.md, "after session 38") SHA-256 `d9c85c4586ad...`, 284 clusters, Dilusion's upload joined to its cluster. Baselines `session-32-linux` and `session-33-linux-nas`. Build `work/bin/s41-final/` (master; since session 41 the search is about 30% faster: a NAS-scale development scan searches in about 27 s, `regress session-33-linux-nas` takes about 4:12). `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- Session 38's opt-in `--apart-in-speed` and session 37's `--overlap-by-chains` are variants, not defaults; their adoption waits for an ADR (ours).
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Slot leave-out scans, and sweep reports of seeds other than 2026 (calibrate reads every sweep report there, session 40), must not sit in a reports directory during `regress`; `robust` reports of opt-in conditions should not replace the baseline's `robust` report either (check what `regress` compares).
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/` (about 21 GB plus session 39's); leave them, and keep your own renders' total under about 40 GB.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), the newest brief (docs/brief-41.md), docs/session-notes.md (the newest two sessions), docs/status.md (newest entries), the roadmap's "Stereo faults before the downmix" and "Recall beyond the corpus recordings", ADR 0020 (platforms; decoding goes through FFmpeg on both), experiments 0025, 0027 and the newest `robust` ones (0180), and the code of `decode.rs` (the FFmpeg arguments), `robust.rs` (its conditions, how excerpts are drawn and rendered, `--only`), `render.rs`, `recall.rs` and `sweep.rs` as far as they render queries.
2. Write docs/brief-42.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-42-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt in work/s42/listening-digests-start.txt; it must be the same at the end. Experiments number from 0194.
3. Build `work/bin/s42-start`; `regress session-32-linux` and `regress session-33-linux-nas` identical.

Steps:

1. Pre-registration, committed before any fault is searched: the faults and their levels (at least: one channel's polarity inverted; one channel silent; one channel with noise at two or three levels; a delay between the channels of about 0.2, 1 and 5 ms; and an unchanged control), how they are applied (to the stereo source before the downmix, by FFmpeg filters or in the harness, on `robust`'s excerpts of the indexed corpus files; files that are mono or nearly mono at the source counted apart, with how "nearly mono" is measured), the channel choices compared as a query-side diagnostic (today's downmix; left only; right only; and, if cheap, the louder channel or the side signal), and the scores (confident, possible or better, and hits against the clean control, per fault and choice, at 262 tracks and at NAS scale; wrong confident answers and related versions raised). Say what would count as a fault worth a roadmap entry and what would make a channel-choice fallback worth an experiment.
2. The harness, tested: the stereo faults as opt-in `robust` conditions (named in `--only`, so `robust`'s default run is unchanged), and a harness-only query channel choice for the diagnostic (for example `--query-channels downmix|left|right|...`). Harness only; the CLI's decoder does not change; `regress` identical.
3. The runs: every fault and channel choice on `robust`'s excerpts at 262 tracks, then at NAS scale (`--other-peaks-dir`), under tmux with logs if long.
4. Experiments (at most 40 lines each, written before the next starts): what each fault costs the downmix; whether one channel recovers it, and at what cost to clean queries and related versions; how many of the corpus's and the NAS store's sources are near mono (from the stored peaks if the store keeps nothing per channel, or by decoding a seeded sample of corpus files only; never decode the whole NAS).
5. Ahead, only when 1-4 are done: (a) the fault over a stretch of a generated mix (`mixes`, one kind, a few seeds at 262 tracks), scored by `mixes` and `seconds`; (b) the reverse case, a fault in the indexed reference rather than the query, on a store-only scratch copy of a few corpus files (never write `work/peaks` or the NAS store).
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 42" section at the end of docs/session-notes.md with a handoff and the questions it leaves for us (whether a decoder fallback is worth an experiment and an ADR); update docs/status.md, the checklist and the roadmap's bullet. Final checks: `scripts/check.sh` green, `regress session-32-linux` and `regress session-33-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: what was built, each fault's cost and the channel choices in a few lines, and anything left.

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
- Scratch data goes under work/s42/ (your session number) and renders under `work/mixes/`; delete your renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00. Start no new run after about 10:30; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 42

- 2026-10-23, 06:11. `master` at `e032cac`, clean. Git identity `Jacek
  Galanciak <jacek.galanciak@gmail.com>`. No tmux server running; load
  average 0.23 (a load of 12-19 over the past 15 minutes, from session
  41's final checks).
- `docs/pair-verdicts.txt`: SHA-256 `f4b1867956ba...`
  (`work/s42/listening-digests-start.txt`). The NAS clusters file:
  SHA-256 `d9c85c4586ad...` (`work/s42/clusters-digest-start.txt`).
- Experiments 0001-0193; this session numbers from 0194. Test set: 4 of
  5 evaluations used (none in this session).
- `work/mixes/` holds 23 GB; 800 GB free on the disk.
- How queries reach the search today. The core decodes every file with
  `ffmpeg ... -map 0:a:0 -vn -sn -dn -ac 1 -ar 8000 -f f32le pipe:1`
  (`decode.rs`, `Decoder::ffmpeg_command`): FFmpeg's own downmix, before
  peaks. `robust` renders its queries in mono: `render_samples` asks
  FFmpeg for `-ac 1` after the playback filter, and `encode` writes a
  mono WAV and encodes it (MP3 at 128 kbit/s for most conditions). The
  sweep's `render_excerpt` keeps two channels (`-ac 2`). So no `robust`
  query has two channels today, and a stereo fault needs a stereo render
  path.
- The peak store keeps one peak list per file, picked from the downmix,
  and no per-channel measure (`store.rs` has no channel field). How wide
  or mono a source is cannot be read from the stored peaks.
- `regress` compares the sweep, the development scans and their
  leave-outs, and calibrate's lines (`regress.rs`); calibrate does not
  read `robust` reports. `robust` writes `robust-seed-<seed>.json` into
  the reports directory and its renders into `<work>/robust/seed-<seed>/`.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief and checklist; start build and both regressions (tmux `s42start`) | 06:11-06:35 |
| 1 | pre-registration in this brief, committed | 06:35-06:50 |
| 2 | harness: stereo fault conditions, `--query-channels`, a width measure; tests; `regress` identical | 06:50-07:40 |
| 3 | runs at 262 tracks, then at NAS scale (tmux) | 07:40-08:40 |
| 4 | experiments 0194 and up | 08:00-09:00 |
| 5 | ahead (a) and (b), if time allows | 09:00-09:40 |
| 6-7 | review, notes, final checks | 09:40-10:15 |

## Decisions

- **(06:14) Start build and checks** (`work/s42/start.sh`, tmux
  `s42start`): the start build `work/bin/s42-start/`, compared with
  `s41-final`; `regress session-32-linux` and `regress
  session-33-linux-nas`; each report compared with its baseline's apart
  from `wall_seconds` (`work/s42/same.py`, session 41's).
