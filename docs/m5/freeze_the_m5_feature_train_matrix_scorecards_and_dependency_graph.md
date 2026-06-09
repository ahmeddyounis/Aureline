# Freeze the M5 feature-train matrix, scorecards, and dependency graph

This document is the human-readable companion to the canonical M5 feature-train matrix checked in at `artifacts/release/m5/freeze_the_m5_feature_train_matrix_scorecards_and_dependency_graph.json`.

## Purpose

The M5 feature-train matrix freezes the canonical control surface for every M5 depth lane. It ensures that no notebook, AI, data, framework, review, companion, or managed-depth row may widen without fresh proof, compatibility report, admin/policy story, and rollback posture.

## Structure

The matrix contains:

- **Lane rows** — one per M5 lane (`notebook`, `data_rich`, `ai_adjacent`, `framework`, `review`, `companion`, `managed_depth`).
- **Scorecards** — per-lane proof packet, compatibility report, admin/policy story, and rollback path refs.
- **Dependency graph** — hard and soft edges between lanes. A hard dependency means the downstream lane must narrow if the upstream lane narrows.
- **Stop rules** — closed conditions that gate promotion. Every gap reason has a corresponding rule.
- **Promotion verdict** — `proceed` or `hold`, computed from firing stop rules.

## Consumption

Downstream docs, Help/About, CLI inspection, and support export surfaces should ingest `support_export_projection()` from the typed model rather than cloning status text.

## Freshness

The matrix is current as of the `as_of` date embedded in the JSON artifact. CI gates recompute the promotion verdict against the stable claim manifest and fail promotion if the matrix is stale or underqualified.
