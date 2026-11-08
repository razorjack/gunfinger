# Session 87 brief: results by recording family, and the split audit

Written on 2026-11-08, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it), `AGENTS.md` and
   `docs/session-rules.md`.
2. This file, and `docs/session-87-checklist.md` (progress).
3. `docs/research/2026-11-05-external-review.md`, "Decisions and
   experiments"; experiments 0116, 0118, 0124, 0134 and 0216; the
   panels in `docs/panels/` (and their copies in `work/s33/panels`).
4. The code: `crates/gunfinger-eval/src/recall.rs` (`family`, `draw`),
   `clusters.rs` (`Clusters`, `Verdicts`), `titles.rs`, and the
   `families` module added this session.

## The prompt (verbatim)

> Start session 87 of Gunfinger: report results by recording family, and audit whether `recall`'s split keeps families apart. The external review of 2026-11-05 (`docs/research/2026-11-05-external-review.md`, "Decisions and experiments") found that fresh seeds of the same recordings test repeatability, not generalisation: experiment 0216's shared negatives are all the Clockwork relationship, 0124 has 248 of 312 negatives in one family, and 0134 is a declared second use of data that shaped its rule. It also found that `recall`'s split key is normalised artist plus title (`crates/gunfinger-eval/src/recall.rs`), not an audio family, so differently tagged copies or differently named versions of one recording could fall on both sides of the split; this has not been shown to happen. docs/session-rules.md now asks every result to state its number of distinct families. This session gives the harness that number and a family-weighted view, and checks the split. It is a measurement session: the product does not change, and the validation halves stay unsearched (the audit reads their manifests and stored relations only, never searches them).
>
> You run unattended: the user cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.
>
> The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).
>
> Time box: started about 07:34 on 2026-11-08; aim to finish within about 2 hours, and by 10:30 at the latest (the user's deadline is 11:00; this is the night's last session). If a step would run past that, scale it down (the family key and the split audit first; the re-reading of past experiments as a list), record the decision in the brief and continue.
>
> Environment (sessions 33-86's handoffs in docs/session-notes.md; check each):
> - Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
> - Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
> - NAS store: 40,666 records, revision `0bd3f0c23fabc698` (session 75 pruned the records of DJ sets and a mixed CD's track fix). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-78-linux` (262 tracks) and `session-78-linux-nas` (NAS scale), both with the 2003 and Nocny scans since session 78; `regress --strict` fails on any difference; a session that adds baselines names them in its handoff, and the next prompt says which to use.
> - Build `work/bin/s86-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
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
> 1. Read CLAUDE.md and follow its reading list, notes "Session 28" and the newest session, experiments 0116, 0118, 0124, 0134 and 0216, the panels in `docs/panels/` and how they are split, the code of `recall.rs` (the split key), `clusters.rs`, `docs/pair-verdicts.txt` and `docs/same-titles.txt` (read only), and how the panels and generated mixes name their sources.
> 2. Write docs/brief-87.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-87-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s87/listening-digests-start.txt. Experiments number from 0297.
> 3. Build `work/bin/s87-start`; the start regressions follow docs/session-rules.md.
>
> Steps:
>
> 1. Design, in the brief before code: the family key (a duplicate cluster joined with the relations the verdicts and same-titles files record, and, if session 86 left one, its library-wide scratch clusters, stated as a source); where reports print it (the number of distinct families beside query counts; a family-weighted recall and wrong-answer rate beside the query-weighted ones; resampling by family or by whole mix for uncertainty); and the split audit (for each panel, whether any family has members on both sides of its split, using stored relations and peaks only, never searching a validation half).
> 2. The implementation, harness only, tested; existing report fields unchanged so `regress` stays identical; new fields added beside them.
> 3. The audit and a re-reading: run the audit on every panel; recompute, from saved reports, the family counts of the experiments that decided ADR 0019, 0021, 0022 and the shared-material rules, and of 0255-0275; a table of query count against family count.
> 4. Experiment (at most 40 lines): families behind the decisions, and the split audit.
> 5. If the audit finds a family on both sides of a split, record which panel and which members, and what it means for the experiments that used that panel, in the experiment and in docs/decisions-queue.md (the validation halves stay unsearched; the panel files do not change).
> 6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
> 7. Wrap-up: a "Session 87" section at the end of docs/session-notes.md with a handoff (how to read the family figures); update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-78-linux` and `regress session-78-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: how many families the main decisions rest on, the split audit, and anything left.
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
> - Scratch data goes under work/s87/; delete renders you no longer need.
> - Prose: no em dashes (spaced en dashes in prose, ASCII in code).
> - Commit after each step that changes tracked files.
> - Time: the user's soft deadline is 11:00 on 2026-11-08. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.

## State at the start

- `master` at `68d654c`, `git status` clean (apart from the untracked
  `yafc20261031.log`). Builds `s86-final` (master) and `s85-final`.
  `work/bin/s87-start/`, built from `68d654c`, is byte-identical to
  `s86-final` (`gunfinger` `cf8393e36bad...`, `gunfinger-eval`
  `58c1be41b896...`; `work/s87/start-binaries.txt`), and the store
  revisions are unchanged (corpus `526a3b978eb3e411`, NAS
  `0bd3f0c23fabc698`, from a 10 s `identify` with each configuration,
  `work/s87/rev-*.json`), so by `docs/session-rules.md` the start
  regressions are skipped; the final ones run.
- Baselines `session-78-linux` and `session-78-linux-nas`. Digests at
  the start (`work/s87/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...` (100 lines: 31 `same`, 45
  `different`), NAS clusters `c4d3da2e7e37...` (284 clusters, 1,004
  files, 2,875 pairs), answer log `3e99ebca589b...`.
- Relations stored at the start: the corpus clusters
  (`work/reports/duplicate-clusters.json`, 17 clusters of 37 files, 47
  pairs), the NAS clusters (above; 706 pairs not the same recording with
  coverage 0.2 or more), the library map
  (`work/reports/library-nas-dnb-peaks/library-map.json`: each corpus
  file and the NAS file it stands for), session 86's scratch clusters
  (`work/s86/clusters/library-wide.json`: 4,880 clusters of 11,157
  files, 946 related pairs with coverage 0.2 or more, 6 cut links), the
  verdicts and `docs/same-titles.txt` (1 judgment).
- The panels: `recall-seed-2026.json` (80 + 80 sources, one per tag
  family; split by `recall::family`, normalised artist and title, of
  29,057 tag families); `shared-material-seed-2026.json` (12 families,
  split by SHA-256 of the name: 5 development, 7 validation) and its
  development file (16 scenarios); `shared-material-2-seed-2026.json`
  (6 families, 3 and 3) and its development file (10 scenarios);
  `shared-mixes-s21*.json` (12 played files); the sweep panels of seeds
  7 and 2026-2029 (80 draws each, held-out recordings left out of the
  index). `work/s33/panels/recall-seed-2026.json` differs from the
  committed panel only in one name's Unicode normalisation (NFC there,
  NFD in `docs/panels/`): the audit compares names in NFC.

## Design (step 1, written before code)

### The family key

`gunfinger-eval families` (module `families.rs`) builds the key from
stored relations only, from library audio and listening, never from a
search result. Its output names each source it read. Three tiers, each
a union of the previous one with more joins (a union-find over file
names; a file in no relation is its own family; a family is named by
its smallest member in byte order):

1. **recording**: the duplicate clusters in use (corpus and NAS files,
   which already follow the `same` verdicts and keep the `different`
   ones apart), each corpus file joined with the NAS file it stands for
   (library map), and, when `--scratch-clusters` names one, session
   86's library-wide clusters (a scratch file, not in use; stated as a
   source wherever it changes a figure).
2. **title**: plus each `docs/same-titles.txt` judgment, the joins that
   scoring credits as right (ADR 0022).
3. **relation**: plus every pair a verdict names, `same` or `different`
   (a verdict is written on a pair because a search found it sharing
   material; the run counts the verdict pairs no clusters file holds),
   and every stored clusters pair whose alignment covers 20% or more
   of the shorter file (`REPORTED_COVERAGE`) with 30 hits or more
   (`RELATED_HITS`, the census of related recordings; added at 07:42,
   see the decisions below). Chains are followed at this tier, unlike in
   scoring.

Decision (conservative): the counts that state independence use the
relation tier, the coarsest one, so a figure never overstates how many
independent families a result rests on; the recording tier is printed
beside it. The relation tier through clusters pairs can chain
unrelated tracks through a common break; the run prints the largest
families, and a family over 20 files is named in the experiment.

### Where it prints

Decision: offline, from saved reports, so that no existing report
changes and `regress` stays identical. `gunfinger-eval families`
writes the key (`--out`) and, given saved reports, prints and writes
(`--summary`) for each: the queries (or plays) and their distinct
families at each tier; query-weighted recall and wrong-answer rate as
the report counts them; family-weighted ones (each family's own rate,
averaged with equal weight per family); and a 95% interval from 2,000
bootstrap resamples (seed fixed) of whole families for `sweep` and
`recall` reports, of whole mixes for `mixes` reports. Recall counts a
query recalled when a right detection is confident; a wrong answer is
a query with a confident wrong detection (a damaged query's partner
counts as neither, as in the reports). Held-out queries' rate is the
share with a confident answer. Unobservable `recall` queries are left
out, as the reports leave them out.

### The split audit

`scripts/analysis/family_audit.py` reads the key (written twice: with
and without the scratch clusters) and the panels, and searches
nothing. For each panel, whether any family has members on both sides
of its split:

- recall: each development source against each validation source, at
  every tier; the tag-family split key itself (disjoint by
  construction, checked).
- shared panels 1 and 2: every file a family lists (recordings, pairs,
  other versions, unjudged files of its title, and the development
  files' scenarios) on each side; also panel 1 against panel 2, both
  halves.
- sweep panels and shared-mixes panels, whose split is held out
  against indexed: each held-out (left-out) recording's family members
  that the panel's recording or the cluster in use leaves indexed.

A family with members on both sides is listed with the panel and the
members. The validation halves' manifests and the stored relations are
read; nothing of a validation half is searched, scored or aligned.

### The re-reading

`scripts/analysis/family_counts.py` reads an experiment's saved data
(`docs/experiments/data/`, or the session's saved reports in `work/`
where the data file holds none) and counts the distinct files and
families of the library files it names, by the key under which they
appear (truths, plays, queries). Experiments: those that decided ADR
0019 (0091, 0092, 0100-0102, 0107), ADR 0021 (0175-0180, 0209-0211,
0232), ADR 0022 (0188, 0230, 0233), the shared-material rules (0116,
0118, 0124, 0134, 0215, 0216), and 0255-0275. A table of query count
against family count goes into the experiment; an experiment without
saved data is listed as such.

No bar is registered: the session measures and adopts nothing.

## Decisions taken during the session

- 07:42. The relation tier's clusters pairs need 30 hits or more
  (`RELATED_HITS`) as well as 20% coverage. With coverage alone the key
  chained 679 files into one family (China Cup's, 76 corpus files):
  coverage counts the span from the first hit to the last, and 620 of
  the NAS clusters' 706 such pairs have under 60 hits (median 9). With
  30 hits the largest family over all three clusters files holds 13
  files; with 60 or 100 hits the families barely change (1,042, 1,030
  and 1,022 pairs).
- 07:42. Family figures are computed offline (`gunfinger-eval
  families`), so no report that `regress` compares changes.
- 07:50. The key also reads the two frozen shared-material panels: each
  family's recordings join at the relation tier. Four of their
  relations are recorded nowhere else (Clockwork's, from experiment
  0026 and ADR 0019; Fractles' and Watermelon's, from answers 35 and
  36 of the answer log; The Nine's, from the notes after session 4),
  and without them 0216's single family (Clockwork) would count as
  two. Reading the validation families' recordings is a manifest read;
  nothing of them is searched.
