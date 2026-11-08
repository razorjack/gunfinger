# Session 85 brief: hash-rarity weighting as a diagnostic score

Written on 2026-11-08, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it), `AGENTS.md` and
   `docs/session-rules.md`.
2. This file, and `docs/session-85-checklist.md` (progress).
3. Experiments 0151, 0208, 0276-0277, 0116, 0118, 0129, 0236 and 0291;
   the notes "Session 74" and "Session 84"; the roadmap's "A confidence
   statistic for shared material" and "The filter and famous breaks";
   `docs/decisions-queue.md` item 7; ADR 0014.
4. The code: `crates/gunfinger-core/src/search/refine.rs` (the second
   pass's `hits_near` and `measure`), `Index::postings` and
   `Index::scanned_postings` (`index.rs`), the new
   `crates/gunfinger-eval/src/rarity.rs`, and how `shared.rs` and
   `recall.rs` report each detection; `scripts/analysis/quantisation.py`
   (`auc`, `separation`: the pooled AUC within hit bands) and
   `shared_outcomes.py` (the roles).

## The prompt (verbatim)

> Start session 85 of Gunfinger: measure hash-rarity weighting as a diagnostic score. A detection's evidence today is its count of aligned hits. Session 74 found that the files holding a famous break keep a median 49-64% of their hits when the search's set-aside lists apply, against 75-81% for the query's own file (experiment 0277): a break's evidence is made of the commonest hashes. Related versions gain as much as true plays under every "more evidence" change measured so far (0151, 0208), and the shared-material question (the roadmap's "A confidence statistic for shared material"; docs/decisions-queue.md item 7) has no statistic that keeps 95% of true plays. Weighting each hit by the inverse log of its hash's posting-list length (an inverse-document-frequency weight over the library) has been named once in the roadmap and never measured. The question here is narrow: at equal raw hits, does the weighted score separate shared passages (breaks, related versions) from a record's own material? It is a measurement session: the confidence rule and the product do not change.
>
> You run unattended: the user cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.
>
> The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).
>
> Time box: started about 04:24 on 2026-11-08; aim to finish within about 2 hours. If a step would run past that, scale it down (the shared panels' development halves at 262 tracks first), record the decision in the brief and continue.
>
> Environment (sessions 33-84's handoffs in docs/session-notes.md; check each):
> - Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
> - Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
> - NAS store: 40,666 records, revision `0bd3f0c23fabc698` (session 75 pruned the records of DJ sets and a mixed CD's track fix). NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `c4d3da2e7e37...`, 284 clusters. Baselines `session-78-linux` (262 tracks) and `session-78-linux-nas` (NAS scale), both with the 2003 and Nocny scans since session 78; `regress --strict` fails on any difference; a session that adds baselines names them in its handoff, and the next prompt says which to use.
> - Build `work/bin/s84-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
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
> 1. Read CLAUDE.md and follow its reading list, notes "Session 74" and the newest session, experiments 0151, 0208, 0276-0277 and the shared panels' newest experiments (the roadmap's "A confidence statistic for shared material" lists them; 0116 and 0118 among them), ADR 0014 and 0019, `docs/research/2026-11-02-break-search.md`, and the code of the second pass's hit counting, the index's posting lists, and how the shared-material experiments computed their AUCs.
> 2. Write docs/brief-85.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-85-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s85/listening-digests-start.txt. Experiments number from 0294.
> 3. Build `work/bin/s85-start`; the start regressions follow docs/session-rules.md.
>
> Steps:
>
> 1. Pre-registration, committed before the scored runs: the weight (for example 1 / ln(e + list length), or the inverse log of the length relative to the index's median; one form fixed in advance, a second only as a stated sensitivity check); where it is computed (the second pass's aligned hits, over the index searched, set-aside lists included or not, stated); the cases: the shared panels' development halves (never the validation halves), with true plays, related versions and shared breaks labelled as the panels label them, `recall`'s development panel, and session 74's break lists; the scores: AUC of the weighted score against raw hits for true plays against shared material, overall and at equal raw hits (matched bins), the number of distinct recording families each rests on, and at NAS scale if time allows; and what would make it worth a design note (a stated AUC gain on both panels, no loss on `recall`).
> 2. The score, harness only, reported beside the hit count, tested (the weight, and that it changes no detection or level); `regress` identical.
> 3. The runs at 262 tracks and NAS scale, under tmux with logs where long.
> 4. Experiment (at most 40 lines): the weighted score against raw hits.
> 5. Update the roadmap's items ("A confidence statistic for shared material", "The filter and famous breaks") and docs/decisions-queue.md item 7 if the result bears on it.
> 6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
> 7. Wrap-up: a "Session 85" section at the end of docs/session-notes.md with a handoff; update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-78-linux` and `regress session-78-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: whether rarity separates shared material at equal hits, on which families, and anything left.
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
> - Scratch data goes under work/s85/; delete renders you no longer need.
> - Prose: no em dashes (spaced en dashes in prose, ASCII in code).
> - Commit after each step that changes tracked files.
> - Time: the user's soft deadline is 11:00 on 2026-11-08. Start no new run after about 10:20; finish what is running, then do the review and wrap-up, and list what is left in the handoff.

## State at the start

- `master` at `28eee28`, `git status` clean (apart from the untracked
  `yafc20261031.log`). `work/bin/s85-start/`, built from `28eee28`, is
  byte-identical to `s84-final` (`gunfinger` `cf8393e36bad...`,
  `gunfinger-eval` `40922689951c...`; `work/s85/start-binaries.txt`).
  The corpus store's revision is unchanged (`526a3b978eb3e411`, from a
  10 s `identify`); the NAS store's too (`0bd3f0c23fabc698`). By
  `docs/session-rules.md` the start regressions are skipped; the final
  ones run.
- Baselines `session-78-linux` and `session-78-linux-nas`; NAS clusters
  `c4d3da2e7e37...` (284 clusters). Digests at the start:
  `work/s85/listening-digests-start.txt`.
- The second pass (`refine.rs`, `hits_near`) looks up each query pair's
  hash in `index.postings(hash)`, every list including those the first
  pass sets aside (`Index::skipping_fullest` empties only
  `scanned_postings`), keeps the asset's postings within 24 frames of
  the fitted alignment, and per 10 s window of query time counts the
  densest run within 2 frames when it holds 3 hits or more. The list
  length is at hand there but not reported, and `Evidence` holds only
  windows, hits and the pass.

## Plan

1. Registration (below) committed with this brief, before any scored run.
2. `gunfinger-eval --rarity` (`rarity.rs`): for `shared` and `recall`,
   each second-pass detection of 30 hits or more measured again at its
   alignment, with every hit weighted; reports go to `reports/rarity/`
   below the usual directory. Tested; `regress` identical.
3. Runs: both shared panels' development halves and `recall`'s
   development panel (clean, damaged, brief, brief-damaged), at 262
   tracks with session 83's 109 NAS files (+109) and at NAS scale.
4. Experiment 0294; roadmap, decisions queue item 7; review; wrap-up.

## Decisions

- *(04:35)* **Where the score is computed: a recount in the harness, not
  inside the second pass.** Carrying a weighted sum out of the second
  pass would add a field to the core's `Evidence` (16 constructors, the
  CLI's `explain` among them) for a diagnostic. Instead `rarity.rs`
  repeats the second pass's count at the detection's reported alignment
  (its fitted speed, the line through its first hit), with the same
  window grid, run width (2 frames) and minimum (3 hits), and weights
  each hit it finds. The core and the CLI do not change, so no detection
  or level can move. Its STFT starts at another sample than the second
  pass's (the chain's start is not in the detection), so its raw count
  can differ a little from the detection's hits: both are reported, the
  agreement is measured (step 3 of the registration), and the matched
  bins use the recount's raw hits, so that the weighted and raw scores
  are always counted over the same hits.
- *(04:35)* **Session 74's break lists are not rerun.** `break-search`
  counts first-pass hits from stored peaks, a separate code path; 0277
  already measured the discrete form of the weight on them (the share of
  hits outside the set-aside lists: holders 49-64%, the query's own file
  75-81%). The sensitivity form below is that same share, so the panels'
  result can be read beside 0277. If time remains after step 4, this is
  the first addition.

## Registration (committed before the scored runs)

**Weight.** Each hit of a detection is weighted by
`w(L) = 1 / ln(e + L)`, where `L` is the number of postings in the hit's
hash list in the index searched: the index the run loads after its
leave-outs, every list counted, the set-aside lists (the fullest 1%,
ADR 0014) included, as the second pass counts hits in every list.
`w` falls from 0.76 (L = 1) to 0.30 (L = 25), 0.22 (L = 100) and 0.14
(L = 1,000). The weighted score is `W = sum of w` over the detection's
hits; the mean weight is `m = W / R`, `R` the raw hits of the same count.
**Sensitivity form (stated, not a second candidate):** `k = K / R`, the
share of the hits whose list is not set aside (0277's measure).

**Where.** The recount described under Decisions: second-pass
(`Pass::Fitted`) detections of 30 hits or more, every role, in `shared`
and `recall`; nothing else is measured.

**Cases.**

- Shared panel 1, development half
  (`docs/panels/shared-material-seed-2026-development.json`, 16
  scenarios, 6 families) and panel 2's
  (`shared-material-2-seed-2026-development.json`, 10 scenarios, 3
  families), all three indexes (everything indexed, played recording
  left out, source left out). Positives: detections of role played (the
  played recording's files, as `shared_outcomes.py` reassigns roles by
  the clusters). Negatives (shared material): roles related and version
  (the related recording and the family's other versions). Partner and
  unjudged other files are left out, as in 0151. The `break` queries
  (a 5 s piece looped) are reported apart as well as in the pool. The
  validation halves are never named.
- `recall`'s development panel, seed 2026, query sets clean, damaged,
  brief and brief-damaged: right detections (own cluster or same title)
  as true plays of real damage and brevity; wrong ones reported apart.
- Session 74's break lists: not rerun (Decisions); 0277's figures are
  cited beside the sensitivity form.

**Scales.** 262 tracks with the 109 NAS files the development halves
and `recall` name (session 83's records, `work/s83/stores/today/other`,
equal to the NAS store's byte for byte), then NAS scale
(`--other-peaks-dir`, the saved index). NAS scale decides the bars; +109
is reported beside it.

**Scores.** On each panel (and both pooled):

- S1, overall: AUC of `W` and AUC of `R`, played against related; the
  gain `AUC(W) - AUC(R)`.
- S2, at equal raw hits: the AUC of `m` within bands of `R` (30-59,
  60-119, 120-239, 240-479, 480-959, 960 and more), pooled over bands by
  their pairs as `quantisation.py`'s `separation` pools (0.5 is no
  information at equal hits); the same for `k`.
- S3, families: S2 per family (panel 1's 6, panel 2's 3) and the number
  of families with S2 above 0.5; each figure says how many families it
  rests on.
- S4, recall: S2 for recall's right detections against the related
  detections of both panels pooled; and the rule's counterpart: with
  `W*` the weighted score that accepts as many of the panels' related
  detections (both panels, every index) as `R >= 240` accepts, how many
  of recall's right detections and of the panels' played detections
  `W >= W*` accepts against `R >= 240`.
- S5, the recount: the share of measured detections whose recount `R`
  lies within 5% of the detection's hits, by role.

**Worth a design note** (all at NAS scale; each failure is reported as a
failure):

- D1: on each panel, S2 for `m` is at least 0.60, and S1's gain is at
  least +0.02.
- D2: no loss on `recall`: S4's AUC is at least 0.50, and `W >= W*`
  accepts at least as many of recall's right detections as `R >= 240`.
- D3: S2 above 0.5 in at least 5 of the 9 families.
- Validity: if fewer than 90% of measured played detections pass S5,
  the result is reported as resting on a recount that does not match
  the second pass, and no design note follows from it.

Passing would lead to a design note for a weighted statistic, to be
registered on data these runs did not use (the validation halves, after
a rule is frozen); adopting anything would be the user's decision and an
ADR. Failing closes the weight as a separator of shared material.

## Run log

- *(04:37)* Build `work/bin/s85-run/` from the working tree with
  `--rarity` (`gunfinger` byte-identical to `s85-start`'s,
  `gunfinger-eval` `b31738b87fb0...`). Queues `work/s85/queue.sh`
  (`small`, `nas1`, `nas2`, `nasr`; 8, 10, 8 and 10 jobs) under tmux;
  work directories `work/s85/w109` and `work/s85/wnas` (`setup.sh`:
  the clusters, the map, reflink copies of the cached renders, the NAS
  index hard-linked).
- *(04:38)* Decision: session 83's +109 store (`work/s83/stores/today/other`)
  was deleted at its end. The +109 store is rebuilt as
  `work/s85/stores/other` by copying the NAS store's own records of
  session 83's 109 files (`work/s83/nas-files-kept.txt`), found by their
  record names (`copy_records.py`; all 109 present). Session 83 found
  those records equal to records made from byte copies, so the arm is
  the same; the report directory stays `library-other`.
