# Tree View Interaction Contract

This document freezes the shared interaction model for hierarchy
surfaces: file trees, outline trees, schema trees, package trees,
component/runtime trees, route trees, dependency trees, write-scope
preview trees, and support/export projections. It layers view behavior
over the structural row record in
[`/docs/ux/tree_row_contract.md`](./tree_row_contract.md) so every
tree-backed surface keeps the same meaning for indentation,
disclosure, lazy hydration, focus, current item, active target,
selection, open state, virtualization, and provider fallback.

Machine-readable companions:

- [`/schemas/ux/tree_row.schema.json`](../../schemas/ux/tree_row.schema.json)
  defines `tree_row_record`, `tree_placeholder_record`,
  `tree_surface_snapshot_record`, and `tree_view_snapshot_record`.
- [`/fixtures/ux/tree_view_cases/`](../../fixtures/ux/tree_view_cases/)
  contains worked view-level examples for deep nesting, mixed generated
  and hidden nodes, filtered hierarchy state, and partially available
  providers.
- [`/fixtures/ux/tree_rows/`](../../fixtures/ux/tree_rows/)
  remains the row/snapshot corpus for structural identity, readiness,
  hidden-scope disclosure, and recovery posture.

This contract composes with:

- [`/docs/ux/tree_row_contract.md`](./tree_row_contract.md) for row
  anatomy, node kind, readiness, hidden-scope, selection-sync,
  generated/read-only, and identity-recovery vocabulary.
- [`/docs/ux/selection_and_scope_contract.md`](./selection_and_scope_contract.md)
  for focus/current/selection/range-anchor separation and batch-scope
  escalation.
- [`/fixtures/ux/selection_and_virtualization_manifest.yaml`](../../fixtures/ux/selection_and_virtualization_manifest.yaml)
  for count terms, focus-return states, and range-anchor vocabulary.
- [`/docs/ux/view_freshness_contract.md`](./view_freshness_contract.md)
  for live, partial, stale, captured, approximate, and export-safe
  scope truth.
- [`/docs/filesystem/filesystem_identity_vocabulary.md`](../filesystem/filesystem_identity_vocabulary.md)
  and [`/docs/generated/lineage_hint_packet.md`](../generated/lineage_hint_packet.md)
  for path identity, aliases, generated-artifact lineage, and safe-edit
  posture.

Where this document disagrees with the product source documents in
`.t2/docs/`, the upstream source wins and this contract, schema, and
fixtures must update in the same change.

## Scope

Frozen here:

- hierarchy-row interaction anatomy: indentation, disclosure control,
  icon/label/status/action slots, focusability, and accessible naming;
- lazy hydration and expandable placeholder behavior for rows whose
  children are not yet loaded, blocked, provider-backed, or partially
  known;
- keyboard navigation for expanded, collapsed, filtered, partially
  hydrated, virtualized, and provider-limited trees;
- separate focus, current item, active target, selection, range anchor,
  and open-editor state;
- virtualization requirements for visible, realized, loaded, matching,
  hidden, not-loaded, and offscreen counts;
- labels and accessible cues for hidden, filtered, generated, ignored,
  blocked, unavailable, unsupported, stale, cached, and provider-backed
  nodes;
- batch-selection, drag/move, and inline-action posture; and
- fallback rules for partial hydration, disconnected providers, missing
  extensions, and missing provider support.

Out of scope:

- a concrete renderer implementation, final component styling, icon
  artwork, file operation algorithms, provider installation flows, or
  symbol-index quality;
- redefining row identity, node kinds, freshness, completeness,
  hidden-scope reasons, generated-lineage posture, or recovery actions
  already owned by the row contract; and
- replacing the collection, selection, drag/drop, or interaction-safety
  contracts for dense non-hierarchical surfaces.

## One Model Across Tree Families

Every hierarchy surface emits the same view-level record shape. A
surface may choose density, iconography, or where the disclosure control
is drawn, but it must not replace the vocabulary below with local
equivalents.

| Surface class | Reuses this contract for | Special risk |
|---|---|---|
| `file_tree` | workspace roots, folders, files, generated artifacts, aliases, ignored paths | hidden ignored scope, symlink/alias identity, generated and read-only rows, move/drop review |
| `outline_tree` | file-local and provider-backed symbols | stale symbols, file-local fallback, approximate mappings, current-editor sync |
| `schema_tree` | database, GraphQL, OpenAPI, config, or data-schema hierarchy | disconnected provider, generated schema fragments, unavailable fields, partial introspection |
| `package_tree` | package manifests, dependency groups, installed extensions, package inventories | provider-limited metadata, policy-blocked packages, generated lockfile rows, provider outage |
| `component_tree` | framework components and preview/runtime projections | approximate source mapping, runtime-only nodes, missing framework provider |
| `runtime_tree` | DOM, debugger, notebook, simulator, or process hierarchy | live identity drift, protocol limits, stale captured snapshots |
| `route_tree` | framework route maps and convention-derived routes | generated routes, missing source files, unsupported framework versions |
| `dependency_tree` | build graph, import graph, package graph, or service graph | provider limits, stale lock metadata, hidden policy dependencies |
| `write_scope_preview_tree` | multi-file review, refactor, generated output, support export | blocked/generated/read-only rows must stay visible before commit |

