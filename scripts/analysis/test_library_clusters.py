import unittest

from library_clusters import summarise


def pair(query, found, same, coverage=0.99, hits=1000, span=100.0):
    return {"query": query, "found": found, "same_recording": same, "coverage": coverage,
            "hits": hits, "query_start_seconds": 0.0, "query_end_seconds": span}


class Summarise(unittest.TestCase):
    def test_old_joins_are_compared_through_the_corpus_files_copies(self):
        old = {"duplicates": [["a.mp3", "second-library/a-copy.mp3", "second-library/a2.mp3"],
                              ["b.mp3", "second-library/b2.mp3"]],
               "pairs": [pair("a.mp3", "second-library/a2.mp3", True),
                         pair("second-library/a-copy.mp3", "second-library/a2.mp3", True),
                         pair("b.mp3", "second-library/b2.mp3", True)]}
        new = {"duplicates": [["a.mp3", "b.mp3", "second-library/a2.mp3", "second-library/b2.mp3"],
                              ["second-library/x.mp3", "second-library/y.mp3"]],
               "pairs": [pair("a.mp3", "second-library/a2.mp3", True),
                         pair("second-library/x.mp3", "second-library/y.mp3", True, hits=5),
                         pair("a.mp3", "b.mp3", True),
                         pair("second-library/x.mp3", "b.mp3", False, coverage=0.5)],
               "cut_links": []}
        map_file = {"copies": [{"corpus": "a.mp3", "stands_for": "a-copy.mp3"}]}

        summary = summarise(new, old, map_file)

        self.assertEqual(summary["old"]["joins"], 2)
        self.assertEqual(summary["a_old_joins_kept"], 2)
        self.assertEqual(summary["a_old_clusters_whole"], 2)
        self.assertEqual(len(summary["b_merging"]), 1)
        self.assertEqual(summary["c_without_corpus"], {"clusters": 1, "files": 2})
        self.assertEqual(summary["d_joins"], 3)
        self.assertEqual(summary["d_sparse"], 1)
        self.assertEqual(summary["d_borderline"], 1)


if __name__ == "__main__":
    unittest.main()
