# 0019: Possible plays inside a confident play leave the tracklist

## Status

Accepted (2026-10-14, our decision). Amends ADR 0017's default
output and the shared-material mark of session 4. The confidence rule,
the possible tier (ADR 0006), plays, scoring and the report's schema are
unchanged.

## Context

The new README showed the Stakka & Skynet mix's tracklist with
"6. 20:22 Stakka And Skynet - Clockwork (Stakka Rmx) (possible; shares
material with Stakka & Skynet - Clockwork)". Our note: the mix plays the
original Clockwork; the Stakka remix was made later and did not exist
then, so the line is a misidentification.

The line is a 94-hit match over 18 s (remix 6:39-6:58) inside the
original's single segment: a passage of the remix that sounds like the
original. Since session 4 a possible play lying between the first and
last second of a confident play of another file is marked "shares
material with" and printed as an entry of its own. In a tool that is
fired and forgotten (ADR 0017), a line of the tracklist reads as a track
that was played, whatever its note says.

Every possible line of the three development mixes' default tracklists
at NAS scale is of this kind, 9 in all, and none is a played track
(experiments 0092, 0100, 0102). The whole-span test also marks the wrong
plays: `plays::group` joins a returning track across gaps of up to 90 s,
so a brief play between its two parts lies inside its span. In the
generated mixes it marks 6 right brief plays (experiment 0107).

## Decision

1. **The mark tests passages.** A possible play shares material with a
   confident play of another file (not an identical copy) when it lies
   entirely inside one passage of that play: its segments joined across
   gaps of at most 10 s (`PASSAGE_GAP_SECONDS`, experiment 0091's
   `passage`). A possible play of the track playing there is not marked:
   when a confident play overlapping most of it lines up with it (the
   track would have started within 5 s of the same mix time) or carries
   the same artist and title, it joins that entry by the entry rule (ADR
   0017), and its file is named under `also:` or `titles disagree:`.
2. **The tracklist leaves marked plays out.** A marked play is never
   grouped (ADR 0017), so it is an entry of its own; `identify` and
   `show` print the tracklist without it. There is no footer saying
   something was left out.
3. **The rest keeps it.** The table (`--format human`) shows the play
   with "shares material with <file> (play N)", CSV with
   `shares_material_with_play`, JSON with `shares_material_with`. The cue
   sheet already lists confident entries only.
4. The mark is derived whenever a report is made or read, so saved
   reports show the new tracklist through `show` without a new search.

## Consequences

- The default tracklists of the three development mixes at NAS scale
  lose 7 of those 9 lines; every other line stays (experiment 0107). The
  two Klute Remix lines of the Vertical mix line up with the confident
  VIP within 1 s, so they join its entry and the remix is named under
  `titles disagree:`, as related versions that line up already are.
- No right play is lost in any stored development report (experiment
  0107, counted per file): the development scans and leave-outs at both
  sizes, the generated mixes (their 6 right brief plays are no longer
  marked and stay as plain possible lines), the sweeps and recall. There,
  every right play the passage test marks lies inside a confident play of
  its own recording (another rip or upload): it joins that entry when it
  lines up or is tagged alike, and otherwise leaves the tracklist while
  the confident play's line still names the track.
- A track that was played can leave the tracklist when it is found only
  as possible and lies inside a passage of a confident play of another
  track, such as a double drop under a stronger track or a tease that
  never returns. None occurs in the stored development reports; the
  table still shows such a play.
- Not changed: a confident play of a related version is still listed
  (the containment rule, experiments 0091 and 0101, is not adopted); a
  possible play that reaches past the confident play, or plays where
  nothing confident plays, is still listed as possible.
- A user who wants the related versions reads the table.
