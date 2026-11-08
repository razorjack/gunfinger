//! Recording families (session 87): which library files belong together,
//! so that a result can say how many independent recordings or relations
//! it rests on, beside its query count (`docs/session-rules.md`).
//!
//! The key is built from stored relations only, from library audio and
//! listening, never from a search result. Each tier joins more than the
//! one before it:
//!
//! - `Recording`: the duplicate clusters, and each corpus file with the
//!   other library's file it stands for (the library map);
//! - `Title`: and each same-title judgment, the joins scoring credits as
//!   right (ADR 0022);
//! - `Relation`: and every pair a verdict by ear names, the same or
//!   different (a verdict is written on a pair a search found sharing
//!   material), and every stored clusters pair whose alignment covers
//!   `REPORTED_COVERAGE` or more of the shorter file with `RELATED_HITS`
//!   or more hits, and the recordings of each family of a shared-material
//!   panel (`docs/panels/shared-material-*.json`), which records relations
//!   judged outside the verdicts file (experiment 0026, answers in the
//!   answer log, notes). Chains are followed here, unlike in scoring, so a
//!   relation family can hold a remix and the remix's own remixes.
//!   Coverage alone would chain chance alignments: it counts the span
//!   from the first hit to the last, and most stored pairs over 20%
//!   have under 10 hits (session 87 joined 679 files that way).
//!
//! A file in no relation is its own family; a family is named by its
//! smallest member in byte order.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::clusters::{Clusters, RELATED_HITS, REPORTED_COVERAGE};
use crate::files::read_json;
use crate::rng::Rng;
use crate::scoring::Level;
use crate::sweep::{Outcome, Query};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Tier {
    Recording,
    Title,
    Relation,
}

pub const TIERS: [Tier; 3] = [Tier::Recording, Tier::Title, Tier::Relation];

/// The pairs of files each tier adds to the one before it.
#[derive(Default)]
pub struct Joins {
    pub same_recording: Vec<(String, String)>,
    pub same_title: Vec<(String, String)>,
    pub related: Vec<(String, String)>,
}

impl Joins {
    /// A clusters file's joins: its clusters' members, and its pairs that
    /// cover `REPORTED_COVERAGE` or more of the shorter file with
    /// `RELATED_HITS` or more hits as related.
    pub fn add_clusters(&mut self, clusters: &Clusters) {
        for members in &clusters.duplicates {
            for pair in members.windows(2) {
                self.same_recording.push((pair[0].clone(), pair[1].clone()));
            }
        }
        for pair in &clusters.pairs {
            if pair.coverage >= REPORTED_COVERAGE && pair.hits >= RELATED_HITS {
                self.related.push((pair.query.clone(), pair.found.clone()));
            }
        }
    }
}

/// A shared-material panel as frozen: each family's recordings share a
/// passage.
#[derive(Deserialize)]
struct SharedPanel {
    families: Vec<SharedFamily>,
}

#[derive(Deserialize)]
struct SharedFamily {
    recordings: Vec<Vec<String>>,
}

impl Joins {
    /// A shared-material panel's joins: each family's files as related.
    pub fn add_shared_panel(&mut self, path: &Path) -> Result<(), String> {
        let panel: SharedPanel = read_json(path)?;
        for family in panel.families {
            let files: Vec<String> = family.recordings.into_iter().flatten().collect();
            for pair in files.windows(2) {
                self.related.push((pair[0].clone(), pair[1].clone()));
            }
        }
        Ok(())
    }
}

