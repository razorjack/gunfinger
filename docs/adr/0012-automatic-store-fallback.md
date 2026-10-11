# 0012: Require explicit store fallback after a partial library scan

## Status

Accepted (2026-10-11, pre-release audit). Supplements ADR 0009; explicit
store-only behavior is unchanged.

## Context

A nested library read failure silently switched to all stored records,
including files excluded by a successfully read ignore file. This widened
the searched collection despite an explicit library request.

## Decision

Automatic fallback is allowed only when the library root is missing.
If the root exists, a scan failure stops the command with actionable advice.
The user can explicitly choose `--store-only`, which still searches all
current stored records regardless of the library's ignore file.

## Consequences

An unreadable subfolder cannot silently widen the searched collection.
Offline missing-root use continues to work; permission and other read errors
require correction or an explicit store-only request.
