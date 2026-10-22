# 0173: how many posting reads repeat a hash already read

**Question.** Session 36, step 5 (the research note's item 6): reading
each repeated hash's postings once per window, or across 2-4 neighbouring
windows, keeps every occurrence as evidence and saves only reads. How many
requested postings repeat a hash already requested, and can the saving
reach the note's stop point (3% of the search at 16 workers)?

**Method.** A counting build of 0169's code, never committed
(`work/s36/lines.rs.repeats`): each worker remembers, per block of 12
windows and rung, the last window that requested each hash, and counts the
postings of each non-empty list whose hash was requested in the same
window, within 2 windows or within 4. The Stakka scan at NAS scale, 16 jobs
(`work/s36/repeats.sh`); the report identical to session 33's apart from
`wall_seconds`.

| Requests (non-empty lists) | Lists | Postings |
|---|--:|--:|
| All | 16,900,600 | 49,808,018,472 |
| Hash already requested in the same window | 871,049 (5.2%) | 2,532,829,319 (5.1%) |
| Within 2 windows (the window and the one before) | 1,841,485 (10.9%) | 5,394,760,673 (10.8%) |
| Within 4 windows | 2,496,737 (14.8%) | 7,428,453,821 (14.9%) |

The first windows of each block have fewer windows before them in the
block, so the cross-window counts are somewhat low: at most a third more
if every window had 3 before it (an upper bound, about 20%).

**Conclusion.** Not implemented. Every repeated occurrence must still be
grouped and pre-filtered with its own anchor, so reading a hash once saves
only part of taking parts (6.3% of the CPU, 0170) and the lookups (0.3%):
at most about 1% of the search within 4 windows, 0.3% within one, below
the 3% stop point. Also below the design report's own screen (25% fewer
bytes read). Item 6 stops here.
