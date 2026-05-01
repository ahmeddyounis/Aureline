# Scope-diff review, widen-cost disclosure, and cross-repo jump continuity contract

This document freezes the cross-surface object model every Aureline
search, navigation, symbol-jump, topology, impact, AI-context,
refactor, and support-export surface uses when it raises a
**scope-diff review sheet** before widening the active scope and when
it emits a **cross-repo jump event** that hops out of one repo / root
into another.

Scope is a first-class object (frozen in
[`scope_truth_packet.md`](./scope_truth_packet.md)). The workset
switcher, scope banner, and cross-repo result group projecting that
object are frozen in
[`workset_scope_and_cross_repo_contract.md`](./workset_scope_and_cross_repo_contract.md).
This contract is the seed for two further surfaces those foundations
feed:

1. the **scope-diff review sheet** — the surface that renders the
   typed list of added / removed roots and modules, the expected
   index and runtime cost, the remote / cache source-note rows,
   the support / portability / readiness impact, the trust-stage
   and policy-overlay impact, and the closed confirm / cancel /
   trust-review / policy-review action set every search,
   navigation, refactor, AI-context, or support-export widening
   passes through;
2. the **cross-repo jump event** — the typed record every search
   row, symbol jump, topology neighbourhood hop, impact-explorer
   row, cited-explainer evidence card, AI-context excerpt,
   review-side semantic hint, navigation deep link, breadcrumb,
   and command-palette jump emits when the user navigates from
   one repo / root into another (or peeks / previews / splits
   into one) so source repo identity, destination repo identity,
   path / symbol anchor, new-pane / split / peek choice, and
   Back-navigation continuity all survive the hop.

The machine-readable schemas live at:

- [`/schemas/workspace/scope_diff_review.schema.json`](../../schemas/workspace/scope_diff_review.schema.json)
- [`/schemas/workspace/cross_repo_jump_event.schema.json`](../../schemas/workspace/cross_repo_jump_event.schema.json)

The companion fixtures live under:

- [`/fixtures/workspace/scope_widening_cases/`](../../fixtures/workspace/scope_widening_cases/)

The durable workset artifact and the typed `scope_widen_diff_record`
this contract reads as evidence live in:

- [`/schemas/workspace/workset_artifact.schema.json`](../../schemas/workspace/workset_artifact.schema.json)
- [`scope_truth_packet.md`](./scope_truth_packet.md)

The workset switcher, scope banner, and cross-repo result group
contract this contract layers on top of lives in:

- [`/schemas/workspace/workset_switcher.schema.json`](../../schemas/workspace/workset_switcher.schema.json)
- [`/schemas/workspace/cross_repo_result_group.schema.json`](../../schemas/workspace/cross_repo_result_group.schema.json)
- [`workset_scope_and_cross_repo_contract.md`](./workset_scope_and_cross_repo_contract.md)

The repository-topology truth contract that backs imported-root,
not-fetched, pointer-only, and shallow-boundary disclosures lives in:

- [`/schemas/workspace/repo_topology_state.schema.json`](../../schemas/workspace/repo_topology_state.schema.json)
- [`repository_topology_edge_case_contract.md`](./repository_topology_edge_case_contract.md)

This contract is normative. Where it disagrees with the PRD, TAD,
TDD, or UI/UX Spec quotations cited in §7, those documents win and
this document MUST be updated in the same change. Where this
document disagrees with a downstream search widen-with-review sheet,
refactor scope footer, AI context inspector widen prompt, support
export widen prompt, symbol-jump landing card, topology / impact
hop, cited-explainer evidence-card jump, navigation deep-link
landing, breadcrumb hop, or command-palette jump that mints its own
widen-cost or back-navigation vocabulary, this document wins and the
surface is non-conforming.

## Why freeze this now

