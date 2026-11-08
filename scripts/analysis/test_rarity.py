"""Tests of rarity.py's scores (session 85)."""
import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import rarity  # noqa: E402


def detection(R, W, family="f"):
    return dict(R=R, W=W, m=W / R, k=1.0, L=1, hits=R, family=family)


class AucTest(unittest.TestCase):
    def test_ties_count_half(self):
        self.assertEqual(rarity.auc([1, 2], [2, 3]), (0.125, 4))
        self.assertEqual(rarity.auc([5], [1, 2, 3]), (1.0, 3))
        self.assertEqual(rarity.auc([], [1]), (None, 0))

    def test_bands_compare_the_mean_weight_at_equal_hits_only(self):
        # Positives have more hits but the same mean weight in each band.
        pos = [detection(50, 25.0), detection(500, 250.0)]
        neg = [detection(40, 20.0), detection(600, 300.0)]
        result = rarity.banded(pos, neg, "m")
        self.assertEqual(result["pooled"], 0.5)
        self.assertEqual(result["pairs"], 2)
        self.assertEqual(rarity.separation(
            [dict(d, family="f") for d in pos], [dict(d, family="f") for d in neg])["auc_R"], 0.5)

    def test_the_rules_counterpart_accepts_as_many_related_detections(self):
        related = [detection(300, 60.0), detection(250, 80.0), detection(100, 50.0)]
        played = [detection(260, 70.0), detection(200, 65.0)]
        result = rarity.accepted_like_the_rule(related, {"played": played})
        self.assertEqual(result["related_at_240"], 2)
        self.assertEqual(result["w_star"], 60.0)
        self.assertEqual(result["played"], dict(at_240=1, at_w_star=2, n=2))


if __name__ == "__main__":
    unittest.main()
