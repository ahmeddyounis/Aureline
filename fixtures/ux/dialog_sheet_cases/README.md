# Dialog and sheet review-surface cases

These fixtures exercise
[`docs/ux/dialog_sheet_contract.md`](../../../docs/ux/dialog_sheet_contract.md)
and validate against
[`schemas/ux/review_surface.schema.json`](../../../schemas/ux/review_surface.schema.json).

| Case | Purpose |
|---|---|
| [`binary_destructive_confirm_modal.json`](./binary_destructive_confirm_modal.json) | A short destructive choice uses a small modal with target, consequence, safe default focus, and specific labels. |
| [`structured_permission_review_sheet.json`](./structured_permission_review_sheet.json) | A permission review uses a sheet because target, scope, denial path, revocation, and policy context must remain visible. |
| [`dense_diff_review_takeover.json`](./dense_diff_review_takeover.json) | A dense diff and evidence review requires a dedicated surface rather than a modal. |
| [`multi_step_setup_handoff_sheet.json`](./multi_step_setup_handoff_sheet.json) | Multi-step setup stays in a sheet and hands long-running work to a durable job row before close. |
| [`nested_product_overlay_denied.json`](./nested_product_overlay_denied.json) | A product-owned nested modal request is denied instead of stacking overlays. |
| [`platform_auth_overlay_exception.json`](./platform_auth_overlay_exception.json) | A platform auth overlay is admitted as the narrow exception and resumes to the invoking sheet. |
| [`ai_context_egress_trust_prompt_sheet.json`](./ai_context_egress_trust_prompt_sheet.json) | An AI tool trust/policy prompt uses a sheet so scope, denial path, and revocation remain visible with explicit action labels. |
| [`publish_review_sheet.json`](./publish_review_sheet.json) | A publish review flow uses a sheet because audience, redaction posture, and recovery context must be inspectable before commit. |
| [`revoke_published_review_confirm_modal.json`](./revoke_published_review_confirm_modal.json) | A short revoke action uses a modal with a visible consequence block and safe default focus on keeping the published state. |
| [`restricted_mode_reopen_prompt_sheet.json`](./restricted_mode_reopen_prompt_sheet.json) | A restricted-mode reopen prompt uses a sheet to keep trust consequences and safe fallback options visible. |
| [`package_review_sheet.json`](./package_review_sheet.json) | A package review uses a sheet because script risk, lockfile impact, and rollback posture require scanning space and explicit labels. |
| [`recovery_reset_review_sheet.json`](./recovery_reset_review_sheet.json) | A recovery/reset review uses a sheet to keep consequences, export-before-reset, and recovery paths inspectable before commit. |
