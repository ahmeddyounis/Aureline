# Publish the M5 admin/policy story register for notebook, AI, data, companion, and sync lanes

This document is the human-readable companion to the canonical M5 admin/policy story register checked in at `artifacts/release/m5/publish_the_m5_admin_policy_story_register_for_notebook_ai_data_companion_and_sync_lanes.json`.

## Purpose

The M5 admin/policy story register publishes the canonical control surface for every M5 depth lane's privacy, trust, access-control, audit, consent, and rollback posture. It ensures that no notebook, AI, data, companion, or sync lane may widen without a complete admin/policy story, a named owner, and current evidence.

## Structure

The register contains:

- **Lane rows** — one per M5 admin/policy lane (`notebook`, `ai_adjacent`, `data_rich`, `companion`, `sync`).
- **Admin/policy story** — per-lane set of required story items (`privacy_disclosure`, `data_retention`, `access_control`, `audit_trail`, `consent_management`, `rollback_policy`), each with its state and artifact ref.
- **Owner map** — each lane binds to a canonical owner with a sign-off record.
- **Stop rules** — closed conditions that gate publication. Every gap reason has a corresponding rule.
- **Publication verdict** — `proceed` or `hold`, computed from firing stop rules.

## Consumption

Downstream docs, Help/About, CLI inspection, and support export surfaces should ingest `support_export_projection()` from the typed model rather than cloning status text.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact. CI gates recompute the publication verdict against the stable claim manifest and the M5 feature-train matrix, and fail publication if the register is stale or underqualified.
