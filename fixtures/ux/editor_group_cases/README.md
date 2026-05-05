# Editor-group fixtures

Seed corpus for the contract frozen in
[`/docs/ux/tabs_editor_groups_contract.md`](../../../docs/ux/tabs_editor_groups_contract.md)
and the boundary schema at
[`/schemas/ux/editor_group_state.schema.json`](../../../schemas/ux/editor_group_state.schema.json).

Each fixture is a single JSON document that exercises one working-set
case without exposing raw absolute paths, raw file bodies, raw terminal
output, raw notebook outputs, raw preview DOM, raw provider payloads, or
credential material. Identity is carried through opaque refs and short
redaction-aware labels.

## Cases

| Fixture | Record kind | Main proof |
| --- | --- | --- |
| [`dirty_generated_preview_overflow.json`](./dirty_generated_preview_overflow.json) | `editor_group_snapshot_record` | Dirty, generated, live-preview, pinned, read-only, blocked, preview, and overflow states remain visible and attributable without icon-only cues. |
| [`compare_tabbed_overflow_mixed.json`](./compare_tabbed_overflow_mixed.json) | `editor_group_snapshot_record` | Tabbed compare roles stay legible under overflow alongside dirty, pinned/read-only, followed, and blocked tabs. |
| [`split_move_restore_identity.json`](./split_move_restore_identity.json) | `editor_group_transition_record` | Split, move, merge, move-to-window, reopen, and restore operations preserve tab, pane, group-lineage, dirty-authority, and focus identity. |
| [`compare_min_width_fallback.json`](./compare_min_width_fallback.json) | `compare_fallback_decision_record` | A requested split compare falls back to tabbed compare before violating minimum useful widths. |
| [`restored_missing_dependency_shared_followed.json`](./restored_missing_dependency_shared_followed.json) | `editor_group_snapshot_record` | Restored layout-only and evidence-only tabs keep missing-dependency placeholders, pinned/read-only combinations, and shared/followed state legible in overflow. |

## Intended usage

- Renderer and layout code can use these fixtures to test tab overflow,
  title recovery, and minimum-width fallback.
- Restore code can assert skeleton-first group recreation before
  dependency hydration.
- Support export code can verify group lineage and dirty attribution
  without exporting private content.
- Compare and diff code can verify tabbed compare, staged peek, and
  explicit-choice fallback before unusable narrow panes are created.
