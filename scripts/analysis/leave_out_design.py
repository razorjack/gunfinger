"""2026-10-21 (session 34): the leave-out patterns for measuring gaps on
detection edges.

Every run of k = 1-4 consecutive manifest slots with a kept, indexed track
on each side must be left out in at least one scan. A run of slots s..s+k-1
needs slot s+k kept, so two runs fit one scan when one starts after the
other's kept slot: as intervals [s, s+k] they must not overlap. Runs are
intervals, so the fewest scans is the most runs that cover one slot, and
assigning runs in order of their first slot to the first scan they fit
reaches it (interval graph colouring).

The first and the last slot stay kept. Slots outside FIRST..LAST stay kept
too: Nocny's slot 17 has no reference, so runs end at slot 15 and slot 16
stays as the neighbour that is indexed.

usage: python3 -I scripts/analysis/leave_out_design.py
"""

SETS = {
    # set: (first and last slot a run may hold)
    "stakka-skynet-knowledge": (2, 10),
    "razorjack-2003-03-29": (2, 11),
    "nocny-trans-vertical-2002-08-23": (2, 15),
}
RUN_LENGTHS = (1, 2, 3, 4)


def runs(first, last):
    return [
        (start, length)
        for start in range(first, last + 1)
        for length in RUN_LENGTHS
        if start + length - 1 <= last
    ]


def design(first, last):
    scans = []
    for start, length in sorted(runs(first, last)):
        for scan in scans:
            if scan[-1][0] + scan[-1][1] < start:
                scan.append((start, length))
                break
        else:
            scans.append([(start, length)])
    return scans


def most_covering(first, last):
    return max(
        sum(1 for start, length in runs(first, last) if start <= slot <= start + length)
        for slot in range(first, last + 2)
    )


def pattern(scan):
    return ",".join(
        str(start) if length == 1 else f"{start}-{start + length - 1}"
        for start, length in scan
    )


def main():
    for name, (first, last) in SETS.items():
        scans = design(first, last)
        print(
            f"{name}: {len(runs(first, last))} runs in {len(scans)} scans "
            f"(at least {most_covering(first, last)})"
        )
        for scan in scans:
            print(f"  {pattern(scan)}")


if __name__ == "__main__":
    main()
