# M5 File-State Badge Groups & Reason Strips — Consumer Contract (B150)

This lane ships one reusable **file-state badge group** and **reason strip** for the six B150
constrained-current-object classes frozen in the constrained-file-state matrix, and wires every shared consumer
surface to the same constrained-object profile so a current object cannot look writable in one surface and
blocked in another.

- **Module:** `crates/aureline-ui/src/m5_file_state_badge_group_and_reason_strip_consumers/`
- **Boundary schema:** `schemas/program/m5-file-state-badge-group-consumers.schema.json`
- **Support export:** `artifacts/support/m5-file-state-badge-group-consumers/support_export.json`
- **Matrix CSV:** `artifacts/support/m5-file-state-badge-group-consumers/matrix.csv`
- **Summary:** `artifacts/support/m5-file-state-badge-group-consumers/summary.md`
- **Fixtures:** `fixtures/editor/m5-file-state-badge-group-consumers/`
- **Emitter:** `cargo run -p aureline-ui --example dump_m5_file_state_badge_group_consumers -- <subcommand>`

## Controlled vocabulary

Every consumer renders the same badge grammar for a given profile:

| Grammar word | Meaning |
| --- | --- |
| `badge_role_word` | A frozen `M5ConstrainedFileStateRole` token — the "one vocabulary" gate. |
| `state_class_label_word` | The controlled state-class label (`read_only`, `generated`, `policy_locked`, `managed`, `projection`, `captured_snapshot`) shown as `Read-only`, `Generated`, `Policy locked`, `Managed`, `Projection`, `Captured snapshot`. |
| `reason_word` | The plain-language cause the reason strip carries. |
| `canonical_source_word` | The canonical source or live target the object relates back to. |
| `write_disposition_word` | The write disposition that makes the object mechanically distinct from a directly-writable object. |
| `safe_next_step_word` | The nearest safe next step (duplicate / detach / overlay / regenerate / request-approval). |
| `co_applicable_state_labels` | The controlled labels for co-applicable states on a multi-state object; both facts stay visible. |

## Consumer surfaces

`tab_chrome`, `breadcrumb_trail`, `status_bar`, `command_palette`, `editor_banner`, `diff_review_header`,
`write_review_sheet`, `ai_automation_path`, `support_export_packet`. Every surface appears in the packet, and
every object class is adopted by two or more distinct surfaces.

## Render postures

- `full_badge_group` — the full badge group + reason strip; offers the write-capable open-safe-next-step review.
- `compact_status_chip` — narrowed to a compact chip (tab / status / breadcrumb), disclosed via a note.
- `palette_availability_gated` — command-palette write availability gated behind the safe-next-step review.
- `export_redacted` — export-safe redaction of surrounding detail.

Only `full_badge_group` offers the write-capable `open_safe_next_step` action; every narrowed posture keeps the
full badge grammar and discloses the narrowing through an explicit note. There is no direct-write action, so a
silent lossy direct write cannot be represented.

## Acceptance criteria coverage

- **AC1** — the generated profile renders the same vocabulary on `editor_banner`, `diff_review_header`,
  `command_palette`, and `status_bar` for the same object.
- **AC2** — every binding names `keyboard_focusable` and `screen_reader_announced` routes so the state class,
  reason, and next safe action are discoverable without pointer-only chrome.
- **AC3** — multi-state objects (`Generated` + `Policy locked`, `Managed` + `Captured snapshot`) keep both facts
  visible; hiding a co-applicable state fails validation.

## Guardrails (each row-invariant must be `false`)

- `presents_constrained_object_as_directly_writable_or_hides_recovery_path`
- `lets_generated_managed_projection_or_archived_objects_silently_fall_back_to_lossy_direct_write`
- `gives_ai_automation_import_or_repair_flows_a_hidden_bypass`
- `leaves_canonical_source_exact_write_target_sync_or_recovery_path_unstated`
- `lets_one_state_class_hide_another_when_both_materially_affect_behavior`

Support / export consumers point back at the constrained-file-state matrix schema and the per-object domain
schema by id; raw secret values, credentials, and private endpoints stay outside the support boundary.
