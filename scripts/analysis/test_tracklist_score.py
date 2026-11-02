"""Tests of tracklist_score.py's line check on a made-up report.

    python3 -m unittest discover -s scripts/analysis -p 'test_tracklist_score.py'
"""

import unittest

import tracklist_score


def play(asset, start, end, confidence="confident"):
    return {"asset": asset, "start_seconds": start, "end_seconds": end,
            "confidence": confidence, "speed": 1.0, "track_start_seconds": 0.0, "hits": 300,
            "tags": {"artist": "Artist", "title": asset}}


PLAYS = [play("one", 0.0, 120.0), play("two", 200.0, 320.0, "possible")]
ENTRIES = [[PLAYS[0]], [PLAYS[1]]]


class CheckLines(unittest.TestCase):
    def test_gap_lines_are_passed_over(self):
        shown = (" 1.    0:00  Artist - one\n"
                 "       2:00  ID - ID (3:20 without a confident match)\n"
                 " 2.    3:20  Artist - two (possible)\n")

        tracklist_score.check_lines(shown, ENTRIES, PLAYS, "mix")

    def test_gap_lines_with_an_estimate_are_passed_over(self):
        shown = (" 1.    0:00  Artist - one\n"
                 "       2:00  ID - ID (3:20 without a confident match; about 1-2 tracks)\n"
                 " 2.    3:20  Artist - two (possible)\n")

        tracklist_score.check_lines(shown, ENTRIES, PLAYS, "mix")

    def test_a_missing_entry_line_still_stops_the_check(self):
        shown = (" 1.    0:00  Artist - one\n"
                 "       2:00  ID - ID (3:20 without a confident match)\n")

        with self.assertRaises(SystemExit):
            tracklist_score.check_lines(shown, ENTRIES, PLAYS, "mix")

    def test_an_id_entry_line_with_a_number_is_not_a_gap_line(self):
        shown = (" 1.    0:00  Artist - one\n"
                 " 2.    2:00  ID - ID (3:20 without a confident match)\n"
                 " 3.    3:20  Artist - two (possible)\n")

        with self.assertRaises(SystemExit):
            tracklist_score.check_lines(shown, ENTRIES, PLAYS, "mix")


def under(asset, start, end, louder, confidence="confident"):
    found = play(asset, start, end, confidence)
    found["under"] = {"play": louder, "asset": "one"}
    return found


class PlaysFoundUnderOthers(unittest.TestCase):
    def setUp(self):
        self.plays = [play("one", 0.0, 120.0), play("two", 200.0, 320.0, "possible")]
        self.under = [under("quiet", 30.0, 100.0, 1)]
        self.entries = [[self.plays[0]], [self.plays[1]]]
        self.under_entries = [[self.under[0]]]

    def test_a_confident_one_is_an_entry_marked_with_the_louder_entry(self):
        shown = (" 1.    0:00  Artist - one\n"
                 " 2.    0:30  Artist - quiet (under 1.)\n"
                 " 3.    3:20  Artist - two (possible)\n")

        tracklist_score.check_lines(shown, self.entries, self.plays, "mix",
                                    self.under_entries, self.under)

    def test_a_wrong_louder_number_stops_the_check(self):
        shown = (" 1.    0:00  Artist - one\n"
                 " 2.    0:30  Artist - quiet (under 2.)\n"
                 " 3.    3:20  Artist - two (possible)\n")

        with self.assertRaises(SystemExit):
            tracklist_score.check_lines(shown, self.entries, self.plays, "mix",
                                        self.under_entries, self.under)

    def test_a_missing_mark_stops_the_check(self):
        shown = (" 1.    0:00  Artist - one\n"
                 " 2.    0:30  Artist - quiet\n"
                 " 3.    3:20  Artist - two (possible)\n")

        with self.assertRaises(SystemExit):
            tracklist_score.check_lines(shown, self.entries, self.plays, "mix",
                                        self.under_entries, self.under)

    def test_a_right_one_is_printed_under_with_its_start_error(self):
        library = tracklist_score.Library.__new__(tracklist_score.Library)
        library.prefix, library.cluster, library.family = "", {}, {}
        library.indexed = {"one", "quiet", "two"}
        mix = {"name": "mix", "plays": [
            {"asset": "one", "start_seconds": 0.0, "end_seconds": 130.0},
            {"asset": "quiet", "start_seconds": 25.0, "end_seconds": 110.0},
            {"asset": "absent", "start_seconds": 140.0, "end_seconds": 190.0}]}
        report = {"plays": [play("one", 0.0, 120.0)],
                  "under": [under("quiet", 30.0, 100.0, 1), under("two", 140.0, 180.0, 1, "possible")]}
        shown = (" 1.    0:00  Artist - one\n"
                 " 2.    0:30  Artist - quiet (under 1.)\n")

        row = tracklist_score.score_mix(mix, report, shown, library)

        quiet = row["truth"][1]
        self.assertEqual(quiet["outcome"], "not found")
        self.assertEqual(quiet["outcome_with_under"], "printed under")
        self.assertEqual(quiet["under_start_error"], 5.0)
        self.assertEqual(row["under_entries"][0]["right"], True)
        self.assertEqual([(f["asset"], f["right"]) for f in row["under_found"]],
                         [("quiet", True), ("two", False)])


if __name__ == "__main__":
    unittest.main()
