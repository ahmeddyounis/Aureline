# Named-workset artifact, scope-truth chip, and scope-widen diff packet

This document freezes the cross-surface object model every Aureline
open, search, support-export, AI-context-inspector, refactor-scope,
and export surface uses when it names **what scope the user is
currently working against**, **how that scope was authored**,
**whether the scope is partial or policy-limited**, **which results
sit outside the current scope**, and **what changes if the scope is
widened**.

Scope is a first-class object, not a surface-local label. A saved
workset is a portable, nameable artifact with a stable id; every
surface reads the same artifact by that id and renders the same
chip contract against the same five scope classes. This packet
lands the vocabulary freeze before the full search, graph,
refactor, review, and support surfaces are implemented, so later
retrofits do not have to reconcile a workset-switcher label, a
search scope banner, a cross-repo result group header, and a
support-packet excerpt that each invented their own scope string.

The machine-readable schema lives at:

- [`/schemas/workspace/workset_artifact.schema.json`](../../schemas/workspace/workset_artifact.schema.json)

The companion fixtures live under:

- [`/fixtures/workspace/workset_examples/`](../../fixtures/workspace/workset_examples/)

The companion chip-contract artifacts live under:

- [`/artifacts/workspace/scope_chip_examples/`](../../artifacts/workspace/scope_chip_examples/)

The runtime projection of a workset — the record a search, graph,
or AI surface activates at query time — lives in the ADR-0009
execution-context schema:

- [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)

The entry, recent-work, and restore-prompt model that hands a
workset off to an open flow lives at:

- [`/docs/workspace/entry_restore_object_model.md`](./entry_restore_object_model.md)

This contract is normative. Where it disagrees with the PRD, TAD,
TDD, or UI/UX spec quotations cited in §7, those documents win and
this document MUST be updated in the same change. Where this
document disagrees with a downstream search-scope banner,
cross-repo result group, support-packet header, or AI-context
inspector's mint of its own scope fields, this document wins and
the surface is non-conforming.

## Why freeze this now

Scope ambiguity is the single most common way a correct-looking
answer becomes a wrong-looking answer. A search result that says
"no matches" when the user expected to see matches in a sibling
repo; a refactor that silently widened past the user's named
workset; an AI context inspector that quoted a file the user
thought was excluded; a support packet that collapsed a sparse
slice into a generic "workspace" label and hid the fact that the
reporter was not looking at what the triager was looking at —
every one of those surfaces turns into a support incident because
scope was not a stable object.

Freezing the artifact, the chip, and the diff record here means:

- open flows render the same `Selected workset` chip the search
  surface will render after open;
- search results say `Current repo`, `Selected workset`, `Full
  workspace`, `Sparse slice`, or `Policy-limited view` — and never
  collapse the last three into a generic `workspace` label;
- cross-repo result groups carry an `Outside current scope` marker
  without the user having to infer it from path prefixes;
- support / export bundles reference the same `workset_id` the
  reporting user saw in the shell, so triage is not guessing;
- later AI, refactor, and review surfaces attach to the same
  artifact id instead of inventing `ai_context_scope`,
  `refactor_scope`, or `export_scope` parallel stable ids.

## Scope

This packet freezes four record kinds inside one schema:

1. `workset_artifact_record` — the durable, portable, nameable
   artifact. One artifact per stable `workset_id` and `scope_id`.
   Includes sparse/full mode, include/exclude patterns, root refs,
   root-kind/result-state rows, membership policy, resolved member
   refs, portability metadata, readiness metadata, and an optional
   policy-limitation block.
2. `scope_truth_chip_record` — the chip projection every open,
   search, support, AI, refactor, and export surface renders. The
   chip quotes `workset_id` and `scope_id`; it never mints a
   parallel scope id.
3. `scope_widen_diff_record` — the typed diff between two
   artifacts (or between an active artifact and a candidate
   widening). Powers the scope-diff review sheet.
