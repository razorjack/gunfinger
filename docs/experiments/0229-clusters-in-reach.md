# 0229: the NAS clusters' copies each ladder brings into reach

**Question** (`docs/brief-51.md`, 07:43, C5). Session 49 counted, with
the ladder taken as ±8.0%, the NAS clusters' copies that a play at a
deck's extreme leaves with no other copy in reach. How many does each
ladder's measured reach bring back?
**Method.** No search. `cluster_speeds.py` (the clusters file of
2026-10-25, 284 clusters; the copies' times from the joins) with the
reach as its fourth argument: each ladder's measured reach R_k from 0226
(8.4, 8.8, 9.2, 9.6, 10.4%), for a deck at ±5% and ±8%;
`work/s51/reach.py`; `data/0229-clusters-in-reach.json`.

| Reach | Deck ±5%: copies alone | clusters with a copy beyond | Deck ±8%: copies alone | clusters |
|---|--:|--:|--:|--:|
| 8.0% (session 49) | 38 | 50 | (degenerate) | |
| L0 8.4% | 29 | 37 | 88 | 93 |
| L1 8.8% | 19 | 23 | 78 | 88 |
| L2 9.2% | 7 | 12 | 74 | 82 |
| L3 9.6% | 3 | 6 | 69 | 79 |
| L5 10.4% | 1 | 1 | 57 | 70 |

"Copies alone": a copy that, played at the deck's extreme, has no other
copy of its recording within reach, so it is found only if it is itself
indexed. Each set is a subset of the one above. At ±8% with an 8.0%
reach every copy is "alone" by rounding (554), so session 49 gave no
figure there.
- Deck ±5%: of session 49's 38, L0's true reach already holds 9, L1 19,
  L2 31 (82%), L3 35 (92%), L5 37 (97%). Of the 29 left at L0, 26 are
  uploads of the one channel that runs 3-5% fast. At L3 three remain
  (two of that channel's uploads, Falcon - Reality and Kemal & Paul
  Reset - Kontempt, and Falcon - The Stand's "Speed Up" upload); at L5
  only the "Speed Up" upload (11.6% from the vinyl rips).
- Deck ±8%: of L0's 88, 58 are that channel's uploads played at +8%,
  15 other copies at +8% and 15 at -8%; L5 leaves 57 (48 of the channel's).
**Conclusion.** The measured edge (8.4%) already leaves fewer copies out
than session 49 counted; L2 brings 82% of its 38 back for a deck at ±5%
and L3 92%. For a deck at ±8% no ladder measured here covers the fast
channel's uploads.
