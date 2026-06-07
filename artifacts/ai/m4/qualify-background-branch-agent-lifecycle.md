# Background branch-agent lifecycle qualification

- Packet: `background-branch-agent-lifecycle:stable:0001`
- Stable run ID: `branch-agent-run:stable:request-cache:0001`
- Schema: `schemas/ai/background-branch-agent-run.schema.json`
- Support export: `artifacts/ai/m4/qualify-background-branch-agent-lifecycle/support_export.json`
- Fixture: `fixtures/ai/m4/qualify-background-branch-agent-lifecycle/lifecycle_packet.json`

## Coverage

- Launch review discloses goal, base, target, tools/connectors, approvals, cost/risk, secret scope, and stop conditions.
- Active rows and checkpoints carry elapsed time, milestone, assumptions, pending approvals, and operator actions.
- Drift drills cover base advance, provider/model route drift, policy epoch change, trust narrowing, and boundary expansion.
- Re-review blocks further writes while preserving diff, logs, checkpoints, and evidence.
- Operator takeover preserves branch identity, checkpoint lineage, tool-call history, validation receipts, and pending writes.
- Completion review exposes diff, validation, evidence, compare-to-base, cleanup, and follow-up commands.
- Cleanup preview preserves review artifacts and retention truth.

## Safety

The packet proves stable branch-agent lanes cannot self-approve mutating tools,
bypass worktree isolation, collapse local-only/side-worktree/managed-remote
loci, self-merge, or directly push protected destinations.
