# M5 Keyboard Assistive Selection Parity And Roving Focus

Dense M5 operational surfaces — pipeline runs, review queues, incidents, graph
lists, marketplace results, and provider/admin tables — only stay trustworthy
when every selection and batch-review operation works from the keyboard and is
legible to a screen reader. Selection bars already make the *selection state* of
a dense collection canonical (membership by stable identity, range-anchor
identity, hidden-selected counts, and a stale-query-snapshot guard). This
contract makes that same model **fully keyboard and assistive-technology usable**
so offscreen virtualization, streaming inserts, and query-backed selection never
collapse into a pointer-only or inaccessible batch model.

The canonical record is the `AssistiveSelectionParityPacket` produced by
`crates/aureline-collections`. It is the source of truth that product surfaces,
diagnostics, support exports, and accessibility/release evidence reuse rather
than re-deriving parity from raw rows.

- Schema:
  `schemas/collections/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr.schema.json`
- Support export:
  `artifacts/collections/m5/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr/support_export.json`
- Markdown summary:
  `artifacts/collections/m5/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr.md`
- Fixtures:
  `fixtures/collections/m5/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr/`
- Conformance dump:
  `crates/aureline-collections/examples/dump_m5_assistive_selection_parity.rs`

## What an assistive selection profile records

Each `AssistiveSelectionProfile` pins one `DenseCollectionSurface`, rendered as a
`CollectionViewKind` (list, tree, table, or queue) under a `CollectionDataMode`
(`static_complete`, `filtered_sorted`, `streaming`, `virtualized`, or
`paginated`), to:

- **A roving dense-focus model.** A `RovingFocusModel` declares its
  `FocusModelKind` (`roving_tabindex` or `aria_activedescendant`) — both keep a
  *single tabstop* so focus has one predictable home. Focus is tracked by stable
  item id, not row index; a visible focus indicator is always rendered; arrow
  keys navigate; and a precise `navigation_bound_label` describes wrap/clamp
  behavior at the ends.
- **The full keyboard/screen-reader command set.** Every profile carries an
  `AssistiveCommand` for each `AssistiveCommandKind` — `select_current`,
  `extend_range`, `clear_selection`, `inspect_hidden_count`, and
  `open_batch_review`. Each command names its keyboard binding, accessible name,
  and live-region announcement, and must be `keyboard_reachable`,
  `screen_reader_reachable`, and never `pointer_only`. The
  `inspect_hidden_count` command must actually name the hidden / outside /
  offscreen population in its announcement.
- **A live-region announcement contract.** A `SelectionAnnouncement` declares the
  `LiveRegionPoliteness` and asserts it announces the selection count and the
  hidden-selected count to assistive technology, with a precise sample.
- **Focus/selection churn resilience.** Per-event `FocusChurnResilience` records
  cover `streaming_insert`, `background_refresh`, `sort_or_filter_change`, and
  `virtualization_recycle`. Each records a `FocusDurabilityOutcome`
  (`focus_held_by_identity` or `focus_reanchored_visible`) and asserts the event
  preserved durable selection, did not steal focus, and announced any change. A
  re-anchor (when the focused item leaves the view) carries a precise label
  describing how focus moved.
- **Offscreen-selection durability.** An `OffscreenSelectionDurability` record
  asserts selection survives virtualization recycling, offscreen members are
  tracked by stable identity, and the hidden/offscreen-selected count is exposed
  to assistive technology, alongside the `offscreen_selected_count`.

## Truth and guardrails

A generic non-answer (`"focus"`, `"action"`, `"key"`, `"changed"`, …) is rejected
everywhere a precise label is required, so the user always hears *why* focus
moved, *what* a command does, and *how many* selected items are hidden.

The packet-level guardrails assert that no selection control is pointer-only;
streaming inserts and virtualization never steal focus; selection survives sort,
filter, and virtualization by stable identity; the hidden-selected count is
always exposed to assistive technology; roving focus is tracked by stable
identity; and broad-action review is keyboard reachable. The consumer projection
asserts that product, diagnostics, support/export, and accessibility evidence all
reuse these records.

## Reconstruction for diagnostics and evidence

`AssistiveSelectionProfile::reconstruction` projects a redaction-aware
`AssistiveSelectionProfileReconstruction` carrying only ids, tokens, labels, and
counts — never raw row bodies or provider payloads — so diagnostics and
accessibility-evidence packets can prove parity for offscreen/virtualized
selection and broad-action review without re-querying the data.

## Regenerating the artifacts

The checked-in support export and Markdown summary are emitted by the conformance
dump and must stay byte-aligned with the in-crate builder (the fixture is
byte-identical to the support export):

```bash
cargo run -p aureline-collections --example dump_m5_assistive_selection_parity \
  > artifacts/collections/m5/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr/support_export.json
cargo run -p aureline-collections --example dump_m5_assistive_selection_parity summary \
  > artifacts/collections/m5/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr.md
cp artifacts/collections/m5/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr/support_export.json \
  fixtures/collections/m5/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr/assistive_selection_parity_and_roving_focus.json
```
