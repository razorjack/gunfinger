# Decisions waiting for the user

Open decisions that block or shape the next sessions, with the evidence
and a recommendation. Sessions add a row when they leave a decision for
us. When the user decides, a session records the decision here (moved to
"Decided"), in `docs/status.md`, and in an ADR where one is needed.
Several decisions can then be adopted together in one session with one
protocol rerun, as session 57 did. Started on 2026-11-05 from the
external review (`docs/research/2026-11-05-external-review.md`).

## Open

### 1. The minimum span (since 2026-10-27)

- **Question.** Whether 15-20 s plays can be confident wherever the 10 s
  windows fall: B(15) (200 hits over at least 15 s of supported time),
  A(20), or today's 3 windows.
- **Evidence.** Roadmap, "A minimum span instead of 3 windows";
  experiments 0136-0142 and 0146-0148. On session 28's brief plays at
  NAS scale, 15 s plays are confident 504 of 522 times under B(15),
  against 228 today. On the fair shared-material test, B(15) fails F1
  by 6 of 168 wrong answers: three 20 s passages placed exactly on a
  window border, where today's rule cannot be confident but names the
  same wrong files at 8-9 of their other positions. A(20) meets F but
  makes fewer brief plays confident.
- **The open point.** Whether such a wrong answer counts as new. The two
  reviewers differ: one reads it as today's error at one more offset;
  the other warns that dismissing it as "the same wrong file" weakens
  the precision requirement, and asks for errors to be scored both per
  passage and as a probability over placement before a new test.
- **Recommendation.** Decide between B(15) and A(20). Then a session
  registers the placement-aware scoring, freezes the rule and confirms
  it once on the shared panels' validation halves, which were kept
  unsearched for this purpose, and writes the ADR.

### 2. `identify --under` on by default (ADR 0025, Proposed)

- **Evidence.** Experiments 0272-0275 and 0281-0282. On the three
  development mixes no entry is gained or lost. On generated double
  drops (32 mixes) the option gains 39 right entries with 0 wrong, and
  on two fresh seeds (32 mixes) 45, also with 0 wrong. Session 79 fixed the fourth drop rule, which had dropped two
  right partners whose lines happened to put their files' starts near
  another track's (0274, 0282), and put each entry's mask on its own
  worker: at NAS scale the option now takes 1.85-2.01 times the
  default's time (about 17-19 s more per hour of mix; before 2.85-2.94
  times, 35-39 s), with 2.0-2.8 GiB more peak memory. The registered bar
  of twice holds on two development mixes and fails on Nocny by 0.3 s
  (0281).
- **Still unmeasured.** Real annotated double drops, key lock under the
  louder track, broadcast compression, and a mistaken foreground (a
  wrong confident entry taken out). The precision evidence rests on
  generated mixes drawn from the 262-track corpus.
- **Recommendation.** Decide the acceptable added seconds per hour of
  mix (now about 18) and peak memory (about 2.5 GiB more). Keep the
  option off until at least one real double drop with a known tracklist
  has been checked; a cap on masks in flight would bound the memory if
  that matters.

### 3. The inverted-channel repair (since 2026-10-31)

- **Evidence.** Experiments 0248, 0262 and 0263: about 0.4% of NAS rips
  have an inverted channel; a repair at indexing recovers the four
  plainly inverted recordings (0 of 48 to 48 of 48 excerpts) with no new
  wrong answer, but loses all 48 queries of the as-ripped audio. The
  registered bar failed through the set-aside cut's moves, not a wrong
  answer.
- **The user's idea (2026-10-31).** Index both the original and the
  repaired form of a flagged file.
- **Recommendation.** A harness pilot of two records per flagged file,
  scored apart and never summed or chained across the two forms (roadmap
  queue, item 6). Then an ADR.

### 4. A command that lists the tracks using a break (since 2026-11-02)

- **Evidence.** Experiments 0276-0277 and
  `docs/research/2026-11-02-break-search.md`: 59 of 64 expected holders
  found, 51 in the first five, but only 25 first, because a famous break
  lines up with 120-403 files.
- **Open.** Whether the command is wanted at all. Its next measurement
  treats every judged holder as relevant (precision at K over judged
  holders, with unjudged candidates kept as unjudged) and needs a short
  listening pack sampled across ranks.

### 5. The bar of zero wrong answers and the seven Cool Rock detections

