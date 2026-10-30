# Session 63 brief: how much the second pass loses to its initial speed estimate

Written on 2026-10-30, so the plan survives context compaction. Work
autonomously and never stop to ask: where a decision is needed, take the
conservative option, write it down here and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it) and `AGENTS.md`.
2. This file, and `docs/session-63-checklist.md` (progress).
3. The proposals note `docs/research/2026-10-29-matching-and-archive-proposals.md`
   (§3 and "Evaluation and adoption boundaries"); the roadmap's "Speed
   that wanders within a play" and "The edge of the ladder";
   `docs/brief-62.md`, `docs/brief-59.md`.
4. `docs/session-notes.md` ("Session 51", "Session 59", "Session 62");
   experiments 0024, 0030, 0032, 0226-0230, 0242 and this session's
   (0251 and up); ADR 0008 and 0021; `docs/calibration.md`.
5. `crates/gunfinger-core/src/search/refine.rs` (`at_fitted_speed`, the
   fits), `search/chains.rs` (`chain_speed`), `search.rs` (`Options`),
   `crates/gunfinger-eval/src/matching.rs`.

`docs/brief-2.md` to `docs/brief-62.md`, `docs/brief-24b.md` and
`docs/brief-nas.md` hold rules that still apply unless this brief
changes them.

## The prompt, verbatim

