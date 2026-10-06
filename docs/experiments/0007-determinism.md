# 0007: is search deterministic?

**Hypothesis.** Search gives the same result on every run: rungs run in
parallel, but `map_in_order` returns them in ladder order.

**Change.** None. Three runs of the development scan at commit 0ea0601 (the
existing report and two new runs), compared as parsed JSON with
`wall_seconds` removed and floats compared exactly.

**Command.** `target/release/gunfinger-eval scan stakka-skynet-knowledge`
twice; copies in `work/reference/det-run-{0,1,2}.json`.

**Result.** All three reports are identical, every float bit for bit.

The 4.4e-16 speed differences seen in experiment 0006 come from code, not
from scheduling: the elegance pass (commit 5559c99) rewrote the speed fit as
a hit-weighted least-squares fit through `weighted_mean`, which changes the
order of floating-point operations. Its verification compared floats to 9
decimal places, so "identical reports" held to that precision, not bit for
bit.

**Conclusion.** Search is deterministic; no fix needed. Regression checks
can compare reports exactly when the code under test does not change
arithmetic, and to a stated precision when it does.
