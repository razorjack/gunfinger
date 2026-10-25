# Question files for `gunfinger-eval audition`

The listening tool (ADR 0016) plays and draws what a question file says.
This is the reference for the format, field by field. The Rust types are
in `crates/gunfinger-eval/src/audition/schema.rs`; `schema::check` enforces
every rule written here, and the server and the producers run it before
using a file.

## Where the files are

| What | Default | Option |
|---|---|---|
| Question files | every `*.json` in `work/audition/questions/`, in name order | `audition FILE...` |
| Answer log | `work/audition/answers.jsonl` | `audition --answers FILE` |
| Pair verdicts | `docs/pair-verdicts.txt` | `gunfinger-eval --verdicts FILE audition` |
| Envelope cache | `work/audition/envelopes/` | follows `--work` |

Producers write question files:

```sh
gunfinger-eval questions pack       # work/audition/questions/01-pack.json
gunfinger-eval questions folders PLAN.toml   # 02-folders.json
gunfinger-eval questions detections REPORT.json   # 03-detections-<report stem>.json
gunfinger-eval questions shared PAIRS.json   # 04-shared-<report stem>.json
```

`questions detections` reads a saved `gunfinger identify -f json` report
that names its library (not a `--store-only` search): one question per
play, the mix from 30 s before the play to 30 s after (within the part
searched) as the reference lane, the play's file placed by its
segments, the strongest segment first (`placement::tile`).

`questions shared` reads a `pair` report and asks of each pair whether
two different recordings share material where they line up (a break, a
loop or a sample) or the match is chance: A whole as the reference lane,
B placed by every alignment of the strongest search of the audio,
strongest first, also those below the hits of a possible play; at each
alignment a span, a `listen` marker where it starts and an `edge` marker
where it ends (markers less than 0.5 s apart joined). Answers (`shared`,
`chance`, `unsure`) go to the answer log only.

## The file

```json
{
  "schema_version": 1,
  "title": "Listening pack",
  "made_by": "gunfinger-eval questions pack",
  "questions": [ ... ]
}
```

| Field | Type | Meaning |
|---|---|---|
| `schema_version` | integer | `1`. Any other value is refused. |
| `title` | string | A heading for this file's questions in the page's queue. |
| `made_by` | string | The command that wrote the file, for the next agent. |
| `questions` | array | The questions, in the order the queue lists them. |

## A question

| Field | Type | Meaning |
|---|---|---|
| `id` | string | `<kind>-<16 hex digits>`: `QuestionId::derive(kind, parts)`, FNV-1a 64 over the kind and the sorted parts. The parts are the question's `files`, plus a distinguishing part where one kind can ask twice about the same files (a detection's start second). Making the questions again gives the same ids, so answers stay attached. Unique across the loaded files. |
| `kind` | string | Lowercase letters and hyphens: `pair`, `folder`, `detection`, `shared`. Shown in the queue and written to the log; the page does nothing else with it. |
| `title` | string | One line for the queue. |
| `text` | string | The question, as the listener reads it. |
| `details` | array of strings | Evidence lines shown under the text (coverage, hits, what the markers mean). May be empty. |
| `files` | array of strings | The files the answer is about, as the harness names them: corpus library paths, `second-library/<path>` for the NAS library, absolute paths otherwise. Written to the answer log. At least one. |
| `timeline` | object | The time axis every lane, marker and span is placed on (below). |
| `lanes` | array | One to nine lanes; keys `1` to `9` switch between them. |
| `markers` | array | Moments to jump to with Up and Down. May be empty. |
| `spans` | array | Shaded stretches of some lanes, such as where two files align. May be empty. |
| `choices` | array | The answers, each with its key. At least one. |
| `effect` | object | Where an answer is written besides the log (below). |

### `timeline`

| Field | Type | Meaning |
|---|---|---|
| `start`, `end` | seconds | The span the page shows and plays; `start < end`. |
| `reference` | lane index, optional | The lane whose own file time the timeline is. Its pieces must have `offset` 0 and `speed` 1, so the page labels the playhead with that lane's time. Absent when the timeline is a sequence of excerpts. |

### A lane

