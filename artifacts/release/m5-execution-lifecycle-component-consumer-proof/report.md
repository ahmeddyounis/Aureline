# M5 Execution-Lifecycle Component Consumers

- Packet: `m5-execution-lifecycle-component-consumers:stable:0001`
- As of: `2026-07-04T00:00:00Z`
- Rows: 15 across 5 consumer groups and 5 / 5 frozen families
- Families reused across groups: 5

## Rows

- **consumer:task-test:run-attempt-header** — surface=task_run_pane group=task_test family=run_attempt_header authority=full label_parity=preserved handoff=none
- **consumer:task-test:rerun-review** — surface=test_explorer group=task_test family=rerun_review authority=full label_parity=preserved handoff=none
- **consumer:task-test:debug-hierarchy** — surface=task_run_pane group=task_test family=debug_hierarchy authority=full label_parity=preserved handoff=none
- **consumer:request-database:run-attempt-header** — surface=request_run_pane group=request_database family=run_attempt_header authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:request-database:input-request-prompt** — surface=database_execution_pane group=request_database family=input_request_prompt authority=full label_parity=preserved handoff=none
- **consumer:request-database:artifact-publish-row** — surface=request_run_pane group=request_database family=artifact_publish_row authority=full label_parity=preserved handoff=none
- **consumer:notebook-preview:run-attempt-header** — surface=notebook_execution_cell group=notebook_preview family=run_attempt_header authority=full label_parity=preserved handoff=none
- **consumer:notebook-preview:input-request-prompt** — surface=notebook_execution_cell group=notebook_preview family=input_request_prompt authority=full label_parity=preserved handoff=none
- **consumer:notebook-preview:artifact-publish-row** — surface=preview_runtime_lane group=notebook_preview family=artifact_publish_row authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:ai-publish:rerun-review** — surface=ai_mediated_run group=ai_publish family=rerun_review authority=inspect_only label_parity=disclosed_narrowed handoff=companion_app
- **consumer:ai-publish:artifact-publish-row** — surface=publish_deploy_flow group=ai_publish family=artifact_publish_row authority=full label_parity=preserved handoff=none
- **consumer:ai-publish:debug-hierarchy** — surface=ai_mediated_run group=ai_publish family=debug_hierarchy authority=inspect_only label_parity=disclosed_narrowed handoff=none
- **consumer:support-export:run-attempt-header** — surface=support_export_replay group=support_export family=run_attempt_header authority=export_only label_parity=disclosed_narrowed handoff=handoff_packet
- **consumer:support-export:rerun-review** — surface=history_activity_center group=support_export family=rerun_review authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:support-export:debug-hierarchy-docs** — surface=help_center_docs group=support_export family=debug_hierarchy authority=read_only label_parity=disclosed_narrowed handoff=none
