# Session 7 brief: diagnostics that need no listening

Written by the agent for itself on 2026-10-08, so the plan survives context
compaction. The owner is away for about 6 hours and cannot listen to
anything before then. Work autonomously and never stop to ask. When
something is ambiguous, take the conservative option, write the decision
down and continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-7-checklist.md` (progress).
3. `docs/notes-for-owner.md` (findings to report).
4. `docs/status.md` (newest entries at the bottom).
5. The newest file in `docs/experiments/`.
6. `docs/references.md` (cite papers by its keys).

`docs/brief-2.md` to `docs/brief-6.md` hold rules that still apply unless
this brief changes them.

## The owner's prompt, verbatim

```text
Start session 7 of Gunfinger: diagnostics that need no listening. The
owner is away for about 6 hours and cannot listen to anything before
then. Work autonomously and never stop to ask. When something is
ambiguous, take the conservative option, write the decision down and
continue.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it),
   docs/brief-6.md (brief-2 to brief-5 hold rules that still apply unless
   this prompt changes them), docs/notes-for-owner.md (the session 6
   summary and "After session 6" at the end), docs/status.md (newest
   entries at the bottom), docs/roadmap.md, docs/calibration.md,
   docs/references.md, and experiments 0027, 0047, 0049, 0051 and 0053.
2. If `git status` shows uncommitted changes, they are reviewed work from
   after session 6 (the verdict chain check, sparse joins, references).
   Run scripts/check.sh and commit them as one commit before anything
   else.
3. Write docs/brief-7.md (this prompt verbatim, the state at the start,
   the plan with times) and docs/session-7-checklist.md. Point CLAUDE.md's
   reading list at them; keep brief-6 among the older briefs. Commit.
4. Build with --release. Run `gunfinger-eval baseline session-7-start`.
   Record the NAS peak store's revision
   (~/.local/share/gunfinger/nas-dnb-peaks; d428ee9585936326 expected).

Rules for this session:
- No listening and no new verdicts: never write a verdict into
  docs/pair-verdicts.txt. Manifests, the clusters' criterion and the
  confidence rule do not change.
- The default matcher does not change. New measures are harness
  diagnostics. Any change to gunfinger-core must leave
  `regress session-7-start` identical.
- The NAS is not mounted and not read. Use its peak store read-only
  through --other-peaks-dir, and check its revision at the end.
- Wall time and memory are not measured this session; the machine may be
  in use. Record them, but compare CPU only within one run.
- Each measurement is an experiment (docs/experiments/NNNN-slug.md, at
  most 40 lines; data with file names only in docs/experiments/data/).
  Commit after each.
- Cite papers by the keys in docs/references.md.
- [OWNER: keep or delete] Test-set evaluation 3 is approved: right after
  the baseline, scan ed-rush-optical-essential-mix once under the
  unchanged default matcher, record it verbatim in
  docs/experiments/test-set-ledger.md with what changed since evaluation
  2, and change nothing because of it. Sick Note (209 hits under the
  single pass) is predicted to become possible.
- [OWNER: keep or delete] At the start, download the Mixotic dataset
  (https://www.cp.jku.at/datasets/fingerprinting/; mixes, ground truth
  and reference songs, about 10.4 GB) in the background into
  work/datasets/mixotic/, each archive in its own new directory. Do not
  evaluate it this session; record what arrived and its layout for a
  later one.

Items, in order:

