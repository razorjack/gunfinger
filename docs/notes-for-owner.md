# Notes for the owner

Findings from the autonomous session that are worth your attention, newest
last. A morning summary goes at the top when the session ends.

## Findings

### Search is deterministic; the elegance pass changed speeds by one ulp

Three runs of the development scan give bit-identical reports (experiment
0007). The tiny speed differences I mentioned earlier (at most 4.4e-16)
came from the elegance pass rewriting the speed fit, not from run-to-run
variation. My earlier claim that the elegance pass gave "identical reports"
held to 9 decimal places, not bit for bit; `docs/status.md` and
`docs/calibration.md` now say so.
