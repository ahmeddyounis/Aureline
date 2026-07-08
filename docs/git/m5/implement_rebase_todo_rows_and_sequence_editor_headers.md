# M5 rebase todo rows and sequence-editor headers: ordered-plan and checkpoint truth

This lane (M05-959) narrows the two **sequence-edit** components frozen in the
[M5 Git-history and risky-mutation component matrix](./freeze_the_m5_git_history_sequence_component_matrix.md)
— `rebase_todo_row` and `sequence_editor_header` — into an implemented,
export-safe row contract so every claimed M5 sequence-edit surface can render an
ordered change plan with commit identity, operation, original order, unresolved
blockers, and checkpoint state without copying per-screen chrome.

- Rust module: `crates/aureline-git/src/implement_rebase_todo_rows_and_sequence_editor_headers/`
- Boundary schema: [`schemas/ui/m5-rebase-todo-sequence-editor-component.schema.json`](../../../schemas/ui/m5-rebase-todo-sequence-editor-component.schema.json)
- Checked support export: [`artifacts/release/m5-rebase-todo-sequence-editor-components-proof/support_export.json`](../../../artifacts/release/m5-rebase-todo-sequence-editor-components-proof/support_export.json)
- Protected fixtures: [`fixtures/ui/m5-rebase-todo-sequence-editor-components/`](../../../fixtures/ui/m5-rebase-todo-sequence-editor-components/)

## Goal

Make sequence editing feel like a truthful ordered change plan rather than a
lossy checklist. A user can review and modify a rebase plan without ever losing
the original order/identity context, and the raw-todo fallback and the structured
card stay meaning-equivalent so the same sequence never appears to mean different
things across desktop, CLI/help, and export surfaces.

## Rebase todo rows: the original-order-and-identity axis

A `RebaseTodoRow` carries the commit's short id, subject, and author
(`CommitIdentityMissing`), its `original_index` and `display_index`, its rebase
`operation` (pick, reword, edit, squash, fixup, drop — never collapsed), and its
`plan_state`. The plan state is **derived**, never independently asserted:
`resolve_todo_plan_state(operation, original_index, display_index)` decides
whether the row is `unchanged`, `reordered`, `squashed_into_previous`, or
`dropped`, and a claimed value that disagrees fails with
`PlanStateMisrepresented`. Drop takes precedence, then squash/fixup, then a
changed position.

| Operation | Rewrites history? | Derived plan state (same position) |
| --- | --- | --- |
| `pick` | no | `unchanged` |
| `reword` | yes (message) | `unchanged` |
| `edit` | yes (content) | `unchanged` |
| `squash` / `fixup` | yes (folds) | `squashed_into_previous` |
| `drop` | yes (removes) | `dropped` |

Unresolved blockers (`SequenceBlockerKind`) are always disclosed
(`UnresolvedBlockerNotDisclosed`), and any history-rewriting step must keep a
reachable recovery checkpoint (`RecoveryCheckpointMissing`,
`SequenceCheckpointState`). At least one `unchanged` row must remain in the corpus
so it proves original order survives editing
(`OriginalOrderPreservationCoverageMissing`).

## Raw-todo fallback stays meaning-equivalent

Each row's `raw_todo_line` (the `pick <sha> <subject>`-style git-rebase-todo
line) must lead with the structured operation's verb token **and** name the same
commit short id (`RawTodoLineMisaligned`). This is the raw-fallback /
structured-card equivalence check: the same sequence can never mean two different
things across surfaces.

## Sequence-editor headers: the whole-session recovery anchor

A `SequenceEditorHeaderRow` names the `onto_ref` (`OntoRefMissing`), the
`original_tip_ref` that anchors recovery (`OriginalTipRecoveryAnchorMissing`), and
how original order is preserved (`OriginalOrderContextMissing`). Its
reordered/squashed/dropped counts stay within the total
(`HeaderCountsInconsistent`), it confirms as a full
`sequence_rewrite_confirm` rather than one ambiguous confirm
(`SequenceConfirmCollapsed`, reusing the frozen `MutationReviewClass`), and any
session that rewrites history keeps a reachable checkpoint
(`HeaderRecoveryCheckpointMissing`).

## Reuse

- `M5GitHistoryComponent` gates each row's `component` (todo rows must be
  `rebase_todo_row`, header rows must be `sequence_editor_header`).
- `GitHistoryDowngradeState` (the shared matrix downgrade vocabulary) is reused
  for both per-row `downgrade_vocab` and packet-level `downgrade_triggers`.
- `ComponentConsumerSurface` (the shared matrix consumer surfaces) is reused for
  `consumer_surfaces`.
- `MutationReviewClass` (the shared matrix mutation-review class) is reused for
  each header's `review_class`, kept at `sequence_rewrite_confirm`.

## Acceptance criteria mapping

- **Users can review and modify sequence plans without losing original
  order/identity context** — `PlanStateMisrepresented` + `CommitIdentityMissing` +
  `OriginalTipRecoveryAnchorMissing` + `OriginalOrderContextMissing` +
  `OriginalOrderPreservationCoverageMissing`.
- **Structured and raw sequence views stay meaning-equivalent across desktop,
  CLI/help, and export surfaces** — `RawTodoLineMisaligned` +
  `raw_and_structured_meaning_equivalent` + `raw_fallback_equivalent_across_surfaces`.

## Regenerating artifacts

The checked export, Markdown summary, and narrowed fixtures are produced by the
`generate_artifacts` test, gated behind an env var so it is inert in CI:

```
GEN_REBASE_SEQUENCE_ARTIFACTS=1 cargo test -p aureline-git --lib \
  implement_rebase_todo_rows_and_sequence_editor_headers::tests::generate_artifacts
```

`checked_export_matches_seed` asserts the checked JSON equals the in-Rust seed
packet, so the artifact can never drift from the contract.