A search that silently widens past the active workset because
"results were available", a refactor that quietly imported a
managed-provider root, a symbol jump that left the user in a
different repo without naming it, a peek that swapped the active
workset, an AI-context inspector that pulled an evidence segment
from outside scope without disclosing it, a Back action that
collapsed to "previous file" and lost the originating search row —
every one of those is a continuity-flatten incident. Each surface
that re-mints its own widen-cost label, source-note row, or
back-navigation kind compounds the problem.

Freezing the scope-diff review-sheet and cross-repo jump-event
contract here means:

- a banner widen, a search "widen with review", a symbol-jump widen,
  a topology / impact / explainer widen, an AI-context widen, a
  refactor widen, a support-export widen, and a navigation deep-link
  widen all read one closed `trigger_class` family and one closed
  `review_action` family;
- the user always sees the typed list of roots / modules added or
  removed, the typed `expected_index_cost_class`, the typed
  `expected_runtime_cost_class`, the typed remote / cache
  `source_class` rows, and the typed support / portability /
  readiness impact before scope changes;
- a cross-repo jump never flattens into an anonymous file open: the
  destination always carries `repo_label`, `root_ref`,
  `workset_ref`, `scope_class`, `anchor_kind`, and an
  `in_scope_marker`, and Back is a typed action whose label class
  preserves the origin surface;
- widening into a managed-provider root, a license/export-controlled
  root, or a scope at a higher trust stage is blocked at the review
  surface, not silently allowed downstream.

## Scope

This contract freezes two record kinds across two schemas:

1. `scope_diff_review_record` — the user-visible review-sheet
   projection for one widening (or narrowing) decision turn. Reads
   the underlying `scope_widen_diff_record` and projects added /
   removed roots, added / removed modules, pattern-change summary,
   expected index and runtime cost, remote / cache source notes,
   support / availability impact, trust and policy impact, and the
   closed action set.
2. `cross_repo_jump_event_record` — one navigation hop with typed
   source / destination anchor blocks, jump kind, jump origin class,
   workset-continuity class, optional scope-diff review reference,
   trust and policy impact, and the back-navigation continuity
   block.

Out of scope:

- Implementing cross-repo refactor engines, topology / impact
  propagation engines, or graph-walk engines. The schemas only
  freeze the vocabulary those engines emit.
- Final copy / microcopy. The closed label families freeze the
  vocabulary; the rendered strings live with the shell
  interaction-safety contract.
- Sparse-checkout / partial-clone / submodule / LFS hydration truth
  itself — that lives in
  [`repository_topology_edge_case_contract.md`](./repository_topology_edge_case_contract.md).
  This contract reuses its honesty labels by reference.
- Search planner stages, ranking, or shard topology — those live in
  [`/docs/search/query_planner_contract_seed.md`](../search/query_planner_contract_seed.md);
  this contract reuses its freshness / readiness / lane vocabulary.

## 1. Scope-diff review sheet

Every widening is gated on a `scope_diff_review_record`. The record
quotes the underlying `scope_widen_diff_record` through `diff_ref`;
the review surface NEVER mints a parallel diff id.

### 1.1 Trigger class

The closed trigger-class family:

- `banner_widen` — the scope banner offered widen and the user
  accepted.
- `switcher_open` — opening a workset switcher row that resolves to
  a different `workset_id`.
- `search_widen_with_review` — the search "widen with review"
  affordance.
- `cross_repo_jump_widen` — a cross-repo jump whose destination is
  outside the active scope.
- `symbol_jump_widen` — a symbol jump whose target is outside the
  active scope.
- `topology_widen` — a topology-map neighbourhood hop that crosses
  the active scope boundary.
- `impact_explorer_widen` — an impact-explorer row whose target sits
  outside the active scope.
- `cited_explainer_widen` — a cited-explainer evidence card whose
  evidence sits outside the active scope.
- `ai_context_widen` — an AI-context inspector pulling evidence from
  outside the active scope.
- `refactor_widen` — a refactor preview whose change set exceeds the
  active scope.
- `review_semantic_hint_widen` — a review-side semantic hint whose
  evidence sits outside the active scope.