```text
Start session 63 of Gunfinger: how much the second pass loses to its initial speed estimate (proposal 3 of `docs/research/2026-10-29-matching-and-archive-proposals.md`). The second pass (`search/refine.rs`) measures a candidate again at the speed the first pass's chain gives (`chain_speed`: a fit through line centres when the chain spans enough, otherwise a hit-weighted mean of rung speeds). Two measured cases show that this estimate decides answers: in 0242 one first-pass line losing one hit changed the kept rung, moving Organikismness - Echos's fitted speed from +1.147% to +0.933% and its second-pass count from 455 to 503 (both right); in 0226 widening the ladder moved Siren Bass from -8.09% to -8.18% and its count from 305 to 214, losing a confident answer. The optional `--speed-per-stretch` (0032) fits stretches afterwards, but is not a search for a better initial alignment. This session measures the available gain first, with a fine local speed search around each candidate's estimate as the reference, on weak true candidates and on shared-material negatives, without changing confidence; then, if the reference shows a gain worth having, a bounded iterative fit against it. It is a measurement session: the second pass and the product do not change; a change is ours, with an ADR.

You run unattended: the user is asleep and cannot answer or listen to anything. Never stop to ask. Where a decision is needed, take the conservative option, write it down in the brief and continue. An orchestrating thread launched you and will read your final message and session notes; no other session runs in this checkout while you run.

The machine: Fedora Linux 44, x86-64, AMD Ryzen 9 5950X (16 cores, 32 threads), 64 GB RAM, DDR4-3600; default jobs (32).

Time box: started about 05:45 (CET); aim to finish within about 2.5 hours (the user's soft deadline is 10:00). If a step would run past that, scale it down (the local-search reference only, 262 tracks first), record the decision in the brief and continue.

Environment (sessions 33-62's handoffs in docs/session-notes.md; check each):
- Rust: `export PATH="$HOME/.cargo/bin:$PATH"`; `RUSTUP_TOOLCHAIN=1.98.0` for `scripts/check.sh` and builds. FFmpeg 8.1.3, Python 3, Node, tmux, `/usr/bin/time`. zsh does not split a command held in a variable: run queues from `sh` scripts.
- Corpus configuration `~/.config/gunfinger/config.toml` (store `work/peaks`, never written by a session). NAS configuration `~/.config/gunfinger/nas-dnb.toml`: store `~/.local/share/gunfinger/nas-dnb-peaks`, library `/mnt/atlas/Music/dnb` (read only; on Ethernet), reached by the harness with `--other-peaks-dir`; saved index `work/indexes/library-nas-dnb-peaks.index`.
- NAS store: 40,678 records, revision `1fc6fe50293ad330`. NAS clusters `work/reports/library-nas-dnb-peaks/duplicate-clusters.json`: SHA-256 `ce293fe6165e...`, 284 clusters. Baselines `session-54-linux` (262 tracks) and `session-54-linux-nas`.
- Build `work/bin/s62-final/` (master). A NAS-scale development scan searches in about 24 s and a NAS regress takes about 4 minutes. `recall`, `shared` and `clusters` need `--panels work/s33/panels`.
- `gunfinger-eval regress` prints what changed and returns success either way: read its output. Since `45f92d5` (2026-10-28) its calibrate reads only the reports its baseline holds; still run variants and other seeds in a scratch work directory, never in the reports directory `regress` compares.
- The user restructured the code on 2026-10-28 without meaning to change behaviour (commits `b5a84e3` to `f94604a`: harness subcommands in modules, named results, `Search` passed through the core's search, enums for closed sets). Session 60 confirmed both regressions identical on a build of master after it.
- Since sessions 53-56: `SpansAtSpeed(0.006)` is the default second suppression (ADR 0021); same-title detections (`docs/same-titles.txt`) are scored right (ADR 0022); `identify` prints gaps as "ID - ID" lines with an estimate of their tracks (ADR 0023, 0024). The minimum span stays undecided. A change to an existing decision needs a new ADR (the next number is in `docs/adr/`).
- The research note `docs/research/2026-10-29-matching-and-archive-proposals.md` (2026-10-29) records six unmeasured proposals and its evaluation boundaries; any session working on one follows its "Evaluation and adoption boundaries".
- `scripts/check.sh` has one load-sensitive test (`decode::watch`) that can fail while every thread is busy; rerun on an idle machine before suspecting a change.
- Renders from earlier sessions sit in `work/mixes/`; leave them, and keep your own renders' total under about 40 GB.
- The user is asleep and cannot listen. Pending listening (Star Trails, Into Music) stays for later. When you record anything the user said, paraphrase it in clean English; never quote the user's raw words in committed text.
- The shell's safety check has at times failed to return verdicts for minutes; if a command gets no verdict, wait and retry, keep commands short, and never treat the failure as a refusal.

First:
1. Read CLAUDE.md, AGENTS.md (every rule applies, including the wording rule), docs/brief.md (never edit it), the newest brief and docs/brief-59.md, docs/session-notes.md ("Session 51", "Session 59" and the newest session), docs/status.md (the newest entries), the proposals note (§3 and "Evaluation and adoption boundaries"), ADR 0008 and 0021, experiments 0024, 0030, 0032, 0226-0230 and 0242, `docs/calibration.md`, the roadmap's "Speed that wanders within a play" and "The edge of the ladder", and the code of `search/` (`chain_speed`, `refine`, `speed_per_stretch`) and `pair.rs` (`--fine-rungs`).
2. Write docs/brief-63.md (this prompt verbatim, the state at the start, the plan and each decision as you take it) and docs/session-63-checklist.md, point CLAUDE.md's reading list at them, and commit. Record the SHA-256 of docs/pair-verdicts.txt and of the NAS clusters file in work/s63/listening-digests-start.txt. Experiments number from 0251.
3. Build `work/bin/s63-start`; `regress session-54-linux` and `regress session-54-linux-nas` identical.

Steps:

1. Pre-registration, committed before any variant run: the reference (for each second-pass candidate, measure again at speeds on a fine grid within a bounded window around the estimate, for example ±0.4% in 0.02% steps, and with the offset re-fitted from nearby correspondences; keep the best and the default), the candidates (weak true plays: the sweep's and recall's confident detections under about 2-3 times the threshold, between-rung speeds, other rips, short chains, Echos and Siren Bass; negatives: shared-material panels' related versions and calibrate's strongest false candidates at both scales; the development halves only), the selection effect (maximising over many alignments raises wrong versions too: report the best-of-grid counts for negatives as carefully as for positives, and a split diagnostic that fits on one half of a candidate's evidence and counts on the other), the scores (how often the best alignment changes an answer, not only a count: confident gained or lost, wrong confident added, the margin between true and false), and what would make a bounded fit worth an ADR proposal.
2. The reference as an opt-in harness diagnostic (harness only; `regress` identical with it off), tested.
3. The runs at 262 tracks and NAS scale, under tmux with logs where long; seeds other than 2026 in a scratch work directory. Also perturb discovery as 0242 did (the set-aside lists of another index, `recall --set-aside-like`) and check whether the reference makes the answer independent of the perturbation.
4. Experiments (at most 40 lines each, written before the next starts): the available gain on positives and negatives; the perturbation; and, only if the gain is worth it, a bounded iterative fit (a small fixed number of rounds, continuity constrained) against the reference.
5. A short decision note (`docs/research/2026-10-30-second-pass-fit-decision.md`): the gain, the selection effect on negatives, the cost, and a recommendation that is ours to take. Update the roadmap's matching bullets and the proposals note's §3 with a pointer.
6. A read-only review of the session's commits and documents by a subagent (it must not launch further agents); fix what it finds.
7. Wrap-up: a "Session 63" section at the end of docs/session-notes.md with a handoff; update docs/status.md and the checklist. Final checks: `scripts/check.sh` green, `regress session-54-linux` and `regress session-54-linux-nas` identical with the final build, docs/pair-verdicts.txt unchanged, the clusters file's SHA-256 unchanged, `git status` clean. Commit. Your final message: the available gain, the effect on negatives, the recommendation, and anything left.

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
- Scratch data goes under work/s63/; delete renders you no longer need.
- Prose: no em dashes (spaced en dashes in prose, ASCII in code).
- Commit after each step that changes tracked files.
- Time: the user's soft deadline is 10:00. Start no new run after about 09:30; finish what is running, then do the review and wrap-up, and list what is left in the handoff.
```

