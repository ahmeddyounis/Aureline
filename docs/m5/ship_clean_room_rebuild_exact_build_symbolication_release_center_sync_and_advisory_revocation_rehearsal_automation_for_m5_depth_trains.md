# Ship clean-room rebuild, exact-build symbolication, release-center sync, and advisory/revocation rehearsal automation for M5 depth trains

This document is the human-readable companion to the canonical M5
rehearsal-automation register checked in at
`artifacts/release/m5/ship_clean_room_rebuild_exact_build_symbolication_release_center_sync_and_advisory_revocation_rehearsal_automation_for_m5_depth_trains.json`.

## Purpose

Where the clean-room rebuild *proof* records the static posture of a marketed
channel, this register is the *automation* layer on top of it. For every claimed
M5 artifact family it runs four standing rehearsals and records their
machine-readable result and expiry state, so claim-narrowing, shiproom
dashboards, the evidence index, and support exports all read one source of truth
instead of tribal memory. A red, stale, missing, or guardrail-tripped rehearsal
narrows the affected family below the launch cutline and holds promotion.

## The four rehearsals

Each family row carries exactly one record per rehearsal kind:

- **`clean_room_rebuild`** — a from-clean-state rebuild proof. A warm-cache-only
  run (`rebuild_provenance: warm_cache_only`) never counts as rebuild proof; it
  narrows the row via the `rebuild_cache_only` gap reason.
- **`exact_build_symbolication`** — exact-build symbol/source-map verification.
  Its freshness may never run ahead of the release-center sync rehearsal; if it
  does, the row narrows via `symbolication_freshness_decoupled`.
- **`release_center_sync`** — release-center / mirror / offline parity check that
  also grounds the support and export freshness surfaces.
- **`advisory_revocation_drill`** — the advisory / emergency-disable / revocation
  rehearsal.

Every rehearsal carries a proof packet whose freshness-SLO state is the expiry
signal (`current`, `due_for_refresh`, `breached`, or `missing`).

## Structure

The register contains:

- **Family rows** — one per M5 artifact family (`notebook_pack`,
  `request_data_asset`, `profiler_replay_artifact`, `framework_template_pack`,
  `docs_pack`, `model_pack`, `companion_offboarding_packet`, `managed_output`).
- **Rehearsal records** — four per row, each with a result, a proof packet, and a
  rebuild-provenance flag.
- **Stop rules** — closed conditions that gate promotion. Every gap reason has a
  corresponding rule.
- **Promotion verdict** — `proceed` or `hold`, computed from firing stop rules.

## Guardrails

- **Warmed caches are not rebuild proof.** A clean-room rebuild rehearsal that
  only rebuilt from a warmed cache narrows its family and holds promotion.
- **Symbolication freshness stays coupled to release-center freshness.** Exact-build
  symbolication freshness may never lead the release-center sync that grounds
  support and export freshness; if it does, the family narrows.

## Consumption

Downstream docs, Help/About, service-health, CLI inspection, and support-export
surfaces ingest `support_export_projection()` from the typed model rather than
cloning status text. Shiproom dashboards and the canonical evidence index ingest
`rehearsal_expiry_feed()` — the same machine-readable rehearsal-result and expiry
state — so neither reconstructs freshness from prose.

## Freshness

The register is current as of the `as_of` date embedded in the JSON artifact. CI
gates recompute the promotion verdict against the stable claim manifest and fail
promotion if the register is stale or underqualified.
