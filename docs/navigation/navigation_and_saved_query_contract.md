# Navigation-continuity, bookmark-drift, and saved-query / deep-link contract

This document is the **product-wide contract** for the durable
navigation and search artifacts every editor, docs, notebook,
review, graph, AI, and search surface reads when restoring back /
forward, recent location, breadcrumb, outline, bookmark / mark, peek
context, saved query, query-history entry, scope binding, search
deep link, or search collection snapshot. It freezes one set of
artifact shapes, one bookmark-drift state machine, one set of
privacy / portability / sync / sharing rules for sensitive query
literals and provider-backed search, and one set of current-versus-
captured scope honesty rules every reopened query and shared deep
link MUST honour, so restore, sync, support export, and cross-
surface navigation never invent incompatible identities.

The contract is normative. Where this document disagrees with the
source product / architecture / UI-UX spec it quotes, the source
wins and this document MUST be updated in the same change. Where
this document disagrees with a downstream editor, docs, notebook,
review, graph, AI, search, or support / export surface's mint of
its own navigation or saved-query state, this document wins and
the surface is non-conforming.

The companion artifacts are:

- [`/schemas/navigation/navigation_artifacts.schema.json`](../../schemas/navigation/navigation_artifacts.schema.json)
  — boundary schema for the
  `navigation_breadcrumb_trail_record`,
  `navigation_outline_snapshot_record`,
  `navigation_bookmark_record`,
  `navigation_history_entry_record`,
  `navigation_peek_context_record`, and
  `navigation_artifact_audit_event_record` shapes.
- [`/schemas/search/saved_query_bundle.schema.json`](../../schemas/search/saved_query_bundle.schema.json)
  — boundary schema for the `saved_query_bundle_record`,
  `search_collection_snapshot_record`, and
  `saved_query_bundle_audit_event_record` shapes. Wraps the
  per-record family at
  [`/schemas/search/saved_query_and_scope_binding.schema.json`](../../schemas/search/saved_query_and_scope_binding.schema.json)
  and never redeclares `saved_query_record`,
  `query_history_entry_record`, `scope_binding_record`, or
  `search_deep_link_binding_record`.
- [`/fixtures/navigation_and_search_artifacts/`](../../fixtures/navigation_and_search_artifacts/)
  — worked-example corpus covering breadcrumb / outline /
  bookmark-drift / navigation-history / peek / saved-query-bundle /
  collection-snapshot rows.

This contract **composes with and does not replace** vocabularies
already frozen in:

- [`/docs/adr/0014-search-readiness-ranking-result-truth.md`](../adr/0014-search-readiness-ranking-result-truth.md)
  — search readiness, result-truth, ranking-reason, hidden-scope,
  partial-truth-cause, semantic-fallback, scope-filter, and
  deep-link drift vocabularies. Every saved-query / collection-
  snapshot / deep-link binding row in this contract resolves
  vocabulary through that ADR via
  `schemas/search/search_result_truth.schema.json`.
- [`/docs/search/query_planner_contract_seed.md`](../search/query_planner_contract_seed.md)
  and
  [`/schemas/search/result_fusion_record.schema.json`](../../schemas/search/result_fusion_record.schema.json)
  — the planner-pass / shard-snapshot / result-fusion vocabulary
  every collection snapshot pins through `originating_planner_pass_ref`.
- [`/schemas/search/saved_query_and_scope_binding.schema.json`](../../schemas/search/saved_query_and_scope_binding.schema.json)
  — the per-record `saved_query_record`,
  `query_history_entry_record`, `scope_binding_record`, and
  `search_deep_link_binding_record` shapes already shipped in
  M00-183. The bundle and snapshot rows added here cite those
  records by opaque ref; this contract MUST NOT redeclare their
  payload shape.
- [`/schemas/workspace/workset_artifact.schema.json`](../../schemas/workspace/workset_artifact.schema.json)
  — named-workset / sparse-scope / scope-truth-chip artifacts. A
  `scope_binding_record` whose `scope_filter_class` is `named_workset`
  pins through this artifact; this contract names the binding rule
  but does not redeclare workset shape.
- [`/schemas/docs/citation_anchor.schema.json`](../../schemas/docs/citation_anchor.schema.json)
  — docs citation-anchor object model. Bundles whose source class
  is `support_export_captured` or `ai_explanation_captured` cite
  citation anchors through this schema rather than re-deriving
  them.
