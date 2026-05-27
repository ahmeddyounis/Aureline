# Diff-first rewrite-flow fixtures

These fixtures exercise the diff-first review and recovery-checkpoint
packet for merge, rebase, cherry-pick, revert, and reset flows. Each
fixture seeds the workspace from the canonical alpha review seed
fixture, then attaches a rewrite flow, diff-first review record,
optional sequence-edit proposal, recovery checkpoint summary,
command-graph operations, and a metadata-safe support/export packet.

The cases prove that review surfaces can show `diff pending`, `diff
approved`, `checkpoint ready`, `protected branch blocked`, `approval
invalidated`, `checks stale`, `paused conflict`, and `sequence running`
as separable, inspectable truths rather than one collapsed status.

| Fixture | Coverage |
| --- | --- |
| `merge_diff_approved_checkpoint_ready.json` | Merge flow diff-approved with captured-ready checkpoint; apply and abort commands are actionable. |
| `rebase_paused_conflict_checkpoint_captured.json` | Rebase paused for conflict resolution with pre-rebase checkpoint captured; continue-after-resolve and abort commands are surfaced, flow is not actionable until conflict is resolved. |
| `cherry_pick_sequence_protected_branch_blocked.json` | Cherry-pick sequence with two ordered operations blocked by protected-branch policy; preview and checkpoint commands are blocked, external-handoff command is offered. |
| `revert_reflog_only_acknowledged.json` | Revert flow with reflog-only recovery disclosure acknowledged; no explicit checkpoint required, diff review still pending. |
| `reset_hard_checkpoint_captured.json` | Hard reset flow with suspicious content flagged and reviewed, checkpoint required and captured; restore-checkpoint command is surfaced alongside apply and abort. |
| `interactive_rebase_sequence_running.json` | Interactive rebase with four-step sequence-edit proposal (two completed, two remaining); sequence-editor and continue commands are surfaced, flow is executing and not yet actionable. |
