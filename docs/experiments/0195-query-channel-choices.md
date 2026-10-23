# 0195: searching one channel, the louder one or the side instead

**Question.** Does a channel choice on the query win back what 0194's
faults cost, and at what cost? Rules: brief 42.

**Change.** Harness only: `robust --query-channels left|right|louder|side`
searches one channel, the louder, or (L - R) / 2; `either` (left and
right), `downmix+side` and `downmix+either` are unions of runs.

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

One channel keeps 61-64% of the downmix's hits on clean queries. Left,
rendered alike in every row, finds 74-78; by inference the joint-stereo
MP3 and the limiter's gain shared with the faulted channel change it.
Wrong confident: none at 262 tracks; at NAS scale only the held-out Cool
Rock Stuff excerpt as the Mobb Deep / Urban Takover upload, 5 detections
under the downmix, 4-18 under the others, 0 under right. One possible
answer is raised at NAS scale (Cause 4 Concern - Abduction as their
remix of Future Sound): on clean queries under left, louder and their
unions; under `stereo-silent` always.

**Conclusion.** No choice meets the pre-registered bar for a fallback:
`louder` loses 3 and `either` 2 of the clean control's 80 (the bar is
1), and the side-louder-than-mid trigger fires on 2 clean queries and on
91 of 100 delayed by 5 ms (brief 42, 06:33). The side wins back every
inverted excerpt but finds nothing in clean audio. The downmix with both
channels (exploratory) keeps every clean query and wins back 77-78 of 80
under each fault but noise at -10 dB (75), at three searches a query.
