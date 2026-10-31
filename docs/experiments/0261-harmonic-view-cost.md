# 0261: what the harmonic view costs

**Question.** Brief 66, step 4: the time and memory of the view in the
harness (decoding reference audio, the separation, the peaks), and what
a product use would need.

**Runs.** The Linux machine (Ryzen 9 5950X, 32 workers), `/usr/bin/time
-v`. 262 tracks: the development plan with `--harmonic` (`dev-262`,
beside 0259's NAS runs) and without it (`cost-262-off`, beside two of
them; renders not cached in either). NAS scale: 0259's four processes.

| Run | Searches | Cases measured | CPU (user + system) | Wall | Peak memory |
|---|--:|--:|--:|--:|--:|
| 262 tracks, without the view | 152 | 0 | 513 s | 2:48 | 0.62 GiB |
| 262 tracks, with it | 182 | 200 | 1,254 s | 5:38 | 4.56 GiB |
| NAS scale, with it, four processes | 1,279 | 9,678 | 38,690 s | 24:06-34:28 | 8.6-9.8 GiB each |

- At 262 tracks the view adds 741 s of CPU; the 30 added (killed and
  damaged) searches take about 100 s at 3.4 s a search, so the view costs
  about 3.2 s of CPU per case, the query's separation included. Decoding
  one reference as indexed (a 6-minute MP3 from its start to 30 s past
  200 s) takes 0.4 s; the rest is the STFTs, the medians (25 values per
  point across time, 17 across frequency), overlap-adds and pickings.
- Memory: a 240 s query's 15,000 frames of 513 complex bins (62 MB) are
  held several times during the separation; with 32 workers this lifts
  the 262-track run from 0.62 to 4.6 GiB and a NAS process to 8.6-9.8 GiB
  (session 52's both panels' development halves, no view: 5.9-6.6 GiB).
- One NAS process ran at about 4.2 cores (a long looped query and its
  many candidates are measured in one worker), so 0259 split the plan.
- A product use needs the reference audio at identification time (a
  decode per candidate, which `identify --store-only` or an unreachable
  library cannot give) or a new stored field: harmonic peaks at 8 x 8 are
  2.16 times the stored 12 x 12 peaks in the same spans (median of the
  262-track cases), about three times today's store with both. Each is a
  decision with its own ADR (ADR 0003, ADR 0017).

**Conclusion.** At about 3 s of CPU per candidate the view is cheap for a
diagnostic and costly for every candidate of a mix; nothing measured in
0259-0260 earns either product path.
