# M5 Setting-Write-Intent and Policy-Constraint Registries

- Packet: `m5-setting-write-intent-and-policy-constraint-registries:stable:0001`
- Label: `M5 setting-write-intent and policy-constraint registries with one write-intent object landing per mutation, writes landing only in the chosen scope and artifact, preview / checkpoint / rollback recovery evidence materialized before any high-risk write applies, canonical / accessible / audit resolution-form coverage, and the complete lock-source / allowed-override-classes / expiry-review / validation-status / review-state / docs-pointer / last-review-revision policy-constraint object across settings-resolver, shell, sync, policy, diagnostics, and support surfaces`
- Consumer surfaces: 6
- Write-intent preview classes: no_op_reversible, low_risk_reversible, material_behavior_change, high_risk_irreversible, destructive_reset, preview_class_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last refresh: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **settings_resolver**: `stable`
  - Owner: Settings-resolver owner
  - Scope: The settings resolver lands the no-op write intent in its chosen workspace scope and artifact — target scope, target artifact, intended value, actor, change reason, preview reference, and checkpoint / rollback recovery reference — from the shared registry and resolves the policy-locked constraint for that setting; a write-intent object missing its preview reference and a policy constraint that masks a locked value without disclosing its lock source degrade honestly instead of reading as a clean pass
  - Write-intent entries: 2 / policy-constraint entries: 2
- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell lands the low-risk write intent in its chosen user scope while disclosing the override-allowed constraint and its lock source; a resolution-form gap on a write-intent entry and on a policy constraint is caught before a screenshot can reintroduce a false-truth reading
  - Write-intent entries: 2 / policy-constraint entries: 2
- **sync_service**: `stable`
  - Owner: Sync-service owner
  - Scope: The sync service lands the material behavior change in its chosen machine scope with preview / checkpoint / rollback evidence and reports the advisory constraint with fallback guidance; a scoped write rewritten into a broader scope is caught before it can land in an unintended artifact
  - Write-intent entries: 2 / policy-constraint entries: 1
- **policy_service**: `stable`
  - Owner: Policy-service owner
  - Scope: The policy service lands the high-risk irreversible write with materialized recovery evidence and bound to the registry while resolving the policy-locked constraint; a write intent that is a hand-copied per-entry assumption and a policy constraint on an unclassified lock class degrade honestly
  - Write-intent entries: 2 / policy-constraint entries: 2
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics renders the same resolved write-intent and policy-constraint truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied write table
  - Write-intent entries: 2 / policy-constraint entries: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved write-intent and policy-constraint truth, so a hand-copied constant, an unstated registry token, a rewritten scope, or a masked lock is visible in evidence rather than hidden behind a screenshot
  - Write-intent entries: 2 / policy-constraint entries: 1
