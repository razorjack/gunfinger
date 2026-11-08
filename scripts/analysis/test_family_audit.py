import unicodedata
import unittest

from family_audit import Key, across, left_out_audit, recall_audit, within


def key(files):
    return Key({"sources": ["test"], "files": {
        f: {"recording": rec, "title": title, "relation": rel} for f, (rec, title, rel) in files.items()}})


KEY = key({
    "a.mp3": ("a.mp3", "a.mp3", "a.mp3"),
    "a-rip.mp3": ("a.mp3", "a.mp3", "a.mp3"),
    "a-remix.mp3": ("a-remix.mp3", "a-remix.mp3", "a.mp3"),
    "same-title.mp3": ("same-title.mp3", "a.mp3", "a.mp3"),
})


class Across(unittest.TestCase):
    def test_a_rip_on_the_other_side_is_found_at_every_tier(self):
        found = across(KEY, {"dev": ["a.mp3"]}, {"val": ["a-rip.mp3"], "other": ["b.mp3"]})
        self.assertEqual([len(found[t]) for t in ("recording", "title", "relation")], [1, 1, 1])
        self.assertEqual(found["recording"][0]["b"], ["val", "a-rip.mp3"])

    def test_a_remix_on_the_other_side_is_found_at_the_relation_tier_only(self):
        found = across(KEY, {"dev": ["a.mp3"]}, {"val": ["a-remix.mp3"]})
        self.assertEqual([len(found[t]) for t in ("recording", "title", "relation")], [0, 0, 1])

    def test_names_are_compared_in_nfc(self):
        decomposed = unicodedata.normalize("NFD", "tête.mp3")
        found = across(key({}), {"dev": ["tête.mp3"]}, {"val": [decomposed]})
        self.assertEqual(len(found["recording"]), 1)


class Within(unittest.TestCase):
    def test_two_sources_of_one_family_on_one_side(self):
        found = within(KEY, {"x": ["a.mp3"], "y": ["same-title.mp3"], "z": ["b.mp3"]})
        self.assertEqual(found["recording"], [])
        self.assertEqual(found["title"], [["a.mp3", ["x", "y"]]])


class LeftOut(unittest.TestCase):
    def test_family_members_the_clusters_in_use_leave_indexed(self):
        in_use = key({"a.mp3": ("a.mp3", "a.mp3", "a.mp3")})
        found = left_out_audit({"a.mp3": ["a.mp3"]}, in_use, KEY)
        self.assertEqual(found, [
            {"left_out": "a.mp3", "tier": "recording", "still_indexed": ["a-rip.mp3"]},
            {"left_out": "a.mp3", "tier": "title", "still_indexed": ["a-rip.mp3", "same-title.mp3"]},
        ])


class Recall(unittest.TestCase):
    def test_the_split_key_and_the_families_of_both_halves(self):
        source = lambda asset, family: {"asset": asset, "family": family}
        panel = {"development": {"sources": [source("a.mp3", "artist - a")]},
                 "validation": {"sources": [source("a-rip.mp3", "artist - a rip"), source("b.mp3", "b")]}}
        result = recall_audit(panel, KEY)
        self.assertEqual(result["split_key_overlap"], [])
        self.assertEqual(len(result["across"]["recording"]), 1)
        self.assertEqual(result["validation_families"]["recording"], 2)


if __name__ == "__main__":
    unittest.main()
