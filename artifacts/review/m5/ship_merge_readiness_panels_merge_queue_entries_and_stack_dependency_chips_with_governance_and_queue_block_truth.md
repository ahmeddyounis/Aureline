# Merge-Readiness Panels: Governance and Queue-Block Truth

- Packet: `merge-readiness-panel:stable:0001`
- Surface: `Merge-readiness panels: governance and queue-block truth`
- Panels: 5 (2 blocked)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-07T00:00:00Z)

## Panels

- **PR #5001** [`provider_managed`]: owner `provider merge queue org/repo`, governance `provider_managed`, readiness `ready_to_merge`, authority `authoritative`
  - queue `PR #5001` [#1 in queue] — ready_to_merge
  - queue `PR #4998` [#2 in queue] — queued_waiting
  - stack `no stack` [standalone] — standalone
- **PR #5002** [`aureline_local_estimate`]: owner `Aureline local queue estimate`, governance `aureline_local_estimate`, readiness `queued_waiting`, authority `estimated`
  - queue `PR #5002` [~#3 estimated] — queued_waiting
  - stack `stack feature-api` [3 of 4] — stack_child_pending
- **PR #5003** [`provider_managed`]: owner `provider merge queue org/repo (unreachable)`, governance `provider_managed`, readiness `queued_waiting`, authority `stale`
  - queue `PR #5003` [#4 (last known)] — queued_waiting
  - stack `stack feature-api` [root] — stack_root_ready
- **PR #5004** [`repo_policy_managed`]: owner `repo merge policy main`, governance `repo_policy_managed`, readiness `blocked_on_stale_base`, authority `blocked`
  - queue `PR #5004` [ejected] — blocked_on_stale_base
  - stack `stack payments` [2 of 3] — stack_parent_blocked
- **PR #5005** [`repo_policy_managed`]: owner `repo merge policy main`, governance `repo_policy_managed`, readiness `blocked_on_approval_recomputation`, authority `blocked`
  - queue `PR #5005` [held] — blocked_on_approval_recomputation
  - stack `no stack` [standalone] — standalone
