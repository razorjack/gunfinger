//! Recurring unidentified passages across a batch of mixes (brief 69,
//! proposal 6 of `docs/research/2026-10-29-matching-and-archive-proposals.md`):
//! generated mixes in which recordings left out of the index occur
//! several times, with different partners, cue points, bass swaps and
//! speeds; their unidentified spans (or the whole mixes) indexed
//! temporarily in memory and searched against each other within an
//! explicit relative-speed range; and the links scored against the
//! construction.
//!
//! A link is one confident detection between two mixes, reported with
//! the spans it supports. Links are kept pairwise: two links between the
//! same two mixes whose spans overlap in both are one correspondence (the
//! two directions of one match), and nothing else is merged, so links of
//! A with B and of B with C never make one of A with C. No title and no
//! whole-track claim is reported.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use gunfinger_core::confidence::Confidence;
use gunfinger_core::gaps::uncovered;
use gunfinger_core::index::Index;
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::{Asset, Library, Timestamp};
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::peaks::extract_peaks;
use gunfinger_core::speed::{Rung, ladder, ladder_with_extra_rungs};
use gunfinger_core::store::{PeakRecord, PeakStore, RecordHeader};
use serde::{Deserialize, Serialize};

use crate::clusters::Clusters;
use crate::files::{read_json, write_json};
use crate::identifiability::Panel;
use crate::matching::Matching;
use crate::mixes::{
    PlannedMix, PlannedPlay, Searcher, SweepIndex, crossfade, draw_unused, search_mix, sweep_index,
};
use crate::padding::Padding;
use crate::rng::Rng;
use crate::shared::Roots;
use crate::sweep::Plan;
use crate::titles::SameTitles;

const MIXES: usize = 12;
/// The sweep seed whose held-out clusters the index leaves out, as the
/// generated mixes' (`regress`'s).
const SWEEP_SEED: u64 = 2026;
const SUBJECTS: usize = 16;
/// Subject i occurs in mixes i + each of these, mod `MIXES`: 4 subjects a
/// mix, each in 3 different mixes.
const OCCURRENCE_OFFSETS: [usize; 3] = [0, 4, 8];
const SUBJECT_MIN_SECONDS: f64 = 330.0;
const PLAY_SECONDS: (f64, f64) = (60.0, 150.0);
/// Occurrences 0 and 1 of a control subject, so that occurrence 2 fits
/// after them in the track.
const CONTROL_PLAY_SECONDS: (f64, f64) = (60.0, 100.0);
/// Opposite deck settings for occurrences 0 and 1; occurrence 2 between.
const FAST: (f64, f64) = (1.04, 1.08);
const SLOW: (f64, f64) = (0.92, 0.96);
const THIRD: (f64, f64) = (0.94, 1.06);
/// Partners last at least this long, so a play of `PLAY_SECONDS.1` fits
/// between 10% and 95% of the track at the fastest speed.
const PARTNER_MIN_SECONDS: f64 = 200.0;
/// Partners and shared-material plays: the generated mixes' ±6%.
const OTHER_SPEEDS: (f64, f64) = (0.94, 1.06);
/// Where occurrence 0 starts, as shares of the track.
const FIRST_CUE: (f64, f64) = (0.10, 0.35);
const CONTROL_FIRST_CUE: (f64, f64) = (0.08, 0.20);
/// Occurrence 1 starts this close to occurrence 0 in the track.
const SECOND_CUE_SECONDS: f64 = 30.0;
/// Occurrence 2 of a subject that is not a control starts this far from
/// occurrence 0 in the track.
const THIRD_CUE_SECONDS: (f64, f64) = (-30.0, 60.0);
/// A control subject's occurrence 2 starts this far after the others
/// end in the track, and ends by this share of it.
const CONTROL_SPACING_SECONDS: f64 = 15.0;
const CONTROL_END_SHARE: f64 = 0.97;
const EARLIEST_SHARE: f64 = 0.05;
const MIN_PLAY_SECONDS: f64 = 40.0;
/// The panel's development half: pairs of records that share a passage,
/// by family, with how they relate.
const SHARED_FAMILIES: [(&str, Relation); 5] = [
    ("dozer", Relation::RelatedVersion),
    ("fire", Relation::RelatedVersion),
    ("sonar", Relation::RelatedVersion),
    ("pyro", Relation::DifferentTunes),
    ("watermelon", Relation::DifferentTunes),
];
const SHARED_PANEL: &str = "shared-material-seed-2026.json";
/// Pair j's records play in mixes 2j and 2j + 5, mod `MIXES`.
const SHARED_SECOND_MIX: usize = 5;
/// A shared-material play starts this long before its side of the passage
/// and lasts the passage plus twice this, at least `PLAY_SECONDS.0`.
const SHARED_LEAD_SECONDS: f64 = 20.0;
const INSERT_SECONDS: f64 = 12.0;
const INSERT_FADE_SECONDS: f64 = 0.5;
const INSERT_MIN_TRACK_SECONDS: f64 = 120.0;
const INSERT_MIXES: [usize; 6] = [0, 2, 4, 6, 8, 10];
/// Stretches without a confident detection this long or longer are a
/// mix's unidentified spans (0159's shortest that reaches every missed
/// DJ-length track).
const UNIDENTIFIED_SECONDS: f64 = 30.0;
/// Pieces stay well inside what a posting addresses
/// (`index::addressable_length`).
const PIECE_SECONDS: f64 = 600.0;
const PIECE_OVERLAP_SECONDS: f64 = 60.0;
/// A play takes part in a correspondence when it overlaps its span this
/// long; the two plays' positions in the source then agree within
/// `POSITION_SECONDS` for a right one.
const SPAN_SECONDS: f64 = 10.0;
const POSITION_SECONDS: f64 = 10.0;
/// Two plays are a positive when their source spans overlap this long.
const POSITIVE_SECONDS: f64 = 30.0;

/// How the two records of a shared-material pair relate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Relation {
    RelatedVersion,
    DifferentTunes,
}

/// Whether a subject's third occurrence overlaps the other two in the
/// track or shares no source time with them (a control subject).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Excerpts {
    Overlapping,
    Apart,
}

/// What a play is in the batch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "role", rename_all = "kebab-case")]
pub enum Role {
    /// A recording left out of the index, in 3 mixes.
    Subject {
        subject: usize,
        occurrence: usize,
        excerpts: Excerpts,
    },
    /// An indexed record.
    Partner,
    /// One side of a shared-material pair.
    Shared { pair: usize, side: usize },
    /// The repeated insert, laid over a subject's occurrence.
    Insert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchPlay {
    pub role: Role,
    pub play: PlannedPlay,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchMix {
    pub name: String,
    pub seconds: f64,
    pub plays: Vec<BatchPlay>,
}

impl BatchMix {
    fn planned(&self) -> PlannedMix {
        PlannedMix {
            name: self.name.clone(),
            seconds: self.seconds,
            plays: self.plays.iter().map(|play| play.play.clone()).collect(),
        }
    }
}

/// A shared-material pair and where each record holds the passage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedPair {
    pub family: String,
    pub relation: Relation,
    pub files: [String; 2],
    pub passages: [(f64, f64); 2],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Batch {
    pub mixes: Vec<BatchMix>,
    pub shared: Vec<SharedPair>,
    /// Every file of the subjects', the controls' and the insert's
    /// clusters, left out of the index.
    pub left_out: BTreeSet<String>,
}

/// The search modes: the pieces indexed and searched.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    /// Each mix's unidentified spans.
    Gaps,
    /// The whole mixes.
    Whole,
}

/// A stretch of one mix, in mix time.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Span {
    pub mix: usize,
    pub start_seconds: f64,
    pub end_seconds: f64,
}

