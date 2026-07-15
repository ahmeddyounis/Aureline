# M5 Setting-Schema-Migration and Compatibility-Window Registries

- Packet: `m5-setting-schema-migration-and-compatibility-window-registries:stable:0001`
- Label: `M5 schema-migration and compatibility-window registries with one migration record landing per version change, migration labels that never overstate fidelity, a compare-before-apply surface materialized before any lossy or manual-review migration applies, canonical / accessible / audit resolution-form coverage, and the complete window-source / supported-version-range / deprecation-review / validation-status / review-state / docs-pointer / last-review-revision compatibility-window object across settings-resolver, shell, sync, policy, diagnostics, and support surfaces`
- Consumer surfaces: 6
- Migration fidelity labels: exact_migration, compatible_migration, lossy_migration, manual_review_migration, fidelity_class_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last refresh: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **settings_resolver**: `stable`
  - Owner: Settings-resolver owner
  - Scope: The settings resolver lands the exact upgrade migration record — old key / alias, new key, transform, compatibility window, rollback note, compare-before-apply reference, and migration provenance reference — from the shared registry and resolves the within-window compatibility label for that setting; a migration record missing its compare-before-apply reference and a compatibility window that masks a deprecated window without disclosing its window source degrade honestly instead of reading as a clean pass
  - Schema-migration entries: 2 / compatibility-window entries: 2
- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell lands the compatible import migration record while disclosing the deprecated-but-supported compatibility window and its window source; a resolution-form gap on a migration entry and on a compatibility window is caught before a screenshot can reintroduce a false-fidelity reading
  - Schema-migration entries: 2 / compatibility-window entries: 2
- **sync_service**: `stable`
  - Owner: Sync-service owner
  - Scope: The sync service lands the lossy restore migration with a materialized compare-before-apply surface and reports the outside-window compatibility label with downgrade guidance; a migration whose fidelity label overstates what the lossy transform preserves is caught before it can imply full fidelity
  - Schema-migration entries: 2 / compatibility-window entries: 1
- **policy_service**: `stable`
  - Owner: Policy-service owner
  - Scope: The policy service lands the manual-review downgrade migration with a materialized compare surface and bound to the registry while resolving the within-window compatibility label; a migration that is a hand-copied per-entry assumption and a compatibility window on an unclassified window class degrade honestly
  - Schema-migration entries: 2 / compatibility-window entries: 2
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics renders the same resolved schema-migration and compatibility-window truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied migration table
  - Schema-migration entries: 2 / compatibility-window entries: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved schema-migration and compatibility-window truth, so a hand-copied constant, an unstated registry token, an overstated fidelity label, or a masked window is visible in evidence rather than hidden behind a screenshot
  - Schema-migration entries: 2 / compatibility-window entries: 1
