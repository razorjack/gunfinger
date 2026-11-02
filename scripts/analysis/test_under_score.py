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


class Cost(Files):
    def test_time_and_the_steps_line_are_read(self):
        stderr = os.path.join(self.dir, "run.err")
        write(stderr, "searched 56:09 of audio in 6.0 s (decoding 2.9 s)\n"
                      "searched under 11 confident tracks in 23.5 s\n"
                      "\tUser time (seconds): 40.50\n"
                      "\tSystem time (seconds): 2.25\n"
                      "\tElapsed (wall clock) time (h:mm:ss or m:ss): 1:02.50\n"
                      "\tMaximum resident set size (kbytes): 536000\n")
        self.assertEqual(under_score.timed(stderr), {
            "under_entries": 11, "under_seconds": 23.5, "user": 40.5, "system": 2.25,
            "wall": 62.5, "rss_kb": 536000.0})

    def test_a_report_without_the_step_is_the_report_less_its_finds(self):
        report = {"plays": [1], "search": {"ladder": "x", "under": "mask"}, "under": []}
        self.assertEqual(under_score.without_under(report),
                         {"plays": [1], "search": {"ladder": "x"}})
        self.assertIn("under", report)


class Fates(Files):
    def test_detections_of_a_play_dropped_as_one_track_keep_their_level(self):
        stderr = os.path.join(self.dir, "variant.err")
        write(stderr, "s73-recording\t/mixes/mix-00.mp3\n"
                      "s73-detection\tloud.mp3\tquiet.mp3\t10.0\t50.0\tPossible\t90\tcandidate\n"
                      "s73-detection\tloud.mp3\tother.mp3\t10.0\t50.0\tConfident\t300\tcandidate\n"
                      "s73-shared\tloud.mp3\tother.mp3\n"
                      "s73-one-track\tloud.mp3\tquiet.mp3\t10.0\t50.0\t90\n"
                      "s73-entry\tloud.mp3\t0.000\t100.000\t1\t1\t1\t1\t0\t0\n"
                      "s73-entry-total\tloud.mp3\t3.000\n")
        recording = json.loads(self.printed(under_score.fates, stderr))[0]
        self.assertEqual([(d["asset"], d["level"], d["fate"]) for d in recording["detections"]],
                         [("quiet.mp3", "Possible", "one-track"),
                          ("other.mp3", "Confident", "shared-material")])


class Diff2(Files):
    def test_a_line_printed_twice_counts_twice(self):
        default = os.path.join(self.dir, "a.txt")
        option = os.path.join(self.dir, "b.txt")
        write(default, " 1.    0:00  A - a\n")
        write(option, " 1.    0:00  A - a\n 2.    0:00  A - a (under 1.)\n")
        result = json.loads(self.printed(under_score.diff, default, option))
        self.assertEqual(result["gained"], ["0:00  A - a"])


class Budget(Files):
    def test_finds_of_entries_left_out_are_lost_and_their_time_saved(self):
        stderr = os.path.join(self.dir, "variant.err")
        write(stderr, "s73-recording\t/mixes/mix-00.mp3\n"
                      "s73-entry\tlong.mp3\t0.000\t200.000\t1\t1\t1\t1\t0\t1\n"
                      "s73-entry-total\tlong.mp3\t3.000\n"
                      "s73-entry\tshort.mp3\t300.000\t50.000\t1\t1\t1\t1\t0\t1\n"
                      "s73-entry-total\tshort.mp3\t1.000\n")
        fates = os.path.join(self.dir, "fates.json")
        write(fates, self.printed(under_score.fates, stderr))
        reports = os.path.join(self.dir, "reports")
        os.mkdir(reports)
        write(os.path.join(reports, "mix-00.json"), json.dumps({"plays": [
            {"asset": "long.mp3", "start_seconds": 0.0},
            {"asset": "short.mp3", "start_seconds": 300.0}]}))
        gained = os.path.join(self.dir, "gained.json")
        write(gained, json.dumps({"rows": [{"mix": "mix-00", "under_entries": [
            {"under_play": 1, "right": True}, {"under_play": 2, "right": True}]}]}))

        table = self.printed(under_score.budget, fates, gained, reports)

        self.assertIn("| all | 2 | 4.0 | 2 | 0 |", table)
        self.assertIn("| span >= 60 s | 1 | 3.0 | 1 | 0 |", table)
        self.assertIn("| 8 longest | 2 | 4.0 | 2 | 0 |", table)


if __name__ == "__main__":
    unittest.main()