4. `workset_scope_consumer_binding` — the non-UI binding that
   proves local, remote, headless, support-export, navigation, and
   refactor consumers preserved the same scope identity or degraded
   with an explicit reason.

Out of scope:

- The full graph-reachability or refactor-propagation
  implementation. Membership policy `dependency_graph_reachable`
  is a seed value here; the graph that resolves it is a later
  milestone.
- The stable multi-root UX. Multi-root chips emit here (root_count
  >= 2); the full multi-root management surface lands later.
- Final copy / microcopy. The label family (`Current repo`,
  `Selected workset`, `Full workspace`, `Sparse slice`,
  `Policy-limited view`, `Outside current scope`) is the frozen
  vocabulary. The rendered strings live with the shell
  interaction-safety contract.
- A new sparse-profile format, VCS-committed workset manifest
  format, or managed-provider workset API. Those land as
  follow-up decisions that reference this artifact by `workset_id`.

## 1. Workset artifact record

Every curated scope — whether user-named, sparse, multi-root,
policy-narrowed, or a single-root fallback — is persisted as one
`workset_artifact_record`. Surfaces read the artifact; they never
derive a parallel stable id for the same scope.

### 1.1 Identity

- `workset_id` — stable opaque id. Never reused after removal.
  Every downstream chip, diff, search packet, support bundle, AI
  context snapshot, refactor preview, and export footer references
  this id.
- `scope_id` — stable opaque scope identity. Restore, remote,
  headless, support-export, navigation, and refactor consumers
  quote this id when they bind the saved scope.
- `workset_name` — human-readable name. Redaction-aware.
- `presentation_subtitle` — optional subtitle (e.g. `Payments
  backend hot path`). Redaction-aware, optional.

Rules (frozen):

1. `workset_id` is write-once. Renaming is a presentation-only
   change that emits a `scope_widen_diff_record` with every entry
   of `diff_class = presentation_only`.
2. A surface that renders a scope label without quoting a
   `workset_id` is non-conforming. The fallback for a bare
   single-root open is still a `workset_artifact_record` with
   `scope_class = current_repo` and one entry in `root_refs`.
3. Deleting a workset removes the record; the id is not reused.
   Support bundles that quote a deleted id MUST mark the chip
   presentation state as `outside_current_scope` with a note.

### 1.2 Scope class

Exactly one per artifact. The set is closed and intentionally
narrower than ADR-0009's runtime `workset_scope` union — this
freeze covers only the five classes the open/search foundations
render on a chip. `review_workspace` and `companion_surface`
activate through execution-context's runtime projection, not
through this artifact.

- `current_repo` — single root, no include/exclude refinement.
  The default for a one-root local open.
- `selected_workset` — user-named workset. Covers one or many
  roots, explicit list / glob / graph-reachable / manifest-driven
  membership.
- `sparse_slice` — a root narrowed by user-authored
  include/exclude patterns. A sparse slice is not a workset
  name; it is a pattern-based narrowing of a single root.
- `full_workspace` — every root in the active workspace.
  Multi-root is represented by `root_refs.length >= 2`.
- `policy_limited_view` — any of the above, narrowed by an
  admin, trust, license, or remote policy. Attaches a
  `policy_limitation` block; the underlying pre-narrowing
  workset is referenced by `underlying_workset_ref`.

Rules (frozen):

1. A single artifact carries exactly one `scope_class`. A surface
   that widens scope swaps to a different artifact (and emits a
   `scope_widen_diff_record`); it does not mutate the class in
   place.
2. `policy_limitation` is required when `scope_class =
   policy_limited_view` and forbidden otherwise.
3. `sparse_slice` carries at least one `include` pattern or at
   least one `exclude` pattern. An empty pattern array on a
   `sparse_slice` is denied with `sparse_slice_requires_patterns`.

### 1.3 Include / exclude patterns and root refs

- `root_refs` — one or more filesystem-root refs. Opaque pointers
  to ADR-0006 filesystem-identity records; raw absolute paths are
  never inlined.
