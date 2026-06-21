# Dictation edit contract

Generated from the `voice_input` seed. Do not edit by hand; regenerate with `cargo run -p aureline-editor --bin aureline_dictation_edit_parity -- write`.

- Packet: `editor:dictation_edit_parity:packet:v1`
- Voice/dictation contract: `docs/ux/voice_and_dictation_contract.md`
- Fixtures: `fixtures/voice/dictation-edit-parity`

## Claimed text-entry surfaces

| Surface | Class | Support | Reason |
| --- | --- | --- | --- |
| `editor.main` | main_editor | supported | `label:dictation:editor_main_full` |
| `field.rename` | single_line_text_field | supported | `label:dictation:rename_field_full` |
| `scm.commit_message` | multi_line_text_area | supported | `label:dictation:commit_message_full` |
| `field.find` | single_line_text_field | degraded_text_only | `label:dictation:find_field_text_only` |
| `terminal.integrated` | terminal | unsupported | `label:dictation:terminal_not_wired` |
| `notebook.cell` | notebook_cell | unsupported | `label:dictation:notebook_not_wired` |
| `extension.custom` | custom_widget | unsupported | `label:dictation:custom_widget_not_wired` |

## Parity scenarios

| Scenario | Surface | Outcome | Edits | Undo/redo | History group | Restored |
| --- | --- | --- | --- | --- | --- | --- |
| `dictation:scenario:dictate_sentence_main_editor` | `editor.main` | applied | 4 | round-trips | parity-clean | - |
| `dictation:scenario:scratch_that_then_redictate` | `editor.main` | applied | 3 | round-trips | - | - |
| `dictation:scenario:commit_message_hosted_provider` | `scm.commit_message` | applied | 3 | round-trips | parity-clean | - |
| `dictation:scenario:cancel_restores_insertion_point` | `field.rename` | cancelled_restored | 0 | - | - | yes |
| `dictation:scenario:terminal_unsupported` | `terminal.integrated` | rejected_unsupported | 0 | - | - | - |
| `dictation:scenario:find_field_degraded` | `field.find` | rejected_degraded | 1 | - | - | - |

## Invariants

- [x] Every dictated edit rides an ordinary text-edit undo class
- [x] Every content edit routes through the shared edit model
- [x] Undo and redo round-trip predictably
- [x] Cancelling a capture restores the prior insertion point
- [x] Unsupported / degraded surfaces reject explicitly
- [x] History groups are parity-clean
- [x] No hidden speech-only buffer
