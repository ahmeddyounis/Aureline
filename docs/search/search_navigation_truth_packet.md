# Search/navigation truth packet, bookmark-history audit, and query-export redaction contract

This contract freezes one shared vocabulary for the search/navigation
truth packet. It exists so milestone close, release readiness, parity
audits, support handoffs, and public-proof review can state, in one
inspectable record, exactly which relevance corpora the search and
navigation surfaces audited, which result-kind / readiness / fallback
states were exercised, what hidden-result counts and scope/workset
caveats narrow the claim, what bookmark-, navigation-history-, and
deep-link-continuity caveats are open, and what redaction posture the
query and result export pipelines guarantee — without scattering the
same information across screenshots, ad-hoc demos, support-bundle
appendices, claim-manifest narrowings, and release-notes paragraphs.

The search/navigation truth packet is a single object family. Every
packet projects one window (milestone close, release train, weekly
governance review, ad-hoc review, support handoff, parity audit) and
renders eight typed sections:

1. The **schema-version pin block** — pinned integer versions of
   `schemas/search/search_result_truth.schema.json`,
   `schemas/search/query_session.schema.json`,
   `schemas/search/saved_query_bundle.schema.json`, and
   `schemas/navigation/navigation_artifacts.schema.json` so a reviewer
   can detect schema drift between the packet and its sources.
2. The **relevance corpus summary** — the corpus-coverage classes the
   packet exercises, the worked-example fixture refs that back the
   claim, and an optional pin into the labelled corpus at
   `artifacts/search/result_truth_labels.yaml`.
3. The **result-kind coverage matrix** — one entry per
   `surface_class` covered (`command_palette`, `full_search`,
   `symbol_jump`, `docs_search`, `graph_overlay`,
   `ai_explanation_overlay`, `support_export`) plus the
   `readiness_state`, `result_truth_class`, `semantic_fallback_state`,
   and `scope_filter_class` values that surface exercised.
4. The **hierarchy fallback matrix** — typed list of the fallback
   layers the packet exercised (`lexical_only_baseline`,
   `structural_fallback_engaged`, `semantic_supplement_engaged`,
   `semantic_disabled_by_policy_engaged`,
   `semantic_unavailable_lexical_only_engaged`,
   `imported_pack_fallback_engaged`, `graph_authority_confirmed`,
   `graph_authority_unconfirmed_heuristic_only`).
5. The **hidden-result aggregate** — total hidden count across the
   packet's covered rows, an `hidden_by_policy_count` subtotal, an
   `hidden_by_scope_count` subtotal, and a histogram of
   `hidden_scope_reason` keyed by count, all preserved through every
   audience-specific redaction.
6. The **scope/workset caveats** — typed list naming named-workset,
   sparse-slice, policy-limited-view, remote-workspace partial-load,
   cross-repo result-group (outside-current-scope / imported-root /
   policy-overlay), proposed-but-not-committed scope widening,
   trust-or-policy-blocked widening, and warm-vs-cold workset drift
   so search-related claims can be narrowed by scope evidence.
7. The **bookmark / navigation-history continuity audit** — typed
   list naming bookmark drift / remap / missing-target /
   scope-unavailable / archived-tombstone presence, navigation-
   history `session_restore_replay` and cross-surface back/forward
   exercise, and deep-link `target_renamed` / `target_moved` /
   `target_branch_drifted` / scope-widened-on-resolve / cross-repo-
   jump / unresolvable presence so restore continuity, renamed-or-
   moved-targets, and scope-widening-or-cross-repo-jump caveats
   stay visible.
8. The **query/result export redaction posture** — typed list
   asserting `raw_query_text_never_crosses_boundary`,
   `saved_query_literal_local_only_by_default`, hashed-export and
   explicit-opt-in disclosure rules, shared-link reopen-with-current-
   viewer-permissions, and the per-export floor that hidden-by-
   policy counts, hidden-scope reasons, partial-truth causes, semantic
   fallback state, provider-attribution chip, and the
   captured-results-not-current-truth claim travel through every
   export pipeline.

The packet also carries the typed window kind, the typed audience and
redaction profile (paired by schema gates), the typed linked-artifact-
families block (the search-readiness vocabulary, query-session
contract, saved-query bundle, navigation-and-saved-query contract,
and cross-repo result-group schema must always be cited), the typed
change-significance summary (informational, release-bearing,
claim-narrowing, claim-widening-blocked), the typed consuming-surface
parity floor, the typed `policy_context`, the typed
`running_build_identity_ref`, and the typed evaluated/minted/frozen/
superseded/withdrawn timestamps. Raw query bodies, raw document
bodies, raw symbol definitions, raw notebook cell bodies, raw URLs,
raw absolute paths, raw provider payloads, and raw secrets MUST NOT
appear; the record carries opaque refs, typed vocabulary, and bounded
reviewable summaries only.