- [`/docs/ux/navigation_and_escalation_contract.md`](../ux/navigation_and_escalation_contract.md)
  — navigation routes, escalation tiers, and progressive-
  disclosure depths. This contract names *durable navigation
  artifacts*; that contract names *which routes a user takes to
  reach them*. The two compose: a bookmark resolved via
  `route.command_palette` is the same `navigation_bookmark_record`
  as one resolved via `route.context_menu`.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — broker-owned redaction pass. Navigation and saved-query
  records carry opaque refs and reviewable labels only; raw
  literals never cross either schema boundary.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  — `freshness_class`, `client_scope`, `redaction_class`
  re-exported without modification.
- [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  — workspace-trust state. Navigation artifacts inherit the
  joining workspace's trust posture; widening trust through a
  bookmark restore is forbidden.

If this document disagrees with those sources, those sources win
and this document plus the schemas are updated in the same change.

This document does not ship a navigation UI, a sync transport, a
saved-query share UI, or a support-export pipeline. It freezes the
row shape those implementations will read and write. The eventual
navigation crate's Rust types are the schema of record; the JSON
Schema exports at
`schemas/navigation/navigation_artifacts.schema.json` and
`schemas/search/saved_query_bundle.schema.json` are the cross-tool
boundaries every non-owning surface reads.

## Why freeze this now

Without one frozen contract every surface is free to invent its
own per-feature notion of "back / forward", "breadcrumb",
"outline", "bookmark", "peek", "saved query", "search deep link",
and "what survives a session restore or support export". Each
divergence widens a different axis silently:

1. *The editor's back stack and the docs viewer's back stack are
   incompatible, so cross-surface back / forward feels broken.*
   Users lose the ability to step back across surfaces because
   each surface owns a different row shape.
2. *A bookmark whose target file was renamed silently re-binds to
   a similarly-named file.* Reviewers and authors cannot tell
   which target the bookmark really points at.
3. *A saved query is auto-shared to an org index because the
   share lane was the default.* Sensitive query literals (file
   names, secret stems, symbol fragments under triage) leak into
   shared infrastructure.
4. *A search deep link reopened from a teammate paints stale rows
   as the current authoritative result set.* Reviewers act on
   results that no longer hold.
5. *A support export captures a saved query but loses the planner
   pass / hidden-result count / partial-truth causes.* Support
   cannot reconstruct what the user actually saw.
6. *A managed-workspace-provisioned saved query is exported from
   a user's local store.* The provider's read-only contract is
   silently broken.

The freeze matters now, ahead of any navigation UI or sync
transport landing, so every later surface can read **the same**
artifact shape, **the same** bookmark-drift state machine,
**the same** privacy / portability / sync / sharing posture, and
**the same** current-versus-captured honesty rules instead of
inventing per-surface equivalents.

## Frozen vocabulary

This contract introduces the following frozen vocabularies. Each
is owned by exactly one of the two boundary schemas; downstream
surfaces re-export by reference and never mint a parallel value.

### Navigation-target kinds (frozen, nine values)

`navigation_target_kind` (owned by
`schemas/navigation/navigation_artifacts.schema.json`):

- `file_path_target` — file row in the workspace (resolved via
  the workspace VFS, never a raw absolute path).
- `symbol_declaration_target` — symbol row in the graph.
- `docs_anchor_target` — docs citation anchor (resolved through
  `schemas/docs/citation_anchor.schema.json`).
- `notebook_cell_target` — notebook cell row.
- `review_diff_hunk_target` — review diff-hunk row.
- `graph_node_target` — graph-node row (when distinct from a
  symbol declaration: package, module, dependency).
- `search_result_packet_target` — `search_result_packet_record`
  id from the search-result-truth registry.
- `terminal_zone_target` — terminal-zone id.
- `output_viewer_object_target` — `output_viewer_object_record`
  id.

### Breadcrumb-segment classes (frozen, nine values)

`breadcrumb_segment_class`: `workspace_root_segment`,
`folder_segment`, `file_segment`, `container_symbol_segment`,
`leaf_symbol_segment`, `docs_section_segment`,
`notebook_section_segment`, `review_section_segment`,
`graph_neighbourhood_segment`. A trail starts at exactly one
`workspace_root_segment` and orders down to the active leaf.

### Outline-node classes (frozen, seven values)

`outline_node_class`: `file_outline_root`,
`container_symbol_node`, `leaf_symbol_node`, `docs_section_node`,
`notebook_cell_node`, `review_section_node`,
`graph_subgraph_node`. An editor outline has a `file_outline_root`
root; a docs outline may root at `docs_section_node`.

### Bookmark-drift states (frozen, six values)

`bookmark_drift_state` (owned by the navigation schema):

| State                | When it applies                                                                                                                       | Required user action                                                                                                          |
|----------------------|----------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------|
| `bound`              | The target resolved exactly under the current build identity.                                                                          | `no_user_action_required_bound_or_remapped` — surface the bookmark normally.                                                    |
| `remapped`           | The surface followed an authoritative rename / move with a recorded mapping; the bookmark owns its `remap_chain_target_id_refs`.       | `no_user_action_required_bound_or_remapped` — render the new target with a "Remapped from prior target" chip.                  |
| `drifted`            | The target moved but the resolver could not pick a single successor; more than one candidate matched.                                  | `user_must_pick_successor_drifted` — refuse the implicit jump; render the candidate list and ask the user to pick one.        |
| `missing_target`     | The target no longer exists at any resolvable location.                                                                                | `user_must_re_anchor_or_delete_missing_target` — refuse re-binding; offer to re-anchor manually or delete the bookmark.        |
| `scope_unavailable`  | The workspace / workset / docs pack / graph / remote shard the target lives in is currently outside admitted scope.                    | `user_must_widen_scope_or_load_pack_or_reach_remote` — render typed reason; offer to widen the scope or load the pack.        |
| `archived`           | The bookmark was retired by the user or by a managed sweep; kept as a tombstone for audit / restore.                                   | `user_must_restore_from_archive_or_acknowledge_tombstone` — list under the archive, never auto-resurrect.                     |

The schema gates pair each state to its required user action; a
downstream surface that re-renders a `missing_target` bookmark as
`bound` denies with
`bookmark_drift_state_required_action_mismatch`.

### Bookmark kind classes (frozen, seven values)

`bookmark_kind_class`: `user_authored_named_bookmark`,
`user_authored_quick_mark`, `session_recent_location`,
`review_pinned_anchor`, `ai_evidence_anchor`,
`support_export_pinned_anchor`,
`managed_workspace_pinned_anchor`. The kind composes with
`redaction_class` and `client_scope`s to govern retention.

### Navigation-history origin and direction classes (frozen)

`navigation_history_origin_class` (ten values):
`user_invoked_jump`, `command_palette_jump`,
`search_result_open`, `deep_link_resolution`,
`back_or_forward_step`, `recent_locations_pick`,
`ai_explanation_jump`, `review_walk_through`,
`notebook_cell_jump`, `session_restore_replay`. Only
`session_restore_replay` is admissible after a window restore;
the entry is replayed from durable history rather than minted
fresh.

`navigation_history_direction_class` (five values):
`navigated_forward_one_step`, `navigated_back_one_step`,
`navigated_to_recent_location`, `navigated_to_new_target`,
`no_op_already_at_target`.

### Peek-projection classes (frozen, nine values)

`peek_projection_class`: `definition_peek`, `reference_peek`,
`implementation_peek`, `type_definition_peek`, `docs_peek`,
`notebook_outputs_peek`, `review_diff_peek`,
`graph_neighbour_peek`, `search_result_preview_peek`. A peek does
NOT mint a navigation-history entry by default; promotion to a
jump pairs the peek to a fresh `navigation_history_entry_record`
and the promoted ref is recorded in
`promoted_to_navigation_history_entry_id_ref`.

### Saved-query source classes (frozen, ten values)

`saved_query_source_class` (owned by
`schemas/search/saved_query_bundle.schema.json`):
`user_authored_local`, `workspace_shared_committed`,
`profile_imported_portable`, `org_shared_admin_curated`,
`org_shared_admin_published_read_only`,
`managed_workspace_provisioned`, `support_export_captured`,
`ai_explanation_captured`, `session_restore_replayed`,
`parity_audit_captured`. The source class pairs with share-policy
and retention class through schema gates.

### Saved-query share-policy classes (frozen, six values)

`saved_query_share_policy_class`: `local_only_no_share`,
`workspace_share_explicit_admit_required`,
`org_share_admin_governed`, `support_export_redacted_only`,
`managed_admin_published_read_only_no_export`,
`share_disabled_by_policy`. `local_only_no_share` is the default
for `user_authored_local`; an admin policy that narrows away the
admitted share lane resolves to `share_disabled_by_policy`.

### Saved-query provider-backing classes (frozen, six values)

`saved_query_provider_backing_class`:
`first_party_local_only`, `first_party_with_remote_index`,
`provider_hosted_index_user_consented`,
`provider_hosted_index_admin_consented`,
`provider_hosted_index_disabled_by_policy`,
`provider_backing_not_applicable`. Bundles whose backing is
`provider_hosted_index_*_consented` MUST emit the provider-
attribution chip on every reopen surface;
`provider_hosted_index_disabled_by_policy` refuses re-resolution
against the provider lane and degrades to lexical-only via
`semantic_unavailable_lexical_only` from the search-result-truth
schema.

### Sensitive-literal handling classes (frozen, five values)

`saved_query_sensitive_literal_handling_class`:
`literal_redacted_at_boundary_default`,
`literal_hashed_with_local_salt`,
`literal_kept_in_local_only_store`,
`literal_kept_in_org_governed_vault`,
`literal_disclosed_with_explicit_user_opt_in`. The default for
every bundle that crosses the schema boundary is
`literal_redacted_at_boundary_default`; the raw literal lives in
the local store only and never leaves it. A literal-disclosing
class without explicit user opt-in denies with
`sensitive_literal_export_requires_explicit_opt_in`.

### Saved-query retention classes (frozen, seven values)

`saved_query_retention_class`:
`local_by_default_no_remote_retention`,
`workspace_committed_with_workspace_retention`,
`org_admin_governed_retention`,
`support_export_redacted_retention`,
`ai_evidence_retention_window`,
`parity_audit_retention_window`,
`managed_provider_retention_locked`. `local_by_default_no_remote_retention`
is the default; the other classes are admitted only when the
source-class and share-policy-class jointly admit them.

### Snapshot-export-field classes (frozen, fifteen values)

`snapshot_export_field_class`:
`snapshot_includes_visible_row_ids_only`,
`snapshot_includes_hidden_result_count_disclosure`,
`snapshot_includes_partial_truth_causes`,
`snapshot_includes_ranking_reason_classes`,
`snapshot_includes_result_truth_classes`,
`snapshot_includes_freshness_class_per_row`,
`snapshot_includes_scope_filter_class_pin`,
`snapshot_includes_workset_id_pin`,
`snapshot_includes_planner_pass_ref`,
`snapshot_includes_shard_authority_inventory`,
`snapshot_includes_citation_anchor_refs`,
`snapshot_includes_provider_attribution_chip`,
`snapshot_includes_redaction_class_pin`,
`snapshot_includes_running_build_identity_pin`,
`snapshot_includes_policy_epoch_pin`. Every honest snapshot MUST
carry at minimum
`snapshot_includes_planner_pass_ref` plus
`snapshot_includes_hidden_result_count_disclosure` (enforced via
schema `allOf` gates).

### Reopen-honesty states (frozen, ten values)

`reopen_honesty_state`:
`current_scope_matches_captured_scope_exact`,
`current_scope_widened_versus_captured_scope_disclosed`,
`current_scope_narrowed_versus_captured_scope_disclosed`,
`current_scope_remapped_versus_captured_scope_disclosed`,
`current_build_drifted_versus_captured_build_disclosed`,
`current_policy_epoch_drifted_versus_captured_disclosed`,
`current_provider_backing_changed_versus_captured_disclosed`,
`current_index_readiness_lower_than_captured_disclosed`,
`captured_results_unverifiable_at_current_build`,
`captured_snapshot_unavailable_replay_offered_only_as_history`.
Every reopen of a saved query / saved-query bundle / search-
collection snapshot MUST resolve to exactly one state and MUST
disclose the state on the reopened surface; a reopened query
that paints stale captured rows as the current authoritative
result set denies with
`reopen_must_disclose_current_versus_captured_drift`.

### Denial-reason vocabularies (frozen)

Navigation: `navigation_target_unknown`,
`navigation_target_kind_mismatch_for_record`,
`breadcrumb_trail_must_start_at_workspace_root`,
`breadcrumb_segment_index_not_contiguous`,
`outline_node_parent_unknown`,
`outline_root_must_have_no_parent`,
`bookmark_drift_state_required_action_mismatch`,
`bookmark_required_user_action_missing_for_non_bound`,
`navigation_history_origin_session_restore_requires_replay_source`,
`peek_projection_unknown_for_target_kind`,
`raw_body_forbidden_on_boundary`,
`navigation_artifacts_schema_version_lagging`,
`policy_epoch_expired`, `policy_blocked`,
`trust_state_excludes_resolution`.

Saved-query bundle: `saved_query_bundle_unknown`,
`saved_query_bundle_member_unknown`,
`saved_query_bundle_member_redaction_class_mismatch`,
`saved_query_bundle_member_share_policy_mismatch`,
`share_policy_disabled_by_policy`,
`sensitive_literal_export_requires_explicit_opt_in`,
`snapshot_planner_pass_ref_missing_required`,
`snapshot_hidden_result_count_disclosure_missing_required`,
`reopen_must_disclose_current_versus_captured_drift`,
`provider_hosted_index_disabled_by_policy`,
`raw_body_forbidden_on_boundary`,
`saved_query_bundle_schema_version_lagging`,
`policy_epoch_expired`, `policy_blocked`.

## Truthfulness posture (normative)

Every rule below is normative. A new editor, docs, notebook,
review, graph, AI, search, or support / export surface that
violates any of them is non-conforming regardless of how the
violation is painted.

1. **One back / forward and recent-location model.** Every
   navigation that crosses a surface boundary mints exactly one
   `navigation_history_entry_record`. Editor, docs, notebook,
   review, search, terminal-zone, and output-viewer surfaces all
   read this record shape; cross-surface back / forward and recent
   locations reference the same row family. A surface that mints
   its own per-surface back stack alongside the durable history
   denies with `navigation_target_kind_mismatch_for_record` when
   the parallel store is queried from another surface.
2. **Bookmarks resolve through one drift state machine.** Every
   resolution emits exactly one of `bound`, `remapped`, `drifted`,
   `missing_target`, `scope_unavailable`, `archived` and pairs to
   exactly one required user action. Silent rebinding of a
   `drifted` or `missing_target` bookmark is forbidden and denies
   with `bookmark_drift_state_required_action_mismatch`.
3. **Breadcrumb trails start at the workspace root.** Segment
   index 0 MUST be `workspace_root_segment`; subsequent indexes
   MUST be strictly increasing and contiguous. A trail that
   starts mid-tree denies with
   `breadcrumb_trail_must_start_at_workspace_root`; a trail with
   gaps denies with `breadcrumb_segment_index_not_contiguous`.
4. **Outlines are flat node lists keyed by parent ref.** Every
   outline node names its parent (or null on the root). The
   schema validates list shape; the outline owner enforces
   cycle / orphan detection and emits
   `outline_node_parent_unknown` or
   `outline_root_must_have_no_parent` on a malformed tree.
5. **Peeks do not mint history entries.** A peek context records
   `originating_navigation_history_entry_id_ref` (the entry the
   peek opened on top of) and, on user-driven promotion to a
   jump, records the paired
   `promoted_to_navigation_history_entry_id_ref`. Auto-promoting
   a peek to a jump on hover, focus, or scroll is non-conforming.
6. **Saved-query source class governs share, retention, and
   provider posture.** `user_authored_local` resolves to
   `local_only_no_share` and `local_by_default_no_remote_retention`
   (both gated by the schema). `support_export_captured` resolves
   to `support_export_redacted_only` plus
   `literal_redacted_at_boundary_default` or
   `literal_hashed_with_local_salt`. `managed_workspace_provisioned`
   resolves to `managed_admin_published_read_only_no_export` plus
   `managed_provider_retention_locked`. A bundle that asserts a
   share lane its source class does not admit denies with
   `saved_query_bundle_member_share_policy_mismatch`.
7. **Sensitive query literals are local-only or hashed by
   default.** Every bundle that crosses the schema boundary
   defaults to `literal_redacted_at_boundary_default`; the raw
   literal lives in the local store only and never leaves it. A
   literal-disclosing class
   (`literal_disclosed_with_explicit_user_opt_in`) without
   explicit user opt-in denies with
   `sensitive_literal_export_requires_explicit_opt_in`.
8. **Provider-backed search is disclosed.** Bundles whose backing
   is `provider_hosted_index_user_consented` or
   `provider_hosted_index_admin_consented` emit a provider-
   attribution chip on every reopen surface; bundles whose
   backing is `provider_hosted_index_disabled_by_policy` degrade
   to lexical-only via `semantic_unavailable_lexical_only` from
   the search-result-truth schema.
9. **Search-collection snapshots are honest about hidden
   counts.** Every snapshot MUST cite at minimum
   `snapshot_includes_planner_pass_ref` plus
   `snapshot_includes_hidden_result_count_disclosure`. A snapshot
   that omits the planner-pass pin denies with
   `snapshot_planner_pass_ref_missing_required`; a snapshot that
   omits the hidden-result-count disclosure denies with
   `snapshot_hidden_result_count_disclosure_missing_required`.
10. **Reopened queries and shared deep links report current
    versus captured state.** Every reopen resolves exactly one
    `reopen_honesty_state` and discloses it on the reopened
    surface. A reopened query that paints stale captured rows as
    the current authoritative result set denies with
    `reopen_must_disclose_current_versus_captured_drift`. The
    captured row set is replay material, not a current truth
    claim.
11. **Search deep links reopen with current permissions.** A
    deep-link binding resolves through the current
    `policy_context` and `running_build_identity_ref`; a deep
    link that bypasses current trust / scope / policy review
    denies with `policy_blocked` or
    `trust_state_excludes_resolution`. Shared deep links MUST
    NOT widen authority beyond what the current viewer already
    holds.
12. **No raw payloads cross either boundary.** Raw absolute
    paths, raw symbol bodies, raw document bodies, raw notebook
    cell text, raw terminal bytes, raw URLs, and raw query
    bodies MUST NOT appear on any record emitted against either
    schema. A row that carries a raw payload denies with
    `raw_body_forbidden_on_boundary`.

## Privacy, portability, sync, and sharing rules

This section names the minimum posture every navigation /
saved-query surface MUST honour.

### Sensitive query literals

- Default handling is `literal_redacted_at_boundary_default`. The
  raw literal lives in the local saved-query store only and never
  leaves it; the boundary carries the
  `query_classification` from
  `schemas/search/search_result_truth.schema.json#search_session_record`
  and an opaque `display_label_opaque_ref` only.
- Hashed export uses `literal_hashed_with_local_salt`. The salt
  is local-only; the hash is non-reversible at the boundary and
  is admitted on `support_export_captured` and
  `parity_audit_captured` lanes.
- Local-only store retention uses
  `literal_kept_in_local_only_store`. The literal stays on the
  user's machine; sync transports MUST NOT replicate the row.
- Org-governed vault retention uses
  `literal_kept_in_org_governed_vault` and is admitted only on
  `org_shared_admin_curated` /
  `org_shared_admin_published_read_only` source classes.
- Explicit opt-in disclosure uses
  `literal_disclosed_with_explicit_user_opt_in` and is the only
  class under which a literal MAY accompany an export. Without
  the explicit user opt-in the export denies with
  `sensitive_literal_export_requires_explicit_opt_in`.

### Org-shared queries

- `org_shared_admin_curated` is admin-curated and writable by
  the admin lane. Consumer surfaces MAY render but MUST honour
  any narrowing the admin set on the bundle (share lane
  reduction, retention shortening, sensitive-literal handling
  upgrade).
- `org_shared_admin_published_read_only` is admin-curated and
  read-only on every consumer. Consumers MUST NOT export, MUST
  NOT sync, and MUST NOT include the row in a support bundle
  unless the bundle is also admin-signed.
- An admin policy that narrows away the admitted share lane
  produces `share_disabled_by_policy`; the bundle is still
  resolvable locally for the curating admin but disabled on
  every consumer until the policy is widened.

### Provider-backed search

- `provider_hosted_index_user_consented` requires explicit user
  opt-in to the provider attribution before the bundle resolves
  against the provider lane. The reopen surface MUST emit the
  provider-attribution chip every time.
- `provider_hosted_index_admin_consented` requires admin opt-in
  in the workspace policy bundle; the user-facing surface still
  emits the provider-attribution chip.
- `provider_hosted_index_disabled_by_policy` refuses re-
  resolution against the provider lane and degrades to lexical-
  only. The reopen surface MUST emit
  `semantic_unavailable_lexical_only` from the search-result-
  truth schema, not paint the surface as broken.

### Support export

- A `support_export_captured` saved-query bundle MUST resolve to
  `support_export_redacted_only` share policy and to
  `literal_redacted_at_boundary_default` or
  `literal_hashed_with_local_salt` literal handling (gated by
  the schema).
- The companion `search_collection_snapshot_record` MUST cite at
  minimum `snapshot_includes_planner_pass_ref`,
  `snapshot_includes_hidden_result_count_disclosure`,
  `snapshot_includes_partial_truth_causes`,
  `snapshot_includes_running_build_identity_pin`, and
  `snapshot_includes_policy_epoch_pin` so the support engineer
  can reconstruct what the user actually saw.
- Raw rendered row text, raw query bodies, raw URLs, and raw
  absolute paths MUST NOT cross the support-export boundary.

### Sync and portability

- `local_by_default_no_remote_retention` is the default
  retention class. Sync transports MUST NOT replicate the row.
- `workspace_committed_with_workspace_retention` is the only
  retention class admitted to a committed workspace artifact;
  the bundle is then versioned with the workspace artifact, not
  separately.
- `profile_imported_portable` source class MAY require
  rebinding when the destination workspace's shard rows or
  workset ids do not match the source workspace; the bundle
  emits the typed reopen-honesty state on first resolution.

## Current-versus-captured scope honesty (normative)

Reopening a saved query, a saved-query bundle, or a search
collection snapshot is **always** a current-versus-captured
comparison; it is **never** a frozen-truth claim about current
results.

1. The reopen flow resolves the current `scope_binding_record`,
   the current `running_build_identity_ref`, the current
   `policy_context.policy_epoch`, and the current
   `saved_query_provider_backing_class` against the captured
   values pinned in the snapshot.
2. The flow assigns exactly one `reopen_honesty_state`:
   - `current_scope_matches_captured_scope_exact` — both scopes
     match; render normally.
   - `current_scope_widened_versus_captured_scope_disclosed`,
     `current_scope_narrowed_versus_captured_scope_disclosed`,
     `current_scope_remapped_versus_captured_scope_disclosed` —
     scope drifted; render the captured rows as replay material
     and re-run against current scope on user request.
   - `current_build_drifted_versus_captured_build_disclosed`,
     `current_policy_epoch_drifted_versus_captured_disclosed`,
     `current_provider_backing_changed_versus_captured_disclosed`,
     `current_index_readiness_lower_than_captured_disclosed` —
     the captured rows are still readable but cannot be
     verified against the current build / policy / provider /
     index; render as replay only.
   - `captured_results_unverifiable_at_current_build` — the
     captured rows refer to ids that no longer resolve; render
     the disclosure and refuse the silent re-render.
   - `captured_snapshot_unavailable_replay_offered_only_as_history` —
     the snapshot is unavailable (purged, redaction-narrowed,
     policy-blocked); offer only the history entry, not the
     captured rows.
3. Every reopen MUST surface the resolved state on the reopened
   surface (chip, banner, or sheet). A reopen that hides the
   state denies with
   `reopen_must_disclose_current_versus_captured_drift`.
4. Shared deep links resolve through the current viewer's
   permissions: the link reopens a search **intent** with
   current permissions, not a frozen result set with the
   sender's permissions.

## Relationship to adjacent contracts

- **Search readiness, ranking, result-truth, and deep-link drift
  vocabulary (ADR 0014)** is the authoritative source for
  `surface_class`, `result_truth_class`, `ranking_reason_class`,
  `hidden_scope_reason`, `partial_truth_cause`,
  `semantic_fallback_state`, `scope_filter_class`, `freshness_class`,
  `redaction_class`, `client_scope`, and `deep_link_drift_state`.
  The bundle / snapshot / reopen-honesty rows in this contract
  cite those vocabularies through
  `schemas/search/search_result_truth.schema.json`.
- **Query-planner contract seed and result-fusion record** is
  the authoritative source for planner-stage ids, shard-row ids,
  lane-grouping ids, and planner-pass / shard-snapshot identity.
  Every `search_collection_snapshot_record` pins through the
  `originating_planner_pass_ref` shape mirrored from
  `schemas/search/result_fusion_record.schema.json`.
- **Saved-query / scope-binding / deep-link binding registry** is
  the per-record family at
  `schemas/search/saved_query_and_scope_binding.schema.json`. The
  bundle / snapshot rows added here cite those records by opaque
  ref and never redeclare their payload shape.
- **Named-workset / sparse-scope / scope-truth chip artifact**
  is the workset registry. A `scope_binding_record` whose
  `scope_filter_class` is `named_workset` resolves through
  `schemas/workspace/workset_artifact.schema.json`; the snapshot
  pins the captured scope binding for the reopen comparison.
- **Docs citation-anchor object model** is the authoritative
  source for citation anchors. Bundles and bookmarks that bind
  `docs_anchor_target` resolve through
  `schemas/docs/citation_anchor.schema.json`.
- **Navigation hierarchy and escalation contract** owns the
  routes, escalation tiers, and progressive-disclosure depths a
  user takes to reach the artifacts in this contract. The two
  compose: a bookmark surfaced through `route.command_palette`
  is the same `navigation_bookmark_record` as one surfaced
  through `route.context_menu`.
- **Workspace-trust contract (ADR-0018)** is the authoritative
  source for workspace-trust state. Navigation artifacts inherit
  the joining workspace's trust posture; widening trust through
  a bookmark restore is forbidden.
- **Broker-owned redaction (ADR-0007)** owns the redaction pass.
  Navigation and saved-query records carry opaque refs and
  reviewable labels only.

## Schema-of-record posture

The eventual navigation crate's Rust types are the schema of
record. The boundary schemas at
`schemas/navigation/navigation_artifacts.schema.json` and
`schemas/search/saved_query_bundle.schema.json` are the cross-
tool boundaries every non-owning surface reads. Adding a new
record kind, navigation-target kind, breadcrumb-segment class,
outline-node class, bookmark-drift state, bookmark kind class,
navigation-history origin / direction class, peek-projection
class, saved-query source / share-policy / provider-backing /
sensitive-literal handling / retention class, snapshot-export-
field class, reopen-honesty state, denial reason, or audit-event
id is additive-minor and bumps `navigation_artifacts_schema_version`
or `saved_query_bundle_schema_version`. Repurposing an existing
value is breaking and requires a new decision row in
`artifacts/governance/decision_index.yaml`.

## Acceptance criteria cross-walk

This contract delivers M00-232's three acceptance bullets:

1. **One back / forward and recent-location model can be
   referenced across editor, docs, notebook, review, and search
   surfaces.** The frozen `navigation_history_entry_record`
   shape, the nine `navigation_target_kind` values, the ten
   `navigation_history_origin_class` values, and the five
   `navigation_history_direction_class` values cover every
   surface in scope. The
   `originating_navigation_history_entry_id_ref` /
   `promoted_to_navigation_history_entry_id_ref` pair on
   `navigation_peek_context_record` keeps peek-without-jump
   honest.
2. **Deep-link fixtures reopen a search intent with current
   permissions rather than claiming frozen certainty about
   current results.** The `reopen_honesty_state` vocabulary,
   the schema-gated minimum on
   `snapshot_export_field_classes`, and the
   `reopen_must_disclose_current_versus_captured_drift` denial
   reason force every reopen to disclose the current versus
   captured drift. Worked fixtures cover the exact-match,
   widened-scope, narrowed-scope, build-drifted, and policy-
   epoch-drifted reopen states.
3. **Sensitive search material is local-only or hashed by
   default unless the contract explicitly says otherwise.** The
   default `literal_redacted_at_boundary_default` handling, the
   schema gates that bind `user_authored_local` to
   `local_only_no_share` and
   `local_by_default_no_remote_retention`, the
   `support_export_captured` gate that forces redacted-or-hashed
   handling, and the
   `sensitive_literal_export_requires_explicit_opt_in` denial
   reason cover the privacy posture end-to-end.

## Non-goals at this milestone

Out of scope until a superseding decision row opens:

- the live navigation crate, the back / forward stack
  implementation, the breadcrumb / outline render pipelines, the
  bookmark UI, the peek surface engine, and the recent-locations
  picker;
- the saved-query share UI, the provider-backed search opt-in
  flow UI, and the support-export pipeline;
- the org-share sync transport and the managed-workspace
  provisioning pipeline;
- the eventual navigation, search, and support / export crates'
  Rust types.

These lines move only by opening a new decision row, not by
editing this contract.

## Reuse guarantee

This contract is reusable by editor, docs, notebook, review,
graph, AI, search, and support / export surfaces without
redefining navigation or saved-query semantics. A new surface
MUST:

1. cite at least one `navigation_target_kind` from §"Frozen
   vocabulary" and resolve every navigable destination through a
   `navigation_target_ref`;
2. mint exactly one `navigation_history_entry_record` per
   user-driven cross-surface navigation;
3. resolve every bookmark through the frozen `bookmark_drift_state`
   machine and surface the typed `bookmark_drift_required_user_action`
   to the user before any silent rebinding;
4. cite the existing `saved_query_record` /
   `query_history_entry_record` / `scope_binding_record` /
   `search_deep_link_binding_record` shapes by opaque ref when
   composing a `saved_query_bundle_record`;
5. emit a `search_collection_snapshot_record` whose
   `snapshot_export_field_classes` carries at minimum
   `snapshot_includes_planner_pass_ref` plus
   `snapshot_includes_hidden_result_count_disclosure` whenever
   it captures a visible result set;
6. resolve every reopen to exactly one `reopen_honesty_state`
   and surface the state on the reopened surface;
7. honour the broker-owned redaction pass: opaque refs and
   reviewable labels only, no raw payloads.

## Source anchors

- `.t2/docs/Aureline_PRD.md` — navigation-continuity, bookmark,
  saved-query, search-deep-link, and support-export requirements.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  navigation / search artifact storage and retention story.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — breadcrumb /
  outline / bookmark / peek / saved-query UX rules.
- `docs/adr/0014-search-readiness-ranking-result-truth.md` —
  result-truth and deep-link drift vocabularies.
- `docs/search/query_planner_contract_seed.md` — planner-pass /
  shard-snapshot / lane-grouping vocabularies.

## Linked artifacts

- Navigation-artifacts boundary schema:
  `schemas/navigation/navigation_artifacts.schema.json`.
- Saved-query bundle and search-collection-snapshot boundary
  schema: `schemas/search/saved_query_bundle.schema.json`.
- Per-record saved-query / scope-binding / deep-link binding
  registry:
  `schemas/search/saved_query_and_scope_binding.schema.json`.
- Search-result-truth boundary schema:
  `schemas/search/search_result_truth.schema.json`.
- Result-fusion / planner-pass boundary schema:
  `schemas/search/result_fusion_record.schema.json`.
- Workspace-workset artifact:
  `schemas/workspace/workset_artifact.schema.json`.
- Docs citation-anchor object model:
  `schemas/docs/citation_anchor.schema.json`.
- Navigation-hierarchy and escalation contract:
  `docs/ux/navigation_and_escalation_contract.md`.
- Worked-example fixtures:
  `fixtures/navigation_and_search_artifacts/`.
