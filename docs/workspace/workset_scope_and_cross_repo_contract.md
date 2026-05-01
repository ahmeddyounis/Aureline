# Workset switcher, scope banner, and cross-repo result-group contract

This document freezes the cross-surface object model the workset
switcher, the scope banner, and every cross-repo result group read
when Aureline names **which workset is active**, **how the active
scope projects onto a row in the picker**, **what banner the user
sees while searching or navigating**, and **how results from
multiple roots stay grouped without flattening multi-repo and
sparse-scope truth into one synthetic workspace**.

Scope is a first-class object (frozen in
[`scope_truth_packet.md`](scope_truth_packet.md)). This contract
is the seed for the three surfaces the open / search foundations
render that scope through:

1. the **workset switcher** — the picker that lists every workset,
   workspace fallback, sparse slice, policy-limited overlay,
   managed workset, and ephemeral session the user can activate;
2. the **scope banner** — the persistent strip that keeps the
   current boundary visible while the user searches, navigates,
   reviews, or hands off to AI / refactor / export surfaces;
3. the **cross-repo result group** — one group per repo / root with
   its in-scope vs outside-scope marker, freshness, readiness,
   imported-root disclosure, hidden-by-workset / hidden-by-policy /
   not-loaded / evidence-missing counts, and open-in-new-pane
   action.

The machine-readable schemas live at:

- [`/schemas/workspace/workset_switcher.schema.json`](../../schemas/workspace/workset_switcher.schema.json)
- [`/schemas/workspace/cross_repo_result_group.schema.json`](../../schemas/workspace/cross_repo_result_group.schema.json)

The companion fixtures live under:

- [`/fixtures/workspace/workset_cross_repo_cases/`](../../fixtures/workspace/workset_cross_repo_cases/)

The durable workset artifact (the thing the switcher rows and the
banner project) and the chip / scope-widen-diff contract live in:

- [`/schemas/workspace/workset_artifact.schema.json`](../../schemas/workspace/workset_artifact.schema.json)
- [`scope_truth_packet.md`](scope_truth_packet.md)

The repository-topology truth contract that backs imported-root,
not-fetched, pointer-only, and wrong-target-root disclosures lives
in:

- [`/schemas/workspace/repo_topology_state.schema.json`](../../schemas/workspace/repo_topology_state.schema.json)
- [`repository_topology_edge_case_contract.md`](repository_topology_edge_case_contract.md)

The unified search planner and result-fusion contract whose
freshness, readiness, lane, and result-truth vocabulary this
contract re-exports lives in:

- [`/schemas/search/result_fusion_record.schema.json`](../../schemas/search/result_fusion_record.schema.json)
- [`/docs/search/query_planner_contract_seed.md`](../search/query_planner_contract_seed.md)

This contract is normative. Where it disagrees with the PRD, TAD,
TDD, or UI/UX Spec quotations cited in §7, those documents win and
this document MUST be updated in the same change. Where this
document disagrees with a downstream switcher row, scope banner,
search result group header, topology map node tooltip, impact
explorer affected-object row, AI context inspector segment, or
cited explainer evidence card that mints its own scope vocabulary,
this document wins and the surface is non-conforming.

## Why freeze this now

A search that says `No results` when the user expected matches in
a sibling repo, a navigation jump that silently swaps the active
workset, an AI inspector that quotes a file the user thought was
excluded, an impact explorer that flattens three repos into one
node, a topology map that hides imported-root provenance behind a
generic edge — every one of those is a scope-flatten incident.
Each surface that re-mints its own switcher row, banner label,
group header, or row marker compounds the problem.

Freezing the switcher row, banner, and cross-repo result group
contract here means:

- the picker projects the same five-class scope vocabulary the
  durable artifact carries (`current_repo`, `selected_workset`,
  `sparse_slice`, `full_workspace`, `policy_limited_view`);
- the banner never claims `active_narrow_safe` while a non-zero
  hidden-result count is known;
- search, symbol navigation, topology maps, impact explorers,
  cited explainers, AI context inspectors, and review-side
  semantic hint surfaces read one scope vocabulary instead of
  inventing surface-local labels;
