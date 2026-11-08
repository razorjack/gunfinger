"""Writes the judgments `break_retrieval.py --judgments` reads from the
answers to `questions breaks` (session 86).

    python3 -I scripts/analysis/break_answers.py QUESTIONS.json ANSWERS.jsonl OUT.tsv

For each question of the file, its answer is its latest `answer` record that
no `undo` took back (`docs/audition-questions.md`, "The answer log"). Each
line: the query's name (the first detail line up to " ("), the holder, and
the answer (`same`, `chance` or `unsure`)."""

import json
import sys


def answers(records):
    """The answer of each question: its latest answer not undone."""
    undone = {r["undoes"] for r in records if r.get("action") == "undo"}
    latest = {}
    for r in sorted(records, key=lambda r: r["seq"]):
        if r.get("action") == "answer" and r["seq"] not in undone:
            latest[r["question"]] = r["choice"]
    return latest


def judgments(questions, records):
    given = answers(records)
    lines = []
    for q in questions["questions"]:
        if q["id"] in given:
            name = q["details"][0].split(" (")[0]
            lines.append((name, q["files"][1], given[q["id"]]))
    return lines


def main(argv):
    questions = json.load(open(argv[1]))
    records = [json.loads(line) for line in open(argv[2]) if line.strip()]
    with open(argv[3], "w") as out:
        for line in judgments(questions, records):
            out.write("\t".join(line) + "\n")


if __name__ == "__main__":
    main(sys.argv)
