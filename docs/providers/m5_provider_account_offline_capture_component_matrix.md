# M5 provider-account / offline-capture component matrix

Status: frozen (M05-916, batch B108)

This contract freezes Aureline's reusable **provider-boundary settings and status
components** so account, mapping, sync, and offline-capture truth stop drifting across
issue, review, incident, support, provider-settings, and CLI provider consumers. It is
the shared component layer that sits on top of the already-claimed M5 connected-provider
registry, target mapping, sync-health, publish-later queue, and export-redaction objects
— it does **not** re-architect the connected-provider registry, sync engine, or a
broader PM suite.

- Authoritative validator: `crates/aureline-provider`, module
  `freeze_the_m5_provider_account_row_project_or_board_mapping_row_sync_behavior_row_offline_capture_row_and_privacy_redaction_row_component_matrix`.
- Boundary schema:
  `schemas/ui/m5-provider-account-offline-capture-component-matrix.schema.json`.
- Checked proof: `artifacts/release/m5-provider-account-offline-capture-proof/`
  (`support_export.json`, `matrix.csv`).
- Design report:
  `artifacts/design/m5-provider-account-offline-capture-component-matrix.md`.
- Narrowed fixtures: `fixtures/ui/m5-provider-account-offline-capture-components/`.
- Headless emitter:
  `cargo run -q -p aureline-provider --bin aureline_provider_account_offline_capture_component_matrix -- <support-export|report|csv|validate|fixture-...>`.

## Component families

The matrix freezes five reusable component families:

1. **provider-account-row** — names its provider identity class, account connection
   state, and tenant scope.
2. **project-or-board-mapping-row** — names its mapping origin and default-destination
   target kind.
3. **sync-behavior-row** — names its sync mode, effective write scope, and queued-draft
   state.
4. **offline-capture-row** — names its offline-capture state and queued-draft state.
5. **privacy-redaction-row** — names its redaction class and metadata-safe export
   boundary.

## Controlled vocabularies

Consumers bind to **one** controlled vocabulary each for: provider identity class,
account connection state (`not_configured` / `signed_in` / `limited_scope` /
`stale_session` / `offline_cached_read` / `policy_blocked`), tenant scope, mapping origin,
mapping target kind, sync mode, effective write scope, offline-capture state, queued-draft
state (shared by the sync-behavior and offline-capture rows), redaction class, and export
boundary. The frozen `vocabulary_set` is the single source of these tokens.

The account connection state vocabulary is the exact acceptance-criteria set: no claimed
M5 provider surface invents an alternate label for `not configured`, `signed in`,
`limited scope`, `stale session`, `offline cached read`, or `policy blocked`, nor for
mapping origin, sync mode, offline-capture state, or metadata-safe export boundary.

## Hard invariants

Every governed component row must satisfy these (each is a `const false` flag in the
schema and a `ComponentInvariantViolated` in the validator):

- `masks_connection_or_scope` — never mask the connection state or the tenant / effective
  write scope.
- `hides_export_or_redaction_boundary` — never hide the redaction class or the export
  boundary that support and export flows will disclose.
- `invents_alternate_state_label` — never invent an alternate label for a governed state.
- `assumes_default_destination_silently` — never assume a default publish destination
  without disclosing its mapping origin.

## Acceptance criteria coverage

- **Single controlled vocabulary** for `not_configured`, `signed_in`, `limited_scope`,
  `stale_session`, `offline_cached_read`, and `policy_blocked` — frozen in
  `vocabulary_set.account_connection_states` and enforced by `VocabularySetDrift`.
- **No alternate labels** for mapping origin, sync mode, offline-capture state, or
  metadata-safe export boundaries — enforced by the four hard invariants above, the
  `no_surface_invents_alternate_state_label` and
  `mapping_sync_offline_export_named_once` governance flags, and the
  `AlternateStateLabelInvented` / `MappingOriginUnstated` / `SyncModeUnstated` /
  `OfflineCaptureStateUnstated` / `ExportBoundaryHidden` / `DefaultDestinationAssumed`
  downgrade triggers.
