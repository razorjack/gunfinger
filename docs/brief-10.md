# Session 10 brief: the owner's verdicts at NAS scale, a new NAS baseline

Written by the agent for itself on 2026-10-10, so the plan survives context
compaction. Work autonomously and never stop to ask. When something is
ambiguous, take the conservative option, write the decision down and
continue.

## Read after every compaction

1. `docs/brief.md` (the original brief; never edit it).
2. This file, and `docs/session-10-checklist.md` (progress).
3. `docs/notes-for-owner.md` ("After session 7" to the end).
4. `docs/status.md` (newest entries at the bottom).
5. The newest file in `docs/experiments/`.
6. `docs/pair-verdicts.txt`, `docs/calibration.md`, ADRs 0004, 0009, 0010.

`docs/brief-2.md` to `docs/brief-9.md` hold rules that still apply unless
this brief changes them (among them: edit no configuration file; while a
timed run goes, only light work, building and tests wait; test fixtures
are generated, never copied from the corpus or the NAS).

## The owner's prompt, verbatim

```text
Start session 10 of Gunfinger: the owner's verdicts at NAS scale, a new
NAS baseline, the 2003 mix in the harness, and recall beyond the corpus
recordings. The session may run up to 10 hours, unattended. The laptop
is on Ethernet with the NAS mounted at /Volumes/atlas. Work autonomously
and never stop to ask. When something is ambiguous, take the
conservative option, write the decision down and continue.

First:
1. Read CLAUDE.md, AGENTS.md, docs/brief.md (never edit it),
   docs/brief-9.md (brief-2 to brief-8 hold rules that still apply
   unless this prompt changes them), docs/notes-for-owner.md (from
   "After session 7" to the end), docs/status.md (newest entries at the
   bottom), docs/pair-verdicts.txt, docs/roadmap.md ("More development
   mixes", "Recall beyond the corpus recordings", "Clustering
   precision", "Owner edits", "No record inside the track length
   range"), docs/calibration.md, ADRs 0004, 0009 and 0010, and
   experiments 0049, 0051, 0053, 0057, 0059, 0060 and 0062-0065.
2. `git status` shows uncommitted changes to docs/notes-for-owner.md and
   docs/status.md: the owner's mixed-CD judgments and their prune,
   written after session 9 and reviewed. Run scripts/check.sh and commit
   them as one commit before anything else.
3. Write docs/brief-10.md (this prompt verbatim, the state at the start,
   the plan with times) and docs/session-10-checklist.md. Point
   CLAUDE.md's reading list at them; keep brief-9 among the older
   briefs. Commit.
4. Build with --release. Run `gunfinger-eval baseline session-10-start`.
   Record the NAS peak store's state (scripts/analysis/store_revision.py
   and store_census.py): 32,441 records, 1,558 skip notes and revision
   5701f221f7b48ee1 expected. `doctor --config
   ~/.config/gunfinger/nas-dnb.toml` should read 31 ignore patterns and
   report no records of ignored or gone files.

Rules for this session:
- The NAS (/Volumes/atlas/Music/dnb) is read, never written. Reading
  and decoding its audio over Ethernet is allowed and expected.
- The NAS peak store is read-only: no `index` or `prune` against it. Its
  revision stays 5701f221f7b48ee1; check it at the end.
- Never touch the held-out test set (ed-rush-optical-essential-mix), and
  never identify, decode or index EM_1999_02_07_-_Ed_Rush_Optical.mp3 at
  the NAS root. Run nothing on the candidate mixes of experiment 0065.
- Verdicts are the owner's: never add or change a line in
  docs/pair-verdicts.txt.
- Manifests do not change, except as the owner allows below.
- The owner allows one manifest edit: in
  corpus/sets/razorjack-2003-03-29/tracklist.toml, turn the five `# nas:`
  comments into references (`second-library/<path>`, as in
  docs/pair-verdicts.txt) once item 2b can read them, and change nothing
  else. Without this permission the five tracks stay absent at both
  sizes.
