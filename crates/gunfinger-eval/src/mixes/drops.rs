//! The drops kind: mixes of the situations a printed tracklist must get
//! right (`Scenario`), in a fixed order, so that each occurs once per
//! mix. Two records play together in a double drop (both at full level,
//! each entering and leaving with a fade) or one plays inside a stronger
//! one, quietly or briefly; a pair of the shared-material panel plays apart
//! and together; a record returns; two rips of one recording play at two
//! times; a held-out record plays alone. Blocks are joined by `join`'s
//! transitions, and every play is beatmatched (tempo, not beat phase) to
//! the play it is mixed with. With `DropsOptions::aligned_drops`, the two
//! records of each unrelated double drop also reach their drops at one
//! mix time, on the beat (`line_up_drops`).

use std::collections::{BTreeMap, BTreeSet};

use gunfinger_core::parallel::map_in_order;
use gunfinger_core::store::PeakRecord;

use super::{
    LongExtras, Pick, PlannedMix, PlannedPlay, Pools, Scenario, SharedPair, Side,
    beatmatched_speed, draw_unused, drawn_speed, join, placed,
};
use crate::clusters::Clusters;
use crate::render::{Playback, RENDER_RATE, render_samples};
use crate::rng::Rng;
use crate::shared::Roots;
use crate::tempo::Structure;

/// A play alone, and the first record of two, lasts this long, or longer
/// where the second needs room.
const PLAY_SECONDS: (f64, f64) = (100.0, 200.0);
/// A double drop: both records at full level for this long, the second
/// entering and the one that leaves leaving with a linear fade of
/// `DROP_FADE_SECONDS`.
const DROP_SECONDS: (f64, f64) = (30.0, 90.0);
const DROP_FADE_SECONDS: (f64, f64) = (4.0, 16.0);
/// The quiet record: this far below the others, as long as a drop.
pub const QUIET_DB: f64 = -9.0;
/// The brief record: this long in all, its fades of `BRIEF_FADE_SECONDS`
/// included, at full level.
pub const BRIEF_SECONDS: (f64, f64) = (20.0, 40.0);
const BRIEF_FADE_SECONDS: (f64, f64) = (2.0, 5.0);
/// The first record plays alone for at least this long before the second
/// comes in, and after it leaves when the first plays on.
const SOLO_SECONDS: f64 = 30.0;
/// Both records of two together are at least this long, so that a play
/// with a drop of the longest kind fits in the track.
const TOGETHER_MIN_SECONDS: f64 = 240.0;
/// A play ends before this share of its track (`in_track`).
const TRACK_END: f64 = 0.97;

/// How the second of two records joins the first: its fades, how long both
/// play between them, its level, and which record plays on.
#[derive(Debug, Clone, Copy)]
struct Overlay {
    fade_in: f64,
    seconds: f64,
    fade_out: f64,
    gain_db: f64,
    stays: Side,
}

impl Overlay {
    /// A double drop; `stays` plays on after it.
    fn drop(stays: Side, rng: &mut Rng) -> Overlay {
        Overlay {
            fade_in: within(DROP_FADE_SECONDS, rng),
            seconds: within(DROP_SECONDS, rng),
            fade_out: within(DROP_FADE_SECONDS, rng),
            gain_db: 0.0,
            stays,
        }
    }

    fn quiet(rng: &mut Rng) -> Overlay {
        Overlay {
            gain_db: QUIET_DB,
            ..Overlay::drop(Side::First, rng)
        }
    }

    fn brief(rng: &mut Rng) -> Overlay {
        let length = within(BRIEF_SECONDS, rng);
        let fade_in = within(BRIEF_FADE_SECONDS, rng);
        let fade_out = within(BRIEF_FADE_SECONDS, rng);
        Overlay {
            fade_in,
            seconds: length - fade_in - fade_out,
            fade_out,
            gain_db: 0.0,
            stays: Side::First,
        }
    }

    /// The second record's time in the mix while both play.
    fn length(&self) -> f64 {
        self.fade_in + self.seconds + self.fade_out
    }

    /// How long the first record must play to hold the second with its
    /// solo time around it.
    fn room(&self) -> f64 {
        match self.stays {
            Side::First => self.length() + 2.0 * SOLO_SECONDS,
            Side::Second => self.length() + SOLO_SECONDS,
        }
    }
}

