"""Tests of tracklist_score.py's line check on a made-up report.

    python3 -m unittest discover -s scripts/analysis -p 'test_tracklist_score.py'
"""

import unittest

import tracklist_score


def play(asset, start, end, confidence="confident"):
    return {"asset": asset, "start_seconds": start, "end_seconds": end,
            "confidence": confidence, "speed": 1.0, "track_start_seconds": 0.0,
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


if __name__ == "__main__":
    unittest.main()
