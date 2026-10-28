//! The listening commands: `pair-review`, `audition` and `questions`.

use std::fs;
use std::path::{Path, PathBuf};

use clap::Subcommand;

use crate::files::{read_lines, stem_or};
use crate::harness::Harness;
use crate::{audition, pair_review};

#[derive(clap::Args)]
pub struct PairReviewArgs {
    /// The listening pack, with `README.md` and the item folders
    /// [default: <work>/listening].
    #[arg(long)]
    pub pack: Option<PathBuf>,
}

#[derive(clap::Args)]
pub struct AuditionArgs {
    /// Question files [default: every `*.json` in
    /// <work>/audition/questions/, in name order].
    pub files: Vec<PathBuf>,
    /// The answer log [default: <work>/audition/answers.jsonl].
    #[arg(long)]
    pub answers: Option<PathBuf>,
    /// Print the page's address without opening a browser.
    #[arg(long)]
    pub no_open: bool,
    /// The port to listen on [default: a free one].
    #[arg(long, default_value_t = 0)]
    pub port: u16,
}

#[derive(clap::Args)]
pub struct QuestionsArgs {
    #[command(subcommand)]
    pub source: QuestionSource,
}

pub fn pair_review(harness: &Harness, args: PairReviewArgs) -> Result<(), String> {
    let PairReviewArgs { pack } = args;
    let pack = pack.unwrap_or_else(|| harness.paths.work.join("listening"));
    let items = pair_review::items(&pack)?;
    let outcome = pair_review::review(
        &items,
        &harness.paths.verdicts,
        &mut std::io::stdin().lock(),
        &mut std::io::stdout(),
        pair_review::ffplay,
    )?;
    println!(
        "\n{} verdicts written, {} items skipped, {} judged before",
        outcome.written, outcome.skipped, outcome.judged_before
    );
    if outcome.cut_left > 0 {
        println!("{} cut joins still without a verdict", outcome.cut_left);
    }
    if outcome.written > 0 {
        println!(
            "To apply them to the last clusters run:\n  {}",
            pair_review::reuse_command(harness.other.other_peaks_dir.as_deref())
        );
    }
    Ok(())
}

pub fn audition(harness: &Harness, args: AuditionArgs) -> Result<(), String> {
    let AuditionArgs {
        files,
        answers,
        no_open,
        port,
    } = args;
    audition::server::serve(&audition::server::Options {
        question_files: question_files(&harness.paths.work, files)?,
        answers: answers.unwrap_or_else(|| audition::answers::default_log(&harness.paths.work)),
        verdicts: harness.paths.verdicts.clone(),
        work: harness.paths.work.clone(),
        open_browser: !no_open,
        port,
    })
}

pub fn questions(harness: &Harness, args: QuestionsArgs) -> Result<(), String> {
    let QuestionsArgs { source } = args;
    let questions_dir = harness.paths.work.join("audition").join("questions");
    let libraries = |roots: &OtherLibraryRoot| -> Result<audition::names::Libraries, String> {
        let absolute = |path: &Path| {
            std::path::absolute(path)
                .map_err(|error| format!("cannot resolve {}: {error}", path.display()))
        };
        Ok(audition::names::Libraries {
            corpus: absolute(&harness.paths.corpus.join("library"))?,
            other: absolute(&roots.second_library_root)?,
        })
    };
    let (mut file, out) = match source {
        QuestionSource::Pack {
            pack,
            plan,
            pair_reports,
            roots,
            out,
        } => {
            let plan = plan
                .as_deref()
                .map(audition::pack::Plan::load)
                .transpose()?
                .unwrap_or_default();
            let file = audition::pack::questions(
                &pack.unwrap_or_else(|| harness.paths.work.join("listening")),
                &plan,
                &audition::pack::Reports::load(&pair_reports)?,
                &libraries(&roots)?,
            )?;
            (
                file,
                out.unwrap_or_else(|| questions_dir.join("01-pack.json")),
            )
        }
        QuestionSource::Folders { plan, roots, out } => {
            let envelopes = audition::audio::Envelopes::below(&harness.paths.work);
            let file = audition::folders::questions(
                &audition::folders::Plan::load(&plan)?,
                &libraries(&roots)?,
                |path| {
                    envelopes
                        .get(path)
                        .map(|envelope| envelope.duration_seconds)
                },
            )?;
            (
                file,
                out.unwrap_or_else(|| questions_dir.join("02-folders.json")),
            )
        }
        QuestionSource::Detections { report, roots, out } => {
            let envelopes = audition::audio::Envelopes::below(&harness.paths.work);
            let file = audition::detections::questions(
                &audition::detections::Report::load(&report)?,
                &libraries(&roots)?,
                |path| {
                    envelopes
                        .get(path)
                        .map(|envelope| envelope.duration_seconds)
                },
            )?;
            let name = stem_or(&report, "report");
            (
                file,
                out.unwrap_or_else(|| questions_dir.join(format!("03-detections-{name}.json"))),
            )
        }
        QuestionSource::Shared { report, roots, out } => {
            let reports = audition::pack::Reports::load(std::slice::from_ref(&report))?;
            let reports: Vec<_> = reports
                .reports
                .into_iter()
                .map(|(_, report)| report)
                .collect();
            let file = audition::shared::questions(&reports, &libraries(&roots)?)?;
            let name = stem_or(&report, "report");
            (
                file,
                out.unwrap_or_else(|| questions_dir.join(format!("04-shared-{name}.json"))),
            )
        }
        QuestionSource::Parts {
            reports,
            files,
            roots,
            out,
        } => {
            let loaded = reports
                .iter()
                .map(|report| audition::parts::load(report))
                .collect::<Result<Vec<_>, String>>()?;
            let files = files.map(|path| read_lines(&path)).transpose()?;
            let file = audition::parts::questions(&loaded, files.as_deref(), &libraries(&roots)?)?;
            let name = stem_or(&reports[0], "report");
            (
                file,
                out.unwrap_or_else(|| questions_dir.join(format!("06-parts-{name}.json"))),
            )
        }
    };
    // The whole command, so the next agent can make the file again.
    file.made_by = std::iter::once(String::from("gunfinger-eval"))
        .chain(std::env::args().skip(1))
        .collect::<Vec<_>>()
        .join(" ");
    audition::schema::save(&out, &file)?;
    println!(
        "wrote {} questions to {}",
        file.questions.len(),
        out.display()
    );
    Ok(())
}

