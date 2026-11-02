"""Tests of under_score.py on made-up manifests, clusters and tracklists.

    python3 -m unittest discover -s scripts/analysis -p 'test_under_score.py'
"""

import io
import json
import os
import tempfile
import unittest
from contextlib import redirect_stdout

import under_score


def write(path, text):
    with open(path, "w") as file:
        file.write(text)

MANIFEST = """
[[track]]
position = 1
start = "0:00"
reference = ["a.mp3"]

[[track]]
position = 2
start = "5:00"
reference = ["b.mp3"]

[[track]]
position = 3
start = "10:00"
reference = ["c.mp3", "extra/c2.mp3"]
"""

CLUSTERS = {"duplicates": [["c.mp3", "extra/c2.mp3", "second-library/nas/c.flac"],
                           ["b.mp3", "second-library/nas/b.mp3"]]}


class Files(unittest.TestCase):
    def setUp(self):
        self.dir = tempfile.mkdtemp()
        self.manifest = os.path.join(self.dir, "tracklist.toml")
        write(self.manifest, MANIFEST)
        self.clusters = os.path.join(self.dir, "clusters.json")
        write(self.clusters, json.dumps(CLUSTERS))

    def printed(self, function, *arguments):
        out = io.StringIO()
        with redirect_stdout(out):
            function(*arguments)
        return out.getvalue()


class Exclude(Files):
    def test_corpus_files_of_the_slots_clusters(self):
        out = os.path.join(self.dir, "out.txt")
        self.printed(under_score.exclude, self.manifest, "2-3", self.clusters, out)
        with open(out) as file:
            self.assertEqual(file.read().split(), ["b.mp3", "c.mp3", "extra/c2.mp3"])

    def test_nas_files_and_copies_with_a_library_map(self):
        out = os.path.join(self.dir, "out.txt")
        library_map = os.path.join(self.dir, "map.json")
        write(library_map, json.dumps({"copies": [{"corpus": "a.mp3", "stands_for": "nas/a.mp3"}]}))
        self.printed(under_score.exclude, self.manifest, "1,3", self.clusters, out, library_map)
        with open(out) as file:
            self.assertEqual(file.read().split(), ["nas/a.mp3", "nas/c.flac"])


class Judge(Files):
    def test_a_file_of_a_playing_slots_cluster_is_right(self):
        truth = under_score.Truth(self.manifest, self.clusters, nas=True)
        right, _ = truth.judge(["nas/c.flac"], 590.0, 700.0)
        self.assertEqual([slot["position"] for slot in right], [3])

    def test_a_file_of_a_slot_not_playing_then_is_wrong(self):
        truth = under_score.Truth(self.manifest, self.clusters, nas=False)
        right, _ = truth.judge(["c.mp3"], 100.0, 200.0)
        self.assertEqual(right, [])

    def test_a_find_in_a_left_out_slot_is_named(self):
        truth = under_score.Truth(self.manifest, self.clusters, nas=False)
        right, left_out = truth.judge(["x.mp3"], 320.0, 400.0, {2})
        self.assertEqual((right, [slot["position"] for slot in left_out]), ([], [2]))


class Diff(Files):
    def test_gained_entries_without_numbers_and_marks_and_gaps_apart(self):
        default = os.path.join(self.dir, "a.txt")
        option = os.path.join(self.dir, "b.txt")
        write(default, " 1.    0:00  A - a\n"
                                 "       2:00  ID - ID (1:00 without a confident match)\n"
                                 " 2.    5:00  B - b\n")
        write(option, " 1.    0:00  A - a\n"
                                " 2.    1:00  Q - q (under 1.)\n"
                                "       2:00  ID - ID (1:00 without a confident match)\n"
                                " 3.    5:00  B - b\n")
        result = json.loads(self.printed(under_score.diff, default, option))
        self.assertEqual((result["lost"], result["gained"], result["gaps_changed"]),
                         ([], ["1:00  Q - q"], False))


if __name__ == "__main__":
    unittest.main()
