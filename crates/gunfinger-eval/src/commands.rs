//! The subcommands of `gunfinger-eval`, each in the module of its kind.

mod clusters;
mod evaluation;
mod library;
mod listening;
mod mixes;
mod pairs;
mod recall;
mod saved_reports;
mod shared;
mod sweep;

use clap::Subcommand;

use crate::harness::Harness;
use crate::stereo::FrontEnd;

#[derive(Subcommand)]
pub enum Command {
    /// Check every set manifest against the library; with --other-peaks-dir,
    /// `second-library/<path>` references against the other store.
    Validate(library::ValidateArgs),
    /// Measure hash survival against residual speed error.
    Survival(library::SurvivalArgs),
    /// Estimate what pairs and triplets of peaks cost (postings, lookups)
    /// and keep (hashes at the true alignment) on the robustness excerpts.
    HashCost(library::HashCostArgs),
    /// Pair each corpus file with its copies in the store of
    /// --other-peaks-dir: files with an identical peak record.
    MapLibrary,
    /// Find duplicate clusters by matching the library against itself.
    Clusters(clusters::ClustersArgs),
    /// List recordings that share material (remixes, VIPs, samples) by
    /// matching the library against itself.
    Related,
    /// Step through the listening pack by ear: each item without a verdict
    /// in --verdicts shows its sheet in short and plays its clips with
    /// ffplay; `s` or `d` appends the `same` or `different` line the sheet
    /// shows. At the end it prints the `clusters --reuse-pairs` command that
    /// applies the new verdicts.
    PairReview(listening::PairReviewArgs),
    /// Serve the listening page (ADR 0016) on 127.0.0.1 and open it in the
    /// default browser, to answer the questions of the question
    /// files by ear, one key per answer. Answers go to the answer log and,
    /// for pairs, `same` and `different` to --verdicts.
    Audition(listening::AuditionArgs),
    /// Write a question file for `audition` from evidence: the listening
    /// pack, release folders, or a report's detections.
    Questions(listening::QuestionsArgs),
    /// Search each pair's first file against its second alone, on several
    /// ladders and at the fitted speed, and list every alignment: why two
    /// rips cover less of each other than the clustering rule needs.
    Pair(pairs::PairArgs),
    /// List the other library's files whose start or end lines up with a
    /// recording other than the rest of the file, with every part of each
    /// (`docs/research/2026-10-27-files-holding-several-recordings.md`):
    /// files that may hold more than one recording, for the ear. Needs
    /// --other-peaks-dir; the other library's ignore file is read when its
    /// root can be reached.
    Parts(pairs::PartsArgs),
    /// Search queries cut from a passage two different recordings share
    /// (alone, looped, with the played file's own material before or
    /// after it, through a weak rip, under a blend, a short break
    /// repeated), with the played recording in the index and left out,
    /// and measure every detection with the peak verifier (queries of one
    /// channel only: not with --front-end power-sum-queries). With
    /// --other-peaks-dir every index holds that library too.
    Shared(shared::SharedArgs),
    /// Rendered mixes of shared material (session 21): for each seed, one
    /// mix per played file of a plan, playing it whole among drawn tracks, searched with the
    /// played recording indexed, left out and (where another rip is
    /// indexed) with the played file alone left out; and the plan's other
    /// audio searched with everything indexed. Every detection carries
    /// the verifier's share, the file's own material on each side, and,
    /// when confident, the strongest other indexed file holding its
    /// passage. The report is `<plan file stem>.json`.
    SharedMixes(shared::SharedMixesArgs),
    /// The library's map of shared passages (`shared-mixes`'s holder
    /// lookup) for a list of files and spans: each searched against the
    /// index without the files its lookup names, listing every file found
    /// with 30 hits or more. The report is `holders-<file stem>.json`.
    Holders(shared::HoldersArgs),
    /// How identifiable shared material is (proposal 5's diagnostic): for
    /// each direction of one half of a shared-material panel, how closely the related file and the played
    /// recording's other rips reproduce the shared passage, in pair hashes,
    /// peaks and audio (searched against each file alone). The report is
    /// `identifiability-<half>.json`.
    Identifiability(shared::IdentifiabilityArgs),
    /// The diagnostic version verifier (sessions 24 and 26): for each play
    /// of a list, each segment's line measured from the reference's own
    /// hits, the reference rendered along it and compared with the mix in
    /// 2 s patches (supported, contradicted or unobservable; distinctive
    /// or held by a sibling), with three summaries per play. Decides
    /// nothing.
    Versions(shared::VersionsArgs),
    /// With --other-peaks-dir: draw the recall panel for a seed from the
    /// other library's records outside the corpus recordings' clusters
    /// (kept in `panels/recall-seed-<seed>.json`), render its development
    /// half's excerpts from that library's audio, and search them.
    Recall(recall::RecallArgs),
    /// Run the seeded speed sweep.
    Sweep(sweep::SweepArgs),
    /// Search a set's audio and score it against its manifest.
    Scan(evaluation::ScanArgs),
    /// Report the confidence margin from the sweep and development reports.
    Calibrate(evaluation::CalibrateArgs),
    /// Measure recall and wrong answers on the sweep's excerpts under
    /// transformations: EQ, noise, codecs, blends, speech, skips, pitch rides,
    /// speeds outside the ladder and key lock.
    Robust(evaluation::RobustArgs),
    /// How wide a seeded sample of library files is: each decoded whole as
    /// two channels, the side's energy against the mid's, how the channels
    /// correlate and the census's class (report
    /// `stereo-width-seed-<seed>.json`). With --other, a sample of the
    /// other library by kind of source (`stereo-width-other-seed-<seed>.json`);
    /// with --files, every MP3 under a folder.
    StereoWidth(library::StereoWidthArgs),
    /// Build a store of the library's files under `--front-end`'s profile
    /// (downmix or power-sum), some decoded from replacement files, for
    /// the front-end variant's experiments; never the library's store.
    VariantStore(library::VariantStoreArgs),
    /// Render seeded mixes of library tracks with exact truth (speeds,
    /// bass swaps, crossfades, cuts, plays of 20-60 s, a returning track,
    /// held-out tracks), search them and score them against the truth.
    /// With --other-peaks-dir the index also holds the other library, less
    /// the held-out clusters' files there.
    Mixes(mixes::MixesArgs),
    /// Slide brief plays of indexed tracks across the 10 s window grid in
    /// 1 s steps, at several lengths and source positions, and compare
    /// the frozen rule with minimum aligned spans.
    Grid(mixes::GridArgs),
    /// Score saved `mixes`, `shared-mixes` or `scan` reports second by
    /// second (Sonnleitner et al. 2016, section 5): seconds labelled
    /// correctly, possibly or wrongly, or not at all, while an indexed
    /// track plays, and silence while none does (specificity); and where
    /// each identified track's labels start and end. Searches nothing and leaves the reports and
    /// their scores as they are. The scores go to `seconds/` (with another
    /// library, `seconds/library-<store directory name>/`)
    Seconds(saved_reports::SecondsArgs),
    /// List the gaps of saved `mixes`, `shared-mixes` or `scan` reports:
    /// the stretches no segment of a confident play covers, at each minimum
    /// length, with how many tracks each holds by its length over the
    /// median interval between the found entries (method 1), against the
    /// truth: the left-out, held-out or absent tracks each gap holds
    /// (`docs/research/2026-10-21-gap-measures.md`). Searches nothing. The
    /// listings go to `gaps/` (with another library,
    /// `gaps/library-<store directory name>/`); the summary counts every
    /// gap of every report given.
    Gaps(saved_reports::GapsArgs),
    /// Counting the tracks in a gap by matching the gap against itself:
    /// each listed gap of saved `mixes` or `scan` reports (as `gaps` lists
    /// them, at one minimum length) is decoded, its pair hashes matched
    /// with its own at speed 1.0, and the stretches joined by repeats
    /// counted as blocks, at several thresholds, beside the truth, method
    /// 1's estimate and the estimate `identify` prints (`tracks`, ADR 0024,
    /// from the gaps and intervals at this minimum length). The listing
    /// goes to
    /// `gaps/self-match-<report stem>.json` (with another library, under
    /// `gaps/library-<store directory name>/`).
    SelfMatch(saved_reports::SelfMatchArgs),
    /// Attribute where a set's identified plays lose evidence: hits per
    /// window against the reference hashes heard, on the rung, at the
    /// fitted and the best local speed, beside a clean render of the same
    /// stretch. For the development set only.
    Loss(evaluation::LossArgs),
    /// Save the reports of the standard evaluation (sweep, development scan
    /// and leave-outs) as a named baseline under `work/baselines/`.
    Baseline(evaluation::BaselineArgs),
    /// Measure resident memory after loading the peak records, building the
    /// index and searching a set's audio, the way `identify` does; with
    /// `--synthetic-copies`, on the scale proxy. Run under
    /// `/usr/bin/time -l` (macOS) or `/usr/bin/time -v` (Linux) for the
    /// kernel's peak.
    Memory(evaluation::MemoryArgs),
    /// Count each indexed record's postings in the fullest posting lists,
    /// the lists `--skip-fullest` sets aside when looking for candidates.
    Fullest(library::FullestArgs),
    /// Rerun the standard evaluation and report what changed against a
    /// saved baseline.
    Regress(evaluation::RegressArgs),
}