impl Span {
    fn overlap(&self, other: &Span) -> f64 {
        if self.mix != other.mix {
            return 0.0;
        }
        (self.end_seconds.min(other.end_seconds) - self.start_seconds.max(other.start_seconds))
            .max(0.0)
    }

    fn length(&self) -> f64 {
        self.end_seconds - self.start_seconds
    }
}

/// One confident detection between pieces of two mixes: the query's span,
/// the reference's, and the reference's time per the query's (the query's
/// turntable speed over the reference's).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Link {
    pub query: Span,
    pub reference: Span,
    pub speed: f64,
    pub hits: u32,
    pub windows: u32,
}

impl Link {
    /// The same link with the mix of the lower number first.
    fn ordered(self) -> Link {
        if self.query.mix <= self.reference.mix {
            return self;
        }
        Link {
            query: self.reference,
            reference: self.query,
            speed: 1.0 / self.speed,
            ..self
        }
    }
}

/// Links between two mixes that overlap in both: one match, found in one
/// direction or both. `a` is in the mix of the lower number.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Correspondence {
    pub a: Span,
    pub b: Span,
    /// The strongest link's speed, as `b`'s mix time per `a`'s (`a`'s
    /// turntable speed over `b`'s), and its hits.
    pub speed: f64,
    pub hits: u32,
    pub links: usize,
}

impl Correspondence {
    /// The time in `b`'s mix that `seconds` in `a`'s maps to, linearly
    /// over the two spans.
    fn map_to_b(&self, seconds: f64) -> f64 {
        let scale = self.b.length() / self.a.length().max(1e-9);
        self.b.start_seconds + (seconds - self.a.start_seconds) * scale
    }

    fn map_to_a(&self, seconds: f64) -> f64 {
        let scale = self.a.length() / self.b.length().max(1e-9);
        self.a.start_seconds + (seconds - self.b.start_seconds) * scale
    }
}

/// The correspondences `links` make: each link, its mixes ordered, joins
/// the first correspondence between the same two mixes whose spans it
/// overlaps in both mixes, or starts one. Nothing else is merged, so
/// within one pair of mixes the result can depend on the links' order
/// (two correspondences that come to overlap stay two).
pub fn correspondences(links: &[Link]) -> Vec<Correspondence> {
    let mut found: Vec<Correspondence> = Vec::new();
    for link in links.iter().map(|link| link.ordered()) {
        let same = found.iter_mut().find(|correspondence| {
            correspondence.a.overlap(&link.query) > 0.0
                && correspondence.b.overlap(&link.reference) > 0.0
        });
        match same {
            Some(correspondence) => {
                correspondence.a = united(correspondence.a, link.query);
                correspondence.b = united(correspondence.b, link.reference);
                correspondence.links += 1;
                if link.hits > correspondence.hits {
                    correspondence.hits = link.hits;
                    correspondence.speed = link.speed;
                }
            }
            None => found.push(Correspondence {
                a: link.query,
                b: link.reference,
                speed: link.speed,
                hits: link.hits,
                links: 1,
            }),
        }
    }
    found
}

fn united(a: Span, b: Span) -> Span {
    Span {
        mix: a.mix,
        start_seconds: a.start_seconds.min(b.start_seconds),
        end_seconds: a.end_seconds.max(b.end_seconds),
    }
}

/// The rungs past each end of the product's ladder that a relative-speed
/// range of ±`range` needs (0 for the ladder's own ±8%).
pub fn extra_rungs_for(range: f64) -> u32 {
    let rungs = ladder();
    let step = rungs[1].speed().0 - rungs[0].speed().0;
    let fastest = rungs[rungs.len() - 1].speed().0;
    (((1.0 + range - fastest) / step) - 1e-9).ceil().max(0.0) as u32
}

/// The turntable ladder covering ±`range`.
pub fn relative_ladder(range: f64) -> Vec<Rung> {
    ladder_with_extra_rungs(extra_rungs_for(range))
}

// ---------------------------------------------------------------------
// Drawing the batch.

fn between((low, high): (f64, f64), rng: &mut Rng) -> f64 {
    low + rng.unit() * (high - low)
}

fn play(asset: &str, source_start: f64, speed: f64, start: f64, seconds: f64) -> PlannedPlay {
    PlannedPlay {
        asset: asset.to_owned(),
        held_out: true,
        source_start_seconds: source_start,
        speed,
        start_seconds: start,
        end_seconds: start + seconds,
        fade_in_seconds: 0.0,
        fade_out_seconds: 0.0,
        bass_cut_until: None,
        bass_cut_from: None,
        ride: None,
        gain_db: 0.0,
        scenario: None,
        side: None,
        drop_seconds: None,
    }
}

/// One subject's three occurrences (asset, source start, speed, mix-time
/// length), or `None` when a control's third does not fit.
fn occurrences(
    files: &[&PeakRecord],
    excerpts: Excerpts,
    rng: &mut Rng,
) -> Option<[(String, f64, f64, f64); 3]> {
    let control = excerpts == Excerpts::Apart;
    let duration = files
        .iter()
        .map(|record| record.header.duration_seconds)
        .fold(f64::INFINITY, f64::min);
    let mut file = || files[rng.below(files.len())].header.source.path.clone();
    let (first, second) = (file(), file());
    let third = file();
    let lengths = if control {
        CONTROL_PLAY_SECONDS
    } else {
        PLAY_SECONDS
    };
    let speeds = [between(FAST, rng), between(SLOW, rng), between(THIRD, rng)];
    let seconds = [between(lengths, rng), between(lengths, rng)];
    let cue = if control {
        CONTROL_FIRST_CUE
    } else {
        FIRST_CUE
    };
    let start0 = between(cue, rng) * duration;
    let start1 = (start0 + between((-SECOND_CUE_SECONDS, SECOND_CUE_SECONDS), rng))
        .max(EARLIEST_SHARE * duration);
    let ends = [
        start0 + seconds[0] * speeds[0],
        start1 + seconds[1] * speeds[1],
    ];
    let (start2, seconds2) = if control {
        let start = ends[0].max(ends[1]) + CONTROL_SPACING_SECONDS;
        let room = (CONTROL_END_SHARE * duration - start) / speeds[2];
        let wanted = between(PLAY_SECONDS, rng);
        if room < MIN_PLAY_SECONDS {
            return None;
        }
        (start, wanted.min(room))
    } else {
        let start = (start0 + between(THIRD_CUE_SECONDS, rng)).max(EARLIEST_SHARE * duration);
        let wanted = between(PLAY_SECONDS, rng);
        let room = (CONTROL_END_SHARE * duration - start) / speeds[2];
        (start, wanted.min(room).max(MIN_PLAY_SECONDS))
    };
    // Occurrences 0 and 1 must fit too: shorten them to the track.
    let fit = |start: f64, seconds: f64, speed: f64| {
        seconds.min((CONTROL_END_SHARE * duration - start) / speed)
    };
    Some([
        (first, start0, speeds[0], fit(start0, seconds[0], speeds[0])),
        (
            second,
            start1,
            speeds[1],
            fit(start1, seconds[1], speeds[1]),
        ),
        (third, start2, speeds[2], seconds2),
    ])
}

