# M5 artifact lineage panels and result summary cards

The artifact lineage panel and the result summary card are two of the eight governed
experiment components frozen by the
[M5 experiment-component matrix](m5_experiment_component_matrix.md). This lane implements those
two families as two co-equal control vectors in one export-safe packet,
[`ArtifactLineagePanelResultSummaryCardControlsPacket`](../../crates/aureline-notebook/src/implement_artifact_lineage_panels_and_result_summary_cards_with_producing_run_identity_stale_diverged_notes_include_raw_toggles_and_export_boundary_truth_across_claimed_m5_experiment_surfaces/mod.rs),
so a claimed M5 notebook, experiment-dashboard, comparison, lineage, share-review, or CLI
surface can project a lineage panel and a summary card that keep experiment outputs
**attached to the run that produced them and honest about export scope before open, compare, or
share** — never anonymous, and never including raw payloads by default.

## What the resolvers decide

The module has two derived resolvers so the honesty of each component is computed, never
asserted.

### `resolve_artifact_lineage`

Given a lineage panel's lineage state, the resolver derives a **traceability class**:

- `lineage_complete` → `fully_traced`
- `regenerated` → `regenerated` (must carry an explicit regenerated note); also fully traced
- `lineage_partial` / `derived_upstream_known` → `partially_traced` (must carry an explicit
  partial-lineage note), not fully traced
- `lineage_broken` → `untraced` (must carry an explicit stale / diverged note), not fully traced
- `derived_upstream_unknown` → `untraced` (must carry an explicit unknown-upstream note), not
  fully traced

Every panel always names its producing run ID, generator step, environment / model fingerprint,
and saved scope, so a user can always tell **which run and lineage produced an artifact** and
how completely it is traced before trusting a compare or share; a broken, diverged, or
unknown-upstream artifact can never read as a fully-traced artifact, and no generated report,
chart, model, or dataset ever reads as an anonymous attachment.

### `resolve_summary_export`

Given a summary card's export scope, the resolver derives an **export disposition**:

- `summary_scope` / `metadata_scope` → `metadata_safe` (metadata-only)
- `evidence_scope` → `evidence_scoped`
- `raw_scope` → `raw_included` (must carry an explicit raw-inclusion warning; the include-raw
  toggle is on)
- `redacted_scope` → `redacted` (must carry an explicit redaction note)
- `export_withheld` → `withheld` (must carry an explicit withheld note)

The include-raw toggle is bound to the derived truth: a raw payload is included **only** when
the scope is raw, so a raw payload is **never included by default** and turning include-raw on is
always an explicit, warned choice. The summary-only, metadata-safe alternative stays visible
before any share.

## Artifact identity, lineage, and export safety

- **Artifact identity and lineage** — every lineage panel names its artifact / label, its
  artifact kind, its producing run ID, its generator step, its environment / model fingerprint,
  its saved scope, and its lineage state, so which run and lineage produced an artifact stays
  **always explicit**.
- **Open / trace / export** — every lineage panel offers the mandatory `open_artifact`,
  `trace_to_run`, and `export_lineage` actions (metadata-first lineage export), plus
  `open_deep_link`, `compare_lineage`, and `copy_artifact_id` as appropriate.
- **Headline, freshness, and export scope** — every summary card names its headline metrics, its
  artifact count, its freshness, its support / report scope, its include-raw toggle, its
  provenance note, and its explicit summary-versus-evidence-versus-raw handoff choice.
- **Review / share-summary-only** — every summary card offers the mandatory `review_export_scope`
  and `share_summary_only` actions, plus `include_raw_payload`, `export_evidence`,
  `open_deep_link`, and `copy_summary_id` as appropriate.
- **Stable deep links** — every next step names a stable `run_object`, `notebook_location`,
  `dataset_catalog_anchor`, or `docs_anchor` deep link with a resolvable reference. A component
  that offers a deep-link action must name a resolvable kind, so a next step is never an
  ephemeral overlay or hidden route.

## Hard invariants

Every component keeps five bools `false`, and validation flags any that is `true`:

- `masks_provenance_or_sensitivity_state` — provenance and sensitivity posture stay visible.
- `hides_producing_run_or_lineage_state` — the producing run and how the artifact is traced stay
  explicit.
- `exposes_raw_payload_by_default` — a raw payload is never included in an export by default.
- `implies_apples_to_apples_without_parity` — a comparison is never implied comparable without
  parity evidence.
- `invents_alternate_state_label` — no surface invents a second word for a governed lineage,
  traceability, content, or export state.

Panels and cards reuse Aureline's existing provenance, redaction, retention, and export-scope
vocabulary instead of inventing artifact-specific exceptions; cached, offline, and local-only
state stays visible.

## Coverage

The checked-in support export exercises every traceability class, every artifact kind class, and
every lineage state across the six seeded lineage panels, and every export disposition, every
summary content class, and every export scope across the six seeded summary cards.

## Source of truth and artifacts

- Boundary schema: [`schemas/ui/m5-artifact-lineage-panel-result-summary-card-controls.schema.json`](../../schemas/ui/m5-artifact-lineage-panel-result-summary-card-controls.schema.json)
- Support export: [`artifacts/release/m5-artifact-lineage-panel-result-summary-card-proof/support_export.json`](../../artifacts/release/m5-artifact-lineage-panel-result-summary-card-proof/support_export.json)
- Matrix CSV: [`artifacts/release/m5-artifact-lineage-panel-result-summary-card-proof/matrix.csv`](../../artifacts/release/m5-artifact-lineage-panel-result-summary-card-proof/matrix.csv)
- Design report: [`artifacts/design/m5-artifact-lineage-panel-result-summary-card.md`](../../artifacts/design/m5-artifact-lineage-panel-result-summary-card.md)
- Scenario fixtures: [`fixtures/ui/m5-artifact-lineage-panel-result-summary-card-controls/`](../../fixtures/ui/m5-artifact-lineage-panel-result-summary-card-controls/)

Regenerate every artifact and fixture from the single seed with the headless emitter:

```sh
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_artifact_lineage_panel_result_summary_card_primitive -- support-export
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_artifact_lineage_panel_result_summary_card_primitive -- csv
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_artifact_lineage_panel_result_summary_card_primitive -- report
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_artifact_lineage_panel_result_summary_card_primitive -- fixture-lineage-panel-broken
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_artifact_lineage_panel_result_summary_card_primitive -- fixture-summary-card-raw-payload
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_artifact_lineage_panel_result_summary_card_primitive -- validate
```