enum Block<'a> {
    Alone(Pick<'a>, Scenario),
    Together {
        scenario: Scenario,
        first: Pick<'a>,
        second: Pick<'a>,
        overlay: Overlay,
    },
}

/// The pairs whose two files are indexed and, with `tagged`, among the
/// files a mix may play; with `other_pairs`, also pairs whose tagged files
/// the index lacks, each record marked with whether it is indexed. A
/// held-out file never plays as a pair's record.
pub(super) fn playable(
    pairs: Vec<SharedPair>,
    records: &[PeakRecord],
    held_out: &BTreeSet<String>,
    tagged: Option<&BTreeSet<String>>,
    other_pairs: bool,
) -> Vec<SharedPair> {
    let indexed: BTreeSet<&str> = records
        .iter()
        .map(|record| record.header.source.path.as_str())
        .filter(|path| !held_out.contains(*path))
        .collect();
    let allowed = |asset: &str| {
        !held_out.contains(asset)
            && tagged.is_none_or(|tagged| tagged.contains(asset))
            && (other_pairs || indexed.contains(asset))
    };
    pairs
        .into_iter()
        .filter(|pair| pair.records.iter().all(|record| allowed(&record.asset)))
        .map(|mut pair| {
            for record in &mut pair.records {
                record.indexed = indexed.contains(record.asset.as_str());
            }
            pair
        })
        .collect()
}

/// Draws a mix of the drops kind: its blocks, then their plays.
pub(super) fn draw(
    name: &str,
    pools: &Pools,
    extras: &LongExtras,
    clusters: &Clusters,
    rng: &mut Rng,
) -> Result<PlannedMix, String> {
    let blocks = draw_blocks(pools, &extras.pairs, clusters, rng)?;
    let mut plays: Vec<PlannedPlay> = Vec::new();
    // The play the next block is joined to.
    let mut current: Option<usize> = None;
    for block in &blocks {
        let previous = current.map(|index| &mut plays[index]);
        match block {
            Block::Alone(pick, scenario) => {
                let wanted = within(PLAY_SECONDS, rng);
                let mut play = next_play(pick, previous, wanted, &extras.periods, rng);
                play.scenario = Some(*scenario);
                plays.push(play);
                current = Some(plays.len() - 1);
            }
            Block::Together {
                scenario,
                first,
                second,
                overlay,
            } => {
                let mut plays_of_two =
                    together((first, second), overlay, previous, &extras.periods, rng);
                if *scenario == Scenario::UnrelatedDrop {
                    let (outer, inner) = &mut plays_of_two;
                    line_up_drops((outer, inner), (first, second), overlay, &extras.structures);
                }
                let (first, second) = plays_of_two;
                for (mut play, side) in [(first, Side::First), (second, Side::Second)] {
                    play.scenario = Some(*scenario);
                    play.side = Some(side);
                    plays.push(play);
                }
                current = Some(match overlay.stays {
                    Side::First => plays.len() - 2,
                    Side::Second => plays.len() - 1,
                });
            }
        }
    }
    let seconds = plays
        .iter()
        .map(|play| play.end_seconds)
        .fold(0.0, f64::max);
    Ok(PlannedMix {
        name: name.to_owned(),
        seconds,
        plays,
    })
}

