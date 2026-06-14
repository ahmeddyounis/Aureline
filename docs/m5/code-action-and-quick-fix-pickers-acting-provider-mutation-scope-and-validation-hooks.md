# Code-action and quick-fix pickers — acting provider, mutation scope, preview requirement, and validation hooks

Stable contract for the code-action and quick-fix pickers across the new
M5 artifact families: framework packs, notebook cells, docs artifacts,
request / structured artifacts, config artifacts, and generated source.

This document is the human-readable companion to the picker truth
packet. The canonical record is checked in at
`artifacts/language/m5/code_action_quick_fix_picker_truth_packet.json`
and validated by the boundary schema at
`schemas/language/code_action_quick_fix_picker_truth.schema.json`. The
packet is owned by `aureline-language`
(`crates/aureline-language/src/code_action_quick_fix_picker_truth_packet/`)
and regenerated from the real validator by the example
`cargo run -p aureline-language --example dump_code_action_quick_fix_picker_truth_packet`.

## Why this exists

A code action or quick fix in a new M5 artifact family only stays
trustworthy if the picker entry says which engine is acting, how far the
mutation reaches, whether a typed preview is required before apply, which
validation hook runs, and which manual or fallback path stays visible
when the acting provider is partial, stale, or low confidence. These
must be explicit per entry rather than hidden behind one generic "apply
fix" affordance. Aureline MUST NOT present LSP, framework analyzers,
graph lanes, notebook adapters, generated-source bridges, or AI overlays
as interchangeable, and MUST NOT let a one-click fix widen scope into
generated or structured artifacts without a preview.

Where the sibling provider/refactor matrix
(`docs/m5/freeze-the-language-provider-diagnostic-cluster-and-refactor-transaction-matrix.md`)
freezes which posture each artifact family *may* claim, and the
semantic-result arbitration packet
(`docs/m5/arbitration-inspectors-disagreement-detail-and-semantic-to-text-fallback-banners.md`)
keeps the *result* each surface anchors honest, this packet certifies the
*picker entry* the user actually invokes.

## What the packet asserts

The packet covers six artifact-family lanes: `framework_pack_lane`,
`notebook_cell_lane`, `docs_artifact_lane`, `request_artifact_lane`,
`config_artifact_lane`, and `generated_source_lane`. Each lane carries a
`picker_lane_quality` headline row that names its acting provider family,
exports a redaction-safe acting-provider label, and states the support
grade it claims, plus one admission row per picker dimension:

- **Acting provider** (`acting_provider_class`, on the headline row) —
  `lsp_provider`, `framework_analyzer`, `semantic_graph_lane`,
  `notebook_adapter`, `generated_source_bridge`, `ai_overlay`, or
  `text_fallback`. The headline row MUST name a concrete acting family
  and export an `acting_provider_label`; the family never reads as
  interchangeable.
- **Apply posture** (`apply_posture_admission`) — the central picker
  output: `inline_safe`, `preview_required`, `compare_only`, or
  `blocked_pending_review`. This row co-binds the **mutation scope**
  (`no_mutation`, `single_file_scope`, `multi_file_scope`,
  `cross_artifact_scope`, `generated_artifact_scope`,
  `structured_artifact_scope`, or `workspace_wide_scope`), the
  **validation hook** (`none_required`, `build_check`, `test_suite`,
  `type_check`, `lint_format`, `schema_validate`, `framework_check`, or
  `manual_review_only`), the typed `preview_completeness_class`, the
  exported `preview_hash_ref`, and the exported `checkpoint_ref`.
- **Generated-asset policy** (`generated_asset_policy_admission`) —
  `not_generated`, `regenerate_before_edit`,
  `edit_with_regeneration_replay`, `edit_blocked_generated_source`, or
  `compare_only_generated`.
- **Fallback / manual path** (`fallback_path_admission`) —
  `none_needed`, `manual_fix_guidance`, `repair_guidance_surfaced`,
  `regenerate_first_guidance`, `broaden_review_guidance`, or
  `disabled_no_fallback`.
- **Provider-disagreement visibility**
  (`provider_disagreement_admission`) —
  `single_provider_no_disagreement`, `winner_loser_both_inspectable`,
  `unresolved_surfaced`, or `policy_override_recorded`. A row that
  collapses the loser into `ranking_only_collapsed` is refused.
- **Rollback checkpoint route** (`rollback_checkpoint_admission`) —
  `exact_undo_via_local_history_checkpoint`,
  `compensating_revert_via_workspace_diff`,
  `grouped_mutation_journal_revert`, `regenerate_first_then_replay`,
  `manual_review_required_no_automatic_path`, or
  `no_safe_rollback_available`.

