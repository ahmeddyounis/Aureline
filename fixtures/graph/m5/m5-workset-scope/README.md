# Fixtures: M5 workset-scope descriptor packet

This directory contains fixture metadata for the `m5_workset_scope_packet`.

The canonical packet is checked in at:

`artifacts/graph/m5/m5-workset-scope.json`

and validated by the typed model in the `aureline-graph` crate (`m5_workset_scope`) and the
JSON Schema at `schemas/graph/m5-workset-scope.schema.json`.

## Coverage

- **One active snapshot.** The packet carries a single `active_snapshot` — a sparse, local,
  `named_workset` slice (`workset:auth-and-billing`) — that discloses a non-zero
  `hidden_result_count` and `not_loaded_count`, so the user can tell how much of the workspace
  remains out of scope. The snapshot id is the replay anchor every binding is stamped with.
- **Explicit and suggested scope-change actions.** The actions exercise both directions
  (`widen`, `narrow`) and both actuations (`explicit`, `suggested`). Every widen action and
  the suggestion set `requires_review: true`, so scope never broadens silently; the explicit
  narrow action does not require review.
- **One binding per consumer surface.** Each of `docs_recall`, `topology_view`,
  `architecture_explainer`, `review_explanation`, `onboarding_tour`, and `ai_context_assembly`
  carries exactly one binding, stamped with the active snapshot id and scope id, and none
  claims whole-workspace knowledge over the sparse slice.
- **Upstream provenance.** The packet binds to the canonical graph-depth governance matrix
  (`artifacts/graph/m5/m5-graph-governance.json`) and the scope-provenance truth packet
  (`artifacts/search/m4/scope_provenance_truth_packet.json`) it extends.

## Guardrails proven

- A widen action or suggestion that is not reviewable fails validation (`SilentBroadening`).
- A binding that claims whole-workspace knowledge over a sparse slice fails validation
  (`FullWorkspaceClaimOverSlice`).
- A binding not stamped with the active snapshot id or scope id fails validation
  (`SnapshotBindingMismatch`, `ScopeIdMismatch`).
- A missing or duplicated consumer-surface binding fails validation (`MissingSurfaceBinding`,
  `DuplicateSurfaceBinding`).
- A full-workspace scope that reports hidden or not-loaded results fails validation
  (`FullScopeHidesResults`).
