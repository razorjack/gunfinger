"""Tests of cluster_speeds.py on a made-up cluster.

    python3 -m unittest discover -s scripts/analysis -p 'test_*.py'
"""

import unittest

import cluster_speeds


def pair(query, found, speed, same=True):
    return {"query": query, "found": found, "speed": speed, "same_recording": same}


# A rip, a second rip at its speed, an upload 4% short (fast) joined to the
# second rip, and an upload 11% short joined only to the first upload.
CLUSTER = {
    "duplicates": [["a.mp3", "b.mp3", "c.m4a", "d.m4a"]],
    "pairs": [pair("a.mp3", "b.mp3", 1.0), pair("c.m4a", "b.mp3", 1.0 / 0.96),
              pair("d.m4a", "c.m4a", 0.96 / 0.89), pair("d.m4a", "a.mp3", 1.2, False)],
}


class Speeds(unittest.TestCase):
    def test_times_are_chained_through_the_joins(self):
        times = cluster_speeds.times_in({"a.mp3", "b.mp3", "c.m4a", "d.m4a"},
                                        CLUSTER["pairs"])
        self.assertAlmostEqual(times["b.mp3"], 1.0)
        self.assertAlmostEqual(times["c.m4a"], 0.96)
        self.assertAlmostEqual(times["d.m4a"], 0.89)

    def test_the_span_and_the_copies_beyond_the_ladder(self):
        result = cluster_speeds.summary(CLUSTER, 0.05)
        row = result["rows"][0]

        self.assertAlmostEqual(row["span"], 1.0 / 0.89 - 1.0)
        self.assertEqual(result["span_over"], {"0.02": 1, "0.04": 1, "0.06": 1, "0.08": 1})
        # Played at +5%, the rips are 1.05 / 0.89 = 1.18 from the fast upload,
        # and the 4% upload 1.05 * 0.96 / 0.89 = 1.13.
        self.assertEqual(row["alone"], [("d.m4a", 1.05)])
        self.assertIn("a.mp3", row["copies_beyond_for_some_play"])


if __name__ == "__main__":
    unittest.main()
