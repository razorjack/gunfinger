import unittest

from family_counts import count


class Count(unittest.TestCase):
    def test_library_files_are_counted_by_their_key_path_and_family(self):
        key = {"a.mp3": {"recording": "a.mp3", "title": "a.mp3", "relation": "a.mp3"},
               "a-rip.mp3": {"recording": "a.mp3", "title": "a.mp3", "relation": "a.mp3"}}
        data = {"plays": [{"truth": {"asset": "a.mp3"}}, {"truth": {"asset": "a-rip.mp3"}},
                          {"truth": {"asset": "b.mp3"}}],
                "mix": "corpus/sets/x/mix.mp3", "note": "not a file"}
        rows = count(key, data)
        self.assertEqual(list(rows), ["plays.truth.asset"])
        self.assertEqual(rows["plays.truth.asset"],
                         {"strings": 3, "files": 3, "recording": 2, "title": 2, "relation": 2})


if __name__ == "__main__":
    unittest.main()
