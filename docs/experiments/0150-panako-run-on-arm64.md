# 0150: Panako with Java 11: built, stopped by LMDB on arm64

**Question.** Session 24b's run of 2026-10-17: with Temurin 11 installed,
does Panako 2.1 (`f1248f7`) run the comparison pre-registered in 0130?

**Method** (`docs/brief-24b.md`, "The run of 2026-10-17";
`work/panako/run.sh`). `JAVA_HOME` = Temurin 11.0.32.1 for Panako's own
commands only; its wrapper unchanged. `build`, `install` (the jar and the
documented configuration with `STRATEGY=PANAKO` in `work/panako/run/`),
then `smoke`, the self-check: `store` of three library files, `query` of
seed 2026's render `000-+3.mp3`, `stats`. Load 44-49 (session 27's runs).

**Result.**

| Step | Outcome |
|---|---|
| `./gradlew shadowJar` | built in 17 s; jar 9.5 MB; no `~/.panako`, `~/.gradle` or `~/.m2` after it |
| Native code in the jar | JGaborator for macOS arm64; LMDB only for macOS x86_64 (0.9.20), Linux and Windows x86_64 (0.9.29) |
| `store`, `query`, `stats` | JGaborator loads; each fails creating its LMDB storage, before decoding: `UnsatisfiedLinkError: dlopen(liblmdb.dylib ...)` (no such file on any search path); exit 1 in 0.8-0.9 s |

Panako's other storages cannot stand in: `FILE` answers no queries
(empty query methods) and `MEM` is empty in every new process, while
`store` and `query` run as separate commands. No Panako count exists.

Outside `work/`, the runs left an empty Java Preferences node,
`be/panako/util`, in `com.apple.java.util.prefs.plist` and in a new
`be.panako.util.plist`; JGaborator's copy of its library in `$TMPDIR`
was deleted by Panako. The node was removed through Java's Preferences
API (and the cleanup's own `tool/JShell` node), then the empty file; the
plist's content equals the copy from before, its bytes differ.

**Conclusion.** By the instruction (native parts failing on arm64), the
Panako part stops. It needs an arm64 `liblmdb.dylib` passed to its JVM
as `-Dlmdbjava.native.lib=<path>` (Panako unchanged), or an x86_64 JDK
under Rosetta 2 with the bundled libraries; either is our
decision. 0130's protocol can then run as written if nothing else
fails (JGaborator has loaded on arm64 but processed no audio yet);
Gunfinger's side is in 0131.