- `support_export_widen` — a support-export bundle whose scope
  exceeds the active workset.
- `navigation_deep_link_widen` — a navigation deep link that lands
  outside the active scope.

Rules (frozen):

1. A surface that widens scope without raising one of the listed
   trigger classes is non-conforming.
2. The trigger class always corresponds to one of the listed
   `consumer_surface_class` values; the review record names both.

### 1.2 Required fields

Every review record carries:

- `review_id` — stable opaque id.
- `diff_ref` — pointer to the underlying `scope_widen_diff_record`.
- `base_workset_ref` / `candidate_workset_ref` — pointers to the
  active and candidate `workset_artifact_record`s. They MUST differ;
  identity never drifts under a review.
- `base_scope_class` / `candidate_scope_class` — re-export of
  `workset_artifact.scope_class`.
- `trigger_class`, `consumer_surface_class[]`.
- `review_state`.
- `added_roots[]`, `removed_roots[]`, `added_modules[]`,
  `removed_modules[]`, `pattern_change_summary`.
- `expected_index_cost_class`, `expected_runtime_cost_class`,
  `expected_runtime_cost_note`.
- `remote_fetch_required`, `remote_or_cache_source_notes[]`.
- `support_availability_impact`.
- `trust_stage_impact_class`, `policy_impact_class`.
- `offered_actions[]`.

### 1.3 Review state

The closed review-state vocabulary:

- `pending_user_review` — sheet rendered, no input yet.
- `in_review` — user is interacting with the sheet.
- `confirmed` — user confirmed widening / narrowing; the candidate
  has not yet activated.
- `cancelled` — user dismissed.
- `blocked_by_trust` — a continuity rule (trust-stage impact) blocks
  the candidate.
- `blocked_by_policy` — a continuity rule (policy-overlay impact)
  blocks the candidate.
- `blocked_by_unavailable_remote` — remote fetch is required but
  the source is unreachable.
- `applied` — the candidate workset is now active.

Rules (frozen):

1. `review_state` in
   `{ blocked_by_trust, blocked_by_policy, blocked_by_unavailable_remote }`
   forbids `confirm_widen` and `confirm_narrow` in `offered_actions`.
2. `trust_stage_impact_class = blocked_by_active_trust_stage` forces
   `review_state = blocked_by_trust`.
3. `policy_impact_class` in
   `{ blocked_by_admin_policy, blocked_by_license_or_export_control }`
   forces `review_state = blocked_by_policy`.
4. The `presentation_only` path on `scope_widen_diff_record` (rename,
   subtitle edit, note edit) NEVER emits a review record; it routes
   through the rename / subtitle path on `workset_artifact`.

### 1.4 Added / removed roots and modules

`added_roots[]` and `removed_roots[]` enumerate one entry per root
added or removed by the candidate. Every entry carries `root_ref`,
`root_label`, `change_class` in `{ root_added, root_removed }`,
`imported_root_disclosed`, and `managed_provider_disclosed`.

`added_modules[]` and `removed_modules[]` enumerate one entry per
folder / module / manifest-entry / graph-seed change. Every entry
carries `module_ref`, `module_label`, `module_kind`, and
`change_class` in `{ module_added, module_removed, pattern_broadened, pattern_narrowed }`.
A module-level entry MAY name its containing root through
`in_root_ref`.

`pattern_change_summary` carries the aggregate counts
(`broadened_count`, `narrowed_count`, `conflicting_pattern_count`).
The verbatim pattern bodies live on the underlying
`scope_widen_diff_record.entries`.

Rules (frozen):

1. A review whose underlying diff has `widens_scope = true` MUST
   contain at least one signal across `added_roots`, `added_modules`,
   or `pattern_change_summary.broadened_count > 0`.
2. A review whose underlying diff has `narrows_scope = true` MUST
   contain at least one signal across `removed_roots`,
   `removed_modules`, or `pattern_change_summary.narrowed_count > 0`.
