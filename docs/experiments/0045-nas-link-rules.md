# 0045: the link rules at NAS scale

**Question.** Item 12 of session 5: do the opt-in link rules (0030) keep
their effect at NAS scale?

**Command.** `work/s5/E [--second-pass --skip-fullest 0.01]
--nearby-rungs --strong-gaps scan stakka-skynet-knowledge`, and `sweep
--seed 2026` under skip at 240; one at a time, against 0039 and 0040.
Today's sweep with the rules did not fit. Data: `data/0045-nas-link-rules.json`.

| Development scan, 26,914 assets | Today's | with links | Skip at 240 | with links |
|---|--:|--:|--:|--:|
| Identified; wrong; possible plays matching no track | 11/11; 6; 3 | 11/11; 7; 3 | 11/11; 6; 3 | 11/11; 6; 3 |
| Weakest identifying | 1,839 | 1,492 | 1,557 | 1,539 |
| Detections matching no track; 30 hits or more; 60 or more | 413,989; 16; 9 | 81,694; 18; 10 | 8,704; 16; 9 | 2,981; 14; 9 |
| Strongest unrelated | 40 (Heretik - Biodome) | 36 (Biodome) | 48 (Genetix - Crunch) | 32 (Crunch) |
| Clockwork's Stakka remix (shared passage) | 94 | 91 | 119 | 94 |
| Wall; CPU; peak | 864.6 s; 6,097 s; 8.37 GB | 692.6 s; 5,963 s; 8.24 GB | 260.6 s; 1,963 s; 4.01 GB | 272.9 s; 1,963 s; 4.02 GB |

Under today's matcher the rules split Star Trails at 13:20, on the
corpus rip (1,492 and 333 hits) and on the alien5ive upload (1,806 and
321): the upload's second segment is the seventh wrong identification.
This is the cost session 4 named (a speed change of more than 0.6%
between windows). Under skip at 240 the play splits there too, but its
rest (18 hits) stays below every tier. Today's wall time falls by 20%
for 2% less CPU (inference: less single-threaded work on detections).

Sweep seed 2026 under skip at 240 with the rules: 540/540, 91 wrong
answers, weakest 680, all 91 confident false detections other uploads
with the same name, as without them (0040); false candidates 45,278 ->
22,561, 30 hits or more 186 -> 159; strongest false without the
same-name uploads 99 (Clockwork ~ its Stakka remix), 6.87x; unrelated
68 -> 64 (Clockwork ~ Simon Static - Rubba Rock). 1,351 s, 12,135 s
CPU, 4.77 GB (1,358 s, 12,153 s, 5.75 GB without).

**Conclusions.** (1) The rules keep removing chance lines at NAS scale
(80% and 66% fewer in the mix, half in the sweep) and lower unrelated
chance. (2) Under today's matcher they split one drifting play into a
wrong identification through an upload; under skip at 240 they change no
level and no answer, as at 262 tracks (0030).