The contract is pre-implementation. It defines the reusable record
shape, the closed vocabularies, the projection rules, the export-
parity floor, the change-significance rules, and the fixture corpus.
It does not implement ranking, indexing, navigation, or search engines.

## Companion artifacts

- [`/schemas/search/search_truth_packet.schema.json`](../../schemas/search/search_truth_packet.schema.json)
  — boundary schema for one `search_navigation_truth_packet_record`
  plus the audit-event family.
- [`/fixtures/search/search_truth_cases/`](../../fixtures/search/search_truth_cases/)
  — worked records covering an informational milestone-close packet,
  a release-bearing packet that asserts parity across every surface
  class, a claim-narrowing packet that pulls a public claim back
  because semantic scoring is degraded under policy, and a
  claim-widening-blocked packet held by a remote-workspace partial
  load plus an unresolvable deep-link cross-repo jump.
- [`./search_readiness_vocabulary.md`](./search_readiness_vocabulary.md)
  and
  [`/schemas/search/search_result_truth.schema.json`](../../schemas/search/search_result_truth.schema.json)
  — search readiness, ranking-reason, hidden-scope, partial-truth-
  cause, semantic-fallback, scope-filter, freshness, and deep-link
  drift vocabularies. The packet re-exports the closed enums it needs
  (surface class, readiness state, result-truth class, scope-filter
  class, hidden-scope reason, partial-truth cause, semantic-fallback
  state, deep-link drift state) without redeclaring them.
- [`./search_query_session_contract.md`](./search_query_session_contract.md)
  and
  [`/schemas/search/query_session.schema.json`](../../schemas/search/query_session.schema.json)
  — the canonical query-session, result-identity, and explanation-
  capture contract. The packet pins the integer
  `query_session_schema_version` it audited.
- [`./search_explainability_contract.md`](./search_explainability_contract.md)
  and
  [`/schemas/search/search_explainability_panel.schema.json`](../../schemas/search/search_explainability_panel.schema.json)
  — same-surface explainability panel for ranking reasons, source
  mix, hidden counts, and policy-blocked counts. The packet's
  result-kind coverage matrix may cite explainability panel rows as
  evidence.
- [`./query_planner_contract_seed.md`](./query_planner_contract_seed.md)
  and
  [`/schemas/search/result_fusion_record.schema.json`](../../schemas/search/result_fusion_record.schema.json)
  — planner-pass, shard-snapshot, lane-grouping, and result-fusion
  vocabulary. Snapshot-export-field-floor entries cite planner-pass
  refs through this family.
- [`/docs/navigation/navigation_and_saved_query_contract.md`](../navigation/navigation_and_saved_query_contract.md),
  [`/schemas/navigation/navigation_artifacts.schema.json`](../../schemas/navigation/navigation_artifacts.schema.json),
  and
  [`/schemas/search/saved_query_bundle.schema.json`](../../schemas/search/saved_query_bundle.schema.json)
  — navigation-continuity, bookmark-drift, saved-query / deep-link,
  and search-collection-snapshot contract. The packet re-exports the
  closed `bookmark_drift_state`, `navigation_history_origin_class`,
  `saved_query_source_class`,
  `saved_query_sensitive_literal_handling_class`,
  `snapshot_export_field_class`, and `reopen_honesty_state` vocabularies
  it needs, and pins the integer
  `navigation_artifacts_schema_version` and
  `saved_query_bundle_schema_version`.
- [`/docs/workspace/workset_scope_and_cross_repo_contract.md`](../workspace/workset_scope_and_cross_repo_contract.md),
  [`/docs/workspace/scope_widening_and_cross_repo_jump_contract.md`](../workspace/scope_widening_and_cross_repo_jump_contract.md),
  [`/schemas/workspace/workset_artifact.schema.json`](../../schemas/workspace/workset_artifact.schema.json),
  [`/schemas/workspace/cross_repo_result_group.schema.json`](../../schemas/workspace/cross_repo_result_group.schema.json),
  and
  [`/schemas/workspace/scope_diff_review.schema.json`](../../schemas/workspace/scope_diff_review.schema.json)
  — named-workset / sparse-slice / cross-repo result-group / scope-
  widening-review artifacts. The packet's scope/workset caveats cite
  workset, scope-binding, cross-repo result-group, and scope-diff-
  review row ids verbatim.
- [`/schemas/docs/citation_anchor.schema.json`](../../schemas/docs/citation_anchor.schema.json)
  — docs citation-anchor object model. Imported-pack and docs-search
  evidence in the packet cites citation anchors through this schema
  rather than re-deriving them.
- [`/artifacts/search/result_truth_labels.yaml`](../../artifacts/search/result_truth_labels.yaml)
  — labelled truth-class corpus. The relevance corpus summary may
  pin into this corpus through `result_truth_label_corpus_ref`.

This contract **composes with and does not replace** vocabularies
already frozen in:

