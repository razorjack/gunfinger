# 0060: the owner's 2003 mix, a second development set

**Question.** What does `identify` find in the owner's 46:10 mix of
2003-03-29, made from collection files (now a development set)?

**Command.** `gunfinger identify --config ~/.config/gunfinger/nas-dnb.toml
--store-only --playback both` (binary of b2d7962, store `d428ee9585936326`):
229 s, 1,567 s CPU, 3.9 GB. Data: `data/0060-razorjack-2003-03-29.json`.

| Start | Track (best file's hits, speed against the rips) |
|---|---|
| 0:01 | Kemal & Rob Data - Konspiracy (4,333, +6.7%) |
| 2:10 | Future Cut - The Specialist (6,872, the INFRA011 file tagged Sex Drive, +4.8%) |
| 6:12 | Narcosis - Escape Route (8,591, +4.7%) |
| 11:55 | Digital & Spirit - Phantom Force (2,233, +6.0%) |
| 14:40 | Tee Bee - Two Faced (2,712, +4.1%) |
| 17:23 | Omni Trio - Kinetic (8,633, +5.9%) |
| 24:24 | Carlito & Addiction - Into Music (2,363, +5.5%) |
| 29:13 | Omni Trio - Artificial Life (Harp Tune) (3,189, +7.0%) |
| 31:04 | Total Science - Yvon Is On (5,437, +5.8%; an upload 3,236) |
| 34:54 | Marcus Intalex & ST Files - Balderdash (5,567, an upload; Arena LP file 3,602) |
| 39:06 | Spirit - Out Of Control (2,767, the SPCC001 rip; Arena LP file 825) |
| 42:13 | DJ Hidden - Skektics (6,591, +4.3%) |

All confident, resampled playback, covering 0:01-46:03. Possible plays
sharing material with Phantom Force: Phantom 2018 (322 hits, 11:31),
Rift feat. Souldrop - Meltdown (98, key lock, 12:31), Fracture's
Astrophonica Edit (118, 13:46); Kinetic's first segment (199, 17:23).

**The owner's check by ear:** every track right. Phantom Force blends in
from 10:47, alone from 11:50 (68 s before its first confident detection);
Kinetic at 17:23 is a tease, brought in again later; Out Of Control was
most likely the Arena LP file (the SPCC001 vinyl is dated 2005).

**Conclusions.** 12/12, 0 wrong; the weakest track's best file has 2,233
hits. The best-scoring file is not always one that existed in 2003 (a
2005 rip, recent uploads), so hits do not show which file was played.
The manifest, drafted from this output and corrected by the owner, has
Gunfinger's boundaries as starts except track 4's; 5 tracks exist only on
the NAS (`reference = []` at 262 tracks, NAS paths as comments).
