# 0055: shared-material scenarios with two judged pairs

**Question.** Item 2 of session 7 (roadmap, "A confidence statistic for
shared material"): for pairs the owner judged different recordings, does
a shared passage alone, repeated, or reaching into distinctive material
make the related recording confident, with or without the played one
indexed? What does the verifier (0054, 1×1) measure there?

**Passages** (`gunfinger-eval pair`): Clockwork 205.4-224.1 s ~ its
Stakka remix 399.5-418.2 s (112-124 hits); China Cup 46.0-68.4 s ~ The
Nine 28.6-51.6 s at 1.024 (38 hits, the shared drums); The Nine
30.8-53.6 s ~ China Cup (29). Each direction is a scenario.

**Command.** `gunfinger-eval shared work/s7/shared-material.json` (commit
1c289ee): 262 tracks, default matcher, native speed. Queries: the
passage for 10 s and whole (18.7-22.8 s); looped to 60, 120, 240 s; to
10, 20, 30 s past its end. Indexes: everything; the played cluster left
out; the played file alone left out, another rip kept (not for the
remix). 96 searches; data `data/0055-shared-material.json`.

| Played → related | Related, played left out: hits, level; share (chance) | Played: hits; share |
|---|---|---|
| Clockwork → remix | 125-144, possible; 0.42-0.48 (0.01) | 659-1,845; 0.88-0.94 |
| Remix → Clockwork | 114-142, possible; 0.45-0.51 (0.01) | 879-1,072; 0.91-0.94 |
| China Cup → The Nine | 26-36, weak; 0.11-0.16 (0.02-0.04) | 685-1,756; 0.82-0.89 |
| The Nine → China Cup | 13-39, weak; 0.12-0.15 (0.00-0.01) | 651-1,785; 0.74-0.86 |

- Alone: 10 s gives no detection of two windows; the whole passage gives
  the played file 651-879 hits in 2 windows (possible) or 734-765 in 3.
- Looped: each repeat is a new alignment and starts a new chain, so the
  strongest related detection stays at 144 hits at 240 s (13-22
  segments). Summing a play's segments would make it confident (0011).
- Into distinctive material the played file gains about 40 hits per
  second; the related one does not move.
- With the played cluster left out nothing is confident in any query.
  Positive control: with the source alone left out, its other rip
  answers with the same hits and levels as with everything indexed.

**Conclusions.** Neither pair reaches the rule in any scenario at 262
tracks, and repetition does not raise it. In the shared passage the
verifier finds 0.42-0.51 (Clockwork) or 0.11-0.16 (China Cup ~ The Nine)
of the related record's peaks against 0.74-0.94 of the played one's: the
passages share material but are not the same audio. Support over time
saturates (0.92-1.00) for both.