- ADR 0014 (search readiness, ranking-reason, hidden-scope, partial-
  truth-cause, semantic-fallback, scope-filter, freshness, deep-link
  drift, redaction-class, client-scope vocabularies).
- ADR 0011 (`freshness_class`, `client_scope`, `redaction_class`).
- ADR 0007 (broker-owned redaction pass).
- ADR 0001 / ADR 0018 (workspace-trust state).
- ADR 0009 (execution-context, scope-filter binding).

If this document disagrees with those sources, those sources win and
this document plus the schema are updated in the same change.

The eventual search / navigation crate's Rust types are the schema of
record. The boundary schema at
`schemas/search/search_truth_packet.schema.json` is the cross-tool
boundary every non-owning surface reads.

## Why freeze this now

Without one frozen packet, every release / milestone / parity audit /
support handoff is free to invent its own way of summarising search
and navigation truth. Each divergence widens a different axis silently:

1. *A release-readiness reviewer reads "search relevance is good"
   from a screenshot.* The reviewer cannot tell which corpora that
   claim was derived from, which scope was active, what was hidden
   under policy, or whether the snapshot is current versus captured.
2. *A support engineer asks "did the user see the policy-blocked
   count?".* Without one packet that pins the hidden-by-policy
   subtotal, the engineer has to reconstruct it from logs that may
   already be redacted.
3. *A claim-manifest reviewer reads "deep links work" without
   knowing which `deep_link_drift_state` cases the packet
   actually exercised.* The claim is unverifiable.
4. *A parity audit between the desktop product and the CLI is
   reduced to "they returned the same rows".* Without the result-
   kind coverage matrix and the consuming-surface parity floor,
   the audit cannot tell whether the surfaces agreed on
   `result_truth_class`, hidden counts, or
   `semantic_fallback_state`.
5. *A public-proof packet exports a saved query body because the
   redaction pipeline did not preserve the `local_only` posture.*
   Sensitive literals leak.
6. *A bookmark restored across a renamed file silently rebinds.*
   The audit lost continuity because no record paired the
   bookmark-drift state to the navigation-history origin and the
   deep-link drift state.

The freeze matters now, ahead of the live ranking / indexing /
navigation / saved-query / support-export pipelines, so every later
surface can read **the same** packet shape, **the same** relevance-
corpus / hidden-result / scope-caveat / continuity / export-redaction
vocabulary, and **the same** audience-redaction pairing rules instead
of inventing per-window equivalents.

## Frozen vocabulary

This contract introduces the following frozen vocabularies. Each is
owned by `schemas/search/search_truth_packet.schema.json`; downstream
surfaces re-export by reference and never mint a parallel value.

### Window kinds (frozen, six values)

`window_kind_class`: `milestone_close_window`,
`release_train_window`, `weekly_governance_review_window`,
`ad_hoc_review_window`, `support_handoff_window`,
`parity_audit_window`. A packet whose window cannot be typed denies
with `window_kind_class_unresolved` rather than collapsing to
`ad_hoc_review_window`.

### Audience and redaction profile (frozen, paired)

`audience_class` (five values): `engineering_internal`,
`support_handoff`, `enterprise_audit`, `release_readiness`,
`public_proof_safe`.

`redaction_profile_class` (five values):
`engineering_internal_only`, `support_handoff_redacted`,
`enterprise_audit_redacted`, `release_readiness_summary`,
`public_proof_safe_zero_payload`.

The schema gates pair every audience to exactly one redaction
profile. A mismatched pairing denies with
`redaction_profile_audience_pairing_mismatch`. The pairing rule:

| `audience_class`        | `redaction_profile_class`            |
|-------------------------|--------------------------------------|
| `engineering_internal`  | `engineering_internal_only`          |
| `support_handoff`       | `support_handoff_redacted`           |
| `enterprise_audit`      | `enterprise_audit_redacted`          |
| `release_readiness`     | `release_readiness_summary`          |
| `public_proof_safe`     | `public_proof_safe_zero_payload`     |

### Corpus coverage (frozen, twelve values)

`corpus_coverage_class`: `covers_palette_command_corpus`,
`covers_full_search_lexical_corpus`,
`covers_full_search_semantic_supplement_corpus`,
`covers_symbol_jump_structural_fallback_corpus`,
`covers_docs_pack_corpus`, `covers_graph_overlay_corpus`,
`covers_ai_explanation_overlay_corpus`,
`covers_support_export_quoted_corpus`,
`covers_provider_overlay_corpus`, `covers_imported_pack_corpus`,
`covers_partial_index_progression_corpus`,
`covers_stale_index_recovery_corpus`. A packet that covers no
corpus denies with `relevance_corpus_summary_empty`.

### Hierarchy fallback matrix (frozen, eight values)

