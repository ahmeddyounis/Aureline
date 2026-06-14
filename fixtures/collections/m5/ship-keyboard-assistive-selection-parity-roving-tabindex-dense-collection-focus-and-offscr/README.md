# M5 Assistive-Selection-Parity Fixtures

## assistive_selection_parity_and_roving_focus.json

A coverage fixture for the assistive-selection-parity packet. It wires the first
real M5 dense surfaces — pipeline run list, review queue, incident list, graph
list, marketplace results, and provider/admin table — onto one normalized
keyboard/assistive selection contract across all four view kinds (list, tree,
table, queue) and both dense-focus models (roving tabindex and
`aria-activedescendant`).

The profiles exercise the parity states the lane must hold:

- **Roving focus held by identity** (pipeline run list, incident list): a roving
  tabindex over streaming rows where new inserts and background refreshes never
  steal the single tabstop — focus and selection hold by stable id.
- **Hidden-selected exposure** (review queue): an `aria-activedescendant` queue
  with 3 selected items outside the current filter, announced to assistive
  technology so a broad action's scope is audible before it runs.
- **Focus re-anchor on churn** (graph tree, provider/admin table): when the
  focused item is recycled out of a virtualized view or dropped by a filter
  change, focus re-anchors on the nearest visible item with a precise,
  announced label.
- **Offscreen-selection durability** (marketplace results): 190 selected with 188
  offscreen in a virtualized table; the selection survives virtualization
  recycling and the offscreen count is exposed to assistive technology.
- **Full command set, no pointer-only controls** (every profile): select-current,
  extend-range, clear-selection, inspect-hidden-count, and open-batch-review are
  each keyboard- and screen-reader-reachable with a keyboard binding, accessible
  name, and announcement.

Every profile keeps a single tabstop tracked by stable identity, preserves
durable selection across streaming/sort/filter/virtualization churn, and exposes
the hidden-selected count to assistive technology. No profile carries raw row
bodies, provider payloads, or credentials.

The fixture validates against
`schemas/collections/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr.schema.json`
and is byte-identical to the checked support export at
`artifacts/collections/m5/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr/support_export.json`.