/// Every file named by a join, with its family at each tier.
#[derive(Debug, Serialize, Deserialize)]
pub struct Families {
    /// What the joins were read from, for the reader of the key.
    pub sources: Vec<String>,
    pub files: BTreeMap<String, FileFamilies>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileFamilies {
    pub recording: String,
    pub title: String,
    pub relation: String,
}

impl Families {
    pub fn build(sources: Vec<String>, joins: &Joins) -> Families {
        let mut joined = Joined::default();
        let mut tiers = Vec::new();
        for (tier, pairs) in [
            (Tier::Recording, &joins.same_recording),
            (Tier::Title, &joins.same_title),
            (Tier::Relation, &joins.related),
        ] {
            for (a, b) in pairs {
                joined.join(a, b);
            }
            tiers.push((tier, joined.names()));
        }
        let mut files: BTreeMap<String, FileFamilies> = BTreeMap::new();
        let relation = &tiers[2].1;
        for file in relation.keys() {
            let name = |tier: usize| tiers[tier].1.get(file).unwrap_or(file).clone();
            files.insert(
                file.clone(),
                FileFamilies {
                    recording: name(0),
                    title: name(1),
                    relation: name(2),
                },
            );
        }
        Families { sources, files }
    }

    /// The family of `file` at `tier`: itself when no join names it.
    pub fn of<'a>(&'a self, file: &'a str, tier: Tier) -> &'a str {
        match self.files.get(file) {
            Some(families) => match tier {
                Tier::Recording => &families.recording,
                Tier::Title => &families.title,
                Tier::Relation => &families.relation,
            },
            None => file,
        }
    }

    /// The families with more than one member at `tier`, largest first,
    /// each with its size.
    pub fn largest(&self, tier: Tier) -> Vec<(String, usize)> {
        let mut sizes: BTreeMap<&str, usize> = BTreeMap::new();
        for file in self.files.keys() {
            *sizes.entry(self.of(file, tier)).or_default() += 1;
        }
        let mut largest: Vec<(String, usize)> = sizes
            .into_iter()
            .filter(|&(_, size)| size > 1)
            .map(|(name, size)| (name.to_owned(), size))
            .collect();
        largest.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        largest
    }
}

/// A union-find over file names.
#[derive(Default)]
struct Joined {
    parent: BTreeMap<String, String>,
}

impl Joined {
    fn root(&mut self, file: &str) -> String {
        let mut root = file.to_owned();
        while let Some(parent) = self.parent.get(&root).filter(|parent| **parent != root) {
            root = parent.clone();
        }
        let mut current = file.to_owned();
        while current != root {
            let next = self
                .parent
                .insert(current, root.clone())
                .unwrap_or_default();
            current = next;
        }
        root
    }

    fn join(&mut self, a: &str, b: &str) {
        for file in [a, b] {
            self.parent
                .entry(file.to_owned())
                .or_insert_with(|| file.to_owned());
        }
        let (a, b) = (self.root(a), self.root(b));
        // The smaller name becomes the root, so a family is named by its
        // smallest member.
        match a.cmp(&b) {
            std::cmp::Ordering::Less => {
                self.parent.insert(b, a);
            }
            std::cmp::Ordering::Greater => {
                self.parent.insert(a, b);
            }
            std::cmp::Ordering::Equal => {}
        }
    }

    /// Every file with its family's name.
    fn names(&mut self) -> BTreeMap<String, String> {
        let files: Vec<String> = self.parent.keys().cloned().collect();
        files
            .into_iter()
            .map(|file| {
                let root = self.root(&file);
                (file, root)
            })
            .collect()
    }
}

/// What a saved report holds that family scoring reads: `sweep` and
/// `recall` reports hold queries, `mixes` reports mixes.
#[derive(Deserialize)]
struct SavedReport {
    #[serde(default)]
    queries: Vec<SavedQuery>,
    #[serde(default)]
    mixes: Vec<SavedMix>,
}

#[derive(Deserialize)]
struct SavedQuery {
    /// A `recall` source none of whose cluster was indexed.
    #[serde(default)]
    unobservable: bool,
    #[serde(flatten)]
    query: Query,
}

#[derive(Deserialize)]
struct SavedMix {
    score: SavedScore,
}

#[derive(Deserialize)]
struct SavedScore {
    plays: Vec<SavedPlay>,
    wrong_confident: Vec<serde::de::IgnoredAny>,
}

#[derive(Deserialize)]
struct SavedPlay {
    truth: SavedTruth,
    level: Level,
}

#[derive(Deserialize)]
struct SavedTruth {
    asset: String,
    held_out: bool,
}

/// One scored item: a query, or a play of a generated mix.
#[derive(Debug, Clone)]
pub struct Item {
    pub asset: String,
    /// Indexed items are recalled or not; held-out ones are scored only by
    /// whether something confident answered.
    pub held_out: bool,
    pub recalled: bool,
    /// For a query, whether a confident detection named another title. A
    /// mix's wrong answers match no play, so they are counted per mix
    /// instead (`Scored::wrong_per_mix`).
    pub wrong: bool,
    /// The generated mix it was played in, for resampling whole mixes.
    pub mix: Option<usize>,
}

/// What resampling for an interval draws whole: families, or mixes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Unit {
    Family,
    Mix,
}

