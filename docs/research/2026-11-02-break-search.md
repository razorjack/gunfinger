# Which tracks use this break? A passage query, measured

Session 74, 2026-11-02. Brief `docs/brief-74.md` (pre-registration),
experiments 0276 (retrieval and noise) and 0277 (the lists set aside and
the cost). The product did not change.

## The question

Listening to session 58's `parts` candidates, the user heard that where
several different titles line up with a file's edges, they usually share
a classic jungle break (Amen, Think, Hot Pants), sometimes as differently
sampled variants; Special Branch (UFO, UFOLP001) is built on such
variants. The user asked whether Gunfinger could answer "I like this
break and have heard it before: which tracks use it?". That is a
different question from `identify`'s: it asks for every file holding a
short passage, not for the recording playing at a moment, and its
answer is a list, not a title.

## What a passage query finds

`gunfinger-eval break-search` takes a file and a span of 20 s, searches
the span's stored peaks against the whole index on the turntable ladder
(±8%), and lists each file once at its strongest detection of 20 hits
or more, strongest first, the query's own duplicate cluster apart. At
NAS scale (40,696 files):

- It finds the judged shared material: 18 of 20 panel directions, 5 of
  7 drum-loop pairs, all six Special Branch spans and 30 of 31 `parts`
  edges the user heard as one recording; 59 of 64 in all. Missed: the
  Watermelon pair (its shared passage plays key-locked in one file; the
  query searched only the turntable ladder), two weak drum loops and
  one `parts` edge.
- The expected file ranks first for 25 of 64 positives and in the first
  five for 51. What ranks above it is mostly not chance: other rips of
  the query's recording (the NAS clusters hold only the corpus
  recordings' rips), other versions of its tune, and other holders of
  the same break. Setting apart the files whose alignment goes on for
  60 s or more of the query (rips and versions, by audio), 39 rank first
  and 53 in the first five (exploratory).
- A random passage lists no other file at 20 hits in the median case;
  apart from rips, the strongest other file has 29 hits at the 90th
  percentile. The exceptions are themselves breaks: a random passage of
  John B - Double-J lists 207 files.

## How famous breaks behave

A famous break's passage lines up with hundreds of files at similar
strength: 403 files (1.0% of the index) for a passage of Knowledge &
Wisdom - Kill A Sound, 390 for Bigger Star - Raid, 348 for J Majik -
Your Sound (Remix), 347 for Krome & Time - License, 232 for Special
Branch at 305-325 s. Each holder has 40-110 hits, at relative speeds
spread over the ladder (a break sampled at another pitch, or a record
at another tempo). In such a list the file `parts` named is one holder
among many (License's at rank 28). Hits cannot rank these holders by
whether a listener would hear the same break; only the ear can.

The lists the first pass sets aside matter for these passages. Only
15-21% of a passage's distinct hashes lie in them, for famous breaks and
random passages alike, but they carry more of a break holder's hits:
with the filter, the expected file keeps a median 49% of its hits for
Special Branch and 64% for the `parts` edges, against 75-81% for the
query's own file. Special Branch's spans list a median 124 other files
with every list and 25 with the filter, and three `parts` edges are
found only with every list. ADR 0014's filter therefore hides most of a
famous break's holders, as the roadmap expected; for this query, which
searches 20 s instead of a mix, searching every list is affordable
(0277).

## The cost

With every list, a NAS-scale query takes a median 8 thread seconds (at
most 21) while 32 queries run at once, about 5 times the filtered
search; the reach of the 20 strongest files adds well under a second.
Loading the saved index takes 2.6 s; the run held 6.2 GiB. Run alone,
ten queries took a median 5.5 thread seconds with every list (at most
9.4) and 0.9 with the filter. Spread over the ladder's 41 rungs on 32
threads, a command would answer in well under a second after loading
its index (an inference: the harness searched each query on one
thread). At 262 tracks a query takes under 0.2 s.

## What a command would print

For a file and a span, the holders strongest first, each with its hits,
its relative speed, where the passage lies in it, and whether the file
goes on matching beyond the passage (another rip or version); the
query's own cluster on a line of its own. For Special Branch at
250-270 s, with every list:

```text
own      Special Branch (UFOLP001 rip)           1,730  1.000  4:10
rip      U.F.O - Special Branch.m4a              715  1.000  4:10   matches 348 s
 1.      Ray Keith - Dark Soldier (VIP Mix)       74  1.042  5:16
 2.      Cyba Space - I Bless The Day (RK Remix)  70  1.046  5:45
 3.      Ray Keith - I Bless The Day              69  1.046  5:40
 4.      Ray Keith - Dark Soldier (Back In The Day Mix)  65  1.041  5:48
 ...     204 files in all at 20 hits or more
```

## Recommendation

By the registered bar the query is not ready for a design note and an
ADR: it finds 92% of the judged shared material and puts it in the first
five 80% of the time, but first only 39% of the time, and the
registered null was dominated by unclustered rips. The failures are of
ranking and labelling, not of retrieval: a famous break has hundreds of
holders that hits do not order, and rips and versions crowd the top of
the list. Our recommendation:

1. Do not add a command yet. If one follows, it searches every list for
   the passage (the cost allows it), marks files that match beyond the
   passage as rips or versions, prints holders as a list with their
   speeds, and makes no title claim.
2. Before a design, ask the ear: for three famous-break queries
   (Special Branch 250-270 s, Krome & Time - License, Bigger Star -
   Raid), whether the first ten passage-only holders play the same
   break, and for two random passages with many holders (Double-J,
   Renegade - Terrorist) whether they are breaks. That tells whether the
   list's order means anything to a listener.
3. Search the key-locked ladder too in any later run; the Watermelon
   pair shows that some shared passages play key-locked.

The decision is ours.
