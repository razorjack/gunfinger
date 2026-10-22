# 0183: marked plays, a record playing or shared material

**Question.** Exploratory, not registered: 0182's marked plays (possible plays inside a passage of a confident play of another file, left out of the tracklist by ADR 0019) are either a record playing under its partner (the mark's cost) or material of a related version (its purpose). Does anything the report holds tell them apart: the play's best segment's hits, its length, or its hits against the partner's best segment?
**Method.** `python3 -I scripts/analysis/tracklist_marked.py` on 0182's saved reports (no search); each marked play labelled by the truth. Counts are plays, so a record found on two rips counts twice.

| Marked plays (262 tracks \| NAS) | n | Best segment hits, median (range) | Seconds at 262 tracks, median (range) | 40 s or more |
|---|--:|--:|--:|--:|
| Playing: brief, inside | 19 \| 32 | 150 (66-233) \| 104 | 27 (17-37) | 0 \| 0 |
| Playing: quiet, inside | 10 \| 14 | 110 (60-207) \| 90 | 25 (12-55) | 1 \| 2 |
| Playing: unrelated drop, inside | 6 \| 12 | 140 (102-186) \| 146 | 63 (42-71) | 6 \| 11 |
| Playing: pair drop (the original inside the remix) | 6 \| 13 | 153 (120-202) \| 148 | 98 (41-153) | 6 \| 13 |
| A related version playing (the Clockwork pair) | 64 \| 101 | 132 (71-155) \| 130 | 19 (13-175) | 19 \| 23 |
| Neither (NAS; by name mostly versions) | 0 \| 9 | 75 (69-108) | NAS: 30 (24-99) | 0 \| 1 |

- Hits overlap: 150 hits or more holds 18 of 41 playing plays at 262 tracks and 4 of the 64 related ones. The ratio to the partner's best segment lies at 0.012-0.069 for playing plays and 0.011-0.037 for related ones at 262 tracks (0.009-0.054 at NAS scale).
- Length separates the drops best: every drop partner left out (12 at 262 tracks) lasts 41 s or more, while the related plays have a median of 19 s (the panel's passage is 19 s); but 19 of the 64 related plays also last 40 s or more (the original found over long stretches of its remix played alone).
- The original inside its remix and the original's material inside the remix played alone are the same relation of audio; only the strength of the evidence differs, and it overlaps (120-202 against 71-155 hits).

**What it shows.** No single measure in the report separates a played record from shared material here. A length bound (a marked play of 40 s or more stays in the tracklist) would print every drop partner of these mixes and 19 of 64 shared-material lines at 262 tracks. In the development mixes' saved reports it would print 2 of the 5 marked plays at NAS scale (session 36's `identify` reports: Mampi Swift - 2000 for 116 s and a Digital & Spirit upload for 125 s, both shared material by 0102 and 0107) and none at 262 tracks (session 35's: the Clockwork remix, 18 s). Brief records would stay out, and quiet ones but 1 at 262 tracks (2 at NAS scale). **Does not show** the balance on real mixes, which hold few drops with ground truth; any change is ours to decide with an ADR (ADR 0019's decision 2).
