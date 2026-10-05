# 0002: duplicate clusters from a library self-match

**Hypothesis.** Rips of one recording align over almost their whole length;
remixes, VIPs and unrelated tracks do not. A strict coverage criterion
separates them with a wide gap.

**Change.** `gunfinger-eval clusters`: every one of the 262 indexed files is
decoded and searched against the whole library (itself included, ignored) on
an 11-rung ladder 0.98..1.02. A pair is the same recording when one detection
covers at least 80% of the shorter file. Clusters are the transitive closure.

**Command.** `target/release/gunfinger-eval clusters` (about 20 min on 10
cores); output `work/reports/duplicate-clusters.json`, log
`work/logs/clusters-1.log`.

**Result.**

- 46 same-recording pairs, all with coverage ≥ 0.984 and speed within
  0.9983..1.0017. The strongest other pair covers 0.39 with 22 hits: a chance
  chain of weak lines, not shared audio.
- 17 clusters with more than one member (15 pairs, 2 triples); the other 228
  files are singletons. Members: China Cup (3), The Nine (3), The Pulse,
  Funktion, Sick Note, Dominion (opus and the Kraken+Arkane MP3), Clockwork,
  Decoy (vinyl sampler and the 22 s shorter Clockwork CD version), Logistics,
  Night Lore, Pathogen, Bios-Fear, SKC Recharger BSE remix (3), Origin Unknown
  Equinox, SKC & Cord Swarm, Aphrodite Fanfare, Aphrodite Fanfare (dubstyle).
- The Stakka remix of Clockwork, Fanfare and Fanfare (dubstyle), and
  Recharger and its BSE remix stay separate, as they should.

**Conclusion.** The criterion is strict with a margin of 0.39 to 0.98; it is
kept. A side observation for the confidence rule: chance chains can span many
windows with few hits (22 hits over 39% of a track), so a window count alone
is weak evidence.