`hierarchy_fallback_class`: `lexical_only_baseline`,
`structural_fallback_engaged`, `semantic_supplement_engaged`,
`semantic_disabled_by_policy_engaged`,
`semantic_unavailable_lexical_only_engaged`,
`imported_pack_fallback_engaged`, `graph_authority_confirmed`,
`graph_authority_unconfirmed_heuristic_only`. A packet that
exercised no fallback layer denies with
`hierarchy_fallback_matrix_empty`.

### Scope/workset caveats (frozen, ten values)

`scope_caveat_class`: `named_workset_in_use`,
`sparse_slice_in_use`, `policy_limited_view_in_use`,
`remote_workspace_partial_load`,
`cross_repo_group_outside_current_scope_present`,
`cross_repo_group_imported_root_present`,
`cross_repo_group_policy_overlay_present`,
`scope_widening_proposed_but_not_committed`,
`scope_widening_blocked_by_trust_or_policy`,
`workset_warm_versus_cold_drift_present`. Each entry MAY cite a
`workset_id_ref`, `scope_binding_id_ref`,
`cross_repo_result_group_id_ref`, and `scope_diff_review_id_ref`
into the workspace contracts.

### Bookmark / navigation-history continuity caveats (frozen, thirteen values)

`bookmark_history_continuity_caveat_class`:
`bookmark_drift_present`, `bookmark_remapped_present`,
`bookmark_missing_target_present`,
`bookmark_scope_unavailable_present`,
`bookmark_archived_tombstone_present`,
`navigation_history_session_restore_replay_present`,
`navigation_history_cross_surface_back_forward_exercised`,
`deep_link_target_renamed_present`,
`deep_link_target_moved_present`,
`deep_link_target_branch_drifted_present`,
`deep_link_scope_widened_on_resolve_present`,
`deep_link_cross_repo_jump_present`,
`deep_link_unresolvable_present`. Each entry MAY pin the cited
bookmark / navigation-history entry / deep-link binding by opaque
ref and MAY record the observed drift state in
`observed_bookmark_drift_state`, `observed_history_origin_class`,
or `observed_deep_link_drift_state`.

### Export redaction posture (frozen, fourteen values)

`export_redaction_posture_class`:
`raw_query_text_never_crosses_boundary`,
`saved_query_literal_local_only_by_default`,
`saved_query_literal_hashed_with_local_salt_for_export`,
`saved_query_literal_disclosed_with_explicit_user_opt_in`,
`shared_link_reopens_with_current_viewer_permissions`,
`shared_link_does_not_widen_authority_beyond_viewer`,
`exported_packet_preserves_hidden_by_policy_count`,
`exported_packet_preserves_hidden_scope_reasons`,
`exported_packet_preserves_partial_truth_causes`,
`exported_packet_preserves_semantic_fallback_state`,
`exported_packet_preserves_provider_attribution_chip`,
`exported_packet_states_captured_results_not_current_truth`,
`support_export_redacts_raw_paths_and_urls`,
`public_proof_export_carries_zero_raw_payload`.

The schema enforces a five-class export-redaction floor: every
packet MUST carry at minimum
`raw_query_text_never_crosses_boundary`,
`saved_query_literal_local_only_by_default`,
`exported_packet_preserves_hidden_by_policy_count`,
`exported_packet_preserves_partial_truth_causes`, and
`shared_link_reopens_with_current_viewer_permissions`. Missing any
of those denies with `export_redaction_posture_minimum_floor_unmet`.

The packet ALSO pins a `snapshot_export_field_floor` (drawn from
`snapshot_export_field_class` at
`schemas/search/saved_query_bundle.schema.json`) requiring at
minimum `snapshot_includes_planner_pass_ref` plus
`snapshot_includes_hidden_result_count_disclosure`.

### Linked artifact families (frozen, fourteen values)

`linked_artifact_family_class`: `search_readiness_vocabulary`,
`search_query_session_contract`, `search_explainability_contract`,
`query_planner_contract_seed`, `saved_query_bundle_contract`,
`saved_query_and_scope_binding_registry`,
`navigation_and_saved_query_contract`,
`navigation_artifacts_schema`, `scope_truth_packet`,
`workset_scope_and_cross_repo_contract`,
`scope_widening_and_cross_repo_jump_contract`,
`cross_repo_result_group_schema`, `citation_anchor_object_model`,
`result_truth_label_corpus`.

The schema enforces a five-class linked-artifact floor: every
packet MUST cite at minimum `search_readiness_vocabulary`,
`search_query_session_contract`, `saved_query_bundle_contract`,
`navigation_and_saved_query_contract`, and
`cross_repo_result_group_schema`. Missing any denies with
`linked_artifact_family_minimum_floor_unmet`.

### Change significance (frozen, four values)

`change_significance_class`: `informational`,
`release_bearing`, `claim_narrowing`, `claim_widening_blocked`.

`claim_narrowing` packets MUST cite
`claim_manifest_row_id_refs`. `claim_widening_blocked` packets
MUST cite at least one `blocking_caveat_classes` entry drawn from
either `scope_caveat_class` or
`bookmark_history_continuity_caveat_class`.