## State at the start of session 63

- 2026-10-30, 05:40 by the Linux machine's clock (CET). The time box is
  about 2.5 hours: aim to finish by about 08:15. The soft deadline is
  10:00; no new run starts after about 09:30.
- `master` at `6379a40`, clean. Git identity `Jacek Galanciak
  <jacek.galanciak@gmail.com>`.
- Digests (`work/s63/listening-digests-start.txt`):
  `docs/pair-verdicts.txt` `51dd1bbac501...`, the NAS clusters file
  `ce293fe6165e...`, `work/audition/answers.jsonl` `4b83d2fc9a2e...`.
- Experiments 0001-0250; this session numbers from 0251. ADRs
  0001-0024. Test set: 4 of 5 evaluations used (none in this session).
- The second pass today (`search/refine.rs`, `at_fitted_speed`): the
  chain's lines on the detection's playback within 1.5 ladder steps of
  its strongest line give the speed (`chain_speed`: a hit-weighted fit
  through line centres with 3 lines or more over enough span, otherwise
  the hit-weighted mean of their rungs); the alignment pivots at the
  strongest line's centre. The span (the chain's first to last hit, 10 s
  of margin either side) is analysed once at that speed; hits of the
  asset within 24 frames of the alignment are kept, and each 10 s
  window's densest run within the line span counts when it has enough
  hits. Candidates with fewer than 10 first-pass hits keep their first
  measurement. `--speed-per-stretch` (0032) refits stretches of 3
  windows afterwards from the drift of their deviations.
- Default reports already made by today's default matcher at NAS scale,
  reused as this session's default where named in the plan: session 62's
  downmix runs in `work/s62/w/reports/library-nas-dnb-peaks/` (`recall`
  clean and brief, its other rips, panel 1, the three scans) and
  `work/s62/w2/` (panel 2); the baselines `session-54-linux` and
  `session-54-linux-nas` (the sweep of seed 2026, the Stakka scan).

## Plan

| Step | What | Expected |
|---|---|---|
| First | reading; digests; brief; start build and regressions | 05:40-06:00 |
| 1 | pre-registration (below), committed with this brief | 06:00 |
| 2 | the reference and the split in the core's options, opt-in from the harness (`--second-pass-fit`); tests; both regressions identical with the run build | 06:00-06:40 |
| 3 | runs (scratch `work/s63/w`, `w262`, `wp1`, `wp2`, `wlike`) under tmux | 06:40-08:00 |
| 4 | experiments 0251 (the gain on positives and negatives), 0252 (the perturbation), 0253 (the bounded fit, only if the bar below says so) | with step 3 |
| 5 | decision note; roadmap; proposals note §3 | 08:00-08:15 |
| 6-7 | review by a separate agent; notes, status, checklist, final checks | 08:15-08:50 |

### Pre-registration (committed before any variant run)

**The question.** How much does the second pass lose by measuring a
candidate at the chain's speed, and does a better alignment change
answers, for true plays and for wrong versions alike?

