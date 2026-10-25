"""Session 49, step 5: how far apart in speed the copies of each NAS
cluster are, and which copies a play at a deck's pitch extremes would put
beyond the ladder (roadmap, "The edge of the ladder").

Each file's time relative to one member comes from the clusters' joins
(`same_recording` pairs: the found file's time over the query's), chained
through the cluster where two files were not compared directly, as
`relative_speed.py` does. A copy X played at speed d is searched against
copy Y at d times Y's time over X's; it is beyond the ladder when that is
more than LADDER from 1.

usage: cluster_speeds.py CLUSTERS OUT.json [DECK]
"""

import collections
import json
import sys

LADDER = 0.08
SPANS = (0.02, 0.04, 0.06, 0.08)


def times_in(members, pairs):
    """Each member's time over the first member's (sorted), through the
    cluster's joins; members no join reaches are left out."""
    edges = collections.defaultdict(dict)
    for p in pairs:
        if p["same_recording"] and p["query"] in members and p["found"] in members:
            edges[p["query"]][p["found"]] = p["speed"]
            edges[p["found"]][p["query"]] = 1.0 / p["speed"]
    first = sorted(members)[0]
    times, queue = {first: 1.0}, [first]
    while queue:
        here = queue.pop()
        for there, k in sorted(edges[here].items()):
            if there not in times:
                times[there] = times[here] * k
                queue.append(there)
    return times


def beyond(times, deck):
    """(played, searched, relative speed) for each two copies that a play
    at 1 - deck or 1 + deck puts beyond the ladder."""
    out = []
    for played, t_played in sorted(times.items()):
        for searched, t_searched in sorted(times.items()):
            if played == searched:
                continue
            for d in (1.0 - deck, 1.0 + deck):
                relative = d * t_searched / t_played
                if abs(relative - 1.0) > LADDER:
                    out.append((played, searched, relative))
    return out


def alone(times, deck):
    """(copy, deck speed) for each copy that, played at 1 - deck or
    1 + deck, has no other copy within the ladder: found only if that
    copy itself is indexed."""
    out = []
    for played, t_played in sorted(times.items()):
        for d in (1.0 - deck, 1.0 + deck):
            others = [abs(d * t / t_played - 1.0) for searched, t in times.items()
                      if searched != played]
            if others and min(others) > LADDER:
                out.append((played, d))
    return out


def summary(clusters, deck):
    rows = []
    for members in clusters["duplicates"]:
        times = times_in(set(members), clusters["pairs"])
        span = max(times.values()) / min(times.values()) - 1.0 if len(times) > 1 else 0.0
        out = beyond(times, deck)
        # A copy every other copy can reach at either extreme stays found
        # even when it is not the one played; these copies cannot.
        unreachable = sorted({searched for _, searched, _ in out})
        rows.append({"members": len(members), "without_speed": len(members) - len(times),
                     "span": span, "times": times,
                     "copies_beyond_for_some_play": unreachable,
                     "played_copies_out_of_reach_of_some_copy": sorted(
                         {played for played, _, _ in out}),
                     "alone": alone(times, deck)})
    rows.sort(key=lambda r: -r["span"])
    return {
        "clusters": len(rows), "deck": deck, "ladder": LADDER,
        "span_over": {f"{s:g}": sum(r["span"] > s for r in rows) for s in SPANS},
        "clusters_with_a_copy_beyond": sum(bool(r["copies_beyond_for_some_play"])
                                           for r in rows),
        "copies_beyond_for_some_play": sum(len(r["copies_beyond_for_some_play"])
                                           for r in rows),
        "members_without_speed": sum(r["without_speed"] for r in rows),
        "copies_alone": [a for r in rows for a in r["alone"]],
        "rows": rows,
    }


def main():
    clusters_file, out = sys.argv[1], sys.argv[2]
    deck = float(sys.argv[3]) if len(sys.argv) > 3 else 0.05
    with open(clusters_file) as file:
        result = summary(json.load(file), deck)
    with open(out, "w") as file:
        json.dump(result, file, indent=1, ensure_ascii=False)
    print({k: v for k, v in result.items() if k != "rows"})
    for r in result["rows"]:
        if r["span"] > 0.04:
            ordered = sorted(r["times"].items(), key=lambda item: item[1])
            print(f"span {100 * r['span']:.1f}% over {r['members']} members:")
            for path, t in ordered:
                print(f"   {t:.4f} {path}")


if __name__ == "__main__":
    main()
