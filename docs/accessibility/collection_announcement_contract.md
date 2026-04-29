# Accessible Collection Announcement Contract

Status: seeded

This contract freezes the accessibility-facing projection for dense
lists, trees, tables, grids, logs, package inventories, review queues,
search results, and admin collections. It gives every collection one
way to announce position, selection count, hidden membership, blocked or
read-only state, and batch scope before virtualization, filtering, or
provider-backed bulk actions fragment the user experience.

Contract identity:

- `collection_announcement_contract_id:
  aureline.accessibility.collection_announcement`
- `collection_announcement_contract_revision: 1`
- `collection_announcement_schema_version: 1`

Companion artifacts:

- [`/schemas/accessibility/collection_announcement.schema.json`](../../schemas/accessibility/collection_announcement.schema.json)
  defines `collection_announcement_record` and
  `collection_announcement_case_record`.
- [`/fixtures/accessibility/collection_announcement_cases/`](../../fixtures/accessibility/collection_announcement_cases/)
  contains seed cases for virtualized search grids, filtered trees,
  stale query snapshots, and keyboard-help parity.
- [`/docs/accessibility/accessibility_tree_contract.md`](./accessibility_tree_contract.md)
  owns role, node, row-index, and virtualization truth.
- [`/docs/accessibility/screen_reader_and_live_region_contract.md`](./screen_reader_and_live_region_contract.md)
  owns live-region delivery, dedupe, coalescing, and stable message IDs.
- [`/docs/ux/selection_and_scope_contract.md`](../ux/selection_and_scope_contract.md)
  owns focus, current item, selection, checked state, range anchor, and
  selection-scope state.
- [`/docs/ux/collection_view_contract.md`](../ux/collection_view_contract.md)
  owns filter ASTs, saved views, count truth, and batch-review packets.

Normative source anchors:

- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Sections 9.1, 9.12, 19.11,
  19.12, and Appendix CD require dense rows to announce position,
  selected state, blocked/read-only state, hidden counts, and batch
  scope without hover-only dependencies.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` Sections 14.15,
  16.65, 19.10, and 19.11 require visible, loaded, matching, selected,
  blocked, hidden, and export scopes to remain explicit across dense
  collections.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` Section 23
  requires semantic accessibility surfaces, structured announcements,
  and assistive-technology regression journeys.
- `.t2/docs/Aureline_Technical_Design_Document.md` Sections 8.8, 8.13,
  and 13.13 require accessibility parity and collection-truth
  verification across virtualization, provider limits, offline mode, and
  exports.

## Scope

This contract is a projection contract. It does not replace the
selection-state record, batch-review packet, accessibility-tree node, or
announcement event. It binds the facts from those records into the one
accessibility payload a screen reader, keyboard help surface, support
snapshot, or QA fixture can inspect.

In scope:

- position-in-set, row index, column index, nesting level, and index
  truth for focused or current collection rows;
- selected state, selected count, hidden-selected count, not-loaded
  selected count, blocked count, and read-only state;
- batch scope, action target scope, visible-vs-all-matching wording, and
  pre-execution announcement requirements;
- virtualization and row-mount invariants that keep accessibility truth
  stable while rows mount, unmount, recycle, or hydrate partially;
- stale-query and prior-snapshot announcement rules for consequential
  batch actions after material dataset changes;
- keyboard-help parity for high-frequency list, tree, and grid actions.

Out of scope:

- final vendor-specific screen-reader phrasing;
- platform adapter implementation details for UIA, NSAccessibility, or
  AT-SPI;
- rendering list, tree, grid, or batch-review UI.

## Required Announcement Fields

Every `collection_announcement_record` MUST expose the following field
groups before a collection row or scope-changing action can claim
assistive-technology parity.

| Group | Required facts |
|---|---|
| Contract identity | contract id, revision, schema version, record id, surface class, collection instance ref, and minted time. |
| Row position | stable item ref, row node ref, position in set, set size, row index, row count, optional column index/count, optional tree level, and index truth class. |
| Row state | selected state, current/focused flags, blocked state, blocked reason, read-only state, and a state label. |
| Counts | selected count, hidden-member count, not-loaded count, visible count, loaded count, matching count, included count, and blocked count, each with count status. |
| Scope | selection-scope class, action-scope class, scope-truth class, execution-origin class, and an action-scope label. |
| Virtualization | row mount state, hydration state, identity and index preservation booleans, and the rule forbidding mount index as identity. |
| Stale basis | dataset identity ref, query/filter basis ref, prior snapshot ref when used, material change classes, and review-or-deny requirements. |
| Keyboard help | high-frequency action kind, command id ref, shortcut label when present, non-hover help surfaces, assistive description message id, and screen-reader action availability. |
| Announcement policy | message id, live-region channel, delivery timing, coalescing strategy, durable fallback ref, and privacy/export posture. |

The unqualified phrases `Select all`, `selected items`, or `batch
action` are insufficient when the action may target a larger set than
the visible viewport. The record MUST say whether the action applies to
`current_item`, `visible_items`, `loaded_items`, `all_matching_items`,
`explicit_custom_set`, or `provider_side_query`.

