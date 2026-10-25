# The edge of the ladder: what a decision would rest on (2026-10-25)

**Status: the evidence and a recommendation.** Nothing here changes the
product. Whether to widen the ladder, search extra rungs only where
nothing confident was found, or leave it, is our decision and needs an
ADR. Session 51's pre-registration (`docs/brief-51.md`, 07:43) and
experiments 0226-0230; earlier 0033 and 0059; the roadmap's "The edge
of the ladder".

## What limits the reach today

`identify` searches 82 rungs: turntable speeds 0.92-1.08, 0.4% apart,
and the same ratios under key lock. A play is found when the mix's
speed relative to an indexed copy of the record lies within about 8.4%:
recall is complete there against the full index (552 of 562 against
other copies only, the misses weak copies) and gone by 9.2% (0226, at
both scales and against the copies the NAS library holds). The limit is relative to the
copy, not the deck's pitch: copies of one recording differ in speed by
up to 11.6% in the NAS clusters, and one YouTube channel's uploads run
3-5% fast against the vinyl rips. For a deck at ±5%, 29 copies have no
other copy within 8.4% (0229); session 49 counted 38 with the ladder
taken as ±8.0%.

## The options, measured

`--extra-rungs k` adds k rungs past each end of both ladders (82 + 4k).
All figures from session 51 (seed 2026; NAS scale unless said):

| | L0 | L1 | L2 | L3 | L5 |
|---|--:|--:|--:|--:|--:|
| Rungs | 82 | 86 | 90 | 94 | 102 |
| Reach (≥ 95% recall, 262 tracks) | 8.4% | 8.8% | 9.2% | 9.6% | 10.4% |
| Other copies only, [8.4%, R_k + 0.4%) | – | 62% → 96% | 42% → 95% | 32% → 95% | 21% → 96% |
| Other copies only, within 8.4% (of 562) | 552 | 551 | 551 | 551 | 551 |
| New wrong confident answers | – | 0 | 1 | 1 | 1 |
| The same, seed 2027's other copies (0230) | – | 0 | (not run) | 1 | 5 |
| Wrong possible answers, other copies only | 24 | 29 | 34 | 38 | 47 |
| Copies alone, deck ±5% (of session 49's 38) | 29 | 19 | 7 | 3 | 1 |
| Process CPU, Stakka scan | 667 s | ×1.046 | ×1.094 | ×1.141 | ×1.229 |

The development scans keep their plays under every ladder, and
calibrate's strongest false candidate and weakest identifying detection
do not move at either scale (0227). The extra cost is the share of rungs
added; sessions 36-46 made the search cheaper as a whole, not the extra
rungs relatively (0228).

The targeted variant T_k (extra rungs only where the default finds
nothing confident) was estimated, not built: per query it keeps the
default's answer when that has anything confident. On the Stakka mix
2.0% of the time has no confident detection, so its cost would be about
×1.001 (T1) to ×1.005 (T5) plus the cost of finding those windows. Its
recall equals L_k's within three queries; it keeps the Siren Bass
answer below, and gives 14-30% fewer wrong possible answers than L_k.

## What stands against each

Two single cases decide the registered criteria:

- **A fit pulled off by the outer rung.** One other-rips query (Siren
  Bass's upload played at -8%, relative -8.05%) is confident under L0
  at a fitted -8.09% with 305 hits and not under any wider ladder, which
  fits it at -8.18% with 214 (rule 240), identically under L1-L5. By
  inference, not measured: the -8.4% rung pulls the chain's fitted speed
  off the true one, and the second pass counts fewer hits. The
  registration asked that no ladder lose a query within
  8.4%, so every L_k fails there; T_k keeps L0's answer.
- **A known bootleg reached at one more speed.** The default already
  answers a held-out excerpt (Aphrodite - Cool Rock Stuff) confidently
  with a YouTube bootleg that holds it at 7 of the 9 sweep speeds; the
  bootleg runs about 4.3% slow. At +5% that pair is 9.7% apart, so L2 and up add an 8th
  (0227). By ear the bootleg is a different mix of the same track
  (`docs/pair-verdicts.txt`), so this is a real wrong answer. T2 and up
  inherit it, because the default has nothing confident on that query.
  Seed 2027 shows the same pattern (0230): China Cup against ƆƐI3C's
  upload, judged different (the hi-hats differ for a few bars), which
  the default answers wrongly 15 times; L3 adds 1 answer, L5 5 (L3's
  among them).

## Risks

- Wider reach also reaches versions, remixes and bootlegs that run at
  other speeds: false candidates below the rule rise by 3-11% and
  wrong possible answers by about 5 per rung pair (24, 29, 34, 38, 47
  against other copies, seed 2026). Possible plays
  are printed, so the user sees more of them in fast or slow passages.
- A targeted search depends on what "no confident line" means at the
  window level, which the estimate does not settle: in a real mix it
  runs in transitions, blends and unknown tracks, exactly where shared
  material lives. Its wrong answers in mixes are not measured.
- The registered runs use one panel (seed 2026); seeds 2027 and 2028
  repeat the other-copies condition only (0230: the gains hold, no loss
  within 8.4%, one rung pair gains 27-35 points there against 33). The
  NAS library is as it is today, and one channel's fast uploads make up
  most of the copies at risk.
- Every new wrong confident answer seen (two pairs, 6 answers) is a
  version judged different by ear that the default already answers
  wrongly at other speeds: wider reach adds speeds at which known
  confusions happen, not new confusions. That is still a wrong answer
  under the brief's first rule.

## Recommendation

By the registered criteria no wider ladder is worth an ADR: each fails
the within-reach clause by the Siren Bass query, and L2-L5 also add the
bootleg answer (L3 and L5 China Cup's too, 0230). The one variant that
passes is T1, the targeted search with one extra rung pair, as a
proposal to build and measure: it brings 19 of the 38 copies back for a
deck at ±5% and its cost is negligible by estimate, but its gain is the
smallest (27-35 points beyond 8.4% on the other panels). Our
recommendation:

1. Keep the default ladder; no ADR now.
2. If the copies at risk matter enough to spend a session, build T1 as
   an opt-in variant (extra rungs searched for the windows without a
   confident line) and measure it on the development mixes and
   generated mixes, where its wrong answers in uncovered windows can be
   seen, before any ADR. T2 or T3 would bring 31-35 of the 38 copies
   back, but the registration rules them out by real wrong answers
   (versions judged different), and precision comes first; choosing them
   anyway would be our judgement against the registered bar.
3. Separately, the Siren Bass case points at the second pass's fitted
   speed near the ladder's end (session 50 saw the same rip lose
   second-pass hits through a shorter chain). It is worth a look before
   any ladder change, since a wider ladder moves the end where it sits.
