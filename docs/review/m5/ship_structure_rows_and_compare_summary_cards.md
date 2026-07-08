# Ship structure rows and compare summary cards

Status: Implemented (M05-966, batch B114)

This contract narrows the `structure_row` and `compare_summary_card` components
frozen in
[`m5-structured-artifact-review-component-matrix`](freeze_the_m5_structured_artifact_review_component_matrix.md)
(M05-964) into implemented, export-safe review controls. It lets a reviewer scan
a structured diff and understand *which object changed* before opening raw text,
and surfaces the *scale and risk* of a large artifact diff without ever hiding
the ability to inspect the underlying raw content.

- Boundary schema: [`schemas/ui/m5-structure-compare-summary-controls.schema.json`](../../../schemas/ui/m5-structure-compare-summary-controls.schema.json)
- Producer: `aureline_review::current_structure_compare_controls_export`
- Release proof: [`artifacts/release/m5-structure-compare-summary-controls-proof/`](../../../artifacts/release/m5-structure-compare-summary-controls-proof/)
- Protected fixtures: [`fixtures/ui/m5-structure-compare-summary-controls/`](../../../fixtures/ui/m5-structure-compare-summary-controls/)

## What the components carry

Every `StructureRow` reuses the frozen `M5ArtifactComponent` tag and answers,
from the row alone:

- **Object identity** (`object_path`, required and non-empty) — the structured
  path or key-path that changed.
- **Object category** (`object_category`: `structured_object` / `package_delta` /
  `metadata_field` / `redacted_field`) — package-centric deltas, plain structured
  objects, metadata-only fields, and redacted fields never blur together.
- **Change kind** (`change_kind`: `added` / `removed` / `modified` /
  `metadata_only` / `redacted_hidden`) — add / remove / modify state stays
  distinct, and metadata-only and redacted-hidden are their own explicit states.
- **Old / new summaries** (`old_summary`, `new_summary`), carried per the change
  kind's disclosure.
- **Confidence or schema note** (`confidence_or_schema_note`, required) and the
  reused `schema_fidelity` (`M5ArtifactFidelityState`).
- **Raw-context jump action** (`raw_context_action`, required) so raw content is
  always reachable.

Every `CompareSummaryCard` rolls up the compare result without flattening it:

- **Changed-object counts** (`change_counts`) broken out by change kind, whose
  `total_changed_objects` must equal the sum of the per-kind counts.
- **Scale flag** (`large_diff`) and **risk markers** (`risk_markers`), each a
  `CompareRiskMarker` with a `RiskSeverity` and a required explanation.
- **Compare-only-versus-write-back safety** (`compare_write_back_safety`) and a
  **raw-context jump action** (`raw_context_action`).

## Derived honesty (the delta this lane enforces)

The old / new summary requirement is *derived* from the change kind by
`resolve_structure_row_disclosure`:

- `added` carries only a new summary; `removed` carries only an old summary.
- `modified` and `metadata_only` carry both.
- `redacted_hidden` keeps its content hidden — both summaries must be empty and a
  `redaction_note` is required, so a redacted field is shown as *hidden* rather
  than silently dropped, and content is never leaked into a summary.

A row's `object_category` and `change_kind` must agree about redaction: a
`redacted_field` carries a `redacted_hidden` kind and vice-versa
(`category_change_kind_inconsistent`).

For compare summary cards, `resolve_compare_summary_disclosure` requires:

- a `large_change_volume` marker whenever the producer flags `large_diff`
  (`scale_risk_marker_missing`), and
- a `redacted_content_present` marker whenever any object is redacted-hidden
  (`redacted_risk_marker_missing`),

while the raw-context jump action is *always* required. This is how a large
artifact diff surfaces scale and risk while keeping the raw content one action
away.

Structure rows and compare summary cards are paired by artifact reference: the
set of `artifact_ref`s across the rows must equal the set across the cards
(`compare_pairing_incomplete`), so per-object detail is never shown without its
roll-up scale, and scale is never shown divorced from the per-object detail.

## Coverage and invariants

- The structure rows must cover the `added`, `removed`, and `modified` change
  kinds (`structure_change_kind_coverage_missing`).
- `total_changed_objects` must equal the per-kind sum
  (`change_counts_inconsistent`); a card that rolls up zero changes is rejected
  (`empty_compare_summary`).
- Every risk marker carries an explanation (`risk_marker_note_missing`).
- The trust-review and consumer-projection blocks assert that object identity,
  distinct change kinds, preserved summaries, always-reachable raw context,
  surfaced scale, explained risk, un-leaked redacted content, and compare-only
  safety all hold.

Raw artifact bodies, raw diffs, credentials, and live provider responses never
cross this boundary; the export is metadata-only and screened by an
export-material heuristic (`raw_boundary_material_in_export`).
