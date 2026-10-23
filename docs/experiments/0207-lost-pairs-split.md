# 0207: why reference pairs are lost: the split

**Method.** 0206 as registered: harness `816fd57`, runs 10:19-11:18, each
exit 0; 40,683 records (R1c, R1C, R2, R3, R4), 262 tracks (R0).
`lost_pairs.py`; `data/0207-lost-pairs-split.json`. Lost: reference pairs
in the passage minus E; shares pooled over each set's detections
(per-detection medians within 0.02, except P and T(0): 0.05-0.17).

| Set | n | E/R | Endpoint gone | Displaced (3; 4-5; 6-8; 9+) | Changed value (near; far) | Outside zone | Displaced 3-8 / E |
|---|--:|--:|--:|--:|--:|--:|--:|
| T(C) clean | 522 | 0.460 | 0.550 | 0.171 (0.127; 0.041; 0.002; 0) | 0.242 (0.242; 0.000) | 0.036 | 0.20 |
| T(c) damaged | 511 | 0.048 | 0.883 | 0.045 (0.025; 0.017; 0.004; 0.000) | 0.063 (0.061; 0.002) | 0.008 | 0.89 |
| T(a) other rips | 178 | 0.248 | 0.681 | 0.108 | 0.183 | 0.028 | 0.33 |
| T(0) 262 tracks | 160 | 0.330 | 0.745 | 0.102 | 0.132 | 0.021 | 0.21 |
| P played, shared | 1,297 | 0.311 | 0.689 | 0.102 | 0.186 | 0.022 | 0.23 |
| R related, shared | 5,356 | 0.075 | 0.883 | 0.040 | 0.070 | 0.007 | 0.49 |
| R(r) related wrong | 380 | 0.042 | 0.932 | 0.022 | 0.042 | 0.004 | 0.50 |
| F unrelated | 2,926 | 0.008 | 0.993 | 0.003 | 0.004 | 0.001 | 0.34 |

- T(c), per reference pair: an endpoint is gone for 84% (clean 30%):
  anchor alone 22%, target alone 22%, both 44% of the lost pairs. With
  both peaks found the pair forms with the same hash (E, 4.8% of
  pairs), is displaced (4.3%) or changes value (6.0%). Damage adds loss
  almost only as endpoints gone; displaced and changed pairs halve with
  the pairs that keep both peaks.
- Displaced targets sit just past the fan-out: rank 3 is 55% of them
  (T(c)), 4-5 37%, 6-8 8%, 9 or later under 1%.
- N recovers changed values: 93% of T(c)'s changed pairs are in N, and
  79% of N is such a pair; 21% of N has an endpoint gone (a near hash
  from other peaks). Exact pairs not "paired, same hash": 4.6% of E in
  T(c), 0.0% clean, 23% in F (chance agreements). The residual (lost,
  "paired, same hash") is 0.1% of the lost pairs or less everywhere.

**Against 0206.** The share is reached through its second clause: in
T(c) the displaced pairs at ranks 3-8 are 4.5% of the lost pairs (under
15%) but 0.89 of E (over 0.25). Expected endpoint gone 60-80%: 88%;
displaced 5-15%: 4.5%; changed 10-25%: 6.3%; clean displaced under 5%:
17% (wrong). **Conclusion.** Under damage pairs are lost because peaks
are lost; displacement is a small share of the loss but large against
what is left, so the fan-out counts are analysed (0208).
