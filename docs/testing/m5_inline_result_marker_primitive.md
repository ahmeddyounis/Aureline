# M5 inline-result-marker primitive

Status: implemented (B107, task M05-910)

This is the second `implement_` lane that narrows the frozen
[M5 test-explorer / watch / triage component matrix](./m5_test_explorer_watch_triage_component_matrix.md)
into one reusable primitive: the **inline result marker**. It closes the gap
between the deeper test session/attempt object model and the reusable source
decoration a user actually reads in an editor gutter or notebook cell — so a
decoration never implies a current local result when the evidence came from an
imported or stale run, or maps only approximately to the current file/cell state.
It is the marker sibling of the
[M5 test-tree-row primitive](./m5_test_tree_row_primitive.md).

Truth source (checked in):

- Schema: `schemas/ui/m5-inline-result-marker.schema.json`
- Support export: `artifacts/release/m5-inline-result-marker-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-inline-result-marker-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-inline-result-marker-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-inline-result-marker-primitive/`

The single mint-from-truth path is the headless emitter
`aureline_runtime_inline_result_marker_primitive`; the in-code seed builders, the
checked support export, and the fixtures never drift.

## What the primitive implements

The matrix names the inline result marker as one governed family and freezes its
controlled vocabulary (marker verdicts, imported/live result origins, result
freshness, failure categories, target classes, environment lanes, attempt lineage
kinds, quarantine ownership classes, release impacts, surface families, deployment
lines, consumer surfaces, accessibility routes, qualification classes, and
downgrade triggers). This lane implements that contract as one resolver so a user
can tell, from the inline marker alone, its pass/fail/error/timeout state, its
stability-or-flaky chip, its target/environment shorthand, its imported/live
class, its last-result freshness, how faithfully it maps to the current file/cell
state, and the recent-attempt lineage behind an open-recent-attempts action — and,
above all, whether the decoration may honestly read as a current live-local result
at all.

### `resolve_inline_result_marker`

Takes one marker's verdict, optional failure category, stability chip, result
origin, result freshness, source-mapping fidelity, target class, environment lane,
attempt lineage, quarantine ownership and release impact, recent-attempt count,
mute flag, opaque marker label, and opaque stable marker identity. Derives the
**marker posture** in a fixed honesty-first order:

1. `quarantined_marker` — the test is muted/quarantined; its ownership and release
   impact head the marker.
2. `unmapped_marker` — the marker no longer maps to any location in the current
   buffer.
3. `approximate_mapping_marker` — the marker maps only approximately; the source
   drifted since the result was produced.
4. `imported_evidence_marker` — the result is imported from an external run, not
   produced live here; reduced certainty.
5. `stale_result_marker` — the live-local result is stale, outdated, or expired.
6. `live_local_marker` — a fresh, live-local, exactly-mapped result; the only
   posture that may honestly imply a current local result.

The marker may be **rerun from the marker** only from a live-local result whose
source still maps to the buffer, **opens recent attempts** only when some exist,
always offers **reveal-marker-evidence** and **export-marker**, and offers
**review-quarantine** only when the test is muted.

## Source-mapping coverage

The source mappings (`exact_mapping`, `approximate_mapping`, `unmapped_to_buffer`,
`no_local_buffer`) are exactly the fidelity distinction the implementation
requirements need for a marker to degrade visibly when it maps only approximately
to the current file/cell state. The seeded worked resolutions exercise every
mapping, and the packet's `mapping_coverage_unproven` lint fails if any is left
unexercised.

## Acceptance criteria

- **Source decorations no longer imply a current local result when the evidence
  came from an imported or stale run.** Only `live_local_marker` reports
  `shows_live_certainty = true` and `implies_current_local_result = true`;
  imported, stale, approximate-mapping, and unmapped markers report
  `carries_reduced_certainty = true` and never `implies_current_local_result`. The
  `certainty_coverage_unproven` lint enforces that both a live-certainty and a
  reduced-certainty marker are proven, and the
  `overstates_imported_or_stale_as_live` invariant is `false` on every row.
- **Editor/notebook inline markers keep parity with the test tree and triage
  consumers on state labels and attempt lineage.** The verdict, failure category,
  and attempt lineage kind are reused verbatim from the frozen matrix; the
  `drops_attempt_lineage` invariant is `false` on every row; and the open-recent-
  attempts action is present exactly when there is attempt lineage to open, proven
  by the `recent_attempts_coverage_unproven` lint.

## Consumer parity

The matrix binds the editor-gutter marker, editor inline marker, notebook-cell
marker, headless/CLI marker, and marker-report export to the same anatomy,
verdicts, stability chips, result origins, freshness states, source mappings,
marker postures, attempt lineage kinds, actions, export fields, and accessibility
routes, so the state / origin / freshness / mapping / attempt-lineage vocabulary
stays identical across editor, notebook, headless/export, and report consumers.

## Governance and redaction

Four hard invariants hold on every row: it never masks its verdict or imported/live
origin, never hides a quarantine's release impact, never renders an imported,
stale, or approximately-mapped run as a current live-local result, and never drops
the attempt lineage a tree or triage consumer would show. Raw log bodies, pasted
paths, credentials, and private endpoints never cross the export boundary; every
marker label and marker identity is carried only as an opaque, export-safe
representation.