- `scope_mode` — `full` or `sparse`. Full-workspace/current-repo
  artifacts use `full`; workset, sparse, and policy-narrowed
  projections use `sparse` when results are intentionally bounded.
- `included_roots` — one row per `root_refs` entry. Each row
  carries `root_kind` plus the result-state label (`loaded`,
  `manifest_known`, `cached`, or `unavailable`) that search,
  navigation, refactor, export, and support surfaces render.
- `patterns` — ordered include/exclude rows. Order is stable;
  conflicts are surfaced on a diff record, not silently resolved.
- `membership_policy` — one of `explicit_root_list`,
  `glob_pattern`, `dependency_graph_reachable`, `manifest_driven`.
- `member_refs` — resolved members after patterns and membership
  policy are applied. Carries a `partial_truth` label per member.

Rules (frozen):

1. A pattern with `applies_to_root_ref = null` applies to every
   root in `root_refs`. A pattern with a specific root ref applies
   only to that root and is a no-op under every other root.
2. Patterns are carried verbatim. The schema does not re-author
   user patterns. Workspace-relative form is strongly preferred;
   absolute paths should be collapsed before crossing the schema
   boundary.
3. `member_refs` MAY be empty for `dependency_graph_reachable` or
   `manifest_driven` membership when the resolver has not run
   yet. The chip then renders `chip_presentation_state =
   active_partial` with a `partial_index_note`.

### 1.4 Portability

The `portability` block is mandatory. It lets export, import,
sync, and support flows reason about whether the artifact will
survive a round-trip.

- `source_class` — `local_only`, `workspace_shared`,
  `profile_imported`, `managed`, `ephemeral_session`.
- `portability_class` — `fully_portable`,
  `portable_with_rebinding`, `machine_local_only`,
  `managed_provider_locked`.
- `includes_machine_local_refs` — true if any root ref points
  at a machine-local device, drive, or virtual root that will
  not rebind on import.
- `includes_managed_provider_refs` — true if any root ref is
  owned by a managed provider.
- `requires_rebinding_on_import` — true if at least one member
  ref or root ref will need to re-resolve when the artifact
  lands on a different machine or profile.
- `profile_sync_group_ref` — optional sync group id when
  `source_class` is `workspace_shared` or `profile_imported`.

Rules (frozen):

1. `managed` source class MUST carry `portability_class =
   managed_provider_locked`. A managed-shared artifact is not
   exportable outside the provider.
2. `ephemeral_session` source class MUST carry `portability_class
   = machine_local_only` and MUST NOT appear in a persisted
   support/export bundle.
3. An artifact with `includes_machine_local_refs = true` is
   never `fully_portable`; the portability class is at most
   `portable_with_rebinding`.

### 1.5 Readiness and hidden-result accounting

The `readiness` block uses the same vocabulary the UI/UX spec
Appendix AC workset switcher row uses.

- `readiness_state` — `cold`, `warming`, `warm`, `partial`,
  `ready`.
- `hidden_result_count_known` — whether an exact hidden count
  is available.
- `hidden_result_count` — exact integer when known; null
  otherwise.
- `partial_index_note` — short free text; redaction-aware.

Rules (frozen):

1. A chip MAY report `some results may be hidden` without a
   number; it MUST NOT invent a number when
   `hidden_result_count_known = false`.
2. A chip that claims `ready` MUST carry
   `hidden_result_count_known = true` (the count may be zero).
   `ready` with unknown hidden count is non-conforming.
3. `partial` and `warming` states keep the chip visible; they
   never silently resolve to an empty-state UI.

### 1.6 Policy-limited narrowing

When `scope_class = policy_limited_view`, the artifact carries a
`policy_limitation` block:

- `underlying_workset_ref` — points at the pre-narrowing artifact
  (which may itself be any non-policy-limited scope class).
- `policy_ref` — opaque reference to the policy that narrowed
  the view. Raw policy bodies never appear.