### Consuming-surface parity floor (frozen, eight values)

`consuming_surface_parity_floor_class`:
`command_palette_parity_required`,
`full_search_parity_required`, `symbol_jump_parity_required`,
`docs_search_parity_required`, `graph_overlay_parity_required`,
`ai_explanation_overlay_parity_required`,
`support_export_parity_required`, `cli_search_parity_required`. A
surface that diverges from the floor is non-conforming and denies
with `consuming_surface_parity_floor_unmet`.

### Denial reasons (frozen)

`search_truth_packet_unknown`, `audience_class_unresolved`,
`redaction_profile_class_unresolved`,
`redaction_profile_audience_pairing_mismatch`,
`window_kind_class_unresolved`,
`relevance_corpus_summary_empty`, `result_kind_coverage_empty`,
`result_kind_coverage_surface_unresolved`,
`hierarchy_fallback_matrix_empty`,
`hidden_result_aggregate_unresolved`,
`hidden_result_count_negative`,
`hidden_result_reason_missing_for_non_zero_count`,
`scope_caveat_class_unresolved`,
`bookmark_history_continuity_caveat_unresolved`,
`export_redaction_posture_minimum_floor_unmet`,
`linked_artifact_family_minimum_floor_unmet`,
`change_significance_class_unresolved`,
`claim_narrowing_requires_claim_manifest_row_refs`,
`claim_widening_blocked_requires_blocking_caveat`,
`consuming_surface_parity_floor_unmet`,
`query_session_schema_version_pin_missing`,
`search_result_schema_version_pin_missing`,
`saved_query_bundle_schema_version_pin_missing`,
`navigation_artifacts_schema_version_pin_missing`,
`raw_body_forbidden_on_boundary`,
`search_truth_packet_schema_version_lagging`,
`policy_epoch_expired`, `policy_blocked`.

### Audit event ids (frozen, seven values)

`search_truth_packet_audit_event_id`:
`search_truth_packet_minted`, `search_truth_packet_updated`,
`search_truth_packet_frozen_for_handoff`,
`search_truth_packet_superseded`, `search_truth_packet_withdrawn`,
`search_truth_packet_audit_denial_emitted`,
`search_truth_packet_schema_version_bumped`.

## Truthfulness posture (normative)

Every rule below is normative. A packet that violates any of them is
non-conforming regardless of how the violation is painted.

1. **One packet, one window, one audience-redaction pairing.** A
   packet pins exactly one `window_kind_class`, one `audience_class`,
   and exactly one paired `redaction_profile_class`. A mismatched
   pairing denies with `redaction_profile_audience_pairing_mismatch`.
2. **Schema-version pins are mandatory.** Every packet carries the
   four schema-version pins above. A packet missing any pin denies
   with the matching `*_schema_version_pin_missing` reason.
3. **Relevance corpus summary cannot be empty.** Every packet pins
   at least one `corpus_coverage_class` and at least one fixture ref.
   An empty summary denies with `relevance_corpus_summary_empty`.
4. **Result-kind coverage is per-surface and explicit.** Every
   covered `surface_class` lists the exercised `readiness_state`,
   `result_truth_class`, and `semantic_fallback_state` values.
   `scope_filter_class` is optional but recommended. Empty coverage
   denies with `result_kind_coverage_empty`.
5. **Hierarchy fallback matrix is mandatory.** Every packet declares
   at least one fallback layer. Empty denies with
   `hierarchy_fallback_matrix_empty`.
6. **Hidden-result aggregate is mandatory and honest.** Every packet
   carries `total_hidden_result_count` (>= 0), a `count_is_approximate`
   flag, an `hidden_by_policy_count` subtotal, and a non-empty
   `hidden_scope_reason_histogram` whenever the total is non-zero.
   A non-zero total without a histogram entry denies with
   `hidden_result_reason_missing_for_non_zero_count`.
7. **Export redaction posture floor is enforced.** Every packet carries
   at minimum the five export-redaction floor classes plus the
   two-class snapshot-export-field floor. Missing any denies with
   `export_redaction_posture_minimum_floor_unmet`.
8. **Linked artifact floor is enforced.** Every packet cites the
   five linked-artifact-family floor classes. Missing any denies
   with `linked_artifact_family_minimum_floor_unmet`.
9. **Change significance pairs to its evidence.** A
   `claim_narrowing` packet cites at least one
   `claim_manifest_row_id_refs` entry. A `claim_widening_blocked`
   packet cites at least one `blocking_caveat_classes` entry.
10. **Frozen / superseded / withdrawn packets MUST set
    `frozen_at`.** A packet that asserts a `superseded_at` or
    `withdrawn_at` timestamp without `frozen_at` denies via the
    schema gates.