- `No results` stays distinct from `No results in this workset`,
  `Index not built for excluded roots`, and `Blocked by trust or
  policy`.

## Scope

This contract freezes five record kinds across two schemas:

1. `workset_switcher_record` — the switcher itself: an ordered
   list of rows plus the active-workset highlight.
2. `workset_switcher_row_record` — one row in the switcher
   (named workset, current-repo fallback, full workspace, sparse
   slice, policy-limited overlay, managed workset, imported
   portable workset, ephemeral session).
3. `scope_banner_record` — the persistent banner keyed to the
   active workset.
4. `cross_repo_result_group_record` — one group per repo / root
   in a search / navigation / topology / impact / explainer
   result set, with in-scope vs outside-scope marker, freshness,
   readiness, hidden-row counts, and topology-reuse pointers.
5. `cross_repo_result_row_record` — one row inside a group, with
   the reserved result-state classes the spec calls for
   (`hidden_by_workset`, `outside_current_scope`, `imported_root`,
   `partially_loaded`, `evidence_missing`) plus the in-scope
   default `in_scope_loaded` and the policy-blocked variant
   `policy_blocked`.

Out of scope:

- Implementing multi-repo indexing, search engines, semantic
  graph engines, or impact-propagation engines. The schemas only
  freeze the vocabulary those engines emit.
- Final copy / microcopy. The closed label families freeze the
  vocabulary; the rendered strings live with the shell
  interaction-safety contract.
- Sparse-checkout / partial-clone / submodule / LFS hydration
  truth itself — that lives in
  [`repository_topology_edge_case_contract.md`](repository_topology_edge_case_contract.md).
  This contract reuses its honesty labels by reference.
- Search planner stages, ranking, or shard topology. Those live
  in
  [`/docs/search/query_planner_contract_seed.md`](../search/query_planner_contract_seed.md);
  this contract reuses its freshness / readiness / lane / result-
  truth vocabulary.

## 1. Workset switcher row

Every row in the workset switcher is a `workset_switcher_row_record`.
Each row corresponds to exactly one `workset_artifact_record` and
quotes its `workset_id` through `workset_ref`; rows never mint a
parallel stable scope id.

### 1.1 Switcher row class

The closed switcher-row vocabulary:

- `named_workset_row` — a user-named workset
  (`scope_class = selected_workset`).
- `current_repo_fallback_row` — the single-root fallback
  (`scope_class = current_repo`). The default for a one-root
  local open.
- `full_workspace_row` — every root in the active workspace
  (`scope_class = full_workspace`).
- `sparse_slice_row` — a root narrowed by user-authored
  include/exclude patterns (`scope_class = sparse_slice`).
- `policy_limited_overlay_row` — any of the above narrowed by an
  admin / trust / license / remote / index-not-built / user-muted
  policy (`scope_class = policy_limited_view`). Carries
  `policy_overlay` and `underlying_workset_ref`.
- `managed_workset_row` — `source_class = managed`. Export
  outside the provider is forbidden.
- `imported_portable_workset_row` — `source_class =
  profile_imported`. Renders rebinding / portability disclosure.
- `ephemeral_session_row` — `source_class = ephemeral_session`.
  Never appears in a persisted support / export bundle.

Rules (frozen):

1. A row's `switcher_row_class` MUST be consistent with the
   underlying artifact's `scope_class` and `source_class`.
   `policy_limited_overlay_row` requires
   `scope_class = policy_limited_view`; `managed_workset_row`
   requires `source_class = managed`; `imported_portable_workset_row`
   requires `source_class = profile_imported`;
   `ephemeral_session_row` requires
   `source_class = ephemeral_session`.
2. Exactly one row per parent `workset_switcher_record` sets
   `is_active = true`. The active row's `workset_ref` MUST
   equal the parent record's `active_workset_ref`.
3. `policy_overlay` is required when
   `switcher_row_class = policy_limited_overlay_row` and forbidden
   on every other row class.
4. `export_workset_artifact` is forbidden in `offered_actions` on
   `managed_workset_row` and `ephemeral_session_row`.
