# File-state surface cases

Fixtures in this directory exercise the shared file-state badge,
reason-strip, and write-review sheet contract.

| Case | Badge fixture | Sheet fixture | Coverage |
|---|---|---|---|
| Canonical local source with read-only authority | [`read_only_local_source_badge_group.json`](./read_only_local_source_badge_group.json) | [`read_only_local_source_write_review.json`](./read_only_local_source_write_review.json) | Current object is canonical, but write authority is blocked by filesystem permission. |
| Generated artifact | [`generated_artifact_badge_group.json`](./generated_artifact_badge_group.json) | [`generated_artifact_write_review.json`](./generated_artifact_write_review.json) | Generated object points to canonical source and generator; stale basis routes through regenerate/open-source actions. |
| Policy-locked source | [`policy_locked_source_badge_group.json`](./policy_locked_source_badge_group.json) | [`policy_locked_source_write_review.json`](./policy_locked_source_write_review.json) | Dirty state is separate from policy write authority and source-of-truth relation. |
| Managed mirror | [`managed_mirror_badge_group.json`](./managed_mirror_badge_group.json) | [`managed_mirror_write_review.json`](./managed_mirror_write_review.json) | Managed upstream remains canonical; detach/request approval/export actions disclose side effects. |
| Projected notebook result view | [`projected_notebook_result_badge_group.json`](./projected_notebook_result_badge_group.json) | [`projected_notebook_result_write_review.json`](./projected_notebook_result_write_review.json) | Projection is current for its execution basis but has no direct source write authority. |
| Captured evidence snapshot | [`captured_evidence_snapshot_badge_group.json`](./captured_evidence_snapshot_badge_group.json) | [`captured_evidence_snapshot_write_review.json`](./captured_evidence_snapshot_write_review.json) | Captured snapshot preserves evidence identity and never claims live writable source. |
