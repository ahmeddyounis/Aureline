# Freeze the M5 rollback, downgrade, claim-narrowing, and staged-promotion rules

This document is the human-readable companion to the canonical M5 rollback/downgrade register checked in at `artifacts/release/m5/freeze_the_m5_rollback_downgrade_claim_narrowing_and_staged_promotion_rules.json`.

## Purpose

The M5 rollback/downgrade register freezes the canonical control surface for how every M5 depth lane rolls back, downgrades, narrows its public claim, and advances through staged promotion. It ensures that no notebook, AI, data, framework, review, companion, or managed-depth row may ship without a defined, tested, and (for release-blocking lanes) exercised rollback path; without downgrade and claim-narrowing automation rules; and without a clear staged-promotion posture.

## Structure

The register contains:

- **Lane rows** — one per M5 lane (`notebook`, `data_rich`, `ai_adjacent`, `framework`, `review`, `companion`, `managed_depth`).
- **Rollback path states** — `defined`, `tested`, `exercised`, or `missing`. Only `tested` and `exercised` paths allow stable promotion.
- **Downgrade rules** — per-lane rules that prescribe how the lane narrows (`automatic_narrowing`, `manual_hold`, `emergency_rollback`, `staged_reversal`).
- **Claim-narrowing rules** — per-lane automation that narrows the claim when specific gap reasons are active.
- **Promotion stages** — `canary`, `pilot`, `stable` stage tracking for staged rollouts.
- **Stop rules** — closed conditions that gate promotion. Every gap reason has a corresponding rule.
- **Promotion verdict** — `proceed` or `hold`, computed from firing stop rules.

## Consumption

Downstream docs, Help/About, CLI inspection, and support export surfaces should ingest `support_export_projection()` from the typed model rather than cloning status text.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact. CI gates recompute the promotion verdict against the stable claim manifest and the M5 feature-train matrix, and fail promotion if the register is stale or underqualified.