3. A review with no diff signal across all of those is non-conforming.

### 1.5 Expected index and runtime cost

The review sheet renders two typed cost bands:

- `expected_index_cost_class` (re-exported from `scope_widen_diff_record`):
  `none` / `cache_warm` / `targeted_index` / `full_reindex` /
  `remote_fetch_required`.
- `expected_runtime_cost_class`:
  `none` / `cheap` / `moderate` / `expensive` /
  `very_expensive_remote_only`.

`expected_runtime_cost_note` is an optional redaction-aware short
note. The review sheet NEVER renders a precise time estimate; the
typed band is the authoritative cost cue.

Rules (frozen):

1. `expected_index_cost_class = remote_fetch_required` requires
   `remote_fetch_required = true`.
2. `expected_runtime_cost_class = very_expensive_remote_only`
   requires `remote_fetch_required = true` and at least one
   `remote_or_cache_source_notes` entry whose
   `remote_round_trip_disclosed = true`.

### 1.6 Remote / cache source notes

`remote_or_cache_source_notes[]` enumerates the cache and remote
sources widening will read from. The closed `source_class` family:

- `workspace_local_cache`
- `profile_local_cache`
- `machine_local_cache`
- `remote_authoritative_required`
- `remote_authoritative_optional`
- `imported_remote_index`
- `imported_provider_index`
- `external_provider_index`
- `accelerator_index`
- `no_external_source_required`

Every entry carries `source_class`, an optional redaction-aware
`source_label`, and `remote_round_trip_disclosed`.

Rules (frozen):

1. Every entry whose `source_class` is in
   `{ remote_authoritative_required, imported_remote_index, imported_provider_index, external_provider_index }`
   MUST set `remote_round_trip_disclosed = true`.
2. `remote_fetch_required = true` requires at least one such entry.
3. A review with zero entries is non-conforming; widening that reads
   only local caches MUST emit one entry with
   `source_class = no_external_source_required`.

### 1.7 Support / availability impact

`support_availability_impact` is a typed block carrying:

- `changes_portability` — true when widening flips
  `workset_artifact.portability.portability_class`.
- `changes_readiness` — true when widening flips
  `workset_artifact.readiness.readiness_state`.
- `includes_managed_provider_refs_after_widen` — true when at least
  one root added is managed-provider-locked.
- `includes_machine_local_refs_after_widen` — true when at least one
  root added points at a machine-local device / drive / virtual
  root.
- `reduces_support_export_completeness` — true when the resulting
  bundle would not be exportable in full.
- `reduces_offline_capability` — true when widening introduces a
  remote-only root or remote-only index.
- `support_availability_impact_disclosed` — boolean. MUST be true
  when any other field on the block is true.

Rules (frozen):

1. `support_availability_impact_disclosed = false` while any other
   field on the block is true is non-conforming.
2. A review that confirms widening into a managed-provider root
   without setting `includes_managed_provider_refs_after_widen = true`
   is non-conforming.

### 1.8 Trust-stage and policy-overlay impact

Closed `trust_stage_impact_class`:

- `no_change`
- `requires_trust_review`
- `requires_trust_uplift`
- `blocked_by_active_trust_stage`

Closed `policy_impact_class`:

- `no_change`
- `layered_policy_overlay_added`
- `policy_overlay_removed`
- `blocked_by_admin_policy`
- `blocked_by_license_or_export_control`

Rules (frozen):

1. `trust_stage_impact_class` in
   `{ requires_trust_review, requires_trust_uplift }` MUST include
   `request_trust_review` in `offered_actions`.
2. `policy_impact_class = blocked_by_admin_policy` MAY include
   `request_policy_review_admin_only` only on a policy-admin
   surface.
3. Trust- and policy-blocked reviews force the corresponding
   `review_state` (see §1.3).

### 1.9 Action set

The closed `review_action` vocabulary:

