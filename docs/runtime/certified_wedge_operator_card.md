# Certified wedge operator card — managed-workspace lifecycle, notebook trust, and install-review facts

This document publishes one **operator-card prototype** that makes three operator-truth lanes visible together on one representative wedge:

1) **Managed-workspace lifecycle** (including local-only continuation and offboarding posture),
2) **Notebook trust + structured round-trip risk**, and
3) **Install-review facts** (activation budget, compatibility label, dependency source, rollback/quarantine posture, and degraded/continue options).

The operator card is **not** a new source of truth. It is a **composite** that quotes existing canonical packets/contracts and shows how a reviewer can follow one end-to-end story from “entry” to “trust” to “install review” to “safe exit”.

If this document disagrees with any canonical source it quotes, the canonical source wins and this card MUST be updated in the same change.

## Companion artifacts

- [`/schemas/runtime/install_review_fact_grid.schema.json`](../../schemas/runtime/install_review_fact_grid.schema.json)
  — the fact-grid record used by install-review sheets and this operator-card prototype.
- [`/fixtures/runtime/operator_card_cases/`](../../fixtures/runtime/operator_card_cases/)
  — worked composite cases (local-only, provider-backed, widget trust risk, activation-budget exceeded, incompatible extension, provider-offline local continuation).

## Canonical sources this card must quote (no parallel vocabularies)

### Lifecycle (managed workspaces, offboarding, local-only continuation)

- [`/docs/managed/managed_workspace_lifecycle_contract.md`](../managed/managed_workspace_lifecycle_contract.md)
- [`/schemas/managed/workspace_lifecycle_state.schema.json`](../../schemas/managed/workspace_lifecycle_state.schema.json)
- [`/artifacts/runtime/managed_workspace_lifecycle.yaml`](../../artifacts/runtime/managed_workspace_lifecycle.yaml)

### Trust (notebooks + structured round-trip risk)

- [`/docs/notebooks/notebook_trust_and_roundtrip_preview_contract.md`](../notebooks/notebook_trust_and_roundtrip_preview_contract.md)
- [`/schemas/notebooks/roundtrip_preview.schema.json`](../../schemas/notebooks/roundtrip_preview.schema.json)
- [`/fixtures/notebooks/roundtrip_preview_cases/`](../../fixtures/notebooks/roundtrip_preview_cases/)

### Route + boundary cues (where a fact came from)

- [`/docs/runtime/target_discovery_and_install_review_taxonomy.md`](./target_discovery_and_install_review_taxonomy.md)
  — `host_boundary_cue_class`, `notebook_trust_rung`, `structured_round_trip_risk_class`, install-review summary slots, activation-budget summary shape.
- [`/docs/runtime/origin_target_route_taxonomy.md`](./origin_target_route_taxonomy.md)
  — `action_route_class`, `action_exposure_class`, authority-linkage vocabulary.

### Install review (compatibility, activation budget, rollback/quarantine, source truth)

- [`/docs/verification/install_review_packet.md`](../verification/install_review_packet.md)
- [`/fixtures/ecosystem/install_review_manifest.yaml`](../../fixtures/ecosystem/install_review_manifest.yaml)
- [`/artifacts/ecosystem/compatibility_label_audit.yaml`](../../artifacts/ecosystem/compatibility_label_audit.yaml)
- [`/artifacts/ecosystem/activation_budget_examples/`](../../artifacts/ecosystem/activation_budget_examples/)
- [`/docs/extensions/extension_lifecycle_and_quarantine_sequence.md`](../extensions/extension_lifecycle_and_quarantine_sequence.md)

### Degraded / “continue without it” vocabulary (reduced-mode options)

- [`/docs/trust/capability_sheet_contract.md`](../trust/capability_sheet_contract.md)
- [`/schemas/trust/capability_sheet.schema.json`](../../schemas/trust/capability_sheet.schema.json)

## Representative wedge this prototype targets

This operator card is written against the representative certified wedge:

- `launch_bundle:python_service_or_data_app.seed`
  (see `artifacts/product/language_bundle_rows.yaml`)

It intentionally includes a notebook-adjacent flow (open/review a notebook, and install a notebook-related component) because the goal is operator-truth cohesion: reviewers should not have to visit separate documents to see lifecycle, trust, and install-review consequences side-by-side.

## Operator card anatomy (composite; no new truth)

An operator card is reviewable if it can answer these questions in one place, using quoted canonical fields:

### 1) Entry: what target are we on, and what boundary did we cross?

Required projections:

- Host boundary cue stack (`host_boundary_cue_stack`) — from
  `docs/runtime/target_discovery_and_install_review_taxonomy.md`.
- Route/exposure tokens (`action_route_class`, `action_exposure_class`) — from
  `docs/runtime/origin_target_route_taxonomy.md`.

### 2) Lifecycle: what is the managed-workspace posture *right now*?

Required projections (when a managed workspace is in play):

- `lifecycle_phase_class`, `underlying_taxonomy_state_class`, `reachability_state_class`
  — from `schemas/managed/workspace_lifecycle_state.schema.json`.
- `continuation_posture.*` — explicitly names local-only continuation and which surfaces remain admissible.
- `offboarding_posture.*` — explicitly names export/offboarding posture.

When no managed workspace exists, the card MUST say that explicitly (no implied cloud dependency).

### 3) Notebook trust: what trust rung and round-trip risk apply?

Required projections (when a notebook or structured-round-trip flow is present):

- `notebook_trust_rung` — from `docs/runtime/target_discovery_and_install_review_taxonomy.md`
  (owned by `docs/notebooks/notebook_trust_and_roundtrip_preview_contract.md`).
- `structured_round_trip_risk_class` — from the same sources.
- At least one quoted `structured_round_trip_preview_sheet_record` ref for worked cases.

### 4) Install review: what are the “facts that matter” before activation?

Required projections:

- One `install_review_fact_grid_record` (schema:
  `schemas/runtime/install_review_fact_grid.schema.json`) rendered inline.
- Every row MUST carry:
  activation-budget class, compatibility label state, dependency source,
  rollback checkpoint + posture, quarantine posture, and a degraded/continue summary.

### 5) Safe exit: what remains usable, and how do we leave safely?

Required projections:

- If the provider is unavailable: local-only continuation posture MUST name what remains usable locally.
- If offboarding is in play: the offboarding posture MUST name what can be exported, when, and from where.

## Field provenance rule (binding)

Every field rendered on the operator card MUST be one of:

- a direct quote of a canonical field on a canonical record (e.g.
  `managed_workspace_lifecycle_state_record.continuation_posture.posture_class`), or
- a projection whose vocabulary is owned by a canonical contract listed above
  (for example: `compatibility_label_state`, `activation_budget_class`,
  `rollback_posture_class`, `quarantine_posture_class`,
  `degraded_capability_class`).

If a reviewer cannot trace a field back to one of those sources, the card is non-conforming.

## Worked cases

The worked cases live in:

- `fixtures/runtime/operator_card_cases/`

They are intentionally “composite” fixtures: each case quotes the managed-workspace lifecycle record (when applicable), the notebook preview/trust record (when applicable), and the fact-grid record so a reviewer can validate the end-to-end story without browsing multiple directories.

