"""Tests of marked_features.py's features (session 48)."""
import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import marked_features as m  # noqa: E402


def segment(start, end, track_start, track_end, hits):
    return {"start_seconds": start, "end_seconds": end,
            "track_start_seconds": track_start, "track_end_seconds": track_end, "hits": hits}


def alignment(query_start, query_end, offset, speed, hits):
    return {"query_start_seconds": query_start, "query_end_seconds": query_end,
            "offset_seconds": offset, "speed": speed, "hits": hits}


class Features(unittest.TestCase):
    def setUp(self):
        # The partner plays its file from 100 s at mix time 0, at speed 1.
        self.partner = {"speed": 1.0, "segments": [segment(0.0, 300.0, 100.0, 400.0, 2000)]}

    def test_shared_material_falls_on_the_alignment(self):
        # The marked file's 50-70 s is the partner's 250-270 s.
        marked = {"speed": 1.0, "segments": [segment(150.0, 170.0, 50.0, 70.0, 90)]}
        shared = [alignment(40.0, 75.0, 200.0, 1.0, 120)]

        f = m.features(marked, self.partner, shared, None)

        self.assertEqual(f["F1"], 120)
        self.assertAlmostEqual(f["F2"], 0.0)
        self.assertAlmostEqual(f["F3"], 0.0)
        self.assertAlmostEqual(f["F5"], 0.0)

    def test_a_record_playing_elsewhere_in_its_track_is_off_the_alignment(self):
        # The marked file plays from 150 s, half of it past the shared span.
        marked = {"speed": 1.02, "segments": [segment(150.0, 170.0, 150.0, 170.0, 100)]}
        shared = [alignment(130.0, 160.0, 0.0, 1.0, 80), alignment(0.0, 20.0, 0.0, 1.0, 40)]

        f = m.features(marked, self.partner, shared, None)

        self.assertEqual(f["F1"], 80)
        # m = 160 s maps to 160 s; the partner is at 260 s.
        self.assertAlmostEqual(f["F2"], 100.0)
        self.assertAlmostEqual(f["F3"], 50.0)
        self.assertAlmostEqual(f["F3s"], 10.0)
        self.assertAlmostEqual(f["F5"], abs(1.0 / 1.02 - 1))

    def test_without_an_alignment_the_files_share_nothing(self):
        marked = {"speed": 1.0, "segments": [segment(150.0, 170.0, 10.0, 30.0, 70)]}

        f = m.features(marked, self.partner, [alignment(0.0, 30.0, 5.0, 1.0, 59)], None)

        self.assertEqual(f["F1"], 59)
        self.assertIsNone(f["F2"])
        self.assertAlmostEqual(f["F3"], 70.0)
        self.assertTrue(m.RULES["R1 F1 < 60"](f))
        self.assertTrue(m.RULES["R2 F2 > 10 s or none"](f))
        self.assertTrue(m.RULES["R3 F3 >= 60"](f))

    def test_outside_counts_overlapping_spans_once(self):
        self.assertAlmostEqual(m.outside((0.0, 10.0), [(2.0, 5.0), (4.0, 6.0), (9.0, 12.0)]), 5.0)
        self.assertAlmostEqual(m.outside((10.0, 0.0), []), 10.0)

    def test_auc_counts_ties_half_and_missing_values_as_the_playing_end(self):
        self.assertEqual(m.auc([3, None], [1, 3], 1), (1 + 0.5 + 1 + 1) / 4)
        self.assertEqual(m.auc([1], [2], -1), 1.0)


if __name__ == "__main__":
    unittest.main()
