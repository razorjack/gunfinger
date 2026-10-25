//! Recall beyond the corpus recordings: a panel of sources drawn from the
//! larger library the corpus was drawn from (`--other-peaks-dir`), whose
//! excerpts are rendered and searched as the sweep's are.
//!
//! The sources are that library's records outside the clusters of the
//! corpus recordings and not copies of corpus files, tagged with an artist
//! and a title, whose length lies in the track length range the library
//! was indexed with. Their recording families (the same normalised artist
//! and title) are split by seed into a development and a validation half,
//! so that no recording is measured in both. Each half draws 60 indexed and
//! 20 held-out sources, one per family, a third from each kind of source.
//! The panel is drawn once per seed, before any result is seen, and kept in
//! `panels/`; only the development half is searched.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use gunfinger_core::confidence::{Pass, Rule};
use gunfinger_core::decode::{Excerpt, decode};
use gunfinger_core::index::Index;
use gunfinger_core::indexing::load_records;
use gunfinger_core::library::{Asset, Library};
use gunfinger_core::parallel::map_in_order;
use gunfinger_core::profile::Profile;
use gunfinger_core::search::WINDOW_SECONDS;
use gunfinger_core::speed::Rung;
use gunfinger_core::store::PeakStore;
use serde::{Deserialize, Serialize};

use crate::clusters::Clusters;
use crate::matching::Matching;
use crate::padding::{Padding, SECOND_LIBRARY_PREFIX};
use crate::render::is_complete;
use crate::rng::Rng;
use crate::robust::{COMBINED_ENCODING, render_combined};
use crate::sweep::{
    Draw, EXCERPT_SECONDS, Outcome, Query, SPEEDS_PERCENT, SpeedRow, outcome, render_all, speed_row,
};
use crate::verifier::Verifier;

const INDEXED_SOURCES: usize = 60;
const HELD_OUT_SOURCES: usize = 20;
/// The track length range the configuration indexes the NAS
/// library with (1:30 to 15:00).
const MIN_SOURCE_SECONDS: f64 = 90.0;
const MAX_SOURCE_SECONDS: f64 = 900.0;

/// Where a source comes from, by the library's top folders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
    /// Scene releases, under `__full_scene/`.
    SceneRelease,
    /// YouTube channels, under `__youtube_archivists/`.
    YoutubeChannel,
    /// Label packs, vinyl rips and the rest.
    Elsewhere,
}

const KINDS: [Kind; 3] = [Kind::SceneRelease, Kind::YoutubeChannel, Kind::Elsewhere];

impl Kind {
    fn of(path: &str) -> Kind {
        if path.starts_with("__full_scene/") {
            Kind::SceneRelease
        } else if path.starts_with("__youtube_archivists/") {
            Kind::YoutubeChannel
        } else {
            Kind::Elsewhere
        }
    }
}

/// A panel as first drawn for a seed.
#[derive(Serialize, Deserialize)]
pub struct Panel {
    pub seed: u64,
    /// The other library's current records when it was drawn, those that
    /// could be sources, and their families.
    pub drawn_from_records: usize,
    pub pool: usize,
    pub families: usize,
    pub development: Half,
    pub validation: Half,
    /// Development sources without a current record any more (ignored or
    /// pruned since the draw). They stay in the draw, so that the rendered
    /// excerpts keep their numbers, and are searched nowhere.
    #[serde(skip)]
    pub left_out: Vec<Source>,
}

#[derive(Serialize, Deserialize)]
pub struct Half {
    pub families: usize,
    /// Indexed sources first, then held-out ones.
    pub sources: Vec<Source>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    /// As the index names it: `second-library/<path>`.
    pub asset: String,
    pub kind: Kind,
    pub family: String,
    pub held_out: bool,
    pub start_seconds: f64,
}

/// A record that may be drawn.
struct Candidate {
    asset: String,
    kind: Kind,
    family: String,
    duration_seconds: f64,
}

impl Panel {
    /// The panel saved for `seed` in `panels`, drawn from the other
    /// library's records and saved there the first time.
    pub fn for_seed(
        other: &PeakStore,
        assets: &[Asset],
        corpus_recordings: &BTreeSet<String>,
        seed: u64,
        panels: &Path,
    ) -> Result<Panel, String> {
        let path = panels.join(format!("recall-seed-{seed}.json"));
        if !path.exists() {
            let candidates = candidates(other, assets, corpus_recordings);
            let panel = draw(&candidates, assets.len(), seed);
            fs::create_dir_all(panels)
                .map_err(|error| format!("cannot create {}: {error}", panels.display()))?;
            let text = serde_json::to_string_pretty(&panel).map_err(|error| error.to_string())?;
            fs::write(&path, text)
                .map_err(|error| format!("cannot write {}: {error}", path.display()))?;
        }
        let text = fs::read_to_string(&path)
            .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
        let mut panel: Panel =
            serde_json::from_str(&text).map_err(|error| format!("{}: {error}", path.display()))?;
        let present: BTreeSet<String> = assets
            .iter()
            .map(|asset| format!("{SECOND_LIBRARY_PREFIX}{}", asset.path))
            .collect();
        panel.left_out = panel.without_record(&present);
        for source in &panel.left_out {
            eprintln!(
                "left out of the recall panel: {} has no record any more; the panel is not drawn again",
                source.asset
            );
        }
        Ok(panel)
    }

    fn without_record(&self, present: &BTreeSet<String>) -> Vec<Source> {
        self.development
            .sources
            .iter()
            .filter(|source| !present.contains(&source.asset))
            .cloned()
            .collect()
    }

    /// The development sources searched: those with a current record.
    pub fn searched_sources(&self) -> impl Iterator<Item = &Source> {
        self.development
            .sources
            .iter()
            .filter(|source| !self.is_left_out(source))
    }

    fn is_left_out(&self, source: &Source) -> bool {
        self.left_out.iter().any(|gone| gone.asset == source.asset)
    }
}

/// The other library's records that may be sources: not among
/// `corpus_recordings` (their clusters and copies of corpus files), tagged
/// with an artist and a title, and in the track length range.
fn candidates(
    other: &PeakStore,
    assets: &[Asset],
    corpus_recordings: &BTreeSet<String>,
) -> Vec<Candidate> {
    let profile = Profile::CURRENT;
    assets
        .iter()
        .filter_map(|asset| {
            let name = format!("{SECOND_LIBRARY_PREFIX}{}", asset.path);
            if corpus_recordings.contains(&name) {
                return None;
            }
            let duration_seconds = other.current_header(asset, &profile)?.duration_seconds;
            if !(MIN_SOURCE_SECONDS..=MAX_SOURCE_SECONDS).contains(&duration_seconds)
                || duration_seconds < 2.0 * EXCERPT_SECONDS
            {
                return None;
            }
            let tags = other.tags(asset)?;
            Some(Candidate {
                kind: Kind::of(&asset.path),
                family: family(tags.artist.as_deref()?, tags.title.as_deref()?),
                asset: name,
                duration_seconds,
            })
        })
        .collect()
}

