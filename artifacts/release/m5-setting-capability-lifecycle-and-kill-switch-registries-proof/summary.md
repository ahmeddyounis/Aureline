# M5 Setting-Capability-Lifecycle and Kill-Switch Registries

- Packet: `m5-setting-capability-lifecycle-and-kill-switch-registries:stable:0001`
- Label: `M5 capability-record and kill-switch-record registries with one capability record landing per capability, dependency markers that never hide behind unpublished flags, a fallback published before any protected capability is claimed, canonical / accessible / audit resolution-form coverage, and the complete disabling-source / disabled-timestamp / preserved-data-reference / explanation-reference / capability-dependency / fallback-reference / last-ledger-revision kill-switch-record object across settings-resolver, shell, sync, policy, diagnostics, and support surfaces`
- Consumer surfaces: 6
- Capability-lifecycle classes: labs, preview, beta, generally_available, graduated, deprecated, lifecycle_class_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last refresh: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **settings_resolver**: `stable`
  - Owner: Settings-resolver owner
  - Scope: The settings resolver lands the Labs capability record — owner, scope, review / expiry, enabled posture, dependency marker, fallback, and rollback note — from the shared registry and records the kill-switch disable for that capability; a capability record missing its rollback note and a kill-switch record that hides its cause without disclosing its reason degrade honestly instead of reading as a clean pass
  - Capability entries: 2 / kill-switch entries: 2
- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell lands the Preview capability record while publishing its dependency marker and fallback and records the policy-disable; a resolution-form gap on a capability entry and on a kill-switch record is caught before a screenshot can reintroduce a false clean-lifecycle reading
  - Capability entries: 2 / kill-switch entries: 2
- **sync_service**: `stable`
  - Owner: Sync-service owner
  - Scope: The sync service lands the Beta capability record with its dependency marker and fallback published and records the dependency-unavailable disable with its cause and data-preservation posture disclosed; a Beta capability that would hide its dependency by publishing no fallback is caught before a stable surface can depend on it
  - Capability entries: 2 / kill-switch entries: 1
- **policy_service**: `stable`
  - Owner: Policy-service owner
  - Scope: The policy service lands the generally-available capability record bound to the registry while recording the review-expired disable; a capability that is a hand-copied per-entry assumption and a kill-switch record on an unclassified class degrade honestly
  - Capability entries: 2 / kill-switch entries: 2
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics renders the same resolved capability and kill-switch truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied capability table
  - Capability entries: 2 / kill-switch entries: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export and claim publication carry the same resolved capability and kill-switch truth, so a hand-copied constant, an unstated registry token, a hidden Labs/Preview dependency, or a hidden kill-switch cause is visible in evidence rather than hidden behind a screenshot
  - Capability entries: 2 / kill-switch entries: 1
