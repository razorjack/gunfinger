# 0055: shared-material scenarios with two judged pairs

**Question.** Item 2 of session 7 (roadmap, "A confidence statistic for
shared material"): with pairs the owner judged different recordings
sharing material, does a shared passage alone, repeated, or reaching into
distinctive material make the related recording confident, with or
without the played recording in the index? And what does the peak
verifier (0054, 1×1) measure there?

**Passages** (`gunfinger-eval pair`, second pass on audio): Clockwork
205.4-224.1 s ~ its Stakka remix 399.5-418.2 s (speed 0.995-0.999,
112-124 hits); China Cup 46.0-68.4 s ~ The Nine 28.6-51.6 s (1.024, 38
hits; the shared drums), and The Nine 30.8-53.6 s ~ China Cup (29 hits).
Each direction is a scenario: the first file played, the second related.

**Command.** `gunfinger-eval shared work/s7/shared-material.json`
(commit 1c289ee), 262 tracks, default matcher, native speed. Queries:
the passage alone for 10 s and whole (18.7-22.8 s); looped to 60, 120,
240 s; from its start to 10, 20, 30 s past its end. Indexes: everything;
the played file's cluster left out; the played file alone left out with
another rip kept (not for the remix, which has none). 96 searches; data
in `data/0055-shared-material.json` (`shared_summary.py`).

| Scenario | Related, played left out: hits; share (chance) | Played, everything: hits; share |
|---|---|---|
| Clockwork → remix | 125-144, possible, 2-3 windows; 0.42-0.48 (0.01) | 659-1,845; 0.88-0.94 |
| Remix → Clockwork | 114-142, possible; 0.45-0.51 (0.01) | 879-1,072; 0.91-0.94 |
| China Cup → The Nine | 26-36, weak; 0.11-0.16 (0.02-0.04) | 685-1,756; 0.82-0.89 |
| The Nine → China Cup | 13-39, weak; 0.12-0.15 (0.00-0.01) | 651-1,785; 0.74-0.86 |

- **Alone.** 10 s gives no detection of two windows. The whole passage
  gives the played recording 651-879 hits in 2 windows (possible) and
  734-765 in 3 (confident); the related one stays as in the table.
- **Looped to 240 s.** Repetition does not add up: each repeat is a new
  alignment, so it starts a new chain. The strongest related detection
  stays at 144 hits (3 windows), and the played file's at 905-1,048. The
  related recording is found once per repeat (13-22 segments); a rule
  summing a play's segments would make it confident (0011 found summing
  unsafe for this reason).
- **Into distinctive material.** The played file gains about 40 hits per
  second (1,056 → 1,845 for Clockwork); the related one does not move.
- **Played left out.** No confident detection in any query; Clockwork's
  remix (either way) is possible, China Cup and The Nine weak.
- **Positive control.** With the source file left out, its other rip
  answers with the same hits and levels as with everything indexed.

**Conclusions.** At 262 tracks neither pair reaches the rule in any
scenario, and repetition alone does not raise it. Inside the shared
passage the verifier finds 0.42-0.51 of the related record's peaks
(Clockwork) or 0.11-0.16 (China Cup ~ The Nine), against 0.74-0.94 for
the played record: the passages share material but are not the same
audio. Support over time saturates (0.92-1.00) for both and does not
help here.
