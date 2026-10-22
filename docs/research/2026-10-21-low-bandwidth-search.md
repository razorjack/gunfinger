# The search on a machine with low memory bandwidth (2026-10-21)

**Status: items 1 and 2 in master (session 36, experiment 0169); items 4-6
measured or counted and dropped (0171-0173).** On the Linux machine (AMD
Ryzen 9 5950X, 16 cores, 32 hardware threads, 64 GB) the NAS-scale search
was slower than on the Mac at the default 32 workers (experiments 0155,
0157, 0158). Two design agents, working independently on one brief
(`work/perf-design/`: `gpt-6-astra.md` and `claude-fable-5-1.md`, with
their scratch measurements), read the search path and proposed changes.
This note consolidates them. Two of the proposals were measured by one
agent and checked again on a clean copy of master: experiment 0167.
Session 36 adopted them (0169) and measured the rest (below, "Session
36").

## What the machine is

- Two core complexes of 8 cores, each with its own 32 MiB L3 (threads 0-7
  and 16-23 share one, 8-15 and 24-31 the other); 32 KiB L1d and 512 KiB
  L2 per core, shared by its two threads; one NUMA node. Board Gigabyte
  B550 VISION D-P, BIOS F18. Power: `throughput-performance`,
  `amd-pstate-epp`, governor and EPP `performance`.
- A probe in safe Rust (`work/perf-design/claude-fable-5-1-scratch/probe`)
  measured a sequential read ceiling of 25.6-27.0 GiB/s from 2 threads up,
  104.6 ns of DRAM latency, and 9-14 GiB/s for random short reads, flat
  from 8 to 32 threads. A 5950X with dual-channel DDR4-3200 or 3600 is
  usually reported at 40-45 GB/s and 80-90 ns (published figures, not
  ours), so the memory may run at a JEDEC default (DDR4-2133 or 2400) or
  in one channel. The DIMM speed needs root: `sudo dmidecode -t 17` shows
  "Configured Memory Speed" and the populated slots (A2 and B2 for two
  channels on this board). If the DIMMs are rated higher than they run,
  XMP in the BIOS raises both bandwidth and, on Zen 3, the Infinity
  Fabric clock. *(Later the same day)* The modules (DDR4-3600) did run at
  2133 MT/s; with XMP on, the probe reads 40-46 GiB/s at 82 ns. The current
  code's search at 32 workers is then 35% shorter, but with item 1 and 2's
  change, or at 16 workers, it does not move (experiment 0168).
- Transparent huge pages are `madvise`, so the 4.6 GB posting array gets
  4 KiB pages. Both agents expect little from huge pages: a scan starts
  only about 612 lists per window and rung, 17 M in a whole search.

## Where the time goes

Counted on the Stakka mix at NAS scale (56:09, 82 rungs, 40,683 assets,
an instrumented build): 21.1 M hash lookups, 49.8 G hits (1.80 M per
window and rung, 2,947 per list), 1.12 G assets with 3 hits or more
reaching the pre-filter (nearly every asset in every window), 24.6 M
(2.2%) sorted, 5.65 M lines.

- The posting reads are 199 GB per search, about 4 GB/s at 16 workers, in
  sequential runs of 11.8 KB on average: far from the streaming ceiling.
  The index layout is not what limits the search here.
- Each hit is then copied into `part` (8 bytes), read twice by the
  counting sort, scattered into `grouped` (8 bytes) and read by the
  pre-filter: about 40 bytes of cache traffic per hit, 2 TB per search.
  This stays fast only while a worker's `part` and `grouped` fit its share
  of the L3. With parts of 4,096 assets they average 2.9 MB per worker
  (5 MB at most): 8 workers per core complex need about 23 MB of its 32
  MiB, 16 workers about 46 MB. At 32 workers the scatter spills to DRAM at
  random addresses, which by inference is why 32 workers take 1.6 times
  the time of 16 (0158).
- Pinned runs agree: 8 workers on one complex 87.9 s, 16 workers on the
  same 8 cores 90.1 s with twice the CPU time. The second thread of a core
  adds nothing to this search.
- `take_part` binary-searches the remainder of every list for each part:
  about 7 dependent misses per list and part outside the lines the part
  uses (169 M searches per search). On the Mac these searches were
  1.3-1.4% of CPU (0113); here they become the largest avoidable cost
  once parts are smaller.
- The query analysis (STFT and peaks, about a quarter of CPU on the Mac's
  profiles) is compute-bound and scales with cores. Sorting and lines are
  3-6%, the second pass 2-3%.
- Not measured here at first: this machine's split of CPU between taking,
  grouping, the pre-filter and the analysis. Session 36 measured it with
  timers in a diagnostic build instead of `perf` (0170; below).

## The proposals, consolidated

Each keeps detections bit for bit identical unless marked otherwise, and
none changes postings, `hash::design()` or the saved index.

1. **Find each part's boundary from the front of the list** (Fable, P3;
   measured, 0167). A doubling search from the front, then a binary
   search between the last two probes, in place of `partition_point` over
   the whole remainder (`search/lines.rs`, `take_part`). Same result on
   sorted lists, so the same hits in the same order. About an hour with a
   test against `partition_point`.
2. **Smaller parts** (both; measured, 0167). `ASSETS_PER_PART` from 4,096
   to 1,024 or 2,048. With item 1 in, 1,024 and 2,048 are equal at 16
   workers and 1,024 is slightly faster at 32. Without item 1, 2,048 is
   slower than 4,096 at 16 workers (56.9-58.6 s against 49.5 s), which is
   why 0096 found smaller parts slower on the Mac. A variant sizes parts
   per window from its hit count (a budget of about 1.5 MiB per worker),
   to be measured against the best constant.