/// The blocks of a mix in their fixed order, every record drawn from a
/// recording of its own apart from the repeats the scenarios make.
fn draw_blocks<'a>(
    pools: &Pools<'a>,
    pairs: &'a [SharedPair],
    clusters: &Clusters,
    rng: &mut Rng,
) -> Result<Vec<Block<'a>>, String> {
    if pairs.is_empty() {
        return Err(String::from(
            "no pair of the shared-material panel has both files indexed for the drops kind",
        ));
    }
    let long: Vec<&PeakRecord> = pools
        .indexed
        .iter()
        .copied()
        .filter(|record| record.header.duration_seconds >= TOGETHER_MIN_SECONDS)
        .collect();
    let mut used: BTreeSet<BTreeSet<String>> = BTreeSet::new();
    let apart = &pairs[rng.below(pairs.len())];
    let paired = &pairs[rng.below(pairs.len())];
    let two = |used: &mut BTreeSet<BTreeSet<String>>, rng: &mut Rng| {
        Ok::<_, String>((
            pick(&long, used, clusters, rng)?,
            pick(&long, used, clusters, rng)?,
        ))
    };
    let rips = draw_rips(&pools.indexed, &mut used, clusters, rng).ok_or_else(too_few)?;
    let unrelated_through = two(&mut used, rng)?;
    let returning = pick(&pools.indexed, &mut used, clusters, rng)?;
    let quiet = two(&mut used, rng)?;
    let brief = two(&mut used, rng)?;
    let unrelated_inside = two(&mut used, rng)?;
    let held_out = Pick {
        held_out: true,
        ..pick(&pools.held_out, &mut used, clusters, rng)?
    };
    let pair_stays = if rng.unit() < 0.5 {
        Side::First
    } else {
        Side::Second
    };
    let two_records = |scenario, (first, second), overlay| Block::Together {
        scenario,
        first,
        second,
        overlay,
    };
    Ok(vec![
        Block::Alone(of_pair(apart, 0), Scenario::Original),
        two_records(
            Scenario::UnrelatedDrop,
            unrelated_through,
            Overlay::drop(Side::Second, rng),
        ),
        Block::Alone(returning.clone(), Scenario::Returning),
        two_records(Scenario::Quiet, quiet, Overlay::quiet(rng)),
        Block::Alone(rips.0, Scenario::Rips),
        two_records(
            Scenario::PairDrop,
            (of_pair(paired, 0), of_pair(paired, 1)),
            Overlay::drop(pair_stays, rng),
        ),
        two_records(Scenario::Brief, brief, Overlay::brief(rng)),
        Block::Alone(held_out, Scenario::HeldOut),
        two_records(
            Scenario::UnrelatedDrop,
            unrelated_inside,
            Overlay::drop(Side::First, rng),
        ),
        Block::Alone(of_pair(apart, 1), Scenario::Remix),
        Block::Alone(returning, Scenario::Returning),
        Block::Alone(rips.1, Scenario::Rips),
    ])
}

fn too_few() -> String {
    String::from("the library has too few tracks for a mix of double drops")
}

/// A record of `pool` from a recording not used yet, which it then joins.
fn pick<'a>(
    pool: &[&'a PeakRecord],
    used: &mut BTreeSet<BTreeSet<String>>,
    clusters: &Clusters,
    rng: &mut Rng,
) -> Result<Pick<'a>, String> {
    draw_unused(pool, used, clusters, rng)
        .map(|record| Pick::of(record, false))
        .ok_or_else(too_few)
}

/// The pair's record `which` (0: the played one, 1: the related one), to
/// be heard over its passage.
fn of_pair(pair: &SharedPair, which: usize) -> Pick<'_> {
    let record = &pair.records[which];
    Pick {
        asset: &record.asset,
        duration_seconds: record.duration_seconds,
        held_out: !record.indexed,
        passage: Some(record.passage),
    }
}

/// Two files of a recording of two or more indexed files, none of whose
/// clusters is used yet.
fn draw_rips<'a>(
    indexed: &[&'a PeakRecord],
    used: &mut BTreeSet<BTreeSet<String>>,
    clusters: &Clusters,
    rng: &mut Rng,
) -> Option<(Pick<'a>, Pick<'a>)> {
    let by_path: BTreeMap<&str, &'a PeakRecord> = indexed
        .iter()
        .map(|record| (record.header.source.path.as_str(), *record))
        .collect();
    let candidates: Vec<&'a PeakRecord> = indexed
        .iter()
        .copied()
        .filter(|record| {
            let cluster = clusters.cluster_of(&record.header.source.path);
            cluster.len() >= 2
                && !used.contains(&cluster)
                && cluster
                    .iter()
                    .all(|path| by_path.contains_key(path.as_str()))
        })
        .collect();
    if candidates.is_empty() {
        return None;
    }
    let first = candidates[rng.below(candidates.len())];
    let cluster = clusters.cluster_of(&first.header.source.path);
    let others: Vec<&String> = cluster
        .iter()
        .filter(|path| **path != first.header.source.path)
        .collect();
    let second = by_path[others[rng.below(others.len())].as_str()];
    used.insert(cluster);
    Some((Pick::of(first, false), Pick::of(second, false)))
}

