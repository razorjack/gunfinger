# 0147: the minimum span on session 28's brief plays at NAS scale, and Panako's seven plays

**Question.** 0146's recall part: which of session 28's brief plays
(0144) does each rule name confidently, and does any add a wrong answer?

**Method.** The variant build (`583bfe3`) rebuilt (b) and (a)+(b)
(`recall --queries brief`, with `--other-rips-only`; 06:59-07:22, exit
0, `work/s29/vwork/`). Queries and today's detections equal session
28's: 1,404 queries each, 21,300 and 20,135 detections, 0 differences
(`span_recall.py same`); today's rule reproduces every stored level.
Counts `span_recall.py count`; Panako's plays `span_recall.py panako`
on session 27's mixes report (identical to master's). Data
`data/0147-minimum-span-brief-plays-nas.json`.

| Confident | Today | A(10) | A(15) | A(20) | B(10) | B(15) | B(20) |
|---|--:|--:|--:|--:|--:|--:|--:|
| (b) 15 s, of 522 | 228 | 519 | 5 | 0 | 519 | 504 | 0 |
| (b) 20 s, of 522 | 493 | 522 | 522 | 9 | 522 | 522 | 499 |
| (b) 15 s from 0-5 s into a window, of 259 | 0 | 258 | 3 | 0 | 258 | 253 | 0 |
| (a)+(b) 15 s; 20 s, of 180 | 37; 141 | 86; 149 | 3; 149 | 0; 3 | 86; 149 | 79; 149 | 0; 134 |
| Panako's 7 plays (0150) | 0 | 7 | 5 | 0 | 7 | 6 | 1 |

- No rule makes a wrong detection confident: in both conditions the 112
  wrong possible detections (strongest 146 hits) stay possible; no
  held-out (360) or unobservable (684) query is confident.
- Today's 15 s plays need to start 5-10 s into a window; under A(10) and
  B(10) the position no longer matters (the 3 misses have 219-230 hits).
  B(15) misses 18: 15 with 13-14 supported seconds (7 of them confident
  today) and the same 3. A(15) keeps almost none: a 15 s play spans under
  15 s. Under A(15) 293 detections of (b) fall to possible, A(20) 1,018.
- (a)+(b): other rips have half the hits (0145); A(10)'s 94 misses at
  15 s all have under 240.
- Panako's plays (20-26 s, 2 windows, 336-587 hits, spans 13.2-19.6 s):
  A(10), B(10) all 7; B(15) all but Aphrodite's Fanfare (14 supported s);
  A(15) also not Silent Witness's Again and Again (span 14.4 s); B(20)
  only Gridlok's Soul Sucker (20 supported s).

**Conclusion.** No variant adds a wrong or held-out answer here. A(10)
and B(10) make 15 s plays confident wherever they start (519 of 522, not
228) and all 7 plays only Panako named; B(15) nearly as many.