## Position And State Rules

1. `position_in_set`, `set_size`, `row_index`, and `row_count` are
   logical collection facts. They MUST come from the collection model or
   provider basis, not from the currently mounted render node list.
2. A selected row that is filtered out, paged out, or unmounted remains
   represented through selected counts and stable item identity.
3. A focused row, current row, selected row, checked row, and activated
   row are separate states. The announcement record may project several
   at once, but it MUST NOT collapse them into one row highlight.
4. Blocked, unavailable, read-only, and inspect-only are separate facts.
   A row may be selected and read-only, blocked but inspectable, or
   unavailable and not selectable; the announcement must preserve those
   differences.
5. Hidden-member count means selected or included identities that are
   outside the current visible window, filter, tree expansion, group, or
   page. It does not include unselected provider matches.

## Virtualization And Row Mount Rules

Virtualization is conforming only when the accessibility facts survive
row recycling:

- stable item refs survive sorting, filtering, scrolling, pagination,
  streaming inserts, and partial hydration;
- accessible row indices survive unmount and remount because indices are
  resolved from the logical collection basis;
- selection membership survives unmount and remount because membership
  is stored by stable item identity or provider-side selection ref;
- partial hydration may defer row body fields, but it MUST NOT defer
  row identity, selected state, blocked/read-only state, position truth,
  or batch-scope truth;
- a mount index, paint index, DOM order, or recycled child ordinal MUST
  NOT be used as durable selection identity or row-index truth.

When the producer cannot satisfy those rules, it MUST downgrade the
surface to `summary_only`, static pagination, or inspect-only state and
announce the narrowed support posture instead of presenting approximate
mount order as truth.

## Batch Scope Announcement Rules

Before a consequential action executes, the announcement record MUST
state the action target scope:

| Action-scope class | Meaning | Required announcement facts |
|---|---|---|
| `current_item` | The action applies only to the current object. | Current item identity and blocked/read-only state. |
| `visible_items` | The action applies to visible rows only. | Visible count and statement that matching or loaded rows outside the view are not included. |
| `loaded_items` | The action applies to client/provider-loaded rows. | Loaded count, visible count, and not-loaded selected count when non-zero. |
| `all_matching_items` | The action applies to every item matching the current query/filter basis. | Matching count and status, hidden-member count, not-loaded count, query/filter basis, and stale-basis state. |
| `explicit_custom_set` | The action applies to explicit selected identities across filters/pages. | Selected count, hidden-member count, stable identity class, and reveal/clear routes. |
| `provider_side_query` | The provider resolves the action target at execution time. | Provider authority ref, matching count status, provider-side basis, and re-resolution warning. |

Batch scope facts MUST be emitted before execution for remote,
destructive, provider-owned, export, and share actions. If the action
scope is wider than the current item or visible rows, a durable review
surface remains the fallback when live-region output is suppressed.

## Stale Query And Prior Snapshot Rules

A collection scope becomes stale when its query, filter AST, sort basis,
dataset identity, provider cursor, policy epoch, or row identity
namespace changes materially after the user reviewed a scope.

Rules:

1. A broad or destructive action against stale scope MUST either reopen
   review or deny execution. It MUST NOT silently apply to the new live
   dataset.
2. When the product continues from a prior snapshot, the announcement
   MUST say that the action uses the prior snapshot, not the current live
   query.
3. When matching counts changed materially, the announcement MUST carry
   both the reviewed count and current count when both are safe to
   disclose.
4. Provider-limited, approximate, cached, partial, stale, or unknown
   counts MUST keep their status in live-region output, durable review,
   CLI/help projections, and support/export packets.
5. A stale provider-side query whose identities cannot be re-resolved
   denies with a stale-scope explanation rather than falling back to
   visible rows.

## Keyboard-Help Parity

High-frequency list, tree, and grid actions MUST be discoverable without
hover:

- select current row;
- extend range from anchor;
- toggle focused identity;
- clear selection;
- reveal or clear hidden selected members;
- select visible rows;
- select loaded rows;
- select all matching items;
- open batch review;
- open row details;
- explain blocked or read-only state.

Each action record MUST cite at least one non-hover help surface:
command palette, keyboard shortcuts help, row accessible description,
screen-reader action menu, status/selection bar, context menu, or docs
help. Hover tooltips may repeat the same help, but they never satisfy
the parity requirement alone.

## Fixture Acceptance

A fixture conforms when a keyboard-only or assistive-technology user can
determine:

- the focused/current row position in the logical collection;
- whether the row is selected, selectable, blocked, read-only, or
  inspect-only;
- selected count, hidden-member count, and not-loaded count;
- whether a command targets visible items, loaded items, all matching
  items, an explicit custom set, or a provider-side query;
- whether the current batch scope is live, stale, revalidated, or based
  on a prior snapshot;
- which keyboard or assistive action exposes selection, range extension,
  hidden-selected review, clear selection, and batch review; and
- which durable surface carries the same facts when live-region output
  is silent or coalesced.

Any fixture that derives accessible position from mounted row order,
loses selection on virtualization, hides hidden selected members, or
uses hover-only help for high-frequency actions is non-conforming.
