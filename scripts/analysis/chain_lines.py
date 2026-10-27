"""Session 59 (experiment 0242): the first-pass candidates of chosen queries
in a `recall --trace-candidates` report, those of the query's own recording,
with their chain's lines (window, rung, hits, span).
Usage: chain_lines.py REPORT POSITION..."""
import json, sys

queries = json.load(open(sys.argv[1]))["queries"]
for position in map(int, sys.argv[2:]):
    query = queries[position]
    print(f"query {position}: {query['asset'].split('/')[-1]} at {query['speed_percent']:+}%")
    for candidate in query.get("candidates") or []:
        if not candidate["correct"]:
            continue
        print(f"  {candidate['hits']} hits, {candidate['windows']} windows at {candidate['speed_percent']:+.4f}%")
        for line in candidate.get("lines", []):
            print(f"    window {line['window']}: rung {line['speed_percent']:+.3f}%, {line['hits']} hits, {line['start_seconds']:.2f}-{line['end_seconds']:.2f} s")