- `narrowing_cause` — `admin_policy`, `trust_policy`,
  `license_or_export_control`, `remote_unavailable`,
  `index_not_built`, `user_muted`.
- `visible_member_count` / `hidden_member_count` — integer
  counts.
- `hidden_member_list_visible` — boolean. MUST be false when
  `narrowing_cause` is `admin_policy` or
  `license_or_export_control`.

Rules (frozen):

1. A `policy_limited_view` chip MUST render the
   `hidden_member_count` when it is > 0, even if the list
   itself is hidden. Counts never collapse.
2. The `underlying_workset_ref` remains resolvable to the
   pre-narrowing artifact. A policy-limited view never
   replaces the underlying artifact; it layers over it.
3. The `reveal_hidden_results_policy_admin_only` chip action
   is permitted only when `hidden_member_list_visible = true`
   and the surface is a policy-admin surface.

## 2. Scope-truth chip record

Every open-flow trust card, search-result group header,
cross-repo result group, support-packet header, AI context
inspector, refactor scope footer, and export scope footer renders
one `scope_truth_chip_record`. The chip is the visible projection
of the artifact; it never invents a parallel stable id.

### 2.1 Surface family

The chip surface class is closed:

- `workset_switcher`
- `scope_banner`
- `search_result_group_header`
- `search_result_row_marker`
- `cross_repo_result_group`
- `open_flow_trust_card`
- `support_packet_header`
- `ai_context_inspector`
- `refactor_scope_footer`
- `export_scope_footer`

Rules (frozen):

1. Every listed surface reads the chip contract. A surface in
   this family that renders its own scope string is
   non-conforming.
2. Adding a new surface class is additive-minor. Removing a
   surface class is a breaking change.
3. A chip instance references a `workset_id`. Two chips on the
   same surface (e.g., a cross-repo result group header and a
   search-result row marker) reference the same `workset_id`
   when they describe the same active scope.

### 2.2 Chip presentation state

- `active_narrow_safe` — narrow boundary is the current scope
  and no hidden content is relevant.
- `active_partial` — scope is active but not fully indexed.
- `active_policy_limited` — scope is active but policy hides
  members.
- `active_widened` — scope was widened beyond the declared
  workset (cross-repo jump with `Widened` banner, refactor
  that exceeded the named workset).
- `outside_current_scope` — the chip marks a result, preview,
  or excerpt that sits outside the current scope.

Rules (frozen):

1. `active_narrow_safe` MUST NOT be emitted when
   `hidden_result_summary.known = true` and the count is > 0;
   the chip state resolves to `active_partial` or
   `active_policy_limited` instead.
2. `outside_current_scope` sets
   `outside_current_scope_marker_visible = true`. The marker
   is the visible cue that distinguishes `No results` from
   `No results in this scope`.
3. `active_widened` chips MUST carry the `open_scope_diff`
   action at a minimum. A widened chip without a diff hook is
   non-conforming.

### 2.3 Chip label family

The closed label family:

- `Current repo`
- `Selected workset` (resolves with the workset name)
- `Sparse slice`
- `Full workspace`
- `Policy-limited view`
- `Outside current scope` (row / excerpt marker)

Rules (frozen):

1. The five scope classes resolve to the first five labels
   (in order). `Outside current scope` is a row-marker chip
   and never stands in for an active-scope chip.
2. A chip that renders a label outside this family is
   non-conforming. Localized copy substitutes the label
   verbatim; it does not add variants.
3. A chip with `scope_class = selected_workset` MUST include
   the workset name in the rendered label. Bare
   `Selected workset` without the name is non-conforming.

### 2.4 Hidden-result summary

When hidden or excluded results are relevant, the chip carries a
`hidden_result_summary`:

- `known` — true if a count is meaningful.
- `count` — integer when known; null otherwise.
- `count_class` — `none_known`, `partial_index`,
  `outside_scope_roots`, `policy_hidden`, `warming_index`,
  `remote_unreachable`.

