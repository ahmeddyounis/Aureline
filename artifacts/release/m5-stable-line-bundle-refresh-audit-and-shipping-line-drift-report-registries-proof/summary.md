# M5 Stable-Line Bundle-Refresh-Audit and Shipping-Line-Drift-Report Registries

- Packet: `m5-stable-line-bundle-refresh-audit-and-shipping-line-drift-report-registries:stable:0001`
- Label: `M5 stable-line bundle-refresh-audit and shipping-line-drift-report registries with one typed bundle-currentness audit recording, for each claimed bundle on the active shipping line — a launch-bundle freshness audit, a launch-bundle reversibility audit, a missing-artifact audit, an imported-user-handoff-bundle audit, an org-approved-bundle audit, or an unsupported-drift audit — its exact affected rows, freshness / reversibility state, missing-artifact posture, rollback target, and required refresh / narrow decision — onboarding / migration / support language never running ahead of a current, reversible bundle, canonical / accessible / audit resolution-form coverage, and a machine-readable shipping-line-drift-report (stale-bundle, non-reversible-bundle, or unsupported-bundle drift) that narrows the affected claim automatically when a claimed bundle drifts and names the active drift reason across start-center, migration / help, release / support, admin / public-proof, shiproom, executive-steering, and program-governance surfaces`
- Consumer surfaces: 6
- Audited bundles: launch_bundle_freshness_audit, launch_bundle_reversibility_audit, missing_artifact_audit, imported_user_handoff_bundle_audit, org_approved_bundle_audit, unsupported_drift_audit, item_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shiproom**: `stable`
  - Owner: Shiproom owner
  - Scope: The shiproom resolves the active shipping line's launch-bundle freshness audit to one typed bundle-currentness object — the audited bundle, its affected rows, freshness / reversibility state, rollback target, and required refresh / narrow decision — from the shared registry and proves the stale-bundle drift report for that bundle; a bundle-refresh-audit object missing its fields and a drift report that keeps support language ahead of a refreshed bundle degrade honestly instead of leaving a stale launch bundle to read as silently supportable
  - Bundle-refresh-audit entries: 2 / shipping-line-drift-report entries: 2
- **release_center**: `stable`
  - Owner: Release-center owner
  - Scope: The release center resolves the launch-bundle reversibility audit and the unsupported-bundle drift report while keeping the active drift reason visible; a line widening its claim while a bundle is stale or non-reversible and a resolution-form gap on a drift report are caught before a screenshot can reintroduce a silently-supportable reading
  - Bundle-refresh-audit entries: 2 / shipping-line-drift-report entries: 2
- **executive_steering**: `stable`
  - Owner: Executive-steering owner
  - Scope: Executive steering resolves the imported-user-handoff-bundle audit while keeping its onboarding / migration claim matched to a current, reversible bundle and reports the shipping-line-drift-report outcome; a bundle audit that is a hand-copied per-entry assumption and a drift report on an unclassified drift scope degrade honestly
  - Bundle-refresh-audit entries: 2 / shipping-line-drift-report entries: 1
- **program_governance**: `stable`
  - Owner: Program-governance owner
  - Scope: Program governance resolves the missing-artifact audit and the non-reversible-bundle drift report bound to the registry; an unstated registry token on a bundle audit is caught before it can drift
  - Bundle-refresh-audit entries: 2 / shipping-line-drift-report entries: 1
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics renders the same resolved bundle-refresh-audit and shipping-line-drift-report truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied line table; the unsupported-drift audit and the stale-bundle drift report stay inspectable off-renderer
  - Bundle-refresh-audit entries: 1 / shipping-line-drift-report entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved bundle-refresh-audit and shipping-line-drift-report truth, so a hand-copied constant, an unstated registry token, a widen-over-stale-bundle attempt, or support language running ahead of a refreshed bundle is visible in evidence — stale, non-reversible, or unsupported drift — rather than hidden behind a screenshot
  - Bundle-refresh-audit entries: 1 / shipping-line-drift-report entries: 1
