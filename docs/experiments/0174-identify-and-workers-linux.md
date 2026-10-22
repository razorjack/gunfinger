# 0174: `identify` at NAS scale with 0169, and the number of workers

**Question.** Session 36, step 6: how much does 0169 save in the user's own
command on the Linux machine, and should the default count one worker per
hardware thread (32) or per core (16)?

**Method.** `gunfinger --config ~/.config/gunfinger/nas-dnb.toml identify
--jobs N -f json <mix>` (the NAS library, its store of 40,683 records,
turntable playback by the configuration, the saved index current) on the
three development mixes, `s35-final` against 0169's build, 16 and 32 jobs,
two rounds in opposite orders (`work/s36/identify-ab.sh`, `-ab2.sh`). A
first, cold run spent most of its 8:54 waiting for the NAS's directory
listing and is left out. Wall time covers listing the NAS library, loading
the 4.6 GB index, decoding and both passes.

| Mix, jobs | Wall, before | Wall, 0169 | User CPU, before | User CPU, 0169 |
|---|--:|--:|--:|--:|
| Stakka, 16 | 45.4, 45.2 s | 45.4, 42.5 s | 373-378 s | 361-363 s |
| Stakka, 32 | 47.8, 49.1 s | 46.1, 45.5 s | 739-741 s | 606-608 s |
| My 2003 mix, 16 | 44.7, 52.0 s | 42.2, 46.8 s | 301-302 s | 291-292 s |
| My 2003 mix, 32 | 38.0, 38.7 s | 39.3, 35.7 s | 578-581 s | 486-489 s |
| Nocny, 16 | 56.3, 49.8 s | 50.0, 54.4 s | 478-480 s | 459 s |
| Nocny, 32 | 54.4, 52.5 s | 48.7, 46.2 s | 992-997 s | 771 s |

Every report byte-identical across builds, job counts and rounds.

**Result.** User CPU falls 3-4% at 16 jobs and 16-22% at 32. Wall times
move by up to 7.3 s between rounds of the same build, from the stages around
the search (the NAS listing above all), so at 16 jobs the change is within
that noise; at 32 jobs 0169 is faster for Stakka and Nocny in both
rounds, by 1.8-6.2 s.

**Workers.** With 0169, the search alone (0169's scans) takes 37.9-38.0 s
at 32 jobs against 45.3-45.4 s at 16, and the full clustering run 23:02 at 32 (16
not measured). In `identify` the two counts are within the noise (32: 35.7-
48.7 s; 16: 42.2-54.4 s), with 1.7 times the CPU time at 32. Recommended:
keep one worker per hardware thread; counting cores would make the search
slower where it can be measured apart. The default is unchanged (our
decision).