/// A saved report read as items, with the wrong confident answers of each
/// mix for a `mixes` report.
pub struct Scored {
    pub items: Vec<Item>,
    pub wrong_per_mix: Option<Vec<usize>>,
}

pub fn read_report(path: &Path) -> Result<Scored, String> {
    let report: SavedReport = read_json(path)?;
    if !report.mixes.is_empty() {
        let items = report
            .mixes
            .iter()
            .enumerate()
            .flat_map(|(number, mix)| {
                mix.score.plays.iter().map(move |play| Item {
                    asset: play.truth.asset.clone(),
                    held_out: play.truth.held_out,
                    recalled: !play.truth.held_out && play.level == Level::Confident,
                    wrong: false,
                    mix: Some(number),
                })
            })
            .collect();
        let wrong = report
            .mixes
            .iter()
            .map(|mix| mix.score.wrong_confident.len())
            .collect();
        return Ok(Scored {
            items,
            wrong_per_mix: Some(wrong),
        });
    }
    let items = report
        .queries
        .iter()
        .filter(|saved| !saved.unobservable)
        .map(|saved| {
            let query = &saved.query;
            let confident_right = query
                .detections
                .iter()
                .any(|outcome| outcome.is_right() && outcome.confident);
            Item {
                asset: query.asset.clone(),
                held_out: query.held_out,
                recalled: !query.held_out && confident_right,
                wrong: query.detections.iter().any(Outcome::is_wrong),
                mix: None,
            }
        })
        .collect();
    Ok(Scored {
        items,
        wrong_per_mix: None,
    })
}

/// A rate over items, weighted by item and by family, with intervals.
#[derive(Debug, Clone, Serialize)]
pub struct Rate {
    pub items: usize,
    pub count: usize,
    pub item_weighted: f64,
    pub family_weighted: f64,
    /// 2.5th and 97.5th percentiles of the item-weighted rate over
    /// resamples of whole units.
    pub item_weighted_interval: [f64; 2],
    pub family_weighted_interval: [f64; 2],
}

#[derive(Debug, Serialize)]
pub struct Summary {
    pub report: String,
    pub resampled: Unit,
    pub indexed: Group,
    pub held_out: Group,
    /// Wrong confident answers per mix (a `mixes` report).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrong_per_mix: Option<WrongPerMix>,
}

#[derive(Debug, Serialize)]
pub struct Group {
    /// Distinct families of the items' files at each tier.
    pub families: BTreeMap<Tier, usize>,
    pub mixes: usize,
    /// Indexed: the share recalled; held-out queries: the share with a
    /// confident wrong answer; none for a mix's held-out plays, whose
    /// wrong answers are counted per mix. Families at the relation tier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<Rate>,
    pub items: usize,
    /// Indexed queries only: the share with a confident wrong answer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrong: Option<Rate>,
}

#[derive(Debug, Serialize)]
pub struct WrongPerMix {
    pub mixes: usize,
    pub wrong: usize,
    pub per_mix: f64,
    pub interval: [f64; 2],
}

pub const RESAMPLES: usize = 2000;
pub const RESAMPLE_SEED: u64 = 87;