/// A play of `pick` for about `wanted` seconds, beatmatched to `previous`
/// and joined to it by `join`, or from the mix's start.
fn next_play(
    pick: &Pick,
    previous: Option<&mut PlannedPlay>,
    wanted: f64,
    periods: &BTreeMap<String, f64>,
    rng: &mut Rng,
) -> PlannedPlay {
    let drawn = drawn_speed(rng);
    let speed = previous
        .as_deref()
        .and_then(|previous| beatmatched_speed(previous, pick.asset, periods))
        .unwrap_or(drawn);
    let start = previous
        .as_deref()
        .map_or(0.0, |previous| previous.end_seconds);
    let seconds = wanted.min(0.85 * pick.duration_seconds / speed);
    let mut play = placed(pick, start, seconds, speed, rng);
    if let Some(previous) = previous {
        join(previous, &mut play, rng);
    }
    play
}

/// The plays of two records together: the first joined to `previous`, the
/// second beatmatched to it and laid over it as `overlay` says. A pair's
/// records are placed so that both passages start where both play at full
/// level, as far as the tracks allow.
fn together(
    (first, second): (&Pick, &Pick),
    overlay: &Overlay,
    previous: Option<&mut PlannedPlay>,
    periods: &BTreeMap<String, f64>,
    rng: &mut Rng,
) -> (PlannedPlay, PlannedPlay) {
    let wanted = within(PLAY_SECONDS, rng).max(overlay.room());
    let mut outer = next_play(first, previous, wanted, periods, rng);
    let length = outer.end_seconds - outer.start_seconds;
    // Where both start to play at full level.
    let full = match overlay.stays {
        Side::First => {
            outer.start_seconds
                + SOLO_SECONDS
                + overlay.fade_in
                + rng.unit() * (length - overlay.room()).max(0.0)
        }
        Side::Second => outer.end_seconds - overlay.fade_out - overlay.seconds,
    };
    let drawn = drawn_speed(rng);
    let speed = beatmatched_speed(&outer, second.asset, periods).unwrap_or(drawn);
    let seconds = match overlay.stays {
        Side::First => overlay.length(),
        Side::Second => overlay.length() + within(PLAY_SECONDS, rng),
    }
    .min(0.85 * second.duration_seconds / speed);
    let mut inner = placed(second, full - overlay.fade_in, seconds, speed, rng);
    inner.fade_in_seconds = overlay.fade_in;
    inner.gain_db = overlay.gain_db;
    match overlay.stays {
        Side::First => inner.fade_out_seconds = overlay.fade_out,
        Side::Second => outer.fade_out_seconds = overlay.fade_out,
    }
    if let (Some(outer_passage), Some(inner_passage)) = (first.passage, second.passage) {
        outer.source_start_seconds = in_track(
            outer_passage.0 - (full - outer.start_seconds) * outer.speed,
            first.duration_seconds,
            length * outer.speed,
        );
        inner.source_start_seconds = in_track(
            inner_passage.0 - overlay.fade_in * inner.speed,
            second.duration_seconds,
            seconds * inner.speed,
        );
    }
    (outer, inner)
}

/// `start` moved into the track, so that `heard` seconds of it from there
/// end before 97% of a track of `duration`.
fn in_track(start: f64, duration: f64, heard: f64) -> f64 {
    start.min(TRACK_END * duration - heard).max(0.0)
}

/// The structure of every record two together may play (the indexed
/// records of `TOGETHER_MIN_SECONDS` or more with a beat period), from its
/// whole audio at native speed.
pub(super) fn structures(
    pools: &Pools,
    periods: &BTreeMap<String, f64>,
    roots: &Roots,
    jobs: usize,
) -> BTreeMap<String, Structure> {
    let wanted: Vec<(String, f64, f64)> = pools
        .indexed
        .iter()
        .filter(|record| record.header.duration_seconds >= TOGETHER_MIN_SECONDS)
        .filter_map(|record| {
            let path = &record.header.source.path;
            let period = *periods.get(path)?;
            Some((path.clone(), record.header.duration_seconds, period))
        })
        .collect();
    let found = map_in_order(&wanted, jobs, |(asset, duration, period)| {
        let samples = render_samples(
            &roots.path(asset).ok()?,
            0.0,
            *duration,
            Playback::Turntable(1.0),
            None,
        )
        .ok()?;
        Some(Structure::of(&samples, RENDER_RATE, *period))
    });
    wanted
        .into_iter()
        .zip(found)
        .filter_map(|((asset, _, _), structure)| structure.map(|structure| (asset, structure)))
        .collect()
}