- `confirm_widen`
- `confirm_narrow`
- `cancel_widen_keep_current_scope`
- `request_trust_review`
- `request_policy_review_admin_only`
- `snapshot_current_scope`
- `copy_diff_id`
- `open_underlying_artifact`
- `narrow_to_current_repo`
- `build_missing_indexes`
- `fetch_missing_objects`

Rules (frozen):

1. At least one cancel route MUST be present in `offered_actions`
   (`cancel_widen_keep_current_scope` or `narrow_to_current_repo`).
2. `confirm_widen` and `confirm_narrow` are mutually exclusive on a
   single review record.
3. Blocked reviews reduce `offered_actions` to
   `cancel_widen_keep_current_scope` plus the relevant escalation
   (`request_trust_review` and/or `request_policy_review_admin_only`).

## 2. Cross-repo jump event

Every navigation hop that crosses (or peeks across) a repo / root
boundary emits a `cross_repo_jump_event_record`. The record quotes
the source side and the destination side as typed `anchor_block`
records; both sides preserve `workset_ref`, `root_ref`, `repo_label`,
`scope_class`, `anchor_kind`, and `presentation_label`.

### 2.1 Anchor block

Every `anchor_block` carries:

- `workset_ref` — non-null pointer to the
  `workset_artifact_record` active at this side. Cross-repo jumps
  preserve workset identity on both sides even when the destination
  is outside the active scope.
- `root_ref` — pointer to the ADR-0006 filesystem-identity root.
- `repo_label` — redaction-aware repo / root label. Cross-repo jumps
  NEVER drop this field.
- `scope_class`, `anchor_kind`, optional `anchor_ref`,
  `presentation_label`, optional `presentation_subtitle`,
  optional `query_session_ref`,
  optional `in_scope_marker`, `freshness_class`, `readiness_state`,
  and `imported_root_disclosed`.

Rules (frozen):

1. `source.workset_ref` and `destination.workset_ref` MUST both be
   non-null.
2. `is_cross_repo = true` requires `destination.repo_label` to be
   present and to differ from `source.repo_label` AND requires
   `destination.in_scope_marker` to be non-null.
3. `destination.in_scope_marker = imported_root` requires
   `destination.imported_root_disclosed = true` and
   `destination.freshness_class` in `{ imported_only, warm_cached }`.

### 2.2 Jump origin class

The closed `jump_origin_class` family:

- `search_result_row`
- `symbol_jump`
- `topology_neighbourhood`
- `impact_explorer_row`
- `cited_explainer_evidence`
- `ai_context_inspector_excerpt`
- `navigation_deep_link`
- `review_semantic_hint`
- `breadcrumb_navigation`
- `command_palette_jump`
- `back_navigation`

`back_navigation` is the explicit class for a Back hop that returns
into a previously visited anchor. Cross-repo jumps NEVER mint a
parallel "back" surface label.

### 2.3 Jump kind

The closed `jump_kind` family:

- `open_in_current_pane`
- `open_in_new_pane`
- `open_in_split`
- `peek_inline`
- `preview_modal`
- `open_in_navigation_deep_link`

`peek_inline` and `preview_modal` are non-committing presentations:
they do not switch the active workset and do not require a
scope-diff review even when the destination sits outside the
current scope.

### 2.4 Workset-continuity class

The closed `workset_continuity_class` family:

- `same_workset` — `source.workset_ref == destination.workset_ref`
  and the destination is in scope.
- `widened_with_review` — the user confirmed a scope-diff review
  and the candidate workset is now active.
- `peeked_outside_scope` — non-committing peek / preview of an
  outside-scope destination.
- `switched_active_workset_with_review` — the jump activated a
  different existing workset after a review.
- `blocked_by_active_workset_boundary` — the jump was denied by a
  continuity rule (trust, policy, or scope boundary).

Rules (frozen):

1. `destination.in_scope_marker = outside_current_scope` MUST set
   `workset_continuity_class` to one of `peeked_outside_scope`,
   `widened_with_review`, `switched_active_workset_with_review`, or
   `blocked_by_active_workset_boundary`.
