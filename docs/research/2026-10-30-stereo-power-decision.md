# Stereo power: what it recovers, what it costs (2026-10-30)

**Status: measured (session 61, experiments 0245-0247); not adopted.**
The product, its front end and every default are unchanged. The variant
is an opt-in harness front end (`--front-end`, `variant-store`); this
note gives our recommendation for whether to take it further.

## The variant

Each channel gets its own STFT (today's window, size and hop, and each
rung's in the search), and the peaks are picked from `P = |X_L|^2 +
|X_R|^2` with no further scaling. FFmpeg's downmix is `M = (L + R) /
sqrt(2)`; with `S = (L - R) / sqrt(2)`, `P = |X_M|^2 + |X_S|^2`, today's
power plus the side's. A mono source gives today's power: a stereo file
with equal channels exactly in exact arithmetic, a one-channel file too
(FFmpeg's `-ac 2` gives each channel the sample at 1/sqrt(2)). Inverting
either channel leaves `P` unchanged to the bit (a unit test). Records
carry their own profile identifier (`... channels=power-sum`).

## What it recovers

- **Faults in the query** (0245, 0246; 80 indexed excerpts): an
  inverted channel 0 to 80 confident, delays of 1 and 5 ms 70 and 37 to
  80, a silent channel 78 to 78-80. Over the inverted half of 24
  generated mixes, 1 of 100 plays confident to 93-94.
- **Faults in the rip** (0246; only with a store built by the variant):
  the 10 copies of 0197 inverted, silent, 1 ms and 5 ms late, 0, 19, 18
  and 4 of 20 confident to 20 each.
- **Query-only** (0245, today's records): every query-side recovery
  above holds against today's downmix records, at 262 tracks and at NAS
  scale, with mono queries searching exactly as before and clean stereo
  queries keeping 95% of the hits and every detection.

## What it costs

- **Noise in one channel is worse.** At 0 dB, 29 of 80 confident under
  the downmix, 18-19 with power-sum queries and 23 with both sides
  built by the variant; by inference the downmix adds the shared signal
  in amplitude, 3 dB more than the power sum does.
- **Mono queries against power-sum records lose marginal excerpts**
  (0246): 6 of 38 default `robust` conditions, rendered mono, fall by
  2-4 of 80 (low-pass 800 Hz, needle skip, clipping, pink noise at 10
  dB, blends at 0 and -6 dB); mono queries keep 0.93-0.99 of the hits.
  This fails the registered bar (within 1 of 80). A mono recording of a
  mix (a tape transfer, a mono broadcast) is the case it stands for.
- **Search time.** Power-sum queries cost 12% more wall time and 13%
  more CPU on the Stakka scan at NAS scale (27.6-27.7 s against
  24.6-24.8 s, 32 jobs, the Linux machine), about 45% more at 262
  tracks, and 2% more peak memory (0247).
- **Records.** Density is unchanged (0.1% fewer peaks and postings).
  Building a record takes the same time (decoding dominates) and 3 times
  the peak memory at 32 jobs.
- **Adoption for records means a new profile and every reference
  rebuilt** (ADR 0003's invalidation rule): the NAS store's 40,678
  records decoded again from the NAS. By scaling the corpus build (265
  files, 29.6 h of audio, 15.7 s at 32 jobs) by record count, about 40
  minutes on the Linux machine if reading over Ethernet keeps up, an
  inference; the Mac's estimate for a first index of 33,596 records was
  1.78 h (session 17). The saved NAS index is rebuilt too. Old records
  cannot be searched under the new profile, so the corpus store, the
  NAS store and every baseline move together.

## Risks

- No wrong confident answer appeared in any run of either phase; at NAS
  scale the only one is session 40's known pair, which the downmix also
  gives. The variant was not run on the shared-material panels, on
  `recall`'s other-rip panels or on generated mixes at NAS scale.
- How often an inverted or delayed channel occurs in the NAS rips or in
  real mixes is still unknown (0196): the gain is certain per fault, its
  frequency is not.
- Noise in one channel and mono queries against new records move the
  other way; both would need their own look before records change.

## Recommendation (ours to take)

Do not change the record profile: the registered bar for an ADR
proposal is not met, and the rebuild moves every store and baseline for
a fault of unknown frequency. The query side alone is the better
candidate: it recovers every query-side fault against today's records,
needs no rebuild, leaves mono queries as they are and costs 12% of
search time at NAS scale. It should get its own pre-registration
(at NAS scale: the shared-material panels, `recall`'s other rips, the
generated mixes, the noise conditions and the frozen acceptance rule
of the proposals note) and, if it passes, an ADR for the CLI's
decoding. Faults in the rip stay unrecovered without a rebuild.
