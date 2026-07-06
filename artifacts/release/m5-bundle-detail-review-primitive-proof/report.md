# M5 Bundle Detail / Review Primitive: Bundle Detail Page and Install / Update Review Sheet

- Packet: `m5-bundle-detail-review-primitive:stable:0001`
- Label: `M5 Bundle Detail / Review Primitive: Bundle Detail Page and Install / Update Review Sheet`
- Bundle-review surfaces: 6 / 6
- Review operations: install, update, remove, drift_review
- Review postures: ready_to_apply, constrained_by_policy, read_only_comparison
- Dependency markers: entitlement_required, policy_gated, preview_capability, labs_capability, mirror_only_source, bounded_platform

## Bundle-review surfaces

- **Bundle detail page**
  - Owner: Bundle detail guild
  - Scope: Bundle detail page listing extensions, presets, tasks, docs, templates, dependency markers, mirror/offline posture, and changelog
  - Worked cases: 1
    - `review:certified-rust-service:0001` → op `install` (posture `ready_to_apply`), 2 diff row(s), source `certified`, range `>=2026.6, <2027.0`
- **Install review sheet**
  - Owner: Install-review guild
  - Scope: Install review sheet enumerating added components, side effects, entitlement/policy dependency markers, and rollback checkpoint creation
  - Worked cases: 1
    - `review:managed-web-app:0002` → op `install` (posture `ready_to_apply`), 2 diff row(s), source `managed_approved`, range `>=2026.7, <2027.0`
- **Update review sheet**
  - Owner: Update-review guild
  - Scope: Update review sheet enumerating changed components, preserving policy-blocked and adopted assets, and deriving a review posture
  - Worked cases: 2
    - `review:framework-pack-update:0003` → op `update` (posture `ready_to_apply`), 2 diff row(s), source `community_reviewed`, range `>=2026.4, <2027.0`
    - `review:policy-constrained-update:0004` → op `update` (posture `constrained_by_policy`), 2 diff row(s), source `managed_approved`, range `>=2026.7, <2027.0`
- **Drift-review sheet**
  - Owner: Drift-review guild
  - Scope: Drift-review sheet comparing local state against the bundle read-only and keeping local overrides visible
  - Worked cases: 1
    - `review:drift-review:0005` → op `drift_review` (posture `read_only_comparison`), 1 diff row(s), source `certified`, range `>=2026.6, <2027.0`
- **Migration review view**
  - Owner: Migration-review guild
  - Scope: Migration review view reconstructing an imported bundle's diffed truth and preserving imported-not-native provenance
  - Worked cases: 1
    - `review:imported-migration:0006` → op `update` (posture `ready_to_apply`), 2 diff row(s), source `imported_pending_review`, range `>=2026.2, <2026.7`
- **Support / export replay**
  - Owner: Support / export guild
  - Scope: Offline replay reconstructing review truth from a certified snapshot and an offline-cache update
  - Worked cases: 2
    - `review:support-replay:0007` → op `install` (posture `ready_to_apply`), 1 diff row(s), source `certified`, range `>=2026.6, <2027.0`
    - `review:offline-update:0008` → op `update` (posture `ready_to_apply`), 1 diff row(s), source `community_reviewed`, range `>=2026.4, <2027.0`
