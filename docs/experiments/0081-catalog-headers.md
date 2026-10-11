# 0081: Reuse the index plan for report freshness

**Hypothesis.** Report freshness and index loading can share one plan of
current records, avoiding a repeated read of every peak header.

**Method.** `work/perf/headers.rs`, `rustc -O` against the release core.
`work/perf/headers > work/logs/perf-headers.log`; twelve alternating pairs,
ten preparations of the copied 262-record store per sample. Compare
`current_sources` + `indexable_assets` + `plan_index` with `current_sources`
+ `plan_index` and compute both revisions from the same planned sources.

**Result.** Identical revisions; median 0.322862 s versus 0.213027 s per ten
preparations (0.66 times wall time, warm local file cache).

**Conclusion.** Keep one index plan through report freshness and catalog
opening. Store-only discovery still reads headers once to discover sources;
removing that second read needs a broader store API and is left for a later
measurement. The measured saving does not predict NAS cold-load latency.