5. `view_policy_overlay_admin_only` is admissible only on
   `policy_limited_overlay_row` and only when the surface is a
   policy-admin surface.

### 1.2 Required fields

Every switcher row carries:

- `workset_ref` — pointer to the workset artifact.
- `workset_name` — redaction-aware human label.
- `scope_class` — re-export of the artifact's `scope_class`.
- `repo_count` — distinct `root_refs` (>= 1; >= 2 means
  multi-root).
- `folder_count` — distinct folder / module member refs.
- `source_class` — `local_only` / `workspace_shared` /
  `profile_imported` / `managed` / `ephemeral_session`.
- `readiness_state` — `cold` / `warming` / `warm` / `partial` /
  `ready`.
- `is_active` — boolean.
- `offered_actions` — at least one
  `switcher_action` from the closed list (open, manage, create,
  rename, duplicate, delete, export, copy id, build missing
  indexes, open scope diff, view policy overlay, open in new
  pane).

The closed `readiness_state` and `source_class` vocabularies are
re-exports of the durable artifact's enums. Surfaces never invent
a sixth readiness or source class.

## 2. Scope banner

The `scope_banner_record` is the persistent strip rendered above
search results, the navigation column, the AI panel, and any
review surface that depends on scope truth. The banner reads the
active workset artifact and projects it.

### 2.1 Banner state

The closed banner-state vocabulary:

- `active_narrow_safe` — current scope is fully indexed and no
  hidden content is relevant.
- `active_partial` — current scope is active but the index is
  partial.
- `active_policy_limited` — current scope has a
  `policy_limitation` overlay.
- `active_widened` — current scope was widened past the named
  workset. Carries a `widen_diff_ref` to a
  `scope_widen_diff_record` and exposes `open_scope_diff` in
  `offered_actions`.
- `active_warming` — current scope is in `cold` / `warming`
  readiness; results may still be filling.
- `active_outside_current_scope_disclosed` — the banner names
  that the user just consumed an out-of-scope result and is
  offered widen / reset.

Rules (frozen):

1. `active_narrow_safe` is forbidden when
   `hidden_result_summary.known = true` and `count > 0`. The
   banner MUST resolve to `active_partial`,
   `active_policy_limited`, or
   `active_outside_current_scope_disclosed` instead.
2. `active_widened` MUST carry a non-null `widen_diff_ref` and
   MUST include `open_scope_diff` in `offered_actions`.
3. `hidden_result_summary.count_class = outside_scope_roots`
   requires `widen_with_review` in `offered_actions`.
4. `policy_overlay` is required when
   `banner_state = active_policy_limited` or
   `scope_class = policy_limited_view`, and forbidden otherwise.
5. `reveal_hidden_results_policy_admin_only` is admissible only
   on a policy-admin surface and only when
   `policy_overlay.hidden_member_list_visible = true`.

### 2.2 Banner label family

The closed banner-label family:

- `Current repo`
- `Selected workset <name>`
- `Sparse slice`
- `Full workspace`
- `Policy-limited view`

A banner with `scope_class = selected_workset` MUST include the
workset name in the rendered label. Bare `Selected workset`
without the name is non-conforming.

### 2.3 Banner action set

The closed banner-action vocabulary:

- `widen_to_full_workspace`
- `widen_with_review`
- `narrow_to_current_repo`
- `open_scope_diff`
- `build_missing_indexes`
- `keep_current_scope`
- `reset_to_default_workset`
- `open_workset_switcher`
- `reveal_hidden_results_policy_admin_only`

Every banner carries at least one action. Banners with no
actions are non-conforming.

## 3. Cross-repo result group

Search, symbol navigation, topology maps, impact explorers, AI
context inspectors, cited explainers, and review-side semantic
hint surfaces all emit `cross_repo_result_group_record`s. Each
group names exactly one root_ref; results from another root render
as a separate group. Two multi-root groups never silently merge
into one synthetic workspace group.

### 3.1 Group header fields

Every group carries:

- `group_id` — stable opaque id.
- `query_session_ref` — pointer to the search / query session
  the group was emitted under.
