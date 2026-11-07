# Session 76 brief: the documents every session reads, made small and current

Written on 2026-11-07, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it), `AGENTS.md` and
   `docs/session-rules.md`.
2. This file, and `docs/session-76-checklist.md` (progress).
3. `docs/research/2026-11-05-external-review.md` (why this session runs)
   and `docs/decisions-queue.md`.
4. The session's start commit is `009fe55`: the old roadmap, notes and
   status are read from it (`git show 009fe55:docs/roadmap.md`).

## The prompt, verbatim

```text
Start session 76 of Gunfinger: make the documents that every session reads small and current, so that later sessions pay for the current state and reach the history only when they need it. The external review of 2026-11-05 (`docs/research/2026-11-05-external-review.md`) found that the reading cost and the repetition across documents take time from new evidence. The sizes today: `docs/roadmap.md` 119 KB (1,784 lines, 40 items; the largest, "A confidence statistic for shared material" and "Where search time goes", are 12-14 KB each and mostly session-by-session history), `docs/session-notes.md` 840 KB, `docs/status.md` 373 KB, the briefs 1.8 MB together (history since 2026-11-05), `AGENTS.md` 19 KB, `docs/calibration.md` 26 KB. `CLAUDE.md` still tells sessions to read the notes and status files at start. Session prompts point at named roadmap items ("the roadmap's 'The filter and famous breaks'") and at named notes sections ("notes 'Session 52'"), and every session adds a paragraph to the items it touched, so the items keep growing. This is a documents session: the product and the harness do not change, and nothing is measured. It is the first of tonight's sessions (sessions 77 onwards fix the harness and measure the active queue's items), so the format it sets is the one they will follow.

You run unattended: the user cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. Moving text is safe; deleting or rewording a fact is not: when in doubt, move the text verbatim into a history file and keep a pointer. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600.

Time box: started about 12:55 on 2026-11-07; aim to finish within about 3 hours, because many sessions follow you tonight. If a step would run past that, scale it down in this order: keep the roadmap rewrite and its history file, then the notes and status split, then the rest; record the decision in the brief and continue.

Environment (sessions 33-75's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,666 records, revision `0bd3f0c23fabc698` (session 75 pruned the records of DJ sets and a mixed CD's track fix). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-54-linux` (262 tracks) and `session-75-linux-nas` (NAS scale); a session that adds baselines names them in its handoff, and the next prompt says which to use.
- Build `work/bin/s75-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
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
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), docs/session-rules.md, docs/decisions-queue.md, the external review note, the whole of docs/roadmap.md, docs/calibration.md, the headings of docs/session-notes.md and docs/status.md and their newest five entries, and the newest brief (docs/brief-75.md) for the format of a brief.
2. Write docs/brief-76.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-76-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s76/listening-digests-start.txt. Experiments number from 0279 (this session is not expected to write one).

Steps:

1. Inventory, in the brief before any move: every document that CLAUDE.md, AGENTS.md, docs/session-rules.md or a session prompt tells a session to read, its size, which parts are current state and which are history; and every reference into the documents you will change, found with `grep` across the repository (path-and-line references such as `docs/session-notes.md:14678`, `docs/status.md:5951`, `docs/roadmap.md:375`; quoted headings such as "the roadmap's 'X'" and "notes 'Session NN'"; relative Markdown links). The moves below must keep every one of them resolvable.
2. The roadmap. Rewrite `docs/roadmap.md` as: a short preface that says how sessions update it (change an item's state and next-step lines; results go into experiments, notes and status, not here); "The active queue" (kept, updated only to say what moved); the open items, one per item, keeping each item's bold name exactly so that quoted references still find it, each with its state (open; measured and undecided; blocked on the user; blocked on a larger library), its one next step, its evidence (experiment, ADR and note numbers) and the few current facts and numbers a session needs to act on it, in a few lines; and a "Done and closed" section with one line per finished or closed item and its ADR or experiment. Move the previous text verbatim to `docs/roadmap-history.md`, under the same headings, with a line at its top naming the commit it was taken from; nothing is deleted. Target: under about 35 KB for the new roadmap. Add no new claims: state, priority and demotions follow the active queue and docs/decisions-queue.md, which already record them; where an item's state is unclear, say so in its line rather than guessing.
3. Session notes and status. Keep in `docs/session-notes.md` only the newest sessions (from about session 70 on) and, at its top, an index with one line per session (number, date, headline, where its section lives). Move the older sections verbatim, with their "Session NN" headings unchanged, into files under `docs/history/` (for example `docs/history/session-notes-01-39.md` and `docs/history/session-notes-40-69.md`). Do the same for `docs/status.md` (keep the entries from 2026-11-01 on; older ones move to `docs/history/status-...md` with an index line per month or per session range). Update every path-and-line reference that pointed into the moved parts, or, where that is impractical, make the index line for that section name its new file so that `grep` finds it; record the counts in the brief.
4. The other documents read every session. `CLAUDE.md`: items 3 and 4 become the newest session's notes and the newest status entries; the topic items (8-15) keep their pointers, updated to the history files where they now point there. `AGENTS.md`: update only its descriptive parts that are stale (the module map has no entry for session 72's `crates/gunfinger/src/under.rs`, the core's `residual` module and `decode::Decoder::render`; check every count and path it states); change no rule. If a rule in AGENTS.md looks outdated or contradicts another document, add it to docs/decisions-queue.md with the evidence instead. `docs/calibration.md`: keep the tables that describe today's values; if a section is history only (for example a change log), move it verbatim to `docs/history/` with a pointer. `docs/session-rules.md`: add the rule from step 2 (how the roadmap is updated) and check it against AGENTS.md and the prompts' rules for contradictions; list any in the brief and fix the ones that are only wording.
5. Verification, by a small script under `work/s76/` (not committed): every non-blank line of the old roadmap, notes and status (from the session's start commit) appears in the union of the new files, as a multiset; every relative Markdown link in `docs/`, `README.md`, `AGENTS.md` and `CLAUDE.md` resolves; every path-and-line reference found in step 1 points to a line that holds the text it pointed to before (or to its moved copy). Record the counts in the brief.
6. A read-only review by a subagent (it must not launch further agents): give it the old roadmap (from the start commit), the new roadmap and the history file, and ask it to list every open point, pending decision, number or caveat that a session would need and that is missing from the new roadmap's item lines (the history file holds it, but the item line should point at it or carry it). Fix what it finds.
7. Wrap-up: a "Session 76" section at the end of docs/session-notes.md with a handoff (what moved where, the new sizes, how sessions should read and update the documents now); a short docs/status.md entry; the checklist. Final checks: no code changed, so `regress` is not run (docs/session-rules.md); `scripts/check.sh` green; docs/pair-verdicts.txt unchanged; the clusters file's SHA-256 unchanged; `git status` clean. Commit. Your final message: the old and new sizes of each document read every session, what moved where, the verification counts, and anything left.

Rules for this session:
- `docs/session-rules.md` binds as well; where it and this list differ, this list wins for this session.
- No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
- Documents only: the product and the harness do not change; no code, test or configuration change apart from what a test that reads a moved document needs (then `scripts/check.sh` and both regressions must stay green and identical).
- Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
- The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
- The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
- Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
- AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
- Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
- Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
- Scratch data goes under work/s76/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 11:00 on 2026-11-08. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 76

- 2026-11-07, 12:46 by the Linux machine's clock (CET). The time box is
  about 3 hours: aim to finish by about 15:50.
- `master` at `009fe55`, clean (apart from the untracked
  `yafc20261031.log`, which is not this project's work). Git identity
  `Jacek Galanciak <jacek.galanciak@gmail.com>`.
- Digests (`work/s76/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `c4d3da2e7e37...`, `work/audition/answers.jsonl` `3e99ebca589b...`.
- Experiments 0001-0278; this session numbers from 0279 and is not
  expected to write one. ADRs 0001-0025. Test set: 4 of 5 evaluations
  used (none in this session).
