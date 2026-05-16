# Change-lineage alpha fixtures

These checked-in JSON fixtures drive the alpha change-lineage family
validated against
[`/schemas/review/change_lineage.schema.json`](../../../../schemas/review/change_lineage.schema.json)
and projected by `aureline_review::change_inspector`. Every fixture is a
review-time projection of one of the alpha change-object fixtures under
[`/fixtures/workspace/m3/change_objects/`](../../../workspace/m3/change_objects/),
extended with conflict-state and publish-readiness truth so the
landing-state inspector can render one row before publish, merge, or
apply.

| Fixture | `change_object_kind` | `active_scope_class` | `publish_readiness_class` | Conflict state | Why it exists |
| --- | --- | --- | --- | --- | --- |
| `branch_main_worktree_ready_to_publish.json` | `branch` | `main_worktree` | `ready_to_publish` | `no_conflicts_detected` | A main-worktree branch ahead of the team base, conflict-free, ready to publish. |
| `branch_blocked_by_review_required.json` | `branch` | `main_worktree` | `blocked_by_review_required` | `no_conflicts_detected` | Conflict-free branch held by managed-workspace review approval. |
| `branch_landed_publicly_inspect_only.json` | `branch` | `detached_inspection` | `not_applicable_inspect_only` | `no_conflicts_detected` | Already-landed public branch shown inspect-only. |
| `worktree_side_worktree_inspect_only.json` | `worktree` | `side_worktree` | `not_applicable_inspect_only` | `no_conflicts_detected` | Side-worktree review playground; inspectable without a remote target. |
| `patch_stack_blocked_by_conflicts.json` | `patch_stack` | `stacked_patch_set` | `blocked_by_conflicts` | `upstream_diverged_requires_rebase` | Three-patch stack riding an open provider review; rebase required first. |

Every fixture keeps raw paths, raw branch names, raw remote URLs, and raw
diff bodies closed; only opaque ref labels and class tokens cross the
boundary. Each fixture quotes the matching `change_object_ref` so review
and support packets read one lineage truth.
