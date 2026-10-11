//! Where a lane plays on a question's timeline: the one placement rule and
//! the arithmetic producers need to follow it.
//!
//! A piece plays file time `offset + speed × t` at timeline time `t`. An
//! alignment found between two files (`B time = offset + speed × A time`)
//! is such a placement over the span of A it covers. A file found at
//! several alignments (an edit, a cut rip) becomes several pieces: each
//! moment of the timeline follows the strongest alignment covering it, and
//! a moment none covers goes on with the alignment before it, so the lane
//! keeps playing where the files stop lining up and the listener hears them
//! part. The page evaluates only `offset + speed × t` (`timeline.js`).

use std::path::Path;

use crate::audition::schema::Piece;

/// A placement of one file on the timeline, over the span it covers, in
/// timeline seconds.
#[derive(Debug, Clone, PartialEq)]
pub struct Alignment {
    pub start: f64,
    pub end: f64,
    pub offset: f64,
    pub speed: f64,
    pub label: String,
}

impl Alignment {
    /// The inverse placement: `alignment` places B on A's timeline; the
    /// result places A on B's (`A time = (B time - offset) / speed`),
    /// over the span of B it covers.
    pub fn inverted(&self) -> Alignment {
        Alignment {
            start: self.offset + self.speed * self.start,
            end: self.offset + self.speed * self.end,
            offset: -self.offset / self.speed,
            speed: 1.0 / self.speed,
            label: self.label.clone(),
        }
    }
}

impl Piece {
    /// The file time this piece plays at timeline time `t`.
    pub fn file_time(&self, t: f64) -> f64 {
        self.offset + self.speed * t
    }
}

/// The pieces that play `file` (`file_seconds` long) over the timeline
/// `[start, end)` following `alignments`, strongest first: each moment
/// follows the strongest alignment covering it; a moment none covers goes
/// on with the alignment before it, or before the first covered moment
/// with the first alignment that follows. Clipped to the file, so a lane
/// is silent where its file has no audio. No alignments, no pieces.
pub fn tile(
    file: &Path,
    file_seconds: f64,
    (start, end): (f64, f64),
    alignments: &[Alignment],
) -> Vec<Piece> {
    let mut cuts: Vec<f64> = vec![start, end];
    for alignment in alignments {
        cuts.extend([alignment.start, alignment.end].map(|at| at.clamp(start, end)));
    }
    cuts.sort_by(f64::total_cmp);
    cuts.dedup();

    // The alignment each stretch between two cuts follows.
    let mut chosen: Vec<(f64, f64, Option<usize>)> = Vec::new();
    for pair in cuts.windows(2) {
        let (from, to) = (pair[0], pair[1]);
        let middle = (from + to) / 2.0;
        let covering = alignments
            .iter()
            .position(|alignment| alignment.start <= middle && middle < alignment.end);
        let carried = chosen.last().and_then(|&(_, _, index)| index);
        chosen.push((from, to, covering.or(carried)));
    }
    let first = chosen.iter().find_map(|&(_, _, index)| index);

    let mut pieces: Vec<Piece> = Vec::new();
    for (from, to, index) in chosen {
        let Some(index) = index.or(first) else {
            continue;
        };
        let alignment = &alignments[index];
        match pieces.last_mut() {
            Some(last) if last.end == from && same_placement(last, alignment) => last.end = to,
            _ => pieces.push(Piece {
                file: file.to_path_buf(),
                start: from,
                end: to,
                offset: alignment.offset,
                speed: alignment.speed,
                label: Some(alignment.label.clone()),
            }),
        }
    }
    pieces
        .into_iter()
        .filter_map(|piece| clip_to_file(piece, file_seconds))
        .collect()
}

fn same_placement(piece: &Piece, alignment: &Alignment) -> bool {
    piece.offset == alignment.offset && piece.speed == alignment.speed
}

