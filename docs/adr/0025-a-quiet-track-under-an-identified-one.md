# 0025: A quiet track under an identified one (`identify --under`)

## Status

Proposed (2026-11-02). Built in session 72 behind `identify --under`,
off by default. The user decided that the step is to be on by default
subject to its cost, and that the cost is measured before that is
decided (session 73). Until then nothing changes without the option:
the search, the confidence rule (ADR 0006, ADR 0013), plays, the entries
(ADR 0017, ADR 0019), the gaps (ADR 0023, ADR 0024), every output and
both regressions are byte-identical. Session 73 measured the cost and
the effect (experiments 0273-0275; "Open points, measured in session
73" below). The status stays Proposed: whether the step is on by default
is our decision.

## Context

In a double drop or a three-deck blend, a quieter record plays under a
louder one. The search finds the louder record confidently and the
quieter one, if at all, as a possible play inside it, which the
tracklist leaves out (ADR 0019). Proposal 4 of
`docs/research/2026-10-29-matching-and-archive-proposals.md` takes the
louder record out of the mix and searches what is left:

- Session 65 (experiments 0255-0258) measured the upper bound on
  generated double drops: the generator's time map, the played file as
  the reference, a spectral mask with gains fitted per band and block
  of about 1 s, each bin losing twice the reference's estimated power
  (`over` 2). With another rip as the reference, as in a real mix, the
  mask left the louder record behind in about a quarter of the cases at
  NAS scale.
- Session 70 (experiments 0269-0271) found why (rips of another
  transfer, 0.1-0.2% off in speed, placed tens of milliseconds off by
  their stored peaks' line), aligned the reference on the mixture before
  the gains are fitted (*Align*: lags per block of 8 s on band levels, a
  Theil-Sen line, refined in 2 ms steps), and took the louder record's
  time map from the whole-mix detection instead of the generator. On two
  fresh test seeds at NAS scale the residual confirmed 61 of 64 quiet
  and brief partners (the plain excerpt 41), 46 of them confidently
  (plain 15), with no wrong confident answer. A rule that needs evidence
  in the original mixture removed no wrong answer and dropped 10 of
  those partners; it is not used.

On 2026-11-02 the user answered the questions this decision needs
(`docs/status.md`, "2026-11-02: the user's answers on a quiet track
under an identified one"): on by default in `identify`, subject to its
cost; today's acceptance rule; the printing to be tried in use; related
versions not printed at first, since Gunfinger is for learning a title;
the cost measured before the default is decided; the best-aligned rip of
the louder track as the reference.

## Decision

1. **The option.** `gunfinger identify --under` runs the step; without
   it nothing runs. There is no configuration key yet: one can follow
   the decision on the default.
2. **Where it runs.** After the report's plays are grouped and marked,
   once for each confident tracklist entry (ADR 0017's entries, so two
   rips of one recording found over the same passage are taken out
   once).
3. **The reference, the best-aligned rip.** The entry's strongest
   confident play (the most hits): of the files of the louder track that
   the search found there, the one the recording's audio supports best.
   The product has no duplicate clusters (a harness file), so the rips it
   can choose from are the entry's files.
4. **The time map.** Each of that play's segments places its file by its
   own line (file start, speed, turntable or key lock) over its span.
   The excerpt is the play's span; between segments nothing is taken
   out.
5. **The audio.** The file is decoded from the library through FFmpeg,
   seeking to each segment's start and changing its speed as the
   harness renders mixes (`decode::Decoder::render`).
6. **The mask.** The core's `residual` with `Mask::UNDER`: *Align*,
   `over` 2, gains per 63 frames (about 1 s). The harness's residual
   calls the same code, so the measured code is the shipped code.
7. **The acceptance rule.** The residual is searched with the same
   index, ladder and matcher; today's confidence rule applies unchanged.
   Weak detections are left out, as plays leave them out.
8. **What is dropped.** A detection in the residual is not reported when
   its file is one of the entry's files; or shares material with the
   file taken out, by their stored peaks (the file's peaks searched
   against that file alone give a detection at the possible tier or
   above, which ADR 0006 reads as "this recording, or one sharing
   material with it"); or has a confident play in the recording that
   overlaps it or lies within 90 s of it (the tracklist names the track
   there already). The kept detections are grouped into plays, and a
   play is dropped when it is one track with a confident play of the
   recording that overlaps it or lies within 90 s of it, by the tests of
   ADR 0017's entry rule (the two line up, or their files are tagged
   alike): another upload of a track the tracklist names there. The
   shared-material rule is the product's reading of the user's answer
   that related versions are not printed at first: a related version of
   the quieter track cannot be told from the right recording, while one
   of the louder track can be found in the library's own audio.
9. **`--store-only`.** The step needs the library's audio; when the
   store's own records are searched (`--store-only`, no library, or a
   library that cannot be read), it does not run, and `identify` says so
   in one line. A file that cannot be decoded is passed over with a
   warning.
10. **The printing, to be judged in use.** Plays found under others are
    grouped into tracks as plays are (ADR 0017). A confident one gets its
    own tracklist entry at its start, marked `(under N.)` with the number
    of the louder track's entry. One found
    only as possible stays out of the tracklist, as ADR 0019 leaves such
    plays out. The table shows every such play in time order, marked
    `under <file> (play N)`; CSV gains a row per such play and two last
    columns, `play` (a play's number in the report, empty for a found
    play) and `under_play`, only when the step ran; JSON holds them in `under`,
    each with `under: {play, asset}`, and `search.under` records the
    step's setting, so `--save-dir` searches again when it changes. The
    cue sheet and the timeline leave them out (a cue sheet cannot hold
    two tracks at once). Saved reports, older or newer, stay readable by
    `show`.

## Consequences

- Reference audio is required: the library must be readable while
  `identify` runs. A copy of the peak store alone cannot take a track
  out.
- Each confident entry costs a decode of part of its file, the mask (an
  STFT, the lag measure on eight phases, the inverse), one search of its
  span and a peaks search per file found there. Session 72's first look
  (experiment 0272): 1.0-3.0 s per confident entry, 5-6.5 times
  `identify`'s time at 262 tracks and 2.4-2.8 times at NAS scale on the
  Linux machine (32 workers). Session 73's measure is below.
- A related version of the louder track, or a track built on the same
  sampled break, is held back by the shared-material rule even when it
  was played; a related version of the quieter track can still be
  printed as that track. On 2026-10-31 the user found that acceptable,
  since a listener who learns a title can find the right version by ear
  (`docs/status.md`, "the last answer, and the user's decisions and
  views").
- When the quieter track goes on alone after the louder one is cut, the
  tracklist already names it (through the same file or another that
  lines up with it or is tagged alike), and its earlier start under the
  louder track is not shown.
- A quieter track found as possible in the plain search and confidently
  under the louder track appears twice in the table: the possible row
  marked `shares material with`, and the row marked `under`.
- Not measured: real mixes' double drops (no truth beyond listening),
  broadcast processing and pitch rides under the louder track.

## Open points, measured in session 73

Session 73 measured the option as shipped against the default on the
same inputs, under criteria registered before the runs
(`docs/brief-73.md`: J1 precision, J2 benefit, J3 cost at most twice the
default's time at NAS scale, J4 reliability).

1. **Precision (J1 met).** No new wrong line and no wrong confident find:
   the three development mixes at both scales and Stakka's 14 leave-out
   patterns at both scales gain no tracklist entry (one possible find
   on the razorjack mix at NAS scale, the next track 75 s before its own
   entry, stays out of the tracklist); 36 generated mixes at NAS scale
   gain 38 entries, all right, and none of 56 confident finds is wrong
   (0273, 0274). No entry is lost and no gap line changes anywhere.
2. **Benefit (J2 met).** On 32 generated double-drop mixes the tracklist
   names 18 of 32 quiet partners (default 2), 18 of 32 brief ones
   (default 6) and 29 of 32 unrelated partners inside the louder record
   (default 20); a gained entry starts a median 1.6 s after the
   partner (0274). Related versions (pair drops) are held back, as
   decided.
3. **Cost (J3 not met).** At NAS scale `identify` takes 2.8-2.9 times
   as long (the step adds 29-40 s to a mix of 46-67 min, about 35 s per
   hour of mix, 2.4-2.9 s per confident entry) and 0.1-0.2 GB more peak
   memory; at 262 tracks 3.2-5.1 times (12-25 s per hour of mix). The
   mask, on one thread, takes 40-70% of the step, the residual's search
   14-42%, the decode 12-23%. No budget by the louder play's length or a
   cap per mix halves the step's time while keeping 90% of the gained
   entries (0275).
4. **Reliability (J4 met).** Every run ended with status 0; no reference
   failed to decode.
5. **The printing.** One gained entry repeated a track the default
   prints over the same passage (another upload, 6.7 s apart in line and
   tagged differently, so ADR 0017 keeps it apart). The fourth drop
   rule's start test, without the overlap condition, dropped one right
   partner whose line happened to start within 0.5 s of another track's
   confident play 36 s earlier. Both are open for us; neither printed a
   wrong title.
6. **For our decision.** By the registered criteria the step would stay
   off by default until it is faster (J3), the cost being the mask's
   single thread above all. Turning it on as it is would cost about 35 s
   per hour of mix at NAS scale on the Linux machine for the gains in 2;
   speed comes last in the product's order, so that is a choice we can
   make with these numbers.

