# Session 80 brief: the precision of `identify --under` on harder generated cases

Written on 2026-11-07, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it), `AGENTS.md` and
   `docs/session-rules.md`.
2. This file, and `docs/session-80-checklist.md` (progress).
3. ADR 0025, experiments 0272-0275 and 0281-0282, `docs/decisions-queue.md`
   item 2, notes "Session 73" and "Session 79".
4. The code: `crates/gunfinger-eval/src/mixes/drops.rs` (the drops kind
   and this session's cases), `mixes.rs` (`render`, `play_audio`),
   `scripts/analysis/tracklist_score.py` (the scores) and
   `crates/gunfinger/src/under.rs`.

## The orchestrator's prompt, verbatim

> Start session 80 of Gunfinger: measure the precision of `identify --under` on the cases its evidence so far does not cover. Its evidence (experiments 0255-0258, 0269-0275 and session 79's) comes from generated double drops of familiar recordings on the turntable ladder: 0 wrong confident partners among the finds, but the external review of 2026-11-05 (`docs/research/2026-11-05-external-review.md`; docs/decisions-queue.md, item 2) points out that those finds are correlated products of one generator and a few recording families, that a related version of the quiet track was found wrong once in the pilot (0258, the Cool Rock upload), and that ADR 0025's "Consequences" lists real double drops, broadcast processing and pitch rides under the foreground as unmeasured. Real annotated double drops do not exist in the corpus and need the user, so this session measures the generated ones that can be built: the shipped option (as session 79 left it), on a registered matrix of harder cases, scored on printed titles. It is a measurement session: the option stays off by default and the product does not change.
>
> You run unattended: the user cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.
>
> The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32). Rendering runs well in parallel here.
>
> Time box: started about 17:48 on 2026-11-07; aim to finish within about 2.5 hours. If a step would run past that, scale it down (the cases the generator already supports first, at NAS scale; one seed per case), record the decision in the brief and continue.
>
> Environment (sessions 33-79's handoffs in docs/session-notes.md; check each):
> - Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
> - Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
> - NAS store: 40,666 records, revision `0bd3f0c23fabc698` (session 75 pruned the records of DJ sets and a mixed CD's track fix). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-78-linux` (262 tracks) and `session-78-linux-nas` (NAS scale), both with the 2003 and Nocny scans since session 78; `regress --strict` fails on any difference; a session that adds baselines names them in its handoff, and the next prompt says which to use.
> - Build `work/bin/s79-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
> - `gunfinger-eval regress` prints what changed and returns success either way: read its output. Since `45f92d5` (2026-10-28) its calibrate reads only the reports its baseline holds; still run variants and other seeds in a scratch work directory, never in the reports directory `regress` compares.
> - The user restructured the code on 2026-10-28 without meaning to change behaviour (commits `b5a84e3` to `f94604a`: harness subcommands in modules, named results, `Search` passed through the core's search, enums for closed sets). Session 60 confirmed both regressions identical on a build of master after it.
> - Since sessions 53-56: `SpansAtSpeed(0.006)` is the default second suppression (ADR 0021); same-title detections (`docs/same-titles.txt`) are scored right (ADR 0022); `identify` prints gaps as "ID - ID" lines with an estimate of their tracks (ADR 0023, 0024). The minimum span stays undecided. A change to an existing decision needs a new ADR (the next number is in `docs/adr/`).
> - Since 2026-11-05: `docs/session-rules.md` holds the standing rules (the older briefs are history), `docs/decisions-queue.md` the decisions waiting for the user, and the roadmap begins with "The active queue" (`docs/research/2026-11-05-external-review.md` explains why). A session that leaves a decision for us adds it to the decisions queue. The six proposals of `docs/research/2026-10-29-matching-and-archive-proposals.md` have all been measured; a session working on one follows its "Evaluation and adoption boundaries".
> - `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
> - Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
> - Tonight (2026-11-07 to 11:00 on 2026-11-08) sessions run one after another, unattended; the user cannot decide or listen. Every listening question asked so far has an answer (docs/status.md, the entries of 2026-10-31 and 2026-11-02), apart from the Planet Dust (DC Breaks Remix) cut (unsure). The user's decisions on a quiet track under an identified one are in docs/status.md, "2026-11-02: the user's answers on a quiet track under an identified one". When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
> - The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.
> - The repository root holds an untracked `yafc20261031.log` that is not this project's work: leave it alone, never commit or delete it; "`git status` clean" means clean apart from it.
>
> First:
> 1. Read CLAUDE.md and follow its reading list, notes "Session 65", "Session 70", "Session 72", "Session 73", "Session 79" and the newest session, ADR 0025, experiments 0255-0258, 0269-0275 and session 79's, docs/decisions-queue.md item 2, and the code of the drops generator (`mixes/`), its playback and fault options, `identify --under` and how the harness scores it.
> 2. Write docs/brief-80.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-80-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s80/listening-digests-start.txt. Experiments number from 0283.
> 3. Build `work/bin/s80-start`; the start regressions follow docs/session-rules.md.
>
> Steps:
>
> 1. Pre-registration, committed before the scored runs: the matrix (each case built from recordings and seeds no session has used where possible, and its number of distinct recording families stated): the quiet partner key-locked (`atempo`) while the foreground is on the turntable ladder, and the reverse; broadcast-style processing of the whole mixture (a compressor or limiter and a lossy re-encode, with stated settings); a foreground with a pitch ride; three tracks at once; the foreground's file left out of the index so that a related version is the foreground's best match (the mistaken foreground); the quiet partner's file left out while a related version of it stays indexed; single-track controls with no partner. The scores: printed tracklist entries (titles, start times) against the construction, wrong confident partners and their duration, partners found by rule and dropped by rule, against the same mixes without the option. What would count as a precision problem (for example any wrong confident partner, or more than a stated share of wrong possible ones), written first.
> 2. Harness additions only where the generator cannot build a case, tested, opt-in, `regress` identical.
> 3. The runs at NAS scale (262 tracks too where cheap), under tmux with logs; renders deleted when no longer needed.
> 4. Experiments (at most 40 lines each, written before the next starts): the cases by family; the mistaken foreground and the related versions.
> 5. Update ADR 0025's open points, docs/decisions-queue.md item 2 and the roadmap's item with what the matrix shows (status stays Proposed).
> 6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
> 7. Wrap-up: a "Session 80" section at the end of docs/session-notes.md with a handoff and any questions for listening (for example a short list of generated cases the user could hear); update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-78-linux` and `regress session-78-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: the precision on each case, any wrong confident partner and why, and anything left.
>
> Rules for this session:
> - `docs/session-rules.md` binds as well; where it and this list differ, this list wins for this session.
> - No push. Commit on master with the configured identity (Jacek Galanciak <jacek.galanciak@gmail.com>). Never rewrite history. Check `git status` before every commit and never use `git add -A`. No AI attribution in any commit: no `Co-Authored-By` line and no mention of an assistant or model.
> - The product's behaviour does not change: no threshold, parameter, rule, default or output change; `identify --under` stays off by default (the default is ours, in ADR 0025). Harness changes and variants are tested, opt-in where they are variants, and keep `regress` identical.
> - Long runs: every run that may last more than about 20 minutes starts detached under tmux with its own log, `/usr/bin/time -v` and an exit line; check the log's exit line afterwards. Give tmux sessions names that are not prefixes of each other.
> - The held-out test set (ed-rush-optical-essential-mix) is out of every step: do not open its manifest or any report of it, do not pass it to any command, do not scan or identify it, and never run test evaluation 5. Never identify, decode or index `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on the candidate mixes of experiment 0065 (among them the mixed CDs it lists and the NAS's `sety/vertical.mp3`). The three development mixes and generated mixes are the only mixes this session searches; name the development sets explicitly, never glob `corpus/sets/*`.
> - The NAS is read, never written, its `.gunfingerignore` included. Do not run `index` or `prune` with the NAS configuration, and do not rebuild or replace the NAS clusters. Never write to `work/peaks`.
> - Agents never write verdicts or listening answers: docs/pair-verdicts.txt does not change, and no answer log is written. Manifests and committed panels do not change. Nothing in this session needs listening; list questions for later.
> - AGENTS.md: no SIMD, no `unsafe`, an experiment file of at most 40 lines written before the next experiment starts (its runs included), evaluation JSON in docs holds file names only, no filename-, set- or tag-derived logic outside the harness's manifests.
> - Repository text follows AGENTS.md's wording rule. The Mac is "the Mac", this machine is "the Linux machine" (its hardware may be named where it explains a time).
> - Leave alone every process this session did not start (a `systemd-inhibit` the user may run among them).
> - Scratch data goes under work/s80/; delete renders you no longer need.
> - Prose: no em dashes (spaced en dashes in prose, ASCII in code).
> - Commit after each step that changes tracked files.
> - Time: the user's soft deadline is 11:00 on 2026-11-08. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.

## State at the start

- `master` at `7dfbd71`, `git status` clean (apart from the untracked
  `yafc20261031.log`). Build `work/bin/s79-final/`: `gunfinger`
  `5829ad77...`, `gunfinger-eval` `8e01bb7a...`.
- NAS store 40,666 records, revision `0bd3f0c23fabc698`; NAS clusters
  `c4d3da2e7e37...` (284 clusters); baselines `session-78-linux` and
  `session-78-linux-nas`. Digests at the start:
  `work/s80/listening-digests-start.txt`.
- `identify --under` as session 79 left it (ADR 0025, Proposed, off by
  default). Its precision evidence: generated drops mixes of seeds 2050,
  2053, 2091 and 2092 at NAS scale (0274, 0282), 0 wrong confident
  partners; the pilot's one wrong related version (0258).
- What the generator builds today (`mixes --kind drops`): turntable
  plays only; a quiet partner (-9 dB), a brief one (20-40 s) and an
  unrelated double drop inside a stronger record; a pair drop; plays
  alone. It has no key lock, no whole-mix processing, no pitch ride in
  the drops kind, no third record and no way to leave a played file out
  of the library searched (the CLI's `--exclude-from` does that).
- The NAS configuration searches `playback = "turntable"`.

## Plan

1. Harness: one opt-in option, `mixes --kind drops --case <case>`, for
   the cases the generator cannot build (below). Without it every draw,
   render and report is unchanged (tested with the existing seed tests;
   `regress` identical).
2. Broadcast processing is done on finished renders with FFmpeg (a
   script under `work/s80/`); it needs no harness change, since the
   timing of the mix does not move.
3. Left-out files: `--exclude-from` lists for `identify`, built from the
   plays the case marks `left_out` (their corpus clusters' files and
   the NAS copies the library map names), added to the usual lists of
   the sweep's held-out recordings.
4. Runs at NAS scale, default and `--under` on the same renders, under
   tmux; scoring with `tracklist_score.py score` and `under-summary`,
   plus a script under `work/s80/` for durations and titles.
5. Experiments 0283 (the cases by family) and 0284 (the mistaken
   foreground and the related versions).

## Pre-registration (step 1), committed before the scored runs

### The matrix

Each case is 12 mixes of the drops kind (12 blocks each: per mix one quiet
partner, one brief partner, two unrelated double drops, one pair drop and
plays alone), drawn from the 208 tagged corpus files (`work/s39/tags-262.json`;
mistaken-over and related-under from every indexed file, Decisions 18:05)
with a seed no session has used, one seed per case. The number of distinct
recordings and shared-material families each case rests on is counted
from its truth and stated in the experiment. Fresh seeds of the 262-track
corpus re-draw recordings earlier sessions also drew (the corpus is the
pool); what is new is each case's construction.

| Case | Seed | Construction | Search |
|---|--:|---|---|
| plain | 8002 | the drops kind as it is | turntable (the NAS configuration) |
| broadcast | 8002 | plain's renders, the whole mixture through `acompressor=threshold=-20dB:ratio=4:attack=5:release=120:makeup=6dB`, `alimiter=limit=0.891:attack=5:release=50` (about -1 dBFS) and re-encoded as MP3 at 96 kbit/s (a second lossy generation) | turntable |
| key-locked-under | 8003 | the second record of each quiet, brief and unrelated block plays key-locked (`atempo`), the first on the turntable | `--playback both` |
| key-locked-over | 8004 | the reverse: the first record key-locked, the second on the turntable | `--playback both` |
| ride-over | 8006 | the first record of each quiet, brief and inside double drop rides its pitch by 1-3% over 5-15 s, starting while the second plays at full level | turntable |
| three | 8007 | the quiet block gains a third record at full level over the quiet one's span: two loud records and a quiet one at once | turntable |
| mistaken-over | 8008 | the first record of the quiet and brief blocks is a shared-material pair's played record over its passage, and that record's files are left out of the library searched, so its related version is the foreground's best match | turntable |
| related-under | 8009 | the second record of the quiet and brief blocks is a pair's played record over its passage, left out of the library searched while its related version stays indexed | turntable |
| controls | 8011 | `--kind long` (4 mixes): plays joined by crossfades, with runs of held-out tracks, no partner anywhere | turntable |

The plays alone in every drops mix (5 per mix) are controls too.

### The scores

Scored on the printed tracklists (`tracklist_score.py score`, every
printed line checked against its rebuilt entry), each case with the
option against the same renders without it:

- each partner's outcome by scenario (printed by default, printed with
  `--under`, gained, found under others as possible only);
- entries gained, right (the played recording or a same-title credit)
  or wrong: a related version of a played record ("related") or nothing
  played there ("unrelated"), with the title printed, its start and its
  duration;
- confident and possible finds under others, right, related or
  unrelated;
- entries of the default lost with the option, and changed gap lines;
- partners found by rule and dropped by rule: the CLI's report holds
  only the plays kept, so a partner dropped by a rule shows as one not
  found under; the counts are of partners found and not found, and
  which rule dropped one is not measured.

### What counts as a precision problem (written before the runs)

- **P1.** Any confident entry gained with the option whose title is
  wrong: unrelated, or a related version, in any case. One exception,
  decided by the user on 2026-10-31 (ADR 0025, "Consequences"): in
  related-under a related version of the left-out quiet partner,
  printed where that partner plays, is the accepted outcome; it is
  counted and listed apart, not as P1.
- **P2.** More than a quarter of the possible finds under others in a
  case match no record playing there or its family (possible finds stay
  out of the tracklist, so this is a lesser problem).
- **P3.** Any entry of the default lost with the option, or any gap line
  changed outside the spans the gained entries take.
- The default's own wrong entries (in mistaken-over its related version
  of the foreground) are reported but are not the option's.

A case with P1 is a precision problem of the option on that
construction, reported with the file, the title and why. No rule is
changed in this session in response.

## Decisions

- 17:55. Cases at 8 mixes each (12 since 18:02, below), one seed per
  case (the time box: 9 cases, two NAS-scale `identify` runs each). The
  262-track scale only if time is left after the NAS runs.
- 17:55. The key-locked cases are searched with `--playback both`: with
  the NAS configuration's `turntable` ladder a key-locked record can only
  be missed, which measures nothing about wrong partners.
- 17:55. Broadcast processing as a post-process of plain's renders, not
  a harness option: it changes no time, so the truth stays the plan's.
- 17:55. "Dropped by rule" cannot be counted from the CLI's report
  without a product or output change, which this session may not make;
  partners found and not found are reported instead.
- 18:02. Before any scored run: 12 mixes per drops case instead of 8
  (the harness change took less time than planned; 36 quiet, brief and
  inside partners per case instead of 24). The controls stay 4 long
  mixes. The start regressions are skipped (`docs/session-rules.md`):
  `s80-start` is byte-identical to `s79-final` (`gunfinger` `5829ad77...`,
  `gunfinger-eval` `8e01bb7a...`), and neither store changed (40,666 and
  265 records, no file newer than brief 79). `s80-run` (from `cf05900`)
  has the same `gunfinger`; only `gunfinger-eval` changed (`41bf0d3d...`).
- 18:05. Seeds and `--tagged`, before any scored run. Of the panel's 24
  pairs (9 families), only Clockwork's two directions have both files
  among the 208 tagged files, so with `--tagged` the drops kind plays
  that one family's pair, and a seed whose sweep holds Clockwork out
  cannot be drawn (8001 failed). Seeds 8001-8020 were probed with
  `--count 0` (panels only); each case takes the first unused seed in
  order whose sweep keeps a pair playable: plain and broadcast 8002,
  key-locked-under 8003, key-locked-over 8004, ride-over 8006, three
  8007, mistaken-over 8008, related-under 8009, controls 8011 (the table
  above is corrected). mistaken-over and related-under need two pairs of
  families other than the mix's own, so they run without `--tagged`
  (every indexed corpus file may play; titles are scored by recording as
  before, and an untagged file prints its file name).
- 18:20. The left-out cases failed to render at first: of the panel's
  pairs only Clockwork's and The Nine's have both records in the corpus,
  so no two pairs of other families were left. The panel's played
  records are mostly corpus files whose related versions are NAS files,
  and these runs search the NAS library, so the two cases draw from
  their own list: pairs whose played record is indexed and not held
  out, the related record in the corpus or only in the NAS library
  (`drops::with_played_record_indexed`, commit `69dd9f5`, build
  `s80-run2`, `gunfinger-eval` `27fdbf19...`; the other cases' draws do
  not read the list). One exclusion list per case leaves its left-out
  recordings out of the library for all 12 mixes, so a block of another
  mix that plays one of them (the mix's own pair) is a mistaken
  foreground too; scoring counts every play by whether the library
  searched holds its recording.
- 18:40. The 262-track scale is not run: the left-out cases need the
  NAS library's related versions (only two pair families have both
  records in the corpus), and the time box goes to the final checks.
- 19:40. The review (a read-only subagent) found no change to the
  default draws, renders or reports, and corrected the documents: P1
  failed in mistaken-over through 2 entries (the third has the right
  title by our same-title judgment, which the script does not read);
  the script's 3 lie in the pair drops of three mixes, not one; the related-under and
  Spider Monkey readings; the default's own wrong plays. It also found
  two limits of the left-out cases' generator: left-out records are not
  beatmatched, and two left-out pairs may share a family. The runs were
  made with that code, so the limits are written into 0284 and the
  notes, and the code is not changed after the runs.