/// The commands that take --other-peaks-dir, by `Command::name`.
const OTHER_LIBRARY_COMMANDS: [&str; 25] = [
    "validate",
    "map-library",
    "clusters --from-peaks",
    "sweep",
    "scan",
    "calibrate",
    "robust",
    "mixes",
    "stereo-width",
    "baseline",
    "regress",
    "memory",
    "fullest",
    "pair",
    "parts",
    "pair-review",
    "recall",
    "shared",
    "shared-mixes",
    "holders",
    "identifiability",
    "versions",
    "seconds",
    "gaps",
    "self-match",
];

/// The commands that analyse audio by `--front-end` (`stereo::FrontEnd`).
const FRONT_END_COMMANDS: [&str; 6] = [
    "robust",
    "scan",
    "mixes",
    "recall",
    "shared",
    "variant-store",
];

/// The commands that take --other-sample.
const OTHER_SAMPLE_COMMANDS: [&str; 6] = [
    "sweep",
    "scan",
    "calibrate",
    "baseline",
    "regress",
    "memory",
];

impl Command {
    /// The command's name on the command line, with `clusters --from-peaks`
    /// told apart from `clusters`: only the first takes another library.
    fn name(&self) -> &'static str {
        match self {
            Command::Validate(_) => "validate",
            Command::Survival(_) => "survival",
            Command::HashCost(_) => "hash-cost",
            Command::MapLibrary => "map-library",
            Command::Clusters(args) if args.from_peaks => "clusters --from-peaks",
            Command::Clusters(_) => "clusters",
            Command::Related => "related",
            Command::PairReview(_) => "pair-review",
            Command::Audition(_) => "audition",
            Command::Questions(_) => "questions",
            Command::Pair(_) => "pair",
            Command::Parts(_) => "parts",
            Command::Shared(_) => "shared",
            Command::SharedMixes(_) => "shared-mixes",
            Command::Holders(_) => "holders",
            Command::Identifiability(_) => "identifiability",
            Command::Versions(_) => "versions",
            Command::Recall(_) => "recall",
            Command::Sweep(_) => "sweep",
            Command::Scan(_) => "scan",
            Command::Calibrate(_) => "calibrate",
            Command::Robust(_) => "robust",
            Command::StereoWidth(_) => "stereo-width",
            Command::VariantStore(_) => "variant-store",
            Command::Mixes(_) => "mixes",
            Command::Grid(_) => "grid",
            Command::Seconds(_) => "seconds",
            Command::Gaps(_) => "gaps",
            Command::SelfMatch(_) => "self-match",
            Command::Loss(_) => "loss",
            Command::Baseline(_) => "baseline",
            Command::Memory(_) => "memory",
            Command::Fullest(_) => "fullest",
            Command::Regress(_) => "regress",
        }
    }
}