/// A held-out item to place in a mix: its role and its play, placed from
/// mix time 0.
struct Item {
    role: Role,
    play: PlannedPlay,
}

/// The files the panel names, and the development half's pairs that
/// `SHARED_FAMILIES` lists, each with its first direction's passage.
fn panel_pairs(panels: &Path) -> Result<(BTreeSet<String>, Vec<SharedPair>), String> {
    let panel: Panel = read_json(&panels.join(SHARED_PANEL))?;
    let files: BTreeSet<String> = panel
        .families
        .iter()
        .flat_map(|family| family.recordings.iter().flatten().cloned())
        .collect();
    let mut pairs = Vec::new();
    for (name, relation) in SHARED_FAMILIES {
        let family = panel
            .families
            .iter()
            .find(|family| family.name == name)
            .ok_or_else(|| format!("the panel has no family {name}"))?;
        if family.half != "development" {
            return Err(format!(
                "family {name} is in the panel's {} half",
                family.half
            ));
        }
        let (direction, passage) = family
            .directions
            .iter()
            .find_map(|direction| {
                let passage = direction.passage.as_ref()?;
                (!passage.control).then_some((direction, passage))
            })
            .ok_or_else(|| format!("family {name} has no shared passage"))?;
        let (Some(related_start), Some(related_end)) =
            (passage.related_start_seconds, passage.related_end_seconds)
        else {
            return Err(format!("family {name}'s passage has no related side"));
        };
        pairs.push(SharedPair {
            family: name.to_owned(),
            relation,
            files: [direction.played.clone(), direction.related.clone()],
            passages: [
                (passage.start_seconds, passage.end_seconds),
                (related_start, related_end),
            ],
        });
    }
    Ok((files, pairs))
}

fn file_length(roots: &Roots, asset: &str) -> Result<f64, String> {
    let source = roots.path(asset)?;
    gunfinger_core::decode::probe(&source)
        .and_then(|probe| probe.length)
        .map(|length| length.as_secs_f64())
        .ok_or_else(|| format!("cannot read the length of {}", source.display()))
}

/// Draws the batch from `records` (the corpus library's): subjects,
/// shared-material plays and the insert left out of the index, partners
/// from `indexed` less them.
pub fn draw_batch(
    records: &[PeakRecord],
    indexed: &BTreeSet<String>,
    clusters: &Clusters,
    panels: &Path,
    roots: &Roots,
    seed: u64,
) -> Result<Batch, String> {
    let mut rng = Rng::new(seed);
    let (panel_files, shared) = panel_pairs(panels)?;
    let in_panel = |record: &PeakRecord| {
        clusters
            .cluster_of(&record.header.source.path)
            .iter()
            .any(|file| panel_files.contains(file))
    };
    let outside_panel = |min_seconds: f64| -> Vec<&PeakRecord> {
        records
            .iter()
            .filter(|record| record.header.duration_seconds >= min_seconds)
            .filter(|record| !in_panel(record))
            .collect()
    };
    let mut used: BTreeSet<BTreeSet<String>> = BTreeSet::new();
    let mut items: Vec<Vec<Item>> = (0..MIXES).map(|_| Vec::new()).collect();
    for (mix, item) in subjects(
        records,
        &outside_panel(SUBJECT_MIN_SECONDS),
        &mut used,
        clusters,
        &mut rng,
    )? {
        items[mix].push(item);
    }
    for (mix, item) in shared_items(&shared, roots, &mut rng)? {
        items[mix].push(item);
    }
    let insert = draw_unused(
        &outside_panel(INSERT_MIN_TRACK_SECONDS),
        &mut used,
        clusters,
        &mut rng,
    )
    .ok_or("the library has no record left for the insert")?;

    let mut left_out: BTreeSet<String> = used.iter().flatten().cloned().collect();
    for shared_pair in &shared {
        for file in &shared_pair.files {
            left_out.extend(clusters.cluster_of(file));
        }
    }
    let partners: Vec<&PeakRecord> = outside_panel(PARTNER_MIN_SECONDS)
        .into_iter()
        .filter(|record| indexed.contains(&record.header.source.path))
        .filter(|record| !left_out.contains(&record.header.source.path))
        .collect();
    let mixes = items
        .into_iter()
        .enumerate()
        .map(|(number, mut held_out)| {
            rng.shuffle(&mut held_out);
            let mut plays = joined(held_out, &partners, clusters, &mut rng)?;
            if INSERT_MIXES.contains(&number) {
                plays.push(laid_insert(&plays, insert)?);
            }
            Ok(BatchMix {
                name: format!("passages-{seed}-mix-{number:02}"),
                seconds: plays
                    .iter()
                    .map(|play| play.play.end_seconds)
                    .fold(0.0, f64::max),
                plays,
            })
        })
        .collect::<Result<_, String>>()?;
    Ok(Batch {
        mixes,
        shared,
        left_out,
    })
}

/// The subjects' occurrences, each with the mix it plays in: subject i in
/// mixes i + `OCCURRENCE_OFFSETS`, the odd ones with a third occurrence
/// apart from the others.
fn subjects(
    records: &[PeakRecord],
    long: &[&PeakRecord],
    used: &mut BTreeSet<BTreeSet<String>>,
    clusters: &Clusters,
    rng: &mut Rng,
) -> Result<Vec<(usize, Item)>, String> {
    let mut items = Vec::new();
    for subject in 0..SUBJECTS {
        let excerpts = if subject % 2 == 1 {
            Excerpts::Apart
        } else {
            Excerpts::Overlapping
        };
        let drawn = loop {
            let record = draw_unused(long, used, clusters, rng)
                .ok_or("the library has too few long recordings for the subjects")?;
            let cluster = clusters.cluster_of(&record.header.source.path);
            let files: Vec<&PeakRecord> = records
                .iter()
                .filter(|record| cluster.contains(&record.header.source.path))
                .collect();
            if let Some(drawn) = occurrences(&files, excerpts, rng) {
                break drawn;
            }
        };
        for (occurrence, (asset, source_start, speed, seconds)) in drawn.into_iter().enumerate() {
            items.push((
                (subject + OCCURRENCE_OFFSETS[occurrence]) % MIXES,
                Item {
                    role: Role::Subject {
                        subject,
                        occurrence,
                        excerpts,
                    },
                    play: play(&asset, source_start, speed, 0.0, seconds),
                },
            ));
        }
    }
    Ok(items)
}

/// Each shared-material pair's two plays over their passage, pair j's in
/// mixes 2j and 2j + `SHARED_SECOND_MIX`.
fn shared_items(
    shared: &[SharedPair],
    roots: &Roots,
    rng: &mut Rng,
) -> Result<Vec<(usize, Item)>, String> {
    let mut items = Vec::new();
    for (pair, shared_pair) in shared.iter().enumerate() {
        for side in 0..2 {
            let asset = &shared_pair.files[side];
            let (from, to) = shared_pair.passages[side];
            let duration = file_length(roots, asset)?;
            let speed = between(OTHER_SPEEDS, rng);
            let source_start = (from - SHARED_LEAD_SECONDS).max(0.0);
            let source_end = (to + SHARED_LEAD_SECONDS)
                .max(source_start + PLAY_SECONDS.0 * speed)
                .min(duration - 0.5);
            items.push((
                (2 * pair + side * SHARED_SECOND_MIX) % MIXES,
                Item {
                    role: Role::Shared { pair, side },
                    play: play(
                        asset,
                        source_start,
                        speed,
                        0.0,
                        (source_end - source_start) / speed,
                    ),
                },
            ));
        }
    }
    Ok(items)
}

