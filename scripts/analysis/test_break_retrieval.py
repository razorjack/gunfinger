import unittest

from break_retrieval import cluster_index, score_search, strongest_outside, summarise


def holder(file, hits, reach=None):
    h = {"file": file, "hits": hits}
    if reach is not None:
        h["reach"] = {"query_start_seconds": 0.0, "query_end_seconds": reach, "hits": 100}
    return h


CLUSTERS = cluster_index({"duplicates": [["q.mp3", "q-rip.mp3"], ["e.mp3", "e-rip.mp3"],
                                         ["u.mp3", "u-rip.mp3"]]})


class ScoreSearch(unittest.TestCase):
    case = {"name": "x", "group": "A", "file": "q.mp3", "expected": ["e.mp3"], "own_extra": []}

    def searched(self):
        return {"file": "q.mp3", "expected_not_indexed": [],
                "own": [holder("q.mp3", 1000), holder("q-rip.mp3", 700)],
                "others": [holder("a.mp3", 90), holder("u-rip.mp3", 80), holder("e-rip.mp3", 60),
                           holder("t.mp3", 50)]}

    def test_rips_of_the_query_and_of_an_expected_file_are_relevant(self):
        scored = score_search(self.case, self.searched(), CLUSTERS, [("t.mp3", "e.mp3")], {})

        self.assertEqual(scored["listed"], 5)
        self.assertEqual(scored["at"][5]["relevant"], 3)
        self.assertEqual(scored["at"][5]["unjudged"], 2)
        self.assertIsNone(scored["at"][5]["unrelated"] or None)
        self.assertEqual(scored["at"][5]["precision_judged"], 1.0)
        self.assertEqual(scored["at"][5]["judged_share"], 0.6)
        self.assertTrue(scored["expected_found"])
        self.assertEqual(scored["expected_hits"], 60)
        self.assertEqual(scored["expected_rank_among_others"], 3)

    def test_judgments_follow_the_holders_cluster_for_their_query_only(self):
        judgments = {"x": {"u.mp3": "chance", "t.mp3": "same"}, "other": {"a.mp3": "chance"}}

        scored = score_search(self.case, self.searched(), CLUSTERS, [], judgments)

        self.assertEqual(scored["unrelated"], 1)
        self.assertEqual(scored["at"][5]["relevant"], 3)
        self.assertEqual(scored["at"][5]["unjudged"], 1)
        self.assertEqual(scored["at"][5]["precision_judged"], 3 / 4)

    def test_the_null_skips_the_querys_cluster(self):
        case = {"name": "n", "group": "N1", "file": "q.mp3", "expected": [], "own_extra": []}
        searched = {"file": "q.mp3", "own": [holder("q-rip.mp3", 900)],
                    "others": [holder("v.mp3", 40, reach=80.0)]}

        self.assertEqual(strongest_outside(case, searched, CLUSTERS), (40, True, "v.mp3"))
        self.assertEqual(strongest_outside(case, searched, cluster_index({"duplicates": []})),
                         (900, True, "q-rip.mp3"))


class Summarise(unittest.TestCase):
    def test_modes_and_ladders_are_scored_apart(self):
        cases = [{"name": "x", "group": "A", "file": "q.mp3", "expected": ["e.mp3"], "own_extra": []},
                 {"name": "n", "group": "N1", "file": "z.mp3", "expected": [], "own_extra": []}]
        search = lambda name, file, lists, others: {
            "name": name, "file": file, "lists": lists, "search_seconds": 1.0,
            "expected_not_indexed": [], "own": [], "others": others}
        report = {"searches": [search("x", "q.mp3", "all", [holder("e.mp3", 70)]),
                               search("n", "z.mp3", "all", [holder("w.mp3", 30)]),
                               search("x", "q.mp3", "filtered", []),
                               search("n", "z.mp3", "filtered", [])]}

        summary = summarise(cases, {"turntable": report}, CLUSTERS, [], {})

        self.assertEqual(summary["turntable/all"]["positives"]["found"], 1)
        self.assertEqual(summary["turntable/all"]["null_T"], 30)
        self.assertTrue(summary["turntable/all"]["T_below_expected_median"])
        self.assertEqual(summary["turntable/filtered"]["positives"]["found"], 0)


if __name__ == "__main__":
    unittest.main()