1. A peak verifier as a harness diagnostic (roadmap "Evidence beyond
   exact pair hashes", step 1; Sonnleitner & Widmer 2016, §VI-C). For
   each detection in a report, take its asset's reference peaks within
   the aligned span. Map each one to its expected place in the query at
   the fitted speed, playback and alignment. Look for a query peak
   within a small time and frequency tolerance. Per detection, record:
   the reference peaks in the span, the peaks found, their share, and
   the share of 1 s slices of the span with a found peak (support over
   time). Count reference peaks found in the query, never query peaks
   matched: a blend adds query peaks. Measure the chance level with
   them: the same detections with the reference shifted several seconds
   away from the alignment, in both directions.
   - Choose the tolerance from at most three settings, on the
     development scan and sweep seed 2026 only. For scale: Qfp's box of
     12 bins by 18 frames at 4 ms hops is about +-6 bins and +-2
     Gunfinger frames. Then freeze it and report seeds 2027-2029,
     leave-outs 3 and 11, `robust --only combined`, `mixes --count 12`
     and `grid` without changing it.
   - Report three groups separately: identifying detections (the weakest
     especially), false candidates from unrelated recordings, and false
     candidates from related recordings (remixes, VIPs, shared breaks).
     The question: at equal hits, does a verifier measure separate
     identifying from false candidates better than hits do?
   - At NAS scale: the development scan and sweep 2027 with the NAS
     store. Report the pairs of the listening list separately and
     unlabelled, since their ground truth waits for the owner: China Cup
     ~ its Prototype upload, Coma ~ Spraycan, Synthesis VIP ~ "Synthesis
     (Remix)", The Nine ~ its Evol Intent VIP.
   - Change no level. Put the case for or against a verifier rule in the
     notes as a proposal.
   (about 2.5 h)

2. Shared-material scenarios with pairs the owner has already judged
   (roadmap "A confidence statistic for shared material"): the passage
   the Clockwork remix shares with the original (owner: shared material
   at 20:22 of the development mix), and China Cup against The Nine
   (owner: shared drums). Find the shared passages with
   `gunfinger-eval pair`. Build queries from library audio:
   a. the shared passage alone, 10, 20 and 30 s;
   b. the shared passage looped to 60, 120 and 240 s (does repetition
      alone reach the rule?);
   c. excerpts that start in the shared passage and run 10, 20 and 30 s
      into distinctive material;
   d. the played recording and all its rips left out of the index while
      the related one stays; and, as a positive control, the source file
      left out while another rip of the same recording stays.
   Run at 262 tracks under the default matcher, with item 1's verifier
   measures. Record hits, windows, level and verifier measures per
   query. (about 1 h)

3. Relative speed (roadmap "The edge of the ladder"). At NAS scale, take
   the saved panels' indexed excerpts whose recording has a rip that is
   not an identical copy. Leave out the source file and its identical
   copies (library map), keep the other rips, and run the panel's
   speeds. The expected answer is the cluster. Report recall against the
   relative speed (the sweep speed combined with the pair's speed from
   the clusters) and the wrong answers, then the same with
   --extra-rungs 3. Count the NAS clusters whose members differ by more
   than 4% in speed. (about 1 h)

4. Wrap-up:
   - experiment files;
   - a session 7 summary at the top of docs/notes-for-owner.md, with
     findings, proposals and decisions for the owner;
   - docs/roadmap.md, and docs/calibration.md if anything moved;
   - docs/status.md and the checklist;
   - the NAS store's revision checked against the start;
   - scripts/check.sh green;
   - commit.
   (about 0.5 h)

Only if time remains:
5. Supported time for joins: for the sparse joins and the borderline
   pairs, the seconds of the shorter file that hold aligned hits, against
   coverage. Add the findings to the listening list.
```

## State at the start of session 7

- 17:40. The reviewed work from after session 6 (verdict chains, sparse
  joins, `docs/references.md`) passed `scripts/check.sh` and is commit
  `cc81599`. Experiments 0001-0053; ADRs 0001-0008. Test set: 2 of 5
  evaluations used; the owner approves evaluation 3 in this prompt.
- Default matcher: the fitted matcher of ADR 0008 (the fullest 1% of
  posting lists skipped for candidates, the link rules, the second pass
  at the fitted speed, 240 hits in 3 windows, possible at 60), both
  playbacks. `--single-pass` gives the matcher before.
- At 262 tracks: sweeps 2026-2029 540/540, 0 wrong; development 11/11;
  margin 6.65× (experiment 0047). At NAS scale (store revision
  `d428ee9585936326`): development 11/11, 0 wrong; four sweeps 540/540
  with 0, 24, 8 and 8 wrong from three pairs for the owner's ear
  (experiments 0051, 0053).
- Mixotic: the download started at 17:40 (`work/scripts/s7-mixotic-download.sh`).
  All three archives are Google Drive files with pre-2021 identifiers;
  Drive answers each with its sign-in page (HTTP 401 on the view page),
  so nothing arrived. The HTML pages were deleted; the three new
  directories under `work/datasets/mixotic/` are empty. Decision: no
  sign-in and no other source this session; the owner can download
  them in a browser.
- Machine: load average 44 (1 minute) right after `scripts/check.sh`,
  swap 3.5 GB used of 5 GB. Other programs run (RubyMine, T3 Code). Wall
  time and memory are recorded but not compared; CPU only within a run.

## Plan, with times

Start 17:40; the owner is back at about 23:40. Item 4 starts no later
than 23:00. Heavy runs go one at a time through
`work/scripts/timed7.sh` (session 6's wrapper with session 7's log
names), long ones under `nohup`.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| Setup | commit, brief, checklist, release build, `baseline session-7-start`, the NAS store's revision | 0:30 | 18:10 |
| Test 3 | one scan of `ed-rush-optical-essential-mix`, ledger | 0:15 | 18:25 |
| 1 | verifier code (harness, from saved reports where possible); three tolerances on the development scan and sweep 2026; freeze; seeds 2027-2029, leave-outs, `robust --only combined`, mixes, grid; NAS development scan and sweep 2027; three groups; proposal | 2:30 | 20:55 |
| 2 | shared passages with `pair`; queries a-d for both pairs; verifier measures | 1:00 | 21:55 |
| 3 | relative-speed sweep at NAS scale, with and without `--extra-rungs 3`; clusters beyond 4% | 1:00 | 22:55 |
| 4 | wrap-up | 0:30 | 23:25 |
| 5 | supported time for joins, only if time remains | | |

Conservative decisions taken up front:

- Both `[OWNER: keep or delete]` items were left in the prompt, so both
  are taken as kept: test evaluation 3 runs once, right after the
  baseline, and nothing changes because of it.
- The verifier lives in `gunfinger-eval` and reads what it needs (the
  query's peaks at the fitted speed and the asset's stored peaks)
  without changing `gunfinger-core`'s behaviour. Where a saved report
  holds the detections, the verifier measures them from the report
  rather than searching again, so the NAS-scale reports of session 6 can
  be used if `regress` shows the matcher unchanged.
- Item 3 is a harness option on the sweep, not a new ground truth: the
  expected answer is the cluster as it stands.