/// A drawn partner of `partners`, whose cluster then joins `used`.
fn partner(
    partners: &[&PeakRecord],
    used: &mut BTreeSet<BTreeSet<String>>,
    clusters: &Clusters,
    rng: &mut Rng,
) -> Result<Item, String> {
    let record = draw_unused(partners, used, clusters, rng)
        .ok_or("the library has too few indexed records for the partners")?;
    let seconds = between(PLAY_SECONDS, rng);
    let speed = between(OTHER_SPEEDS, rng);
    let duration = record.header.duration_seconds;
    let earliest = 0.1 * duration;
    let latest = 0.95 * duration - seconds * speed;
    let mut placed = play(
        &record.header.source.path,
        earliest + rng.unit() * (latest - earliest),
        speed,
        0.0,
        seconds,
    );
    placed.held_out = false;
    Ok(Item {
        role: Role::Partner,
        play: placed,
    })
}

/// `held_out` in order, a partner before, between and after them,
/// placed one after another and joined by crossfades.
fn joined(
    held_out: Vec<Item>,
    partners: &[&PeakRecord],
    clusters: &Clusters,
    rng: &mut Rng,
) -> Result<Vec<BatchPlay>, String> {
    let mut used = BTreeSet::new();
    let mut sequence: Vec<Item> = vec![partner(partners, &mut used, clusters, rng)?];
    for item in held_out {
        sequence.push(item);
        sequence.push(partner(partners, &mut used, clusters, rng)?);
    }
    let mut plays: Vec<BatchPlay> = Vec::new();
    let mut start = 0.0;
    for Item { role, mut play } in sequence {
        let seconds = play.end_seconds - play.start_seconds;
        play.start_seconds = start;
        play.end_seconds = start + seconds;
        if let Some(previous) = plays.last_mut() {
            crossfade(&mut previous.play, &mut play, rng);
        }
        start = play.end_seconds;
        plays.push(BatchPlay { role, play });
    }
    Ok(plays)
}

/// The insert from the middle of `insert`, over the middle of the first
/// subject's occurrence among `plays`.
fn laid_insert(plays: &[BatchPlay], insert: &PeakRecord) -> Result<BatchPlay, String> {
    let over = plays
        .iter()
        .find(|play| matches!(play.role, Role::Subject { .. }))
        .ok_or("a mix of the insert has no subject")?;
    let middle = (over.play.start_seconds + over.play.end_seconds) / 2.0;
    let mut laid = play(
        &insert.header.source.path,
        insert.header.duration_seconds / 2.0 - INSERT_SECONDS / 2.0,
        1.0,
        middle - INSERT_SECONDS / 2.0,
        INSERT_SECONDS,
    );
    laid.fade_in_seconds = INSERT_FADE_SECONDS;
    laid.fade_out_seconds = INSERT_FADE_SECONDS;
    Ok(BatchPlay {
        role: Role::Insert,
        play: laid,
    })
}

// ---------------------------------------------------------------------
// Scoring against the construction.

/// The stretch of the track `play` plays.
fn source_span(play: &PlannedPlay) -> (f64, f64) {
    (play.source_start_seconds, play.source_at(play.end_seconds))
}

fn mix_span(mix: usize, play: &PlannedPlay) -> Span {
    Span {
        mix,
        start_seconds: play.start_seconds,
        end_seconds: play.end_seconds,
    }
}

/// The mix time at which `play` reaches source time `source`.
fn mix_time(play: &PlannedPlay, source: f64) -> f64 {
    play.start_seconds + (source - play.source_start_seconds) / play.speed
}

/// A play of the batch by mix and position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct PlayRef {
    pub mix: usize,
    pub play: usize,
}

/// How a correspondence reads against the construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Verdict {
    Right,
    Insert,
    OneSourceAnotherPassage,
    SharedPassage,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredCorrespondence {
    #[serde(flatten)]
    pub correspondence: Correspondence,
    pub verdict: Verdict,
    /// The pairs of plays whose source positions it aligns (right or
    /// insert), or which it overlaps (the wrong verdicts).
    pub plays: Vec<[PlayRef; 2]>,
    /// For a shared-passage link: how far its span reaches past either
    /// end of the panel's passage in each mix, in seconds (0 inside).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outside_passage_seconds: Option<[f64; 2]>,
}

/// Two plays of one recording in different mixes whose source spans
/// overlap by `POSITIVE_SECONDS` or more, and how a correspondence found
/// them.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Positive {
    pub plays: [PlayRef; 2],
    pub roles: [Role; 2],
    pub overlap_seconds: f64,
    /// The second play's turntable speed over the first's.
    pub relative_speed: f64,
    pub found: bool,
    /// Over the found correspondence covering most of the overlap: how
    /// much later than the overlap it starts and earlier than it ends,
    /// in each mix, and the share of the overlap it covers in the first.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_late_seconds: Option<[f64; 2]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_early_seconds: Option<[f64; 2]>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub covered_share: Option<f64>,
}

/// A control subject's pair of occurrences that share no source time.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ControlPair {
    pub plays: [PlayRef; 2],
    pub linked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Score {
    pub correspondences: Vec<ScoredCorrespondence>,
    pub positives: Vec<Positive>,
    pub control_pairs: Vec<ControlPair>,
    /// Shared-material pairs linked, by pair.
    pub shared_linked: Vec<bool>,
}

struct Truth<'a> {
    batch: &'a Batch,
    recordings: Vec<Vec<BTreeSet<String>>>,
}

