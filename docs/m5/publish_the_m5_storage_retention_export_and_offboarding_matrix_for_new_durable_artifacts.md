# Publish the M5 storage, retention, export, and offboarding matrix for new durable artifacts

This document is the human-readable companion to the canonical M5 storage, retention, export, and offboarding matrix checked in at `artifacts/release/m5/publish_the_m5_storage_retention_export_and_offboarding_matrix_for_new_durable_artifacts.json`.

## Purpose

The M5 storage, retention, export, and offboarding matrix publishes the canonical control surface for every new durable artifact class introduced in Milestone 5. It ensures that no user-owned local, workspace-owned managed, AI memory, sync state, session export, or derived cache artifact class may widen without a complete retention posture, a named owner, and current evidence.

## Structure

The matrix contains:

- **Artifact rows** — one per M5 durable artifact class (`user_owned_local`, `workspace_owned_managed`, `ai_memory`, `sync_state`, `session_export`, `derived_cache`).
- **Retention posture** — per-class set of required posture indicators (`storage_defined`, `retention_defined`, `export_defined`, `offboarding_tested`), each with its state and artifact ref.
- **Owner map** — each class binds to a canonical owner with a sign-off record.
- **Stop rules** — closed conditions that gate publication. Every gap reason has a corresponding rule.
- **Publication verdict** — `proceed` or `hold`, computed from firing stop rules.

## Consumption

Downstream docs, Help/About, CLI inspection, and support export surfaces should ingest `support_export_projection()` from the typed model rather than cloning status text.

## Freshness

The matrix is current as of the `as_of` date embedded in the JSON artifact. CI gates recompute the publication verdict against the stable claim manifest and the M5 feature-train matrix, and fail publication if the matrix is stale or underqualified.
