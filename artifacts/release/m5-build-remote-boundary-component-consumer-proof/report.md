# M5 Build/Remote-Boundary Component Consumers

- Packet: `m5-build-remote-boundary-component-consumers:stable:0001`
- As of: `2026-07-11T00:00:00Z`
- Rows: 15 across 6 consumer classes and 8 / 8 frozen families
- Controls lanes adopted: 4 / 4
- Boundary dispositions preserved: 13 / 13
- Families reused across classes: 7

## Rows

- **consumer:run-test-debug:host-boundary-strip** — surface=run_test_debug_ui class=run_test_debug family=host_boundary_strip lane=host_origin authority=full label_parity=preserved handoff=none
- **consumer:run-test-debug:execution-origin-receipt-row** — surface=run_test_debug_ui class=run_test_debug family=execution_origin_receipt_row lane=host_origin authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:run-test-debug:adapter-confidence-chip** — surface=shell_ui class=run_test_debug family=adapter_confidence_chip lane=adapter_discovery authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:notebook:adapter-confidence-chip** — surface=notebook_ui class=notebook family=adapter_confidence_chip lane=adapter_discovery authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:notebook:discovery-diff-card** — surface=notebook_ui class=notebook family=discovery_diff_card lane=adapter_discovery authority=inspect_only label_parity=disclosed_narrowed handoff=none
- **consumer:preview:managed-workspace-lifecycle-card** — surface=preview_ui class=preview family=managed_workspace_lifecycle_card lane=managed_lifecycle authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:preview:workspace-expiry-banner** — surface=preview_ui class=preview family=workspace_expiry_banner lane=expiry_continuation authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:ai-tool-routing:execution-origin-receipt-row** — surface=product_ui class=ai_tool_routing family=execution_origin_receipt_row lane=host_origin authority=inspect_only label_parity=disclosed_narrowed handoff=none
- **consumer:ai-tool-routing:discovery-diff-card** — surface=product_ui class=ai_tool_routing family=discovery_diff_card lane=adapter_discovery authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:companion-handoff:local-safe-continuation-card** — surface=companion_ui class=companion_handoff family=local_safe_continuation_card lane=expiry_continuation authority=override_gated label_parity=disclosed_narrowed handoff=companion_app
- **consumer:companion-handoff:suspend-resume-rebuild-review-sheet** — surface=companion_ui class=companion_handoff family=suspend_resume_rebuild_review_sheet lane=managed_lifecycle authority=read_only label_parity=disclosed_narrowed handoff=companion_app
- **consumer:support-export:host-boundary-strip** — surface=support_export class=support_export family=host_boundary_strip lane=host_origin authority=export_only label_parity=disclosed_narrowed handoff=support_packet
- **consumer:support-export:managed-workspace-lifecycle-card** — surface=support_export class=support_export family=managed_workspace_lifecycle_card lane=managed_lifecycle authority=export_only label_parity=disclosed_narrowed handoff=support_packet
- **consumer:support-export:suspend-resume-rebuild-review-sheet** — surface=support_export class=support_export family=suspend_resume_rebuild_review_sheet lane=managed_lifecycle authority=export_only label_parity=disclosed_narrowed handoff=support_packet
- **consumer:incident:workspace-expiry-banner** — surface=incident_ui class=support_export family=workspace_expiry_banner lane=expiry_continuation authority=read_only label_parity=disclosed_narrowed handoff=none
