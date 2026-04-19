# Fixture: multicursor edit collapses to one undo group across N cursors

## Scenario

The user places three cursors on three consecutive lines and types
the character `,`. Then the user presses `Cmd-Z` once.

Representative content (before, with caret marks):

```
let a = 1|
let b = 2|
let c = 3|
```

Representative content (after):

```
let a = 1,
let b = 2,
let c = 3,
```

## Hooks exercised

- `text_edit_apply` — fires once per cursor for the keystroke (or
  once for the merged transaction, depending on instrumentation
  granularity, but always reports the same `undo_group_id`).
- `transaction_apply` — fires exactly once for the multicursor
  transaction.
- `undo_apply` — fires once for the single `Cmd-Z`.
- `redo_apply` — fires once for `Cmd-Shift-Z`.

## Undo classes emitted

- `multi_cursor_text_edit`

## Stack elements stressed

- Single-undo-group rule across N selection ranges in one
  keystroke.
- Buffer's per-cursor selection model.

## Expected observable outcomes

- One `Cmd-Z` reverts all three commas in one step. Half-revert
  (e.g. only the third line restored) is a contract violation —
  the journal recorded one `undo_group_id` for all three cursors.
- `transaction_apply` fires exactly once with `class_id =
  multi_cursor_text_edit` and a `per_cursor_edits` list of length
  three.
- `redo_apply` re-applies all three commas atomically.
- A subsequent single-cursor edit on a different line opens a new
  `text_edit` group; it does not extend the multicursor group.

## ADR sections motivating this fixture

- Undo / redo and transaction grouping — multicursor coalesces
  into one group across all cursors in one keystroke.
- Undo-class taxonomy — `multi_cursor_text_edit` row, including
  the `half_revert_observed` reopen trigger.