impl Truth<'_> {
    fn play(&self, at: PlayRef) -> &BatchPlay {
        &self.batch.mixes[at.mix].plays[at.play]
    }

    fn same_recording(&self, a: PlayRef, b: PlayRef) -> bool {
        self.recordings[a.mix][a.play] == self.recordings[b.mix][b.play]
    }

    /// The plays of `span`'s mix that overlap it by `SPAN_SECONDS` or more.
    fn overlapping(&self, span: &Span) -> Vec<PlayRef> {
        self.batch.mixes[span.mix]
            .plays
            .iter()
            .enumerate()
            .filter(|(_, play)| mix_span(span.mix, &play.play).overlap(span) >= SPAN_SECONDS)
            .map(|(play, _)| PlayRef {
                mix: span.mix,
                play,
            })
            .collect()
    }

    /// Whether `correspondence` maps a moment of `a` to the same source
    /// position in `b`, within `POSITION_SECONDS`: at the middle of `a`'s
    /// overlap with the span, or failing that at the middle of `b`'s.
    fn aligns(&self, correspondence: &Correspondence, a: PlayRef, b: PlayRef) -> bool {
        let (pa, pb) = (&self.play(a).play, &self.play(b).play);
        let middle = |play: &PlannedPlay, span: &Span| {
            (play.start_seconds.max(span.start_seconds) + play.end_seconds.min(span.end_seconds))
                / 2.0
        };
        let inside = |play: &PlannedPlay, seconds: f64| {
            (play.start_seconds..=play.end_seconds).contains(&seconds)
        };
        let at_a = middle(pa, &correspondence.a);
        let at_b = middle(pb, &correspondence.b);
        let moments = [
            (at_a, correspondence.map_to_b(at_a)),
            (correspondence.map_to_a(at_b), at_b),
        ];
        moments.iter().any(|&(in_a, in_b)| {
            inside(pa, in_a)
                && inside(pb, in_b)
                && (pa.source_at(in_a) - pb.source_at(in_b)).abs() <= POSITION_SECONDS
        })
    }

    fn insert_share(&self, span: &Span) -> f64 {
        self.batch.mixes[span.mix]
            .plays
            .iter()
            .filter(|play| play.role == Role::Insert)
            .map(|play| mix_span(span.mix, &play.play).overlap(span))
            .sum::<f64>()
            / span.length().max(1e-9)
    }

    fn verdict(&self, correspondence: &Correspondence) -> ScoredCorrespondence {
        let in_a = self.overlapping(&correspondence.a);
        let in_b = self.overlapping(&correspondence.b);
        let pairs: Vec<[PlayRef; 2]> = in_a
            .iter()
            .flat_map(|&a| in_b.iter().map(move |&b| [a, b]))
            .collect();
        let is_insert = |[a, _]: &[PlayRef; 2]| self.play(*a).role == Role::Insert;
        let aligned: Vec<[PlayRef; 2]> = pairs
            .iter()
            .copied()
            .filter(|&[a, b]| self.same_recording(a, b) && self.aligns(correspondence, a, b))
            .collect();
        let scored = |verdict: Verdict, plays: Vec<[PlayRef; 2]>| ScoredCorrespondence {
            correspondence: correspondence.clone(),
            verdict,
            plays,
            outside_passage_seconds: None,
        };
        let right: Vec<[PlayRef; 2]> = aligned.iter().copied().filter(|p| !is_insert(p)).collect();
        if !right.is_empty() {
            return scored(Verdict::Right, right);
        }
        if !aligned.is_empty()
            || (self.insert_share(&correspondence.a) >= 0.5
                && self.insert_share(&correspondence.b) >= 0.5)
        {
            return scored(Verdict::Insert, aligned);
        }
        let same: Vec<[PlayRef; 2]> = pairs
            .iter()
            .copied()
            .filter(|&[a, b]| self.same_recording(a, b))
            .collect();
        if !same.is_empty() {
            return scored(Verdict::OneSourceAnotherPassage, same);
        }
        let shared: Vec<[PlayRef; 2]> = pairs
            .iter()
            .copied()
            .filter(|&[a, b]| {
                matches!(
                    (self.play(a).role, self.play(b).role),
                    (Role::Shared { pair: x, side: s }, Role::Shared { pair: y, side: t })
                        if x == y && s != t
                )
            })
            .collect();
        if let Some(&[a, b]) = shared.first() {
            let outside = [
                self.outside_passage(a, &correspondence.a),
                self.outside_passage(b, &correspondence.b),
            ];
            return ScoredCorrespondence {
                outside_passage_seconds: Some(outside),
                ..scored(Verdict::SharedPassage, shared)
            };
        }
        scored(Verdict::Other, pairs)
    }

    /// How far `span` reaches past either end of the panel's passage as
    /// `at` (a shared-material play) holds it, in seconds.
    fn outside_passage(&self, at: PlayRef, span: &Span) -> f64 {
        let batch_play = self.play(at);
        let Role::Shared { pair, side } = batch_play.role else {
            return 0.0;
        };
        let (from, to) = self.batch.shared[pair].passages[side];
        let (start, end) = (
            mix_time(&batch_play.play, from),
            mix_time(&batch_play.play, to),
        );
        (start - span.start_seconds)
            .max(span.end_seconds - end)
            .max(0.0)
    }
}

/// Scores `correspondences` against `batch`; `recording` names each
/// play's recording (its cluster).
pub fn score(
    batch: &Batch,
    correspondences: &[Correspondence],
    recording: impl Fn(&str) -> BTreeSet<String>,
) -> Score {
    let truth = Truth {
        batch,
        recordings: batch
            .mixes
            .iter()
            .map(|mix| {
                mix.plays
                    .iter()
                    .map(|play| recording(&play.play.asset))
                    .collect()
            })
            .collect(),
    };
    let scored: Vec<ScoredCorrespondence> = correspondences
        .iter()
        .map(|correspondence| truth.verdict(correspondence))
        .collect();
    let refs: Vec<PlayRef> = batch
        .mixes
        .iter()
        .enumerate()
        .flat_map(|(mix, plays)| (0..plays.plays.len()).map(move |play| PlayRef { mix, play }))
        .collect();
    let found_pairs = |verdicts: &[Verdict]| -> Vec<(&ScoredCorrespondence, [PlayRef; 2])> {
        scored
            .iter()
            .filter(|scored| verdicts.contains(&scored.verdict))
            .flat_map(|scored| scored.plays.iter().map(move |&pair| (scored, pair)))
            .collect()
    };
    let aligned = found_pairs(&[Verdict::Right, Verdict::Insert]);
    let mut positives = Vec::new();
    let mut control_pairs = Vec::new();
    for (i, &a) in refs.iter().enumerate() {
        for &b in &refs[i + 1..] {
            if a.mix == b.mix || !truth.same_recording(a, b) {
                continue;
            }
            let (pa, pb) = (&truth.play(a).play, &truth.play(b).play);
            let (sa, sb) = (source_span(pa), source_span(pb));
            let (from, to) = (sa.0.max(sb.0), sa.1.min(sb.1));
            let overlap = to - from;
            let linked: Vec<&ScoredCorrespondence> = aligned
                .iter()
                .filter(|(_, pair)| *pair == [a, b] || *pair == [b, a])
                .map(|(scored, _)| *scored)
                .collect();
            if let (
                Role::Subject {
                    subject: x,
                    occurrence: i,
                    excerpts: Excerpts::Apart,
                },
                Role::Subject {
                    subject: y,
                    occurrence: j,
                    ..
                },
            ) = (truth.play(a).role, truth.play(b).role)
                && x == y
                && (i == 2 || j == 2)
            {
                let any_link = scored.iter().any(|scored| {
                    scored
                        .plays
                        .iter()
                        .any(|pair| *pair == [a, b] || *pair == [b, a])
                });
                control_pairs.push(ControlPair {
                    plays: [a, b],
                    linked: any_link,
                });
            }
            if overlap < POSITIVE_SECONDS {
                continue;
            }
            let truth_a = (mix_time(pa, from), mix_time(pa, to));
            let truth_b = (mix_time(pb, from), mix_time(pb, to));
            let best = linked.iter().max_by(|x, y| {
                covered(&x.correspondence, truth_a, a.mix).total_cmp(&covered(
                    &y.correspondence,
                    truth_a,
                    a.mix,
                ))
            });
            let span_in = |scored: &ScoredCorrespondence, mix: usize| {
                if scored.correspondence.a.mix == mix {
                    scored.correspondence.a
                } else {
                    scored.correspondence.b
                }
            };
            positives.push(Positive {
                plays: [a, b],
                roles: [truth.play(a).role, truth.play(b).role],
                overlap_seconds: overlap,
                relative_speed: pb.speed / pa.speed,
                found: best.is_some(),
                start_late_seconds: best.map(|scored| {
                    [
                        span_in(scored, a.mix).start_seconds - truth_a.0,
                        span_in(scored, b.mix).start_seconds - truth_b.0,
                    ]
                }),
                end_early_seconds: best.map(|scored| {
                    [
                        truth_a.1 - span_in(scored, a.mix).end_seconds,
                        truth_b.1 - span_in(scored, b.mix).end_seconds,
                    ]
                }),
                covered_share: best.map(|scored| covered(&scored.correspondence, truth_a, a.mix)),
            });
        }
    }
    let shared_linked = (0..batch.shared.len())
        .map(|pair| {
            scored.iter().any(|scored| {
                scored.verdict == Verdict::SharedPassage
                    && scored.plays.iter().any(|&[a, _]| {
                        matches!(truth.play(a).role, Role::Shared { pair: p, .. } if p == pair)
                    })
            })
        })
        .collect();
    Score {
        correspondences: scored,
        positives,
        control_pairs,
        shared_linked,
    }
}

