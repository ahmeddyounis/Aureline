# Banner / inline notice fixtures

Worked fixtures for [`/docs/ux/banner_notice_contract.md`](../../../docs/ux/banner_notice_contract.md)
and [`/schemas/ux/context_notice.schema.json`](../../../schemas/ux/context_notice.schema.json).

Each JSON file is a `context_notice_case_record` that exercises:

- notice class (`info`/`warning`/`error`/`restricted`/`success_long_lived`)
- narrowest truthful scope (`file`/`pane`/`workspace`/`account`)
- placement ladder (`inline_notice`/`banner`/`review_sheet`/`durable_activity_state`)
- dismissal semantics (“still visible after dismissal” via a durable indicator)
- banner coalescing (no stacking when a summary banner suffices)

## Cases

| Fixture | Scenario axis |
| --- | --- |
| [`restricted_workspace.json`](./restricted_workspace.json) | Workspace is restricted; blocked operations remain visible, and dismissal cannot clear the restriction. |
| [`policy_block_inline_action.json`](./policy_block_inline_action.json) | A specific action is policy-blocked; inline notice names what still works and links to policy inspection. |
| [`quota_warning_account_scope.json`](./quota_warning_account_scope.json) | Account quota warning projects as a workspace banner with account scope and a durable indicator after dismissal. |
| [`degraded_remote_state.json`](./degraded_remote_state.json) | Remote attach degrades; local editing still works; reconnect and inspect paths remain visible. |
| [`update_required_notice.json`](./update_required_notice.json) | Update required gates managed workflows; notice escalates to review sheet for details and safe next steps. |
| [`low_disk_storage_safety.json`](./low_disk_storage_safety.json) | Low disk threatens save safety; error-class notice remains durable until resolved. |
| [`corruption_recovery_success_long_lived.json`](./corruption_recovery_success_long_lived.json) | Recovery completed but verification remains; long-lived success stays visible until acknowledged. |

