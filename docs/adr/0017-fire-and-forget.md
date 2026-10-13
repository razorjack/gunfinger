# 0017: A tool you fire and forget

## Status

Accepted (2026-10-13, session 16). Records the product principle from our prompts
in their words, the scope and priorities that follow from it, the default
output of `identify` and `show`, and how the tracklist groups plays into
one entry per track. It ends two roadmap items for
the product ("Report edits", "The Track/AudioAsset model") and limits a
third ("Titles of mislabelled files") to flagging.

## Context

From the prompt, 2026-10-13:

> I want gunfinger to be something you fire, get results and forget. Not
> something you work with. If multiple files are matched for one track
> detected in the DJ set, the user will check all of them and most likely
> find the correct title among the detected files. If the user has
> mis-managed tags or library, it's their fault, and gunfinger should not
> be a tool to fix the library. It's not a set detection studio. Users are
> responsible for tagging their files, and the quality of titles detected
> by gunfinger follows the quality of the data they indexed.

Sessions 10 to 15 built tools to judge by ear (`pair-review`, the
listening page of ADR 0016), and the roadmap named the next step: edits
of reports (confirming, rejecting or renaming plays, kept when a mix is
identified again) and an editable model of tracks above files. Both
would make Gunfinger something the user works with.

## Decision

### Scope

- Gunfinger identifies tracks in a recording and reports them. The user
  runs it, reads the result and is done. Between runs it keeps only the
  peak store (with the tags it read) and the saved index.
- Out of scope for the product:
  - **Report edits**: confirming, rejecting or renaming plays, and
    keeping such decisions across identifications. Nobody reviews the
    output, so it must be right as printed.
  - **The Track/AudioAsset model**: an editable grouping of files into
    tracks. It would make the user curate the library inside Gunfinger.
  - **Fixing the library**: editing or suggesting tags, choosing between
    files' disagreeing titles, renaming or moving files. Titles come from
    the user's tags, or from file names where a file has no title tag;
    their quality is the user's responsibility.
- The listening page (`gunfinger-eval audition`) and `pair-review` stay
  development tools. They settle ground truth for evaluation (pair
  verdicts, duplicate clusters, judgments of release folders), and
  nothing in the product reads their answers. `listen` and `review`
  stay as they are: they play a saved report and change nothing.

### Priorities

1. **Precision.** Nobody checks the output, so a confident wrong title is
   the worst result. The confidence rule stays frozen; anything that adds
   evidence faces the shared-material challenge first (roadmap, "A
   confidence statistic for shared material").
2. **Every matching file, disagreeing titles flagged, nothing chosen.**
   When several files match one passage of the mix, the output names all
   of them, and the user recognises the right title among their names.
   Where the files' tags give different titles, the output says so
   without choosing one, and it never rewrites a tag.
3. **Unattended reliability.** A run ends with results or a clear error.
   One bad or unreachable file fails alone, and nothing waits forever.
4. **Speed.**

### The default output

`identify` and `show` print the tracklist unless `--format` says
otherwise (from session 16; agreed): one numbered line per track
with its start in the mix, named from the strongest file's tags, followed
by the names of the other files found there and a flag where their titles
disagree. The table of plays (`--format human`) stays for every play,
segment, speed and file path; JSON stays the format for programs and for
`show`.

- **The table keeps its name, `human`, with no alias.** A clearer name
  (`table`, `plays`) with `human` kept as an alias would give one format
  two names in the help, the completions and the man page, and `stats
  --format human` would still use the old word. The help text now says
  what the table holds.
- **Several recordings** keep a heading `== <path>` before each one's
  output, as before; `--save-dir` writes each JSON report and prints the
  tracklist.
- **Nothing found** prints `no tracks found`, so an empty result does not
  look like a run that printed nothing.
- **No colour in the tracklist**, also on a terminal: it is the text the
  user copies. `--color` still colours the table, the timeline, the
  headings and messages; `--quiet` silences stderr only.
- **Programs read JSON.** No script or harness code parses the default
  output; `gunfinger-eval` calls the core directly.

### One entry per track

The tracklist and the cue sheet list tracks, not plays. Two plays are
one entry when they overlap for at least half of the shorter play (the
overlap test of `same_recording`) and either:

- they are one recording on two records: with each file's own speed taken
  out, the track would have started at the same mix time, within 5 s
  (`same_recording`, unchanged since session 11); or
- their files' tags give the same artist and the same title, compared by
  their letters and digits in lower case as `other_titles` compares
  titles, however the two line up (`same_tagged_track`, from session 16).

A chain of such pairs is one entry too, whatever order the plays come in.
The rule needs both an artist and a title tag on both files, compared as
the files carry them in the report; a file without them would be named by
its file name, which is never read for titles. A possible play marked as
sharing material with another recording is never grouped, by either test.
Tags are never rewritten. The entry is named after its strongest play (a
confident one first, then the most hits), and the other names the files
give follow it (`also:`, or `titles disagree:` when the files of the
entry's confident plays carry more than one title between them, compared
as for `other_titles`); names that differ only in letter case are listed
once, as before. The table of plays, CSV and JSON keep every play.

Why: the 2003 development mix at NAS scale listed DJ Hidden - Skektics
twice (42:13 and 42:15). An upload with about 40 s more lead-in than the
vinyl rips is the same track over the same passage, but its alignment puts
the track's start 41.7 s away, so `same_recording` keeps it apart. By our
rule the tags name the track: two files over one passage with the
same artist and title are one track for the reader, whether they are one
recording, an edit or a re-press. Files tagged with another title over the
same passage stay apart unless they line up, so a remix or a VIP keeps
its own line.

## Consequences

- Future sessions do not propose editing workflows, stored decisions
  about reports, tag fixing or ways to pick one of several titles. Work
  goes into what is found, how precisely, how clearly it is listed, how
  reliably a run finishes, and how fast.
- Duplicate clusters, pair verdicts and the answer log remain the
  evaluation's view of which files are one recording. The product groups
  files into tracklist entries from the evidence of one run, without a
  stored model.
- A badly tagged library gives badly named results. Retagging a file
  changes its modification time, so the next `index` reads its peaks and
  tags again.