pub fn summarise(report: &str, scored: &Scored, families: &Families) -> Summary {
    let unit = if scored.wrong_per_mix.is_some() {
        Unit::Mix
    } else {
        Unit::Family
    };
    let indexed: Vec<&Item> = scored.items.iter().filter(|item| !item.held_out).collect();
    let held_out: Vec<&Item> = scored.items.iter().filter(|item| item.held_out).collect();
    let group = |items: &[&Item], held: bool| Group {
        families: TIERS
            .iter()
            .map(|&tier| {
                let distinct: BTreeSet<&str> = items
                    .iter()
                    .map(|item| families.of(&item.asset, tier))
                    .collect();
                (tier, distinct.len())
            })
            .collect(),
        mixes: items
            .iter()
            .filter_map(|item| item.mix)
            .collect::<BTreeSet<_>>()
            .len(),
        rate: match (held, unit) {
            (false, _) => Some(rate(items, families, unit, |item| item.recalled)),
            (true, Unit::Family) => Some(rate(items, families, unit, |item| item.wrong)),
            (true, Unit::Mix) => None,
        },
        items: items.len(),
        wrong: (!held && unit == Unit::Family)
            .then(|| rate(items, families, unit, |item| item.wrong)),
    };
    Summary {
        report: report.to_owned(),
        resampled: unit,
        indexed: group(&indexed, false),
        held_out: group(&held_out, true),
        wrong_per_mix: scored
            .wrong_per_mix
            .as_ref()
            .map(|wrong| wrong_per_mix(wrong)),
    }
}

/// The share of `items` for which `counts` holds, by item and by
/// relation family, with intervals from resampling whole `unit`s.
pub fn rate(
    items: &[&Item],
    families: &Families,
    unit: Unit,
    counts: impl Fn(&Item) -> bool,
) -> Rate {
    let family = |item: &Item| families.of(&item.asset, Tier::Relation).to_owned();
    let unit_of = |item: &Item| match unit {
        Unit::Family => family(item),
        Unit::Mix => item.mix.map_or_else(|| family(item), |mix| mix.to_string()),
    };
    let mut by_unit: BTreeMap<String, Vec<(String, bool)>> = BTreeMap::new();
    for item in items {
        by_unit
            .entry(unit_of(item))
            .or_default()
            .push((family(item), counts(item)));
    }
    let units: Vec<&Vec<(String, bool)>> = by_unit.values().collect();
    let point =
        |drawn: &[&Vec<(String, bool)>]| weighted(drawn.iter().flat_map(|unit| unit.iter()));
    let (item_weighted, family_weighted) = point(&units);
    let mut rng = Rng::new(RESAMPLE_SEED);
    let mut item_draws = Vec::with_capacity(RESAMPLES);
    let mut family_draws = Vec::with_capacity(RESAMPLES);
    if !units.is_empty() {
        for _ in 0..RESAMPLES {
            // Families drawn twice count as two families.
            let drawn: Vec<Vec<(String, bool)>> = (0..units.len())
                .map(|draw| {
                    units[rng.below(units.len())]
                        .iter()
                        .map(|(name, counted)| (format!("{draw}:{name}"), *counted))
                        .collect()
                })
                .collect();
            let (by_item, by_family) = weighted(drawn.iter().flatten());
            item_draws.push(by_item);
            family_draws.push(by_family);
        }
    }
    Rate {
        items: items.len(),
        count: items.iter().filter(|item| counts(item)).count(),
        item_weighted,
        family_weighted,
        item_weighted_interval: interval(&mut item_draws),
        family_weighted_interval: interval(&mut family_draws),
    }
}

/// The share counted, by item and as the mean of each family's share.
fn weighted<'a>(entries: impl Iterator<Item = &'a (String, bool)>) -> (f64, f64) {
    let mut by_family: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    for (family, counted) in entries {
        let entry = by_family.entry(family).or_default();
        entry.0 += usize::from(*counted);
        entry.1 += 1;
    }
    let (counted, total) = by_family
        .values()
        .fold((0, 0), |(c, t), &(counted, total)| (c + counted, t + total));
    if total == 0 {
        return (0.0, 0.0);
    }
    let family_mean = by_family
        .values()
        .map(|&(counted, total)| counted as f64 / total as f64)
        .sum::<f64>()
        / by_family.len() as f64;
    (counted as f64 / total as f64, family_mean)
}