Every row binds a `support_class`, `evidence_class`, `known_limit_class`,
`downgrade_automation_class`, and `confidence_class`, carries
`evidence_refs`, and excludes raw source material, secrets, and ambient
authority. The packet is metadata-only: raw source bodies, refactor
diffs, generated artifact bodies, notebook cell outputs, provider
payloads, secrets, and ambient credentials never cross the boundary. The
`acting_provider_label`, `preview_hash_ref`, and `checkpoint_ref` fields
carry redaction-safe display strings and opaque ids only.

## Apply-posture safety invariants

The picker extends — it does not weaken — the launch-language refactor
safety model. The validator refuses, rather than silently publishing, any
of the following on an `apply_posture_admission` row:

- **Scope widening without preview**
  (`inline_apply_widens_scope_without_preview`) — an `inline_safe`
  posture whose mutation scope reaches `cross_artifact_scope`,
  `generated_artifact_scope`, `structured_artifact_scope`, or
  `workspace_wide_scope`. One-click fixes may apply inline only within a
  single or multi-file scope; widening into generated or structured
  artifacts requires `preview_required`.
- **Preview without a preview hash** (`missing_preview_hash_ref`) — a
  `preview_required` or `compare_only` posture that exports no preview
  hash.
- **Preview without a completeness label**
  (`missing_preview_completeness_label`) — a preview posture that carries
  no typed `preview_completeness_class`.
- **Mutating apply without a checkpoint** (`missing_checkpoint_ref`) — a
  posture that actually writes (`inline_safe` or `preview_required`) over
  a mutating scope but exports no rollback checkpoint ref.

Two further guardrails protect provider honesty:

- **Disagreement collapse** (`disagreement_collapsed_to_ranking_only`) —
  a `provider_disagreement_admission` that drops the losing provider into
  ranking-only output is refused; the winner, the loser, and the
  downgrade reason stay inspectable.
- **Hidden guidance on low confidence** (`manual_fix_guidance_hidden`) —
  a `fallback_path_admission` row that goes `low_confidence` may not
  present a `none_needed` or `disabled_no_fallback` fallback; the
  manual-fix or repair guidance stays visible.

## Crosswalk to the launch-language refactor transaction model

This packet **extends** — it does not redefine — the launch-language
refactor transaction contract
(`schemas/language/refactor_transaction_truth.schema.json`) and the
M5 provider/refactor matrix
(`artifacts/language/m5/provider_refactor_matrix_truth_packet.json`). It
reuses the matrix's closed provider-family, generated-artifact-policy,
rollback-path, preview-completeness, support, evidence, known-limit,
downgrade-automation, confidence, promotion-state, and consumer-surface
vocabularies verbatim instead of minting a local synonym set, and adds
only the apply-posture, mutation-scope, validation-hook, fallback-path,
and disagreement-visibility vocabulary the pickers need. The typed
preview, completeness labeling, and rollback checkpoints the matrix and
the M4 refactor model already require carry forward unchanged.

## Narrowing and downgrade automation

A lane carries a `certified` claim only when its headline row names a
concrete provider family and exports an acting-provider label, every
required picker dimension is enumerated, every binding is bound, every
mutating apply states a posture and exports the preview hash, completeness
label, and checkpoint ref its posture requires, narrowed rows carry their
disclosure refs, and all ten required consumer projections preserve the
packet verbatim. A row that loses any of those drops **below**
`certified` rather than silently inheriting an adjacent certified row.

### Disclosure anchors

- `#auto_block_on_missing_evidence` — the lane's headline row blocks the
  certified claim when its required evidence is missing.
- `#auto_narrow_on_missing_fixture` — an admission row narrows when its
  certified fixture is missing or stale.
- `#auto_narrow_on_provider_unavailable` — narrows when the acting
  provider becomes unavailable.
- `#auto_narrow_on_preview_partial` — narrows when a preview drops below
  complete coverage.
- `#auto_demote_on_low_confidence` — demotes when confidence drops below
  the certified bar, keeping manual-fix guidance visible.
- `#manual_only_pending_review` — holds the row for manual review until
  automation lands.

## Consumer surfaces

The packet is read verbatim by ten consumer surfaces:
`framework_pack_panel`, `notebook_surface`, `request_runner`,
`preview_surface`, `docs_surface`, `generated_artifact_surface`,
`support_export`, `release_proof_index`, `help_about`, and
`conformance_dashboard`. A projection that collapses any closed
vocabulary, reminted truth, or drops a required surface blocks the stable
claim. The `support_export`, `release_proof_index`, and `help_about`
surfaces re-export the packet through the support-export wrapper without
admitting private material, so Help/About, the release proof index, and
support bundles all read the same picker truth.

## Relationship to the canonical M5 evidence index

This row is a depth-lane proof governed by the canonical M5 evidence
index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).
Any marketed or support-class row that depends on the code-action /
quick-fix pickers narrows automatically when this packet's evidence is
missing, stale, or downgraded.
