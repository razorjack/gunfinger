# 0195: searching one channel, the louder one or the side instead

**Question.** Does a channel choice made on the query win back what a
fault costs the downmix (0194), and at what cost? Brief 42's rules.

**Change.** Harness only: `robust --query-channels left|right|louder|side`
decodes each query in two channels and searches one, the one with the
higher RMS, or (L - R) / 2. `either` (left and right), `downmix+side`
and `downmix+either` (exploratory) are unions of runs, per query.

**Command.** `robust --only stereo-...` per choice at 262 tracks (1:04-1:08)
and NAS scale (3:32-3:57); data `data/0195-query-channel-choices.json`.

| Confident of 80 (262 tracks; NAS where it differs) | Control | Inverted | Silent | Noise 10, 0, -10 dB | Delay 0.2, 1, 5 ms |
|---|--:|--:|--:|--:|--:|
| Downmix (today) | 80 | 0 | 78 | 80, 29, 0 | 80, 70, 37 |
| Left (the clean channel) | 77 | 75 | 78 | 74, 76, 75 | 75, 76, 75 |
| Right (the faulted one) | 78 | 77 | 0 | 53, 0, 0 | 76, 78 (77), 76 |
| Louder | 77 | 76 | 78 | 62, 2, 0 | 75, 76, 75 |
| Side | 0 | 80 | 78 | 0, 0, 0 | 78, 69, 41 |
| Either channel | 78 | 77 | 78 | 74, 76, 75 | 76, 78 (77), 76 |
| Downmix + side (exploratory) | 80 | 80 | 78 | 80, 29, 0 | 80, 72, 57 |
| Downmix + either (exploratory) | 80 | 77 | 78 | 80, 78, 75 | 80, 78 (77), 77 |

One channel keeps 61-64% of the downmix's hits on clean queries. Wrong
confident: none at 262 tracks; at NAS scale only the held-out Cool Rock
Stuff excerpt as the Mobb Deep / Urban Takover upload, also where a
fault hid it from the downmix (5 detections over the rows; left, either
and downmix + either 18, louder 14, downmix + side 7, side 4, right 0).
Raised: one possible answer at NAS scale whenever only the left channel
is heard, Cause 4 Concern - Abduction as their remix of Future Sound.

**Conclusion.** No choice meets the pre-registered bar for a fallback:
`louder` loses 3 and `either` 2 of the clean control's 80 (the bar is
1), and the side-louder-than-mid trigger fires on 2 clean queries and on
91 of 100 delayed by 5 ms (brief 42, 06:33). The side wins back every
inverted excerpt but finds nothing in clean audio. A union of the
downmix with both channels keeps every clean query and wins back 77-78
of 80 under each fault but noise at -10 dB (75), for three searches a
query (exploratory).