/// Artist and title in lower case, with every run of other characters than
/// letters and digits as one space.
fn family(artist: &str, title: &str) -> String {
    let text = format!("{artist} - {title}").to_lowercase();
    text.split(|character: char| !character.is_alphanumeric())
        .filter(|word| !word.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Splits the families in two by seed, then draws each half.
fn draw(candidates: &[Candidate], records: usize, seed: u64) -> Panel {
    let mut rng = Rng::new(seed);
    let mut families: Vec<&str> = candidates
        .iter()
        .map(|candidate| candidate.family.as_str())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    rng.shuffle(&mut families);
    let (development, validation) = families.split_at(families.len() / 2);
    let mut half = |families: &[&str]| {
        let families: BTreeSet<&str> = families.iter().copied().collect();
        let members: Vec<&Candidate> = candidates
            .iter()
            .filter(|candidate| families.contains(candidate.family.as_str()))
            .collect();
        Half {
            families: families.len(),
            sources: draw_half(&members, &mut rng),
        }
    };
    Panel {
        seed,
        drawn_from_records: records,
        pool: candidates.len(),
        families: families.len(),
        development: half(development),
        validation: half(validation),
        left_out: Vec::new(),
    }
}

/// 60 indexed and 20 held-out sources, one per family, an equal share from
/// each kind, each with an excerpt clear of the intro and outro as the
/// sweep draws them.
fn draw_half(members: &[&Candidate], rng: &mut Rng) -> Vec<Source> {
    let mut taken_families = BTreeSet::new();
    let mut sources = Vec::new();
    for (wanted, held_out) in [(INDEXED_SOURCES, false), (HELD_OUT_SOURCES, true)] {
        for (number, &kind) in KINDS.iter().enumerate() {
            let share = wanted / KINDS.len() + usize::from(number < wanted % KINDS.len());
            let mut pool: Vec<&&Candidate> = members
                .iter()
                .filter(|candidate| candidate.kind == kind)
                .collect();
            rng.shuffle(&mut pool);
            let mut taken = 0;
            for candidate in pool {
                if taken == share {
                    break;
                }
                if !taken_families.insert(candidate.family.as_str()) {
                    continue;
                }
                let duration = candidate.duration_seconds;
                let earliest = 0.15 * duration;
                let latest = 0.85 * duration - EXCERPT_SECONDS;
                sources.push(Source {
                    asset: candidate.asset.clone(),
                    kind,
                    family: candidate.family.clone(),
                    held_out,
                    start_seconds: earliest + rng.unit() * (latest - earliest).max(0.0),
                });
                taken += 1;
            }
        }
    }
    sources
}

/// What a report's queries are: the panel's excerpts as rendered (clean)
/// or under `robust`'s combined damage, whole or cut to brief plays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub enum QuerySet {
    #[default]
    Clean,
    Damaged,
    Brief,
    BriefDamaged,
}

impl QuerySet {
    pub fn damaged(self) -> bool {
        matches!(self, QuerySet::Damaged | QuerySet::BriefDamaged)
    }

    pub fn brief(self) -> bool {
        matches!(self, QuerySet::Brief | QuerySet::BriefDamaged)
    }

    /// `recall-seed-<seed>.json` for the clean excerpts in the usual index,
    /// with `-other-rips`, `-damaged` and `-brief` added as they apply.
    pub fn report_name(self, seed: u64, other_rips_only: bool) -> String {
        let mut name = format!("recall-seed-{seed}");
        if other_rips_only {
            name.push_str("-other-rips");
        }
        if self.damaged() {
            name.push_str("-damaged");
        }
        if self.brief() {
            name.push_str("-brief");
        }
        name + ".json"
    }
}

/// Brief plays: their lengths; the held-out audio before them, at least
/// `BRIEF_LEAD_IN_SECONDS` and up to a search window more, drawn, so that
/// a play starts anywhere on the window grid; the held-out audio after.
const BRIEF_SECONDS: [f64; 2] = [15.0, 20.0];
const BRIEF_LEAD_IN_SECONDS: f64 = 3.0;
const BRIEF_TAIL_SECONDS: f64 = 3.0;
/// The held-out source after a brief play is this many places after the
/// one before it, in the panel's held-out list.
const CONTEXT_STEP: usize = 7;

/// An excerpt rendered for the search: a panel source at a speed, and under
/// damage the source whose excerpt plays beneath it.
#[derive(Clone)]
pub struct Rendered {
    pub source: Source,
    pub speed_percent: f64,
    pub path: PathBuf,
    pub partner: Option<Source>,
}

/// The development half's excerpts, rendered at every speed from the other
/// library's audio under `root` into `dir`, reusing earlier renders.
pub fn render(
    panel: &Panel,
    root: &Path,
    dir: &Path,
    jobs: usize,
) -> Result<Vec<Rendered>, String> {
    let draws: Vec<Draw> = panel
        .development
        .sources
        .iter()
        .map(|source| Draw {
            asset: source.asset.clone(),
            held_out: source.held_out,
            start_seconds: source.start_seconds,
        })
        .collect();
    fs::create_dir_all(dir).map_err(|error| format!("cannot create {}: {error}", dir.display()))?;
    let selected = draws
        .iter()
        .enumerate()
        .filter(|(number, _)| !panel.is_left_out(&panel.development.sources[*number]));
    let rendered = render_all(
        |draw| audio_file(root, &draw.asset),
        selected,
        &SPEEDS_PERCENT,
        dir,
        jobs,
    )?;
    Ok(rendered
        .into_iter()
        .filter_map(|(draw, speed_percent, path)| {
            panel
                .development
                .sources
                .iter()
                .find(|source| source.asset == draw.asset)
                .map(|source| Rendered {
                    source: source.clone(),
                    speed_percent,
                    path,
                    partner: None,
                })
        })
        .collect())
}

/// The same excerpts under `robust`'s combined damage (wow, a beatmatched
/// partner at -6 dB, broadcast processing, a low-bitrate stream), rendered
/// into `dir`, reusing earlier renders. Each source's partner is the next
/// searched source of its group (`partners`).
pub fn render_damaged(
    panel: &Panel,
    root: &Path,
    dir: &Path,
    jobs: usize,
) -> Result<Vec<Rendered>, String> {
    fs::create_dir_all(dir).map_err(|error| format!("cannot create {}: {error}", dir.display()))?;
    let work = damaged_renders(panel, dir);
    let failures: Vec<String> = map_in_order(&work, jobs, |rendered| {
        let partner = rendered.partner.as_ref()?;
        if is_complete(&rendered.path) {
            return None;
        }
        let label = format!(
            "recall, damaged {}",
            rendered
                .path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
        );
        render_combined(
            &audio_file(root, &rendered.source.asset),
            rendered.source.start_seconds,
            1.0 + rendered.speed_percent / 100.0,
            (&audio_file(root, &partner.asset), partner.start_seconds),
            &label,
            &rendered.path,
        )
        .err()
    })
    .into_iter()
    .flatten()
    .collect();
    match failures.first() {
        Some(failure) => Err(failure.clone()),
        None => Ok(work),
    }
}

fn audio_file(root: &Path, asset: &str) -> PathBuf {
    root.join(asset.strip_prefix(SECOND_LIBRARY_PREFIX).unwrap_or(asset))
}

/// The damaged renders to make or reuse. A render holds its partner's
/// audio, and the partners change when the searched sources do, so each
/// file is named by its source's and its partner's numbers in the panel.
fn damaged_renders(panel: &Panel, dir: &Path) -> Vec<Rendered> {
    let partners = partners(panel);
    let numbers: BTreeMap<&str, usize> = panel
        .development
        .sources
        .iter()
        .enumerate()
        .map(|(number, source)| (source.asset.as_str(), number))
        .collect();
    let mut work = Vec::new();
    for (number, source) in panel.development.sources.iter().enumerate() {
        if panel.is_left_out(source) {
            continue;
        }
        let partner = partners.get(&source.asset);
        let partner_number = partner.map_or(number, |partner| numbers[partner.asset.as_str()]);
        for speed_percent in SPEEDS_PERCENT {
            let name = format!(
                "{number:03}-{partner_number:03}-{speed_percent:+.0}.{}",
                COMBINED_ENCODING.extension()
            );
            work.push(Rendered {
                source: source.clone(),
                speed_percent,
                path: dir.join(name),
                partner: partner.cloned(),
            });
        }
    }
    work
}

/// The source whose excerpt plays beneath each source's under damage: the
/// next searched source of its group (indexed or held out) in panel order,
/// the first after the last, so that a held-out query holds no indexed
/// audio.
fn partners(panel: &Panel) -> BTreeMap<String, Source> {
    let mut partners = BTreeMap::new();
    for held_out in [false, true] {
        let group: Vec<&Source> = panel
            .searched_sources()
            .filter(|source| source.held_out == held_out)
            .collect();
        for (number, source) in group.iter().enumerate() {
            let next = group[(number + 1) % group.len()];
            partners.insert(source.asset.clone(), next.clone());
        }
    }
    partners
}

/// What `--other-rips-only` leaves out of the index besides the held-out
/// recordings: each searched indexed source's own record and the files of
/// its cluster whose peak records are identical to it (the same audio
/// filed again), so that only other rips can answer, as `sweep
/// --other-rips` does. `assets` are the other library's current records.
pub fn own_records(
    panel: &Panel,
    clusters: &Clusters,
    other: &PeakStore,
    assets: &[Asset],
) -> BTreeSet<String> {
    let profile = Profile::CURRENT;
    let by_path: BTreeMap<&str, &Asset> = assets
        .iter()
        .map(|asset| (asset.path.as_str(), asset))
        .collect();
    let peaks = |name: &str| {
        let asset = by_path.get(name.strip_prefix(SECOND_LIBRARY_PREFIX)?)?;
        match other.load(asset, &profile) {
            Ok(record) => Some(record.peaks),
            Err(error) => {
                eprintln!("not compared for --other-rips-only: {error}");
                None
            }
        }
    };
    with_identical_copies(panel.searched_sources(), clusters, peaks)
}

/// The indexed sources among `sources` with the members of their clusters
/// whose peaks, as `peaks` reads them, equal the source's.
fn with_identical_copies<'s, P: PartialEq>(
    sources: impl Iterator<Item = &'s Source>,
    clusters: &Clusters,
    peaks: impl Fn(&str) -> Option<P>,
) -> BTreeSet<String> {
    let mut left_out = BTreeSet::new();
    for source in sources.filter(|source| !source.held_out) {
        left_out.insert(source.asset.clone());
        let Some(own) = peaks(&source.asset) else {
            continue;
        };
        for member in clusters.cluster_of(&source.asset) {
            if member != source.asset && peaks(&member).is_some_and(|found| found == own) {
                left_out.insert(member);
            }
        }
    }
    left_out
}