/// Fails when --other-peaks-dir or --other-sample, which are global, is
/// given to a command that does not take it.
fn check_global_options(harness: &Harness, command: &Command) -> Result<(), String> {
    let name = command.name();
    if harness.other.other_sample.is_some() && !OTHER_SAMPLE_COMMANDS.contains(&name) {
        return Err(format!(
            "this command does not take --other-sample; {} do",
            listed(&OTHER_SAMPLE_COMMANDS)
        ));
    }
    let front_end = harness.search.matching.front_end;
    if front_end != FrontEnd::Downmix && !FRONT_END_COMMANDS.contains(&name) {
        return Err(format!(
            "this command does not take --front-end; {} do",
            listed(&FRONT_END_COMMANDS)
        ));
    }
    if front_end == FrontEnd::PowerSum && harness.other.other_peaks_dir.is_some() {
        return Err(String::from(
            "the other library's records are the downmix's: --front-end power-sum takes no --other-peaks-dir (power-sum-queries does)",
        ));
    }
    if front_end != FrontEnd::Downmix && harness.search.verify {
        return Err(String::from(
            "the peak verifier analyses one channel: --verify takes the downmix front end only",
        ));
    }
    if harness.other.other_peaks_dir.is_some() && !OTHER_LIBRARY_COMMANDS.contains(&name) {
        return Err(format!(
            "this command does not take --other-peaks-dir; {} do",
            listed(&OTHER_LIBRARY_COMMANDS)
        ));
    }
    Ok(())
}

