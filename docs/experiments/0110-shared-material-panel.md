# 0110: the shared-material panel, from the judgements by ear

**Question.** Session 20, step 2 (roadmap, "A confidence statistic for
shared material"): which recording families do the judgements by ear
give, split by family, each direction's shared passage frozen before any
scenario is searched?

**Method.** `scripts/analysis/shared_panel.py`, no audio read: the 13
`different` verdicts, the 2 `shared` answers (35, 36), and two
judgements in the docs (Clockwork ~ its Stakka Rmx, 0026 and ADR 0019;
China Cup ~ The Nine, notes after session 4). Recordings are session
18's NAS clusters (read only); 57 unclustered files whose tags give a
family's titles were searched against it with `pair` (11 s), and 6
joined a recording by the clustering rule (80% coverage), mostly uploads
indexed since (`joined_in_session_20`). Order: the oracle committed
(0ae0c6d), the split (seed 2026, SHA-256 order of family names), then
`pair` on every judged pair both ways (30 directions, 325 s). 0076's
unjudged pairs (Killa Bees, Tsunami, Star Trails, The Nine's ChaseR
Bootleg, the Fire versions not judged against the original) are not
members; their files in a judged family are listed as other versions.

| Half | Families | Recordings | Pairs | Directions (controls) | Files |
|---|---|--:|--:|--:|--:|
| Development | dozer, fire, playground-shades, pyro, sonar, watermelon | 14 | 8 | 16 (2) | 24 |
| Validation | clockwork, fractles, mind-overload, night-gasp, t-minus, the-nine | 13 | 7 | 14 (4) | 42 |

**Passages** (`passage_rule`): the strongest alignment of `pair`'s audio
second pass (corpus played files) or stored peaks at the fitted speed,
else the key-locked ladder's. Development: 41-910 hits over 24-188 s
(Watermelon ~ Monotone 60 and 41, key-locked; Sonar 286 and 151; Fire
449-600; Pyro 910 and 412). Under 25 hits a direction is a control:
Playground Shades ~ Interlude and Night Gasp ~ Dark Soldier VIP (the
notes' chance spans, 6-9 hits), Fractles ~ We Enter (12 and 8, though
shared material was heard at a 40-hit NAS detection).

**Conclusions.** `docs/panels/shared-material-seed-2026.json` (the
panel) and `…-development.json` (`shared`'s plan, development half). The
draw put The Nine (0076's strongest case) and Clockwork in validation;
no validation query was rendered or searched.
