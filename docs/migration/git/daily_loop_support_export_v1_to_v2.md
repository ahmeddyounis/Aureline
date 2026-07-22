<!--
SPDX-FileCopyrightText: 2026 Aureline contributors
SPDX-License-Identifier: Apache-2.0
-->

# Daily-loop support export v1 to v2

## Why v2 is required

`git_daily_loop_support_export_record` v1 copied the complete
`DailyLoopTarget` into a record that simultaneously declared raw-path export
forbidden. That target could contain repository and worktree absolute paths,
the Git-directory path, caller refs, branch names, and display labels. The
shape contradicted the support and security boundary, so v1 is not safe to
forward as a support export.

Version 2 is a privacy-narrowing replacement. The record kind is unchanged and
the schema version is `2`, but `target` is now a strict redacted projection.
The owning schema is
[`daily_loop_support_export.schema.json`](../../../schemas/git/daily_loop_support_export.schema.json).

## Field mapping

| V1 source | V2 field | Rule |
|---|---|---|
| `target.workspace_ref` | `target.workspace_ref_digest` | Domain-separated SHA-256 projection; raw value omitted. |
| `target.repo.repo_ref` | `target.repo_ref_digest` | Domain-separated SHA-256 projection; raw value omitted. |
| `target.worktree.worktree_ref` | `target.worktree_ref_digest` | Domain-separated SHA-256 projection; raw value omitted. |
| `target.repo.is_bare` | `target.repository_class` | `bare_repository` or `worktree_repository`. |
| `target.repo.is_shallow` | `target.is_shallow` | Boolean retained. |
| `target.worktree.is_linked` | `target.is_linked_worktree` | Boolean retained. |
| `target.worktree.head_label` | `target.head_state_class` | Reduced to `attached`, `detached`, or `unavailable`; branch name omitted. |
| `affected_paths` | `affected_path_count` | Count retained; every path body omitted. |
| `outcome_reason` | none | Omitted because backend/Git text is not an export boundary. |

V2 also pins `redaction_profile_ref`, declares both raw path and raw ref-name
export forbidden, and enumerates the omitted field families. Invalid or
unbounded observation timestamps are projected as `unavailable`.

## Reader, writer, and downgrade behavior

- Writers must emit v2 only.
- A retained v1 JSON object may be inspected locally, but it must not be sent,
  attached, logged, or embedded in another support packet.
- If the source `DailyLoopResult` is still available, reproject it with the v2
  exporter. Do not copy the v1 `target` object into a replacement record.
- If source truth is unavailable, omit the legacy row and disclose that the
  unsafe legacy support row was excluded. Do not synthesize target digests from
  incomplete display text.
- Downgrade from v2 to v1 is forbidden. A consumer that only understands v1
  must reject the row or upgrade; it must not reconstruct raw fields.

The canonical redacted fixture is
[`support_export_redacted_v2.json`](../../../fixtures/git/m4/daily_loop_beta/support_export_redacted_v2.json).
