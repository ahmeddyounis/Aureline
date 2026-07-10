# DRI-registry rows and merge-readiness strips

- Packet: `m5-dri-registry-merge-readiness-controls:stable:0001`
- Surface: `M5 DRI-registry rows and merge-readiness strips: primary/backup role aliases, support-or-escalation path, queue-or-branch target truth, blocker counts, export-packet actions, and no-silent-mergeability widening across claimed governed review, release, and shiproom surfaces`
- DRI-registry rows: 6 (1 with an advisory owner)
- Merge-readiness strips: 6 (5 with an outstanding blocker)
- Proof freshness SLO: 168 hours (last refresh: 2026-07-10T00:00:00Z)

## DRI-registry rows

- **service/api-gateway** — owner `codeowners_authoritative`, escalation `continuous_to_owner`, freshness `currently_verified`, parity `provider_authoritative`
- **service/billing-core** — owner `codeowners_authoritative`, escalation `continuous_to_owner`, freshness `refresh_due`, parity `provider_authoritative`
- **service/search-index** — owner `registry_declared`, escalation `degraded_fallback`, freshness `stale_superseded`, parity `ci_only`
- **path/services/notifications** — owner `registry_declared`, escalation `not_configured`, freshness `never_verified`, parity `not_evaluated_here`
- **path/legacy/import** — owner `advisory_heuristic`, escalation `broken_no_fallback`, freshness `unknown_freshness`, parity `local_estimate`
- **path/experimental/graph** — owner `unresolved`, escalation `continuous_to_owner`, freshness `currently_verified`, parity `stale_relative_to_head`

## Merge-readiness strips

- **Merge blocked: two required checks are failing** — target `merge_queue`, blockers 2, next `resolve_blockers`, parity `provider_authoritative`
- **Merge estimate: local review is clean but unconfirmed** — target `target_branch`, blockers 1, next `request_provider_evaluation`, parity `local_estimate`
- **Merge gate stale: evaluated against an older base** — target `stacked_branch`, blockers 1, next `refresh_stale_base`, parity `stale_relative_to_head`
- **Merge pending: CI reports pass but the provider gate is unconfirmed** — target `protected_branch`, blockers 1, next `await_queue_position`, parity `ci_only`
- **Merge gate not evaluated on this build** — target `no_target`, blockers 1, next `escalate_to_owner`, parity `not_evaluated_here`
- **Merge allowed: the provider confirms the gate is clear** — target `merge_queue`, blockers 0, next `ready_to_merge`, parity `provider_authoritative`
