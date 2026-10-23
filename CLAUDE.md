# CLAUDE.md

Follow `AGENTS.md`.

At session start, and after any context compaction, read these before doing
anything else:

1. `docs/brief.md` (the brief; never edit it)
2. `docs/brief-41.md` (the current session's brief) and
   `docs/session-41-checklist.md` (its progress), with experiments
   0108, 0169-0173 and 0189 and up, the roadmap's speed bullet ("Where
   search time goes"), `docs/research/2026-10-21-low-bandwidth-search.md`
   and the code of `crates/gunfinger-core/src/search/lines.rs`;
   `docs/brief-2.md`,
   `docs/brief-3.md`, `docs/brief-4.md`, `docs/brief-5.md`,
   `docs/brief-6.md`, `docs/brief-7.md`, `docs/brief-8.md`,
   `docs/brief-9.md`, `docs/brief-10.md`, `docs/brief-11.md`,
   `docs/brief-12.md`, `docs/brief-13.md`, `docs/brief-14.md`,
   `docs/brief-15.md`, `docs/brief-16.md`, `docs/brief-17.md`,
   `docs/brief-18.md`, `docs/brief-19.md`, `docs/brief-20.md`,
   `docs/brief-21.md`, `docs/brief-22.md`, `docs/brief-23.md`,
   `docs/brief-24.md`, `docs/brief-24b.md` (session 24b, Panako, run
   beside session 24), `docs/brief-25.md`, `docs/brief-26.md`,
   `docs/brief-27.md`, `docs/brief-28.md`, `docs/brief-29.md`,
   `docs/brief-30.md`, `docs/brief-32.md`, `docs/brief-33.md`,
   `docs/brief-34.md`, `docs/brief-35.md`, `docs/brief-36.md`, `docs/brief-37.md`,
   `docs/brief-38.md`, `docs/brief-39.md` and `docs/brief-40.md` hold rules that still apply unless brief 41
   changes them
3. `docs/session-notes.md` (findings to report)
4. `docs/status.md` (running log: where the work stands)
5. the newest file in `docs/experiments/`; the third development set's
   manifest, `corpus/sets/nocny-trans-vertical-2002-08-23/tracklist.toml`
   (never edit it)
6. for work on the NAS collection: `docs/brief-nas.md`,
   `docs/nas-checklist.md` and `docs/nas-plan.md` (the code is merged;
   session 34 reads the NAS, never writes it, and does not rebuild the
   NAS clusters session 33 built on the Linux machine)
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
   AGENTS.md's "Platforms" and the notes of sessions 32 and 33 (session
   33: the NAS-scale harness on the Linux machine, experiments 0155-0158)
14. for marking time without a confident match in tracklists, and
   estimating how many tracks a gap holds: the roadmap's "Time without a
   confident match", `docs/research/2026-10-20-unidentified-gaps.md`,
   the pre-registration `docs/research/2026-10-21-gap-measures.md`,
   experiment 0154, session 34's (0159-0162) and session 35's (0163-0166)
15. for plays whose speed changes within them (pitch rides): the
   roadmap's "Speed that wanders within a play", experiments 0022, 0024,
   0030, 0032 and 0166, session 37's (0175-0178) and session 38's (0179
   and up)
