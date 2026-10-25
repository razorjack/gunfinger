"""Tests of ladder_edge.py on made-up clusters and reports.

    python3 -m unittest discover -s scripts/analysis -p 'test_*.py'
"""

import unittest

import ladder_edge


def pair(query, found, speed, same=True):
    return {"query": query, "found": found, "speed": speed, "same_recording": same}


# A rip, its identical copy in the other library, and an upload 4% short
# (fast) against the rip.
PAIRS = [pair("a.mp3", "second-library/a.mp3", 1.0),
         pair("second-library/up.m4a", "a.mp3", 1.0 / 0.96),
         pair("a.mp3", "b.mp3", 1.3, same=False)]
CLUSTER = {"a.mp3", "second-library/a.mp3", "second-library/up.m4a"}
CLUSTER_OF = {member: CLUSTER for member in CLUSTER}
MAP = {"copies": [{"corpus": "a.mp3", "stands_for": "a.mp3", "also": []}]}


def detection(asset, correct, confident, hits):
    return {"asset": asset, "correct": correct, "confident": confident, "hits": hits}


def query(speed, detections, asset="a.mp3", held_out=False):
    return {"asset": asset, "speed_percent": speed, "held_out": held_out,
            "detections": detections}


class RelativeSpeed(unittest.TestCase):
    def test_the_full_index_takes_the_nearest_copy(self):
        edges = ladder_edge.joins(PAIRS)

        # Slow, the source itself is nearest; fast, the fast upload is.
        slow = ladder_edge.relative_percent("a.mp3", -9.2, CLUSTER, set(), edges)
        fast = ladder_edge.relative_percent("a.mp3", 9.2, CLUSTER, set(), edges)

        self.assertAlmostEqual(slow, -9.2)
        self.assertAlmostEqual(fast, 100 * (0.96 * 1.092 - 1))

    def test_other_rips_leave_the_upload_alone(self):
        edges = ladder_edge.joins(PAIRS)
        draws = [{"asset": "a.mp3", "held_out": False}]
        left_out = ladder_edge.left_out_by_other_rips(draws, CLUSTER_OF, MAP)

        self.assertEqual(left_out, {"a.mp3", "second-library/a.mp3"})
        # Played 5% slow against an upload whose time is 0.96 of the rip's.
        relative = ladder_edge.relative_percent("a.mp3", -5.0, CLUSTER, left_out, edges)
        self.assertAlmostEqual(relative, 100 * (0.96 * 0.95 - 1))


class Scores(unittest.TestCase):
    def rows(self, queries):
        report = {"queries": queries}
        return ladder_edge.query_rows(report, CLUSTER_OF, set(), ladder_edge.joins(PAIRS))

    def test_recall_tiers_and_wrong_answers(self):
        rows = self.rows([
            query(-9.2, [detection("a.mp3", True, False, 80), detection("b.mp3", False, False, 61)]),
            query(-10.0, [detection("a.mp3", True, False, 40)]),
            query(-8.4, [detection("a.mp3", True, True, 900), detection("b.mp3", False, True, 300)]),
        ])

        s = ladder_edge.summary(rows, rows)
        self.assertEqual((s["queries"], s["confident"], s["possible"]), (3, 1, 2))
        self.assertEqual((s["wrong_confident"], s["wrong_possible"]), (1, 1))
        self.assertEqual(s["new_wrong_confident"], [])
        cells = {c["bin"]: (c["queries"], c["confident"]) for c in s["by_relative_speed_bin"]}
        self.assertEqual(cells["8.4-8.8"], (1, 1))
        self.assertEqual(cells["9.2-9.6"], (1, 0))
        self.assertEqual(cells["10.0-10.4"], (1, 0))

    def test_new_wrong_answers_and_the_targeted_estimate(self):
        default = self.rows([query(9.6, [detection("a.mp3", True, False, 30)]),
                             query(5.0, [detection("a.mp3", True, True, 900)])])
        wider = self.rows([query(9.6, [detection("a.mp3", True, True, 700),
                                       detection("b.mp3", False, True, 250)]),
                           query(5.0, [detection("a.mp3", True, True, 900),
                                       detection("b.mp3", False, True, 260)])])

        s = ladder_edge.summary(wider, default)
        self.assertEqual(s["new_wrong_confident"], [("a.mp3", 5.0, "b.mp3"), ("a.mp3", 9.6, "b.mp3")])
        # The targeted search keeps the default's answer at +5%, which was
        # confident, and takes the wider ladder's at +9.6%.
        t = ladder_edge.summary(ladder_edge.targeted(default, wider), default)
        self.assertEqual(t["confident"], 2)
        self.assertEqual(t["new_wrong_confident"], [("a.mp3", 9.6, "b.mp3")])

    def test_held_out_queries_count_only_as_wrong(self):
        rows = self.rows([query(9.2, [detection("b.mp3", False, True, 250)],
                                asset="c.mp3", held_out=True)])

        s = ladder_edge.summary(rows, rows)
        self.assertEqual((s["queries"], s["held_out_confident"], s["wrong_confident"]), (0, 1, 1))


class Uncovered(unittest.TestCase):
    def test_overlapping_confident_spans_count_once(self):
        scan = {"duration_seconds": 100.0, "detections": [
            {"start_seconds": 0.0, "end_seconds": 30.0, "confident": True},
            {"start_seconds": 20.0, "end_seconds": 50.0, "confident": True},
            {"start_seconds": 60.0, "end_seconds": 90.0, "confident": False},
            {"start_seconds": 70.0, "end_seconds": 80.0, "confident": True}]}

        self.assertAlmostEqual(ladder_edge.uncovered(scan), 0.4)


if __name__ == "__main__":
    unittest.main()