/// Moves two records of a double drop so that each reaches, at one mix
/// time, the strongest drop it can play then (`Structure::drop_within`),
/// as a DJ lines drops up: the first keeps its span in the mix and moves
/// in its track, and the moment both reach full level moves as little as
/// it must within the overlay's room; the second moves with that moment
/// and in its track. Both are beatmatched, so the drops' beats stay in
/// phase. Where either has no structure or no drop fits, nothing moves.
fn line_up_drops(
    (outer, inner): (&mut PlannedPlay, &mut PlannedPlay),
    (first, second): (&Pick, &Pick),
    overlay: &Overlay,
    structures: &BTreeMap<String, Structure>,
) {
    let (Some(outer_structure), Some(inner_structure)) =
        (structures.get(first.asset), structures.get(second.asset))
    else {
        return;
    };
    let drawn_full = inner.start_seconds + overlay.fade_in;
    // The mix times at which both may reach full level.
    let (earliest, latest) = match overlay.stays {
        Side::First => (
            outer.start_seconds + SOLO_SECONDS + overlay.fade_in,
            outer.end_seconds - SOLO_SECONDS - overlay.fade_out - overlay.seconds,
        ),
        Side::Second => (drawn_full, drawn_full),
    };
    if latest < earliest {
        return;
    }
    let outer_heard = (outer.end_seconds - outer.start_seconds) * outer.speed;
    let latest_source = TRACK_END * first.duration_seconds - outer_heard;
    let Some(outer_drop) = outer_structure.drop_within(
        (earliest - outer.start_seconds) * outer.speed,
        latest_source + (latest - outer.start_seconds) * outer.speed,
    ) else {
        return;
    };
    // The drawn moment where the first's source start allows it, else the
    // nearest that does.
    let full = drawn_full
        .min(outer.start_seconds + outer_drop / outer.speed)
        .max(outer.start_seconds + (outer_drop - latest_source) / outer.speed)
        .clamp(earliest, latest);
    let inner_heard = (inner.end_seconds - inner.start_seconds) * inner.speed;
    let entering = overlay.fade_in * inner.speed;
    let Some(inner_drop) = inner_structure.drop_within(
        entering,
        TRACK_END * second.duration_seconds - inner_heard + entering,
    ) else {
        return;
    };
    outer.source_start_seconds = outer_drop - (full - outer.start_seconds) * outer.speed;
    let shift = full - drawn_full;
    inner.start_seconds += shift;
    inner.end_seconds += shift;
    inner.source_start_seconds = inner_drop - entering;
    outer.drop_seconds = Some(full);
    inner.drop_seconds = Some(full);
}

fn within((low, high): (f64, f64), rng: &mut Rng) -> f64 {
    low + rng.unit() * (high - low)
}

#[cfg(test)]
mod tests {
    use gunfinger_core::library::{Asset, Timestamp};
    use gunfinger_core::profile::Profile;
    use gunfinger_core::store::RecordHeader;

    use super::super::PairRecord;
    use super::*;

    fn record(path: &str, duration_seconds: f64) -> PeakRecord {
        PeakRecord {
            header: RecordHeader {
                profile: Profile::CURRENT.id(),
                source: Asset {
                    path: path.to_owned(),
                    size: 1,
                    modified: Timestamp {
                        seconds: 0,
                        nanos: 0,
                    },
                },
                duration_seconds,
            },
            peaks: Vec::new(),
        }
    }

    fn pair() -> SharedPair {
        SharedPair {
            family: String::from("clockwork"),
            records: [
                PairRecord {
                    asset: String::from("original.mp3"),
                    duration_seconds: 420.0,
                    passage: (205.4, 224.1),
                    indexed: true,
                },
                PairRecord {
                    asset: String::from("remix.mp3"),
                    duration_seconds: 520.0,
                    passage: (399.5, 418.2),
                    indexed: true,
                },
            ],
        }
    }

