# 0099: where a first `index` spends its time

**Question.** Step 3 of session 17: a first `index` of the NAS collection
runs at about 1.9 s of wall time per hour of audio, CPU-bound (0064).
How much of it is Gunfinger's analysis, and can that be cut with peak
records that stay bit-identical?

**Method.** First runs into fresh scratch stores under `work/s17/scratch/`
(`--config /dev/null`, 10 jobs; `work/s17/index-run.sh`), never the NAS
store: the corpus library (265 records, 29.58 h, local disk) and the NAS
folder `__youtube_archivists/error-bat` (187 records, 18.93 h; no
candidate of 0065; read-only, after a warm-up run). A B B A, the start
build against `s17-step2` (0096-0098). `sample` on the gunfinger process
gives its own CPU; `/usr/bin/time` adds FFmpeg's and FFprobe's.

| Library | Build | Wall | User; sys CPU |
|---|---|--:|--:|
| corpus | `s17-start` | 51.7, 51.3 s | 320, 319 s; 168, 168 s |
| corpus | `s17-step2` | 49.5, 49.4 s | 295, 295 s; 175, 176 s |
| NAS folder | `s17-start` | 36.6, 36.7 s | 220, 216 s; 126, 122 s |
| NAS folder | `s17-step2` | 35.4, 35.2 s | 199, 195 s; 126, 121 s |
| corpus, FFmpeg alone (same command, 10 at a time) | | 43.7 s | 255 s; 165 s |

Every file of the four stores of each series is identical (records and
tag notes; records hold no write time). Another process used about half a core
in the NAS series' last two runs.

**Where it goes** (corpus): FFmpeg's decode and resampling is about 420
s of the run's 488 s of CPU, and nearly all its system time; Gunfinger's
own CPU was 67 s (peak picking 38 s, the rest of the STFT and log power
19 s, reading FFmpeg's output and spawning 8 s, store writes 2.4 s) and
is 35 s with 0097's picker (`pick` 38 -> 9 s). Tags are read by FFprobe
in about 30 ms a file.

**Conclusion.** 0097 makes a first `index` 4% faster (3.5% on the NAS
folder) with identical records; no further change. FFmpeg now takes 86%
of the CPU, and the run is within 13% of FFmpeg alone; its options are
fixed (brief 6.1, the profile), so what is left is out of scope. At this
folder's rate (1.86 s a hour of audio, 1.93 s before), a first index of
the NAS collection (3,433 h in 0064) takes about 1.78 h instead of 1.84
h. Data `data/0099-first-index-profile.json`.
