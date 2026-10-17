# 0142: the shared panels' short passages after silence

**Question.** Session 27, pre-registered at 03:01 after the review found
0141's lead-in matched by fire-2's and fire-6's wrong versions: with
digital silence before each short passage, does today's rule give the
answers the variants give on the panels' 20 s files?

**Method.** As 0141, with `S27_SILENCE_ONLY` (the tree of `812342d` on
`s27-span`, never merged; `--work work/s27/vsilence`): up to 20 s of
each passage after 3, 5 and 7 s of silence, every scenario (48 and 30
queries, 117 and 78 results), 03:01-03:09. Same scripts and readout;
0 reproduction problems. Data `data/0142-*.json`.

| Silence-led queries | Today | A(10) | A(15) | A(20) | B(10) | B(15) | B(20) |
|---|--:|--:|--:|--:|--:|--:|--:|
| Shared 1, played left out: wrong of 42 | 10 | 11 | 10 | 0 | 11 | 10 | 8 |
| Shared 2, played left out: wrong of 24 | 0 | 0 | 0 | 0 | 0 | 0 | 0 |
| Short file under the rule = today's rule silence-led (of 117; 78) | 44; 29 | 111; 70 | 112; 71 | 44; 29 | 111; 70 | 112; 70 | 111; 68 |

- 0140's six wrong answers: today's rule names the same wrong file
  confidently in all 3 silence-led queries of each, 18 of 18, from the
  passage's start, with hits like the 20 s files' (fire-2 635-709 and
  fire-6 675-684 against 642 and 703; pyro-2 293-303 against 270).
- Today's rule is also wrong where the 20 s file at the grid's origin
  never shows it: pyro-1 after 5 s names Pyro's Ignorance (248 hits, 3
  windows). A(10) and B(10) add one wrong answer against today's rule,
  pyro-1 after 3 s (Ignorance, 250 hits in 2 windows, span 12.5 s), in 3
  indexes; A(15) and B(15) none. A(20) leaves no passage of 20 s or less
  confident (73 and 46 results change).
- Where A(10) on the 20 s file and today's rule after silence differ:
  pyro-1 (3 pairs), 14-15 s passages still in two windows (3; shared 2:
  5 with 3 more near 60 hits).

**Conclusion.** Prediction held. After silence, today's rule names the
same three wrong versions every time, and one more; on the panel only
the 20 s files' two windows keep them possible. On these queries today's
rule is wrong as often as A(15) and B(15), once less than A(10) and
B(10). A fair test of a span rule counts brief passages at several
offsets. Criterion 1 still fails as written.
