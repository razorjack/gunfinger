"""Tests of drift_offsets.py's measure and choice on made-up `pair` reports.

    python3 -m unittest discover -s scripts/analysis -p 'test_*.py'
"""

import unittest

import drift_offsets


def alignment(query, speed, offset, seconds):
    return {
        "query_start_seconds": query[0], "query_end_seconds": query[1],
        "found_start_seconds": offset + speed * query[0],
        "found_end_seconds": offset + speed * query[1],
        "speed": speed, "key_locked": False, "hits": 10 * len(seconds), "windows": 1,
        "coverage": 0.0, "offset_seconds": offset,
        "supported": len(seconds) / 100.0, "supported_seconds": list(seconds),
    }


def report(fitted, fine=None):
    searches = [{"name": "peaks, at the fitted speed", "alignments": fitted}]
    for percent, alignments in (fine or {}).items():
        searches.append({"name": f"peaks, fine rung {percent:+.1f}%",
                         "alignments": alignments})
    return {"query": "a.mp3", "found": "second-library/b.opus",
            "query_seconds": 100.0, "found_seconds": 110.0, "searches": searches}


# Phoenix's kind: the fitted speed holds the first 40 s; the rest drifts
# 0.4% faster and lines up on a fine rung, 0.24 s from the main line at
# its end; a repeat 60 s away lines up on another rung.
DRIFTING = report(
    [alignment((0, 40), 1.0, 0.0, range(0, 40))],
    {-0.4: [alignment((50, 70), 0.97, 60.0, range(50, 70))],
     +0.4: [alignment((40, 100), 1.004, -0.16, range(40, 100))],
     +0.8: [alignment((10, 30), 1.008, -0.1, range(10, 30))]})


class Measure(unittest.TestCase):
    def test_the_fitted_speed_alone_keeps_only_its_own_stretch(self):
        self.assertAlmostEqual(drift_offsets.drift_of(DRIFTING, 0, 10.0), 0.40)

    def test_a_fine_rung_within_the_width_adds_the_drifting_stretch(self):
        self.assertAlmostEqual(drift_offsets.drift_of(DRIFTING, 0.4, 10.0), 1.00)
        self.assertAlmostEqual(drift_offsets.drift_of(DRIFTING, 0.2, 10.0), 0.40)

    def test_a_line_that_leaves_the_main_line_by_more_than_t_is_not_near(self):
        # +0.4% rung: 0.24 s from the main line at 100 s.
        self.assertAlmostEqual(drift_offsets.drift_of(DRIFTING, 0.4, 0.2), 0.40)
        self.assertAlmostEqual(drift_offsets.drift_of(DRIFTING, 0.4, 0.3), 1.00)

    def test_seconds_are_counted_once(self):
        self.assertAlmostEqual(drift_offsets.drift_of(DRIFTING, 1.0, 10.0), 1.00)

    def test_no_fitted_alignment_is_zero(self):
        self.assertEqual(drift_offsets.drift_of(report([]), 1.0, 10.0), 0.0)

    def test_a_report_without_fine_rungs_is_refused(self):
        with self.assertRaises(ValueError):
            drift_offsets.drift_of(report([alignment((0, 40), 1.0, 0.0, range(40))]),
                                   0.4, 10.0)

    def test_measures_hold_every_width_and_near(self):
        row = drift_offsets.measures_with_drift(DRIFTING)
        self.assertAlmostEqual(row["drift_0_10"], 0.40)
        self.assertAlmostEqual(row["drift_0.4_10"], 1.00)
        self.assertIn("drift_1_20", row)
        self.assertIn("near_10", row)


def row(label, values):
    return {"query": f"{label}-{values['drift_0.2_10']}.mp3", "found": "b.mp3",
            "label": label, **values}


class Choice(unittest.TestCase):
    def test_fewest_leave_one_out_errors_then_t_10_then_the_smaller_width(self):
        def values(value):
            return {drift_offsets.name(w, t): value for t in drift_offsets.NEARS
                    for w in drift_offsets.WIDTHS}

        # Every (W, T) separates the judged pairs but 0.2/2, which places
        # three judged same pairs below every different one.
        same = [row("judged same", {**values(0.9), "drift_0.2_2": 0.1 + i / 100})
                for i in range(1, 4)]
        same += [row("judged same", values(0.8 + i / 100)) for i in range(5)]
        different = [row("judged different", values(0.2 + i / 100)) for i in range(5)]
        groups = {"a same": same, "a different": different, "b": [], "c": [], "d": []}

        found = drift_offsets.candidates(groups)

        self.assertEqual(found[0]["measure"], "drift_0.2_10")
        self.assertEqual(found[-1]["measure"], "drift_0.2_2")
        self.assertEqual(found[0]["loo_errors"], 0)


if __name__ == "__main__":
    unittest.main()