Rules (frozen):

1. `count_class = outside_scope_roots` is the cue a search
   surface renders when results exist in other roots but are
   excluded by the active workset. The chip offers
   `widen_to_full_workspace` or `widen_with_review`.
2. `count_class = policy_hidden` never exposes the exact
   hidden list unless the surface is a policy-admin surface
   and the underlying artifact permits it.
3. A chip with `known = false` MAY render `some results may
   be hidden` but MUST NOT render a number.

### 2.5 Offered actions

The `offered_actions` array carries typed chip actions:

- `widen_to_full_workspace`
- `widen_with_review`
- `narrow_to_current_repo`
- `open_scope_diff`
- `build_missing_indexes`
- `keep_current_scope`
- `reveal_hidden_results_policy_admin_only`
- `open_in_new_pane`
- `copy_workset_id`
- `export_workset_artifact`

Rules (frozen):

1. Every chip carries at least one action. Chips with no
   actions are denied with `chip_actions_missing`.
2. `reveal_hidden_results_policy_admin_only` is offered only
   when the underlying artifact's
   `policy_limitation.hidden_member_list_visible = true` and
   the surface class is a policy-admin surface.
3. `widen_with_review` is required on any chip where
   `chip_presentation_state = active_widened` or where
   `hidden_result_summary.count_class = outside_scope_roots`.

## 3. Scope-widen diff record

Widening scope is always an explicit action. The diff record is
the evidence for that action and powers the UI/UX spec Appendix
AC.4 scope-diff review sheet.

### 3.1 Shape

- `diff_id` — stable opaque id.
- `base_workset_ref` / `candidate_workset_ref` — the two
  artifacts being compared.
- `entries[]` — typed diff entries. At least one.
- `widens_scope` / `narrows_scope` — booleans derived from the
  entries.
- `changes_portability` / `changes_readiness` — booleans.
- `presentation_only` — true when every entry has
  `diff_class = presentation_only`.
- `expected_index_cost_class` — `none`, `cache_warm`,
  `targeted_index`, `full_reindex`, `remote_fetch_required`.
- `remote_fetch_required` — boolean.

### 3.2 Diff class vocabulary

- `identity_change`
- `member_added`
- `member_removed`
- `pattern_broadened`
- `pattern_narrowed`
- `policy_narrowed`
- `policy_widened`
- `readiness_changed`
- `portability_changed`
- `presentation_only`

Rules (frozen):

1. `identity_change` is forbidden on any diff where both base
   and candidate exist. Workset identity never drifts under a
   diff; a new `workset_id` means a new artifact.
2. A presentation-only diff MUST NOT set any of
   `widens_scope`, `narrows_scope`, `changes_portability`, or
   `changes_readiness`. The rename, subtitle-edit, and note
   edit path resolves through this gate.
3. `expected_index_cost_class = full_reindex` or
   `remote_fetch_required = true` MUST surface in the diff
   summary the review sheet renders; the user never widens
   into a costly materialization by accident.

## 4. How each surface uses the packet

### 4.1 Open flows

- A trust card on open quotes the `workset_id` the recent-work
  entry referenced. The open-flow trust card chip
  (`surface_class = open_flow_trust_card`) renders `Current
  repo`, `Selected workset <name>`, `Sparse slice`, or
  `Full workspace`.
- Opening a portable-state package that carries a
  `portable_with_rebinding` artifact renders an
  `active_partial` chip with the `open_scope_diff` action so
  rebinding is reviewable.
- An `add_root` entry verb that widens an existing workspace
  emits a `scope_widen_diff_record` against the prior
  `full_workspace` artifact before the new root materializes.

### 4.2 Search results

- The scope banner (`surface_class = scope_banner`) reads the
  active artifact and renders the chip.
- Cross-repo result groups emit one
  `cross_repo_result_group` chip per group; results that sit
  outside the active workset's roots render a
  `search_result_row_marker` chip with
  `chip_presentation_state = outside_current_scope`.