/// The part of `piece` whose file time lies within `[0, file_seconds]`,
/// if any lasts a millisecond or more.
pub fn clip_to_file(piece: Piece, file_seconds: f64) -> Option<Piece> {
    const SHORTEST: f64 = 0.001;
    // file time = offset + speed × t, with speed > 0.
    let first = (0.0 - piece.offset) / piece.speed;
    let last = (file_seconds - piece.offset) / piece.speed;
    let start = piece.start.max(first);
    let end = piece.end.min(last);
    (end - start >= SHORTEST).then_some(Piece {
        start,
        end,
        ..piece
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn alignment(start: f64, end: f64, offset: f64, label: &str) -> Alignment {
        Alignment {
            start,
            end,
            offset,
            speed: 1.0,
            label: label.to_owned(),
        }
    }

    /// Start, end and offset rounded to microseconds, and the label.
    fn spans(pieces: &[Piece]) -> Vec<(f64, f64, f64, &str)> {
        let micro = |seconds: f64| (seconds * 1e6).round() / 1e6;
        pieces
            .iter()
            .map(|piece| {
                (
                    micro(piece.start),
                    micro(piece.end),
                    micro(piece.offset),
                    piece.label.as_deref().unwrap_or(""),
                )
            })
            .collect()
    }

    #[test]
    fn an_edit_plays_each_alignment_and_goes_on_through_the_gaps() {
        // An edit with 22 s cut before A 0:40 and 45 s more before 5:34:
        // the third alignment covers the intro, the first the body, the
        // second the end.
        let alignments = [
            alignment(40.2, 320.0, -22.0, "alignment 1"),
            alignment(334.6, 407.5, -66.6, "alignment 2"),
            alignment(10.1, 29.7, -9.4, "alignment 3"),
        ];

        let pieces = tile(Path::new("/b.mp3"), 352.2, (0.0, 420.5), &alignments);

        assert_eq!(
            spans(&pieces),
            [
                // B has no audio before A 9.4 under alignment 3.
                (9.4, 40.2, -9.4, "alignment 3"),
                (40.2, 334.6, -22.0, "alignment 1"),
                // B ends at 352.2 s: A 418.8 under alignment 2.
                (334.6, 418.8, -66.6, "alignment 2"),
            ]
        );
    }

    #[test]
    fn the_strongest_alignment_wins_where_two_cover_a_moment() {
        let alignments = [
            alignment(0.0, 300.0, 1.0, "strong"),
            alignment(100.0, 120.0, 50.0, "a repeat"),
        ];

        let pieces = tile(Path::new("/b.mp3"), 1000.0, (0.0, 300.0), &alignments);

        assert_eq!(spans(&pieces), [(0.0, 300.0, 1.0, "strong")]);
    }

    #[test]
    fn a_lane_with_no_alignment_has_no_pieces() {
        assert!(tile(Path::new("/b.mp3"), 100.0, (0.0, 100.0), &[]).is_empty());
    }

    #[test]
    fn the_formula_and_its_inverse_meet_at_the_same_moment() {
        let a_to_b = Alignment {
            start: 10.0,
            end: 200.0,
            offset: -21.99,
            speed: 0.9984,
            label: String::from("1"),
        };
        let b_to_a = a_to_b.inverted();
        let piece = |alignment: &Alignment| Piece {
            file: "/x".into(),
            start: alignment.start,
            end: alignment.end,
            offset: alignment.offset,
            speed: alignment.speed,
            label: None,
        };

        let b_time = piece(&a_to_b).file_time(123.4);
        let a_time = piece(&b_to_a).file_time(b_time);

        assert!((a_time - 123.4).abs() < 1e-9, "{a_time}");
        assert!((b_to_a.start - (-21.99 + 0.9984 * 10.0)).abs() < 1e-12);
    }

    #[test]
    fn clipping_keeps_the_part_inside_the_file() {
        let piece = Piece {
            file: "/x".into(),
            start: 0.0,
            end: 100.0,
            offset: -10.0,
            speed: 2.0,
            label: None,
        };

        let clipped = clip_to_file(piece.clone(), 50.0).unwrap();

        assert_eq!((clipped.start, clipped.end), (5.0, 30.0));
        assert_eq!(clip_to_file(piece, 0.0005), None);
    }
}
