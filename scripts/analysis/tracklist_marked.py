"""Session 39 (experiment 0183): marked plays (shares_material_with) in the drops mixes' reports, split by truth, with their hits, length and hits against the partner. Run from the repository root."""
import json, sys, statistics
sys.path.insert(0, 'scripts/analysis')
import tracklist_score as t
NAS = '/home/razorjack/.local/share/gunfinger/nas-dnb-peaks'
for scale, clusters, store in (('262', 'work/reports/duplicate-clusters.json', None),
                               ('nas', 'work/reports/library-nas-dnb-peaks/duplicate-clusters.json', NAS)):
    groups = {}
    for s in (2026, 2028):
        lib = t.Library(clusters, 'work/s39/panels/shared-material-seed-2026.json',
                        store or f'work/s39/store-{s}', f'work/s39/nas-exclude-{s}.txt' if store else None)
        mixes = {x['mix']['name']: x['mix'] for x in json.load(open(f'work/reports/mixes-drops-seed-{s}.json'))['mixes']}
        for name, mix in mixes.items():
            plays = json.load(open(f'work/s39/cli-{scale}/seed-{s}/{name}.json'))['plays']
            for p in plays:
                p['asset'] = lib.key(p['asset']); p['same_audio'] = [lib.key(x) for x in p.get('same_audio', [])]
            for tp in mix['plays']:
                tp['recording'] = lib.recording(tp['asset']); tp['family'] = lib.family_of(tp['asset'])
            for p in plays:
                if 'shares_material_with' not in p:
                    continue
                partner = plays[p['shares_material_with']['play'] - 1]
                playing = [tp for tp in mix['plays'] if not tp['held_out'] and t.holds(p, tp['recording']) and t.overlap(p, tp)]
                fam = any(not tp['held_out'] and tp['family'] == lib.family_of(p['asset']) and t.overlap(p, tp) for tp in mix['plays'])
                kind = ('playing ' + playing[0].get('scenario', '-')) if playing else ('related' if fam else 'other')
                best = max(seg['hits'] for seg in p['segments'])
                groups.setdefault(kind, []).append((best, p['end_seconds'] - p['start_seconds'], best / max(seg['hits'] for seg in partner['segments'])))
    print(scale)
    for kind, rows in sorted(groups.items()):
        h = sorted(r[0] for r in rows); d = sorted(r[1] for r in rows); q = sorted(r[2] for r in rows)
        print(f"  {kind:22} n={len(rows):3} best segment hits min/median/max {h[0]}/{statistics.median(h):.0f}/{h[-1]}"
              f"  seconds {d[0]:.0f}/{statistics.median(d):.0f}/{d[-1]:.0f}  hits/partner's best {q[0]:.3f}/{statistics.median(q):.3f}/{q[-1]:.3f}")
    for kind, rows in sorted(groups.items()):
        print(f"  {kind:22} >=40 s {sum(r[1] >= 40 for r in rows)}, >=150 best hits {sum(r[0] >= 150 for r in rows)}, ratio >=0.04 {sum(r[2] >= 0.04 for r in rows)}")