/// The share of `truth` (a stretch of mix `mix`) that the
/// correspondence's span in that mix covers.
fn covered(correspondence: &Correspondence, truth: (f64, f64), mix: usize) -> f64 {
    let span = if correspondence.a.mix == mix {
        correspondence.a
    } else {
        correspondence.b
    };
    let truth = Span {
        mix,
        start_seconds: truth.0,
        end_seconds: truth.1,
    };
    span.overlap(&truth) / truth.length().max(1e-9)
}

// ---------------------------------------------------------------------
// Searching the batch.

/// The stretches searched for one mix in `mode`, cut into pieces of at
/// most `PIECE_SECONDS` overlapping by `PIECE_OVERLAP_SECONDS`.
pub fn pieces(stretches: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut pieces = Vec::new();
    for &(start, end) in stretches {
        let mut from = start;
        loop {
            let to = (from + PIECE_SECONDS).min(end);
            pieces.push((from, to));
            if to >= end {
                break;
            }
            from = to - PIECE_OVERLAP_SECONDS;
        }
    }
    pieces
}

/// A piece of a mix as an indexed record of the harness's own name.
fn piece_record(
    mix: usize,
    (start, end): (f64, f64),
    samples: &[f32],
    matching: &Matching,
) -> PeakRecord {
    let profile = matching.profile();
    PeakRecord {
        header: RecordHeader {
            profile: profile.id(),
            source: Asset {
                path: format!("batch-mix-{mix:02}/{start:.3}-{end:.3}"),
                size: 0,
                modified: Timestamp {
                    seconds: 0,
                    nanos: 0,
                },
            },
            duration_seconds: samples.len() as f64 / f64::from(profile.sample_rate),
        },
        peaks: extract_peaks(samples, &profile),
    }
}

fn slice(samples: &[f32], rate: f64, (start, end): (f64, f64)) -> &[f32] {
    let at = |seconds: f64| ((seconds * rate).round().max(0.0) as usize).min(samples.len());
    &samples[at(start)..at(end)]
}

/// One mode and range searched over the batch.
#[derive(Serialize, Deserialize)]
pub struct Run {
    pub mode: Mode,
    pub relative_speed_range: f64,
    pub rungs: usize,
    pub slowest: f64,
    pub fastest: f64,
    pub pieces: usize,
    pub piece_seconds: f64,
    pub postings: usize,
    pub index_seconds: f64,
    /// Elapsed, and the sum of each piece's search time on its thread.
    pub search_seconds: f64,
    pub search_thread_seconds: f64,
    pub links: Vec<Link>,
    pub score: Score,
}

/// What the pairwise search of one mode and range found, and its cost.
struct Searched {
    links: Vec<Link>,
    postings: usize,
    index_seconds: f64,
    search_seconds: f64,
    /// The sum of each piece's search time on its thread.
    search_thread_seconds: f64,
}

/// The pairwise search of `pieces` (each a mix's piece and its samples)
/// against all of them, confident links only, a piece's own mix left out.
fn pairwise(
    pieces: &[(usize, (f64, f64), &[f32])],
    rungs: &[Rung],
    matching: &Matching,
    jobs: usize,
) -> Result<Searched, String> {
    let started = Instant::now();
    let records: Vec<PeakRecord> = map_in_order(pieces, jobs, |&(mix, span, samples)| {
        piece_record(mix, span, samples, matching)
    });
    let index = matching.index(Index::build(&records).map_err(|error| error.to_string())?);
    let index_seconds = started.elapsed().as_secs_f64();
    let by_path: BTreeMap<&str, usize> = records
        .iter()
        .enumerate()
        .map(|(position, record)| (record.header.source.path.as_str(), position))
        .collect();
    let profile = matching.profile();
    let started = Instant::now();
    let found: Vec<Result<(Vec<Link>, f64), String>> =
        map_in_order(pieces, jobs, |&(mix, span, samples)| {
            let began = Instant::now();
            let mut links = Vec::new();
            for detection in matching.search(&index, samples, &profile, rungs, 1) {
                if detection.evidence.confidence() != Confidence::Confident {
                    continue;
                }
                let path = &index.asset(detection.asset).path;
                let &(other, other_span, _) = by_path
                    .get(path.as_str())
                    .and_then(|&position| pieces.get(position))
                    .ok_or_else(|| format!("the pieces' index holds an unknown record {path}"))?;
                if other == mix {
                    continue;
                }
                links.push(Link {
                    query: Span {
                        mix,
                        start_seconds: span.0 + detection.start_seconds,
                        end_seconds: span.0 + detection.end_seconds,
                    },
                    reference: Span {
                        mix: other,
                        start_seconds: other_span.0 + detection.track_start_seconds,
                        end_seconds: other_span.0 + detection.track_end_seconds,
                    },
                    speed: detection.speed.0,
                    hits: detection.evidence.hits,
                    windows: detection.evidence.windows,
                });
            }
            Ok((links, began.elapsed().as_secs_f64()))
        });
    let search_seconds = started.elapsed().as_secs_f64();
    let found: Vec<(Vec<Link>, f64)> = found.into_iter().collect::<Result<_, String>>()?;
    Ok(Searched {
        search_thread_seconds: found.iter().map(|(_, seconds)| seconds).sum(),
        links: found.into_iter().flat_map(|(links, _)| links).collect(),
        postings: index.posting_count(),
        index_seconds,
        search_seconds,
    })
}

pub struct Options<'a> {
    pub seed: u64,
    pub modes: &'a [Mode],
    pub ranges: &'a [f64],
    pub same_titles: &'a SameTitles,
    pub panels: &'a Path,
    pub other_root: Option<&'a Path>,
    /// The ladder each mix is searched with against the library.
    pub ladder: &'a [Rung],
    pub matching: &'a Matching,
    pub padding: &'a Padding,
    pub out: &'a Path,
    pub keep_renders: bool,
    pub jobs: usize,
}

/// One mix's search against the library.
#[derive(Serialize, Deserialize)]
pub struct LibrarySearch {
    pub mix: usize,
    pub confident: usize,
    pub unidentified: Vec<(f64, f64)>,
    /// Subjects' and shared-material plays: the share of each play's span
    /// that the unidentified spans cover.
    pub held_out_covered: Vec<(usize, f64)>,
    /// Confident detections of no play of their recording in the mix.
    pub wrong: usize,
}

#[derive(Serialize, Deserialize)]
pub struct PassagesReport {
    pub seed: u64,
    pub build: String,
    pub indexed_assets: usize,
    pub batch: Batch,
    pub render_and_search_seconds: f64,
    pub library_searches: Vec<LibrarySearch>,
    pub runs: Vec<Run>,
}

