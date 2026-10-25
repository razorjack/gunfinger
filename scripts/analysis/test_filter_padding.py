"""Tests of filter_padding.py on made-up reports.

    python3 -m unittest discover -s scripts/analysis -p 'test_*.py'
"""

import unittest

import filter_padding


def query(asset, candidates, detections, held_out=False):
    return {"asset": asset, "start_seconds": 10.0, "speed_percent": 0.0, "held_out": held_out,
            "candidates": [{"asset": a, "correct": c, "hits": h, "windows": 3} for a, c, h in candidates],
            "detections": [{"asset": a, "correct": c, "confident": f, "hits": h}
                           for a, c, f, h in detections]}


def report(queries, hashes, assets, cut=100):
    return {"queries": queries,
            "set_aside": {"hashes": hashes,
                          "census": {"shortest_skipped": cut, "skipped_lists": len(hashes),
                                     "skipped_postings": sum(a["skipped"] for a in assets),
                                     "postings": sum(a["postings"] for a in assets),
                                     "assets": assets}}}


ASSETS = [{"path": "a.mp3", "postings": 100, "skipped": 10},
          {"path": "b.mp3", "postings": 100, "skipped": 20}]
PADDED_ASSETS = ASSETS + [{"path": "padding-copy/1/a.mp3", "postings": 100, "skipped": 40}]


class FilterPaddingTest(unittest.TestCase):
    def test_a_copy_names_its_original(self):
        self.assertEqual(filter_padding.original("padding-copy/3/second-library/x/y.mp3"),
                         "second-library/x/y.mp3")
        self.assertEqual(filter_padding.original("x/y.mp3"), "x/y.mp3")

    def test_the_panel_selection_holds_the_clusters_of_indexed_sources(self):
        panel = {"draws": [{"asset": "a.mp3", "held_out": False}, {"asset": "h.mp3", "held_out": True}]}
        duplicates = [["a.mp3", "a2.mp3"], ["h.mp3", "h2.mp3"]]
        self.assertEqual(filter_padding.panel_files(panel, duplicates), ["a.mp3", "a2.mp3"])

    def test_the_share_selection_is_seeded(self):
        paths = [f"{n}.mp3" for n in range(50)]
        first = filter_padding.share_files(paths, 2026, 0.1)
        self.assertEqual(len(first), 5)
        self.assertEqual(first, filter_padding.share_files(list(reversed(paths)), 2026, 0.1))

    def test_a_lost_candidate_and_a_lost_answer_are_counted(self):
        base = report([
            query("a.mp3", [("a.mp3", True, 100)], [("a.mp3", True, True, 500)]),
            query("b.mp3", [("b.mp3", True, 12)], [("b.mp3", True, True, 300)]),
        ], [1, 2], ASSETS)
        padded = report([
            query("a.mp3", [("a.mp3", True, 40), ("padding-copy/1/a.mp3", True, 40)],
                  [("a.mp3", True, True, 500)]),
            query("b.mp3", [("x.mp3", False, 11)], [("b.mp3", True, False, 100), ("x.mp3", False, True, 300)]),
        ], [2, 3, 4], PADDED_ASSETS)

        result = filter_padding.compare(base, padded, {"a.mp3"})

        self.assertEqual(result["moved"], {"in": 2, "out": 1, "kept": 1})
        self.assertEqual(result["u1_lost_confident"], [["b.mp3", 10.0, 0.0]])
        self.assertEqual(result["u2_lost_candidate"], [["b.mp3", 10.0, 0.0]])
        self.assertEqual(result["u3_wrong_confident"], 1)
        self.assertEqual(result["ratio_min"], 0.4)
        self.assertEqual(result["ratio_median_copied"], 0.4)
        self.assertEqual(result["set_aside_share"], {"copies": 0.4, "originals": 0.1, "rest": 0.2})
        self.assertEqual(result["confident"], 1)

    def test_copies_of_one_wrong_file_count_once(self):
        detections = [("x.mp3", False, False, 80), ("padding-copy/1/x.mp3", False, False, 80)]
        row = filter_padding.own(query("a.mp3", [("x.mp3", False, 20), ("padding-copy/1/x.mp3", False, 20)],
                                       detections))
        self.assertEqual((row["wrong_possible"], row["wrong_possible_files"]), (2, 1))
        self.assertEqual(row["wrong_candidate_files"], 1)


if __name__ == "__main__":
    unittest.main()