    /// 24 tracks of 160-390 s, two of them one recording, four held out,
    /// every beat period alike so that beatmatching keeps each speed.
    fn library() -> (Vec<PeakRecord>, BTreeSet<String>, Clusters, LongExtras) {
        let records: Vec<PeakRecord> = (0..24)
            .map(|n| record(&format!("t{n:02}.mp3"), 160.0 + 10.0 * f64::from(n)))
            .collect();
        let held_out = (20..24).map(|n| format!("t{n:02}.mp3")).collect();
        let clusters = Clusters {
            criterion: String::new(),
            duplicates: vec![vec![String::from("t02.mp3"), String::from("t03.mp3")]],
            pairs: Vec::new(),
            cut_links: Vec::new(),
        };
        let periods = records
            .iter()
            .map(|record| (record.header.source.path.clone(), 0.35))
            .chain([
                (String::from("original.mp3"), 0.35),
                (String::from("remix.mp3"), 0.35),
            ])
            .collect();
        let extras = LongExtras {
            periods,
            structures: BTreeMap::new(),
            pairs: vec![pair()],
            rides: super::super::RideDraw::default(),
        };
        (records, held_out, clusters, extras)
    }

    fn full_level_overlap(first: &PlannedPlay, second: &PlannedPlay) -> f64 {
        (first.end_seconds - first.fade_out_seconds)
            .min(second.end_seconds - second.fade_out_seconds)
            - (first.start_seconds + first.fade_in_seconds)
                .max(second.start_seconds + second.fade_in_seconds)
    }

    fn inside(outer: &PlannedPlay, inner: &PlannedPlay) -> bool {
        outer.start_seconds + SOLO_SECONDS <= inner.start_seconds + 1e-9
            && inner.end_seconds + SOLO_SECONDS <= outer.end_seconds + 1e-9
    }

