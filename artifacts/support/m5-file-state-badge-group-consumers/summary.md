# File-State Badge Groups & Reason Strips: One Vocabulary Across Surfaces

- Packet: `m5-file-state-badge-group-consumers:stable:0001`
- Surface: `M5 file-state badge groups & reason strips (one vocabulary across surfaces)`
- Consumer bindings: 19 (12 narrowed, 7 multi-state)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-16T00:00:00Z)

## Consumer bindings

- **Read-only vendored source (blocked in-place write)** [`fsbg-readonly-editor`]: object `read_only` on `editor_banner`, posture `full_badge_group`, role `state_badge_classification`
- **Read-only vendored source (blocked in-place write)** [`fsbg-readonly-tab`]: object `read_only` on `tab_chrome`, posture `compact_status_chip`, role `state_badge_classification`
- **Read-only vendored source (blocked in-place write)** [`fsbg-readonly-support`]: object `read_only` on `support_export_packet`, posture `export_redacted`, role `state_badge_classification`
- **Generated artifact that is also policy locked (multi-state)** [`fsbg-generated-editor`]: object `generated` (+ policy_locked) on `editor_banner`, posture `full_badge_group`, role `blocked_write_reason`
- **Generated artifact that is also policy locked (multi-state)** [`fsbg-generated-diff`]: object `generated` (+ policy_locked) on `diff_review_header`, posture `full_badge_group`, role `blocked_write_reason`
- **Generated artifact that is also policy locked (multi-state)** [`fsbg-generated-status`]: object `generated` (+ policy_locked) on `status_bar`, posture `compact_status_chip`, role `blocked_write_reason`
- **Generated artifact that is also policy locked (multi-state)** [`fsbg-generated-palette`]: object `generated` (+ policy_locked) on `command_palette`, posture `palette_availability_gated`, role `blocked_write_reason`
- **Policy-locked protected config (approval-gated write)** [`fsbg-policy-writesheet`]: object `policy_locked` on `write_review_sheet`, posture `full_badge_group`, role `canonical_source_relation`
- **Policy-locked protected config (approval-gated write)** [`fsbg-policy-breadcrumb`]: object `policy_locked` on `breadcrumb_trail`, posture `compact_status_chip`, role `canonical_source_relation`
- **Policy-locked protected config (approval-gated write)** [`fsbg-policy-support`]: object `policy_locked` on `support_export_packet`, posture `export_redacted`, role `canonical_source_relation`
- **Managed mirror that is also a captured snapshot (multi-state)** [`fsbg-managed-writesheet`]: object `managed` (+ captured_snapshot) on `write_review_sheet`, posture `full_badge_group`, role `exact_write_target`
- **Managed mirror that is also a captured snapshot (multi-state)** [`fsbg-managed-ai`]: object `managed` (+ captured_snapshot) on `ai_automation_path`, posture `palette_availability_gated`, role `exact_write_target`
- **Managed mirror that is also a captured snapshot (multi-state)** [`fsbg-managed-status`]: object `managed` (+ captured_snapshot) on `status_bar`, posture `compact_status_chip`, role `exact_write_target`
- **Projection / virtual view (writes resolve to the backing source)** [`fsbg-projection-diff`]: object `projection` on `diff_review_header`, posture `full_badge_group`, role `canonical_source_relation`
- **Projection / virtual view (writes resolve to the backing source)** [`fsbg-projection-palette`]: object `projection` on `command_palette`, posture `palette_availability_gated`, role `canonical_source_relation`
- **Projection / virtual view (writes resolve to the backing source)** [`fsbg-projection-tab`]: object `projection` on `tab_chrome`, posture `compact_status_chip`, role `canonical_source_relation`
- **Captured snapshot of a preserved past state (not the current live object)** [`fsbg-snapshot-editor`]: object `captured_snapshot` on `editor_banner`, posture `full_badge_group`, role `state_badge_classification`
- **Captured snapshot of a preserved past state (not the current live object)** [`fsbg-snapshot-ai`]: object `captured_snapshot` on `ai_automation_path`, posture `palette_availability_gated`, role `state_badge_classification`
- **Captured snapshot of a preserved past state (not the current live object)** [`fsbg-snapshot-support`]: object `captured_snapshot` on `support_export_packet`, posture `export_redacted`, role `state_badge_classification`
