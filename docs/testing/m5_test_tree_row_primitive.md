# M5 test-tree-row primitive

Status: implemented (B107, task M05-909)

This is the first `implement_` lane that narrows the frozen
[M5 test-explorer / watch / triage component matrix](./m5_test_explorer_watch_triage_component_matrix.md)
into one reusable primitive: the **test-tree row**. It closes the gap between the
deeper test discovery/session/quarantine object model and the reusable tree row a
user actually reads when they rerun, debug, or triage a failing test — so a user
can tell, from the row alone, exactly what will rerun or debug and with what
certainty, before any action leaves the tree.

Truth source (checked in):

- Schema: `schemas/ui/m5-test-tree-row.schema.json`
- Support export: `artifacts/release/m5-test-tree-row-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-test-tree-row-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-test-tree-row-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-test-tree-row-primitive/`

The single mint-from-truth path is the headless emitter
`aureline_runtime_test_tree_row_primitive`; the in-code seed builders, the checked
support export, and the fixtures never drift.

## What the primitive implements

The matrix names the test-tree row as one governed family and freezes its
controlled vocabulary (test identity classes, imported/live result origins,
result freshness, current verdicts, target classes, environment lanes, quarantine
ownership classes, release impacts, surface families, deployment lines, consumer
surfaces, accessibility routes, qualification classes, and downgrade triggers).
This lane implements that contract as one resolver so a user can tell, from the
tree row alone, which item class the row represents, its stable identity, its
current state and last-result freshness, its imported/live origin, its
target/environment shorthand, its parameterized-case count, and its
mute/quarantine state and release impact — and, above all, exactly what selection
will rerun or debug.

### `resolve_test_tree_row`

Takes one item's tree item class, identity class, result origin, result freshness,
current verdict, target class, environment lane, quarantine ownership and release
impact, parameterized-case count, mute flag, opaque item label, and opaque stable
item identity. Derives the **row posture** in a fixed honesty-first order:

1. `quarantined_row` — the item is muted/quarantined; its ownership and release
   impact head the row.
2. `partial_discovery_row` — a partial-discovery placeholder, ambiguous identity,
   or unattributed origin; what will rerun is not yet certain.
3. `imported_evidence_row` — the result is imported from an external run, not
   produced live here; reduced certainty.
4. `stale_result_row` — the live-local result is stale, outdated, or expired.
5. `suite_aggregate_row` — a suite or parameterized template that fans out.
6. `live_concrete_row` — a concrete, live-local, fresh case; the highest-certainty
   row and the only posture that presents full live certainty.

The **rerun scope** is derived directly from the item class so a rerun/debug never
silently widens beyond what the row names: suite → `whole_suite`, template →
`parameterized_group`, concrete case → `single_case`, notebook-backed →
`notebook_cells`, imported result → `imported_replay_only`, partial-discovery →
`nothing_concrete_yet`. The row always offers **reveal-item-identity** and
**export-row**, offers **rerun-item** only when the scope is locally rerunnable,
offers **debug-item** only for a concrete runnable leaf, and offers
**review-quarantine** only when the item is muted.

## Item-class coverage

The item classes (`suite`, `template`, `concrete_case`, `notebook_backed_item`,
`imported_result`, `partial_discovery_placeholder`) are exactly the distinction
the acceptance criteria require the row to make explicit. The seeded worked
resolutions exercise every item class, and the packet's
`item_class_coverage_unproven` lint fails if any is left unexercised.

## Acceptance criteria

- **A user can tell what will actually rerun or debug from the row alone.** The
  rerun scope is derived 1:1 from the item class and carried explicitly; the
  `rerun_coverage_unproven` lint enforces that both a locally-rerunnable row
  (offering `rerun_item`) and a not-locally-rerunnable row (imported-replay or
  nothing-yet, withholding `rerun_item`) are proven, and the
  `widens_rerun_scope_silently` invariant is `false` on every row.
- **Imported or partial-discovery items no longer inherit the same visual
  certainty as current live results.** Only `live_concrete_row` reports
  `shows_live_certainty = true`; imported and partial rows report
  `carries_reduced_certainty = true` and never `shows_live_certainty`. The
  `certainty_coverage_unproven` lint enforces that both a live-certainty and a
  reduced-certainty row are proven, and the `overstates_imported_certainty`
  invariant is `false` on every row.

## Consumer parity

The matrix binds the test-explorer tree, editor-gutter tree, run-panel tree,
headless/CLI tree, and test-report export to the same anatomy, item classes,
identity classes, result origins, row postures, rerun scopes, actions, export
fields, and accessibility routes, so the identity / origin / freshness /
rerun-scope / quarantine vocabulary stays identical across desktop,
headless/export, and report consumers.

## Governance and redaction

Four hard invariants hold on every row: it never masks its item identity class or
imported/live origin, never hides a quarantine's release impact, never renders
imported or partial-discovery items with live certainty, and never silently widens
the rerun scope. Raw log bodies, pasted paths, credentials, and private endpoints
never cross the export boundary; every item label and item identity is carried
only as an opaque, export-safe representation.