/// `a, b and c`.
fn listed(names: &[&str]) -> String {
    match names.split_last() {
        Some((last, [])) => (*last).to_owned(),
        Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
        None => String::new(),
    }
}

pub fn run(harness: &Harness, jobs: usize, command: Command) -> Result<(), String> {
    check_global_options(harness, &command)?;
    match command {
        Command::Validate(args) => library::validate(harness, args),
        Command::Survival(args) => library::survival(harness, args),
        Command::HashCost(args) => library::hash_cost(harness, jobs, args),
        Command::MapLibrary => library::map_library(harness, jobs),
        Command::Clusters(args) => clusters::clusters(harness, jobs, args),
        Command::Related => library::related(harness, jobs),
        Command::PairReview(args) => listening::pair_review(harness, args),
        Command::Audition(args) => listening::audition(harness, args),
        Command::Questions(args) => listening::questions(harness, args),
        Command::Pair(args) => pairs::pair(harness, jobs, args),
        Command::Parts(args) => pairs::parts(harness, jobs, args),
        Command::Shared(args) => shared::shared(harness, jobs, args),
        Command::SharedMixes(args) => shared::shared_mixes(harness, jobs, args),
        Command::Holders(args) => shared::holders(harness, jobs, args),
        Command::Identifiability(args) => shared::identifiability(harness, args),
        Command::Versions(args) => shared::versions(harness, jobs, args),
        Command::Recall(args) => recall::recall(harness, jobs, args),
        Command::Sweep(args) => sweep::sweep(harness, jobs, args),
        Command::Scan(args) => evaluation::scan(harness, jobs, args),
        Command::Calibrate(args) => evaluation::calibrate(harness, args),
        Command::Robust(args) => evaluation::robust(harness, jobs, args),
        Command::StereoWidth(args) => library::stereo_width(harness, jobs, args),
        Command::VariantStore(args) => library::variant_store(harness, jobs, args),
        Command::Mixes(args) => mixes::mixes(harness, jobs, args),
        Command::Grid(args) => mixes::grid(harness, jobs, args),
        Command::Seconds(args) => saved_reports::seconds(harness, args),
        Command::Gaps(args) => saved_reports::gaps(harness, args),
        Command::SelfMatch(args) => saved_reports::self_match(harness, jobs, args),
        Command::Loss(args) => evaluation::loss(harness, jobs, args),
        Command::Baseline(args) => evaluation::baseline(harness, args),
        Command::Memory(args) => evaluation::memory(harness, jobs, args),
        Command::Fullest(args) => library::fullest(harness, args),
        Command::Regress(args) => evaluation::regress(harness, jobs, args),
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use super::*;

    #[derive(Parser)]
    struct Cli {
        #[command(flatten)]
        harness: Harness,
        #[command(subcommand)]
        command: Command,
    }

    fn checked(arguments: &[&str]) -> Result<(), String> {
        let cli = Cli::try_parse_from([&["gunfinger-eval"], arguments].concat())
            .map_err(|error| error.to_string())?;
        check_global_options(&cli.harness, &cli.command)
    }

    #[test]
    fn the_front_end_goes_only_to_the_commands_that_analyse_by_it() {
        assert_eq!(checked(&["--front-end", "power-sum", "robust"]), Ok(()));
        assert_eq!(
            checked(&["--front-end", "power-sum-queries", "scan", "stakka"]),
            Ok(())
        );
        assert_eq!(
            checked(&["--front-end", "power-sum-queries", "recall"]),
            Ok(())
        );
        assert_eq!(
            checked(&["--front-end", "power-sum-queries", "shared", "plan.json"]),
            Ok(())
        );
        assert_eq!(checked(&["robust", "--verify"]), Ok(()));
        let refused = |arguments: &[&str], words: &str| {
            checked(arguments).is_err_and(|message| message.contains(words))
        };
        assert!(refused(
            &["--front-end", "power-sum", "sweep"],
            "does not take --front-end"
        ));
        assert!(refused(
            &["--front-end", "power-sum", "robust", "--verify"],
            "--verify"
        ));
    }

    #[test]
    fn power_sum_records_are_not_searched_beside_the_other_librarys() {
        let other = ["--other-peaks-dir", "/nonexistent"];

        assert!(
            checked(&[&other[..], &["--front-end", "power-sum", "scan", "stakka"]].concat())
                .is_err_and(|message| message.contains("takes no --other-peaks-dir"))
        );
        assert_eq!(
            checked(
                &[
                    &other[..],
                    &["--front-end", "power-sum-queries", "scan", "stakka"]
                ]
                .concat()
            ),
            Ok(())
        );
    }
}