3. **The default worker count** (both). Items 1 and 2 make 32 workers
   faster than 16 on this machine (38.5 s against 45.7 s of search), so the
   default of one worker per hardware thread can stay. Without them,
   `--jobs 16` is the stopgap. Counting physical cores instead would be a
   platform function (ADR 0020) and our decision; decide after items 1
   and 2 are in.
4. **Narrower hits** (both, in two forms; not measured). Once grouped, a
   hit's asset is known from its slice, so `grouped` needs only the frame
   and an anchor index relative to the window's first anchor: 4 bytes
   instead of 8 (GPT-6-Astra). Raw hits could be 6 bytes as two arrays
   (`Vec<Posting>` and `Vec<u16>`; Fable) or as `[u16; 3]`. A window with
   more anchors than a `u16` holds must fall back to the wide form, never
   wrap. A few percent expected at 16 workers, more where parts spill.
5. **Group straight from the lists, without `part`** (Fable, P5;
   uncertain). Count assets from the list fronts, then scatter from them
   into `grouped`: one buffer per worker and about a third less traffic
   per hit. The same idea lost on the Mac (0096 +6%, 0103 +7%), where
   nothing spilled; here the gain would be cache capacity. Only after a
   profile.
6. **Read repeated hashes once** (GPT-6-Astra; not measured). Sort a
   window's requests by hash, so equal hashes find their part once and
   their postings are read once, keeping every occurrence as evidence.
   Measure first how many requested bytes are repeats. Extending it to
   two to four windows or neighbouring key-locked rungs is a larger
   change with an uncertain result.
7. **Smaller buffers and sorts** (GPT-6-Astra): the line vector made per
   call, `densest_clusters`' candidates, a merge of block results instead
   of the final stable sort. Low single-digit gains at most; only where a
   profile shows them.
8. **Not recommended for speed now.** In-memory posting compression: a
   sample of 8,192 lists gives 3.06-3.12 bytes per posting (LEB128 asset
   gaps, frames kept at 2 bytes, restart blocks of 128), against 4.01
   today, but the posting reads are not the limit, and a decoder would sit
   in the hottest loop. It remains the answer to memory at 65,536 assets
   and to the on-disk size (ADR 0011), with a new ADR. Sharding by core
   complex or NUMA node: one node, and a shard of hundreds of MB does not
   fit an L3. Huge pages: little expected; the `memmap2` crate's anonymous
   map and `advise(Advice::HugePage)` are safe functions, a new dependency
   and our decision.
9. **Changes to output** (a retrieval study and our decision first):
   skipping more of the fullest lists (the only lever that reduces the
   hits themselves), FFT lengths with small prime factors (0109, 0117,
   0119).

## An order for a session

1. Items 1 and 2 with their tests (`front_partition` against
   `partition_point` on random, empty and edge lists); the three
   development scans and the leave-outs at NAS scale at 16 and 32
   workers, byte for byte apart from `wall_seconds`; the clusters sample
   (`clusters --from-peaks --sample 48`); `regress` at both scales.
2. The default worker count, decided from those times.
3. A profile with `perf` at 16 and 32 workers, if installed, to decide
   items 4-6.
4. Item 4, then 5 or 6 if the profile supports them, each against its
   stop point (3-5% at 16 workers).
5. The user, any time: `sudo dmidecode -t 17`, XMP if the DIMMs run below
   their rating, then the same scan again.

## Session 36

All at 3600 MT/s, the Stakka scan at NAS scale, reports identical to
session 33's apart from `wall_seconds`.

- Items 1 and 2 in master (0169): `front_partition` with tests, parts of
  1,024 assets. Search 45.3 s instead of 47.2 s at 16 workers, 37.9-38.0 s
  instead of 49.0-49.2 s at 32; the full NAS clustering run 23:02 (52:30 in
  session 33 at 2133 MT/s with the old code), its file byte-identical.
  The per-window hit budget was 2.3-2.7% faster than 1,024, below the 3%
  set for it, and was not kept.
- The split (0170), with items 1 and 2 in, at 16 and 32 workers: the
  pre-filter 43% and 46% of the search's CPU, grouping 28% and 24%,
  query analysis 14% and 15%, taking parts 6% and 7%, sorting and lines
  6%, the second pass 2%, lookups 0.3%.
- Item 4 (0171): grouped hits as 4 bytes 4.4% slower at 16 workers, raw
  hits as 6 bytes 8.2% slower; both slower at 32. Dropped.
- Item 5 (0172): grouping straight from the lists 7.8% slower at 16
  workers. Dropped.
- Item 6 (0173): 5.1% of requested postings repeat a hash of the same
  window, 14.9% within 4 windows; since every occurrence is still grouped
  and filtered, the saving is at most about 1%. Not implemented.
- Item 3 (0174): keep one worker per hardware thread. With items 1 and
  2, 32 workers take 16% less search time than 16, and `identify` is not
  slower at 32; the default is unchanged (our decision).
- Left: item 7 (smaller buffers and sorts: sorting and lines are 6%, the
  second pass 2%), and the pre-filter itself, now the largest stage
  (integer phases lost on the Mac, 0108; not tried here).

`regress` prints what changed and returns success either way
(`regress.rs`, `compare`): read its output, do not rely on its exit
status.
