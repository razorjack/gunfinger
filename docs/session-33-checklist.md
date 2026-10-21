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
- [~] 5. Recall under the hardest conditions; compared with 0144 and
  0145; experiment 0156
- [ ] 6. Development scans with leave-outs at NAS scale; `seconds`;
  compared with 0067, 0100 and 0139; experiment 0157
- [ ] 7. (Ahead) shared panels at NAS scale; compared with 0122 and
  0129; experiment 0158
- [ ] 8. (Ahead) `recall --verify`; compared with 0151; experiment 0159

## Wrap-up

- [ ] 9. Read-only review by a subagent; fixes
- [ ] 10. Notes "Session 33", status, checklist, roadmap, ADR 0020 if
  needed
- [ ] `scripts/check.sh` green; `regress session-32-linux` identical with
  the final build; `docs/pair-verdicts.txt` unchanged; `git status`
  clean
- [ ] The final message