- `No results` is distinguished from `No results in this
  scope`: the latter is a chip with
  `hidden_result_summary.count_class = outside_scope_roots`
  and a `widen_with_review` action.
- Lexical results warm ahead of semantic and cross-repo
  results. A search surface that is partway through warming
  renders `active_partial` and carries a
  `partial_index_note`; it MUST NOT claim `active_narrow_safe`.

### 4.3 Support / export packets

- A support packet header (`surface_class =
  support_packet_header`) quotes the reporting user's
  `workset_id`. Triage replays the workset against the same
  artifact; no new stable id is minted.
- An export scope footer (`surface_class =
  export_scope_footer`) carries the chip plus the
  `export_workset_artifact` action. Exports of a
  `managed_provider_locked` artifact are blocked at this
  surface, not at the downstream export writer.
- Support bundles that reference a deleted workset render the
  chip with `chip_presentation_state = outside_current_scope`
  and a note explaining the id is historical.

### 4.4 AI, refactor, export, and later surfaces

- AI context inspectors (`surface_class =
  ai_context_inspector`) quote the `workset_id` the AI turn
  was scoped to. AI that reads from outside the named
  workset renders one chip per outside-scope excerpt with
  `outside_current_scope_marker_visible = true`.
- Refactor previews (`surface_class = refactor_scope_footer`)
  carry the chip plus a `scope_widen_diff_record` reference
  whenever a refactor would exceed the active scope. Silent
  widening is forbidden.
- Export surfaces (`surface_class = export_scope_footer`)
  attach the chip to every exported bundle so the importer
  sees what was exported under what scope.
- Later review, AI-apply, and cross-repo navigation surfaces
  add their surface class to this packet and reuse the same
  chip; they do not invent parallel scope labels.

## 5. Cross-repo result grouping and multi-root truth

- Multi-root artifacts carry `root_count >= 2` on the chip.
  Every cross-repo result group quotes its parent
  `workset_id` plus the specific root ref the group was
  resolved from.
- Two multi-root artifacts with different `workset_id`s are
  never silently merged. A search that accepts results from
  both emits two chips (one per workset) and a
  `scope_widen_diff_record` describing the union.
- `Outside current scope` markers always name the offending
  root by ref; the chip never collapses a foreign root into
  `somewhere else`.

## 6. Schema and artifact lifecycle

- Adding a new `scope_class`, `chip_surface_class`,
  `chip_action`, `narrowing_cause`, `diff_class`,
  `source_class`, `portability_class`, or `readiness_state`
  is additive-minor and bumps
  `workset_artifact_schema_version`.
- Repurposing an existing enum value is breaking and requires
  a new decision row.
- Removing a `chip_surface_class` is breaking; surfaces in
  the seed family may not be silently unhooked.
- The Rust types in the eventual workspace-shell crate are
  the schema of record once the crate lands; this file is
  the cross-tool boundary for diagnostics, support,
  portability review, and later AI/refactor/export tooling.

## 7. References

- PRD §5 workset / monorepo language.
- TAD §7.6 workset scope object, sparse-scope descriptor,
  workset switcher / scope banner.
- TDD §5 scope widening denial, scope-widening drills,
  workset-scope descriptor.
- UI/UX spec §12.8 scope review, workset portability, and
  cross-repo jump guarantees.
- UI/UX spec Appendix AC workset, sparse-scope, and
  cross-repo truth templates.
- UX style guide §scope-banner and cross-repo result group
  templates.
- Milestones document §6.25 execution-context,
  workset/scope, and recovery-ladder contract.
- ADR-0009 execution-context and scope
  (`/schemas/runtime/execution_context.schema.json`
  `workset_scope` discriminated union — the runtime
  projection that activates one of these artifacts at query
  time).
- Entry, recent-work, and restore-prompt model
  (`/docs/workspace/entry_restore_object_model.md` — hands a
  workset off to an open flow by `workset_id`).