fn wrong_per_mix(wrong: &[usize]) -> WrongPerMix {
    let mean = |values: &[usize]| {
        if values.is_empty() {
            0.0
        } else {
            values.iter().sum::<usize>() as f64 / values.len() as f64
        }
    };
    let mut rng = Rng::new(RESAMPLE_SEED);
    let mut draws: Vec<f64> = (0..if wrong.is_empty() { 0 } else { RESAMPLES })
        .map(|_| {
            let drawn: Vec<usize> = (0..wrong.len())
                .map(|_| wrong[rng.below(wrong.len())])
                .collect();
            mean(&drawn)
        })
        .collect();
    WrongPerMix {
        mixes: wrong.len(),
        wrong: wrong.iter().sum(),
        per_mix: mean(wrong),
        interval: interval(&mut draws),
    }
}

/// The 2.5th and 97.5th percentiles (nearest rank).
fn interval(draws: &mut [f64]) -> [f64; 2] {
    if draws.is_empty() {
        return [0.0, 0.0];
    }
    draws.sort_by(f64::total_cmp);
    let at = |share: f64| {
        draws[((share * draws.len() as f64).ceil() as usize).clamp(1, draws.len()) - 1]
    };
    [at(0.025), at(0.975)]
}

pub fn print_key(key: &Families) {
    for source in &key.sources {
        println!("source: {source}");
    }
    println!("{} files named by a join", key.files.len());
    for tier in TIERS {
        let largest = key.largest(tier);
        let shown: Vec<String> = largest
            .iter()
            .take(3)
            .map(|(name, size)| format!("{size} ({name})"))
            .collect();
        println!(
            "{tier:?}: {} families of two or more files; largest {}",
            largest.len(),
            shown.join(", ")
        );
    }
}

pub fn print_summary(summary: &Summary) {
    println!("{}", summary.report);
    for (label, group) in [
        ("indexed", &summary.indexed),
        ("held out", &summary.held_out),
    ] {
        if group.items == 0 {
            continue;
        }
        let families: Vec<String> = group
            .families
            .iter()
            .map(|(tier, count)| format!("{count} {tier:?}"))
            .collect();
        let what = if label == "indexed" {
            "recall"
        } else {
            "confident wrong"
        };
        println!(
            "  {label}: {} items, families {}{}",
            group.items,
            families.join(", ").to_lowercase(),
            if group.mixes > 0 {
                format!(", {} mixes", group.mixes)
            } else {
                String::new()
            }
        );
        if let Some(rate) = &group.rate {
            print_rate(what, rate, summary.resampled);
        }
        if let Some(wrong) = &group.wrong {
            print_rate("wrong", wrong, summary.resampled);
        }
    }
    if let Some(wrong) = &summary.wrong_per_mix {
        println!(
            "  wrong confident: {} in {} mixes, {:.3} per mix (95% {:.3}-{:.3} by mix)",
            wrong.wrong, wrong.mixes, wrong.per_mix, wrong.interval[0], wrong.interval[1]
        );
    }
}

