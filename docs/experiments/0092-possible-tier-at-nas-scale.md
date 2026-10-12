# 0092: the possible tier at NAS scale, from stored reports

**Question.** Step 4 of session 15: at 60, 80, 100 and 130 hits, possible
plays per hour, of what, and which true plays would a higher threshold
hide? **Method.** `possible_tier.py` on the NAS development reports (no
new runs; 0090's clusters): a play below the confident rule whose
strongest segment reaches the threshold is right (its slot or query
accepts it), related (a `different` verdict or a 30-hit pair between the
clusters, else tags: same name or version) or unrelated; also 262-track
robust, grid, mixes. `data/0092-possible-tier-at-nas-scale.json`.

| Threshold (hits) | 60 | 80 | 100 | 130 |
|---|--:|--:|--:|--:|
| Development mixes (1.71 h): possible plays per hour | 3.5 | 2.9 | 1.2 | 0.6 |
| of which related; unrelated by measurement | 5; 1 | 4; 1 | 2; 0 | 1; 0 |
| Sweeps and recall (29.9 h of excerpts): per hour | 12.3 | 10.4 | 8.6 | 6.7 |
| of which right (weaker copies); related; unrelated | 196; 171; 1 | 190; 120; 0 | 180; 78; 0 | 168; 32; 0 |
| True plays found only as possible, NAS (mixes, sweeps, recall) | 0 | 0 | 0 | 0 |
| Robust (262): of 278 found only as possible, hidden | 0 | 48 | 76 | 128 |
| Grid's brief plays (255) and mixes' plays of 20-26 s (9), hidden | 0 | 0 | 0 | 0 |

The mixes' possible plays: Star Trails' Synergy Remix (79), the Clockwork
Stakka remix twice (94), Phantom 2018 (146) and Fracture's edit of
Phantom Force (118), all related; and Rift feat. Souldrop - Meltdown, 98
hits in 5 windows, key-locked, over Phantom Force solo (12:31-13:15).
`pair` finds at most 14 hits between Meltdown and either Phantom Force
file, but it tries only turntable and key-locked relations; this one
implies Phantom Force at +6% pitch, +2% tempo in Meltdown (a sample?).

**Strongest unrelated** (0052, 26,969 assets: sweep 64, scan 32): sweeps
64 (Clockwork ~ Rubba Rock again), stakka scan 32, audio not in the index
36; the 2003 mix (not in 0052) 98 if Meltdown is chance. Brief true plays
are possible by their windows, not hits (162-754).

**Conclusions.** At NAS scale the tier shows 3.5 plays per hour of real
mix, 5 of 6 related versions; no development true play depends on it. A
higher threshold costs only damaged excerpts (robust: 17-46% of its
possible-only true plays at 80-130). ADR 0006's rule asks for twice the
strongest unrelated: 130 if Meltdown shares Phantom Force material, 196
if it is chance (near the rule of 240; ADR 0006 drops the tier at it).
