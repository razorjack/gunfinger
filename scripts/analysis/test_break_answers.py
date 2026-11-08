import unittest

from break_answers import judgments


class Judgments(unittest.TestCase):
    def test_the_latest_answer_not_undone_counts(self):
        questions = {"questions": [
            {"id": "break-1", "details": ["special-branch-250 (rank 5 of 223)"], "files": ["q.mp3", "h1.mp3"]},
            {"id": "break-2", "details": ["parts-raid (rank 1 of 435)"], "files": ["r.mp3", "h2.mp3"]},
            {"id": "break-3", "details": ["parts-raid (rank 2 of 435)"], "files": ["r.mp3", "h3.mp3"]}]}
        records = [
            {"seq": 1, "action": "answer", "question": "break-1", "choice": "chance"},
            {"seq": 2, "action": "answer", "question": "break-1", "choice": "same"},
            {"seq": 3, "action": "answer", "question": "break-2", "choice": "chance"},
            {"seq": 4, "action": "undo", "undoes": 3, "question": "break-2"},
            {"seq": 5, "action": "answer", "question": "other-9", "choice": "same"}]

        self.assertEqual(judgments(questions, records), [("special-branch-250", "h1.mp3", "same")])


if __name__ == "__main__":
    unittest.main()
