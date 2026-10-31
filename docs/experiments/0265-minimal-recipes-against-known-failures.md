# 0265: the minimal recipes against the roadmap's known failures

**Question** (brief 68). Do the minimal recipes of 0264 reproduce when
replayed, and is any class missing from the roadmap?
**Command.** `work/s68/replays.sh` (`work/bin/s68-replay/`, master at
`424185d`): `counterexamples --replay` on the first recipe of each class
picked (`work/s68/replay-picks.txt`), which renders and searches it again.
Data: `data/0265-counterexample-replays.json` (with each render's detections).

| Recipe (class; minimal transformations) | Replay | What the detections show | Known as |
|---|---|---|---|
| 262 0003 (gap: inverted channel) | holds | the target ends at 76.7 s, where the fault starts (77.0 s) | 0198, "Stereo faults before the downmix" |
| 262 0001, 0178 (gap: noise at -10 and 0 dB) | hold | the target ends 3.8 and 17.6 s after the fault starts | 0194, 0198 |
| 262 0013 (lost: a 46 s play under a partner at -1 dB) | holds | 200 hits in 5 windows (possible); the partner 1,424 | 0184, a record inside another |
| 262 0175 (gap: a blend of 49 s at -1.4 dB) | holds | the target ends 37.6 s before its play, under the partner | "Detection boundaries" (0020), here longer |
| 262 0007 (gap: a 1.0% ride, then a 48 s blend at -0.4 dB) | holds | before the ride 908 hits; after it, under the blend, 118 (one speed 1.0100) | 0175-0176 and 0184 together |
| 262 0039 (lost: a 0.5% ride and a 53 s blend at -4.3 dB) | holds | one detection at 1.0032 (190 hits), between the two speeds | one fitted speed per play (0024, 0032) and 0184 |
| 262 0064 (lost: a 2.3% ride at 14 s and a 5 ms delay) | holds | 492 hits in 2 windows before the ride; 218 after, under the fault | "rides inside short plays" (open) and 0198 |
| 262 0018 (gap: a -2.8% ride and noise at -10 dB) | holds | the ridden side (441 hits) ends 8 s into the fault | 0176 and 0198 |
| NAS 0002 (wrong: The Nine's Evol Intent VIP over The Nine) | holds | 430 and 327 hits, no transformation | the roadmap's shared material (877) |
| NAS 0000 (wrong: Cool Rock Shook Ones over Cool Rock Stuff) | holds | no transformation | the roadmap's verdicts (1245-1262) |
| NAS 0057 (wrong: Pyro - Ignorance over Time Is Broken) | holds | no transformation | related recordings (0038, 0148) |

- All 12 replays hold their failure and agree with the saved outcome
  (the first replay build compared floats exactly and called 4 of them
  different by one unit in the last place; fixed in `424185d`).
- Each replay took 3-7 s at 262 tracks and 14-21 s at NAS scale, most
  of it building or loading the index.
- By the registered reading no class is new: each minimal recipe is one
  known failure, or two known ones in one play (a ride's side or one
  fitted speed, made weaker by a blend or a fault).

**Conclusion.** Criterion (b) holds (known classes from minimal recipes
of one or two transformations, replayed), (a) does not (no new class).
The compounds are the pilot's only finding beyond the roadmap: a ride
of 0.5-1.0%, harmless alone (0176), loses its later side under a blend.
