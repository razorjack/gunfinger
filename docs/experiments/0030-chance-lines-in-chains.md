# 0030: chance lines in chains, two opt-in link rules

**Question.** Item 3 of session 4: a chain links the next window's line
on any rung if it agrees on the reference time, so a 3-hit chance line can
add a window (0020). Do link rules remove these without losing plays?

**Change** (commit afc731c, opt-in). `--nearby-rungs`: linked lines lie
within 1.5 steps (0.6%) of speed. `--strong-gaps`: across an empty
window, both lines have 10 hits or more. Measured together, under today's
matcher and skip at 240, with the full protocol (`s4-protocol.sh`).

**Clockwork remix, 20:22-20:40** (`explain --windows`). The chain's third
window is a 3-hit key-locked line at -8.00% beside lines at +1.2-1.6%:
105 hits in 3 windows. With `--nearby-rungs` 102 hits in 2 windows;
`--strong-gaps` alone changes nothing (the windows are adjacent).

| | Today's | with rules | Skip at 240 | with rules |
|---|--:|--:|--:|--:|
| Sweeps 2026-2029; weakest identifying | 2,160/2,160; 403 | same | 2,160/2,160; 658 | same |
| Strongest false; margin | 97; 4.15× | 97; 4.15× | 119; 5.53× | 99; 6.65× |
| False candidates; 30 hits or more | 70,506; 55 | 22,646; 51 | 4,503; 41 | 2,086; 46 |
| Development 11/11: detections; strongest false | 6,114; 94 | 1,497; 91 | 232; 119 | 87; 94 |
| Mixes: confident, possible; overshoot | 107, 8; 86.0 s | 106, 9; 7.6 s | 106, 9; 7.6 s | same |
| Mixes: true coverage missed | 606.3 s | 628.5 s | 616.4 s | 615.3 s |
| Grid confident at 10, 15, 20 s | 8, 76, 148 | 0, 67, 147 | 0, 67, 146 | same |
| Grid mean span at 10, 20 s | 11.4, 21.1 s | 9.7, 19.7 s | 9.7, 19.7 s | same |

Leave-outs 3 and 11: 0 wrong throughout. The lost mix play (mix 10,
Dominion, 21.5 s) was confident only through a chance line 23 s earlier;
with the rules it is possible, 537 hits in 2 windows, as under skip. Star
Trails' play (1,839 hits, 36 windows) splits where the DJ moved the speed
from +4.5% to +3.1%: 1,492 and 333 hits, both confident, one play with two
segments in the report. Data: `data/0030-chance-lines-in-chains.json`.

**Conclusions.** (1) Under today's matcher the rules remove 68% of false
candidates and 91% of the boundary overshoot, giving what skip at 240
already gives; the grid's 10 s plays are no longer confident through
chance windows. (2) Under skip they raise the margin from 5.53× to 6.65×
(the remix's 119 hits in 5 windows become 94 in 3) and change nothing
else; split chains add 5 false candidates of 30 hits or more.
