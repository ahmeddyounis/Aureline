# Freeze the language-provider, diagnostic-cluster, and refactor-transaction matrix for framework, notebook, generated, and structured-artifact lanes

Stable contract for the provider, diagnostic-cluster, and
refactor-transaction matrix that governs the new code-understanding
artifact families: framework packs, notebook cells, generated source,
structured artifacts, and the code-understanding graph.

This document is the human-readable companion to the matrix truth
packet. The canonical record is checked in at
`artifacts/language/m5/provider_refactor_matrix_truth_packet.json` and
validated by the boundary schema at
`schemas/language/provider_refactor_matrix_truth.schema.json`. The
matrix is owned by `aureline-language`
(`crates/aureline-language/src/provider_refactor_matrix_truth_packet/`).

## Why this exists

The new artifact families only stay trustworthy if provider identity,
disagreement, completeness, acting engine, mutation scope, validation
plan, and rollback posture remain explicit instead of being hidden
behind one generic semantic result. Aureline MUST NOT present LSP,
framework analyzers, graph lanes, notebook adapters, generated-source
bridges, or AI overlays as interchangeable. The matrix is the single
source that names, per artifact family, which language-provider family
acts, what capability was negotiated, how provider disagreement is
arbitrated, where diagnostics come from, how fresh the result is, which
semantic posture the family may claim, which refactor classes it may
run, how complete a preview is, what policy governs generated assets,
which downgrade labels are allowed, and how a change rolls back.

## What the packet asserts

The packet covers five artifact-family lanes:
`framework_pack_lane`, `notebook_cell_lane`, `generated_source_lane`,
`structured_artifact_lane`, and `code_understanding_graph_lane`. Each
lane carries a `matrix_lane_quality` headline row that names its acting
provider family and the support grade it claims, plus one admission row
per matrix dimension:

- **Provider family** (`provider_family_class`) — `lsp_provider`,
  `framework_analyzer`, `semantic_graph_lane`, `notebook_adapter`,
  `generated_source_bridge`, `ai_overlay`, or `text_fallback`. The
  headline and semantic-mode rows MUST name a concrete acting family;
  the family never reads as interchangeable.
- **Capability negotiation** (`capability_negotiation_admission`) —
  `full_semantic_negotiated`, `partial_semantic_negotiated`,
  `text_fallback_negotiated`, `capability_declined`, or
  `negotiation_pending`.
- **Conflict arbitration** (`conflict_arbitration_admission`) —
  `single_provider_no_conflict`, `arbitrated_winner_loser_preserved`,
  `unresolved_disagreement_surfaced`, or `policy_override_recorded`. The
  losing provider and the downgrade reason stay inspectable; conflict is
  never collapsed into a ranking-only result.
- **Diagnostic source** (`diagnostic_source_admission`) —
  `compiler_build`, `lsp`, `linter_formatter`, `framework_schema`,
  `runtime_test_debug`, `policy_trust`,
  `generated_artifact_validation`, or `notebook_kernel`.
- **Result provenance** (`result_provenance_admission`) —
  `live_semantic`, `cached_semantic`, `partial_semantic`,
  `text_heuristic`, `imported_scan`, or `stale_pending_refresh`.
- **Semantic-layer mode** (`semantic_layer_mode_admission`) — the
  central matrix output: `semantic_rename`, `previewable_refactor`,
  `code_action_mutation`, `text_fallback`, `notebook_generated_bridge`,
  `compare_only`, or `unsupported`.
- **Refactor transaction** (`refactor_transaction_admission`) — binds a
  refactor class (`rename`, `extract`, `inline`, `move`,
  `organize_imports`, `schema_codegen_rewrite`, `ai_planned_transform`,
  `notebook_generated_edit`, or `compare_only_no_mutation`) together
  with its preview `completeness_class` and `rollback_path_class`. A
  mutating refactor that lacks a typed preview completeness or a safe
  rollback path is refused (`mutation_bypasses_preview_or_rollback`).
- **Generated-artifact policy** (`generated_artifact_policy_admission`)
  — `not_generated`, `regenerate_before_edit`,
  `edit_with_regeneration_replay`, `edit_blocked_generated_source`, or
  `compare_only_generated`.
- **Allowed downgrade label** (`downgrade_label_admission`) — `none`,
  `semantic_to_text_fallback`, `full_to_partial_completeness`,
  `previewable_to_compare_only`, `mutation_to_preview_only`,
  `provider_unavailable_text_only`, or
  `generated_edit_to_regenerate_first`.

Every row binds a `support_class`, `evidence_class`, `known_limit_class`,
`downgrade_automation_class`, and `confidence_class`, carries
`evidence_refs`, and excludes raw source material, secrets, and ambient
authority. The packet is metadata-only: raw source bodies, refactor
diffs, generated artifact bodies, notebook cell outputs, provider
payloads, secrets, and ambient credentials never cross the boundary.

## Crosswalk to the launch-language refactor transaction model

This matrix **extends** — it does not redefine — the launch-language
refactor transaction contract at
`docs/languages/m4/finalize-the-refactor-transaction-model-plus-preview-validate.md`
(`schemas/language/refactor_transaction_truth.schema.json`). The launch
refactor classes (`rename`, `extract`, `inline`, `move`,
`organize_imports`) and their preview/validate/apply/rollback discipline
carry forward unchanged. The matrix only adds framework, notebook,
generated-source, structured-artifact, and graph rows on top, plus the
generated-asset and AI-planned/`schema_codegen_rewrite`/
`notebook_generated_edit` refactor classes, none of which may bypass the
typed preview, completeness labeling, or rollback checkpoints the M4
model already requires. The M5 `certified` support grade is the superset
claim that includes the M4 `launch_stable` refactor lanes.

## Narrowing and downgrade automation

A lane carries a `certified` claim only when its headline row names a
concrete provider family, every required matrix dimension is enumerated,
every binding is bound, mutating refactors carry a typed completeness
and a safe rollback path, narrowed rows carry their disclosure refs, and
all ten required consumer projections preserve the packet verbatim. A
row that loses any of those drops **below** `certified` rather than
silently inheriting an adjacent certified row. The downgrade-automation
vocabulary names the trigger that narrows a row automatically.

### Disclosure anchors

- `#auto_block_on_missing_evidence` — the lane's headline row blocks the
  certified claim when its required evidence is missing.
- `#auto_narrow_on_missing_fixture` — an admission row narrows when its
  certified fixture is missing or stale.
- `#auto_narrow_on_provider_unavailable` — narrows when the acting
  provider family becomes unavailable.
- `#auto_narrow_on_conflict_unresolved` — narrows when provider conflict
  is left unresolved.
- `#auto_narrow_on_preview_partial` — narrows when a preview drops below
  complete coverage.
- `#auto_narrow_on_stale_provenance` — narrows when result provenance
  goes stale.
- `#auto_demote_on_low_confidence` — demotes when confidence drops below
  the certified bar.
- `#manual_only_pending_review` — holds the row for manual review until
  automation lands.

## Consumer surfaces

The matrix is read verbatim by ten consumer surfaces:
`framework_pack_panel`, `notebook_surface`, `request_runner`,
`preview_surface`, `docs_surface`, `generated_artifact_surface`,
`support_export`, `release_proof_index`, `help_about`, and
`conformance_dashboard`. A projection that collapses any closed
vocabulary, reminted truth, or drops a required surface blocks the
stable claim.

## Relationship to the canonical M5 evidence index

This row is a depth-lane proof governed by the canonical M5 evidence
index (`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).
Any marketed or support-class row that depends on the matrix narrows
automatically when this packet's evidence is missing, stale, or
downgraded.
