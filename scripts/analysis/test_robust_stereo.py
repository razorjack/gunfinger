"""Tests of robust_stereo.py on small fixtures (Python's unittest)."""

import unittest

import robust_stereo


def query(condition, number, ratio, confident=False, hits=0, wrong=(), held_out=False):
    return dict(
        condition=condition,
        speed=0.95,
        number=number,
        held_out=held_out,
        best_hits=hits,
        confident=confident,
        possible=confident,
        wrong_confident=sum(1 for _, level in wrong if level == "confident"),
        wrong_possible=sum(1 for _, level in wrong if level == "possible"),
        strongest_wrong_hits=100 if wrong else 0,
        stereo=dict(
            side_to_mid_db=ratio,
            wrong=[dict(asset=a, level=level, hits=100, windows=3) for a, level in wrong],
        ),
    )


def by_key(*queries):
    return {robust_stereo.key(q): q for q in queries}


class EitherTest(unittest.TestCase):
    def test_a_query_found_in_one_channel_is_found_and_wrong_answers_are_counted_once(self):
        left = by_key(query("stereo-silent", 0, -10.0, confident=True, hits=500, wrong=[("a", "possible")]))
        right = by_key(query("stereo-silent", 0, -10.0, hits=20, wrong=[("a", "possible"), ("b", "confident")]))

        union = robust_stereo.either(left, right)[("stereo-silent", 0.95, 0)]

        self.assertTrue(union["confident"])
        self.assertEqual(union["best_hits"], 500)
        self.assertEqual(union["wrong_possible"], 1)
        self.assertEqual(union["wrong_confident"], 1)

    def test_a_wrong_asset_at_both_levels_counts_once_as_confident(self):
        a = by_key(query("stereo-control", 0, -10.0, wrong=[("a", "confident")]))
        b = by_key(query("stereo-control", 0, -10.0, wrong=[("a", "possible")]))
        c = by_key(query("stereo-control", 0, -10.0))

        union = robust_stereo.either(c, b, a)[("stereo-control", 0.95, 0)]

        self.assertEqual((union["wrong_confident"], union["wrong_possible"]), (1, 0))


class SummaryTest(unittest.TestCase):
    def test_rows_split_by_width_and_count_raised_wrong_answers(self):
        downmix = by_key(
            query("stereo-control", 0, -30.0, confident=True, hits=400, wrong=[("old", "possible")]),
            query("stereo-control", 1, -5.0, confident=True, hits=800),
            query("stereo-inverted", 0, 30.0, hits=0, wrong=[("old", "possible"), ("new", "confident")]),
            query("stereo-inverted", 1, 5.0, confident=True, hits=400),
        )

        summary = robust_stereo.summarise({"downmix": downmix})
        inverted = next(r for r in summary["rows"] if r["condition"] == "stereo-inverted")

        self.assertEqual(summary["widths"], {"0": -30.0, "1": -5.0})
        self.assertEqual((inverted["narrow_confident"], inverted["narrow"]), (0, 1))
        self.assertEqual((inverted["stereo_confident"], inverted["stereo"]), (1, 1))
        self.assertEqual(inverted["median_retention"], 0.25)
        self.assertEqual(inverted["raised_confident"], 1)
        self.assertEqual(inverted["raised_possible"], 0)
        self.assertEqual(inverted["raised_assets"], ["new"])
        self.assertEqual(inverted["side_louder"], 2)


if __name__ == "__main__":
    unittest.main()
