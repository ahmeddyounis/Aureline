# Freeze the M5 structured-artifact review component matrix

Status: Frozen (M05-964, batch B114)

This contract freezes the reusable component vocabulary and state model for
structured-artifact review before more M5 compare, merge, and artifact-review
surfaces fork locally. It is the single canonical source of truth for the nine
reusable components users trust while comparing, merging, reviewing, or handing
off structured or media-like artifacts (notebooks, manifests, lockfiles, SBOMs,
source maps, crash adjuncts, coverage/profile captures, and design snapshots).

- Boundary schema: [`schemas/ui/m5-structured-artifact-review-component-matrix.schema.json`](../../../schemas/ui/m5-structured-artifact-review-component-matrix.schema.json)
- Producer: `aureline_review::current_stable_m5_artifact_component_matrix_export`
- Release proof: [`artifacts/release/m5-structured-artifact-review-proof/`](../../../artifacts/release/m5-structured-artifact-review-proof/)
- Design matrix: [`artifacts/design/m5-structured-artifact-review-component-matrix.md`](../../../artifacts/design/m5-structured-artifact-review-component-matrix.md)
- Protected fixtures: [`fixtures/ui/m5-structured-artifact-review-components/`](../../../fixtures/ui/m5-structured-artifact-review-components/)

## Components

| Component | Maturity | Canonical source contract |
| --- | --- | --- |
| `artifact_identity_bar` | stable | `schemas/ui/m5-artifact-provenance-bundle-card.schema.json` |
| `diff_mode_switcher` | stable | `schemas/notebook/ship_cell_aware_diff_metadata_filters_output_include_or_exclude_state_and_raw_json_fallback.schema.json` |
| `structure_row` | stable | `schemas/preview/inspect_to_source_tree_mapping.schema.json` |
| `merge_decision_row` | stable | `schemas/notebook/implement_notebook_merge_flows_base_or_ours_or_theirs_lineage_and_conflict_review_sheets.schema.json` |
| `generated_artifact_notice` | stable | `schemas/generated/generated-artifact-descriptor.schema.json` |
| `rendered_compare_viewer` | beta | `schemas/review/implement-normalized-pipeline-run-rows-log-viewers-artifact-browsers-and-safe-preview-trust-classes.schema.json` |
| `media_metadata_rail` | preview | `schemas/design-system/m5-design-system-contract-matrix.schema.json` |
| `redaction_or_trust_badge_set` | stable | `schemas/ui/m5-provider-offline-capture-privacy-redaction-row.schema.json` |
| `compare_summary_card` | stable | `schemas/ui/m5-manifest-diff-card.schema.json` |

Two components are deliberately narrowed below Stable: `rendered_compare_viewer`
(Beta — render trust across media/design-snapshot rendering is still qualifying)
and `media_metadata_rail` (Preview — media metadata extraction coverage is still
qualifying). Every component still lists its canonical source contract, and the
validator rejects any row whose `source_contract_refs` omits its component's
canonical contract (`component_source_contract_mismatch`).

## Per-row truth (the delta this matrix enforces)

Each row carries five structured-artifact honesty fields, each with its own
validation failure so a surface cannot silently drop one:

- `canonical_source_disclosure` — artifact class and canonical source of truth
  (`canonical_source_disclosure_missing`).
- `fidelity_narrowing_vocab` — the render/schema fidelity vocabulary the
  component must preserve (`fidelity_narrowing_vocab_missing`). Values:
  `structured_faithful`, `structured_partial`, `schema_unrecognized`,
  `render_untrusted`, `raw_fallback`, `redacted_or_withheld`.
- `compare_write_back_safety` — whether the artifact is compare-only or writable
  (`compare_write_back_safety_missing`).
- `render_trust_disclosure` — the render-trust posture
  (`render_trust_disclosure_missing`).
- `generated_from_relation` — the generated-from / source-of-truth relation
  (`generated_from_relation_missing`).

## Guardrails enforced

- Compare-only artifacts are never silently promoted to writable state.
- Structured modes are never flattened into raw fallback without explanation; a
  raw/export-safe fallback is always labeled.
- Generated-from / source-of-truth relations are never hidden behind generic
  file chrome.
- Render, schema, and merge fidelity narrowing is always explicit.
- Raw artifact bodies, raw render payloads, raw media bytes, credentials, and
  live provider responses stay outside the support boundary.

## Acceptance criteria coverage

- Design, release, help, and support packets reference this one canonical
  structured-artifact component matrix instead of local widget descriptions.
- New M5 compare and merge work points to these governed components rather than
  inventing parallel artifact-review chrome, binding each component to the
  notebook, generated-artifact, lockfile/manifest, SBOM, source-map,
  crash-artifact, and design-snapshot corpora already used by M5 review lanes.