/// A brief play to search: the first `seconds` of `excerpt`, after the
/// last `lead_in_seconds` of the held-out excerpt `before` and before the
/// first `BRIEF_TAIL_SECONDS` of `after`, both at the same speed.
struct BriefPlay<'a> {
    excerpt: &'a Rendered,
    seconds: f64,
    lead_in_seconds: f64,
    before: &'a Rendered,
    after: &'a Rendered,
}

/// Two brief plays of each excerpt, one per length, in the order of the
/// excerpts (sources, then speeds), then lengths. Query `n`'s context is
/// held-out sources `n` and `n + CONTEXT_STEP` of the held-out list
/// (cyclic), the next one taken where either is the source itself or the
/// two coincide. The lead-ins are drawn with `seed` in the same order.
fn brief_plays(excerpts: &[Rendered], seed: u64) -> Result<Vec<BriefPlay<'_>>, String> {
    let mut held_out: Vec<&Source> = Vec::new();
    for excerpt in excerpts.iter().filter(|excerpt| excerpt.source.held_out) {
        if !held_out
            .iter()
            .any(|source| source.asset == excerpt.source.asset)
        {
            held_out.push(&excerpt.source);
        }
    }
    let at_speed = |source: &Source, speed_percent: f64| {
        excerpts.iter().find(|excerpt| {
            excerpt.source.asset == source.asset && excerpt.speed_percent == speed_percent
        })
    };
    let mut rng = Rng::new(seed);
    let mut plays = Vec::new();
    for excerpt in excerpts {
        for seconds in BRIEF_SECONDS {
            let number = plays.len();
            let own = excerpt.source.asset.as_str();
            let missing = || format!("no held-out context for {own}");
            let before = context(&held_out, number, own, None).ok_or_else(missing)?;
            let after = context(&held_out, number + CONTEXT_STEP, own, Some(&before.asset))
                .ok_or_else(missing)?;
            plays.push(BriefPlay {
                excerpt,
                seconds,
                lead_in_seconds: BRIEF_LEAD_IN_SECONDS + rng.unit() * WINDOW_SECONDS,
                before: at_speed(before, excerpt.speed_percent).ok_or_else(missing)?,
                after: at_speed(after, excerpt.speed_percent).ok_or_else(missing)?,
            });
        }
    }
    Ok(plays)
}