2. `workset_continuity_class` in
   `{ widened_with_review, switched_active_workset_with_review }`
   MUST set `scope_diff_review_ref` to a non-null pointer.
3. `workset_continuity_class = same_workset` requires
   `source.workset_ref == destination.workset_ref` AND
   `destination.in_scope_marker` in
   `{ in_current_scope, policy_overlay_layered }`.
4. `workset_continuity_class = peeked_outside_scope` MUST set
   `jump_kind` to `peek_inline` or `preview_modal`.
5. `workset_continuity_class = blocked_by_active_workset_boundary`
   forbids `confirm_jump_in_current_pane`,
   `confirm_jump_in_new_pane`, `confirm_jump_in_split`,
   `confirm_peek_inline`, and `confirm_preview_modal` in
   `offered_actions`, and requires `cancel_jump`.
6. `trust_stage_impact_class = blocked_by_active_trust_stage` forces
   `workset_continuity_class = blocked_by_active_workset_boundary`.
7. `policy_impact_class` in
   `{ blocked_by_admin_policy, blocked_by_license_or_export_control }`
   forces `workset_continuity_class = blocked_by_active_workset_boundary`.

### 2.5 Back navigation continuity

Every record carries a `back_navigation` block:

- `back_target_kind` — closed family covering
  `prior_pane_focus`, `prior_search_result_row`,
  `prior_topology_node`, `prior_impact_row`, `prior_evidence_card`,
  `prior_command_palette_state`, `prior_navigation_anchor`,
  `prior_workset_switcher_row`, and `no_back_target_available`.
- `back_action_label_class` — closed family covering
  `back_to_search_result`, `back_to_symbol_origin`,
  `back_to_topology_node`, `back_to_impact_row`,
  `back_to_evidence_card`, `back_to_navigation_anchor`,
  `back_to_workset_switcher`, `back_to_command_palette`,
  `back_to_prior_pane`, and `no_back_action_available`.
- `preserves_source_workset_ref` (boolean).
- `preserves_source_query_session_ref` (boolean).
- `preserves_source_anchor_ref` (boolean).
- `can_back_to_origin` (boolean).
- optional `back_pane_target_kind` covering `back_in_same_pane`,
  `back_to_origin_pane`, `back_closes_split`, `back_closes_peek`,
  `back_closes_preview`.

Rules (frozen):

1. `preserves_source_workset_ref` MUST be true unless
   `workset_continuity_class = switched_active_workset_with_review`.
2. `preserves_source_query_session_ref` MUST be true unless
   `workset_continuity_class = blocked_by_active_workset_boundary`.
3. `jump_kind` in
   `{ open_in_split, open_in_new_pane, peek_inline, preview_modal }`
   requires `preserves_source_anchor_ref = true`.
4. `can_back_to_origin = false` requires
   `back_target_kind = no_back_target_available` AND
   `back_action_label_class = no_back_action_available`.
5. A cross-repo jump that lands the user without a typed Back label
   (rendered as a generic "Back" or as a flat "previous file") is
   non-conforming.

### 2.6 Action set

The closed `jump_action` vocabulary:

- `confirm_jump_in_current_pane`
- `confirm_jump_in_new_pane`
- `confirm_jump_in_split`
- `confirm_peek_inline`
- `confirm_preview_modal`
- `cancel_jump`
- `open_scope_diff`
- `switch_active_workset_with_review`
- `request_trust_review`
- `request_policy_review_admin_only`
- `fetch_missing_object`
- `build_missing_index`
- `copy_target_ref`
- `copy_workset_id`

## 3. Continuity rules (cross-cutting)

The two records together freeze the continuity rules every widening
or jumping surface MUST satisfy:

1. **Sparse boundaries are never silently overridden.** A jump
   whose destination falls outside the active workset's roots /
   patterns either resolves through `peeked_outside_scope` (no
   scope change), `widened_with_review` (scope change after a
   `scope_diff_review_record`), or `blocked_by_active_workset_boundary`
   (denied). The fourth path — silent override — is non-conforming.
