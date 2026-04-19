# Fixture: single-cursor typing coalesces into one undo group per word

## Scenario

The user opens a small source file at column zero of an empty line
and types the word `hello` followed by a space and the word `world`.
Then the user presses `Escape`, moves the caret to a different line,
and presses `Backspace` once.

Representative content (before):

```
fn greet() {
    |
}
```

Representative content (after):

```
fn greet() {
    hello world
}
```

## Hooks exercised

- `text_edit_apply` — fires once per character commit.
- `transaction_apply` — fires once per coalesced undo group as it
  commits to the journal.
- `undo_apply` — fires once per `Cmd-Z` / `Ctrl-Z`.

## Undo classes emitted

- `text_edit`

## Stack elements stressed

- Coalescing rules: typing inside one cursor coalesces until a word
  boundary, caret jump, selection change, or idle window.
- Per-buffer undo journal append.

## Expected observable outcomes

- The five characters of `hello` form one undo group; the space and
  the five characters of `world` form a second undo group.
- After the caret jump, the next typed character starts a third
  undo group.
- One `Cmd-Z` removes `world` (and the preceding space) but leaves
  `hello`. A second `Cmd-Z` removes `hello`. Each undo emits one
  `undo_apply`.
- `text_edit_apply` fires for every character commit; the journal
  records one transaction per coalesced group, not one per
  keystroke.
- The redo stack survives both undo steps; redoing twice restores
  the original `hello world` state.

## ADR sections motivating this fixture

- Undo / redo and transaction grouping — single-cursor coalescing
  rules.
- Undo-class taxonomy — `text_edit` row.
