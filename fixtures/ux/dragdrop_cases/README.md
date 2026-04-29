# Drag-and-drop worked-example corpus

Seed corpus for the contract frozen in
[`/docs/ux/clipboard_history_contract.md`](../../../docs/ux/clipboard_history_contract.md)
§6 and for the `dragdrop_case` axes referenced by the
interaction-safety packets in
[`/docs/ux/shell_interaction_safety_contract.md`](../../../docs/ux/shell_interaction_safety_contract.md).

Every file is a standalone JSON document. A fixture is a
**seed**: it pins the drop-target surface, the resolved
`drop_result_verb`, the modifier-key cue, the insertion-preview
axes, the consequence + recovery class, the cross-window
detach posture, the responsive-fallback posture, and the
failure-tier placement when the drop refuses to commit.

Every fixture:

- Resolves every axis to vocabulary already frozen in the
  clipboard / history contract §3 or in the re-exported
  upstream contracts (shell-interaction-safety, undo-class
  rows, source-fidelity packet, text-fidelity packet,
  state-and-recovery taxonomy, navigation-and-escalation
  contract, attention / activity taxonomy,
  suspicious-content packet, a11y packet).
- Cites at least one `next_step_decision_hook`, one
  recovery-class token (or `evidence_only_no_rerun` where
  applicable), one failure-tier id when the drop refuses to
  commit, and one fixture-level
  `overclaims_reversibility = false` assertion.
- Asserts that the resolved verb, the insertion-preview, the
  modifier-key cue, and the required-visible-field set are
  keyboard-reachable and announced.
- Carries no raw absolute paths, raw URLs, raw credential
  material, raw prompt text, or raw artifact bytes. Every
  identity is an opaque ref; every timestamp is an ISO-8601
  placeholder.
- Honours the forbidden-generic-copy rule — `Working...`,
  `Something went wrong`, `Error`, `Failed`, `Try again`,
  `Unavailable` are quoted only in `forbidden_rendering`,
  never as the surface's real copy.

## Cases

| Fixture | Drop target | Verb | Modifier | Consequence | Recovery |
| --- | --- | --- | --- | --- | --- |
| [`editor_move_file_within_workspace.json`](./editor_move_file_within_workspace.json) | `editor_canvas` / explorer row | `move` | (none) | `reversible_local` | `exact_undo` |
| [`explorer_copy_on_option_modifier.json`](./explorer_copy_on_option_modifier.json) | `editor_canvas` / explorer row | `copy` | Option / Alt | `reversible_local` | `exact_undo` |
| [`review_attach_evidence.json`](./review_attach_evidence.json) | `review_and_diff_canvas` | `attach` | (none) | `recoverable_durable` | `restore_from_checkpoint` |
| [`install_attach_import_verified_publisher.json`](./install_attach_import_verified_publisher.json) | `install_update_attach_canvas` | `import` | Command+Option / Ctrl+Alt | `external_shared` | `restore_from_checkpoint` |
| [`terminal_paste_then_run_blocked_until_preview.json`](./terminal_paste_then_run_blocked_until_preview.json) | `terminal_canvas` | `attach` → `blocked` until preview approved | (none) | `external_shared` | `evidence_only_no_rerun` |
| [`protected_path_drop_blocked.json`](./protected_path_drop_blocked.json) | `editor_canvas` / protected path | `blocked` | (any) | n/a | `no_recovery_available` |
| [`cross_window_detach_preserves_lineage.json`](./cross_window_detach_preserves_lineage.json) | `review_and_diff_canvas` → detached companion window | `open` | (none) | `recoverable_durable` | `restore_from_checkpoint` |
| [`compact_shell_drop_denies_hidden_required_field.json`](./compact_shell_drop_denies_hidden_required_field.json) | `install_update_attach_canvas` in compact-shell | `import` → denial | Command+Option / Ctrl+Alt | `external_shared` | n/a (denied) |

## Schema references

- Dedicated cross-window transfer contract:
  [`/docs/ux/cross_window_transfer_contract.md`](../../../docs/ux/cross_window_transfer_contract.md).
- Dedicated cross-window transfer schema:
  [`/schemas/ux/window_transfer_action.schema.json`](../../../schemas/ux/window_transfer_action.schema.json).
- Dedicated cross-window transfer fixtures:
  [`/fixtures/ux/cross_window_transfer_cases/`](../cross_window_transfer_cases/).
- Clipboard / history contract:
  [`/docs/ux/clipboard_history_contract.md`](../../../docs/ux/clipboard_history_contract.md).
- Undo-group lineage examples:
  [`/artifacts/ux/undo_group_examples.yaml`](../../../artifacts/ux/undo_group_examples.yaml).
- Shell interaction-safety schema:
  [`/schemas/ux/interaction_safety.schema.json`](../../../schemas/ux/interaction_safety.schema.json).
- Shell interaction-safety contract:
  [`/docs/ux/shell_interaction_safety_contract.md`](../../../docs/ux/shell_interaction_safety_contract.md).
- Undo-class rows:
  [`/artifacts/architecture/undo_class_rows.yaml`](../../../artifacts/architecture/undo_class_rows.yaml).
- Source-fidelity / undo packet:
  [`/docs/verification/source_fidelity_and_undo_packet.md`](../../../docs/verification/source_fidelity_and_undo_packet.md).
- State-and-recovery taxonomy:
  [`/docs/ux/state_and_recovery_taxonomy.md`](../../../docs/ux/state_and_recovery_taxonomy.md).

## Build identity

Every fixture reserves `running_build_identity_ref` as an
opaque seed id. A later lane resolves it against the build-
identity record without renaming the field.
