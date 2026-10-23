"""Tests of listening_pack.py's selection on a made-up clusters report.

    python3 -m unittest discover -s scripts/analysis -p 'test_*.py'
"""

import json
import os
import sys
import tempfile
import unittest

import listening_pack


def pair(query, found, coverage, hits, same=False):
    return {
        "query": query, "found": found, "coverage": coverage, "speed": 1.0, "hits": hits,
        "same_recording": same, "query_start_seconds": 0.0, "query_end_seconds": 100.0,
        "found_start_seconds": 0.0, "found_end_seconds": 100.0,
    }


BORDERLINE = {
    "duplicates": [],
    "pairs": [
        pair("a.mp3", "second-library/a.opus", 0.73, 1452),
        pair("b.mp3", "second-library/b.opus", 0.41, 6),
        pair("c.mp3", "second-library/c.opus", 0.45, 30),
    ],
}

# Two clusters (d and e, three files each) and pairs kept apart.
DISAGREEING = {
    "duplicates": [["d.mp3", "second-library/d.opus", "second-library/d2.opus"],
                   ["e.mp3", "second-library/e.opus", "second-library/e2.opus"]],
    "pairs": [
        pair("d.mp3", "second-library/d.opus", 0.99, 900, same=True),
        pair("d.mp3", "second-library/d2.opus", 0.98, 900, same=True),
        pair("e.mp3", "second-library/e.opus", 0.97, 900, same=True),
        pair("d.mp3", "second-library/e.opus", 0.30, 300),
        pair("d.mp3", "second-library/e2.opus", 0.25, 250),
        pair("e.mp3", "second-library/f.opus", 0.20, 20),
    ],
}

MEASURES = [
    {"query": "d.mp3", "found": "second-library/d.opus", "near_10": 0.95},
    {"query": "d.mp3", "found": "second-library/d2.opus", "near_10": 0.40},
    {"query": "e.mp3", "found": "second-library/e.opus", "near_10": 0.60},
    {"query": "d.mp3", "found": "second-library/e.opus", "near_10": 0.70},
    {"query": "d.mp3", "found": "second-library/e2.opus", "near_10": 0.80},
    {"query": "e.mp3", "found": "second-library/f.opus", "near_10": 0.90},
]


class Select(unittest.TestCase):
    def select(self, *options, clusters=BORDERLINE):
        folder = tempfile.mkdtemp()
        store = "".join(f"{p[len('second-library/'):]}\n" for p in (
            pair_["found"] for pair_ in clusters["pairs"]))
        files = {
            "clusters": json.dumps(clusters),
            "verdicts": "# none yet\n",
            "store": store,
        }
        with open(os.path.join(folder, "measures"), "w") as file:
            json.dump(MEASURES, file)
        options = [os.path.join(folder, "measures") if o == "MEASURES" else o for o in options]
        for name, text in files.items():
            with open(os.path.join(folder, name), "w") as file:
                file.write(text)
        out_pairs, out_items = os.path.join(folder, "pairs"), os.path.join(folder, "items")
        arguments = [os.path.join(folder, name) for name in files] + [out_pairs, out_items]
        stdout, sys.stdout = sys.stdout, open(os.devnull, "w")
        try:
            listening_pack.select(arguments + ["--controls", "0", *options])
        finally:
            sys.stdout.close()
            sys.stdout = stdout
        with open(out_items) as file:
            return [(item["kind"], item["pairs"][0]["query"], item["pairs"][0]["found"])
                    for item in json.load(file)]

    def queries(self, *options):
        return [query for _, query, _ in self.select(*options)]

    def test_borderline_pairs_on_fewer_than_30_hits_are_left_out(self):
        self.assertEqual(self.queries(), ["a.mp3", "c.mp3"])

    def test_the_minimum_of_hits_can_be_set(self):
        self.assertEqual(self.queries("--min-hits", "5"), ["a.mp3", "c.mp3", "b.mp3"])
        self.assertEqual(self.queries("--min-hits", "100"), ["a.mp3"])

    def test_disagreements_are_the_lowest_joins_and_the_highest_pairs_apart(self):
        chosen = self.select("--disagreements", "1", "--measures", "MEASURES",
                             clusters=DISAGREEING)

        # The join holding least near the offset (d2, 0.40); the pair apart
        # holding most with 30 hits or more (e2, 0.80; f has 20 hits), one
        # per pair of clusters.
        self.assertEqual(chosen, [
            ("disagreement", "d.mp3", "second-library/d2.opus"),
            ("disagreement", "d.mp3", "second-library/e2.opus"),
        ])

    def test_without_the_option_no_disagreement_is_chosen(self):
        self.assertEqual(self.select(clusters=DISAGREEING), [])


if __name__ == "__main__":
    unittest.main()