11. **No raw payloads cross the boundary.** Raw query bodies, raw
    document bodies, raw symbol definitions, raw notebook cell
    bodies, raw URLs, raw absolute paths, raw provider payloads, and
    raw secrets MUST NOT appear. A row that carries a raw payload
    denies with `raw_body_forbidden_on_boundary`.
12. **Captured results are replay material, not current truth.**
    The packet inherits the saved-query bundle / collection-snapshot
    rule that captured rows are replay material, never frozen-truth
    claims about current results. The export-redaction posture
    `exported_packet_states_captured_results_not_current_truth`
    travels through every export pipeline.

## Audience-redaction pairing (normative)

The audience-redaction pairing table above is the only admitted
pairing. A reviewer reading a packet whose audience is
`public_proof_safe` MUST see `redaction_profile_class =
public_proof_safe_zero_payload` and MUST NOT see
`engineering_internal_only`. The pairing is enforced by the schema
gates; an exporter MAY NOT bypass the pairing by minting an
out-of-band field.

A packet narrowed for an audience whose pairing the source packet
does not admit MUST be re-minted under a new packet id (the
`supersedes_packet_id_ref` field carries the chain). The original
packet is not edited in-place to change audience.

## Hidden-result aggregate rules (normative)

The hidden-result aggregate is the export-safe view of the per-row
counts on the covered `search_result_packet_record` rows. It exists
because every audience reads the aggregate, but only some audiences
read the per-row breakdown.

- `total_hidden_result_count` is the sum of `hidden_result_count`
  across the covered packets. It is `0` when no rows were hidden.
- `count_is_approximate` is `true` whenever any contributing row had
  `count_is_approximate = true`. The aggregate MUST NOT silently
  round an approximate aggregate to an exact one.
- `hidden_by_policy_count` is the subtotal of rows whose
  `hidden_scope_reason` is `policy_narrows_source`,
  `trust_state_excludes_surface`, `redaction_narrowed`, or
  `pack_quarantined`. The subtotal travels through every export
  pipeline so support and audit reviewers can count policy-blocked
  hits without re-scanning per-row data.
- `hidden_by_scope_count` is the subtotal of rows whose
  `hidden_scope_reason` is `sparse_scope_excludes_root`,
  `outside_loaded_scope`, `excluded_by_user_filter`,
  `client_scope_excludes_surface`, or `remote_shard_unreachable`.
- `hidden_scope_reason_histogram` is the per-reason breakdown. A
  non-zero `total_hidden_result_count` MUST emit at least one
  histogram entry; missing entries deny with
  `hidden_result_reason_missing_for_non_zero_count`.

## Bookmark / navigation-history continuity audit rules (normative)

Each `bookmark_history_continuity_entry` ties one caveat class to
the cited bookmark, navigation-history entry, or deep-link binding
plus the observed drift state.

- `bookmark_drift_present`, `bookmark_remapped_present`,
  `bookmark_missing_target_present`,
  `bookmark_scope_unavailable_present`, and
  `bookmark_archived_tombstone_present` cite the
  `bookmark_id_ref` and the observed `bookmark_drift_state`.
- `navigation_history_session_restore_replay_present` and
  `navigation_history_cross_surface_back_forward_exercised` cite
  the `navigation_history_entry_id_ref` and the observed
  `navigation_history_origin_class`.
- `deep_link_target_renamed_present`,
  `deep_link_target_moved_present`,
  `deep_link_target_branch_drifted_present`,
  `deep_link_scope_widened_on_resolve_present`,
  `deep_link_cross_repo_jump_present`, and
  `deep_link_unresolvable_present` cite the
  `search_deep_link_binding_id_ref` and the observed
  `deep_link_drift_state`.

The packet does NOT redeclare the bookmark drift state machine, the
navigation-history origin/direction vocabulary, or the deep-link
drift vocabulary. It re-exports the closed enums it needs and cites
the canonical owners by id.

## Export redaction posture rules (normative)

The export redaction posture is the packet's commitment to the
query/result export pipelines. It composes with — does not replace —
the saved-query bundle / collection-snapshot redaction rules:

- `raw_query_text_never_crosses_boundary` mirrors the
  `query_text_material_class` discipline at
  `schemas/search/query_session.schema.json`.
- `saved_query_literal_local_only_by_default` mirrors the
  `literal_redacted_at_boundary_default` /
  `literal_kept_in_local_only_store` posture at
  `schemas/search/saved_query_bundle.schema.json`.
- `saved_query_literal_hashed_with_local_salt_for_export` is the
  hashed-export posture; the salt is local-only and MUST NOT be
  reused across tenants, workspaces, or support packets.
- `saved_query_literal_disclosed_with_explicit_user_opt_in` is the
  only class under which a literal MAY accompany an export. Without
  the explicit user opt-in, exports deny with
  `sensitive_literal_export_requires_explicit_opt_in` at the
  saved-query bundle layer.
- `shared_link_reopens_with_current_viewer_permissions` mirrors the
  reopen-honesty contract: shared deep links resolve through the
  current viewer's `policy_context` and `running_build_identity_ref`.
