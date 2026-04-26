# Sequence-help and leader-overlay fixtures

Worked examples for
[`/docs/commands/sequence_and_modal_discoverability_contract.md`](../../../docs/commands/sequence_and_modal_discoverability_contract.md)
and
[`/schemas/commands/leader_overlay.schema.json`](../../../schemas/commands/leader_overlay.schema.json).

Each fixture carries one top-level record from the modal and sequence
discoverability boundary:

- `modal_state_cue_record`
- `sequence_help_row_record`
- `leader_overlay_row_record`
- `shortcut_teaching_row_record`
- `command_language_parity_row_record`

Fixtures include fixture-only `$schema` and `__fixture__` keys for
humans. Validators strip those two keys before validating the top-level
record.

## Cases

- [`leader_overlay_partial_git.json`](./leader_overlay_partial_git.json)
  - partial leader prefix with waiting state, next-key guidance, visible
  macro/register cues, timeout/cancel hint, and pivots to palette,
  settings, docs, and migration guidance.
- [`operator_pending_delete_count.json`](./operator_pending_delete_count.json)
  - operator-pending sequence with count, pending delete operator,
  destructive-scope guidance, and keyboard-only recovery routes.
- [`shortcut_teaching_import_conflict.json`](./shortcut_teaching_import_conflict.json)
  - frequent-action shortcut teaching after an imported keymap creates
  a reviewable conflict with an extension binding.
- [`colon_write_parity.json`](./colon_write_parity.json)
  - colon-style command entry proving palette, leader overlay, modal
  sequence help, docs, and automation all project the same command id.
