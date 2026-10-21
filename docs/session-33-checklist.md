# Session 33 checklist

Updated as work completes. `[x]` done and committed, `[~]` in progress,
`[ ]` not started, `[-]` not done (with the reason). See `docs/brief-33.md`
for scope and rules.

## First

- [x] Reading (First, item 1)
- [x] Brief, checklist and `CLAUDE.md` committed
- [x] `docs/pair-verdicts.txt` digest in
  `work/s33/listening-digests-start.txt`
- [x] Start build `work/bin/s33-start/`; `regress session-32-linux`
  identical (68.7 s)

## Steps

- [x] 2. `map-library`: 250 of 265 (the Mac's 247 of 262 and the 3
  extra corpus records)
- [x] 3. NAS clusters (`--cut-sparsest`, panel copy in NFC; run 1
  without `--recall-panel` first): 284 clusters, every difference from
  0128 explained; experiment 0155 (with steps 2 and 4)
- [x] 4. `baseline session-33-linux-nas`; `regress` of it identical
  (10:55); `devscore.py --nas` agrees
- [x] 5. Recall under the hardest conditions (eight runs, 03:57-05:27);
  compared with 0144 and 0145; experiment 0156
- [x] 6. Development scans with leave-outs at NAS scale; `seconds`;
  compared with 0067, 0100 and 0139; experiment 0157 (11 reports with the
  Mac's scores; per-second one second from 0139)
- [-] 7. (Ahead) shared panels at NAS scale: skipped, step 6 did not
  finish before 05:30 (the orchestrating thread's note); script
  `work/s33/ahead.sh`
- [-] 8. (Ahead) `recall --verify`: skipped for the same reason; script
  `work/s33/ahead-8.sh`

- [x] Added: experiment 0158 (16 workers against 32), for the
  orchestrating thread's question on speed

## Wrap-up

- [x] 9. Read-only review by a subagent; fixes (numbers, claims narrowed,
  scripts, handoff; brief at 06:20)
- [x] 10. Notes "Session 33", status, checklist, roadmap, ADR 0020
  (the workers' finding)
- [x] `scripts/check.sh` green (2:59); `regress session-32-linux`
  identical with `work/bin/s33-final/` (70.2 s); `docs/pair-verdicts.txt`
  unchanged; `git status` clean after the commit
- [ ] The final message
