# 0035: the NAS store as indexed

**Question.** Item 1 of session 5 (NAS plan steps 1, 2): how fast did the
owner's index run read the NAS over Wi-Fi, and what does the store hold?

**Method.** `scripts/analysis/store_census.py` reads every header, tag
sidecar and skip note (read-only; identical records by a digest of the
peak bytes); `doctor` and `stats` (78.2 s, 56.5 s CPU, 3.07 GB) with
`--config /dev/null --peaks-dir <store>`. Data: `data/0035-*.json`.

| Indexing | NAS over Wi-Fi (owner's run) | Local disk (0028) |
|---|--:|--:|
| Files; audio; bytes decoded | 26,890; 2,806 h; 181.3 GB | 262; 29.3 h; 2.78 GB |
| Wall time; per track; per hour of audio | 17,317 s; 0.64 s; 6.2 s | 52.5 s; 0.20 s; 1.8 s |
| Throughput; mean bitrate of the files | 84 Mbit/s; 144 kbit/s | 423 Mbit/s; 211 kbit/s |

At the local rate per hour of audio the run would take about 5,040 s
(inference: AAC and Opus decode as cheaply as MP3), 3.4 times less. The
tags pass took 18 ms per file with 10 jobs (490 s).

| Store | Value |
|---|---|
| YouTube downloads, 8 channels (+2 with 5 files) | 24,293 records (90.3%), 2,527 h; 16,355 m4a, 7,938 opus |
| Other folders (754 at the top level) | 2,597 records, 279 h; 2,459 mp3, 138 m4a |
| Length | mean 6.26 min, median 6.29, deciles 4.9-7.5, range 1:30-14:57 |
| Tags (all current) | artist and title 26,699 (99.3%); title only 47; none 144 (125 in other folders) |
| Identical peak records | 101 groups, 103 files beyond the first; 99 groups are rips kept in two release folders |
| Artist and title on several files | 1,519 names on 3,220 files; 1,497 across folders, mostly channels |

| `stats` | NAS store | Corpus (262) |
|---|--:|--:|
| Peaks per second; store bytes per second | 37.5; 188 | 37.3; 187 |
| Postings per second; bytes per posting (delta varint) | 74.8; 4.01 (3.15) | 74.4; 5.07 (4.19) |
| Buckets: mean, p99, max; fullest 1% share | 360, 4,538, 106,517; 26.1% | 3.7, 55, 1,796; 30.7% |
| Index | 755 M postings, 3.03 GB | 7.8 M postings, 39.7 MB |

**Conclusions.** (1) Indexing over Wi-Fi was I/O-bound: 84 Mbit/s, where
the CPU would take about 290 Mbit/s of these files (inference). (2) Peaks
and postings per second equal the corpus's on YouTube AAC and Opus. (3)
Repeated names are mostly different encodes, not byte-identical copies.