    #[test]
    fn a_mix_of_drops_plays_every_scenario_in_its_order() {
        let (records, held_out, clusters, extras) = library();
        let pools = Pools::new(&records, &held_out);
        for seed in 0..20 {
            let mix = draw("mix-00", &pools, &extras, &clusters, &mut Rng::new(seed)).unwrap();
            let plays = &mix.plays;

            let scenarios: Vec<(Scenario, Option<Side>)> = plays
                .iter()
                .map(|play| (play.scenario.unwrap(), play.side))
                .collect();
            let (first, second) = (Some(Side::First), Some(Side::Second));
            assert_eq!(
                scenarios,
                [
                    (Scenario::Original, None),
                    (Scenario::UnrelatedDrop, first),
                    (Scenario::UnrelatedDrop, second),
                    (Scenario::Returning, None),
                    (Scenario::Quiet, first),
                    (Scenario::Quiet, second),
                    (Scenario::Rips, None),
                    (Scenario::PairDrop, first),
                    (Scenario::PairDrop, second),
                    (Scenario::Brief, first),
                    (Scenario::Brief, second),
                    (Scenario::HeldOut, None),
                    (Scenario::UnrelatedDrop, first),
                    (Scenario::UnrelatedDrop, second),
                    (Scenario::Remix, None),
                    (Scenario::Returning, None),
                    (Scenario::Rips, None),
                ]
            );
            // A double drop through: both at full level for 30-90 s, then
            // the second plays on; inside: the first plays on.
            let through = full_level_overlap(&plays[1], &plays[2]);
            assert!((30.0..=90.0).contains(&through), "{through}");
            assert!(plays[2].end_seconds > plays[1].end_seconds + SOLO_SECONDS);
            let within_drop = full_level_overlap(&plays[12], &plays[13]);
            assert!((30.0..=90.0).contains(&within_drop), "{within_drop}");
            assert!(inside(&plays[12], &plays[13]));
            // The quiet record: -9 dB inside the first; the brief one:
            // 20-40 s at full level inside the first.
            assert!(inside(&plays[4], &plays[5]));
            assert_eq!((plays[4].gain_db, plays[5].gain_db), (0.0, QUIET_DB));
            let brief = plays[10].end_seconds - plays[10].start_seconds;
            assert!((20.0..=40.0).contains(&brief), "{brief}");
            assert!(inside(&plays[9], &plays[10]));
            assert_eq!(plays[10].gain_db, 0.0);
            // The pair: apart over each passage, then together.
            for (play, record) in [(&plays[0], 0), (&plays[14], 1)] {
                let passage = pair().records[record].passage;
                assert_eq!(play.asset, pair().records[record].asset);
                assert!(play.source_start_seconds <= passage.0);
                assert!(play.source_at(play.end_seconds) >= passage.1);
            }
            assert_eq!(
                (plays[7].asset.as_str(), plays[8].asset.as_str()),
                ("original.mp3", "remix.mp3")
            );
            let drop = full_level_overlap(&plays[7], &plays[8]);
            assert!((30.0..=90.0).contains(&drop), "{drop}");
            // Where a passage fits, it starts where both play at full
            // level; else the play is moved to the track's start or end.
            let both_full = plays[8].start_seconds + plays[8].fade_in_seconds;
            for (play, record) in [(&plays[7], 0), (&plays[8], 1)] {
                let record = &pair().records[record];
                let heard = play.source_at(play.end_seconds) - play.source_start_seconds;
                let latest = 0.97 * record.duration_seconds - heard;
                let start = play.source_start_seconds;
                assert!(
                    (play.source_at(both_full) - record.passage.0).abs() < 1e-6
                        || start == 0.0
                        || (start - latest).abs() < 1e-9,
                    "{play:?}"
                );
            }
            // The repeats; a held-out record; every other record from a
            // recording of its own; every play inside its track.
            assert_eq!(plays[3].asset, plays[15].asset);
            assert_ne!(
                plays[3].source_start_seconds,
                plays[15].source_start_seconds
            );
            let rips = BTreeSet::from([plays[6].asset.as_str(), plays[16].asset.as_str()]);
            assert_eq!(rips, BTreeSet::from(["t02.mp3", "t03.mp3"]));
            assert!(
                plays
                    .iter()
                    .all(|play| play.held_out == (play.scenario == Some(Scenario::HeldOut)))
            );
            assert!(held_out.contains(&plays[11].asset));
            let recordings: BTreeSet<BTreeSet<String>> = plays
                .iter()
                .map(|play| clusters.cluster_of(&play.asset))
                .collect();
            // Repeats: the returning record, the rips and both pair records.
            assert_eq!(recordings.len(), plays.len() - 4);
            for play in plays {
                let duration = records
                    .iter()
                    .find(|record| record.header.source.path == play.asset)
                    .map_or_else(
                        || {
                            pair()
                                .records
                                .iter()
                                .find(|r| r.asset == play.asset)
                                .unwrap()
                                .duration_seconds
                        },
                        |record| record.header.duration_seconds,
                    );
                assert!(play.source_start_seconds >= 0.0);
                assert!(play.source_at(play.end_seconds) <= duration, "{play:?}");
            }
            // Beatmatched: the second of two plays at the first's speed.
            for (outer, inner) in [(1, 2), (4, 5), (7, 8), (9, 10), (12, 13)] {
                assert!((plays[outer].speed - plays[inner].speed).abs() < 1e-12);
            }
            assert!((mix.seconds - plays[16].end_seconds).abs() < 1e-9);
        }
    }