/// Where `questions` takes its evidence from.
#[derive(Subcommand)]
pub enum QuestionSource {
    /// The listening pack's items, in its README's order, as pair
    /// questions; written to <work>/audition/questions/01-pack.json.
    Pack {
        /// The pack [default: <work>/listening].
        #[arg(long)]
        pack: Option<PathBuf>,
        /// Question texts and extra lanes by item number (TOML; see
        /// `docs/audition-questions.md`).
        #[arg(long)]
        plan: Option<PathBuf>,
        /// `pair` reports whose exact alignments replace the sheets'
        /// rounded ones, and which place extra lanes (repeatable).
        #[arg(long = "pair-report")]
        pair_reports: Vec<PathBuf>,
        #[command(flatten)]
        roots: OtherLibraryRoot,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Release folders from a plan (TOML), each asked "is this a mixed
    /// CD?"; written to <work>/audition/questions/02-folders.json.
    Folders {
        plan: PathBuf,
        #[command(flatten)]
        roots: OtherLibraryRoot,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// The plays of a saved `identify` report (`-f json`), each asked "is
    /// this detection right?"; written to
    /// <work>/audition/questions/03-detections-<report name>.json.
    Detections {
        report: PathBuf,
        #[command(flatten)]
        roots: OtherLibraryRoot,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// The pairs of a `pair` report, each asked "do these different
    /// recordings share material where they line up, or is it chance?";
    /// written to <work>/audition/questions/04-shared-<report name>.json.
    Shared {
        report: PathBuf,
        #[command(flatten)]
        roots: OtherLibraryRoot,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// The candidates of `parts` reports, each asked "is this file one
    /// recording throughout, or does it change record?"; written to
    /// <work>/audition/questions/06-parts-<first report name>.json.
    Parts {
        #[arg(required = true)]
        reports: Vec<PathBuf>,
        /// The candidates to ask, one file per line (`second-library/<path>`),
        /// in order [default: every candidate worth the ear].
        #[arg(long)]
        files: Option<PathBuf>,
        #[command(flatten)]
        roots: OtherLibraryRoot,
        #[arg(long)]
        out: Option<PathBuf>,
    },
}

/// Where the files named `second-library/<path>` are.
#[derive(clap::Args)]
pub struct OtherLibraryRoot {
    /// The root of the other library's files (the NAS).
    #[arg(long, default_value = "/Volumes/atlas/Music/dnb")]
    second_library_root: PathBuf,
}

/// The question files named, or every `*.json` of the default folder in
/// name order.
fn question_files(work: &Path, named: Vec<PathBuf>) -> Result<Vec<PathBuf>, String> {
    if !named.is_empty() {
        return Ok(named);
    }
    let folder = work.join("audition").join("questions");
    let mut files: Vec<PathBuf> = fs::read_dir(&folder)
        .map_err(|error| {
            format!(
                "cannot read {} ({error}); make questions first with `gunfinger-eval questions pack` or name question files",
                folder.display()
            )
        })?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|extension| extension == "json"))
        .collect();
    files.sort();
    if files.is_empty() {
        return Err(format!("{} holds no question files", folder.display()));
    }
    Ok(files)
}