- `shared_link_does_not_widen_authority_beyond_viewer` mirrors the
  workspace-trust rule (ADR-0018): shared links MUST NOT widen
  authority beyond what the current viewer already holds.
- `exported_packet_preserves_hidden_by_policy_count`,
  `exported_packet_preserves_hidden_scope_reasons`, and
  `exported_packet_preserves_partial_truth_causes` mirror the
  search-readiness vocabulary's "never silently drop the count" rule.
- `exported_packet_preserves_semantic_fallback_state` and
  `exported_packet_preserves_provider_attribution_chip` mirror the
  semantic-fallback and provider-attribution disclosure rules.
- `exported_packet_states_captured_results_not_current_truth`
  mirrors the saved-query bundle / collection-snapshot honesty rule
  that captured rows are replay material, not current truth.
- `support_export_redacts_raw_paths_and_urls` and
  `public_proof_export_carries_zero_raw_payload` are the audience-
  specific guarantees applied on top of the floor.

## Linked artifact families (normative)

The packet's linked-artifact-families block names which artifact
families this packet composes with. The five-class floor
(`search_readiness_vocabulary`, `search_query_session_contract`,
`saved_query_bundle_contract`, `navigation_and_saved_query_contract`,
`cross_repo_result_group_schema`) is enforced by the schema gates.
Optional opaque refs into each family let the support / audit
reviewer follow each linkage to its canonical owner.

A packet that audits AI-explanation overlay rows SHOULD also cite
`citation_anchor_object_model` so the imported / docs / AI evidence
chain is auditable end-to-end. A packet that asserts parity with the
result-truth labelled corpus SHOULD also cite
`result_truth_label_corpus` and pin its
`result_truth_label_corpus_ref` in the relevance corpus summary.

## Relationship to adjacent contracts

- **Search readiness, ranking-reason, hidden-scope, partial-truth-
  cause, semantic-fallback, scope-filter, freshness, deep-link drift
  vocabularies (ADR 0014)** — authoritative source for every closed
  enum the packet re-exports. Adding a value there is additive-minor
  and bumps `search_result_schema_version`; this packet picks up the
  new value automatically through its schema-version pin.
- **Canonical query-session contract** — authoritative source for
  the query-session / result-identity / explanation-capture record
  family. The packet pins
  `schemas/search/query_session.schema.json`'s
  `query_session_schema_version`.
- **Saved-query bundle and search-collection-snapshot contract** —
  authoritative source for saved-query source class, share-policy
  class, provider-backing class, sensitive-literal handling class,
  retention class, snapshot-export-field class, and reopen-honesty
  state. The packet pins
  `saved_query_bundle_schema_version` and uses the
  `snapshot_export_field_class` vocabulary for the export-redaction
  posture.
- **Navigation-and-saved-query contract** — authoritative source for
  bookmark drift state, bookmark kind class, navigation-history
  origin and direction class, peek projection class, and
  navigation-artifact denial reasons. The packet pins
  `navigation_artifacts_schema_version`.
- **Workset-scope and cross-repo result-group contracts** —
  authoritative source for workset, scope-banner, cross-repo
  result-group, and scope-diff-review record families. The packet's
  scope/workset caveats cite workset, scope-binding, cross-repo
  result-group, and scope-diff-review row ids verbatim.
- **Citation anchor object model** — authoritative source for docs
  citation anchors. Imported-pack and docs-search evidence in the
  packet cites citation anchors through this schema.
- **Workspace-trust contract (ADR-0018)** — authoritative source for
  workspace-trust state. Shared deep links MUST NOT widen authority
  beyond what the current viewer already holds; the packet's
  export-redaction posture
  `shared_link_does_not_widen_authority_beyond_viewer` mirrors that.
- **Broker-owned redaction (ADR-0007)** — owns the redaction pass.
  Search-truth packets carry opaque refs and reviewable labels only.

## Schema-of-record posture

The eventual search / navigation crate's Rust types are the schema
of record. The boundary schema at
`schemas/search/search_truth_packet.schema.json` is the cross-tool
boundary every non-owning surface reads. Adding a new audience class,
redaction-profile class, window-kind class, corpus-coverage class,
hierarchy-fallback class, scope-caveat class, bookmark-history-
continuity caveat class, export-redaction-posture class, linked-
artifact-family class, change-significance class, parity-floor surface
class, denial reason, or audit-event id is additive-minor and bumps
`search_truth_packet_schema_version`. Repurposing an existing value is
breaking and requires a new decision row in
`artifacts/governance/decision_index.yaml`.

## Acceptance criteria cross-walk

This contract delivers M00-520's three acceptance bullets:

