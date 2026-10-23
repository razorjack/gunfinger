# 0206: why reference pairs are lost under damage: pre-registration

**Question.** Roadmap, "Evidence beyond exact pair hashes", step 2's
open part: under combined damage the verifier finds a median 37% of the
reference peaks while E keeps 9.6% of the clean exact pairs (0151). Is
a lost reference pair lost because an endpoint is gone, because its
target is displaced in the anchor's zone, or because its value changed?
Would pairing the query with more targets recover it?

**Split** (per reference pair at the alignment; full rules in brief 44,
step 1). Found: a query peak within ±1 frame and ±1 bin of the
reference peak's place (the verifier's rule). Endpoint gone (anchor,
target, both); else, over candidate peaks, paired (the target is the
query anchor's first or second target: same hash, near, far), displaced
(rank 3, 4-5, 6-8, 9+ in the zone), outside the zone. Lost: reference
pairs minus E; kinds reported for near and unmatched pairs apart.

**Fan-outs** 3, 5, 8, on the verifier's analysis only, index and
records at fan-out 2: fan-out 2 pairs first (E), then the extra pairs
against the reference pairs E left; each reference pair once; G(F) the
extra exact correspondences; chance at the four shifts.

**Runs** (40,683 records): R1c recall damaged, R1C clean, R2 other rips,
R3, R4 the shared panels' development halves; R0 `robust --only combined`
at 262 tracks. Sets as 0151's: T(c), T(C), T(a), F, R(r), P,
R; T(0), W(0) at 262 tracks. Shrinks: R2, then R4.

**Opens the fan-out step:** in T(c), pooled, displaced pairs at ranks 3-8
are at least 15% of the lost pairs or 25% of E (else one exploratory line).

**Scores.** G(F)/E per set and E band; hits + G(F) reaching 240 over 3
windows (T(c), T(a), F, R(r), R); per shared search; AUC of G(F)/E for
P~R, T(c)~R(r), T(c)~F; CPU as query pairs at F over at 2.
**Worth a rules experiment** only if (a) T(c) median G(F)/E >= 0.25 and
>= 10% of T(c) under 240 reach 240; (b) R's median G(F)/E at most half
of P's in every E band with 10 of each, pooled AUC P~R >= 0.75; (c) no F
reaches 240. R gaining as much as P settles it.

**Expected:** endpoint gone 60-80% of the lost, displaced 5-15%,
changed value 10-25%; clean displaced under 5%. **Result:** 0207 and up.
