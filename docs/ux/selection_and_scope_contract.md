# Selection and scope contract for dense collections

This document freezes the in-flight selection model every dense
collection uses before it emits a batch-review packet. It applies to
search results, problems, dependency tables, review inboxes, pipeline
logs, package inventories, work-item lists, admin grids, and any future
surface where a user can act on more than one item.

The companion artifacts are:

- [`/schemas/collections/selection_state.schema.json`](../../schemas/collections/selection_state.schema.json)
  - boundary schema for the live selection-state record a collection
  surface exposes to UI, accessibility, support, and audit tooling.
- [`/fixtures/collections/selection_cases/`](../../fixtures/collections/selection_cases/)
  - seeded cases for visible-to-matching escalation, hidden selected
  disclosure, range-anchor stability, and provider-backed review or
  cancel flows.
- [`/docs/ux/collection_view_contract.md`](./collection_view_contract.md)
  - broader collection contract for filter ASTs, saved views, and
  batch-review packets.
- [`/docs/verification/focus_and_batch_scope_packet.md`](../verification/focus_and_batch_scope_packet.md)
  - verification packet for focus return, range selection, count truth,
  and accessibility review.

This contract owns the state a dense collection presents while the user
is selecting and deciding. The batch-review packet owns the reviewed
population once the user chooses a consequential action.

## State Model

Dense collection rows must keep five states distinct:

| State | Meaning | Required behavior |
|---|---|---|
| `focus` | Where keyboard input goes now | Exactly one window-local focus owner is exposed to keyboard and assistive technology. |
| `current_item` | The row driving details, preview, or status panes | Current-row emphasis may follow focus, but it is not selection. |
| `selection` | Stable item identities admitted to the current batch set | Selection survives sort, filter, pagination, streaming, and virtualization by identity. |
| `checked_state` | A checkbox or tri-state control rendered in a row | A selection checkbox may mirror selection; an independent boolean checkbox must say it is not selection. |
| `activation` | The command Enter, double-click, or the primary action will invoke | Activation opens or acts on the current item; it must not silently widen or clear selection. |

Required keyboard behavior:

- Space toggles selection for the focused row when the row has a
  selection role. It must not activate the row.
- If Space toggles an independent checked state instead, the row must
  disclose that checked state is not batch selection.
- Shift extends from the visible or announced range anchor. Scrolling
  and virtualization do not move the anchor.
- Ctrl or Cmd toggles the focused item by stable identity without
  clearing the rest of the set.
- Enter activates the current item without changing selection unless a
  reviewed command explicitly says otherwise.
- Clear selection is a first-class keyboard and assistive-technology
  action. Clearing also resets the range anchor.

## Scope Labels And Counters

Every collection with multi-select must expose a selection bar or
equivalent summary when one or more items are selected. The summary must
include:

- selected count;
- scope label: current item, visible rows, loaded rows, all matching
  items, or custom set;
- hidden selected count when non-zero;
- not-loaded selected count when non-zero;
- blocked or skipped count when already known;
- clear-selection action and reset path;
- broadened-scope state when the selection has escalated beyond visible
  or loaded rows.

The unqualified label `Select all` is forbidden. A control whose real
scope is only the rendered viewport must use visible-row language, such
as `Select visible rows`. A control whose real scope is only the client
or provider cursor window must use loaded-row language, such as
`Select loaded rows`. Only a control that truly targets the query or
provider result set may use all-matching language, such as
`Select all 247 matching items`.

Result counters must not collapse visible, loaded, matching, selected,
hidden selected, and not loaded into one number. Every count carries a
status: exact, approximate, provider-limited, stale, cached, partial, or
unknown.

## Escalation

Selection broadening is a deliberate state transition:

1. The user selects the current item, a visible range, a loaded set, or
   a custom identity set.
2. If an all-matching scope is available, the collection offers a second
   explicit action that names the matching count and freshness.
3. Before broadening, the surface reviews what will change: visible
   count, loaded count, matching count, hidden selected count, not-loaded
   count, blocked count if known, and provider-limit status.
4. The broadened scope remains cancellable. Cancelling returns to the
   prior visible, loaded, or custom set and preserves the prior anchor
   unless the user clears selection.
5. If the query, provider basis, or dataset identity changes after
   broadening, consequential actions must route through review again or
   deny with a stale-scope explanation.

## Identity Rules

Selection follows stable item identity, never row position.

- Sorting changes render order only; selected identities and the range
  anchor remain bound to the same logical items.
- Filtering may hide selected identities, but it must disclose the
  hidden selected count and provide reveal or clear actions.
- Streaming arrivals are not automatically selected unless the user has
  accepted an all-matching query scope that names future arrivals as part
  of the provider basis.
- Virtualization may unmount rows, but it must not drop selected
  identities or move the range anchor.
- Pagination and cursor windows rebase by item identity. A selected item
  leaving the loaded page becomes not loaded, not unselected.
- Provider-backed collections pin provider identity or a provider-side
  selection ref. If the provider can no longer resolve the pinned basis,
  the next consequential action must review drift or deny.
- Local aliases are allowed only when disclosed. A local alias must not
  be replayed across sessions as though it were provider identity.

## Batch Review

Consequential actions require preview or review:

| Action class | Required review |
|---|---|
| Routine non-mutating action on the current item | No batch review; current-item wording is enough. |
| Local reversible batch action | Inline preview is acceptable when included and skipped counts are exact. |
| Remote, destructive, provider-owned, export, or share action | Review sheet required before execution. |

The review sheet must summarize included, excluded, blocked, skipped,
hidden selected, and not-loaded items separately. It must name the
execution origin, recovery posture, provider-count status, and cancel
path before the user commits.

Blocked items are pre-commit ineligible. Skipped items are items the
action intentionally did not apply during or after execution. These
counts must not be collapsed into one unavailable bucket.

## Fixture Acceptance

A seeded collection fixture conforms only when a keyboard-only or
assistive-technology user can determine:

- the current focus owner and current item;
- whether Space will select, check, or do nothing;
- selected count and current scope;
- whether scope has broadened beyond visible or loaded rows;
- hidden selected and not-loaded counts;
- how to reveal hidden selected items;
- how to clear selection and reset the range anchor;
- whether the next batch action requires review; and
- where focus returns after review or cancel.

Any fixture that loses selection on sort, filter, pagination,
streaming, or virtualization is non-conforming. Any fixture that uses
position indexes as durable selection identity is non-conforming.