- `active_workset_ref` — pointer to the active workset artifact.
- `scope_class` — re-export of the artifact's scope class.
- `root_ref` — pointer to the ADR-0006 filesystem-identity root
  the group's results live under.
- `repo_label` — redaction-aware repo / root label.
- `module_or_path_label` — redaction-aware module-or-path cue.
- `in_scope_marker` — `in_current_scope` /
  `outside_current_scope` / `imported_root` /
  `policy_overlay_layered`.
- `freshness_class` — `authoritative_live` / `warm_cached` /
  `degraded_cached` / `stale` / `imported_only` / `unverified` /
  `unknown`.
- `readiness_state` — `not_indexed` / `hot_set_only` /
  `partial_index` / `warm_index` / `fully_indexed` /
  `stale_index` / `reindexing` / `index_unavailable`.
- `result_truth_class` — `exact` / `imported` / `heuristic` /
  `hybrid`.
- `group_presentation_state` — `loaded_complete` /
  `loaded_partial` / `warming` / `stale_disclosed` /
  `policy_overlay_disclosed` / `imported_only_disclosed` /
  `outside_current_scope_disclosed` / `evidence_missing_disclosed`.
- `row_count_total` — total visible rows.
- `rows[]` — ordered visible rows.
- `hidden_count_summary` — typed counts for
  `hidden_by_workset`, `hidden_by_policy`, `not_loaded`, and
  `evidence_missing`.
- `topology_reuse` — reuse pointers + caveat-preservation list
  (see §5).
- `consumer_surface_class[]` — surface families this group is
  admissible on.
- `offered_actions[]` — at least one group action.

Rules (frozen):

1. `in_scope_marker = outside_current_scope` MUST include
   `widen_with_review` or `open_scope_diff` in
   `offered_actions`.
2. `in_scope_marker = imported_root` MUST set
   `freshness_class = imported_only` or `warm_cached` and
   `result_truth_class != exact`.
3. `group_presentation_state = loaded_complete` MUST set every
   `hidden_count_summary` count to 0.
4. `group_presentation_state = policy_overlay_disclosed` MUST set
   `hidden_by_policy_known = true`.
5. `group_presentation_state = outside_current_scope_disclosed`
   MUST set `in_scope_marker = outside_current_scope`.
6. `row_count_total` MUST equal `rows.length`.
7. `reveal_hidden_results_policy_admin_only` is admissible only
   on a policy-admin surface and only when the underlying
   `workset_artifact.policy_limitation.hidden_member_list_visible
   = true`.

### 3.2 Reserved result-state classes

Per the M00-503 spec, every cross-repo row resolves to one of
seven `result_state_class` values. The five reserved classes are
first-class:

- `hidden_by_workset` — row exists but is excluded by the active
  workset's roots / patterns. The group reports its count under
  `hidden_count_summary.hidden_by_workset_count`.
- `outside_current_scope` — row sits outside the active scope.
  Marker is visible (`outside_scope_marker_visible = true`),
  opening drops the user into a scope-diff review sheet.
- `imported_root` — row is sourced from an imported / accelerator
  index. `imported_root_disclosed = true`. The user is told the
  row is reconstructable but not authoritative.
- `partially_loaded` — row exists by manifest / pointer but is
  not yet fetched, materialized, or hydrated. Maps onto the
  topology honesty labels `not_fetched` / `pointer_only` /
  `shallow_boundary`. Offered actions include
  `fetch_missing_object` or `build_missing_index`.
- `evidence_missing` — row's target identity is known but its
  evidence (citation / snippet / blame) cannot be reconstructed
  from the current index state. AI / cited-explainer surfaces
  use this to disclose a citation-missing row instead of
  dropping the row. `result_truth_class != exact`.

Two additional classes complete the closed vocabulary:

- `in_scope_loaded` — the default for a fully loaded in-scope
  row.
- `policy_blocked` — row is hidden by the active workset's
  policy overlay rather than by its roots / patterns. Forbids
  `open_in_new_pane` and `open_in_current_pane`; reduces
  `offered_actions` to inspection / admin-review variants.

Rules (frozen):

1. `result_state_class = outside_current_scope` requires
   `outside_scope_marker_visible = true` and
   `open_outside_current_scope_with_review` in
   `offered_actions`.