pub fn run(
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    options: &Options,
) -> Result<PassagesReport, String> {
    let roots = Roots {
        library: &library.root,
        other: options.other_root,
    };
    let records = load_records(
        library,
        store,
        &options.matching.profile(),
        &BTreeSet::new(),
    )
    .0;
    let held_out = Plan::for_seed(&records, clusters, SWEEP_SEED, options.panels)?.held_out;
    let indexed: BTreeSet<String> = records
        .iter()
        .map(|record| record.header.source.path.clone())
        .filter(|path| !held_out.contains(path))
        .collect();
    let batch = draw_batch(
        &records,
        &indexed,
        clusters,
        options.panels,
        &roots,
        options.seed,
    )?;
    let SweepIndex { index, .. } = sweep_index(
        records,
        clusters,
        SWEEP_SEED,
        options.panels,
        options.matching,
        options.padding,
        &batch.left_out,
    )?;
    let renders = options.out.join("renders");
    let searcher = Searcher {
        index: &index,
        ladder: options.ladder,
        matching: options.matching,
        verifier: None,
        fault: None,
    };
    let started = Instant::now();
    let numbered: Vec<(usize, &BatchMix)> = batch.mixes.iter().enumerate().collect();
    let searched: Vec<(LibrarySearch, Vec<f32>)> =
        map_in_order(&numbered, options.jobs, |&(number, mix)| {
            let path = renders.join(format!("{}.mp3", mix.name));
            let result = search_mix(
                &mix.planned(),
                &path,
                &roots,
                clusters,
                options.same_titles,
                &searcher,
            )?;
            let covered: Vec<(f64, f64)> = result
                .detections
                .iter()
                .filter(|found| found.confidence() == Confidence::Confident)
                .map(|found| (found.start_seconds, found.end_seconds))
                .collect();
            let unidentified = uncovered(&covered, 0.0, mix.seconds, UNIDENTIFIED_SECONDS);
            let held_out_covered = mix
                .plays
                .iter()
                .enumerate()
                .filter(|(_, play)| matches!(play.role, Role::Subject { .. } | Role::Shared { .. }))
                .map(|(position, play)| {
                    let span = mix_span(number, &play.play);
                    let inside: f64 = unidentified
                        .iter()
                        .map(|&(start, end)| {
                            span.overlap(&Span {
                                mix: number,
                                start_seconds: start,
                                end_seconds: end,
                            })
                        })
                        .sum();
                    (position, inside / span.length())
                })
                .collect();
            let audio = options.matching.decode_query(&path)?;
            let samples = audio
                .mono()
                .ok_or("passages searches the downmix only (--front-end downmix)")?
                .to_vec();
            Ok((
                LibrarySearch {
                    mix: number,
                    confident: covered.len(),
                    unidentified,
                    held_out_covered,
                    wrong: result.score.wrong_confident.len(),
                },
                samples,
            ))
        })
        .into_iter()
        .collect::<Result<_, String>>()?;
    let render_and_search_seconds = started.elapsed().as_secs_f64();
    if !options.keep_renders {
        fs::remove_dir_all(&renders)
            .map_err(|error| format!("cannot delete {}: {error}", renders.display()))?;
    }
    let rate = f64::from(options.matching.profile().sample_rate);
    let mut runs = Vec::new();
    for &mode in options.modes {
        let pieces: Vec<(usize, (f64, f64), &[f32])> = searched
            .iter()
            .enumerate()
            .flat_map(|(mix, (search, samples))| {
                let stretches = match mode {
                    Mode::Gaps => search.unidentified.clone(),
                    Mode::Whole => vec![(0.0, batch.mixes[mix].seconds)],
                };
                pieces(&stretches)
                    .into_iter()
                    .map(move |span| (mix, span, slice(samples, rate, span)))
            })
            .collect();
        for &range in options.ranges {
            let rungs = relative_ladder(range);
            let Searched {
                links,
                postings,
                index_seconds,
                search_seconds,
                search_thread_seconds,
            } = pairwise(&pieces, &rungs, options.matching, options.jobs)?;
            let found = correspondences(&links);
            let score = score(&batch, &found, |asset| clusters.cluster_of(asset));
            runs.push(Run {
                mode,
                relative_speed_range: range,
                rungs: rungs.len(),
                slowest: rungs[0].speed().0,
                fastest: rungs[rungs.len() - 1].speed().0,
                pieces: pieces.len(),
                piece_seconds: pieces.iter().map(|(_, (start, end), _)| end - start).sum(),
                postings,
                index_seconds,
                search_seconds,
                search_thread_seconds,
                links,
                score,
            });
        }
    }
    Ok(PassagesReport {
        seed: options.seed,
        build: std::env::current_exe()
            .map(|path| path.display().to_string())
            .unwrap_or_default(),
        indexed_assets: index.assets().len(),
        batch,
        render_and_search_seconds,
        library_searches: searched.into_iter().map(|(search, _)| search).collect(),
        runs,
    })
}

pub fn print_summary(report: &PassagesReport) {
    println!(
        "seed {}: {} mixes, {} indexed files; renders and library searches {:.0} s",
        report.seed,
        report.batch.mixes.len(),
        report.indexed_assets,
        report.render_and_search_seconds
    );
    for run in &report.runs {
        let subjects: Vec<&Positive> = run
            .score
            .positives
            .iter()
            .filter(|positive| matches!(positive.roles[0], Role::Subject { .. }))
            .collect();
        let found = subjects.iter().filter(|positive| positive.found).count();
        let count = |verdict: Verdict| {
            run.score
                .correspondences
                .iter()
                .filter(|scored| scored.verdict == verdict)
                .count()
        };
        println!(
            "{:?} ±{:.0}% ({} rungs): {} pieces, search {:.0} s ({:.0} thread s); subjects' positives {found} of {}; right {}, insert {}, one source another passage {}, shared passage {}, other {}; control pairs linked {} of {}",
            run.mode,
            100.0 * run.relative_speed_range,
            run.rungs,
            run.pieces,
            run.search_seconds,
            run.search_thread_seconds,
            subjects.len(),
            count(Verdict::Right),
            count(Verdict::Insert),
            count(Verdict::OneSourceAnotherPassage),
            count(Verdict::SharedPassage),
            count(Verdict::Other),
            run.score
                .control_pairs
                .iter()
                .filter(|pair| pair.linked)
                .count(),
            run.score.control_pairs.len(),
        );
    }
}

/// Where the report goes.
pub fn report_path(out: &Path, seed: u64) -> PathBuf {
    out.join(format!("passages-seed-{seed}.json"))
}