## View Snapshot Record

`tree_view_snapshot_record` captures the interaction state for one tree
view at one moment. It cites row identities from a
`tree_surface_snapshot_record` or another row owner, but it does not
become the authority for the underlying objects.

Required field groups:

| Field group | Rule |
|---|---|
| `view_snapshot_id`, `surface_class`, `scope_ref` | Identify the hierarchy surface and declared scope. |
| `readiness_state`, `freshness_class`, `completeness_class` | Mirror the source tree's current readiness and scope claim. |
| `keyboard_navigation` | Declares the arrow-key, typeahead, Home/End, activation, and multi-select model. |
| `virtualization` | Names realized/visible/represented/offscreen counts, focus preservation, and accessibility semantics. |
| `selection_state` | Keeps focus, current row, active row, open rows, selected rows, hidden selected rows, not-loaded selected rows, and range anchor separate. |
| `batch_selection` | Exposes visible, loaded, matching, selected, hidden selected, not-loaded, and blocked counts before a batch action. |
| `drag_move` | Names the view-level drag/reorder/move/copy posture. |
| `inline_action_policy` | Names when actions are inline versus deferred. |
| `provider_fallbacks[]` | Lists unavailable, disconnected, missing, blocked, or degraded providers and the preserved fallback. |
| `row_states[]` | Provides row-local indentation, disclosure, hydration, interaction, action exposure, and drag/move posture. |

Screenshots, support export, accessibility captures, and automation
logs may quote compact labels, but the structured snapshot is the
record of truth.

## Row Anatomy And Indentation

Every rendered tree row has the following ordered anatomy:

1. indentation region, including optional depth guide;
2. disclosure affordance when the row can expand, collapse, hydrate, or
   explain why children are unavailable;
3. node-kind icon or equivalent accessible kind label;
4. primary label and optional secondary label;
5. state badges for partial, stale, generated, hidden, read-only,
   policy-limited, remote-limited, unsupported, cached, imported, or
   approximate mapping;
6. selection/control affordance only when the row participates in batch
   selection;
7. inline actions only when the action-exposure policy admits them; and
8. deferred context action route for everything else.

Indentation is derived from `depth`, `parent_row_id`, and
`structural_identity_ref`; it is not identity. Virtualization, sorting,
filtering, and hydration must not infer object identity from pixel
offset, sibling order, label text, or indentation guide state.

Depth must remain stable while a row is mounted. When a parent is
filtered out but a descendant remains visible, the row either renders an
ancestor summary/placeholder or exposes a hidden-ancestor disclosure;
it must not pretend the descendant moved to a shallower hierarchy.

## Disclosure And Lazy Hydration

Disclosure controls use `tree_disclosure_state`:

| State | Meaning | Required behavior |
|---|---|---|
| `leaf_no_affordance` | The row has no known children. | No disclosure control; keyboard Right does not imply hidden children. |
| `collapsed_ready` | Children are known and hidden by collapse. | Right expands without provider work. |
| `expanded_ready` | Children are known and visible. | Left collapses or moves to parent by keyboard model. |
| `collapsed_unhydrated` | Children may exist but are not loaded. | Right starts hydration or expands to an explicit placeholder. |
| `expanding_hydrating` | Hydration is underway. | Keep focus and selection on the row or placeholder; no blank child gap. |
| `expanded_partial` | Some children are visible while others are hidden, filtered, not loaded, or provider-limited. | Show child placeholder or inline/ancestor hidden-count disclosure. |
| `blocked_unavailable` | Children are blocked by policy, trust, scope, or unsupported state. | Disable expansion and expose reason/action. |
| `provider_unavailable` | Backing provider is disconnected, missing, or degraded. | Preserve cached rows or placeholder; route recovery through provider fallback. |
| `placeholder_expandable` | The row is itself an expandable placeholder. | Keyboard and accessibility expose that expansion inspects or retries the missing scope. |

Lazy hydration must preserve:

- `structural_identity_ref` for rows that survive hydration;
- expansion state for still-known ancestors;
- focus and selection by identity;
- offscreen and hidden counts; and
- accessibility order and announcements.

