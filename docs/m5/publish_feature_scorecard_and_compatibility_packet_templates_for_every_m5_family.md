# Publish feature scorecard and compatibility-packet templates for every M5 family

This document is the human-readable companion to the canonical M5 template register checked in at `artifacts/release/m5/publish_feature_scorecard_and_compatibility_packet_templates_for_every_m5_family.json`.

## Purpose

The M5 template register publishes the canonical scorecard and compatibility-packet templates for every M5 feature family. It ensures that no notebook, AI, data, framework, review, companion, or managed-depth family may widen without a complete, current scorecard template and compatibility-packet template.

## Structure

The register contains:

- **Family rows** — one per M5 family (`notebook`, `data_rich`, `ai_adjacent`, `framework`, `review`, `companion`, `managed_depth`).
- **Scorecard templates** — per-family required sections (proof packet, compatibility report, admin/policy story, rollback path, owner sign-off) with publication state.
- **Compatibility-packet templates** — per-family required sections (schema surface, API surface, CLI surface, platform matrix, downgrade behavior, mixed-version posture, deprecation window) with publication state.
- **Stop rules** — closed conditions that gate publication. Every gap reason has a corresponding rule.
- **Publication verdict** — `proceed` or `hold`, computed from firing stop rules.

## Consumption

Downstream docs, Help/About, CLI inspection, and support export surfaces should ingest `support_export_projection()` from the typed model rather than cloning status text.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact. CI gates recompute the publication verdict against the stable claim manifest and fail promotion if the register is stale or underqualified.