- Sizes of the documents read every session (bytes): `docs/roadmap.md`
  118,978 (1,784 lines); `docs/session-notes.md` 840,449 (15,003
  lines); `docs/status.md` 373,487 (6,136 lines); `AGENTS.md` 18,789;
  `CLAUDE.md` 3,675; `docs/calibration.md` 25,941;
  `docs/session-rules.md` 5,565; `docs/decisions-queue.md` 8,337.
- No code changes in this session, so `regress` is not run
  (`docs/session-rules.md`): the product and the harness do not change.

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief, checklist, `CLAUDE.md` | 12:46-13:15 |
| 1 | inventory of the documents read and of every reference into the documents that change | 13:15-13:35 |
| 2 | the roadmap rewritten; `docs/roadmap-history.md` | 13:35-14:30 |
| 3 | notes and status split into `docs/history/` with indexes | 14:30-14:55 |
| 4 | `CLAUDE.md`, `AGENTS.md`'s descriptive parts, `docs/calibration.md`, `docs/session-rules.md` | 14:55-15:10 |
| 5 | verification script under `work/s76/` | 15:10-15:20 |
| 6 | read-only review of the roadmap by a subagent; fixes | 15:20-15:35 |
| 7 | wrap-up, final checks | 15:35-15:50 |

## Decisions

