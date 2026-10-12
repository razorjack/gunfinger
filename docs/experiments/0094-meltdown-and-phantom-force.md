# 0094: is Meltdown's 98-hit alignment shared material or chance?

**Question.** Follow-up to 0092: the possible-tier threshold hinges on
whether Rift feat. Souldrop - Meltdown (98 hits, key-locked +4%, during
Phantom Force in the 2003 mix) shares material with Phantom Force. The
mix plays the PHUD1 rip at 1.0452 (turntable), so a shared passage would
be Phantom Force pitched up 4.5% at +0.5% tempo inside Meltdown, which a
key-locked or turntable search cannot test. Resampling Meltdown by
1/1.0452 turns that into a key-locked relation of -3.85%, which it can.
**Method.** `gunfinger explain --store-only --playback both --asset`
(NAS store, every candidate listed, weak ones included) on excerpts cut
and resampled with ffmpeg (`work/s15/meltdown/`). Measurement only.

| Query (PF: Phantom Force) | Asset searched | Best candidate (KL: key-locked) |
|---|---|--:|
| The mix around 12:53 | Meltdown | 100 hits, 5 windows, KL +4.02%, track 0:56-1:42 |
| The mix, 12:31-13:16 alone | Meltdown | 95, 5, KL +4.05%, track 0:44-1:25 |
| PF rip 2:36-3:18 sped up to 1.0452, as the mix plays it | Meltdown | 53, 4, KL +3.60% |
| Meltdown 0:57-1:42 | phantom | 6 (Remote Control) |
| The same at 1/1.0452 | phantom | no Phantom Force; Phantom 2018 43, KL -2.62% |
| Control: PF rip shifted +4.52% pitch, +0.5% tempo (rubberband) | phantom | 43, KL +3.60% |
| Control, then at 1/1.0452 | phantom | PF rip 1,292, KL -3.85%; upload 102 |

**What it shows.** The method finds a re-pitched copy of the passage
(1,292 hits); Meltdown gives Phantom Force nothing, so Meltdown does not
hold that passage re-pitched. Phantom Force itself, without the mix,
aligns with Meltdown at 53 hits on the same relation, and the mix's
alignment lands 12 s apart in Meltdown in two searches (8 bars at about
174 BPM): a pattern that repeats in Meltdown, such as a drum break, or a
rhythmic coincidence. **Does not show** which; one passage, one pair.
**To judge by ear**: do they share a break or loop? If yes, the
strongest unrelated chance stays 64 (threshold 130); if no, 98 (196).
Data `data/0094-meltdown-and-phantom-force.json`.

**By ear (2026-10-12, after the session).** Different tracks,
similar in character, sharing the full drum break: shared material, not
chance. The strongest unrelated candidate stays Clockwork ~ Rubba Rock
(64 hits), now itself to judge by ear.