| Field | Type | Meaning |
|---|---|---|
| `name` | string | A short label: `A`, `B`, `original`, `CD`. |
| `pieces` | array | What the lane plays where. In timeline order, not overlapping, inside the timeline. Gaps are silent. For a lane placed by alignments: every alignment it follows. |
| `main` | array of pieces, optional | The lane at its main alignment (the one with the most hits) over the whole timeline, clipped to the file: one offset and speed. Under the rules of `pieces`. |
| `from_start` | array of pieces, optional | The lane's file from its start at the timeline's start, at the main alignment's speed, clipped to the file: no offset. Under the rules of `pieces`. |

`main` and `from_start` are the lane's other placements (below); the
reference lane has neither. Files without them load as before.

### A piece: the placement rule

A piece places a span of one file on the timeline:

```text
file time = offset + speed × t        for timeline time t in [start, end)
```

| Field | Type | Meaning |
|---|---|---|
| `file` | absolute path | The audio file. The server serves only files named here. |
| `start`, `end` | seconds | The piece's span on the timeline. |
| `offset` | seconds | File time at timeline time 0. |
| `speed` | number | File seconds per timeline second, 0.5 to 2. With speed matching on, the page plays the piece at `playbackRate = speed` (pitch and tempo together), so it keeps the timeline's clock. |
| `label` | string, optional | What the piece is ("alignment 2", "track 3, last 20 s"), shown with the lane. |

A pair aligned as `B time = offset + speed × A time` (the `pair` report's
alignments) is a lane B piece with that `offset` and `speed` on A's
timeline. A folder excerpt of track 3 from 361.1 s, at timeline 40 s, has
`offset` 321.1 and `speed` 1. Producers compute pieces with
`audition::placement` (tiling several alignments, clipping to the file's
length); the page evaluates only the formula above, in `timeline.js`.
A lane follows only alignments with at least the 60 hits of a possible
play (`placement::followed`). A `pair` search also lists weaker ones,
chance alignments and repeats of a section minutes away, and following
one starts the lane in the wrong place: 8 hits once put lane B of a pack
item 3:50 into its file for the first 33 s of A.

### Other placements: the `O` key

Following every alignment can also mislead. Where the main alignment
ends, the strongest other alignment covering the next stretch takes
over, and it can be a section repeated minutes away: in session 40's
pack, lane B of two rips of one recording (Dead Air, Alien Girl, both
judged `same`) played its section from minutes earlier against the
last minute or two of A. So `placement::aligned_lane` gives a lane
placed by alignments two other placements, `main` and `from_start`, and
the page's `O` key (or the button beside the transport line) goes from
`pieces` to `main` to `from_start` and back, passing over a placement no
lane of the question has:

| Placement | What the aligned lanes play |
|---|---|
| `pieces` (the default) | every alignment, as the producer tiled them |
| `main` | the main alignment's offset and speed throughout, so two rips of one recording play the same moment end to end |
| `from_start` | the file from its start at the main alignment's speed |