- **Observed.** `AGENTS.md` requires the sweep to clear zero wrong
  answers before any set is scanned. At NAS scale `calibrate` reports 7
  false detections accepted, all of the Cool Rock Stuff pair that the
  user judged on 2026-10-30 to carry different titles (0239, 0278). The
  calibration register's rule says to keep the confident threshold
  about twice the strongest false candidate; that candidate is 282 hits
  and the threshold is 240. At 262 tracks the bar holds (0 accepted).
- **Recommendation.** Keep the threshold: raising it to about 564 would
  cost brief plays and still not separate long passages that are truly
  shared. Record that the zero-wrong bar is met at 262 tracks and that
  the 7 NAS detections are a known failure kept visible in every
  baseline, and change the register's "If it moves" text to say so.

### 6. Real mixes for development and a new held-out set

- **Observed.** The three development mixes score 11/11, 12/12 and 17/17
  at both scales with 0 wrong, so they no longer show failures. The
  held-out test set has one evaluation left and has been heard and
  corrected since evaluation 1 (ADR 0006 says evaluation 2 was not
  blind). 136 real sets on the NAS and the user's own mixes have been
  listed as candidates since session 9 and have no role.
- **Recommendation.** The user gives roles to a batch of real mixes
  before anything searches them (ADR 0004): 4-6 new development mixes
  with tracklists written by ear (the user's own mixes first), and 3-4
  as a new held-out set whose tracklists are committed before any
  search. An ADR amending ADR 0004 records the new set. Evaluation 5 is
  spent only on a frozen bundle of adopted changes, with the new set.

### 7. Presentation of related versions instead of a statistic

- **Observed.** The roadmap calls shared material "the main open
  accuracy question" at NAS scale, while on 2026-10-31 the user said a
  related version named for the track playing is acceptable, because the
  listener learns the title and can find the right version by ear.
- **Recommendation.** If the user confirms this view, the roadmap's "A
  confidence statistic for shared material" becomes "presentation of
  shared passages" (for example marking a confident play whose file
  shares its passage with another library file), and the statistic work
  stops.

### 8. Diagnostic code kept after a negative result

- **Observed.** `docs/brief.md` and `AGENTS.md` say that an experiment
  that loses is deleted from the tree. The harness and the core keep
  several opt-ins and subcommands whose experiments concluded "not
  adopted" (for example the second-pass fits of 0251-0253, the power-sum
  front end of 0245-0250, `harmonic` of 0259-0261, `drop_fullest`,
  `trim_ends`, `overlap_by_chains`). The harness has about 44,000 lines
  against the core's 14,000, and `scripts/analysis/` holds 112 scripts.
- **Recommendation.** Amend the rule: a kept diagnostic names its next
  use and the condition for deleting it, in the experiment or the
  roadmap item; everything else is deleted in one cleanup session with
  `regress` identical.

### 9. Smaller ones

- **Worker count on Linux.** At 16 workers, NAS-scale scans took about
  30% less time than with the default 32 on the Linux machine (0158).
  Should the default count cores rather than hardware threads? (Session
  76: since the two `take_part` changes of 0167, in master since 0169,
  32 workers are faster than 16, 37.9 s against 45.3 s for the Stakka
  scan's search, and the roadmap's text concluded that the default need
  not change.)
- **Key-locked ladder by default.** `--playback both` costs 1.57 times
  the CPU of the turntable ladder alone, and no development play uses
  key lock. The user chose both on 2026-10-06. One option is to keep it
  and search key lock only over stretches with no confident match
  (roadmap queue, item 8); that needs no reversal.
- **Public datasets.** Mixotic and UnmixDB need a download by browser
  (`docs/references.md`, "Datasets"). With them, `gunfinger-eval
  seconds` gives the first external per-second score.
- **The Planet Dust (DC Breaks Remix) cut** stays "unsure" from the
  2026-10-30 listening.
- **`AGENTS.md`'s development sets** (session 76). "Evaluation rules"
  names `stakka-skynet-knowledge` and `razorjack-2003-03-29` as the
  development sets; `nocny-trans-vertical-2002-08-23` has been the third
  since session 18 (experiment 0100; `CLAUDE.md`, the roadmap's "More
  development mixes"). Should the rule name all three?
- **What `AGENTS.md` says `docs/status.md` holds** (session 76). "Before
  changing a decision" describes it as "current state, exact commands
  and numbers"; since 2026-11-05 `docs/session-rules.md` keeps measured
  results in the experiment files and one short entry per session in
  the status file, and since session 76 its older entries are in
  `docs/history/`. Should the reading order point at the experiments,
  the roadmap's item lines and the newest handoff instead?

## Decided

(none yet)
