# Branch Agent Launch Review

- Stable run ID: `branch-agent-run:stable:request-cache:0001`
- Goal: Refactor the request cache behind a side-worktree review packet
- Base: `base:main@sha256:base-reviewed`
- Target: `change-object:side-worktree:request-cache`
- Tools/connectors: repo search, side-worktree patch writer, local validation runner
- Approval gates: launch review acceptance, mutating tool approval per checkpoint, completion review before landing
- Cost/risk band: `medium_cost_medium_risk`
- Secret scope: `read_only_redacted_handles`
- Stop conditions: secrets found, base branch advanced, cross-worktree write attempt
