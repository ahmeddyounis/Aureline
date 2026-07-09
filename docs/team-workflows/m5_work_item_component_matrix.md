# M5 work-item component matrix

Status: frozen (M05-980, batch B116)

This contract freezes Aureline's reusable **provider-backed work-item components** so
canonical identity, provider authority, local-versus-provider state, linked engineering
context, side-effect preview, and publish-later continuity stop drifting across issue,
task, incident, review, support, and CLI work-item consumers. It is the shared component
layer that sits on top of the already-claimed M5 work-item detail, provider link-state,
status-transition, evidence-link, and offline-handoff objects — it does **not**
re-architect the provider sync engines or tracker backends.

- Authoritative validator: `crates/aureline-provider`, module
  `freeze_the_m5_work_item_component_matrix`.
- Boundary schema: `schemas/ui/m5-work-item-component-matrix.schema.json`.
- Per-component canonical contracts (what downstream rows point to):
  `schemas/ui/m5-work-item-row.schema.json`,
  `schemas/ui/m5-provider-chip-group.schema.json`,
  `schemas/ui/m5-relation-strip.schema.json`,
  `schemas/ui/m5-sync-pending-pill.schema.json`,
  `schemas/ui/m5-work-item-detail-header.schema.json`,
  `schemas/ui/m5-status-transition-sheet.schema.json`,
  `schemas/ui/m5-related-evidence-card.schema.json`,
  `schemas/ui/m5-offline-handoff-packet-card.schema.json`.
- Checked proof: `artifacts/release/m5-work-item-component-proof/`
  (`support_export.json`, `matrix.csv`).
- Design report: `artifacts/design/m5-work-item-component-matrix.md`.
- Narrowed fixtures: `fixtures/ui/m5-work-item-components/`.
- Headless emitter:
  `cargo run -q -p aureline-provider --bin aureline_work_item_component_matrix -- <support-export|report|csv|validate|fixture-...>`.

## Component families

The matrix freezes eight reusable component families:

1. **work-item-row** — names its canonical kind, provider authority, and local state.
2. **provider-chip-group** — names provider authority (who owns the object).
3. **relation-strip** — names its linked engineering context (branch / PR / review / test
   / incident).
4. **sync-pending-pill** — names its local-versus-provider state.
5. **work-item-detail-header** — names its canonical kind and provider authority.
6. **status-transition-sheet** — previews its transition side effects before write.
7. **related-evidence-card** — names its evidence provenance.
8. **offline-handoff-packet-card** — names its handoff destination and metadata-safe export
   boundary.

## Controlled vocabularies

Consumers bind to **one** controlled vocabulary each for: work-item kind; provider
authority (`provider_owned` / `local_draft` / `mirrored_read_only` / `imported_snapshot` /
`unlinked_local` / `policy_pinned`); local-versus-provider state (`synced_with_provider` /
`local_only_draft` / `queued_for_publish` / `publish_deferred` / `publish_failed` /
`conflict_held`); relation kind; evidence kind; transition effect; handoff destination; and
export boundary. The frozen `vocabulary_set` is the single source of these tokens, enforced
by `VocabularySetDrift`.

Two families are shipped narrowed while parity proof lands, and both stay visible:
`status_transition_sheet` → **Beta**, `offline_handoff_packet_card` → **Preview**.

## Hard invariants

Every governed component row must satisfy these (each is a `const false` flag in the schema
and a `ComponentInvariantViolated` in the validator):

- `masks_identity_or_authority` — never mask the canonical identity or provider authority.
- `hides_local_or_publish_later_state` — never hide the local-versus-provider or
  publish-later state.
- `invents_alternate_state_label` — never invent an alternate label for a governed state.
- `uses_generic_ticket_wording` — never let generic ticket / task wording conceal provider
  ownership, queued state, or linked context.

## Acceptance criteria coverage

- The checked-in matrix enumerates row / header / sheet / card states, controlled labels,
  degraded (Beta / Preview) states, and export-safe fallbacks
  (`offline_handoff_packet_card` export boundaries) for each of the eight components.
- Downstream M5 team-workflow rows point to **one** canonical component family — the eight
  per-component schemas above — instead of restating work-item UI truth ad hoc, and share
  the frozen `vocabulary_set` for provider authority, sync state, linked context, and
  publish-later continuity.
