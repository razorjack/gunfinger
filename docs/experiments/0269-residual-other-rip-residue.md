# 0269: why another rip leaves A in the residual, and two fixes

**Question.** Brief 70, step 1 (pre-registered in `c3217e8`): with another rip of A as the reference and the true time map, A stayed confident after masking in 115 of 403 cases at NAS scale (0257). What differs from the cases where the mask removes A, and does *Align* (the reference shifted onto the mixture by measured lags) or *Align* with gains over 0.5 s remove it?
**Method.** Development seeds 2026 and 2028 (session 39's renders, copied), `gunfinger-eval --work work/s70/w-<scale>-<seed> --panels work/s65/panels --ladder turntable [--other-peaks-dir ~/.local/share/gunfinger/nas-dnb-peaks] mixes --kind drops --seed S --count 16 --other-root /mnt/atlas/Music/dnb --tagged work/s39/tags-262.json --residual --residual-timing truth,detected --residual-fix off,align,align-fine` (`work/bin/s70-run`; four runs side by side: 262 tracks 28 min, NAS scale 57 min, 14.6 GB peak). No fix reproduces 0255 and 0257 exactly. `scripts/analysis/residual_residue.py --diagnose --probe`.

Other rips at NAS scale, true time map (A confident after masking):

| Rip, by its stored peaks' line | Cases | No fix | *Align* | *Align*, fine gains |
|---|--:|--:|--:|--:|
| At speed 1 (within 20 ppm), corpus / NAS | 101 / 149 | 0 / 4 | 0 / 0 | 0 / 0 |
| At another speed, corpus | 31 | 20 | 0 | 0 |
| At another speed, NAS | 122 | 91 | 31 | 32 |
| All (0257's 403) | 403 | 115 | 31 | 32 |

- Where A stays (no fix) against where it goes, medians: the rip's line 7,510 (corpus) and 4,888 (NAS) hits against 26,448; its speed 1,673 and 1,987 ppm from 1 against 0; the reference's lag in the excerpt 12 and 62 ms against 0 (NAS: 67 of 95 over 20 ms; correlation of band levels at no lag 0.39 against 0.97 at the lag found); the gain fit's spread 2.8 and 7.6 dB against 0.3-0.9.
- The seek is not the cause: the render by seek and the decoded samples agree within 5 ms for both files in 90 of the 95 NAS cases and all 20 corpus ones. Lossy encoding is unlikely to be (an inference: the codec varies with the transfer): the reference is lossy under 256 kb/s in 94 of 95 cases where A stays and 170 of 176 where it goes; AAC at 127 kb/s is 40 of 95 and 42 of 176.
- After *Align*, the NAS rips A survives (31) are the ones off most: median lag 112 ms before the fix, gain spread 2.2 dB after it (1.65 where *Align* removes A).
- Quiet and brief Bs (64), confirmed / confident, NAS scale: no fix 50 / 32, *Align* 61 / 38, fine 61 / 36 (plain 40 / 8); 262 tracks: 62 / 47, 63 / 45, 63 / 43 (plain 41 / 8). Wrong confident in M and MH: 0 under every fix at both scales (MH: 768 searches, 64 cases at two scales under two timings and three fixes); wrong possible in M 4-8.

**Against the pre-registration.** *Align* reduces the cases where another rip leaves A confident from 115 to 31 (73%, the bar half), adds no wrong confident answer and confirms 11 more quiet and brief Bs: it counts. *Align* with finer gains leaves 32 (a difference under 5): **frozen fix *Align***.

**What it shows.** The residue is timing: a rip of another transfer runs 0.1-0.2% off in speed, and the line its stored peaks give lands it off in the excerpt (medians 12 ms for corpus rips, 28 ms for NAS files, 62 ms where A stays), where a 128 ms window's power no longer matches; the spectral mismatch mostly follows from the misalignment (the gain spread falls from 7 to 2 dB once aligned). An inference, not tested: the 31 left need a finer map (wow, or blocks under 0.3 correlation). Also: 250 of 0257's 403 "other rips" align with the played file at speed 1 with about 26,400 hits (by inference the same transfer, another encode), and there the mask works as with the played file. **Does not show** real vinyl mixes, or other masters played at other speeds. Data `data/0269-residual-residue-and-detected-timing.json`.