pub fn save(out: &Path, report: &PassagesReport) -> Result<(), String> {
    write_json(&report_path(out, report.seed), report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matching::Fit;
    use crate::stereo::FrontEnd;

    fn span(mix: usize, start_seconds: f64, end_seconds: f64) -> Span {
        Span {
            mix,
            start_seconds,
            end_seconds,
        }
    }

    fn link(query: Span, reference: Span) -> Link {
        Link {
            query,
            reference,
            speed: 1.0,
            hits: 500,
            windows: 6,
        }
    }

    #[test]
    fn the_two_directions_of_one_match_are_one_correspondence() {
        let found = correspondences(&[
            link(span(0, 100.0, 160.0), span(1, 300.0, 360.0)),
            link(span(1, 305.0, 365.0), span(0, 104.0, 166.0)),
        ]);

        assert_eq!(found.len(), 1);
        assert_eq!(found[0].a, span(0, 100.0, 166.0));
        assert_eq!(found[0].b, span(1, 300.0, 365.0));
        assert_eq!(found[0].links, 2);
    }

    #[test]
    fn links_through_a_shared_mix_are_never_merged_into_a_third() {
        let found = correspondences(&[
            link(span(0, 100.0, 160.0), span(1, 300.0, 360.0)),
            link(span(1, 300.0, 360.0), span(2, 50.0, 110.0)),
            link(span(0, 400.0, 460.0), span(1, 300.0, 360.0)),
        ]);

        let mixes: Vec<(usize, usize)> = found
            .iter()
            .map(|correspondence| (correspondence.a.mix, correspondence.b.mix))
            .collect();
        assert_eq!(mixes, [(0, 1), (1, 2), (0, 1)]);
        assert_eq!(found[0].a, span(0, 100.0, 160.0));
        assert_eq!(found[2].a, span(0, 400.0, 460.0));
    }

    fn subject_play(mix: usize, start: f64, source_start: f64) -> BatchPlay {
        BatchPlay {
            role: Role::Subject {
                subject: 0,
                occurrence: mix,
                excerpts: Excerpts::Overlapping,
            },
            play: play("subject.mp3", source_start, 1.0, start, 120.0),
        }
    }

    fn batch_of(plays: Vec<BatchPlay>) -> Batch {
        Batch {
            mixes: plays
                .into_iter()
                .enumerate()
                .map(|(number, play)| BatchMix {
                    name: format!("mix-{number}"),
                    seconds: 400.0,
                    plays: vec![play],
                })
                .collect(),
            shared: Vec::new(),
            left_out: BTreeSet::new(),
        }
    }

    fn singleton(asset: &str) -> BTreeSet<String> {
        BTreeSet::from([asset.to_owned()])
    }

    #[test]
    fn a_right_link_of_a_with_b_and_of_b_with_c_does_not_find_a_with_c() {
        // One recording, the same source passage, in three mixes.
        let batch = batch_of(vec![
            subject_play(0, 100.0, 60.0),
            subject_play(1, 200.0, 60.0),
            subject_play(2, 0.0, 60.0),
        ]);
        let found = correspondences(&[
            link(span(0, 110.0, 200.0), span(1, 210.0, 300.0)),
            link(span(1, 220.0, 290.0), span(2, 20.0, 90.0)),
        ]);

        let score = score(&batch, &found, singleton);

        let verdicts: Vec<Verdict> = score.correspondences.iter().map(|c| c.verdict).collect();
        assert_eq!(verdicts, [Verdict::Right, Verdict::Right]);
        let found: Vec<(usize, usize, bool)> = score
            .positives
            .iter()
            .map(|positive| (positive.plays[0].mix, positive.plays[1].mix, positive.found))
            .collect();
        assert_eq!(found, [(0, 1, true), (0, 2, false), (1, 2, true)]);
    }

    #[test]
    fn a_link_between_two_passages_of_one_source_is_not_right() {
        let batch = batch_of(vec![
            subject_play(0, 100.0, 60.0),
            subject_play(1, 100.0, 60.0),
        ]);
        // 60 s later in the second mix: the record's own repeat.
        let found = correspondences(&[link(span(0, 110.0, 150.0), span(1, 170.0, 210.0))]);

        let score = score(&batch, &found, singleton);

        assert_eq!(
            score.correspondences[0].verdict,
            Verdict::OneSourceAnotherPassage
        );
        assert!(!score.positives[0].found);
    }

    #[test]
    fn a_range_of_eight_percent_is_the_products_ladder() {
        assert_eq!(extra_rungs_for(0.08), 0);
        assert_eq!(relative_ladder(0.08), ladder());
    }

    #[test]
    fn a_range_of_eighteen_percent_reaches_opposite_deck_settings() {
        let rungs = relative_ladder(0.18);
        let (slowest, fastest) = (rungs[0].speed().0, rungs[rungs.len() - 1].speed().0);

        assert_eq!(extra_rungs_for(0.18), 25);
        assert_eq!(rungs.len(), 91);
        assert!((slowest - 0.82).abs() < 1e-9 && (fastest - 1.18).abs() < 1e-9);
        assert!(slowest < 0.92 / 1.08 && 1.08 / 0.92 < fastest);
    }

    const MATCHING: Matching = Matching {
        single_pass: false,
        drop_fullest: 0.0,
        speed_per_stretch: false,
        trim_ends: false,
        overlap_by_spans: false,
        overlap_by_chains: false,
        apart_in_speed: None,
        front_end: FrontEnd::Downmix,
        second_pass_fit: Fit::ChainSpeed,
    };

    /// Tone bursts at pseudo-random frequencies, a new pair every 50 ms.
    fn bursts(seconds: f64, rate: u32) -> Vec<f32> {
        let mut rng = Rng::new(69);
        let burst = (0.05 * f64::from(rate)) as usize;
        let mut samples = vec![0.0_f32; (seconds * f64::from(rate)) as usize];
        for chunk in samples.chunks_mut(burst) {
            let tones = [200.0 + 3300.0 * rng.unit(), 200.0 + 3300.0 * rng.unit()];
            for (i, sample) in chunk.iter_mut().enumerate() {
                let t = i as f64 / f64::from(rate);
                let envelope = (std::f64::consts::PI * i as f64 / burst as f64).sin();
                let value: f64 = tones
                    .iter()
                    .map(|hz| (2.0 * std::f64::consts::PI * hz * t).sin())
                    .sum();
                *sample = (0.3 * envelope * value) as f32;
            }
        }
        samples
    }

    /// `samples` played `speed` times faster, by linear interpolation.
    fn faster(samples: &[f32], speed: f64) -> Vec<f32> {
        (0..((samples.len() - 2) as f64 / speed) as usize)
            .map(|i| {
                let at = i as f64 * speed;
                let (whole, fraction) = (at.floor() as usize, at.fract() as f32);
                samples[whole] * (1.0 - fraction) + samples[whole + 1] * fraction
            })
            .collect()
    }

    #[test]
    fn a_passage_played_fifteen_percent_faster_is_found_only_within_eighteen_percent() {
        let profile = MATCHING.profile();
        let original = bursts(60.0, profile.sample_rate);
        let played = faster(&original, 1.15);
        let index = MATCHING
            .index(Index::build(&[piece_record(0, (0.0, 90.0), &original, &MATCHING)]).unwrap());
        let confident = |range: f64| -> Vec<f64> {
            MATCHING
                .search(&index, &played, &profile, &relative_ladder(range), 1)
                .into_iter()
                .filter(|detection| detection.evidence.is_confident())
                .map(|detection| detection.speed.0)
                .collect()
        };

        assert!(confident(0.08).is_empty());
        let speeds = confident(0.18);
        assert_eq!(speeds.len(), 1);
        assert!((speeds[0] - 1.15).abs() < 0.004, "{speeds:?}");
    }

    #[test]
    fn a_long_stretch_is_cut_into_overlapping_pieces() {
        assert_eq!(
            pieces(&[(0.0, 1300.0), (1400.0, 1450.0)]),
            [
                (0.0, 600.0),
                (540.0, 1140.0),
                (1080.0, 1300.0),
                (1400.0, 1450.0)
            ]
        );
    }
}
