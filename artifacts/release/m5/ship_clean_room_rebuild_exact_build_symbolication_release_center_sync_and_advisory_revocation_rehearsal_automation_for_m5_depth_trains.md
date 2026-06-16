# M5 Rehearsal-Automation Register

## Overview

This artifact is the canonical M5 rehearsal-automation register. It binds every
claimed M5 artifact family to four standing rehearsals — clean-room rebuild,
exact-build symbolication, release-center sync, and advisory/revocation drill —
and records their machine-readable result and expiry state so a stale, red,
missing, or guardrail-tripped rehearsal narrows its family automatically before
promotion.

## Checked-in artifact

- `artifacts/release/m5/ship_clean_room_rebuild_exact_build_symbolication_release_center_sync_and_advisory_revocation_rehearsal_automation_for_m5_depth_trains.json`

## Schema

- `schemas/governance/ship_clean_room_rebuild_exact_build_symbolication_release_center_sync_and_advisory_revocation_rehearsal_automation_for_m5_depth_trains.schema.json`

## Fixtures

- `fixtures/release/m5/ship_clean_room_rebuild_exact_build_symbolication_release_center_sync_and_advisory_revocation_rehearsal_automation_for_m5_depth_trains/`

## Guardrails enforced

- Warmed caches alone never stand in for clean-room rebuild proof.
- Exact-build symbolication freshness is never decoupled from release-center
  sync (and therefore support/export) freshness.

## Downstream consumers

- `crates/aureline-release` — typed model, validation, and export/feed projections
- shiproom dashboards — ingest `rehearsal_expiry_feed()`
- canonical evidence index — ingests `rehearsal_expiry_feed()`
- Help/About, service-health, and support export — ingest `support_export_projection()`
