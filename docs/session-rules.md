# Standing rules for sessions

These rules bind every session unless its own brief changes one by name.
They collect what briefs 2-75 and the orchestrator's prompts repeated, so
that the older briefs can be read as history instead of as live rules
(external review of 2026-11-05,
`docs/research/2026-11-05-external-review.md`). `AGENTS.md` and
`docs/brief.md` still apply in full and take precedence. When a rule in
an older brief matters and is missing here, copy it here in the same
commit that relies on it.

## Git and wording

- No push. Commit on master with the configured identity. Never rewrite
  history. Check `git status` before every commit and never use
  `git add -A`. No AI attribution in any commit: no `Co-Authored-By`
  line and no mention of an assistant or model.
- Commit after each step that changes tracked files.
- Repository text follows `AGENTS.md`'s wording rule. No em dashes:
  spaced en dashes in prose, ASCII in code. The Mac is "the Mac"; the
  Linux machine is "the Linux machine" (its hardware may be named where
  it explains a time).
- When recording anything the user said, paraphrase it in clean English;
  never quote the user's raw words in committed text.

## Data that sessions never touch

- The held-out test set (`ed-rush-optical-essential-mix`) is out of every
  step: do not open its manifest or any report of it, do not pass it to
  any command, do not scan or identify it, and never run test evaluation
  5 (one is left; ADR 0004). Never identify, decode or index
  `EM_1999_02_07_-_Ed_Rush_Optical.mp3` at the NAS root. Run nothing on
  the candidate mixes of experiment 0065 (among them the mixed CDs it
  lists and the NAS's `sety/vertical.mp3`). Name development sets
  explicitly; never glob `corpus/sets/*`.
- The validation halves of the recall panel and of both shared-material
  panels stay unsearched: no run, score or look at their detections.
  They are kept for a frozen rule's confirmation, and a session uses one
  only when its brief says so, after the rule is frozen in a commit.
- The NAS is read, never written, its `.gunfingerignore` included. Do not
  run `index` or `prune` with the NAS configuration and do not rebuild or
  replace the NAS clusters, unless the user has approved that prune or
  rebuild for the session by name. Never write to `work/peaks`.
- Agents never write verdicts or listening answers:
  `docs/pair-verdicts.txt` does not change, and no answer log is written.
  Agents never add entries to `docs/same-titles.txt`. Manifests and
  committed panels do not change. List questions for the user instead.
- `corpus/` is private. Nothing is published externally.
- Leave alone every process the session did not start, and the untracked
  `yafc20261031.log` in the repository root ("`git status` clean" means
  clean apart from it).

## Runs

- Every run that may last more than about 20 minutes starts detached
  under tmux with its own log, `/usr/bin/time -v` and an exit line; check
  the log's exit line afterwards. Give tmux sessions names that are not
  prefixes of each other.
- Timing comparisons run alone on an otherwise idle machine, A B B A.
  When several runs share the machine, give them one total worker budget
  and do not report their times as product timings.
- Scratch data goes under `work/sNN/`. Delete renders and per-session
  builds that no saved report depends on before the session ends; keep
  the last two final builds in `work/bin/`.
- `gunfinger-eval regress` prints what changed and returns success either
  way: read its output, and check that both reports hold the same number
  of queries. Run variants and other seeds in a scratch work directory,
  never in the reports directory `regress` compares.
- Start-of-session regressions may be skipped when the start build's
  binaries are byte-identical to the previous session's final build and
  the store revisions are unchanged; record the digests compared. The
  final regressions always run.
- A prune moves fitted speeds and weak candidates of untouched files
  through the set-aside cut (ADR 0014, experiment 0242). One check
  attributes such moves: the pre-prune index's lists restore them. No
  further tracing is needed, and the count of moves is reported.

## Experiments and decisions

- Pre-register before scored runs and commit the registration. Report a
  failed bar as a failure. A revised candidate gets a new registration
  and data the first run did not use.
- An experiment file is at most 40 lines and is written before the next
  experiment starts.
- Fresh seeds of the same recordings test repeatability, not
  generalisation: say how many distinct recordings or relation families
  a result rests on, beside the query count.
- The confident threshold is never lowered on a strongest-false figure
  that excludes same-title credits added in the same session.
- A change to an existing decision needs a new ADR; the next number is in
  `docs/adr/`. Defaults and adoption are the user's to decide; sessions
  write the evidence into `docs/decisions-queue.md`.

## Where each fact is written

- Measured results: the experiment file only. Other documents cite it by
  number and repeat at most the headline figure.
- `docs/status.md`: one short paragraph per session with pointers.
- `docs/session-notes.md`: the handoff (baselines, store revision,
  builds, data paths, open questions) and findings that no experiment
  holds; aim for about 60 lines per session.
- `docs/roadmap.md`: an item's current state and its next step.
- Open decisions: `docs/decisions-queue.md`.
