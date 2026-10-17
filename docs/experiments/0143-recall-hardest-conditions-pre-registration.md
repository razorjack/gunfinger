# 0143: recall under the hardest conditions combined: pre-registration

**Question.** Session 28, step 4 (the roadmap's "Recall beyond the
corpus recordings"): at NAS scale, how often does the recall panel's
development half name its own recording under other rips only, brief
plays and combined damage, alone and together, and why do misses fail?

**Panel and index.** `docs/panels/recall-seed-2026.json`, development
half: 58 indexed and 20 held-out sources (2 pruned, as 0074), 9 speeds
(-8..+8%). Corpus plus the NAS store at 40,126 records; held-out
clusters left out (0128's clusters, which agree with the six new
verdicts). Default matcher and rule (240 hits in 3 windows; possible 60).

**Conditions.** C: the 30 s excerpts. (a): also each indexed source's
own record left out; its cluster's other rips count as right; queries
with no other rip in the index (at least 36 of 58 sources) are
unobservable, scored like held-out ones, never misses. (b): 15 and 20 s
of the excerpt's start, after 3 + o s (o uniform in [0, 10), seeded) of
a held-out excerpt and before 3 s of another, cut together in the
decoded samples. (c): `robust`'s `combined` (wow 0.55 Hz, 0.2%; a
beatmatched partner at -6 dB, the next panel source of the same group,
credited apart; broadcast chain; AAC 64 kbps) at the panel's speeds.
(a)+(b)+(c); pairs where cheap: (a)+(c), (b)+(c), (a)+(b).

**Measures.** Confident and possible recall; wrong confident and
possible detections with their files; held-out and unobservable queries
with a confident answer; each miss's reason (never a candidate, too few
hits after the second pass, 240 hits in too few windows); every
confident wrong answer checked with `pair` (80% coverage = cluster gap,
still wrong).

**Expected.** C 522/522, no wrong; (a) at least 90%; (b) about the
grid's 42% (15 s) and 91% (20 s); (c) about `robust`'s 5% confident and
81% possible; (a)+(b)+(c) next to none confident.

**Notable, declared before any count.** Any confident wrong answer
(apart: under 80% `pair` coverage); any held-out or unobservable query
confident; (a) under 90% or misses concentrated in one kind; (b) more
than 10 points under the grid's rate; (c) over 15% confident or under
60% possible; any wrong possible detection of 200 hits or more.

Full text: `docs/brief-28.md`, step 4. **Result:** 0144 and up.
