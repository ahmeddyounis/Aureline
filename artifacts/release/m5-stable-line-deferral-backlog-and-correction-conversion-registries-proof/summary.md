# M5 Stable-Line Deferral-Backlog and Correction-Conversion Registries

- Packet: `m5-stable-line-deferral-backlog-and-correction-conversion-registries:stable:0001`
- Label: `M5 stable-line deferral-backlog and correction-conversion registries with one typed deferral-backlog object recording every bounded launch-era "may slip to v1.0.x" deferral per supported line — a bounded-feature, performance-posture, migration-path, compatibility-caveat, known-limit, or documentation-gap deferral, each carrying its exact affected rows, correction status, linked claim rows, rollback target, and required narrow / defer / ship decision — support language never running ahead of a completed correction, canonical / accessible / audit resolution-form coverage, and a machine-readable correction-conversion report (a shipped correction, an explicit defer, or a claim narrowing) that narrows the affected claim automatically when a bounded correction misses its target train and names the active conversion reason across shiproom, release-center, executive-steering, program-governance, diagnostics, and support surfaces`
- Consumer surfaces: 6
- Deferral items: bounded_feature_deferral, performance_posture_deferral, migration_path_deferral, compatibility_caveat_deferral, known_limit_deferral, documentation_gap_deferral, item_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shiproom**: `stable`
  - Owner: Shiproom owner
  - Scope: The shiproom resolves the supported line's bounded-feature deferral to one typed backlog object — the deferral item, its affected rows, correction status, rollback target, and required narrow / defer / ship decision — from the shared registry and proves the shipped-correction conversion for that item; a deferral-backlog object missing its fields and a correction conversion that keeps support language ahead of a completed correction degrade honestly instead of leaving a launch-time caveat to read as silently resolved
  - Deferral-backlog entries: 2 / correction-conversion entries: 2
- **release_center**: `stable`
  - Owner: Release-center owner
  - Scope: The release center resolves the performance-posture deferral and the claim-narrowing correction-conversion report while keeping the active conversion reason visible; a line widening its claim while a bounded deferral stays open and a resolution-form gap on a correction conversion are caught before a screenshot can reintroduce a silently-resolved reading
  - Deferral-backlog entries: 2 / correction-conversion entries: 2
- **executive_steering**: `stable`
  - Owner: Executive-steering owner
  - Scope: Executive steering resolves the compatibility-caveat deferral while keeping its public claim matched to a shipped correction and reports the correction-conversion outcome; a deferral backlog that is a hand-copied per-entry assumption and a correction conversion on an unclassified conversion scope degrade honestly
  - Deferral-backlog entries: 2 / correction-conversion entries: 1
- **program_governance**: `stable`
  - Owner: Program-governance owner
  - Scope: Program governance resolves the migration-path deferral and the explicit-defer correction-conversion report bound to the registry; an unstated registry token on a deferral backlog is caught before it can drift
  - Deferral-backlog entries: 2 / correction-conversion entries: 1
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics renders the same resolved deferral-backlog and correction-conversion report truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied line table; the documentation-gap deferral and the shipped-correction conversion stay inspectable off-renderer
  - Deferral-backlog entries: 1 / correction-conversion entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved deferral-backlog and correction-conversion report truth, so a hand-copied constant, an unstated registry token, a widen-over-open-scope-debt attempt, or support language running ahead of a completed correction is visible in evidence — shipped, explicitly deferred, or narrowed — rather than hidden behind a screenshot
  - Deferral-backlog entries: 1 / correction-conversion entries: 1
