# Collection-truth drills

Keyboard and screen-reader drills that every claimed beta collection surface MUST pass.
Sourced from the seeded corpus in
`crates/aureline-shell/src/collection_truth_corpus/mod.rs`. Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- drills-md > \
  docs/qe/m3/collection_truth_drills.md
```

## Drill index

| Surface | Drill class | Drill id |
| ------- | ----------- | -------- |
| `search_or_result_grid` | `keyboard_anchor_range_selection` | `drill:keyboard-anchor-range-selection` |
| `admin_or_settings_grid` | `screen_reader_hidden_selected_inspection` | `drill:screen-reader-hidden-selected-inspection` |
| `review_inbox` | `keyboard_batch_review_open` | `drill:keyboard-batch-review-open` |
| `log_or_event_collection` | `saved_view_switcher_under_virtualization` | `drill:saved-view-switcher-virtualized` |

## `drill:keyboard-anchor-range-selection` -- Anchor-based range selection across virtualized rows

- Surface: `search_or_result_grid`
- Drill class: `keyboard_anchor_range_selection`

### Steps

1. Focus the first row to set the anchor
1. Press Shift+ArrowDown to extend the range to the next row
1. Press Shift+PageDown to extend across the virtualization window boundary
1. Press Shift+End to extend through the last loaded row

### Expected assertions

- Anchor row id stays stable across virtualization window scrolls
- Range selection never includes blocked or hidden rows silently
- Selected count narration matches `visible_or_loaded` escalation

### Virtualization invariants

- Anchor row remains in the loaded buffer after window recycling
- Selection state survives a viewport scroll cycle
- Loaded count never drops below the anchor row index

### Accessibility narration

> `Selected 5 of 200 loaded rows; 0 hidden, 2 blocked; anchor row 1.`

## `drill:screen-reader-hidden-selected-inspection` -- Inspect hidden-selected count via screen reader

- Surface: `admin_or_settings_grid`
- Drill class: `screen_reader_hidden_selected_inspection`

### Steps

1. Activate the scope counter strip via the keyboard
1. Navigate to the hidden-selected count row
1. Activate the Inspect hidden selected affordance
1. Verify the hidden-selected rows are narrated by their stable ids

### Expected assertions

- Hidden-selected count is non-zero and visible to the screen reader
- Inspect affordance announces stable row ids without payload literals
- Batch review summary hidden_count equals the hidden-selected count

### Virtualization invariants

- Hidden-selected rows remain countable when scrolled out of the viewport
- Scope counter strip never collapses hidden into visible

### Accessibility narration

> `3 selected rows are hidden by the current view. Inspect to review them before continuing.`

## `drill:keyboard-batch-review-open` -- Open the batch-review sheet from a consequential action

- Surface: `review_inbox`
- Drill class: `keyboard_batch_review_open`

### Steps

1. Focus the consequential action affordance
1. Press Enter to request the action
1. Verify the batch-review sheet captures focus
1. Tab through included, excluded, blocked, and hidden count rows
1. Verify continue is disabled when the scope is ambiguous

### Expected assertions

- Sheet appears before destructive, export-bearing, or provider-backed actions
- Sheet exposes included, excluded, blocked, and hidden rows distinctly
- Continue control reflects `continue_enabled` from the record

### Virtualization invariants

- Sheet survives the virtualization window scrolling underneath
- Selected count rendered on the sheet matches the count strip

### Accessibility narration

> `Batch review: 6 included, 12 excluded, 0 blocked, 2 hidden. Continue available; cancel restores selection.`

## `drill:saved-view-switcher-virtualized` -- Switch saved view under virtualization

- Surface: `log_or_event_collection`
- Drill class: `saved_view_switcher_under_virtualization`

### Steps

1. Focus the saved-view switcher
1. Select a drifted saved view
1. Verify the drift disclosure is narrated
1. Switch back and confirm anchor row id is restored

### Expected assertions

- Drift disclosure (provider/policy/columns) is announced before switching
- Fallback behavior (`preserve`, `subset`, `refuse`, `rebind`, `recreate`) is announced
- Anchor row id survives switching back to the previous saved view

### Virtualization invariants

- Loaded buffer does not collapse during switch
- Scope counter axes refresh without dropping `visible` or `loaded`

### Accessibility narration

> `Saved view changed to Errors -- last 24 h. Captured cursor stale; rebind required before reuse.`

