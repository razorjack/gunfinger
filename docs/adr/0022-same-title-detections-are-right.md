# 0022: A confident detection of a near-identical version is right

## Status

Accepted (2026-10-27), our decision. Amends ADR 0004's scoring (decision
5): a confident detection is also right when the file it names is
judged to carry the same title as the truth, and such credits are
counted apart. ADR 0004's clusters, sweep, development set, calibration,
pass bar and the held-out evaluation's rules (freeze, at most five
evaluations, the ledger) do not change. The product does not change.
By our decision this also changes the scoring of `docs/brief.md`,
section 7.6 (a [FIXED] item, "any other confident detection is a wrong
identification"); the brief itself is not edited.

## Context

Gunfinger identifies track titles (ADR 0017): the user reads the
titles of the files a play names. The harness credits a detection only
through the truth's duplicate cluster, which holds the files of one
recording. When a mix plays Bad Company - China Cup and the search names
ƆƐI3C's China Cup upload, the harness counts a wrong confident answer,
because we judged the upload `different` by ear on 2026-10-22
(`docs/pair-verdicts.txt` line 84): it is almost identical, but its
hi-hats differ for a few bars. These detections are 18 of the 25 false
confident detections over the NAS-scale sweeps of seeds 2026-2028
(experiment 0188), every wrong confident answer of seed 2027 in
experiment 0230, and 16 in experiment 0059 (18 with three extra rungs;
0059 calls the upload China Cup's Prototype upload). The title named is
right.

Joining the upload into China Cup's cluster would answer the scoring
question with the wrong tool: a cluster is one recording, and the
verdicts and `clusters` decide recordings. A version is a different
recording that may carry the same title.

## Decision

1. **The principle.** A confident detection of a file judged to carry
   the truth's title names the right track and counts as right, even
   where the two files are different recordings.
2. **The file and its owner.** `docs/same-titles.txt` holds the
   judgments, one per line: `same-title` and two files, separated by
   tabs, named as in the clusters files (corpus library paths, or
   `second-library/<path>` for the NAS library). The user owns it and
   writes it by hand; agents never add entries, and code never derives
   one from file names or tags. Its first entry is our decision of
   2026-10-27 (`docs/status.md`, "session 39's listening questions, and
   four decisions"): `extra/b-bad_company-china_cup.mp3` against
   ƆƐI3C's upload. `titles::SameTitles` parses it and refuses, naming the
   file and the line, a line that does not start with `same-title`, a
   line without two files, a file judged against itself and a pair
   judged twice. A missing file holds no judgments. The harness's
   global option `--same-titles` names another file.
3. **What a judgment relates.** The two files' duplicate clusters: a
   detection of any file of one, where the truth is the other, is a
   same-title credit. Judgments are not followed through chains; each
   pair of recordings needs its own. Only scoring reads the file. The
   clusters, the verdicts, `clusters`, and what the harness draws or
   leaves out by cluster (the sweep's held-out fifth, recall's
   left-out copies, the mixes' draws) do not.
4. **The scoring, with the credits counted apart.**
   - Sweep and `recall`: a detection is `same_title` when the
     judgments credit its file to the excerpt's cluster (`correct` keeps
     its meaning, the excerpt's own cluster). A confident one is not a
     wrong answer and recalls an indexed excerpt; each speed row counts
     them (`same_title`, written when not zero), and the table prints
     the column.
   - Scan: a track's window also accepts the files credited to its
     references by title; a confident one identifies the track and is
     listed in `credited_through_title`; `Score.same_title` counts them.
   - Generated mixes (`mixes`, `grid`, the drops) and `robust`: the
     same, each with its own count.
   - Calibrate: a same-title detection is not a false candidate, and in
     the sweep it may identify a query, as it may identify a track in a
     scan. Calibrate prints the confident same-title credits. The
     detections on held-out excerpts, the possible tier's null on audio
     not in the index, keep every detection.
   - `seconds`, `loss`, and the commands that measure shared material
     or decide recordings (`shared`, `shared-mixes`, `identifiability`,
     `holders`, `related`, `pair`, `clusters`) keep the cluster alone.
5. **No threshold moves.** The confident rule and the possible tier rest
   on false candidates that are not versions of the played recording.
   ƆƐI3C's detections lay above the rule (261-273 hits, 0230), counted
   only as false candidates the rule accepted.

## Consequences

- A sweep that draws China Cup reports its ƆƐI3C detections as
  same-title credits instead of wrong answers; experiment 0233 lists
  the counts before and after.
- Saved reports written before this ADR have no `same_title` fields;
  calibrate and `regress` read them as before, so a baseline from before
  counts those detections as wrong.
- A judgment is a claim about titles, so a wrong entry hides a wrong
  answer behind a credit. The credits are counted in every report so
  they can be checked, and the file stays the user's.
- Other judged `different` pairs that we described as the same tune in
  another mix (Tower Bass, Cool Rock Stuff, the Stalker versions) are
  questions for us, not entries.
- The product, the clusters file, the verdicts and the held-out test set
  are unchanged. A later test evaluation is scored by the same code, so
  a same-title credit would count as right there too, listed apart; the
  first-run result and the ledger's earlier evaluations stay as
  recorded. No test evaluation ran for this ADR.
