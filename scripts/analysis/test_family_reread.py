import unittest

from family_reread import files_at, summarise


class Reread(unittest.TestCase):
    def test_files_are_collected_under_a_path_through_lists(self):
        data = {"plays": [{"truth": {"asset": "a.mp3"}}, {"truth": {"asset": ["b.mp3", "c.mp3"]}},
                          {"other": 1}]}
        self.assertEqual(files_at(data, "plays", "truth", "asset"), ["a.mp3", "b.mp3", "c.mp3"])

    def test_items_are_counted_by_relation_family_or_by_label(self):
        fams = lambda f: {"recording": f, "title": f, "relation": "a.mp3"}
        key = {"x/a.mp3": fams("x/a.mp3"), "x/a-remix.mp3": fams("x/a-remix.mp3")}
        rows = [("0001", "ADR", "plays", ["a.mp3", "a-remix.mp3", "b.mp3", "a.mp3"], "file"),
                ("0002", "ADR", "negatives", ["fire", "fire", "fire", "pyro"], "label")]
        files, labels = summarise(rows, key)
        self.assertEqual((files["count"], files["families"], files["recording_families"]), (4, 2, 3))
        self.assertEqual(files["largest_share"], 0.75)
        self.assertEqual((labels["families"], labels["recording_families"], labels["largest_share"]),
                         (2, None, 0.75))


if __name__ == "__main__":
    unittest.main()
