"""Tests of pair_offsets.py's measures and scores on made-up `pair` reports.

    python3 -m unittest discover -s scripts/analysis -p 'test_*.py'
"""

import unittest

import pair_offsets


def alignment(query, found, offset, hits, seconds):
    return {
        "query_start_seconds": query[0], "query_end_seconds": query[1],
        "found_start_seconds": found[0], "found_end_seconds": found[1],
        "speed": 1.0, "key_locked": False, "hits": hits, "windows": 1,
        "coverage": 0.0, "offset_seconds": offset,
        "supported": len(seconds) / 100.0, "supported_seconds": list(seconds),
    }


def report(fitted, ladder=None, query_seconds=100.0, found_seconds=110.0):
    return {
        "query": "a.mp3", "found": "second-library/b.opus",
        "query_seconds": query_seconds, "found_seconds": found_seconds,
        "searches": [
            {"name": "peaks, turntable ladder", "alignments": ladder or []},
            {"name": "peaks, at the fitted speed", "alignments": fitted},
        ],
    }


class Measures(unittest.TestCase):
    def test_a_skip_keeps_the_alignments_near_the_main_offset(self):
        # Dilusion's kind: a cut shifts the rest by a second or two.
        measured = pair_offsets.measures_of(report([
            alignment((40, 98), (45, 103), 5.0, 600, range(40, 98)),
            alignment((0, 30), (4, 34), 4.0, 300, range(0, 30)),
            alignment((31, 39), (37.5, 45.5), 6.5, 60, range(31, 39)),
        ]))

        self.assertAlmostEqual(measured["coverage"], 0.58)
        self.assertAlmostEqual(measured["supported"], 0.96)
        self.assertAlmostEqual(measured["hits_per_second"], 600 / 58)
        self.assertAlmostEqual(measured["near_2"], 0.96)
        self.assertAlmostEqual(measured["near_10"], 0.96)
        self.assertEqual(measured["longest_gap"], 2.0)
        self.assertEqual(measured["far_jumps"], 0)

    def test_sections_found_elsewhere_leave_a_gap_near_the_main_offset(self):
        # China Cup's kind: the other alignments lie 60 and 250 s away.
        measured = pair_offsets.measures_of(report([
            alignment((0, 50), (0, 50), 0.0, 900, range(0, 50)),
            alignment((80, 100), (80, 100), 0.1, 90, range(80, 100)),
            alignment((52, 60), (-8, 0), -60.0, 80, range(52, 60)),
            alignment((62, 75), (-188, -175), -250.0, 80, range(62, 75)),
        ]))

        self.assertAlmostEqual(measured["supported"], 0.91)
        self.assertAlmostEqual(measured["near_10"], 0.70)
        self.assertAlmostEqual(measured["near_20"], 0.70)
        self.assertEqual(measured["longest_gap"], 30.0)
        self.assertEqual(measured["far_jumps"], 3)

    def test_coverage_is_the_widest_span_of_the_ladder_or_the_fitted_speed(self):
        measured = pair_offsets.measures_of(report(
            [alignment((0, 50), (0, 50), 0.0, 900, range(0, 50))],
            ladder=[alignment((0, 85), (0, 85), 0.0, 1200, [])],
        ))

        self.assertAlmostEqual(measured["coverage"], 0.85)

    def test_a_longer_query_is_measured_in_the_found_files_seconds(self):
        measured = pair_offsets.measures_of(report(
            [alignment((10, 90), (0, 40), -10.0, 400, range(0, 40))],
            query_seconds=200.0, found_seconds=50.0,
        ))

        self.assertAlmostEqual(measured["coverage"], 0.8)
        self.assertEqual(measured["longest_gap"], 10.0)

    def test_without_an_alignment_at_the_fitted_speed_a_pair_looks_different(self):
        measured = pair_offsets.measures_of(report([]))

        self.assertEqual(measured["near_10"], 0.0)
        self.assertEqual(measured["longest_gap"], 100.0)
        self.assertEqual(measured["far_jumps"], pair_offsets.NO_ALIGNMENT_JUMPS)

    def test_the_longest_run_counts_the_files_ends(self):
        self.assertEqual(pair_offsets.longest_run_without({3, 4}, 10), 5)
        self.assertEqual(pair_offsets.longest_run_without(set(), 4), 4)


class Scores(unittest.TestCase):
    def test_auc_counts_ties_half(self):
        self.assertEqual(pair_offsets.auc([3, 2], [1, 2], True), 0.875)
        self.assertEqual(pair_offsets.auc([0], [5], False), 1.0)

    def test_the_best_threshold_separates_the_labels(self):
        labelled = [(1.0, False), (2.0, False), (5.0, True), (6.0, True)]

        threshold, errors = pair_offsets.best_threshold(labelled, True)

        self.assertEqual((threshold, errors), (3.5, 0))

    def test_the_best_threshold_takes_the_lower_median_of_ties(self):
        # 1.5 and 3.5 both leave one error; of two the lower is the median.
        labelled = [(1.0, False), (2.0, True), (3.0, False), (4.0, True)]

        threshold, errors = pair_offsets.best_threshold(labelled, True)

        self.assertEqual((threshold, errors), (1.5, 1))

    def test_leave_one_out_misjudges_a_pair_among_the_others(self):
        apart = [(1.0, False), (2.0, False), (3.0, False), (7.0, True), (8.0, True),
                 (9.0, True)]

        self.assertEqual(pair_offsets.leave_one_out(apart, True), [])
        self.assertEqual(pair_offsets.leave_one_out(
            [(1.0, False), (4.0, False), (3.0, True), (5.0, True)], True), [1, 2])

    def test_a_lower_is_same_measure_says_same_at_or_below_the_threshold(self):
        self.assertTrue(pair_offsets.says_same(2.0, 2.0, False))
        self.assertFalse(pair_offsets.says_same(2.5, 2.0, False))


if __name__ == "__main__":
    unittest.main()