    #[test]
    fn aligned_drops_meet_on_the_beat_and_change_nothing_else() {
        let (records, held_out, clusters, plain) = library();
        // A drop every 29 beats, so that every window holds one.
        let drops: Vec<f64> = (1..40).map(|n| f64::from(n) * 29.0 * 0.35).collect();
        let mut extras = library().3;
        extras.structures = records
            .iter()
            .map(|record| {
                let seconds = record.header.duration_seconds;
                let path = record.header.source.path.clone();
                (path, Structure::with_drops(&drops, seconds, 0.35))
            })
            .collect();
        let pools = Pools::new(&records, &held_out);
        let mut lined_up = 0;
        for seed in 0..20 {
            let before = draw("mix-00", &pools, &plain, &clusters, &mut Rng::new(seed)).unwrap();
            let after = draw("mix-00", &pools, &extras, &clusters, &mut Rng::new(seed)).unwrap();
            let plays = &after.plays;

            for (index, (old, new)) in before.plays.iter().zip(plays).enumerate() {
                if [1, 2, 12, 13].contains(&index) {
                    assert_eq!((&old.asset, old.speed), (&new.asset, new.speed));
                } else {
                    assert_eq!(old, new);
                }
            }
            for (outer, inner) in [(1, 2), (12, 13)] {
                let (outer, inner) = (&plays[outer], &plays[inner]);
                let Some(at) = inner.drop_seconds else {
                    assert_eq!(outer.drop_seconds, None);
                    continue;
                };
                lined_up += 1;
                assert_eq!(outer.drop_seconds, Some(at));
                assert!((inner.start_seconds + inner.fade_in_seconds - at).abs() < 1e-9);
                for play in [outer, inner] {
                    // On a beat of the record's own grid, within a beat of a
                    // drop: the rise is found to half a second, and every
                    // beat here is as strong.
                    let source = play.source_at(at);
                    let drop = drops
                        .iter()
                        .map(|drop| (drop - source).abs())
                        .fold(f64::INFINITY, f64::min);
                    assert!(drop <= 0.36, "{source}");
                    let beats = source / 0.35;
                    assert!((beats - beats.round()).abs() * 0.35 < 0.011, "{source}");
                    let duration = records
                        .iter()
                        .find(|record| record.header.source.path == play.asset)
                        .unwrap()
                        .header
                        .duration_seconds;
                    assert!(play.source_start_seconds >= 0.0);
                    assert!(play.source_at(play.end_seconds) <= TRACK_END * duration + 1e-9);
                }
                let both = full_level_overlap(outer, inner);
                assert!((30.0..=90.0).contains(&both), "{both}");
            }
            assert!(inside(&plays[12], &plays[13]));
            assert!(plays[2].end_seconds > plays[1].end_seconds + SOLO_SECONDS);
        }
        assert!(lined_up >= 20, "{lined_up} of 40 drops lined up");
    }

    #[test]
    fn pairs_play_only_when_both_files_are_indexed_and_tagged() {
        let records = vec![record("original.mp3", 420.0), record("remix.mp3", 520.0)];
        let none = BTreeSet::new();

        assert_eq!(
            playable(vec![pair()], &records, &none, None, false).len(),
            1
        );
        let held_out = BTreeSet::from([String::from("remix.mp3")]);
        assert!(playable(vec![pair()], &records, &held_out, None, false).is_empty());
        assert!(playable(vec![pair()], &records, &held_out, None, true).is_empty());
        let tagged = BTreeSet::from([String::from("original.mp3")]);
        assert!(playable(vec![pair()], &records, &none, Some(&tagged), false).is_empty());
    }

    #[test]
    fn other_pairs_play_a_file_the_index_lacks_as_a_record_outside_it() {
        let records = vec![record("original.mp3", 420.0)];
        let none = BTreeSet::new();
        let tagged = BTreeSet::from([String::from("original.mp3"), String::from("remix.mp3")]);

        assert!(playable(vec![pair()], &records, &none, Some(&tagged), false).is_empty());
        let pairs = playable(vec![pair()], &records, &none, Some(&tagged), true);
        let indexed: Vec<bool> = pairs[0]
            .records
            .iter()
            .map(|record| record.indexed)
            .collect();
        assert_eq!(indexed, [true, false]);

        let (library, held_out, clusters, mut extras) = library();
        extras.pairs = pairs;
        let pools = Pools::new(&library, &held_out);
        let mix = draw("mix-00", &pools, &extras, &clusters, &mut Rng::new(3)).unwrap();
        for play in &mix.plays {
            assert_eq!(
                play.held_out,
                play.asset == "remix.mp3" || held_out.contains(&play.asset)
            );
        }
    }

    #[test]
    fn a_quiet_play_is_nine_decibels_down_and_a_plain_play_keeps_its_old_form() {
        let mut play = PlannedPlay {
            asset: String::from("a.mp3"),
            held_out: false,
            source_start_seconds: 10.0,
            speed: 1.0,
            start_seconds: 0.0,
            end_seconds: 30.0,
            fade_in_seconds: 0.0,
            fade_out_seconds: 0.0,
            bass_cut_until: None,
            bass_cut_from: None,
            ride: None,
            gain_db: 0.0,
            scenario: None,
            side: None,
            drop_seconds: None,
        };
        let plain = serde_json::to_string(&play).unwrap();
        assert!(
            !plain.contains("gain_db") && !plain.contains("scenario"),
            "{plain}"
        );

        play.gain_db = QUIET_DB;
        assert!((play.gain() - 0.354_813).abs() < 1e-6);
    }
}