1. **Search and navigation claims can be reviewed as a packet
   instead of screenshot collections or ad hoc demos.** The frozen
   `search_navigation_truth_packet_record` shape, the schema-version
   pin floor, the result-kind coverage matrix, the hierarchy fallback
   matrix, the hidden-result aggregate, and the linked-artifact-family
   floor produce one inspectable record that the milestone scorecard,
   release-evidence packet, support handoff bundle, AI evidence
   packet, parity audit, and public-proof index can quote. Worked
   fixtures cover an informational milestone-close packet, a
   release-bearing packet that asserts parity across every surface,
   a claim-narrowing packet under semantic-disabled-by-policy, and a
   claim-widening-blocked packet held by a remote partial load and an
   unresolvable cross-repo deep-link.
2. **Bookmark / history continuity and exported query privacy remain
   explicit under restore, scope drift, and cross-repo navigation.**
   The thirteen-class
   `bookmark_history_continuity_caveat_class` vocabulary plus the
   per-entry pin to bookmark / navigation-history / deep-link refs
   plus the observed drift state, combined with the fourteen-class
   `export_redaction_posture_class` vocabulary plus the five-class
   floor, force every packet to disclose continuity exposure and
   export-pipeline guarantees in typed terms.
3. **The packet can narrow search-related claims when evidence is
   stale, partial, or limited to certain corpora or workset sizes.**
   The `claim_narrowing` and `claim_widening_blocked` change-
   significance classes, the schema gates that force claim-row refs
   on narrowing and blocking-caveat classes on blocked, and the ten-
   class scope/workset caveat vocabulary let a packet pull a public
   claim back to internal-only when the corpus or workset is too
   narrow.

## Non-goals at this milestone

Out of scope until a superseding decision row opens:

- the live ranking, indexing, or navigation engines;
- the saved-query share UI, the provider-backed search opt-in flow
  UI, the support-export pipeline, the org-share sync transport, and
  the managed-workspace provisioning pipeline (frozen out at
  `navigation_and_saved_query_contract.md`);
- the eventual search, navigation, and support / export crates' Rust
  types;
- the milestone-scorecard / release-evidence shiproom packet UI that
  consumes this packet — that surface composes with this contract via
  the consuming-surface parity floor and is not redeclared here.

These lines move only by opening a new decision row, not by editing
this contract.

## Reuse guarantee

This contract is reusable by milestone, release-evidence, support,
parity-audit, and public-proof surfaces without redefining search /
navigation truth. A new surface MUST:

1. cite at least one `corpus_coverage_class` from §"Frozen
   vocabulary" and back the claim with a non-empty
   `corpus_fixture_refs` list;
2. emit one `result_kind_coverage_entry` per `surface_class`
   covered, with non-empty `exercised_readiness_states`,
   `exercised_result_truth_classes`, and
   `exercised_semantic_fallback_states`;
3. emit a non-empty `hierarchy_fallback_classes` list;
4. emit a `hidden_result_aggregate` with a non-empty histogram
   whenever `total_hidden_result_count > 0`;
5. cite the five-class linked-artifact floor and the five-class
   export-redaction floor;
6. pin the four schema-version floor entries;
7. honour the broker-owned redaction pass: opaque refs and
   reviewable labels only, no raw payloads.

## Source anchors

- `.t2/docs/Aureline_PRD.md` — search relevance, navigation
  continuity, saved-query / deep-link, scope-banner, and cross-repo
  result-group requirements.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` — search /
  navigation artifact storage, retention, and export story.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — bookmark / outline /
  saved-query / scope-banner UX rules.
- `docs/adr/0014-search-readiness-ranking-result-truth.md` —
  search-readiness, result-truth, ranking-reason, hidden-scope,
  partial-truth, semantic-fallback, scope-filter, deep-link drift
  vocabularies.
- `docs/search/search_query_session_contract.md` — query-session,
  result-identity, explanation-capture contract.
- `docs/navigation/navigation_and_saved_query_contract.md` —
  navigation-continuity, bookmark-drift, saved-query / deep-link
  contract.

## Linked artifacts

- Boundary schema: `schemas/search/search_truth_packet.schema.json`.
- Worked fixtures: `fixtures/search/search_truth_cases/`.
- Search-readiness vocabulary: `docs/search/search_readiness_vocabulary.md`.
- Search-result-truth schema: `schemas/search/search_result_truth.schema.json`.
- Query-session schema: `schemas/search/query_session.schema.json`.
- Saved-query bundle schema: `schemas/search/saved_query_bundle.schema.json`.
- Navigation-artifacts schema: `schemas/navigation/navigation_artifacts.schema.json`.
- Cross-repo result-group schema: `schemas/workspace/cross_repo_result_group.schema.json`.
- Workset-artifact schema: `schemas/workspace/workset_artifact.schema.json`.
- Scope-diff-review schema: `schemas/workspace/scope_diff_review.schema.json`.
- Citation-anchor schema: `schemas/docs/citation_anchor.schema.json`.
- Result-truth label corpus: `artifacts/search/result_truth_labels.yaml`.