Hydration may append children, refine approximate counts, or replace a
placeholder with rows. It must not reorder already realized siblings
unless the source tree issues a new snapshot/delta sequence that names
the reorder.

## State Axes

Tree rows keep five interaction states separate:

| Axis | Meaning | Non-conforming collapse |
|---|---|---|
| Focus | Where keyboard input goes now. | Treating selected rows as focused. |
| Current item | The row driving details, preview, or status panes. | Treating current item as selected for batch actions. |
| Active target | The row that maps to the active editor, runtime target, provider context, or ancestor of that target. | Highlighting a row as active without a stable target ref. |
| Selection | Stable row identities admitted to the current batch set. | Binding selection to row index, viewport position, or label. |
| Open state | Whether the row's target is open in an editor, preview, secondary group, or external surface. | Treating open rows as selected or focused. |

Selection survives filtering, virtualization, sort/reorder, streaming
arrivals, and hydration by stable identity. If a selected row becomes
hidden or not loaded, the view exposes `hidden_selected_count` or
`not_loaded_selected_count` and provides reveal, clear, or review routes.

## Keyboard Navigation

Every tree view declares one keyboard navigation model.

Required behavior:

- Up/Down moves focus through the current focus order without changing
  selection unless a range extension is active.
- Left collapses an expanded branch, or moves to the parent when the
  branch is already collapsed according to the declared model.
- Right expands a collapsed branch; if the branch is unhydrated, it
  starts hydration or moves focus to an announced placeholder.
- Space toggles row selection only when the row participates in batch
  selection. It must not activate or expand the row by accident.
- Enter activates the current row or node-kind default action; it must
  not silently widen selection or bypass provider fallback.
- Shift extends selection from the stable range anchor; virtualization
  and hydration do not move the anchor.
- Ctrl/Cmd toggles the focused identity without clearing the rest of
  the set when multi-select is enabled.
- Typeahead names whether it searches visible rows, loaded rows, all
  matching rows after review, or is disabled.
- Home/End names whether it moves within the realized window, loaded
  tree, or full tree with virtual scroll.

Placeholder rows are keyboard reachable when they explain missing scope,
offer recovery, retry hydration, or carry selected/active/open state.

## Virtualization And Accessibility

Virtualization is allowed only when the view can preserve honest
counts, focus order, selection state, and accessibility semantics.

The `tree_virtualization` record must separate:

- `visible_row_count`: rows actually visible in the viewport;
- `realized_row_count`: mounted rows including overscan;
- `total_represented_row_count`: loaded or known rows represented by
  the current view;
- `offscreen_before_count` and `offscreen_after_count`: rows outside the
  realized window;
- `hidden_filtered_count`: rows excluded by filter or search narrowing;
- `not_loaded_count`: known or possible rows not yet loaded; and
- hidden-scope disclosures from the row contract for ignored,
  policy-hidden, generated-hidden, unsupported, remote-unreachable, or
  redacted rows.

Accessibility rules:

- The tree exposes a role model: `tree_treeitem_group`, `treegrid`,
  `outline_tree`, or `native_platform_tree`.
- `setsize` and `posinset` truth may be exact, exact for loaded or
  visible siblings, or unknown until hydrated; approximate or unknown
  values must be announced as such.
- Offscreen rows may be unmounted, but selected, focused, active, open,
  and range-anchor identities remain in the view snapshot.
- Focus return after hydration, filter change, provider reconnect,
  modal review, or row removal follows the declared focus-preservation
  rule.
- Hidden, filtered, generated, ignored, blocked, unavailable, stale,
  partial, cached, and provider-limited cues are not color-only.

## Labels And Cues

The following label classes are frozen. Product copy may localize the
visible string, but the token and accessible meaning must survive.

| Class | Required cue |
|---|---|
| Hidden by filter | Filter count or ancestor summary plus clear/reveal action when allowed. |
| Hidden by workspace rules or ignore rules | Hidden-scope disclosure with exact/approximate/unknown count and inspect action. |
| Generated | Generated badge plus lineage/source/regenerate action when known. |
| Ignored | Ignored-scope disclosure; ignored rows must not disappear from support export when selected or active. |
| Blocked | Policy/trust/provider blocked badge plus reason and recovery/review route. |
| Unavailable | Provider or extension unavailable label plus preserved capability and fallback route. |
| Unsupported | Unsupported row or placeholder; never a blank subtree. |
| Stale or cached | Stale/cached badge plus timestamp/source ref and refresh/revalidate action. |
| Partial | Partial badge plus count scope and the action that can widen, hydrate, or inspect the gap. |

Generated, ignored, blocked, unavailable, unsupported, stale, cached,
and partial rows may be visually compact, but their accessible names and
support/export records must preserve the state class, count truth, and
reason.

