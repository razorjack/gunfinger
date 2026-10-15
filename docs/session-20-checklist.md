# Session 20 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-20.md`
for scope and rules.

## Setup

- [x] `git status` clean at `2836414`
- [x] Brief, checklist; `CLAUDE.md` points to them
- [x] Release build in `work/bin/s20-start/`; `baseline session-20-start`
- [x] The NAS store at the start: 38,105 records, `b285da9984422fb5`, as
  expected

## Steps

- [x] 2a. 12 families from the 13 `different` pairs, the two `shared`
  answers and the two judgements in the docs; 6 unclustered rips joined
  by the clustering rule; 0076's unjudged pairs are candidates
- [x] 2b. Passages from `pair` (30 directions, a fixed rule; 6 controls
  under 25 hits)
- [x] 2c. Oracle committed first (0ae0c6d); split by family, seed 2026;
  `docs/panels/shared-material-seed-2026.json` and its development plan
- [x] 2d. Experiment 0110
- [x] 3. `shared` at NAS scale (`--other-peaks-dir`), with context
  before, a weak rip, a blend masking the context, a short break
  repeated; 10 new tests; the development half in one command (32 min);
  wrong confident in 106 of 218 main-case queries, 12 of 14 directions;
  experiment 0111
- [x] 4. Proposal 5's diagnostic on the development half
  (`gunfinger-eval identifiability`, 4 tests, 2.4 min): outcome 3 in 11
  of 14 directions (Dozer, Fire, Sonar, Pyro-1), outcome 1 in Pyro-2
  (thin, against a poor upload), Watermelon separated today; where the
  evidence falls and density under damage; no rule adopted; experiment
  0112. Follow-up written first: true plays in the development mixes
  (`--verify scan`) are as sparse as the related files; no cut on hits
  per second or the verifier's share keeps 95% of them and rejects half
  of the wrong plays; experiment 0114
- [x] 5. 16 listening questions in `work/audition/s20/` (pairs no
  verdict covers, from 0076 and the development mixes, one joined rip,
  Meltdown ~ Phantom Force); `listening_pack.py candidates`; the page
  checked with a temporary answer log and a copy of the verdicts (16
  questions, audio and envelopes 200, nothing answered)
- [x] 6. Proposal 3's first step: `take_part` split (a build never
  committed) on the 2003 mix at NAS scale, twice under `sample`, reports
  byte-identical: the binary searches 1.3-1.4% of CPU, the copying
  9.1%; under the 5% stop point, proposal 3 stops; worktree removed;
  experiment 0113
- [ ] 7. Read-only review by a subagent; fixes

## Wrap-up

- [ ] Notes "Session 20", status, roadmap ("A confidence statistic for
  shared material"), the research note (proposal 5 measured)
- [ ] `regress session-20-start` identical with the final build;
  `scripts/check.sh` green
- [ ] The NAS store's revision at the end
- [ ] `git diff docs/pair-verdicts.txt` empty and
  `work/audition/answers.jsonl` unchanged before every commit
- [ ] Commit; the final message
