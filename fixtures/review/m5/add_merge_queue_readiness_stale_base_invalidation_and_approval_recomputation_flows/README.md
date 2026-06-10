# Merge-Queue Readiness, Stale-Base Invalidation, and Approval-Recomputation Fixtures

These fixtures are valid, export-safe packets that exercise the labeling and
narrowing behavior the canonical support export keeps green. Each one keeps the
trust-review and consumer-projection invariants satisfied and proof freshness
valid — the difference is which states are narrowed and why.

## stale_base_eject_to_author.json

The base advanced and produced a conflict, so the entry is `blocked_on_stale_base`,
its stale-base invalidation action is `eject_to_author` with an explicit label, and
approvals were `reset_full` from two to zero. Demonstrates that a base advance that
invalidates an entry is labeled rather than silently kept green, and that a full
approval reset never overstates the remaining approval count.

## approval_reset_offline.json

CI status is unavailable because the queue is offline, so the entry is
`blocked_on_checks` with `unknown` base freshness, its stale-base row records
`no_action_needed` (no base advance was observed), and approval recomputation is
`not_applicable` because it could not run. Demonstrates that readiness stays
explicit and is not overstated when checks and recomputation cannot be confirmed.