Lanes without the placement keep their pieces; the reference lane plays
its own time in every placement, so the playhead's time does not move.
Every question opens with `pieces`, as it opens with speed matching on.
The placement in force shows on the button, in each re-placed lane's
label and in its info line (the piece's offset, speed and label);
markers and shading still show the alignments. An answer's record is
the same in every placement. `questions pack` writes both for lane B and
the plan's extra lanes, `questions shared` for lane B; `questions
detections` and `questions folders` write neither, nor `questions pack`
for a lane B with no alignment to follow.

### A marker

| Field | Type | Meaning |
|---|---|---|
| `at` | seconds | Inside the timeline. |
| `label` | string | Shown on hover and in the readout when the playhead reaches it. |
| `kind` | `listen`, `edge` or `join` | `listen`: a moment worth hearing (a clip's moment, a CD's track boundary, a detection's start). `edge`: where lanes start or stop lining up. `join`: where a lane goes on in another file or another place of its file. Drawn in three colours. Up and Down stop at `listen` and `edge` markers and pass over `join` markers; a click takes any of them. |

### A span

| Field | Type | Meaning |
|---|---|---|
| `start`, `end` | seconds | Inside the timeline, `start < end`. |
| `lanes` | array of lane indices | The lanes shaded. |
| `label` | string | Shown on hover. |

### A choice

| Field | Type | Meaning |
|---|---|---|
| `key` | one lowercase letter | The key that answers. Not a control key of the page: `b j k l m o z` (and Space, Tab, digits and arrows, which are not letters). Unique in the question. |
| `label` | string | Shown in the legend. |
| `value` | string | Written to the log, and what the effect acts on. Unique in the question. |

### `effect`

| `type` | Data | What an answer does |
|---|---|---|
| `log` | none | Appends to the answer log only. |
| `pair_verdict` | `related`: array of two-file arrays | The question's `files` are exactly two, named as the verdicts file names them. `same` and `different` append `same\|different<TAB>a<TAB>b` to the verdicts file, as `pair-review` does, and the log; any other value (`unsure`) the log only. The question shows as answered when the verdicts file judges its pair or a `related` pair, in either order (the rule of `Verdicts::judges`, which `pair-review` uses). Then the same verdict again is logged without a second line, and any other answer, `unsure` included, is refused. The choices' values must be `same`, `different` and optionally `unsure`. |

A new place to write answers is a new `Effect` variant in `schema.rs`
and its code in `effects.rs`; that module's documentation lists what
else assumes the verdicts file today.

## The answer log

`work/audition/answers.jsonl`, one JSON object per line, appended on the
listener's keypress only. A server holds the log's lock while it runs, so a
second `audition` on the same log says so and exits; verdict lines are
written under the verdicts file's lock (ADR 0016, "One writer").

```json
{"seq":1,"time":"2026-10-12T08:15:02Z","action":"answer","question":"pair-3f1c...","kind":"pair","title":"1. Cut join: ...","files":["second-library/a.opus","second-library/b.mp3"],"choice":"different","verdict_line":"different\tsecond-library/a.opus\tsecond-library/b.mp3\n"}
{"seq":2,"time":"2026-10-12T08:16:40Z","action":"undo","undoes":1,"question":"pair-3f1c...","kind":"pair","title":"1. Cut join: ...","files":["second-library/a.opus","second-library/b.mp3"],"choice":"different","verdicts":"removed"}
```

- `seq` counts the records from 1. `verdict_line` holds the exact bytes
  an answer appended to the verdicts file (absent when it appended
  nothing).
- An `undo` names the answer it takes back (`undoes`) and says what
  happened to its verdict line: `removed`, `left` (it was no longer the
  file's whole last line) or `none`. `Z` takes back the latest answer
  in force only if its question is loaded; otherwise it writes nothing
  and names the question.
- An answer writes its verdict line before its log record. If the
  record cannot be written, the line stays without one and the page
  says so; `Z` cannot take that line back.
- A question's answer is its latest `answer` record that no `undo` took
  back; a pair question judged in the verdicts file shows that verdict
  first.

## Examples

Generic file names. A pair (the listening pack's kind):

```json
{
  "schema_version": 1,
  "title": "Listening pack",
  "made_by": "gunfinger-eval questions pack",
  "questions": [{
    "id": "pair-0123456789abcdef",
    "kind": "pair",
    "title": "2. Borderline: Artist - Title ~ 111-artist-title",
    "text": "Is B an edit of A, or another version?",
    "details": ["Clusters' alignment: coverage 79.3% of the shorter file, 3321 hits"],
    "files": ["second-library/album/01-artist-title.mp3", "second-library/compilation/111-artist-title.mp3"],
    "timeline": {"start": 0.0, "end": 420.5, "reference": 0},
    "lanes": [
      {"name": "A", "pieces": [
        {"file": "/Volumes/nas/music/album/01-artist-title.mp3", "start": 0.0, "end": 420.5, "offset": 0.0, "speed": 1.0}]},
      {"name": "B", "pieces": [
        {"file": "/Volumes/nas/music/compilation/111-artist-title.mp3", "start": 9.4, "end": 40.2, "offset": -9.4, "speed": 0.9984, "label": "alignment 3"},
        {"file": "/Volumes/nas/music/compilation/111-artist-title.mp3", "start": 40.2, "end": 334.6, "offset": -21.99, "speed": 0.9984, "label": "alignment 1"},
        {"file": "/Volumes/nas/music/compilation/111-artist-title.mp3", "start": 334.6, "end": 420.5, "offset": -66.6, "speed": 0.9984, "label": "alignment 2"}],
       "main": [
        {"file": "/Volumes/nas/music/compilation/111-artist-title.mp3", "start": 22.03, "end": 374.79, "offset": -21.99, "speed": 0.9984, "label": "alignment 1, throughout"}],
       "from_start": [
        {"file": "/Volumes/nas/music/compilation/111-artist-title.mp3", "start": 0.0, "end": 352.76, "offset": 0.0, "speed": 0.9984, "label": "from the file's start"}]}
    ],
    "markers": [
      {"at": 40.2, "label": "alignment 1 starts", "kind": "edge"},
      {"at": 176.1, "label": "control: inside alignment 1", "kind": "listen"}
    ],
    "spans": [{"start": 40.2, "end": 320.0, "lanes": [0, 1], "label": "alignment 1: 3324 hits"}],
    "choices": [
      {"key": "s", "label": "same recording", "value": "same"},
      {"key": "d", "label": "different recordings", "value": "different"},
      {"key": "u", "label": "unsure", "value": "unsure"}
    ],
    "effect": {"type": "pair_verdict", "related": []}
  }]
}
```

A folder (is it a mixed CD?), one lane of excerpts:

```json
{
  "id": "folder-89abcdef01234567",
  "kind": "folder",
  "title": "Artist - Album",
  "text": "Is this a mixed CD (a DJ mix: each track blended into the next), a continuous CD (a few seconds of crossfade between otherwise separate tracks), or separate tracks?",
  "details": ["/Volumes/nas/music/artist-album/", "3 tracks, 2 boundaries: the last 20 s of each track, then the first 20 s of the next"],
  "files": ["second-library/artist-album/01-intro.mp3", "second-library/artist-album/02-tune.mp3", "second-library/artist-album/03-outro.mp3"],
  "timeline": {"start": 0.0, "end": 80.0},
  "lanes": [{"name": "CD", "pieces": [
    {"file": "/Volumes/nas/music/artist-album/01-intro.mp3", "start": 0.0, "end": 20.0, "offset": 41.3, "speed": 1.0, "label": "track 1, last 20 s"},
    {"file": "/Volumes/nas/music/artist-album/02-tune.mp3", "start": 20.0, "end": 40.0, "offset": -20.0, "speed": 1.0, "label": "track 2, first 20 s"},
    {"file": "/Volumes/nas/music/artist-album/02-tune.mp3", "start": 40.0, "end": 60.0, "offset": 280.0, "speed": 1.0, "label": "track 2, last 20 s"},
    {"file": "/Volumes/nas/music/artist-album/03-outro.mp3", "start": 60.0, "end": 80.0, "offset": -60.0, "speed": 1.0, "label": "track 3, first 20 s"}]}],
  "markers": [
    {"at": 20.0, "label": "track 1 → track 2", "kind": "listen"},
    {"at": 40.0, "label": "track 2: from its start to its end", "kind": "join"},
    {"at": 60.0, "label": "track 2 → track 3", "kind": "listen"}
  ],
  "spans": [],
  "choices": [
    {"key": "y", "label": "mixed CD", "value": "mixed"},
    {"key": "c", "label": "continuous: short crossfades", "value": "continuous"},
    {"key": "n", "label": "separate tracks", "value": "separate"},
    {"key": "u", "label": "unsure", "value": "unsure"}
  ],
  "effect": {"type": "log"}
}
```

A detection (is it right?), the mix as the reference lane:

```json
{
  "id": "detection-fedcba9876543210",
  "kind": "detection",
  "title": "Play 4 at 12:03: Artist - Title (confident)",
  "text": "Is this the track playing in the mix here?",
  "details": ["confident, 1484 hits in 15 windows, turntable at +6.60%", "Mix 12:03–14:30"],
  "files": ["/Users/me/mixes/mix.mp3", "second-library/label/Artist - Title.opus"],
  "timeline": {"start": 693.0, "end": 900.0, "reference": 0},
  "lanes": [
    {"name": "mix", "pieces": [{"file": "/Users/me/mixes/mix.mp3", "start": 693.0, "end": 900.0, "offset": 0.0, "speed": 1.0}]},
    {"name": "track", "pieces": [{"file": "/Volumes/nas/music/label/Artist - Title.opus", "start": 693.0, "end": 900.0, "offset": -770.4, "speed": 1.066, "label": "segment 1"}]}
  ],
  "markers": [
    {"at": 723.0, "label": "the play starts", "kind": "listen"},
    {"at": 870.0, "label": "the play ends", "kind": "edge"}
  ],
  "spans": [{"start": 723.0, "end": 870.0, "lanes": [0, 1], "label": "segment 1: confident, 1484 hits in 15 windows, speed +6.60%"}],
  "choices": [
    {"key": "y", "label": "right", "value": "confirmed"},
    {"key": "n", "label": "wrong", "value": "wrong"},
    {"key": "u", "label": "unsure", "value": "unsure"}
  ],
  "effect": {"type": "log"}
}
```

Shared material (two different recordings), A as the reference lane and
B following each alignment:

```json
{
  "id": "shared-0123456789abcdef",
  "kind": "shared",
  "title": "1. Artist - Tune ~ Other - Remix",
  "text": "A and B are different recordings that line up where the lanes are shaded. Do they share material there, such as a drum break, a loop or a sample, or is the match chance?",
  "details": ["B follows every alignment of the search \"audio, both ladders\" in the pair report, strongest first, also those below the 60 hits of a possible play:", "alignment 1: A 1:52-2:38 ~ B 0:34-1:18, 62 hits in 5 windows, B played at -4.13% (key-locked: B's tempo differs, not its pitch)"],
  "files": ["Artist - Tune.mp3", "second-library/label/Other - Remix.opus"],
  "timeline": {"start": 0.0, "end": 425.4, "reference": 0},
  "lanes": [
    {"name": "A", "pieces": [{"file": "/Users/me/library/Artist - Tune.mp3", "start": 0.0, "end": 425.4, "offset": 0.0, "speed": 1.0}]},
    {"name": "B", "pieces": [{"file": "/Volumes/nas/music/label/Other - Remix.opus", "start": 76.5, "end": 425.4, "offset": -73.33, "speed": 0.9587, "label": "alignment 1"}]}
  ],
  "markers": [
    {"at": 112.4, "label": "alignment 1 starts: 62 hits", "kind": "listen"},
    {"at": 158.1, "label": "alignment 1 ends", "kind": "edge"}
  ],
  "spans": [{"start": 112.4, "end": 158.1, "lanes": [0, 1], "label": "alignment 1: A 1:52-2:38 ~ B 0:34-1:18, 62 hits in 5 windows, B played at -4.13% (key-locked: B's tempo differs, not its pitch)"}],
  "choices": [
    {"key": "s", "label": "shared material (a break, loop or sample)", "value": "shared"},
    {"key": "c", "label": "chance", "value": "chance"},
    {"key": "u", "label": "unsure", "value": "unsure"}
  ],
  "effect": {"type": "log"}
}
```

## What the page asks the server

All paths start with `/<token>/`; `api/` routes answer JSON unless noted.

| Request | Answer |
|---|---|
| `GET /` | the page; `GET <name>.js` and `<name>.css` its embedded files |
| `GET api/session` | the loaded files' titles, every question with its state, the window length, the paths of the answer log and the verdicts file |
| `GET api/audio?file=PATH&window=K` | `audio/wav`: window `K` (30 s) of a listed file, 16-bit stereo, 44.1 kHz; `404` past the file's end, `503` when it cannot be decoded now (the page tries again every 3 s and waits) |
| `GET api/envelope?file=PATH` | a listed file's waveform: per 50 ms bucket the minimum, the maximum and the RMS level (each scaled to -127..127), and its length |
| `POST api/answer` `{"question": ID, "key": K}` | the question's new state and a message, or `409` with the reason |
| `POST api/undo` | what was undone and a message |
| `POST api/reveal` `{"file": PATH}` | shows a listed file in the file manager: on macOS the Finder selects it (`open -R FILE`), on Linux `xdg-open FOLDER` opens its folder (ADR 0020); `500` names the command when it fails |

## What a window costs

Measured in session 14 (see `docs/status.md`). The server answers a 30 s
window (the WAV is 5.3 MB: 30 s × 44,100 × 2 channels × 2 bytes) in
0.06-0.11 s: a NAS mp3 0.07 s, a NAS opus 0.11 s, a local m4a 0.06 s
(FFmpeg alone took 0.09-0.23 s on a first try during the design). In the
page a window costs 70-105 ms to fetch and 10 ms to decode; its buffer
is 10.6 MB (32-bit floats). The page keeps at most 24 windows (about
250 MB; a folder question reached 242 MB) and evicts the least recently
used one that the moment being played does not need. After a far jump,
the first sound waited about a second in the page checks while the
windows loaded.
