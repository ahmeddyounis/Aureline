# Change-object alpha fixtures

These checked-in JSON fixtures drive the alpha change-object family validated
against
[`/schemas/workspace/change_object.schema.json`](../../../../schemas/workspace/change_object.schema.json)
and projected by `aureline_git::change_objects`. They cover the three
change-object kinds (`branch`, `worktree`, `patch_stack`) plus a public-landed
and a pending-merge case so support and review exports can quote one record
truth ahead of publish, merge, or apply.

| Fixture | `change_object_kind` | `landing_state_class` | Why it exists |
| --- | --- | --- | --- |
| `branch_local_pending_publish.json` | `branch` | `pending_publish_to_remote` | Local branch awaiting publish to a provider-bound origin. |
| `branch_pending_merge.json` | `branch` | `pending_merge_into_base` | Local branch pending merge under managed-workspace authority. |
| `branch_landed_publicly.json` | `branch` | `landed_publicly` | Inspect-only record for a branch that has already landed publicly. |
| `worktree_linked_local_only.json` | `worktree` | `local_only_no_remote_yet` | Linked local worktree; no remote attached. |
| `patch_stack_provider_pull_request.json` | `patch_stack` | `pending_patch_apply` | Three-patch stack pending apply onto an open provider review. |

Every fixture keeps raw paths, raw branch names, raw remote URLs, and raw diff
bodies closed; only opaque ref labels and class tokens cross the boundary.
