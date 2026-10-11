# 0013: A confident segment makes its play confident

## Status

Accepted (2026-10-11, pre-release audit). Supersedes ADR 0006's selection
of play confidence by hit count. Thresholds and grouping are unchanged.

## Context

A confident segment could be followed by a shorter possible segment with
more hits but too few windows. Choosing confidence by hit count demoted
the whole play. The evaluation harness already used maximum confidence.

## Decision

A play takes the highest confidence independently reached by any segment.
Hits and windows are never pooled to qualify. The highest-hit segment
still supplies the representative speed and playback.

## Consequences

Additional evidence cannot demote a confident play. Core and harness agree
on play confidence, without changing any segment or confidence threshold.