## Batch Selection

Tree multi-select inherits the dense collection selection model, with
hierarchy-specific constraints:

- `Select all` is forbidden. Controls must say visible rows, loaded
  rows, matching rows, or explicit custom set.
- Selecting a parent does not automatically select hidden, not-loaded,
  provider-blocked, generated-read-only, or policy-blocked descendants
  unless a review step names those descendants and count status.
- Parent tri-state visuals are presentation only; the backing
  `batch_selection` record names selected, visible, loaded, matching,
  hidden selected, not-loaded selected, and blocked counts separately.
- Batch actions over generated, blocked, unavailable, provider-owned,
  remote, destructive, share/export, or move/copy populations require a
  review packet before execution.
- Hidden selected descendants remain inspectable from keyboard and
  assistive technology.

## Drag Or Move Posture

Drag, reorder, move, and copy operations use
`tree_drag_move_posture_class`:

| Posture | Meaning |
|---|---|
| `not_supported` | The surface does not support drag or move. |
| `preview_only` | Drag previews target identity but no drop is admitted. |
| `local_reorder_allowed` | Reordering is local, reversible, and exact. |
| `move_requires_review` | Move changes workspace/provider state and routes through review. |
| `copy_requires_modifier` | Copy is distinct from move and requires the platform modifier or explicit command. |
| `external_drop_review_required` | Cross-surface or external drops require preview/review. |
| `blocked_by_policy` | Policy/trust blocks drop; explain route is available. |
| `blocked_generated_or_read_only` | Generated/read-only posture blocks mutation. |
| `provider_move_unavailable` | Provider cannot currently admit the move. |

Drop targets must be resolved by structural identity and authority, not
by row label or viewport position. A virtualized tree may show drop
indicators for realized rows only; dropping into an unhydrated branch
requires hydrate or review first.

## Inline And Deferred Actions

Inline actions are limited to routine, row-local, low-risk commands such
as reveal, open, refresh, retry, clear filter for this branch, or inspect
state. A row may expose inline actions when:

- the row is focused, hovered, or width permits;
- the action does not change batch scope silently;
- the action is keyboard reachable;
- the action has an accessible name; and
- no hidden or provider-limited consequence is being bypassed.

Actions must be deferred to a context menu, command palette, review
sheet, or detail surface when they are destructive, batch-affecting,
remote/provider-owned, generated-artifact mutation, move/copy, export,
share, policy/trust-related, or need revalidation.

Rows may expose at most three inline actions. Overflow actions remain
reachable through the declared deferred action route.

## Provider And Extension Fallback

Provider-backed rows use `tree_provider_fallback` when backing data is
partial, disconnected, missing, blocked, unsupported, or degraded.

Fallback rules:

1. Partial hydration shows known rows and an expandable placeholder. It
   does not imply an empty subtree.
2. Disconnected providers keep cached rows inspectable when safe and
   mark mutations/reveal actions that require revalidation.
3. Missing extensions or providers render an unavailable row or
   placeholder with preserved capability and enable/install/reconnect
   route when policy allows.
4. Unsupported scopes render an unsupported row or placeholder with the
   fallback route. They do not collapse into a missing child count.
5. Provider-blocked or policy-blocked rows name the blocker and preserve
   selected/active/open state until the user clears or resolves it.
6. A provider reconnect may refresh rows and counts, but it must not
   silently widen selection, drop hidden-selected identities, or rebind
   focus by position.
7. Support/export captures preserve provider state, fallback behavior,
   count truth, and affected row refs with redaction-aware labels.

## Non-Conforming Patterns

- A blank subtree while hydration, provider reconnect, or indexing is
  still running.
- A row that looks expandable but exposes no keyboard or accessibility
  disclosure state.
- A filtered tree that moves descendants to shallower depth without
  hidden-ancestor disclosure.
- A virtualized tree that drops selection, active/open state, range
  anchor, or focus when rows unmount.
- A parent selection that silently includes hidden, generated, blocked,
  unavailable, or not-loaded descendants.
- A package or schema tree that hides provider outage by rendering an
  empty tree.
- Inline destructive or provider-owned actions that bypass review.
- Drag/drop targets resolved by visible row index instead of structural
  identity.

## Required Fixture Coverage

The view fixture corpus covers:

- deep nesting with lazy hydration and virtualized offscreen counts;
- mixed generated, ignored, hidden, and blocked nodes;
- filtered trees with hidden-selected and active/open state preserved;
- schema and package trees with partially available, missing, or
  disconnected providers; and
- inline action, deferred action, batch-selection, drag/move, focus, and
  accessibility semantics across those cases.
