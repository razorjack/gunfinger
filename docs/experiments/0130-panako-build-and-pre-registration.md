# 0130: Panako as an outside benchmark: the build, and the pre-registration

**Question.** Session 24b: on the same queries at 262 tracks, how do
Panako's answers (Six & Leman 2014; Six 2021) compare with Gunfinger's,
and at what cost? A reference point only; the product does not change.

**Build** (`docs/brief-24b.md`, step 2). Panako `f1248f7` (2026-09-17;
last code change 2024-05-29), version 2.1, AGPL-3.0, cloned under
`work/panako/`. `./gradlew shadowJar` with Gradle's home under `work/`
downloaded Gradle 7.2 and failed on the settings file: `Unsupported
class file major version 65`. Gradle 7.2 runs on Java 8-16; the only
JDK here is Temurin 21 (needs Gradle 8.5 or later). Panako's CI builds
on JDK 11. By our rule the Panako part stops: it needs a JDK
of 11-16 for the build, or a newer Gradle and Shadow plugin in the local
clone's wrapper. Next possible obstacle: native code for macOS arm64 in
its LMDB and JGaborator jars, unknown until a build resolves them.
Nothing was written outside `work/`.

**Pre-registration** (brief, step 3, before any count), in short:

- Panako's `config.properties` defaults with `STRATEGY=PANAKO`; paths
  under `work/`; one thread; every printed result is a confident answer.
  Speed = matched reference span / query span (amended at 21:50, before
  any count: its time factor is 1/(2 - speed); brief, A1-A10).
- Indexes: all 262 files (cost measured here); per sweep seed, the
  library minus the panel's held-out recordings; optional, without
  stakka's references (the `--leave-out 11` equivalent).
- Queries: sweep seeds 2026-2029 (720 renders each); the three
  development mixes whole with `monitor`; then seed 2026's 12 generated
  mixes. Scoring as `sweep::speed_row`, `scoring::score` (90 s windows,
  clusters) and `mixes::score`; speed and start errors reported, a
  stricter recall without answers off by more than 3 s or 1 point.
- One stricter acceptance, chosen on seed 2026's sweep only: 1 + the
  highest score of a wrong answer; every panel reported at both.
- Gunfinger's side from its saved reports, confident detections only.

**Result.** No Panako count: the build did not run (with Java 11, 0150).

**Conclusion.** The comparison waits on a JDK of 11-16 (brief, step 2);
with the amendments the run needs no decision after a count.
