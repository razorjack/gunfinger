# CLAUDE.md

Follow `AGENTS.md`.

At session start, and after any context compaction, read these before doing
anything else:

1. `docs/brief.md` (the brief; never edit it)
2. `docs/session-rules.md` (the standing rules), `docs/decisions-queue.md`
   (decisions waiting for the user) and the roadmap's "The active queue";
   then the current session's brief and checklist (the newest
   `docs/brief-NN.md` and `docs/session-NN-checklist.md`; each session
   names them here, with what else it needs read). The older briefs are
   history, not live rules: the standing rules are collected in
   `docs/session-rules.md` (2026-11-05). Read an older brief only when a
   document points to it for a procedure, for example `docs/brief-64.md`
   for a prune. The current session is 83: `docs/brief-83.md` and
   `docs/session-83-checklist.md`, with experiments 0034 and 0207, the
   roadmap's "Peaks across frequency bands" (active queue item 7) and
   `docs/calibration.md`'s row "Peak neighbourhoods that widen with
   frequency".
3. `docs/session-notes.md`: its index and the newest session's section
   (the handoff: baselines, store revision, builds, open questions);
   older sessions are in `docs/history/`, named in the index
4. `docs/status.md`: the newest entries (the running log; older entries
   are in `docs/history/`, named in its index). The roadmap's item
   lines hold each item's state and next step; its history is in
   `docs/roadmap-history.md`, under the same item names
5. the newest file in `docs/experiments/`; the third development set's
   manifest, `corpus/sets/nocny-trans-vertical-2002-08-23/tracklist.toml`
   (never edit it)
6. for work on the NAS collection: `docs/brief-nas.md`,
   `docs/nas-checklist.md` and `docs/nas-plan.md` (the code is merged;
   sessions read the NAS and never write it, and do not rebuild the NAS
   clusters session 33 built on the Linux machine, apart from prunes the
   user approves by name: `docs/session-rules.md`)
7. for the listening tool (`gunfinger-eval audition`):
   `docs/adr/0016-listening-tool.md`, `docs/audition-questions.md` and
   every file in `crates/gunfinger-eval/src/audition/web/`
8. for matching or search-speed work: the proposals in
   `docs/research/2026-10-14-matching-proposals.md` (session 19
   measured proposals 1 and 2, session 20 proposals 3 and 5; session 21
   tested proposal 5's candidates in rendered mixes; session 22 ran part
   1 of proposal 2's retrieval study); on a machine with low memory
   bandwidth (the Linux machine), `docs/research/2026-10-21-low-bandwidth-search.md`
   and experiments 0167-0174
9. for shared-material work: the panels in `docs/panels/` and the
   newest experiments on them (sessions 20, 21, 23, 24 and 25); for the
   version verifier, `docs/research/2026-10-16-version-verification.md`
   and its version 2 (session 26, `docs/brief-26.md`)
10. for the confidence rule's window condition and per-second scoring:
   the roadmap's "A minimum span instead of 3 windows", session 27's
   experiments (0136-0142) and session 29's fair test (0146-0148; the
   variant worktree `s29-span`, never merged)
11. for recall beyond the corpus recordings: the roadmap's "Recall
   beyond the corpus recordings", experiments 0027, 0068 and 0074, and
   session 28's (0143-0145; 0150 is session 24b's)
12. for evidence beyond exact pair hashes: the roadmap's bullet of that
   name, the peak verifier's experiments (0054-0058) and session 30's
   tolerant pair count (0149 and up; `verifier.rs`)
13. for work on Linux or another platform: `docs/adr/0020-macos-and-linux.md`,
   AGENTS.md's "Platforms" and the notes of sessions 32 and 33
   (`docs/history/session-notes-01-39.md`; session 33: the NAS-scale
   harness on the Linux machine, experiments 0155-0158)
14. for marking time without a confident match in tracklists, and
   estimating how many tracks a gap holds: the roadmap's "Time without a
   confident match", `docs/research/2026-10-20-unidentified-gaps.md`,
   the pre-registration `docs/research/2026-10-21-gap-measures.md`,
   experiment 0154, session 34's (0159-0162) and session 35's (0163-0166)
15. for plays whose speed changes within them (pitch rides): the
   roadmap's "Speed that wanders within a play", experiments 0022, 0024,
   0030, 0032 and 0166, session 37's (0175-0178) and session 38's (0179
   and up)