/// The held-out source at `position` of the list (cyclic), or the first
/// after it that is neither `own` nor `taken`.
fn context<'s>(
    held_out: &[&'s Source],
    position: usize,
    own: &str,
    taken: Option<&str>,
) -> Option<&'s Source> {
    (0..held_out.len())
        .map(|step| held_out[(position + step) % held_out.len()])
        .find(|source| source.asset != own && Some(source.asset.as_str()) != taken)
}

/// The samples of a brief play: the three parts cut together.
fn brief_samples(play: &BriefPlay, sample_rate: u32) -> Result<Vec<f32>, String> {
    let decoded = |path: &Path| {
        decode(path, sample_rate, Excerpt::default())
            .map(|audio| audio.samples)
            .map_err(|error| error.to_string())
    };
    let samples_in = |seconds: f64| (seconds * f64::from(sample_rate)).round() as usize;
    let before = decoded(&play.before.path)?;
    let own = decoded(&play.excerpt.path)?;
    let after = decoded(&play.after.path)?;
    let lead_in = samples_in(play.lead_in_seconds).min(before.len());
    let mut samples = before[before.len() - lead_in..].to_vec();
    samples.extend_from_slice(&own[..samples_in(play.seconds).min(own.len())]);
    samples.extend_from_slice(&after[..samples_in(BRIEF_TAIL_SECONDS).min(after.len())]);
    Ok(samples)
}

/// Why an indexed excerpt was not recalled: its own cluster never became a
/// candidate (no chain of two windows or more in the first pass), or the
/// strongest detection of it had too few hits for the rule, or enough hits
/// in too few windows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Miss {
    NeverACandidate,
    TooFewHits,
    TooFewWindows,
}

