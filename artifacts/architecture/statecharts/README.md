# Protected Object Statechart Pack

This directory contains the family-level lifecycle statecharts that
compose with the overview at
[`docs/architecture/lifecycle_statecharts.md`](../../../docs/architecture/lifecycle_statecharts.md)
and the boundary schema at
[`schemas/governance/lifecycle_state.schema.json`](../../../schemas/governance/lifecycle_state.schema.json).

Each file uses the same columns:

- `Terminal`, `Recoverable`, and `Retryable` make state posture explicit.
- `Recovery` names one of the shared recovery transition classes when a
  transition crosses failure, timeout, cancel, retry, rollback,
  downgrade, or stale-reconciliation boundaries.
- `Preview` and `Checkpoint` state whether the transition must show a
  review artifact or preserve a checkpoint/idempotency handle.
- `Evidence / export / audit fields` names the owning field families
  downstream UI, automation, support, and docs surfaces must preserve.

Family files:

- [`workspace_session.md`](./workspace_session.md)
- [`document_buffer.md`](./document_buffer.md)
- [`command_invocation.md`](./command_invocation.md)
- [`notification.md`](./notification.md)
- [`task_run_attempt.md`](./task_run_attempt.md)
- [`collaboration_session.md`](./collaboration_session.md)
- [`migration_import_session.md`](./migration_import_session.md)
- [`repair_transaction.md`](./repair_transaction.md)

