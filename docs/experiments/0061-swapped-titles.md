# 0061: two releases with swapped titles, checked against the audio

**Question.** The owner's research (ChatGPT, not verified here) says two
records circulate under swapped titles: Future Cut's INFRA012, with The
Specialist as "Sex Drive", and Bad Company's DSCI4 Coma / Spraycan,
whose label swaps the sides (Discogs: Coma 6:18, Spraycan 6:34). What
do the collection's files hold?

**Command.** Future Cut: the NAS clusters of session 6 (store
`d428ee9585936326`). Bad Company: `gunfinger-eval --other-peaks-dir <NAS
store> pair data/0061-coma-spraycan-pairs.txt` (binary of experiment
0060; 54 s). Data: `data/0061-coma-spraycan-pairs.json`.

**Future Cut.** In the scene release `Infrared-INFRA011-Future_Cut-2000-sour/`,
`a-future_cut-sex_drive-sour.mp3` is in The Specialist's cluster (both
TrT INFRA012 rips, the corpus file, the i-witness upload), and
`aa-future_cut-specialist-sour.mp3` is in Razor's Edge's. Neither joins
`dfect-dnb/Moving Fusion - Sexdrive.m4a`. Sonic & 2D's Sex Drive is not
in the library.

**Bad Company.**

| File | Length | Shares audio with |
|---|--:|---|
| `(2000) Coma & Spraycan (DSCI4)/01 Spraycan.mp3` | 6:16 | nothing (at most 6 hits either way) |
| `(2000) Coma & Spraycan (DSCI4)/02 Coma.mp3` (corpus `extra/02 Coma.mp3`) | 6:31 | the upload: 0:00-3:49 (2,318 hits), then pieces |
| `i-witness-dnb/Bad Company - Spraycan.opus` | 6:25 at +1.7% | `02 Coma.mp3` |

The owner hears `02 Coma.mp3` and the upload as one recording
(2026-10-09).

**Conclusions.** The audio settles what the titles do not. The sour
release's "Sex Drive" is The Specialist and its "Specialist" is Razor's
Edge. By length alone (not yet checked by ear), the DSCI4 folder's
`01 Spraycan.mp3` is Coma and `02 Coma.mp3` is Spraycan, as the upload
is titled. `identify` names a play after its best file's tags, so it
prints the wrong title whenever such a file scores best: track 2 of the
2003 mix appears as "Future Cut - Sex Drive" (6,872 hits; the INFRA 012
upload is a separate play with 2,475). No manifest references these files.
