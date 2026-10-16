# 0136: a minimum span or supported time instead of 3 windows: pre-registration

**Question.** Session 27, step 2 (the roadmap's "A minimum span instead
of 3 windows"): under today's matcher (ADR 0008), does replacing the
confident rule's `windows >= 3` with a minimum span or a minimum
supported time make more brief plays confident and fewer levels depend
on the window grid, without a new wrong confident answer on any panel?

**Today's rule** (`confidence.rs`, `FITTED_RULE`): a segment is
confident at 240 hits in 3 windows (10 s windows of query time, each
holding a densest run of 3 hits or more), possible at 60 hits; a play
takes its best segment's level (ADR 0013). **The variants** replace
`windows >= 3` only, the hits and the possible tier unchanged, so plays
and segments stay the same and only levels between possible and
confident can move:

- A(S): `end - start >= S`, the first to the last aligned hit, S = 10,
  15, 20 s. A(10) cannot demote (3 windows span more than 10 s).
- B(S): `supported >= S`, distinct whole seconds after the first hit
  holding an aligned hit (`pair`'s supported time, 0057), S = 10, 15,
  20 s.

**Panels.** Grid (960 queries), sweeps 2026-2029, mixes seeds 2026 and
7, `robust`, the three development mixes at 262 tracks and at NAS scale
with leave-outs, the recall panel's development half, both shared
panels' development halves. A is counted offline from master's saved
reports (M) where they hold the first and last hit, after today's rule
reproduces their stored levels and scores exactly; A and B from one run
of each panel with a variant build in a never-merged worktree (V), which
only adds supported time (and first and last hits where missing) to its
reports and writes to `work/s27/vwork/`. V's detections must equal M's
where both exist at the same library state.

**Criteria, both required.** (1) No new wrong confident answer on any
panel (shared panels first; a wrong possible answer turning confident
counts, also beside an existing wrong title). (2) On the grid, strictly
more confident queries and strictly fewer flips ((brief, length) pairs
whose verdict changes with the offset alone) than today. Beside: level
changes of true plays, the shortest identification, per-second measures.
Full text: `docs/brief-27.md`, step 2. **Result:** 0137 and up.