fn print_rate(what: &str, rate: &Rate, unit: Unit) {
    let unit = match unit {
        Unit::Family => "family",
        Unit::Mix => "mix",
    };
    println!(
        "    {what}: {}/{} = {:.1}% by item (95% {:.1}-{:.1} by {unit}), {:.1}% by family (95% {:.1}-{:.1})",
        rate.count,
        rate.items,
        100.0 * rate.item_weighted,
        100.0 * rate.item_weighted_interval[0],
        100.0 * rate.item_weighted_interval[1],
        100.0 * rate.family_weighted,
        100.0 * rate.family_weighted_interval[0],
        100.0 * rate.family_weighted_interval[1],
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pairs(list: &[(&str, &str)]) -> Vec<(String, String)> {
        list.iter()
            .map(|(a, b)| ((*a).to_owned(), (*b).to_owned()))
            .collect()
    }

    fn key() -> Families {
        let joins = Joins {
            same_recording: pairs(&[("b.mp3", "a.mp3"), ("c.mp3", "d.mp3")]),
            same_title: pairs(&[("d.mp3", "e.mp3")]),
            related: pairs(&[("e.mp3", "a.mp3"), ("x.mp3", "y.mp3")]),
        };
        Families::build(vec![String::from("test")], &joins)
    }

    #[test]
    fn each_tier_joins_more_and_names_a_family_by_its_smallest_member() {
        let families = key();
        assert_eq!(families.of("b.mp3", Tier::Recording), "a.mp3");
        assert_eq!(families.of("d.mp3", Tier::Recording), "c.mp3");
        assert_eq!(families.of("e.mp3", Tier::Recording), "e.mp3");
        assert_eq!(families.of("e.mp3", Tier::Title), "c.mp3");
        assert_eq!(families.of("b.mp3", Tier::Title), "a.mp3");
        for file in ["a.mp3", "b.mp3", "c.mp3", "d.mp3", "e.mp3"] {
            assert_eq!(families.of(file, Tier::Relation), "a.mp3");
        }
        assert_eq!(families.of("y.mp3", Tier::Relation), "x.mp3");
        assert_eq!(families.of("y.mp3", Tier::Title), "y.mp3");
    }

    #[test]
    fn a_file_in_no_join_is_its_own_family() {
        assert_eq!(key().of("lone.mp3", Tier::Relation), "lone.mp3");
    }

    #[test]
    fn largest_families_come_first() {
        assert_eq!(
            key().largest(Tier::Relation),
            vec![(String::from("a.mp3"), 5), (String::from("x.mp3"), 2)]
        );
    }

    #[test]
    fn clusters_add_their_members_and_their_pairs_with_enough_coverage_and_hits() {
        let clusters: Clusters = serde_json::from_str(
            r#"{"criterion": "", "duplicates": [["a", "b", "c"]], "pairs": [
                {"query": "a", "found": "r", "coverage": 0.3, "speed": 1.0, "hits": 40, "same_recording": false},
                {"query": "a", "found": "s", "coverage": 0.1, "speed": 1.0, "hits": 40, "same_recording": false},
                {"query": "a", "found": "t", "coverage": 0.4, "speed": 1.0, "hits": 6, "same_recording": false}
            ]}"#,
        )
        .unwrap();
        let mut joins = Joins::default();
        joins.add_clusters(&clusters);
        assert_eq!(joins.same_recording, pairs(&[("a", "b"), ("b", "c")]));
        assert_eq!(joins.related, pairs(&[("a", "r")]));
    }

    #[test]
    fn a_shared_panel_relates_every_file_of_a_family() {
        let dir = std::env::temp_dir().join(format!("gunfinger-panel-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("shared-material.json");
        std::fs::write(
            &path,
            r#"{"seed": 1, "families": [{"name": "f", "half": "validation", "recordings": [["a", "b"], ["c"]]}]}"#,
        )
        .unwrap();
        let mut joins = Joins::default();
        joins.add_shared_panel(&path).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
        assert_eq!(joins.related, pairs(&[("a", "b"), ("b", "c")]));
        assert!(joins.same_recording.is_empty());
    }

    fn item(asset: &str, recalled: bool, mix: Option<usize>) -> Item {
        Item {
            asset: asset.to_owned(),
            held_out: false,
            recalled,
            wrong: false,
            mix,
        }
    }

    #[test]
    fn family_weighting_gives_each_family_one_vote() {
        // Family a.mp3 holds four queries, all recalled; x.mp3 one, missed.
        let items = [
            item("a.mp3", true, None),
            item("b.mp3", true, None),
            item("c.mp3", true, None),
            item("e.mp3", true, None),
            item("x.mp3", false, None),
        ];
        let refs: Vec<&Item> = items.iter().collect();
        let rate = rate(&refs, &key(), Unit::Family, |item| item.recalled);
        assert_eq!(rate.count, 4);
        assert!((rate.item_weighted - 0.8).abs() < 1e-12);
        assert!((rate.family_weighted - 0.5).abs() < 1e-12);
        // Two families: a resample holds a twice, x twice, or one of each.
        assert!((rate.item_weighted_interval[0] - 0.0).abs() < 1e-12);
        assert!((rate.item_weighted_interval[1] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn one_family_gives_no_spread() {
        let items = [item("a.mp3", true, None), item("b.mp3", false, None)];
        let refs: Vec<&Item> = items.iter().collect();
        let rate = rate(&refs, &key(), Unit::Family, |item| item.recalled);
        assert_eq!(rate.item_weighted_interval, [0.5, 0.5]);
        assert_eq!(rate.family_weighted, 0.5);
    }

    #[test]
    fn mixes_are_resampled_whole() {
        let items = [
            item("a.mp3", true, Some(0)),
            item("x.mp3", true, Some(0)),
            item("lone.mp3", false, Some(1)),
        ];
        let refs: Vec<&Item> = items.iter().collect();
        let rate = rate(&refs, &key(), Unit::Mix, |item| item.recalled);
        // Mix 0 alone recalls 2 of 2, mix 1 alone 0 of 1.
        assert_eq!(rate.item_weighted_interval, [0.0, 1.0]);
        assert!((rate.item_weighted - 2.0 / 3.0).abs() < 1e-12);
    }

    #[test]
    fn a_sweep_report_is_read_as_queries_and_a_recall_report_leaves_unobservable_ones_out() {
        let dir = std::env::temp_dir().join(format!("gunfinger-families-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("recall.json");
        let detection = |correct: bool, confident: bool| {
            format!(
                r#"{{"asset": "z", "correct": {correct}, "confident": {confident}, "windows": 3, "hits": 300, "speed_percent": 0.0, "fitted": true}}"#
            )
        };
        let query = |asset: &str, held_out: bool, unobservable: bool, detections: &[String]| {
            format!(
                r#"{{"asset": "{asset}", "held_out": {held_out}, "unobservable": {unobservable}, "start_seconds": 0.0, "speed_percent": 0.0, "detections": [{}]}}"#,
                detections.join(",")
            )
        };
        let text = format!(
            r#"{{"queries": [{}, {}, {}, {}]}}"#,
            query("a.mp3", false, false, &[detection(true, true)]),
            query("x.mp3", false, false, &[detection(false, true)]),
            query("y.mp3", false, true, &[detection(false, true)]),
            query("lone.mp3", true, false, &[detection(false, false)]),
        );
        std::fs::write(&path, text).unwrap();
        let scored = read_report(&path).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
        let assets: Vec<(&str, bool, bool, bool)> = scored
            .items
            .iter()
            .map(|item| {
                (
                    item.asset.as_str(),
                    item.held_out,
                    item.recalled,
                    item.wrong,
                )
            })
            .collect();
        assert_eq!(
            assets,
            vec![
                ("a.mp3", false, true, false),
                ("x.mp3", false, false, true),
                ("lone.mp3", true, false, false),
            ]
        );
        let summary = summarise("recall.json", &scored, &key());
        assert_eq!(summary.indexed.families[&Tier::Relation], 2);
        assert_eq!(summary.indexed.wrong.as_ref().unwrap().count, 1);
        assert_eq!(summary.held_out.items, 1);
        assert_eq!(summary.held_out.rate.as_ref().unwrap().count, 0);
    }

    #[test]
    fn wrong_answers_per_mix_are_resampled_by_mix() {
        let wrong = wrong_per_mix(&[0, 0, 0, 4]);
        assert_eq!(wrong.wrong, 4);
        assert!((wrong.per_mix - 1.0).abs() < 1e-12);
        assert_eq!(wrong.interval[0], 0.0);
        assert!(wrong.interval[1] >= 2.0);
    }
}
