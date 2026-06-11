# Freeze the M5 depth-claim manifest, feature-family packets, and qualification matrix

This document is the human-readable companion to the canonical M5 depth-claim manifest checked in at `artifacts/release/m5/freeze_the_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix.json`.

## Purpose

The depth-claim manifest freezes the canonical depth claim every M5 feature family publishes and the qualification matrix that grounds it. Where the feature-train matrix speaks for lanes, scorecards, and the dependency graph, this manifest is the single control source for *what each family may claim* and *why*. No family keeps a Stable depth claim once its proof freshness expires, generated-artifact lineage is missing, locale parity drifts, or its support packet lags shipped behavior.

## Structure

The manifest contains:

- **Feature-family packets** — one per M5 family (`notebook`, `data_rich`, `ai_adjacent`, `framework`, `review`, `companion`, `managed_depth`). Each packet binds the family to the stable depth claim it backs and the lifecycle label it effectively publishes after narrowing.
- **Qualification matrix** — one cell per qualification dimension, per family: `scorecard`, `compatibility`, `proof_freshness`, `lineage`, `locale_parity`, `support_packet`, `accessibility`, and `downgrade_automation`. Every dimension is an explicit, inspectable truth; the matrix must cover every dimension exactly once.
- **Narrowing reasons** — the closed set of reasons a family drops below the cutline. A non-qualified, non-waived cell must name its narrowing reason.
- **Stop rules** — closed conditions that gate promotion. Every narrowing reason has a corresponding rule.
- **Promotion verdict** — `proceed` or `hold`, computed from the firing stop rules.

## Narrowing rules

- A family carries a Stable (or LTS) depth claim only when its qualification matrix is fully qualified, the proof packet is current within its freshness SLO, any waiver is unexpired, lineage is present, locale parity holds, the support packet matches shipped behavior, accessibility is signed, and the owner has signed off.
- A family that loses any of those must drop **below** the cutline rather than inherit an adjacent qualified family. The published label is a hard ceiling: it may never exceed the depth claim's canonical label.
- A family held provisionally rides an active, unexpired waiver; an expired waiver narrows it.

## Consumption

Downstream docs, Help/About, CLI inspection, and support-export surfaces should ingest `support_export_projection()` from the typed model rather than cloning status text.

## Freshness

The manifest is current as of the `as_of` date embedded in the JSON artifact. CI gates recompute the promotion verdict against the stable claim manifest and fail promotion if the manifest is stale or underqualified.
