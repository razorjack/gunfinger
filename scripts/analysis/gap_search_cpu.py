"""Session 84: the timing scans' process CPU (user + system) and the reports' wall_seconds per arm.

gap_search_cpu.py LOGS PREFIX OUT.json: reads LOGS/PREFIX-<set>-<i>-<arm>.log (with /usr/bin/time -v)
and the scan report beside it (.json); prints medians, ranges and ratios to the first arm (L0 or B).
"""
import glob
import json
import os
import re
import statistics
import sys

logs, prefix, out_path = sys.argv[1:4]
rows = {}
for log in sorted(glob.glob(f"{logs}/{prefix}-*.log")):
    name = os.path.basename(log)[len(prefix) + 1:-4]
    m = re.match(r"(.+)-(\d+)-(\w+)$", name)
    s, i, arm = m.group(1), int(m.group(2)), m.group(3)
    text = open(log).read()
    user = float(re.search(r"User time \(seconds\): ([\d.]+)", text).group(1))
    system = float(re.search(r"System time \(seconds\): ([\d.]+)", text).group(1))
    wall = json.load(open(log[:-4] + ".json"))["wall_seconds"]
    rows.setdefault(s, {}).setdefault(arm, []).append((user + system, wall))
out = {}
for s, arms in rows.items():
    ref = next(iter(a for a in ("L0", "B") if a in arms))
    base_cpu = statistics.median(c for c, _ in arms[ref])
    base_wall = statistics.median(w for _, w in arms[ref])
    for arm, values in arms.items():
        cpu = [c for c, _ in values]
        wall = [w for _, w in values]
        out[f"{s} {arm}"] = dict(cpu=statistics.median(cpu), cpu_range=[min(cpu), max(cpu)],
                                 cpu_ratio=round(statistics.median(cpu) / base_cpu, 4),
                                 wall=round(statistics.median(wall), 2),
                                 wall_ratio=round(statistics.median(wall) / base_wall, 4), runs=len(values))
        print(f"{s:34s} {arm:3s} CPU {statistics.median(cpu):7.1f} s [{min(cpu):.1f}-{max(cpu):.1f}] x{statistics.median(cpu)/base_cpu:.3f}; "
              f"search {statistics.median(wall):5.1f} s x{statistics.median(wall)/base_wall:.3f} ({len(values)} runs)")
json.dump(out, open(out_path, "w"), indent=1)
