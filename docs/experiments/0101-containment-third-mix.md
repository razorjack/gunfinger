# 0101: the containment rule on the third development mix

**Question.** Session 18, step 3: what does the containment rule (a
confident play inside a stronger one of another cluster is shared
material) remove and cost at 0091's setting (passages, 10 s, with and
without a 0.5 bound on the ratio of hits) on 0100's mix and every report?

**Method.** `containment.py` (0091's grid), the new mix's scans and NAS
leave-outs added, session 18's clusters. New: every scan (not the
leave-outs) with the strongest file of each identified track dropped,
its other rips keeping their hits (does a weaker copy still identify it?).

| Passages, 10 s; bound none / 0.5 | Confident plays | Wrong | Removed | Right demoted |
|---|--:|--:|--:|--:|
| New mix, 262 tracks | 9 | 0 | 0 / 0 | 0 / 0 |
| New mix, NAS | 36 | 0 | 0 / 0 | 0 / 0 |
| New mix, NAS leave-outs (3 seeds) | 88 | 0 | 0 / 0 | 0 / 0 |

- The new mix's closest call: Anthrax's play would need 99 s of tolerance
  (NAS; 117 s at 262) to lie inside Blindside's stronger play.
- Strongest file dropped, at every grid point, in the scans: 13 of 39
  identified tracks (262) and 45 of 57 (NAS) keep a weaker confident
  copy; none loses it.
- Every other row is 0091's, unchanged (432 rows): no wrong play in the
  other scans, sweeps, recall, robust, generated mixes; 0076's
  passages 26 -> 0 runs with a wrong play, 10 right demoted, 2 runs losing
  the right recording when only weaker rips remain (bound 0.5: 26 -> 6, 2
  demoted, none lost); new judged pairs 5 of 6 related plays removed;
  with the played cluster left out, nothing the rule can fix.

**What it shows.** The other versions of the mix's VIPs and remixes share
little audio with the played ones (0100: 0-38 hits in `pair`, 6-19 in
their slots; the Klute Remix 166-263, possible in the mix and marked by
`identify` as sharing material). No real mix has a wrong play to remove.

**Conclusion.** Not adopted (our decision); recommended: do not
adopt. On real mixes it removes nothing and costs nothing; its gain is
measured only on constructed passages, and it cannot help where the
played recording is missing. If adopted, passages with 10 s and a 0.5
bound never lost a right recording. Data `data/0101-*.json`.
