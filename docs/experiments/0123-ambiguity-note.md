# 0123: the ambiguity note for shared material, measured

**Question.** Session 23, step 5: would "(shares this passage with:
Artist - Title)" on a tracklist line flag related versions' wrong
confident plays while leaving real true plays alone?

**Pre-registration** (`docs/brief-23.md`, `0afceb5` at 02:43, before any
count under the fresh clusters). Session 21's (c): a confident play is
flagged when each of its confident segments has a file outside its own
recording holding the segment's span with 60 hits or more in the run's
index. Worth proposing only if both hold, per play: at most 5% of the
development mixes' true plays and at least half of the rendered mixes'
wrong related plays (played recording left out) flagged.

**Counts.** Fresh clusters; 4c shrank (brief, 09:25), so criterion 2
comes from step 3's re-score of session 21's run (no value unknown).
`ambiguity_note.py`, `shared_mixes.py score`; `data/0123-*`.

| Criterion | Flagged | Required | |
|---|--:|--:|---|
| 1. True plays, 3 development mixes (step 4) | 15 of 108 (13.9%) | 5% at most | fails |
| 2. Wrong related plays, rendered mixes | 61 of 87 (70%) | half at least | passes |

- The 15 (one play per file) are 6 tracks; `pair` covers 3.7-54.4% of
  each with each file it names: no missed rip, no near-identical
  version. Remix or edit by title: Clockwork 3 (Stakka RMX), Star Trails
  3 (Synergy Remix), Phantom Force 2 (Astrophonica Edit, Phantom 2018),
  We R The Ones VIP 2 (Klute Remix). Other: Stab 2 Def 4 (Mampi Swift -
  2000, 22%; by ear the same music, another EQ and arrangement), Omni
  Trio's Artificial Life 1 (Secret Life's Silent Storm Remix).
- Beside, deciding nothing: the panel's main case flags every wrong play
  of 58 of 106 wrong confident queries (all Fire's), none in Dozer, Pyro,
  Sonar (no third version indexed), and all 75 right plays of the
  positive control. Rendered true plays 17 of 495 (104 unknown), 0118's
  18 of 292 (33). The exploratory (c)+(a1) cut rejects 0 of the 108.
- Chance reached 66-78 hits per excerpt at NAS scale (0040-0042); the
  lowest flagged values are 67-76 (Clockwork, Phantom Force), where
  `pair` aligns file and holder with 98-106 hits.
- Tracklists: TODO.

**Conclusion.** Criterion 1 fails, criterion 2 passes: by the rule
written first the note is not worth proposing. It marks true plays of
tracks whose remix, edit or close version is indexed, and only the wrong
plays with a third version indexed (Fire's). We decide.