2. `result_state_class = imported_root` requires
   `imported_root_disclosed = true`.
3. `result_state_class = partially_loaded` requires
   `fetch_missing_object` or `build_missing_index` in
   `offered_actions`.
4. `result_state_class = evidence_missing` forbids
   `result_truth_class = exact` and requires
   `preview_evidence` in `offered_actions`.
5. `lane_class` in
   `{ imported_remote_index, imported_provider_index,
   external_provider_index }` MUST set
   `imported_root_disclosed = true`.

### 3.3 `No results` is not the same as `No results in this scope`

Empty result sets resolve through the group-presentation state
vocabulary, not through a generic empty list:

- `loaded_complete` with `rows = []` and every hidden count = 0
  → `No results`.
- `loaded_partial` or `outside_current_scope_disclosed` with at
  least one excluded row → `No results in this workset`. The
  group exposes `widen_with_review` and quotes the excluded-row
  count.
- `policy_overlay_disclosed` with `hidden_by_policy_count > 0`
  → `Blocked by trust or policy`. The group quotes the policy
  reference and exposes
  `reveal_hidden_results_policy_admin_only` only on a
  policy-admin surface.
- `loaded_partial` with `not_loaded_count > 0` and lane drawn
  from imported / partial-clone / sparse-checkout sources
  → `Index not built for excluded roots`. The group exposes
  `build_missing_indexes` or `fetch_missing_objects`.

A group that renders `loaded_complete` while at least one
hidden-row count is positive is non-conforming.

## 4. How each surface uses the contract

### 4.1 Workset switcher (open flow, command palette)

- The picker reads one `workset_switcher_record`. The active row
  is the one whose `workset_ref` matches
  `active_workset_ref`.
- Opening a row is the `open_workset` action; opening into a
  new pane is `open_in_new_pane`. Renaming, duplicating, and
  deleting route through their named actions and resolve
  against the underlying workset artifact (which emits a
  `scope_widen_diff_record` for non-presentation changes).
- The switcher is a read-only projection of the durable
  artifact set. A row that disagrees with its underlying
  `workset_artifact_record` (different `scope_class`,
  different `source_class`, different `readiness_state`) is
  non-conforming and is rebuilt before render.

### 4.2 Scope banner (search, navigation, AI panel, review)

- The banner activates against the active workset on every
  surface that depends on scope. Search, symbol navigation,
  AI context, and review-side semantic hints all read the
  same banner.
- Widening through the banner emits a
  `scope_widen_diff_record` (workset_artifact.schema.json) and
  flips the banner state to `active_widened` until the user
  confirms the diff or cancels.
- The banner cooperates with the search planner: while the
  planner is still warming, `banner_state = active_warming`;
  once warming completes the state resolves to either
  `active_narrow_safe` (no hidden rows known) or
  `active_partial` (hidden rows known).

### 4.3 Cross-repo result groups (search, symbol jump, topology, impact, explainer, AI inspector)

- Search results render one group per `root_ref`. Cross-repo
  result grouping is the difference between
  `Found 23 results` (one synthetic count across roots) and
  `Found 23 results across 3 repos · 4 outside the current
  scope · 2 imported`.
- Symbol jump targets that resolve outside the active workset
  emit a row with `result_state_class = outside_current_scope`
  and `open_outside_current_scope_with_review` in
  `offered_actions`. Silent jumps across worksets are
  forbidden.
- Topology maps cite the group through `topology_reuse.graph_node_ref`.
  The map's node tooltip is required to preserve the same
  `in_scope_marker`, `imported_root_disclosed`, and
  `hidden_count_summary` the group carries; the topology
  surface never minted a parallel scope vocabulary.
- Impact explorers cite the group through
  `topology_reuse.impact_root_ref`. An impact-explorer row that
  drops the outside-scope marker is non-conforming.
- Cited explainers and AI context inspectors cite the group
  through `topology_reuse.explainer_evidence_ref` and the row's
  `evidence_segment_ref`. A cited evidence card whose source
  row is `outside_current_scope` MUST disclose that.
