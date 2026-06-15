# M5 storage-pressure banner (human-readable)

Companion to the boundary schema at
[`/schemas/storage/m5_storage_pressure.schema.json`](../../schemas/storage/m5_storage_pressure.schema.json),
the contract at
[`/docs/storage/m5_storage_pressure_contract.md`](../../docs/storage/m5_storage_pressure_contract.md),
and the scenario corpus under
[`/fixtures/storage/m5_storage_pressure_cases/`](../../fixtures/storage/m5_storage_pressure_cases/).

A storage-pressure banner is the operator-facing object the shell shows when
low-disk or managed-quota pressure narrows a surface. It replaces a silent trim
with an honest disclosure. Every storage-class and low-disk-ladder value
re-exports verbatim from
[`/artifacts/runtime/storage_classes.yaml`](../runtime/storage_classes.yaml); the
eviction order follows the frozen sequence drilled in
[`/artifacts/runtime/low_disk_drills.yaml`](../runtime/low_disk_drills.yaml). The
pressure-class, pressure-source, paused-work, eviction-step disposition,
state-loss-guard, and escalation columns are the only sets this banner
introduces.

## What a banner states

- **Pressure class** — `constrained`, `degraded`, or `protect_core`, the runtime
  governor tier that maps to the low-disk floor reached.
- **Pressure source** — `low_disk_floor`, `managed_tenant_quota`,
  `per_workspace_quota`, or `per_class_ceiling`.
- **Paused work** — the background lanes deferred before any deletion.
- **Next eviction order** — the full frozen ladder, early → late, with each
  step's disposition and whether it was applied at this pressure.
- **Protected classes** — the classes left untrimmed.
- **Open inspector / open review** — the actions that move the user from the
  banner into the storage inspector and the class-selective clear-data review.

## The frozen eviction order

| # | Ladder step | Target class | Disposition |
| --- | --- | --- | --- |
| 1 | stop_speculative_fetch_and_prefetch | — | pause background work, delete nothing |
| 2 | pause_managed_replication_and_pack_refresh | — | pause background work, delete nothing |
| 3 | trim_interactive_hot_cache | interactive_hot_cache | trim disposable cache |
| 4 | trim_knowledge_cache_rebuildable | knowledge_cache | trim rebuildable cache (rebuild-pending) |
| 5 | trim_artifact_cache_unpinned | artifact_cache | trim only unpinned entries |
| 6 | trim_prebuild_environment_unpinned | prebuild_environment_cache | trim only unpinned entries |
| 7 | expire_unpinned_evidence_past_retention | evidence_support_cache | expire only unpinned evidence past retention |
| 8 | user_owned_recovery_state_only_under_explicit_review | user_owned_recovery_state | **protected — never auto; reviewed escalation only** |

A pressure tier auto-applies a contiguous prefix of the ladder: `constrained`
stops at step 3, `degraded` at step 6, `protect_core` at step 7. **Step 8 is
never applied automatically at any tier.**

## Pressure tiers and what they shed first

| Pressure class | Auto-applies through | What is shed | Protected, not trimmed |
| --- | --- | --- | --- |
| constrained | trim_interactive_hot_cache | speculative fetch paused; disposable hot cache trimmed | evidence + user-owned recovery |
| degraded | trim_prebuild_environment_unpinned | rebuildable knowledge cache + unpinned artifact / prebuild caches | evidence + user-owned recovery |
| protect_core | expire_unpinned_evidence_past_retention | unpinned evidence past retention (pinned and in-window evidence retained) | user-owned recovery |

## Scenarios covered

`low_disk_constrained_pauses_then_trims_disposable`,
`low_disk_degraded_trims_rebuildable_unpinned`,
`low_disk_protect_core_expires_unpinned_evidence_only`,
`managed_quota_ceiling_narrows_surface`, and
`quota_pressure_refuses_user_owned_state`.

## Guardrails enforced by the banner

- The eviction order is the full frozen sequence; no step is skipped, reordered,
  or hidden in logs.
- User-owned recovery state is never auto-trimmed under any pressure tier; its
  guard always reclaims zero bytes.
- Evidence retains pinned and in-window entries; only unpinned evidence past
  retention may expire, and only at `protect_core`.
- Managed quota or disk pressure never silently deletes local user-owned state.
  When only protected state remains over the ceiling, the banner asks for a
  reviewed class-specific decision instead of deleting anything.
- No banner ever reports `authoritative_state_loss`.
