# 0178: overlap judged on chains after the second pass (opt-in)

**Question.** Session 37, step 4: does judging overlap on the first-pass
chains, instead of the second pass's widened spans (0175), keep both
sides of a ride without a new wrong confident answer?

**Change** (`8899de4`, opt-in `--overlap-by-chains`; off by default,
`regress` identical when off). After the second pass, overlap is judged
on first-pass detections, which the first suppression already left apart,
so the second suppression is skipped: it only keeps what the default drops. Criteria (brief 05:20): V1
90% of lost sides of 30 s+, and of their seconds, on new seeds 3741-3742;
V2 no rise in wrong confident detections and no new wrong one; V3 no
right answer lost.

| Run (default -> variant) | Result |
|---|---|
| Rides 3741-3742: sides lost (no detection; 30 s+ uncovered) | 183 -> 0 (12,606 of 12,609 s recovered); 225 -> 2 |
| Rides 3741-3742: wrong confident; right plays | 0 -> 0; 381 -> 381 |
| Sweep 2026, 262 tracks; NAS scale | identical; 4 of 720 gain a weak detection (6-15 hits), recall 540, wrong 7 -> 7 |
| Stakka and 2 leave-outs, both scales | wrong 0 -> 0, identified same; Star Trails' 13:13 side added (306-315 hits) |
| 8 other development reports, NAS scale | wrong 0 -> 0, right same; Into Music and Phantom Force segments added (right) |
| Generated mixes 2026: short; long | identical; 42 weak or possible added, wrong 0 -> 0, right 191 -> 191 |
| First shared panel, development half, NAS scale (622 queries) | no file newly confident in any query; 56 confident detections added: 52 of the played recording, 4 of a wrong version already named |
| CPU: `regress` 262 tracks; NAS scale; rides searches | +0.2%; +0.4%; -0.3% to -0.4% (0177) |

- Not criteria: false candidates of 30 hits+ 18 -> 19 at 262 tracks (95 at
  NAS scale), strongest false unchanged; one long mix gains a second
  possible segment (60 hits) of Fanfare's Dubstyle version, already
  possible beside the playing Fanfare.
- Kept sides overlap in reported time (median 5 s in 0175, 19.5 s at
  most in 0176's traces); plays group them as before. The shared default
  is the same run's `default_final` trace. Data `data/0178-overlap-by-chains.json`.

**Conclusion.** V1 and V3 hold; V2 fails by its letter on the shared
panel only: in fire-6's two "beyond" queries (indexed, played left out)
the Rogue Unit remix, wrongly confident there by default (4,126 hits),
gains a second segment (366 and 627 hits). Not adopted; an ADR would
supersede ADR 0008's consequence for overlapping parts and decide how a
play shows overlapping segments, and whether a wrong file's extra segment
counts as a new wrong answer.