2. **Trust stage is never silently uplifted.** A widening or jump
   whose candidate scope requires a higher trust stage than the
   active stage routes through `requires_trust_review` /
   `requires_trust_uplift` (review surface) or
   `blocked_by_active_trust_stage` (denial). It NEVER activates a
   higher trust stage as a side effect of widening / jumping.
3. **Active workset identity is never silently swapped.** A jump
   that activates a different existing workset routes through
   `switched_active_workset_with_review` and carries a non-null
   `scope_diff_review_ref`. A jump that the user expected to
   remain inside the active workset NEVER swaps it.
4. **Cross-repo jumps preserve repo identity.** Every cross-repo
   jump carries `destination.repo_label`, `destination.root_ref`,
   `destination.workset_ref`, and a typed `anchor_kind`. The
   destination is never rendered as an anonymous file open.
5. **Back is a typed action.** `back_navigation.back_action_label_class`
   names the origin surface; `preserves_source_workset_ref`,
   `preserves_source_query_session_ref`, and
   `preserves_source_anchor_ref` are the explicit continuity flags
   the surface MUST honour. A surface that flattens Back into a
   generic label that erases the origin is non-conforming.
6. **Imported-root destinations are visibly disclosed.** A
   destination whose `in_scope_marker = imported_root` carries
   `imported_root_disclosed = true` and a `freshness_class` in
   `{ imported_only, warm_cached }`. The user is told the row is
   reconstructable but not authoritative.
7. **Costed widenings name their cost.** A confirmed widening
   whose `expected_index_cost_class = full_reindex` or
   `expected_runtime_cost_class = expensive` /
   `very_expensive_remote_only` always names that on the review
   sheet. Silent reindexing or silent remote fetches are
   non-conforming.

## 4. How each surface uses the contract

### 4.1 Scope banner / workset switcher

- The banner's `widen_with_review` action and the switcher's
  `open_scope_diff` action both raise a `scope_diff_review_record`
  with `trigger_class = banner_widen` or `switcher_open`.
- Confirming the review activates the candidate workset; cancelling
  keeps the active workset unchanged.

### 4.2 Search, symbol jump, command palette

- A search row's `widen_with_review` raises a review with
  `trigger_class = search_widen_with_review`.
- A symbol jump whose target is outside scope raises a
  `cross_repo_jump_event_record`. If the user accepts widening, the
  event references the review through `scope_diff_review_ref` and
  resolves through `widened_with_review`.
- Command-palette jumps resolve through
  `jump_origin_class = command_palette_jump`.

### 4.3 Topology, impact, cited explainer, AI context inspector

- A topology hop, impact-explorer row, cited-explainer evidence
  card, or AI-context excerpt that crosses scope raises both
  records: a review for the scope change and a jump event for the
  navigation hop. Reuse pointers
  (`cross_repo_result_group.topology_reuse.graph_node_ref`,
  `impact_root_ref`, `explainer_evidence_ref`) survive on the jump
  event through `result_group_ref` / `result_row_ref`.

### 4.4 Refactor, support export, navigation deep link

- A refactor preview whose change set exceeds the active scope
  raises a review with `trigger_class = refactor_widen`. Cancelling
  keeps the refactor scoped to the active workset.
- A support export that bundles roots beyond the active workset
  raises a review with `trigger_class = support_export_widen`. The
  review's `support_availability_impact` block is the contract for
  what the export bundle will and will not include.
- A navigation deep link that lands outside the active scope raises
  both records: a review (`navigation_deep_link_widen`) and a jump
  event (`jump_origin_class = navigation_deep_link`,
  `jump_kind = open_in_navigation_deep_link`).

### 4.5 Review-side semantic hints

- A PR review hover that pulls evidence from outside the active
  scope raises a review with
  `trigger_class = review_semantic_hint_widen`. Confirmation never
  activates a different workset; the review records that scope was
  widened explicitly for evidence preview.

