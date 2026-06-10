# AI Review Findings, Ownership Hints, and Durable Anchor Fixtures

This directory contains fixture files for the AI review-findings lane, which binds
read-only AI review findings — each anchored to a location by a durable anchor
that survives edits — together with advisory ownership hints and review-workspace
integration that publishes findings into review only behind a human gate.

## Files

- `valid_packet.json` — A fully valid review-findings packet that passes all
  validation invariants. Mirrors the checked-in support export.
- `anchor_drift_undisclosed.json` — A packet whose drifted `style` finding leaves
  its anchor's `rebind_disclosed` false, triggering `anchor_drift_undisclosed`.
