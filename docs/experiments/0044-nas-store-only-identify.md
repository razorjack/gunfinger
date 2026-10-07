# 0044: identifying from the NAS peak store alone

**Question.** Item 9 of session 5 (NAS plan step 10): with the NAS
unmounted, what do the store-only index and `identify --store-only` cost,
and does it find what the harness finds?

**Command.** `/usr/bin/time -l gunfinger --config /dev/null --peaks-dir
<NAS store> --min-track 1:30 --max-track 15:00 identify <development
mix> --store-only --playback both -f json -v` (today's matcher), then
`show -f tracklist`. No external disk was attached: the store is on the
internal disk. Data: `data/0044-nas-store-only-identify.json`.

| Store-only `identify`, 26,890 records | Figure |
|---|--:|
| Index: postings; size; build (warm cache) | 755,152,725; 3,029 MB; 43.6 s |
| `stats` on the store alone, for comparison (0035) | 78.2 s; 3.07 GB |
| Whole run: wall; CPU; peak footprint | 856.0 s; 6,137 s; 8.35 GB |
| Report's `library_revision` | `d709390272e41475` (the store's) |
| Plays: confident; possible; `show -f tracklist` lines | 28; 2; 20 |

Every play matches a detection of the harness scan (0039, today's) of
the same NAS record and mix time with the same hits and windows. The
harness has 6 more: second corpus rips of the 9 NAS records that stand
for two corpus files each. The two possible plays are known shared
material (Star Trails (Synergy Remix) at 10:27, 75 hits; Clockwork's
Stakka remix at 20:22, 94), shown with "shares material with" its play.

The 11 tracks take 28 confident plays on 2-4 records each (rips and
uploads), at speeds that differ by each upload's own speed (Night Lore
+2.68% against the vinyl rip, -0.27% against the alien5ive upload). The
tracklist merges plays whose tags give the same name, so 7 tracks appear
twice ("STAKKA and SKYNET - knight lore", "Stakka And Skynet - Nightlore";
Konflict and Kemal & Rob Data for Star Trails; "Kraken+Arkane", "Kraken").

**Conclusions.** (1) The store alone identifies the mix as the harness
does: same plays, same evidence, the revision recorded. (2) It needs 8.4
GB and 14 minutes at this size under today's matcher; skip at 240 would
need about 4 GB and 4.3 minutes (harness, 0042). (3) A tracklist from a large
collection lists a track once per tag spelling; grouping plays by mix
time would list each track once (a proposal, not implemented).
