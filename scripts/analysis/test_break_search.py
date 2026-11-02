import unittest

from break_search import percentile, score_positive, strongest_other


def holder(file, hits):
    return {"file": file, "hits": hits}


class ScorePositive(unittest.TestCase):
    case = {"expected": ["e1.mp3", "e2.mp3"], "own_extra": ["rip.mp3"]}

    def test_rank_counts_only_unrelated_files_above_the_best_expected_file(self):
        searched = {"expected_not_indexed": [], "others": [
            holder("rip.mp3", 900), holder("x.mp3", 80), holder("e2.mp3", 60),
            holder("y.mp3", 50), holder("e1.mp3", 40)]}

        scored = score_positive(self.case, searched)

        self.assertEqual((scored["found"], scored["rank"], scored["above"], scored["hits"]),
                         (True, 2, 1, 60))
        self.assertEqual((scored["expected_listed"], scored["expected"]), (2, 2))

    def test_expected_files_the_index_does_not_hold_are_not_missed(self):
        searched = {"expected_not_indexed": ["e1.mp3", "e2.mp3"], "others": [holder("x.mp3", 80)]}

        self.assertIsNone(score_positive(self.case, searched))

    def test_a_positive_without_an_expected_file_listed_is_missed(self):
        searched = {"expected_not_indexed": ["e2.mp3"], "others": [holder("x.mp3", 80)]}

        scored = score_positive(self.case, searched)

        self.assertEqual((scored["found"], scored["above"], scored["expected"]), (False, 1, 1))


class Noise(unittest.TestCase):
    def test_the_strongest_other_file_skips_own_extra_and_expected_files(self):
        case = {"expected": ["e.mp3"], "own_extra": ["rip.mp3"]}
        searched = {"others": [holder("rip.mp3", 500), holder("e.mp3", 90), holder("x.mp3", 30)]}

        self.assertEqual(strongest_other(case, searched), 30)
        self.assertEqual(strongest_other(case, {"others": []}), 0)

    def test_percentile_is_the_nearest_rank(self):
        values = list(range(1, 41))

        self.assertEqual(percentile(values, 0.95), 38)
        self.assertEqual(percentile([5], 0.95), 5)
        self.assertIsNone(percentile([], 0.95))


if __name__ == "__main__":
    unittest.main()
