# Fixture: single-file rename refactor with named undo group

## Scenario

The user invokes "Rename Local Symbol" on the local variable `x`
inside one function in one file. The refactor previews seven
occurrences inside that function. The user accepts the preview.
Then the user presses `Cmd-Z` once.

Representative content (before):

```
fn sum(values: &[i32]) -> i32 {
    let mut x = 0;
    for v in values { x += v; }
    x
}
```

Representative content (after):

```
fn sum(values: &[i32]) -> i32 {
    let mut total = 0;
    for v in values { total += v; }
    total
}
```

## Hooks exercised

- `undo_group_open` — fires once when the refactor opens its named
  group.
- `transaction_apply` — fires once per occurrence rewritten (seven
  times here), each carrying the same `undo_group_id`.
- `undo_group_close` — fires once when the refactor commits.
- `undo_apply` — fires once for the single `Cmd-Z` and reverts all
  seven occurrences.

## Undo classes emitted

- `refactor_single_file`

## Stack elements stressed

- Named undo group lifecycle (open → N transactions → close).
- Compensatable posture: the inverse is itself a legal forward
  operation, so redo survives a divergent edit elsewhere in the
  file (until the divergent edit overlaps the renamed range).

## Expected observable outcomes

- The journal records seven transactions with `class_id =
  refactor_single_file` sharing one `undo_group_id` and a non-null
  `label` like "rename `x` to `total`".
- `Cmd-Z` reverts all seven occurrences atomically.
- After undo, a single `Cmd-Shift-Z` re-applies all seven.
- After undo + a divergent edit on an unrelated line + `Cmd-Shift-Z`,
  redo still re-applies the seven occurrences (compensatable).
- The accessibility tree publishes the rename's
  `undo_group_open` / `undo_group_close` so an assistive technology
  can announce "rename applied to seven occurrences" or undo it as
  one step.

## ADR sections motivating this fixture

- Undo / redo and transaction grouping — named undo groups for
  command-originated mutations.
- Undo-class taxonomy — `refactor_single_file` row.
