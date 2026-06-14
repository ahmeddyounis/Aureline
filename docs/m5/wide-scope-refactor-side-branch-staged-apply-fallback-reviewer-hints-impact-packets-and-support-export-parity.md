# Wide-scope refactor fallback — side-branch / staged-apply postures, reviewer hints, impact packets, and support/export parity

Stable contract for the safe fallback posture that a wide-scope or
low-confidence transform takes instead of an apply-all on the live
workspace, across the new M5 artifact families: framework packs, notebook
cells, docs artifacts, request / structured artifacts, config artifacts,
and generated source.

This document is the human-readable companion to the wide-scope refactor
fallback truth packet. The canonical record is checked in at
`artifacts/language/m5/wide_scope_refactor_fallback_truth_packet.json` and
validated by the boundary schema at
`schemas/language/wide_scope_refactor_fallback_truth.schema.json`. The
packet is owned by `aureline-language`
(`crates/aureline-language/src/wide_scope_refactor_fallback_truth_packet/`)
and regenerated from the real validator by the example
`cargo run -p aureline-language --example dump_wide_scope_refactor_fallback_truth_packet`.

## Why this exists

A wide-scope refactor that spans many files, a monorepo slice, unloaded
scope, generated edges, notebook mappings, or a multi-root workspace — or
any transform whose confidence or completeness fell below the frozen
threshold — must not default to an apply-all on the live workspace. The
trustworthy behavior is to *fall back*: apply onto a side branch or an
isolated worktree, apply in reviewed stages, or stay compare-only, with a
clear rationale, a rollback path, reviewer / owner hints, an impact packet,
and support / export that preserve the refactor lineage and the
missing-scope explanation.

Where the sibling typed refactor transaction packet
(`docs/m5/typed-refactor-transactions-completeness-labels-generated-artifact-policy-validation-plans-and-rollback-checkpoints.md`)
certifies that a transform is a typed transaction, and the provider /
refactor matrix
(`docs/m5/freeze-the-language-provider-diagnostic-cluster-and-refactor-transaction-matrix.md`)
freezes which posture each artifact family *may* claim, this packet
certifies *how a low-confidence or wide-scope transaction reaches source*.

## What the packet asserts

The packet covers six artifact-family lanes: `framework_pack_lane`,
`notebook_cell_lane`, `docs_artifact_lane`, `request_artifact_lane`,
`config_artifact_lane`, and `generated_source_lane`. Each lane carries a
`fallback_lane_quality` headline row that names its acting engine family,
exports a redaction-safe engine-identity label, binds the refactor class,
carries the transform's `refactor_id`, and states the support grade it
claims, plus one admission row per fallback dimension:

- **Apply posture** (`apply_posture_admission`) — `side_branch_apply`,
  `worktree_apply`, `staged_apply`, `compare_only_review`,
  `apply_all_on_live_workspace`, or `blocked_pending_review`, co-bound with
  the target scope (`single_file_scope`, `multi_file_scope`,
  `cross_artifact_scope`, `generated_artifact_scope`,
  `structured_artifact_scope`, or `workspace_wide_scope`), the typed
  `scope_completeness_class` (`complete`, `partial`, `blocked`, or
  `unsupported`), the `confidence_class` (`high_confidence`,
  `medium_confidence`, or `low_confidence`), and the `missing_scope_count`
  (the size of the missing-scope set).
- **Impact packet** (`impact_packet_admission`) — the
  `impacted_target_count` and `impacted_owner_count`, plus whether an impact
  summary (`impact_summary_present`) and a missing-scope explanation
  (`missing_scope_explanation_present`) are attached, plus the exported
  `impact_packet_ref`.
- **Reviewer hint** (`reviewer_hint_admission`) — `codeowners_reviewer`,
  `recent_author_reviewer`, `owning_team_reviewer`,
  `manual_assignment_required`, or `no_reviewer_required`, plus whether an
  owner hint (`owner_hint_present`) is attached and the exported
  `review_anchor_ref`.
- **Rollback path** (`rollback_path_admission`) —
  `exact_undo_via_local_history_checkpoint`,
  `compensating_revert_via_workspace_diff`,
  `grouped_mutation_journal_revert`, `regenerate_first_then_replay`,
  `manual_review_required_no_automatic_path`, or
  `no_safe_rollback_available`, plus the exported `checkpoint_ref` on
  automatic routes.
- **Support-export parity** (`support_export_parity_admission`) — whether
  the support / export channel preserves the refactor lineage
  (`preserves_refactor_lineage`) and the missing-scope explanation
  (`preserves_missing_scope_explanation`), plus the exported `lineage_ref`.
- **Provider-disagreement visibility**
  (`provider_disagreement_admission`) — `single_provider_no_disagreement`,
  `winner_loser_both_inspectable`, `unresolved_surfaced`, or
  `policy_override_recorded`. A row that collapses the loser into
  `ranking_only_collapsed` is refused.

Every row binds a `support_class`, `evidence_class`, `known_limit_class`,
`downgrade_automation_class`, and `confidence_class`, carries
`evidence_refs`, and excludes raw source material, secrets, and ambient
authority. The packet is metadata-only: raw source bodies, refactor diffs,
generated artifact bodies, notebook cell outputs, provider payloads,
secrets, and ambient credentials never cross the boundary. The
`engine_identity_label`, `impact_packet_ref`, `review_anchor_ref`,
`checkpoint_ref`, and `lineage_ref` fields carry redaction-safe display
strings and opaque ids only.

