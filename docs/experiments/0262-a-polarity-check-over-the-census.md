# 0262: a polarity check at indexing, over a larger census of the NAS rips

**Question.** Brief 67, pre-registered (`faeadee`): how many NAS files
would the check repair (lag-0 correlation -0.5 or less, strongest within
±80 samples at lag 0, channels within 20 dB), and how many may be wide
recordings rather than faults? (Harness only: `stereo-width`'s `repaired`.)

**Command.** `stereo-width --other --seed 2026 --sample 6000 --exclude
work/s62/census-exclude.txt` (16 jobs, alone, read only); `polarity_census.py
REPORT OUT 0248-REPORT` (0248's files judged by the rule from their measures).
Data: `data/0262-a-polarity-check-over-the-census.json`. **A departure:**
session 64 pruned 4 uploads, so the draw is not the superset of 0248's
that was registered: it holds 1,234 of its 2,000 files (all 667 scene
releases), not Keltech - Axis or Plasmic - The Weapon. Decided after
the census (brief 67, 02:28): estimates and F use the union (6,766).

| Repaired (weighted to 40,659 files) | Scene | Uploads | Elsewhere | Share (95%) |
|---|--:|--:|--:|--:|
| Registered census (2,000 per kind) | 2 | 0 | 1 | 0.01% (0-0.21%) |
| Union with 0248 (2,000; 2,634; 2,132) | 2 | 2 | 1 | 0.08% (0.02-0.28%), about 31 |
| Union: plainly inverted (side 10 dB or more above the mid) | 1 | 2 | 1 | about 29 |

The five: Paradox - Filmscape (new; -0.99, side 22.0 dB above the mid),
Usual Suspects - Advent (-0.95, 15.9), Plasmic - The Weapon (-0.94,
14.9), Keltech - Axis (-0.92, 13.0): plainly inverted; We - Hello World
(-0.76, 8.6 dB): uncertain (unsure by ear too). The next lowest in 6,766
files are -0.32 (the 5 ms delayed `sety/` file) and -0.27 (a wide We
track); DJ Sense (+0.62 at lag 0) is not repaired.
**Partly inverted** (the 6,000 only; 0248's report has no windows). 29
unflagged files have a 30 s window at -0.5 or less; none has more than
29% of its windows there; 26 correlate at +0.34 to +0.98 as a whole.
Three rips of Bad Company - Refuge share the same 2 of 16 windows: by
inference stereo effects of the recording.
**Cost.** 6,000 files (642 h of audio) in 9:43 at 16 jobs over Ethernet,
617 a minute: all 40,674 once in about 66 minutes (extrapolated).
**Conclusion.** The check would change about 0.08% of the store (about
31 records, at most 0.28%); 0248's 0.4% rested on 3 uploads in 667. Bar
item 5: 2 of 3 plainly inverted on the registered census (fails), 4 of 5
on the union (holds).