- Review-side semantic hint surfaces (e.g. PR review hovers)
  read the same group + the same `consumer_surface_class`
  flag. They never collapse imported-root provenance behind a
  generic "remote index" badge.

## 5. Topology / impact / cited-explainer reuse

The `topology_reuse` block on every `cross_repo_result_group_record`
makes graph-backed scope truth reusable across surfaces. The
block carries:

- `reuse_surface_class[]` — surface families that may read this
  group. The seed includes `topology_map`, `impact_explorer`,
  `cited_explainer`, `ai_context_inspector`,
  `review_semantic_hint`, `support_export`, and
  `navigation_deep_link`.
- `preserve_caveats[]` — closed list of caveats the reusing
  surface MUST keep when it re-renders the group:
  `outside_current_scope`, `imported_root`,
  `hidden_by_workset_count`, `hidden_by_policy_count`,
  `not_loaded_count`, `evidence_missing_count`,
  `policy_overlay_layered`, `stale_disclosure`,
  `warming_disclosure`.
- `graph_node_ref` — pointer to the topology graph node the
  group corresponds to (when applicable).
- `impact_root_ref` — pointer to the impact-explorer reason
  root (when applicable).
- `explainer_evidence_ref` — pointer to the cited explainer
  evidence card (when applicable).

Rules (frozen):

1. A topology / impact / explainer surface that reads a
   group through `topology_reuse` MUST preserve every caveat
   in `preserve_caveats`. Dropping a caveat is non-conforming.
2. A surface that minted a parallel scope vocabulary (its own
   in-scope flag, its own freshness class, its own hidden-row
   count format) is non-conforming. The group is the
   authority.
3. Hidden-result counts, imported-root markers, and
   out-of-workset disclosures survive every cross-surface jump
   in the same query session: a search row → topology node →
   impact row → cited evidence chain re-renders the same
   counts and markers.

## 6. Schema and contract lifecycle

- Adding a new `switcher_row_class`, `banner_state`,
  `switcher_action`, `banner_action`, `in_scope_marker`,
  `result_state_class`, `lane_class`, `consumer_surface_class`,
  or `group_presentation_state` is additive-minor and bumps
  `workset_switcher_schema_version` or
  `cross_repo_result_group_schema_version`.
- Repurposing an existing enum value is breaking and requires
  a new decision row.
- Removing a `consumer_surface_class` is breaking; surfaces in
  the seed family may not be silently unhooked.
- The Rust types in the eventual workspace-shell, search, and
  graph crates are the schema of record once they land; this
  file is the cross-tool boundary for diagnostics, support,
  portability review, and later AI / refactor / export
  tooling.

## 7. References

- PRD §5 workset / monorepo language.
- TAD §7.6 workset scope object, sparse-scope descriptor,
  workset switcher / scope banner.
- TAD §12.6 partial-truth labels (re-exported here as
  `partial_truth_label`).
- TDD §5 scope widening denial, scope-widening drills,
  workset-scope descriptor.
- UI/UX Spec §12.7 topology maps, impact explorer, codebase
  explainer.
- UI/UX Spec §12.8 scope review, workset portability,
  cross-repo jump guarantees.
- UI/UX Spec Appendix AC.1 workset switcher row.
- UI/UX Spec Appendix AC.2 scope banner.
- UI/UX Spec Appendix AC.3 cross-repo result group checklist.
- UI/UX Spec Appendix AC.4 scope-diff review sheet.
- UX Style Guide scope-banner and cross-repo result group
  templates.
- Milestones document §6.25 execution-context, workset/scope,
  and recovery-ladder contract.
- Named-workset artifact, scope-truth chip, and scope-widen
  diff packet
  ([`scope_truth_packet.md`](scope_truth_packet.md)).
- Repository-topology edge-case truth contract
  ([`repository_topology_edge_case_contract.md`](repository_topology_edge_case_contract.md)).
- Unified search query-planner, shard topology, and result-
  fusion contract
  ([`/docs/search/query_planner_contract_seed.md`](../search/query_planner_contract_seed.md)).
- ADR-0009 execution-context and runtime workset_scope union
  ([`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)).
