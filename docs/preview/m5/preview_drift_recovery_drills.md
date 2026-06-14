# M5 preview drift-recovery drills

This document is the contract for the M5 preview drift-recovery drills. It
hardens the claimed M5 preview lanes for the failures users actually hit in
framework / runtime work — hot-reload resets, stale source maps, lost dev
servers, device reconnects, expired browser sessions, replaced runtimes, and
data-posture flips — by binding each failure onto one governed transition object
that proves the session fails honestly instead of going blank or silently
jumping to the wrong source.

Where the
[preview-session descriptors](preview_session_descriptors.md) packet materializes
the *static* per-session truth a surface presents right now, and the
[source-first preview / browser-runtime inspection matrix](freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix.md)
freezes the *qualification* of each claimed surface, this packet materializes the
*transition-time* truth: a before/after snapshot for each drift event and the
preserved-truth and recovery story that goes with it.

Source remains canonical; the drill is derivative — never a second writable
truth model. A drift never silently swaps the target the session was bound to, a
stale source map can no longer claim an exact mapping, a lost dev server or
expired browser session can no longer claim a live runtime, a data-posture flip
must actually change the governed data chip, and every degraded post-drift state
exports a precise degraded label and trigger that survives reopen and export.

## Source of truth

- Packet type: `PreviewDriftRecoveryDrillSet`
  (`crates/aureline-preview/src/preview_drift_recovery/`).
- Boundary schema:
  `schemas/preview/preview_drift_recovery_drill_set.schema.json`.
- Checked support export:
  `artifacts/preview/m5/preview_drift_recovery_drills/support_export.json`.
- Markdown summary:
  `artifacts/preview/m5/preview_drift_recovery_drills.md`.
- Protected fixtures:
  `fixtures/preview/m5/preview_drift_recovery_drills/`.
- Conformance dump: `cargo run -p aureline-preview --example dump_m5_preview_drift_recovery_drills [support|summary]`.

## Drift events

Each drill exercises exactly one closed `drift_event_class`, and every event in
the vocabulary is drilled at least once:

- `hot_reload_reset` — the module graph swapped and in-memory state was
  discarded while the runtime stayed reachable.
- `stale_source_map` — the source map that backed jumps drifted from the
  canonical source.
- `dev_server_lost` — the dev server backing a live preview disappeared.
- `device_reconnect` — a tethered device dropped and is re-attaching.
- `browser_session_expired` — the browser-runtime session expired.
- `runtime_replaced` — a different runtime took over behind the preview.
- `data_posture_flip` — the data posture flipped between live / mock / captured.

## Before / after truth snapshots

Each drill carries a `before` and an `after` `DriftTruthSnapshot`. Both reuse the
frozen vocabularies — `source_sync_class`, `data_posture`, `freshness_class`,
`target_kind`, `device_capability_class`, `source_mapping_quality`, and
`runtime_origin_class` — plus `runtime_backed` and `reconnect_required`. A
snapshot is internally honest: a `live` posture is backed by a live runtime that
can emit live events, a `captured` posture has no live runtime, a runtime-only
view is runtime-backed, a `drifted_from_source` view cannot claim an `exact`
source map, and an `in_sync_from_source` view cannot carry a `stale` map.

## Preserved-truth and event-honesty invariants

The drill enforces that the recovery preserves the truth the session was bound
to, and that the post-drift snapshot is honest for the specific failure:

- **Target preserved** — `after.target_kind == before.target_kind`. A drift
  never silently swaps the bound target.
- **Stale source map** — the post-drift `source_mapping_quality` is `stale` or
  `unavailable`; it cannot keep claiming `exact` or `heuristic`.
- **Dev server lost / browser session expired** — the post-drift snapshot is no
  longer `runtime_backed`; the live claim is dropped.
- **Hot-reload reset / device reconnect** — the runtime identity
  (`runtime_origin_class`) is preserved; the same runtime / device is still
  behind the view.
- **Runtime replaced** — the view does not carry the previous runtime's in-sync
  claim forward unchanged; it re-derives sync state honestly.
- **Data-posture flip** — the post-drift `data_posture` actually differs from the
  pre-drift posture.

## Degraded post-drift state

A post-drift snapshot is degraded when it is unidentified, drifted, past its
freshness SLO, mapping-degraded, or reconnect-required. A degraded drill records
a `downgrade_trigger` from the closed `DriftRecoveryTrigger` vocabulary and a
precise, non-generic `degraded_label`; a cleanly recovered drill carries neither.
`survives_reopen_export` asserts the degraded truth survives reopen and export
rather than resetting to a blank or "current" state. The set always demonstrates
both at least one degraded drill and at least one clean recovery.

## Recovery routes

Each drill declares at least one `recovery_route`, and every route must be
admissible for its drift event (the Rust `allowed_recovery_routes_for` map and
the schema enum stay in lockstep). For example, `reattach_device_session` is only
admissible for a device reconnect, and `remap_source_then_reload` is only
admissible for a stale source map.

## Guardrails

- Source remains canonical; the drill set is not a second writable truth model.
- A drift never silently swaps the bound target.
- Runtime state never hides source-mapping uncertainty across a drift.
- Inspect-only holds are never auto-upgraded into write-capable flows.
- Embedded preview / browser boundaries are not blurred into product authority.
- Degraded post-drift state exports precise truth that survives reopen.

## Downstream consumers

Product, docs/help, diagnostics, support export, and release-control surfaces
ingest these drills directly instead of re-narrating recovery behavior by hand,
and degraded post-drift state is visibly labeled below current in every surface.