## Fallback safety invariants

The fallback posture extends — it does not weaken — the launch-language
refactor safety model. The validator refuses, rather than silently
publishing, any of the following:

- **Unsafe apply-all below the frozen threshold**
  (`unsafe_apply_all_below_threshold`) — an `apply_posture_admission` that
  offers `apply_all_on_live_workspace` while confidence is below
  `high_confidence`, completeness is below `complete`, or the scope is wide
  (anything beyond `single_file_scope` / `no_mutation`). Apply-all on the
  live workspace is only ever permitted for a narrow, complete,
  high-confidence transform; wide-scope and low-confidence transforms
  default to a side-branch, worktree, staged-apply, or compare-only
  fallback.
- **Completeness overclaim** (`scope_completeness_overclaimed`) — an
  `apply_posture_admission` that labels the preview `complete` while
  `missing_scope_count` is greater than zero.
- **Empty or unexplained impact packet** (`empty_impact_packet`,
  `missing_impact_summary`, `missing_impact_packet_ref`,
  `impact_packet_drops_missing_scope`) — an `impact_packet_admission` that
  documents no impacted targets, or documents impacted targets without an
  impact summary or packet ref, or — when the lane left targets out of
  scope — drops the missing-scope explanation.
- **Reviewer / owner hint dropped** (`missing_review_anchor_ref`,
  `missing_owner_hint`) — a `reviewer_hint_admission` that routes to a
  reviewer but exports no review anchor or attaches no owner hint.
- **Writing fallback without a safe rollback**
  (`writing_fallback_without_safe_rollback`, `missing_checkpoint_ref`) — a
  lane whose apply posture writes source (side-branch, worktree, staged, or
  apply-all) while its rollback route offers no safe recovery
  (`no_safe_rollback_available`), or a `rollback_path_admission` that claims
  an automatic checkpoint route but exports no checkpoint ref.
- **Support / export drops lineage or missing-scope**
  (`support_export_drops_lineage`, `support_export_drops_missing_scope`,
  `missing_lineage_ref`) — a `support_export_parity_admission` that drops
  the refactor lineage or the missing-scope explanation, or exports no
  lineage ref.
- **Disagreement collapse** (`disagreement_collapsed_to_ranking_only`) — a
  `provider_disagreement_admission` that drops the losing engine into
  ranking-only output is refused; the winner, the loser, and the downgrade
  reason stay inspectable.

## Crosswalk to the launch-language refactor safety model

This packet **generalizes** — it does not redefine — the launch-language
refactor transaction contract and the M5 typed refactor transaction
packet. It reuses the matrix's closed provider-family, refactor-class,
mutation-scope, completeness, rollback-path, support, evidence, known-limit,
downgrade-automation, confidence, promotion-state, and consumer-surface
vocabularies and the picker's disagreement-visibility vocabulary verbatim
instead of minting a local synonym set, and adds only the apply-posture and
reviewer-hint vocabulary the fallback flows need. The typed preview,
completeness labeling, grouped mutation journal, and rollback checkpoints
the typed transaction packet already requires carry forward unchanged onto
the fallback flows.

## Narrowing and downgrade automation

A lane carries a `certified` claim only when its headline row names a
concrete engine, exports an engine-identity label, binds a refactor class,
every required fallback dimension is enumerated, every binding is bound,
apply-all is offered only under the frozen narrow / complete /
high-confidence threshold, the impact packet preserves the missing-scope
explanation, reviewer hints carry their review anchor and owner hint, the
writing fallback carries a safe rollback path with its checkpoint ref,
support / export preserves the refactor lineage and missing-scope
explanation, narrowed rows carry their disclosure refs, and all ten
required consumer projections preserve the packet verbatim. A row that
loses any of those drops **below** `certified` rather than silently
inheriting an adjacent certified row.

### Disclosure anchors

- `#auto_block_on_missing_evidence` — the lane's headline row blocks the
  certified claim when its required evidence is missing.
- `#auto_narrow_on_missing_fixture` — an admission row narrows when its
  certified fixture is missing or stale.
- `#auto_narrow_on_provider_unavailable` — narrows when the acting engine
  becomes unavailable.
- `#auto_narrow_on_preview_partial` — narrows when a preview drops below
  complete coverage.
- `#auto_demote_on_low_confidence` — demotes when the confidence tier drops
  below the certified bar.
- `#manual_only_pending_review` — holds the row for manual review until
  automation lands.

## Consumer surfaces

The packet is read verbatim by ten consumer surfaces:
`framework_pack_panel`, `notebook_surface`, `request_runner`,
`preview_surface`, `docs_surface`, `generated_artifact_surface`,
`support_export`, `release_proof_index`, `help_about`, and
`conformance_dashboard`. A projection that collapses any closed vocabulary,
reminted truth, or drops a required surface blocks the stable claim. The
`support_export`, `release_proof_index`, and `help_about` surfaces
re-export the packet through the support-export wrapper without admitting
private material, so Help/About, the release proof index, and support
bundles all read the same fallback truth — including the refactor lineage
and missing-scope explanation the support-export parity row guarantees.

## Relationship to the canonical M5 evidence index

This row is a depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).
Any marketed or support-class row that depends on the wide-scope refactor
fallback posture narrows automatically when this packet's evidence is
missing, stale, or downgraded.