**The reference** (`--second-pass-fit grid`). For each candidate the
second pass measures whose default measurement holds 80 hits or more (a
third of the rule's 240; below it a ±0.4% move would need to triple the
hits; the largest best-to-default ratio seen is reported to check that
margin): the default measurement, and the same span analysed again at
41 speeds s0 × (1 + 0.0002 k), k = -20..20 (±0.4% in 0.02% steps, s0 the
chain's speed), on the same pivot. At each speed the asset's hits are
collected within 48 frames of the alignment; the offset is re-fitted as
the hit-weighted mean deviation of each window's densest run; the hits
within 24 frames (today's reach) of the re-fitted alignment are measured
as today. The measurement with the most hits is kept (at a tie the
default, then the smaller |k|, then the slower speed); its speed,
boundaries and evidence replace the default's. Confidence is today's
rule (240 hits in 3 windows), applied unchanged; the second suppression
follows as today. Candidates below the floor, and the first pass, are
unchanged.

**The split diagnostic** (`--second-pass-fit cross`). The same 42
measurements. A candidate's measured windows are split by the parity of
their window number. On each half, the alignment (speed and re-fitted
offset) with the most hits on that half is chosen among the 42 (ties as
above); the other half's windows are counted at that alignment, within
24 frames. The evidence is the two counted halves together; the speed
the hit-weighted mean of the two halves' chosen speeds. When both halves
choose the default it equals the default. A gain under the grid that
the split keeps is not selection; one the split loses is.

**The bounded fit** (`--second-pass-fit iterated`, run in step 4 only if
the bar for the gain passes). From the default measurement, at most 2
rounds: a new speed and offset from the hit-weighted least-squares drift
of the measured windows' deviations (the fit `--speed-per-stretch` uses
for a stretch, over the whole candidate), and the span analysed again
there. A round whose speed would leave ±0.4% of the chain's speed, or
with fewer than 2 measured windows, stops the fit at the previous
round; a change below 0.005% stops it after that round. The last
round's measurement is kept, more hits or fewer (no choice by count).
The same floor of 80 hits.

**Candidates** (development data only; the held-out test set, the
panels' validation halves and unassigned mixes are out):

- Positives: `recall --queries clean,brief` and `recall
  --other-rips-only --queries clean,brief` at NAS scale (Echos is query
  911, and 910); `sweep` (seed 2026) at 262 tracks and NAS scale (odd
  percents lie between rungs); `sweep --other-rips` (seed 2026, NAS
  scale; Siren Bass's upload at -8%); the three development scans at NAS
  scale (Stakka also at 262); the played files of both shared panels'
  development halves. Weak true plays: confident detections of the
  truth's cluster or title under 720 hits (3× the rule) by the default,
  and detections of the truth's cluster with 80-239 hits.
- Negatives: the shared panels' `related` and `other` detections
  (development halves of panel 1 and panel 2, NAS scale); every
  detection in `recall`, the sweeps and the scans of a file outside the
  truth's cluster and title (not a damaged query's partner); calibrate's
  false candidates at 262 tracks and NAS scale (from the sweep and the
  Stakka scan of the variant's own reports directory).

**The perturbation.** `recall --queries clean,brief --set-aside-like
work/s36/w/indexes/library-nas-dnb-peaks.index` (the lists the pre-prune
index set aside, read only; 0242) under the default and the grid; for
each, the queries whose confident answer or right hits differ between
today's lists and the old ones, and Echos's speed and hits.

**Scores.** Per set and per variant against the default: (S1) confident
right answers gained and lost; (S2) wrong confident detections added
(in the panels, `related` or `other` newly confident; calibrate's false
accepted); (S3) the ratio of hits, variant over default, for weak true
candidates and for negatives of 80 hits or more (median, 90th
percentile, largest); (S4) per query the margin between the best right
and the best wrong detection, and calibrate's weakest identifying,
strongest false and margin at both scales; (S5) the selection effect,
the grid's ratios beside the split's, for positives and negatives;
(S6) the fitted speed's error against the played speed (sweep and
`recall` clean, own file); (S7) cost: the Stakka scan's search time at
NAS scale, default against each variant run, alone on the machine.

**The bar for the gain** (decides whether step 4's bounded fit runs):
(a) the split gains 3 or more confident right answers over the
positives at NAS scale, or (b) the split's median hit ratio over the
weak true candidates is 1.05 or more. The selection effect is reported
either way.

**What would make a bounded fit worth an ADR proposal** (all of them,
for the iterated fit against the default): (B1) no new wrong confident
detection in any set at either scale, and no related version newly
confident in the panels (the known Cool Rock Stuff excerpt against the
Mobb Deep / Urban Takover upload, wrong under the default, and
same-title credits are not new); (B2) no confident right answer lost
over the positives; (B3) at least half of the split's gained confident
answers; (B4) calibrate at each scale: the strongest false candidate at
most 5% higher and the margin at most 5% lower; (B5) the Stakka scan's
search time at NAS scale at most 3% more; (B6) under the perturbation no
more moved queries than the default's. If every item passes, the
decision note recommends an ADR proposal, which is ours to take;
otherwise it records what failed.

## Decisions

- **(05:43) Start checks.** `work/bin/s63-start/` from master
  (`cmp`-equal to `s62-final`).
  `regress session-54-linux` (0:50) and `regress session-54-linux-nas`
  (3:47) identical, every report equal to its baseline's apart from
  `wall_seconds`.
- **(05:52) The variants live in the core's options, set only by the
  harness.** The second pass and its measurements are private to
  `search/refine.rs`, so the reference cannot be built in the harness
  alone. As `--speed-per-stretch` does, the core gained an opt-in
  field, `Options::second_pass_fit` (`SecondPassFit`: `ChainSpeed`, the
  default, `BestOfGrid`, `CrossFitted`, `Iterated`; `search/refine/fit.rs`),
  which only the harness's `--second-pass-fit` sets; the matcher, the
  CLI and every default are unchanged (`383d91a`). The bounded fit was
  written with the others, since it is a few lines; it runs only if the
  bar for the gain passes. `refine.rs` gained a `Candidate` (the inputs
  the measurements share) instead of an eight-argument function.
- **(05:57) Regressions with the run build** (`work/bin/s63-run/`,
  fits off): `regress session-54-linux` (0:51) and `regress
  session-54-linux-nas` (3:47) identical, 8 of 8 reports;
  `cargo test --release` for `fit::` (4 tests) and the harness's
  matching tests green.
- **(05:57) Runs started** (scratch directories from
  `work/s63/setup.sh`: reflink copies of the sweep, `recall` and both
  panels' renders, copies of the clusters files, the library map and
  the baselines, the saved NAS index hard-linked). Five tmux queues:
  `s63qgrid` and `s63qcross` (NAS scale: `regress
  session-54-linux-nas`, `sweep --other-rips`, the two other development
  scans, `recall` clean and brief, its other rips), `s63pgrid` and
  `s63pcross` (both panels' development halves), `s63q262` (`regress
  session-54-linux` under each fit, then the default's `sweep
  --other-rips` at NAS scale). The default's other reports are session
  62's downmix runs (`work/s62/w`, `work/s62/w2`), made by today's
  default matcher; the default's sweep and Stakka scans are the
  baselines'.
- **(05:57) The experiments run in order.** The perturbation (0252)
  starts after 0251 is written; the bounded fit (0253) after 0252.
- **(06:26) A run added to 0251: 0226's Siren Bass case.** Under
  today's ladder Siren Bass's upload at -8% is confident by the default
  (305 hits at -8.093%); 0226's loss came with extra rungs. So `sweep
  --other-rips --extra-rungs 1` runs under the default and each fit
  (tmux `s63siren`; reports in each directory's `extra-rungs-1/`), the
  one case the prompt names that today's ladder does not reproduce.
- **(06:56) The bar for the gain passes on (a); the bounded fit runs,
  and the experiments are renumbered.** On `recall`'s brief plays the
  split gains 5 confident right answers and loses 1 (the grid the same
  5 and 1); nothing else measured so far gains one. (b) fails (the
  split's median ratio 1.017 on the brief plays, 0.99-1.0 elsewhere).
  So the bounded fit runs, as registered. For the order of experiments
  (each file before the next one's runs), the bounded fit becomes 0252
  and the perturbation 0253, so that the perturbation's runs can
  include the bounded fit for B6 (the default, the grid and the
  iterated fit under the old lists). Queues `work/s63/q-iterated.sh`,
  `r-iterated.sh`, `p-iterated.sh` (directories `wi`, `wir`, `wpi1`,
  `wpi2`) start once 0251 is written.
- **(06:57) Panel 2 beside panel 1** (a time-box decision, as in
  session 62): panel 1 under each fit had made 27 of its 40 searches
  in an hour, so panel 2 runs now under each fit in its own directory
  (`wpg2`, `wpc2`; tmux `s63ptwogrid`, `s63ptwocross`), and a watcher
  (`work/s63/stop-p1-*.sh`) stops the first queues as soon as panel 1
  logs its exit, before they would repeat panel 2.
- **(07:37) 0251 written and committed (`9bb890b`); 0252's runs
  started**: tmux `s63iq` (the standard protocol at NAS scale, other
  rips with and without one extra rung pair, the two other scans, then
  `regress session-54-linux`), `s63ir` (`recall` clean and brief),
  `s63irtwo` (`recall`'s other rips, directory `wir2`; `s63ir` is
  stopped by `work/s63/stop-ir.sh` after its first run), `s63ipone` and
  `s63iptwo` (the two panels).
- **(07:45) 0251's conclusion corrected** after a count of how the
  brief plays' answers changed: most cross 240 hits in 3 windows (grid
  25 of 28 changes, split 15 of 18 gains and 2 of 3 losses); 2 gain and
  1 loses a third window. The file said the 3-window rule decided them.