## 5. Schema and contract lifecycle

- Adding a new `trigger_class`, `review_state`, `review_action`,
  `expected_runtime_cost_class`, `remote_or_cache_source_class`,
  `trust_stage_impact_class`, `policy_impact_class`,
  `jump_origin_class`, `jump_kind`, `workset_continuity_class`,
  `back_target_kind`, `back_action_label_class`, or
  `consumer_surface_class` is additive-minor and bumps
  `scope_diff_review_schema_version` or
  `cross_repo_jump_event_schema_version`.
- Repurposing an existing enum value is breaking and requires a
  new decision row.
- Removing a `consumer_surface_class` is breaking; surfaces in the
  seed family may not be silently unhooked.
- The Rust types in the eventual workspace-shell, search, graph,
  refactor, and AI-context crates are the schema of record once
  they land; this file is the cross-tool boundary for diagnostics,
  support, portability review, and later AI / refactor / export
  tooling.

## 6. Forbidden collapses

The following are explicitly non-conforming. The contract names them
so retrofits can be denied at review:

- A widening that activates a candidate workset without a
  `scope_diff_review_record` (every widening is explicit and
  costed).
- A review record that confirms widening while
  `review_state` is `blocked_by_trust`, `blocked_by_policy`, or
  `blocked_by_unavailable_remote`.
- A review record whose `support_availability_impact` block hides a
  managed-provider, machine-local, or readiness regression behind
  `support_availability_impact_disclosed = false`.
- A review record whose remote-fetch path is hidden (no
  `remote_or_cache_source_notes` entry with
  `remote_round_trip_disclosed = true` while
  `remote_fetch_required = true`).
- A review record with no diff signal (no added / removed roots, no
  added / removed modules, and no pattern changes).
- A cross-repo jump that drops `destination.repo_label`,
  `destination.root_ref`, or `destination.workset_ref`.
- A cross-repo jump whose destination is outside the active scope
  but whose `workset_continuity_class = same_workset`.
- A cross-repo jump whose destination is outside the active scope
  but that activates a candidate workset without a
  `scope_diff_review_ref`.
- A jump record whose Back affordance is a generic "Back" (no
  typed `back_action_label_class`) on a surface that does have a
  back target.
- A jump record whose `back_navigation.preserves_source_query_session_ref = false`
  on a non-blocked jump (the source query session always survives
  a cross-repo hop).
- A peek / preview that swaps the active workset (peek and preview
  are non-committing by definition).
- An imported-root destination whose `imported_root_disclosed = false`
  or whose `freshness_class` is not in `{ imported_only, warm_cached }`.

## 7. References

- PRD §5 workset / monorepo language; PRD §6 scope-widening drills.
- TAD §7.6 workset scope object; TAD §12.6 partial-truth labels.
- TDD §5 scope widening denial, scope-widening drills,
  workset-scope descriptor.
- UI/UX Spec §12.7 topology maps, impact explorer, codebase
  explainer.
- UI/UX Spec §12.8 scope review, workset portability, cross-repo
  jump guarantees.
- UI/UX Spec Appendix AC.4 scope-diff review sheet.
- UI/UX Spec Appendix AC.5 cross-repo jump back-navigation.
- UX Style Guide scope-banner and cross-repo result group templates.
- Milestones document §6.25 execution-context, workset/scope, and
  recovery-ladder contract.
- Named-workset artifact, scope-truth chip, and scope-widen diff
  packet ([`scope_truth_packet.md`](./scope_truth_packet.md)).
- Workset switcher, scope banner, and cross-repo result group
  contract
  ([`workset_scope_and_cross_repo_contract.md`](./workset_scope_and_cross_repo_contract.md)).
- Repository-topology edge-case truth contract
  ([`repository_topology_edge_case_contract.md`](./repository_topology_edge_case_contract.md)).
- ADR-0009 execution-context and runtime workset_scope union
  ([`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)).
