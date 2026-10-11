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
```

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
| `kind` | string | Lowercase letters and hyphens: `pair`, `folder`, `detection`. Shown in the queue and written to the log; the page does nothing else with it. |
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
| `pieces` | array | What the lane plays where. In timeline order, not overlapping, inside the timeline. Gaps are silent. |

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

### A marker

| Field | Type | Meaning |
|---|---|---|
| `at` | seconds | Inside the timeline. |
| `label` | string | Shown on hover and in the readout when the playhead reaches it. |
| `kind` | `listen`, `edge` or `join` | `listen`: a moment worth hearing (a clip's moment, a CD's track boundary, a detection's start). `edge`: where lanes start or stop lining up. `join`: where a lane goes on in another file or another place of its file. Drawn in three colours. |

### A span

| Field | Type | Meaning |
|---|---|---|
| `start`, `end` | seconds | Inside the timeline, `start < end`. |
| `lanes` | array of lane indices | The lanes shaded. |
| `label` | string | Shown on hover. |

### A choice

| Field | Type | Meaning |
|---|---|---|
| `key` | one lowercase letter | The key that answers. Not a control key of the page: `b j k l m z` (and Space, Tab, digits and arrows, which are not letters). Unique in the question. |
| `label` | string | Shown in the legend. |
| `value` | string | Written to the log, and what the effect acts on. Unique in the question. |

### `effect`

| `type` | Data | What an answer does |
|---|---|---|
| `log` | none | Appends to the answer log only. |
| `pair_verdict` | `related`: array of two-file arrays | The question's `files` are exactly two, named as the verdicts file names them. `same` and `different` append `same\|different<TAB>a<TAB>b` to the verdicts file, as `pair-review` does, and the log; any other value (`unsure`) the log only. The question shows as answered when the verdicts file judges its pair or a `related` pair (`Verdicts::judges`). The choices' values must be `same`, `different` and optionally `unsure`. |

A new place to write answers is a new `Effect` variant in `schema.rs`
and its code in `answers/effects.rs`.

## The answer log

`work/audition/answers.jsonl`, one JSON object per line, appended on the
listener's keypress only:

```json
{"seq":1,"time":"2026-10-12T08:15:02Z","action":"answer","question":"pair-3f1c...","kind":"pair","title":"1. Cut join: ...","files":["second-library/a.opus","second-library/b.mp3"],"choice":"different","verdict_line":"different\tsecond-library/a.opus\tsecond-library/b.mp3\n"}
{"seq":2,"time":"2026-10-12T08:16:40Z","action":"undo","undoes":1,"question":"pair-3f1c...","kind":"pair","title":"1. Cut join: ...","files":["second-library/a.opus","second-library/b.mp3"],"choice":"different","verdicts":"removed"}
```

- `seq` counts the records from 1. `verdict_line` holds the exact bytes
  an answer appended to the verdicts file (absent when it appended
  nothing).
- An `undo` names the answer it takes back (`undoes`) and says what
  happened to its verdict line: `removed`, `left` (it was no longer the
  end of the file) or `none`.
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
        {"file": "/Volumes/nas/music/compilation/111-artist-title.mp3", "start": 334.6, "end": 420.5, "offset": -66.6, "speed": 0.9984, "label": "alignment 2"}]}
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
  "text": "Is this a mixed CD (each track runs into the next)?",
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
  "title": "Play 4 at 12:03: Artist - Title",
  "text": "Is this the track playing in the mix here?",
  "details": ["confident, 1484 hits in 15 windows, turntable at +6.60%"],
  "files": ["/Users/me/mixes/mix.mp3", "second-library/label/Artist - Title.opus"],
  "timeline": {"start": 693.0, "end": 900.0, "reference": 0},
  "lanes": [
    {"name": "mix", "pieces": [{"file": "/Users/me/mixes/mix.mp3", "start": 693.0, "end": 900.0, "offset": 0.0, "speed": 1.0}]},
    {"name": "track", "pieces": [{"file": "/Volumes/nas/music/label/Artist - Title.opus", "start": 693.0, "end": 900.0, "offset": -770.4, "speed": 1.066}]}
  ],
  "markers": [{"at": 723.0, "label": "play starts", "kind": "edge"}],
  "spans": [{"start": 723.0, "end": 870.0, "lanes": [0, 1], "label": "the play: 1484 hits"}],
  "choices": [
    {"key": "y", "label": "right", "value": "confirmed"},
    {"key": "n", "label": "wrong", "value": "wrong"},
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
| `GET api/audio?file=PATH&window=K` | `audio/wav`: window `K` (30 s) of a listed file, 16-bit stereo, 44.1 kHz |
| `GET api/envelope?file=PATH` | a listed file's minimum and maximum per 50 ms bucket, and its length |
| `POST api/answer` `{"question": ID, "key": K}` | the question's new state and a message, or `409` with the reason |
| `POST api/undo` | what was undone and a message |
| `POST api/reveal` `{"file": PATH}` | reveals a listed file in the Finder (`open -R`) |

## What a window costs

Measured in session 14 (see `docs/status.md`): FFmpeg decodes a 30 s
window of a NAS file in 0.1-0.25 s (mp3 0.13 s, opus 0.23 s, m4a
0.09 s); the WAV is 5.3 MB (30 s × 44,100 × 2 channels × 2 bytes); the
page's decoded buffer is 10.6 MB (32-bit floats). The page's cache holds
at most 24 windows (about 250 MB).