impl Miss {
    /// `None` when the query was recalled or held out.
    pub fn of(query: &Query, rule: Rule) -> Option<Miss> {
        if query.held_out
            || query
                .detections
                .iter()
                .any(|outcome| outcome.correct && outcome.confident)
        {
            return None;
        }
        let strongest = query
            .detections
            .iter()
            .filter(|outcome| outcome.correct)
            .max_by_key(|outcome| outcome.hits);
        Some(match strongest {
            None => Miss::NeverACandidate,
            Some(outcome) if outcome.hits < rule.min_hits => Miss::TooFewHits,
            Some(_) => Miss::TooFewWindows,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct RecallReport {
    pub seed: u64,
    /// Each indexed source's record and its identical copies were left out
    /// of the index, so only other rips could answer.
    #[serde(default)]
    pub other_rips_only: bool,
    #[serde(default)]
    pub query_set: QuerySet,
    pub indexed_assets: usize,
    pub held_out_clusters: usize,
    /// Records left out for `other_rips_only`.
    #[serde(default)]
    pub left_out_records: usize,
    pub per_speed: Vec<SpeedRow>,
    /// Recall per kind of source, over every speed.
    pub per_kind: Vec<KindRow>,
    pub queries: Vec<RecallQuery>,
}

#[derive(Serialize, Deserialize)]
pub struct KindRow {
    pub kind: Kind,
    pub indexed_queries: usize,
    pub recalled: usize,
    pub wrong_answers: usize,
    pub held_out_queries: usize,
    pub held_out_confident: usize,
    /// Indexed queries whose recording has no file in the index (other
    /// rips only): neither recalled nor missed, scored like held-out ones.
    #[serde(default)]
    pub unobservable_queries: usize,
    #[serde(default)]
    pub unobservable_confident: usize,
}

#[derive(Serialize, Deserialize)]
pub struct RecallQuery {
    pub kind: Kind,
    pub family: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub miss: Option<Miss>,
    /// An indexed source whose recording has no file in the index.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub unobservable: bool,
    /// The source played beneath it (damaged queries).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brief: Option<Brief>,
    #[serde(flatten)]
    pub query: Query,
}

/// Where a brief play sat in its query.
#[derive(Serialize, Deserialize)]
pub struct Brief {
    pub seconds: f64,
    /// Query time at which the play starts: the held-out lead-in.
    pub starts_at_seconds: f64,
    pub before: String,
    pub after: String,
}

pub struct Options<'a> {
    pub seed: u64,
    pub padding: &'a Padding,
    pub ladder: &'a [Rung],
    pub matching: &'a Matching,
    /// Left out of the index besides the held-out recordings
    /// (`own_records`); empty for the usual index.
    pub left_out: &'a BTreeSet<String>,
    /// Measure every detection the second pass counted with the peak
    /// verifier; the others have no fitted alignment to verify.
    pub verify: bool,
    pub jobs: usize,
}

/// Searches each set of rendered excerpts against the corpus and the other
/// library, with the held-out sources' clusters and `left_out` left out of
/// one index, and hands each set's report to `report` as it finishes.
/// Indexed excerpts must return their own cluster, held-out ones nothing
/// confident; a damaged excerpt's partner counts as neither.
pub fn run(
    library: &Library,
    store: &PeakStore,
    clusters: &Clusters,
    sets: &[(QuerySet, &[Rendered])],
    options: &Options,
    mut report: impl FnMut(RecallReport) -> Result<(), String>,
) -> Result<(), String> {
    let profile = Profile::CURRENT;
    let held_out: BTreeSet<String> = sets
        .iter()
        .flat_map(|(_, excerpts)| excerpts.iter())
        .filter(|excerpt| excerpt.source.held_out)
        .flat_map(|excerpt| clusters.cluster_of(&excerpt.source.asset))
        .collect();
    let excluded: BTreeSet<String> = held_out.union(options.left_out).cloned().collect();
    let (records, _) = load_records(library, store, &profile, &BTreeSet::new());
    let index = options
        .matching
        .index(options.padding.index(&records, &profile, &excluded)?);
    let verifier = options
        .verify
        .then(|| Verifier::new(&records, options.padding.second.as_ref()));
    drop(records);
    let indexed: BTreeSet<&str> = index
        .assets()
        .iter()
        .map(|asset| asset.path.as_str())
        .collect();
    let searcher = Searcher {
        index: &index,
        indexed: &indexed,
        clusters,
        verifier: verifier.as_ref(),
        options,
    };
    for &(set, excerpts) in sets {
        let searched: Vec<Result<RecallQuery, String>> = if set.brief() {
            let plays = brief_plays(excerpts, options.seed)?;
            map_in_order(&plays, options.jobs, |play| {
                let samples = brief_samples(play, profile.sample_rate)?;
                Ok(searcher.query(
                    play.excerpt,
                    &samples,
                    Some(Brief {
                        seconds: play.seconds,
                        starts_at_seconds: play.lead_in_seconds,
                        before: play.before.source.asset.clone(),
                        after: play.after.source.asset.clone(),
                    }),
                ))
            })
        } else {
            map_in_order(excerpts, options.jobs, |excerpt| {
                let audio = decode(&excerpt.path, profile.sample_rate, Excerpt::default())
                    .map_err(|error| error.to_string())?;
                Ok(searcher.query(excerpt, &audio.samples, None))
            })
        };
        let queries = searched
            .into_iter()
            .collect::<Result<Vec<RecallQuery>, String>>()?;
        report(RecallReport {
            seed: options.seed,
            other_rips_only: !options.left_out.is_empty(),
            query_set: set,
            indexed_assets: index.assets().len(),
            held_out_clusters: held_out
                .iter()
                .map(|asset| clusters.cluster_of(asset))
                .collect::<BTreeSet<_>>()
                .len(),
            left_out_records: options.left_out.len(),
            per_speed: SPEEDS_PERCENT
                .iter()
                .map(|&speed| {
                    let observable = queries.iter().filter(|query| !query.unobservable);
                    speed_row(speed, observable.map(|query| &query.query))
                })
                .collect(),
            per_kind: KINDS.iter().map(|&kind| kind_row(kind, &queries)).collect(),
            queries,
        })?;
    }
    Ok(())
}

/// One index and what scoring a query against it needs.
struct Searcher<'a> {
    index: &'a Index,
    /// The names of the index's files.
    indexed: &'a BTreeSet<&'a str>,
    clusters: &'a Clusters,
    verifier: Option<&'a Verifier<'a>>,
    options: &'a Options<'a>,
}

impl Searcher<'_> {
    fn query(&self, excerpt: &Rendered, samples: &[f32], brief: Option<Brief>) -> RecallQuery {
        let Options {
            ladder, matching, ..
        } = *self.options;
        let source = &excerpt.source;
        let own_cluster = self.clusters.cluster_of(&source.asset);
        let partner_cluster = excerpt
            .partner
            .as_ref()
            .map(|partner| self.clusters.cluster_of(&partner.asset))
            .unwrap_or_default();
        let detections = matching.search(self.index, samples, &Profile::CURRENT, ladder, 1);
        let query = Query {
            asset: source.asset.clone(),
            held_out: source.held_out,
            start_seconds: source.start_seconds,
            speed_percent: excerpt.speed_percent,
            detections: detections
                .iter()
                .map(|detection| {
                    let verified = self
                        .verifier
                        .filter(|_| detection.evidence.pass == Pass::Fitted)
                        .and_then(|verifier| {
                            verifier.verify(self.index, detection, samples, &Profile::CURRENT)
                        });
                    let mut outcome = outcome(self.index, detection, &own_cluster, verified);
                    outcome.partner = is_partner(&outcome, &partner_cluster);
                    outcome
                })
                .collect(),
            candidates: None,
        };
        let unobservable = is_unobservable(source, &own_cluster, self.indexed);
        RecallQuery {
            kind: source.kind,
            family: source.family.clone(),
            miss: if unobservable {
                None
            } else {
                Miss::of(&query, matching.rule())
            },
            unobservable,
            partner: excerpt
                .partner
                .as_ref()
                .map(|partner| partner.asset.clone()),
            brief,
            query,
        }
    }
}

/// A detection of the partner's cluster, which is neither right nor wrong.
/// The own cluster comes first: a source beneath itself stays right.
fn is_partner(outcome: &Outcome, partner_cluster: &BTreeSet<String>) -> bool {
    !outcome.correct && partner_cluster.contains(&outcome.asset)
}

/// An indexed source none of whose cluster is in the index: no answer can
/// be right, so it is scored like a held-out source.
fn is_unobservable(
    source: &Source,
    own_cluster: &BTreeSet<String>,
    indexed: &BTreeSet<&str>,
) -> bool {
    !source.held_out
        && !own_cluster
            .iter()
            .any(|member| indexed.contains(member.as_str()))
}

fn kind_row(kind: Kind, queries: &[RecallQuery]) -> KindRow {
    let of_kind: Vec<&RecallQuery> = queries.iter().filter(|query| query.kind == kind).collect();
    let indexed: Vec<&Query> = of_kind
        .iter()
        .filter(|query| !query.query.held_out && !query.unobservable)
        .map(|query| &query.query)
        .collect();
    let held_out: Vec<&Query> = of_kind
        .iter()
        .filter(|query| query.query.held_out)
        .map(|query| &query.query)
        .collect();
    let unobservable: Vec<&Query> = of_kind
        .iter()
        .filter(|query| query.unobservable)
        .map(|query| &query.query)
        .collect();
    KindRow {
        kind,
        indexed_queries: indexed.len(),
        recalled: indexed
            .iter()
            .filter(|query| {
                query
                    .detections
                    .iter()
                    .any(|outcome| outcome.correct && outcome.confident)
            })
            .count(),
        wrong_answers: indexed.iter().map(|query| confident_wrong(query)).sum(),
        held_out_queries: held_out.len(),
        held_out_confident: held_out
            .iter()
            .filter(|query| confident_wrong(query) > 0)
            .count(),
        unobservable_queries: unobservable.len(),
        unobservable_confident: unobservable
            .iter()
            .filter(|query| confident_wrong(query) > 0)
            .count(),
    }
}

fn confident_wrong(query: &Query) -> usize {
    query
        .detections
        .iter()
        .filter(|outcome| outcome.confident && !outcome.correct && !outcome.partner)
        .count()
}

pub fn print_summary(report: &RecallReport) {
    println!(
        "recall panel seed {}, development half, {:?}{}: {} indexed assets, {} clusters held out, {} records left out",
        report.seed,
        report.query_set,
        if report.other_rips_only {
            ", other rips only"
        } else {
            ""
        },
        report.indexed_assets,
        report.held_out_clusters,
        report.left_out_records
    );
    println!(
        "{:>7} {:>8} {:>8} {:>6}",
        "speed", "recalled", "recall", "wrong"
    );
    for row in &report.per_speed {
        println!(
            "{:>+6.0}% {:>4}/{:<3} {:>7.1}% {:>6}",
            row.speed_percent,
            row.recalled,
            row.indexed_excerpts,
            row.recall * 100.0,
            row.wrong_answers
        );
    }
    for row in &report.per_kind {
        println!(
            "{:?}: {}/{} recalled, {} wrong answers; held out: {} of {} with a confident answer; unobservable: {} of {}",
            row.kind,
            row.recalled,
            row.indexed_queries,
            row.wrong_answers,
            row.held_out_confident,
            row.held_out_queries,
            row.unobservable_confident,
            row.unobservable_queries
        );
    }
    let mut misses: BTreeMap<String, usize> = BTreeMap::new();
    for query in &report.queries {
        if let Some(miss) = query.miss {
            *misses.entry(format!("{miss:?}")).or_default() += 1;
        }
    }
    println!("misses: {misses:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate(asset: &str, family: &str) -> Candidate {
        Candidate {
            asset: format!("{SECOND_LIBRARY_PREFIX}{asset}"),
            kind: Kind::of(asset),
            family: family.to_owned(),
            duration_seconds: 300.0,
        }
    }

    #[test]
    fn a_family_is_its_normalised_artist_and_title() {
        assert_eq!(
            family("Ed Rush & Optical", "Dozer (VIP)"),
            "ed rush optical dozer vip"
        );
        assert_eq!(
            family("ED RUSH  & OPTICAL", "Dozer [VIP]"),
            family("Ed Rush & Optical", "Dozer (VIP)")
        );
    }

    #[test]
    fn kinds_come_from_the_top_folders() {
        assert_eq!(Kind::of("__full_scene/1999/a.mp3"), Kind::SceneRelease);
        assert_eq!(
            Kind::of("__youtube_archivists/channel/a.opus"),
            Kind::YoutubeChannel
        );
        assert_eq!(Kind::of("Label - Pack/a.mp3"), Kind::Elsewhere);
    }

    #[test]
    fn halves_share_no_family_and_draw_one_source_per_family() {
        let mut candidates = Vec::new();
        for number in 0..400 {
            let family = format!("artist {} - title", number / 2);
            for folder in ["__full_scene/s", "__youtube_archivists/c", "label"] {
                candidates.push(candidate(&format!("{folder}/{number}.mp3"), &family));
            }
        }

        let panel = draw(&candidates, candidates.len(), 2026);

        let families = |half: &Half| -> BTreeSet<String> {
            half.sources
                .iter()
                .map(|source| source.family.clone())
                .collect()
        };
        for half in [&panel.development, &panel.validation] {
            assert_eq!(half.sources.len(), 80);
            assert_eq!(families(half).len(), 80, "one source per family");
            let held_out: Vec<Kind> = half
                .sources
                .iter()
                .filter(|source| source.held_out)
                .map(|source| source.kind)
                .collect();
            assert_eq!(held_out.len(), 20);
            assert_eq!(
                held_out
                    .iter()
                    .filter(|&&kind| kind == Kind::Elsewhere)
                    .count(),
                6
            );
            assert!(half.sources[..60].iter().all(|source| !source.held_out));
        }
        assert!(families(&panel.development).is_disjoint(&families(&panel.validation)));
        assert_eq!(panel.families, 200);
    }

    #[test]
    fn a_source_without_a_record_is_left_out_and_the_others_keep_their_order() {
        let mut candidates = Vec::new();
        for number in 0..400 {
            let family = format!("artist {number} - title");
            for folder in ["__full_scene/s", "__youtube_archivists/c", "label"] {
                candidates.push(candidate(&format!("{folder}/{number}.mp3"), &family));
            }
        }
        let mut panel = draw(&candidates, candidates.len(), 2026);
        let drawn: Vec<String> = panel
            .development
            .sources
            .iter()
            .map(|source| source.asset.clone())
            .collect();
        let pruned = drawn[1].clone();
        let present: BTreeSet<String> = candidates
            .iter()
            .map(|candidate| candidate.asset.clone())
            .filter(|asset| *asset != pruned)
            .collect();

        panel.left_out = panel.without_record(&present);

        let left_out: Vec<&str> = panel
            .left_out
            .iter()
            .map(|source| source.asset.as_str())
            .collect();
        assert_eq!(left_out, [pruned.as_str()]);
        let searched: Vec<&str> = panel
            .searched_sources()
            .map(|source| source.asset.as_str())
            .collect();
        let expected: Vec<&str> = drawn
            .iter()
            .filter(|asset| **asset != pruned)
            .map(String::as_str)
            .collect();
        assert_eq!(searched, expected);
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../work/tests/recall-pruned");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        // Cached kept sources retain their original draw numbers. The
        // removed source has neither audio nor cached excerpts.
        for (number, source) in panel.development.sources.iter().enumerate() {
            if !panel.is_left_out(source) {
                for speed in SPEEDS_PERCENT {
                    fs::write(dir.join(format!("{number:03}-{speed:+.0}.mp3")), b"cached").unwrap();
                    fs::write(
                        dir.join(format!("{number:03}-{speed:+.0}.mp3.complete")),
                        "6",
                    )
                    .unwrap();
                }
            }
        }
        let rendered = render(&panel, &dir.join("missing-library"), &dir, 1).unwrap();
        assert_eq!(rendered.len(), 79 * SPEEDS_PERCENT.len());
        assert_eq!(rendered[SPEEDS_PERCENT.len()].source.asset, drawn[2]);
        assert!(
            rendered[SPEEDS_PERCENT.len()]
                .path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .starts_with("002-")
        );
        panel.left_out = panel.development.sources.clone();
        assert!(
            render(&panel, &dir.join("missing-library"), &dir.join("cold"), 1)
                .unwrap()
                .is_empty()
        );
        fs::remove_dir_all(&dir).unwrap();
        assert_eq!(
            panel.development.sources.len(),
            80,
            "the draw stays as it was"
        );
    }

    fn query(held_out: bool, outcomes: &[(bool, bool, u32, u32)]) -> Query {
        Query {
            asset: String::from("second-library/a.mp3"),
            held_out,
            start_seconds: 60.0,
            speed_percent: 0.0,
            detections: outcomes
                .iter()
                .map(|&(correct, confident, hits, windows)| Outcome {
                    asset: String::new(),
                    correct,
                    confident,
                    windows,
                    hits,
                    speed_percent: 0.0,
                    fitted: true,
                    partner: false,
                    verified: None,
                })
                .collect(),
            candidates: None,
        }
    }

    #[test]
    fn a_miss_says_why_the_own_cluster_was_not_confident() {
        let rule = gunfinger_core::confidence::FITTED_RULE;
        let miss = |held_out, outcomes: &[(bool, bool, u32, u32)]| {
            Miss::of(&query(held_out, outcomes), rule)
        };

        assert_eq!(miss(false, &[(true, true, 900, 3)]), None);
        assert_eq!(miss(true, &[]), None);
        assert_eq!(
            miss(false, &[(false, true, 900, 3)]),
            Some(Miss::NeverACandidate)
        );
        assert_eq!(
            miss(false, &[(true, false, 120, 3), (true, false, 80, 4)]),
            Some(Miss::TooFewHits)
        );
        assert_eq!(
            miss(false, &[(true, false, 400, 2)]),
            Some(Miss::TooFewWindows)
        );
    }

    fn source(asset: &str, held_out: bool) -> Source {
        Source {
            asset: format!("{SECOND_LIBRARY_PREFIX}{asset}"),
            kind: Kind::of(asset),
            family: asset.to_owned(),
            held_out,
            start_seconds: 60.0,
        }
    }

    fn panel_of(sources: Vec<Source>) -> Panel {
        Panel {
            seed: 2026,
            drawn_from_records: 0,
            pool: 0,
            families: 0,
            development: Half {
                families: sources.len(),
                sources,
            },
            validation: Half {
                families: 0,
                sources: Vec::new(),
            },
            left_out: Vec::new(),
        }
    }

    fn rendered(sources: &[Source]) -> Vec<Rendered> {
        sources
            .iter()
            .flat_map(|source| {
                SPEEDS_PERCENT.iter().map(|&speed_percent| Rendered {
                    source: source.clone(),
                    speed_percent,
                    path: PathBuf::from(format!("{}-{speed_percent}.mp3", source.family)),
                    partner: None,
                })
            })
            .collect()
    }

    #[test]
    fn report_names_say_the_index_and_the_queries() {
        assert_eq!(
            QuerySet::Clean.report_name(2026, false),
            "recall-seed-2026.json"
        );
        assert_eq!(
            QuerySet::BriefDamaged.report_name(2026, true),
            "recall-seed-2026-other-rips-damaged-brief.json"
        );
        assert_eq!(
            QuerySet::Brief.report_name(7, false),
            "recall-seed-7-brief.json"
        );
        assert!(QuerySet::Damaged.damaged() && !QuerySet::Damaged.brief());
    }

    #[test]
    fn a_partner_is_the_next_searched_source_of_the_same_group() {
        let mut panel = panel_of(vec![
            source("a.mp3", false),
            source("b.mp3", false),
            source("gone.mp3", false),
            source("c.mp3", false),
            source("x.mp3", true),
            source("y.mp3", true),
        ]);
        panel.left_out = vec![panel.development.sources[2].clone()];

        let partners = partners(&panel);

        let partner = |asset: &str| {
            partners[&format!("{SECOND_LIBRARY_PREFIX}{asset}")]
                .family
                .clone()
        };
        assert_eq!(partner("a.mp3"), "b.mp3");
        assert_eq!(
            partner("b.mp3"),
            "c.mp3",
            "a source left out is passed over"
        );
        assert_eq!(partner("c.mp3"), "a.mp3", "the first after the last");
        assert_eq!(partner("x.mp3"), "y.mp3", "held out beneath held out");
        assert_eq!(partner("y.mp3"), "x.mp3");
        assert_eq!(partners.len(), 5);
    }

    #[test]
    fn a_query_is_unobservable_when_its_cluster_left_the_index_and_partners_are_marked_apart() {
        let indexed_source = source("own.mp3", false);
        let held_out_source = source("held.mp3", true);
        let cluster = |names: &[&str]| -> BTreeSet<String> {
            names
                .iter()
                .map(|name| format!("{SECOND_LIBRARY_PREFIX}{name}"))
                .collect()
        };
        let own = cluster(&["own.mp3", "other-rip.mp3"]);
        let other_rip = format!("{SECOND_LIBRARY_PREFIX}other-rip.mp3");
        let with_other_rip = BTreeSet::from([other_rip.as_str()]);
        let without = BTreeSet::new();

        assert!(!is_unobservable(&indexed_source, &own, &with_other_rip));
        assert!(is_unobservable(&indexed_source, &own, &without));
        assert!(
            !is_unobservable(&held_out_source, &cluster(&["held.mp3"]), &without),
            "held-out queries are scored as held out, not as unobservable"
        );

        let partner_cluster = cluster(&["beneath.mp3", "own.mp3"]);
        let detection = |name: &str, correct: bool| Outcome {
            asset: format!("{SECOND_LIBRARY_PREFIX}{name}"),
            correct,
            confident: true,
            windows: 3,
            hits: 300,
            speed_percent: 0.0,
            fitted: true,
            partner: false,
            verified: None,
        };
        assert!(is_partner(
            &detection("beneath.mp3", false),
            &partner_cluster
        ));
        assert!(!is_partner(&detection("own.mp3", true), &partner_cluster));
        assert!(!is_partner(
            &detection("elsewhere.mp3", false),
            &partner_cluster
        ));
    }

    #[test]
    fn a_damaged_render_is_named_by_its_partner_so_a_new_partner_renders_again() {
        let mut panel = panel_of(vec![
            source("a.mp3", false),
            source("b.mp3", false),
            source("c.mp3", false),
        ]);
        let dir = Path::new("damaged");
        let name = |renders: &[Rendered]| {
            renders[0]
                .path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .into_owned()
        };

        let before = damaged_renders(&panel, dir);
        panel.left_out = vec![panel.development.sources[1].clone()];
        let after = damaged_renders(&panel, dir);

        assert_eq!(before.len(), 3 * SPEEDS_PERCENT.len());
        assert_eq!(name(&before), "000-001--8.m4a");
        assert_eq!(after.len(), 2 * SPEEDS_PERCENT.len());
        assert_eq!(after[0].partner.as_ref().unwrap().family, "c.mp3");
        assert_eq!(
            name(&after),
            "000-002--8.m4a",
            "not the render with b beneath it"
        );
    }

    #[test]
    fn brief_plays_sit_between_two_other_held_out_excerpts_at_the_same_speed() {
        let mut sources = vec![source("own.mp3", false)];
        sources.extend((0..20).map(|number| source(&format!("held-{number:02}.mp3"), true)));
        let excerpts = rendered(&sources);

        let plays = brief_plays(&excerpts, 2026).unwrap();

        assert_eq!(plays.len(), excerpts.len() * BRIEF_SECONDS.len());
        for (number, play) in plays.iter().enumerate() {
            assert_eq!(play.seconds, BRIEF_SECONDS[number % 2]);
            assert!((3.0..13.0).contains(&play.lead_in_seconds));
            for context in [play.before, play.after] {
                assert!(context.source.held_out);
                assert_ne!(context.source.asset, play.excerpt.source.asset);
                assert_eq!(context.speed_percent, play.excerpt.speed_percent);
            }
            assert_ne!(play.before.source.asset, play.after.source.asset);
        }
        // Query n's context: held-out sources n and n + 7, cyclic.
        assert_eq!(plays[0].before.source.family, "held-00.mp3");
        assert_eq!(plays[0].after.source.family, "held-07.mp3");
        assert_eq!(plays[15].before.source.family, "held-15.mp3");
        assert_eq!(plays[15].after.source.family, "held-02.mp3");
        let again = brief_plays(&excerpts, 2026).unwrap();
        assert!(
            plays
                .iter()
                .zip(&again)
                .all(|(a, b)| a.lead_in_seconds == b.lead_in_seconds)
        );
        let spread: Vec<f64> = plays.iter().map(|play| play.lead_in_seconds).collect();
        assert!(
            spread.iter().any(|&lead_in| lead_in < 5.0)
                && spread.iter().any(|&lead_in| lead_in > 11.0)
        );
    }

    #[test]
    fn a_held_out_brief_play_takes_its_context_from_the_other_held_out_sources() {
        let sources: Vec<Source> = (0..20)
            .map(|number| source(&format!("held-{number:02}.mp3"), true))
            .collect();
        let excerpts = rendered(&sources);

        let plays = brief_plays(&excerpts, 2026).unwrap();

        // Query 0 is held-00's own: its context moves on to the next source.
        assert_eq!(plays[0].excerpt.source.family, "held-00.mp3");
        assert_eq!(plays[0].before.source.family, "held-01.mp3");
        assert_eq!(plays[0].after.source.family, "held-07.mp3");
    }

    #[test]
    fn a_brief_play_is_the_lead_in_the_play_and_the_tail_cut_together() {
        use crate::render::{Encoding, encode};
        let dir =
            std::env::temp_dir().join(format!("gunfinger-recall-brief-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let rate = crate::render::RENDER_RATE;
        let file = |name: &str, loud: bool| {
            let samples: Vec<f32> = (0..30 * rate)
                .map(|n| {
                    if loud {
                        0.5 * (n as f32 * 0.07).sin()
                    } else {
                        0.0
                    }
                })
                .collect();
            let path = dir.join(name);
            encode(&samples, None, Encoding::Lossless, &path).unwrap();
            Rendered {
                source: source(name, false),
                speed_percent: 0.0,
                path,
                partner: None,
            }
        };
        let (before, own, after) = (
            file("before.wav", false),
            file("own.wav", true),
            file("after.wav", false),
        );
        let play = BriefPlay {
            excerpt: &own,
            seconds: 15.0,
            lead_in_seconds: 4.5,
            before: &before,
            after: &after,
        };

        let samples = brief_samples(&play, 8000).unwrap();

        assert_eq!(samples.len(), ((4.5 + 15.0 + 3.0) * 8000.0) as usize);
        let loudness = |from: f64, to: f64| {
            let part = &samples[(from * 8000.0) as usize..(to * 8000.0) as usize];
            part.iter().map(|x| x.abs()).sum::<f32>() / part.len() as f32
        };
        assert!(
            loudness(0.0, 4.4) < 1e-3,
            "the lead-in is the silent file's end"
        );
        assert!(loudness(4.6, 19.4) > 0.1, "then the play");
        assert!(loudness(19.6, 22.5) < 1e-3, "then the silent tail");
        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn other_rips_only_leaves_out_each_indexed_source_and_its_identical_copies() {
        let own = source("own.mp3", false);
        let lonely = source("lonely.mp3", false);
        let held = source("held.mp3", true);
        let name = |path: &str| format!("{SECOND_LIBRARY_PREFIX}{path}");
        let clusters = Clusters {
            duplicates: vec![vec![
                name("own.mp3"),
                name("copy/own.mp3"),
                name("vinyl/own.mp3"),
            ]],
            criterion: String::new(),
            pairs: Vec::new(),
            cut_links: Vec::new(),
        };
        let peaks = |asset: &str| match asset.strip_prefix(SECOND_LIBRARY_PREFIX)? {
            "own.mp3" | "copy/own.mp3" => Some(vec![1, 2, 3]),
            "vinyl/own.mp3" => Some(vec![1, 2, 4]),
            _ => Some(vec![9]),
        };

        let left_out = with_identical_copies([&own, &lonely, &held].into_iter(), &clusters, peaks);

        let expected: BTreeSet<String> =
            [name("own.mp3"), name("copy/own.mp3"), name("lonely.mp3")].into();
        assert_eq!(
            left_out, expected,
            "the other rip stays; held-out sources go with their clusters elsewhere"
        );
    }

    fn recall_query(
        held_out: bool,
        unobservable: bool,
        outcomes: &[(bool, bool, bool)],
    ) -> RecallQuery {
        RecallQuery {
            kind: Kind::Elsewhere,
            family: String::from("a"),
            miss: None,
            unobservable,
            partner: None,
            brief: None,
            query: Query {
                asset: String::from("second-library/a.mp3"),
                held_out,
                start_seconds: 60.0,
                speed_percent: 0.0,
                detections: outcomes
                    .iter()
                    .map(|&(correct, partner, confident)| Outcome {
                        asset: String::new(),
                        correct,
                        confident,
                        windows: 3,
                        hits: 300,
                        speed_percent: 0.0,
                        fitted: true,
                        partner,
                        verified: None,
                    })
                    .collect(),
                candidates: None,
            },
        }
    }

    #[test]
    fn unobservable_queries_and_partners_are_scored_apart() {
        let queries = vec![
            recall_query(false, false, &[(true, false, true), (false, true, true)]),
            recall_query(false, false, &[(false, false, true)]),
            recall_query(false, true, &[(false, false, true)]),
            recall_query(false, true, &[]),
            recall_query(true, false, &[(false, true, true)]),
        ];

        let row = kind_row(Kind::Elsewhere, &queries);

        assert_eq!(row.indexed_queries, 2);
        assert_eq!(row.recalled, 1);
        assert_eq!(row.wrong_answers, 1, "the partner's detection is not wrong");
        assert_eq!(
            (row.unobservable_queries, row.unobservable_confident),
            (2, 1)
        );
        assert_eq!((row.held_out_queries, row.held_out_confident), (1, 0));
    }
}
