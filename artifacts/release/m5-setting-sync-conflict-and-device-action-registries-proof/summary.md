# M5 Setting-Sync-Conflict and Device-Action Registries

- Packet: `m5-setting-sync-conflict-and-device-action-registries:stable:0001`
- Label: `M5 sync-conflict and device-action registries with one conflict packet landing per conflict, resolutions that never collapse into last-writer-wins, local authoritative state preserved before any protected conflict applies, canonical / accessible / audit resolution-form coverage, and the complete actor / action-timestamp / transport-state / policy-state / capability-dependency / attribution-reference / last-ledger-revision device-action-record object across settings-resolver, shell, sync, policy, diagnostics, and support surfaces`
- Consumer surfaces: 6
- Sync-conflict classes: same_key_divergent, policy_locked, missing_capability, machine_only, delete_versus_modify, stale_remote, conflict_class_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last refresh: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **settings_resolver**: `stable`
  - Owner: Settings-resolver owner
  - Scope: The settings resolver lands the same-key-divergent sync-conflict packet — field path, local and remote revisions, keep-local option, keep-synced option, compare reference, and blocked-state reason — from the shared registry and records the pause device action for that device; a conflict packet missing its blocked-state reason and a device action that hides a revoke cause without disclosing its reason degrade honestly instead of reading as a clean pass
  - Sync-conflict entries: 2 / device-action entries: 2
- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell lands the policy-locked import conflict while preserving local authoritative state and records the resume device action; a resolution-form gap on a conflict entry and on a device action is caught before a screenshot can reintroduce a false clean-sync reading
  - Sync-conflict entries: 2 / device-action entries: 2
- **sync_service**: `stable`
  - Owner: Sync-service owner
  - Scope: The sync service lands the machine-only outage conflict with local authoritative state preserved and records the revoke device action with its cause and local-authority posture disclosed; a machine-only conflict that would silently overwrite local state is caught before it can collapse into last-writer-wins
  - Sync-conflict entries: 2 / device-action entries: 1
- **policy_service**: `stable`
  - Owner: Policy-service owner
  - Scope: The policy service lands the stale-remote review conflict with local durable state authoritative and bound to the registry while recording the forget device action; a conflict that is a hand-copied per-entry assumption and a device action on an unclassified class degrade honestly
  - Sync-conflict entries: 2 / device-action entries: 2
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics renders the same resolved sync-conflict and device-action truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied conflict table
  - Sync-conflict entries: 2 / device-action entries: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved sync-conflict and device-action truth, so a hand-copied constant, an unstated registry token, a collapsed last-writer-wins resolution, or a hidden device-action ledger is visible in evidence rather than hidden behind a screenshot
  - Sync-conflict entries: 2 / device-action entries: 1