- The matcher and the confidence rule do not change. Detection at 262
  tracks does not change: `regress session-10-start` must be identical
  after every code change.
- One heavy run at a time (clusters, sweeps, scans). While one runs,
  write code and documents but start no other search. Record wall time,
  CPU time and peak memory for every run.
- Each measurement is an experiment (docs/experiments/NNNN-slug.md, at
  most 40 lines; data with file names only in docs/experiments/data/).
  Commit after each item; check `git status` before every commit and
  never use `git add -A`.
- Time: items are in priority order. At 9 hours, stop what is running
  at a safe point, write down what is left and do the wrap-up.

Items, in order:

1. Setup (25 min): the steps above.

2. Code needed before the one clusters run (about 2 h 15 min):
   a. Verdict links (roadmap "More development mixes", step 2): a `same`
      verdict adds a link only between files of the libraries the run
      searches, and never to a file the ignore file leaves out. A
      verdict naming any other file is reported (path and line) and
      adds nothing. Today the Synthesis VIP verdict names an ignored
      Dangerous Drums 2 file, and a corpus-only `clusters` run would add
      every NAS file of the verdicts to the corpus clusters. Tests for
      both. (35 min)
   b. Manifests that reference NAS files (step 1): `second-library/
      <path>` references are valid with --other-peaks-dir (checked
      against the NAS store) and count as absent tracks at 262 tracks;
      `validate`, `scan` and scoring handle both sizes. Then make the
      manifest edit if the owner allowed it, and validate the set at
      both sizes. (1 h)
   c. A recall panel drawn from the NAS (roadmap "Recall beyond the
      corpus recordings"), drawn and saved before any result is seen,
      like the sweep panels in docs/panels/ (file names only):
      - sources: NAS records outside the corpus clusters and inside the
        track length range, drawn by seed and spread over the kinds of
        source (scene releases in __full_scene/, YouTube channels in
        __youtube_archivists/, label packs and vinyl rips elsewhere);
      - split into a development and a validation half by recording
        family (the same normalised artist and title stay in one half);
        this session searches only the development half;
      - the development half mirrors a sweep panel: 60 indexed and 20
        held-out sources; the validation half is drawn the same way and
        not searched;
      - excerpts are rendered by FFmpeg from the NAS audio, at the
        sweep's speeds, lengths and encoding.
      Its development sources join the clusters run as queries (2d).
      (45 min)
   d. Extra queries for `clusters` with --other-peaks-dir: every NAS
      file a manifest references, and the panel's development sources,
      join the corpus files as queries. (20 min)

3. The NAS clusters run (about 2 h; 72-85 minutes at 27,042 records in
   experiment 0049, now 32,441 records and about 80 more queries). It
   applies the owner's verdicts and checks them against the last run
   first. Report (experiment): what changed against session 6's
   clusters (the verdicts: China Cup, Coma ~ "Spraycan", The Specialist,
   Synthesis VIP, Alien Girl, fractles and Phoenix joined, the Sonar
   revision apart; the new folders' rips; the pruned mixed CDs); the new
   borderline pairs (40-80%) and sparse joins; the panel sources'
   clusters. While it runs, write without searching:
   - the fix in roadmap "No record inside the track length range"
     (20 min);
   - a listening pack (roadmap "Owner edits", the pair review), about
     1 h: for each borderline pair the owner has not judged, each sparse
     join, and 10 joins drawn by seed as controls, a folder under
     work/listening/ with a sheet (both full paths, under
     ~/Projects/OpenSource/gunfinger/corpus/library/ or
     /Volumes/atlas/Music/dnb/; coverage, supported time, each
     alignment with its times, and where the files stop lining up) and
     short clips rendered by FFmpeg from the files: around each place
     where the files stop lining up, and once inside the main
     alignment as a control, 8 s of one file then the same 8 s of the
     other at the first file's speed, and the two aligned as a stereo
     clip (one file left, the other right). Then
     work/listening/README.md listing every pair, in the order to
     listen.
   After the run: render the pack for the new pairs and list them in the
   notes with their folders.

4. The new NAS baseline (about 2 h 15 min), under the default matcher,
   with the new clusters: development scan, leave-outs 3 and 11 (seed
   2026), sweeps 2026-2029 and `calibrate`; then razorjack-2003-03-29
   at both sizes, with leave-out 3 (seed 2026) at both sizes. Compare
   with the NAS-scale results of experiments 0051, 0053 and 0063, and
   explain each change: the verdicts, the new rips, the pruned mixed CDs
   and the store's growth. Put the baseline in the calibration register.

5. The panel's development half (about 40 min): recall per speed and
   per kind of source, wrong answers, held-out sources' confident
   answers, and for each miss whether it never became a candidate, had
   too few hits in the second pass, or had enough hits in too few
   windows (the harness should say this; add it if it does not).

6. The 2003 mix's experiments (about 45 min, roadmap "More development
   mixes"): Phantom Force and all its rips left out (do Phantom 2018 or
   the Fracture edit become confident?); the PHUD1 rip alone left out
   (does the start move back from 10:48 towards 11:55?); the evidence
   over the Kinetic tease from 17:23.

7. Wrap-up (35 min): experiment files; a "Session 10" section at the end
   of docs/notes-for-owner.md, leading with what the owner must do (the
   listening pack: where it is, how many pairs, in what order);
   docs/status.md, the checklist, the calibration register and the
   roadmap. Check the NAS store's revision. Commit. Last, print for the
   owner what was done, the numbers, the listening pack, the decisions
   waiting for them and what is left.
```

## State at the start of session 10

- 01:17. `git status`: the owner's reviewed changes to
  `docs/notes-for-owner.md` and `docs/status.md`; `scripts/check.sh`
  green (2:29), committed as `9116ec3`. Experiments 0001-0065; ADRs
  0001-0010. Test set: 3 of 5 evaluations used.
- The machine: 10 cores, 32 GB. The default route is `en7` (Ethernet);
  `/Volumes/atlas/Music/dnb` is mounted (935 entries at the root).
- The NAS store (`~/.local/share/gunfinger/nas-dnb-peaks`): 32,441
  `.peaks`, 32,441 `.tags`, 1,558 `.skip`, `library.txt` (revision and
  census recorded in the setup step of the checklist).
- `work/reports/library-nas-dnb-peaks/` holds session 6's NAS clusters
  (`duplicate-clusters.json`, 2026-10-08 07:53, store `d428ee9585936326`,
  27,042 records), the library map of the same store (03:21), and the
  NAS-scale reports of experiments 0051, 0053 and 0059.
- What the code does today (read before planning):
  - `clusters::Verdicts::links` returns every `same` verdict as a link,
    whatever files it names; `clusters::find` (corpus) and
    `clusters::merged` (with `--other-peaks-dir`) add them all. So a
    corpus-only run puts the NAS files of 8 `same` verdicts into the
    corpus clusters, and the Synthesis VIP verdict names
    `second-library/Underfire UDFRCD003 - Dangerous Drums Volume 2 (2000)/CD1/...`,
    which the ignore file leaves out and the prune removed from the store.
  - `find_clusters_around` checks the verdicts against the last run's
    pairs before searching (`check_verdicts_against_last_run`); its
    queries are the corpus records only.
  - `manifest::load_set` checks each reference against the corpus
    library's assets; `validate` does not take `--other-peaks-dir`.
    Scoring credits a track through `Clusters::cluster_of` of its
    references.
  - `sweep::Plan::for_seed` keeps each seed's draw in `docs/panels/`;
    excerpts are rendered from `corpus/library` by `render::render_excerpt`
    (30 s, nine speeds, MP3 128 kbit/s).
  - `pair` lists the alignments of two files at the fitted speed with
    supported time (experiment 0057).

## Plan, with times

Start 01:17; the 9-hour stop is 10:17, the hard end 11:17.

| Step | What | Estimate | Planned end |
|---|---|--:|--:|
| 1 | setup: commit, brief, checklist, build, `baseline session-10-start`, store revision and census, `doctor` | 0:25 | 01:45 |
| 2a | verdict links only between searched, non-ignored files; tests; regress; commit | 0:35 | 02:20 |
| 2b | `second-library/` references in manifests; `validate --other-peaks-dir`; scan and scoring at both sizes; tests; the manifest edit; validate at both sizes; regress; commit | 1:00 | 03:20 |
| 2c | the NAS recall panel: draw and save (development and validation halves by family); render the development half from the NAS | 0:45 | 04:05 |
| 2d | manifest references and the panel's development sources as extra `clusters` queries; tests; regress; commit | 0:20 | 04:25 |
| 3 | `map-library`, then the NAS `clusters` run; meanwhile the track-range message and the listening pack; after it, `pair` and clips for new items, experiment | 2:30 | 06:55 |
| 4 | the NAS baseline: scan, leave-outs, sweeps 2026-2029, `calibrate`; the 2003 mix at both sizes with leave-out 3; experiment, calibration register | 2:15 | 09:10 |
| 5 | the panel's development half, with why each miss failed; experiment | 0:40 | 09:50 |
| 6 | the 2003 mix's three experiments | 0:25 | 10:17 |
| 7 | wrap-up | 0:35 | 10:52 |

Item 6 gets what is left before 10:17; the wrap-up may run past it.

## Conservative decisions taken up front

- The manifest edit changes the five `# nas:` lines of tracks 1, 3, 4, 6
  and 8 (the tracks with `reference = []`) into their references and
  nothing else. The header comment ("Five tracks have no file in
  corpus/library and are `[]`") and the notes "on the NAS only" stay as
  written, though the header is then out of date; the notes say so for
  the owner. The other `# nas:` lines (tracks with corpus references)
  stay comments.
- The files a run searches: without `--other-peaks-dir`, the corpus
  library's assets (`Library::scan` applies its ignore file); with it,
  also the other store's current records, named `second-library/<path>`,
  less the files the other library's ignore file leaves out when the
  store names a library root that can be read. A store-only run cannot
  read an unmounted library's ignore file; it then says so, and only
  records count (a prune removes the records of ignored files). A
  verdict naming any other file, `same` or `different`, is printed with
  its line and adds nothing.
- The library map is built again (`map-library`) before the clusters
  run: the store has changed since session 6's map (27,042 records).
  Session 6's NAS reports are copied to `work/s10/before/` first.
- A borderline pair counts as judged when the owner has a verdict on a
  file of one cluster against a file of the other. The listening pack
  holds one item per pair of clusters, represented by its strongest
  pair, the others listed in the sheet.
- `pair` is a search: it runs only when no other heavy run does. The
  pack's alignments for pairs known from session 6's clusters come from
  a `pair` run before the clusters run; clips are rendered with FFmpeg at
  low priority (`nice`) during the clusters run, as light work.
- The recall panel: the pool is the store's current records whose
  length is within the NAS configuration's range (1:30-15:00) and at
  least 60 s, as the sweep needs, outside session 6's NAS clusters and
  not copies of corpus files (the map built again first). Kinds:
  `__full_scene/`, `__youtube_archivists/`, everything else. Each half
  draws its sources in equal thirds per kind (20 indexed each; 7, 7 and 6
  held out), one per family. A family is the normalised artist and title
  from the store's tags; records without both tags (406) are left out of
  the pool rather than grouped by file name, which AGENTS.md rules out.
  The validation half is drawn and saved, never rendered or searched.
- The extra `clusters` queries come from the manifests of sets named on
  the command line (`--manifest`), not from every manifest, so the
  held-out test set's manifest is never read; `validate` takes a set name
  for the same reason.
