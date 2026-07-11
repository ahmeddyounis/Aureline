# M5 Notebook-Kernel-Output Component Consumers

- Packet: `m5-notebook-kernel-output-component-consumers:stable:0001`
- As of: `2026-07-11T00:00:00Z`
- Rows: 16 across 6 consumer classes and 8 / 8 frozen families
- Controls lanes adopted: 4 / 4
- Kernel/output dispositions preserved: 13 / 13
- Families reused across classes: 7

## Rows

- **consumer:notebook-editor:notebook-document-header** — surface=notebook_ui class=notebook_editor family=notebook_document_header lane=document_kernel authority=full label_parity=preserved handoff=none
- **consumer:notebook-editor:kernel-state-strip** — surface=notebook_ui class=notebook_editor family=kernel_state_strip lane=document_kernel authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:notebook-editor:kernel-picker-row** — surface=kernel_manager_ui class=notebook_editor family=kernel_picker_row lane=kernel_choice authority=override_gated label_parity=disclosed_narrowed handoff=none
- **consumer:notebook-editor:kernel-origin-pill** — surface=kernel_manager_ui class=notebook_editor family=kernel_origin_pill lane=kernel_choice authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:notebook-editor:output-trust-banner** — surface=output_viewer_ui class=notebook_editor family=output_trust_banner lane=output_trust authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:notebook-editor:output-provenance-chip-group** — surface=output_viewer_ui class=notebook_editor family=output_provenance_chip_group lane=output_trust authority=inspect_only label_parity=disclosed_narrowed handoff=none
- **consumer:notebook-editor:kernel-recovery-card** — surface=notebook_ui class=notebook_editor family=kernel_recovery_card lane=restart_recovery authority=override_gated label_parity=disclosed_narrowed handoff=none
- **consumer:diff-review:output-trust-banner** — surface=review_ui class=diff_review family=output_trust_banner lane=output_trust authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:diff-review:restart-consequence-card** — surface=review_ui class=diff_review family=restart_consequence_card lane=restart_recovery authority=inspect_only label_parity=disclosed_narrowed handoff=none
- **consumer:debug:kernel-state-strip** — surface=debugger_ui class=debug family=kernel_state_strip lane=document_kernel authority=inspect_only label_parity=disclosed_narrowed handoff=none
- **consumer:debug:restart-consequence-card** — surface=debugger_ui class=debug family=restart_consequence_card lane=restart_recovery authority=override_gated label_parity=disclosed_narrowed handoff=none
- **consumer:ai-context:kernel-origin-pill** — surface=ai_context_ui class=ai_context family=kernel_origin_pill lane=kernel_choice authority=inspect_only label_parity=disclosed_narrowed handoff=none
- **consumer:ai-context:output-provenance-chip-group** — surface=ai_context_ui class=ai_context family=output_provenance_chip_group lane=output_trust authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:cli:output-trust-banner** — surface=cli_surface class=cli family=output_trust_banner lane=output_trust authority=export_only label_parity=disclosed_narrowed handoff=cli_headless
- **consumer:support-export:notebook-document-header** — surface=support_export class=support_export family=notebook_document_header lane=document_kernel authority=export_only label_parity=disclosed_narrowed handoff=support_packet
- **consumer:support-export:kernel-recovery-card** — surface=support_export class=support_export family=kernel_recovery_card lane=restart_recovery authority=export_only label_parity=disclosed_narrowed handoff=support_packet
