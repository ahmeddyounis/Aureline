# Provider-status strips, capability-negotiation drawers, and result-provenance pills

Stable contract for the three reusable code-understanding UI objects that
keep provider truth inspectable across the M5 framework, notebook,
generated-source, preview, docs-linked, and structured-artifact surfaces:

- a **provider-status strip** — which provider lane is active, where it
  runs, what lifecycle state it is in, and the route to inspect why a
  capability is partial or unavailable;
- a **capability-negotiation drawer** — the participating providers, the
  selected winner / fused result, scope limits, freshness, and retry /
  restart recovery actions; and
- a **result-provenance pill** — provenance attached to definitions,
  references, completions, rename previews, and framework-aware results,
  without forcing users into raw logs.

This document is the human-readable companion to the surface truth packet.
The canonical record is checked in at
`artifacts/language/m5/provider_status_surface_truth_packet.json` and
validated by the boundary schema at
`schemas/language/provider_status_surface_truth.schema.json`. The packet is
owned by `aureline-language`
(`crates/aureline-language/src/provider_status_surface_truth_packet/`).

## Why this exists

These surfaces only stay trustworthy if every result keeps its provider
identity, disagreement, completeness, acting engine, freshness, and
recovery posture explicit instead of being hidden behind one generic
semantic result. The three objects are the user-facing carriers of that
truth, and this packet is the single source that names, per surface, which
provider family acts, where it runs, what state it is in, which capability
was negotiated, how a provider disagreement is shown, which scope and
freshness apply, and which result a provenance pill is attached to.

## Reused vocabulary — not a local synonym set

The packet does **not** re-mint provider vocabulary. It reads the closed
vocabularies frozen by the provider/refactor matrix packet
(`crates/aureline-language/src/provider_refactor_matrix_truth_packet/`,
reviewer doc
`docs/m5/freeze-the-language-provider-diagnostic-cluster-and-refactor-transaction-matrix.md`)
for provider family, capability negotiation, conflict, result provenance,
preview completeness, and downgrade label, and reuses the shared support,
evidence, known-limit, downgrade-automation, confidence, promotion-state,
and consumer-surface vocabularies. The matrix artifact is listed in the
packet's `source_contract_refs`, so this packet is a real consumer of the
matrix, not a parallel copy. It adds only the UI-object vocabulary those
surfaces need on top: surface, object kind, provider locality, lifecycle
state, display label, capability-detail route, participant role,
selected-result form, scope limit, freshness, and recovery action.

## What the packet asserts

The packet covers six surfaces: `framework_surface`, `notebook_surface`,
`generated_source_surface`, `preview_surface`, `docs_linked_surface`, and
`structured_artifact_surface`. Every covered surface carries a
`surface_object_presence` row for each of the three object kinds —
`provider_status_strip`, `capability_negotiation_drawer`, and
`result_provenance_pill` — and each certified presence pulls in the
admission rows that object owns:

- **Provider-status strip** — a `provider_lane_state_admission` row binding
  where the provider runs (`provider_locality_class`) and its lifecycle
  state (`provider_lifecycle_state_class`), and a
  `capability_detail_route_admission` row binding the negotiated capability
  and an inspectable `capability_detail_route_class`.
- **Capability-negotiation drawer** — `participating_provider_admission`
  rows binding each participant's role and the conflict class, a
  `negotiation_result_admission` row binding the selected-result form, a
  `scope_and_freshness_admission` row binding scope limit and freshness,
  and a `drawer_recovery_action_admission` row binding the retry / restart
  recovery action.
- **Result-provenance pill** — a `provenance_anchor_admission` row binding
  the anchor target and result provenance, and a
  `provenance_downgrade_admission` row binding the allowed downgrade label.

## Invariants the validator enforces

The validator narrows a row below `certified` instead of inheriting an
adjacent certified row whenever:

- a label-bearing row uses a raw internal process name as the only
  user-facing provider label (`raw_process_name_only_label`) — a raw
  process name is never the only label;
- a capability-detail route resolves to an opaque loading spinner
  (`capability_detail_route_is_opaque_spinner`) — notebook, generated,
  workset, and sparse-scope limits are never hidden behind a generic
  spinner instead of an inspectable route;
- a drawer surfaces a provider disagreement but drops the losing provider
  (`losing_provider_not_preserved`) — disagreement is never collapsed to a
  ranking-only result; the losing provider and downgrade reason stay
  inspectable;
- a rename-preview pill bypasses a typed, complete preview
  (`preview_anchor_bypasses_typed_preview`) — this preserves, and never
  weakens, the launch-language refactor safety model, so AI-planned,
  organize-imports, schema/codegen, and notebook/generated edits cannot
  bypass preview, completeness labeling, or rollback checkpoints;
- a provenance pill forces users into raw logs (`provenance_requires_raw_logs`);
- a UI-object dimension is bound on a row class that does not own it
  (`*_not_permitted_on_row_class`), an object kind disagrees with its row
  class (`object_kind_row_class_mismatch`), a certified row leaves a binding
  unbound (`certified_with_unbound_binding`), a narrowed row drops its
  disclosure ref, or a row admits raw source bodies, secrets, or ambient
  authority past the boundary.

## Consumer projections

All ten required consumer surfaces — the framework-pack panel, notebook
surface, request runner, preview surface, docs surface, generated-artifact
surface, support export, release proof index, Help/About proof card, and
the conformance dashboard — must preserve the closed vocabulary verbatim. A
projection that collapses any dimension (for example the result-provenance
vocabulary that distinguishes live, cached, partial, text-heuristic,
imported, and stale results) is refused.

## Narrowing rule

Any marketed or support-class row that depends on these surface objects
narrows automatically when the packet's evidence is missing, stale, or
downgraded: a surface that loses a concrete provider family, an object-kind
presence, an admission row, an inspectable capability-detail route, a
preserved losing provider, a typed preview completeness, a disclosure ref,
or a consumer projection drops **below** `certified` instead of inheriting
an adjacent certified row.

## Executable proof

`crates/aureline-language/tests/provider_status_surface_truth_packet.rs`
loads every fixture and the checked-in packet, asserts the materialized
promotion state, finding counts, and closed token sets, and proves the
packet covers every required surface, every object kind on each surface, and
all ten consumer projections. The fixture corpus lives at
`fixtures/language/m5/provider_status_surface_truth_packet/` and is
regenerated deterministically by
`tools/regenerate_provider_status_surface_truth_packet.py`.
