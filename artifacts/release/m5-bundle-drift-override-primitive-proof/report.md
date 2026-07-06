# M5 Bundle Drift / Override Primitive: Drift Banner, Local-Override Rows, and Rollback / Remove Card

- Packet: `m5-bundle-drift-override-primitive:stable:0001`
- Label: `M5 Bundle Drift / Override Primitive: Drift Banner, Local-Override Rows, and Rollback / Remove Card`
- Bundle-drift surfaces: 6 / 6
- Drift kinds: local_only_edit, bundle_version_drift, missing_artifact, imported_gap, stale_certification, policy_entitlement_narrowing
- Granularities: field, package, task
- Significances: harmless_local_preference, support_significant

## Bundle-drift surfaces

- **Workspace drift banner**
  - Owner: Workspace drift guild
  - Scope: Workspace drift banner enumerating local-only edits, version drift, and missing artifacts with rebase / keep-local / compare choices
  - Worked cases: 1
    - `drift:rust-service:0001` → drift `diverged` (op `drift_review`), 2 override row(s), 3 kind(s), significance `support_significant`
- **Bundle detail drift panel**
  - Owner: Bundle detail guild
  - Scope: Bundle detail drift panel offering a rebase to the bundle with a one-step rollback checkpoint before mutation
  - Worked cases: 1
    - `drift:web-app-rebase:0002` → drift `bundle_ahead` (op `update`), 1 override row(s), 2 kind(s), significance `support_significant`
- **Extension drift row**
  - Owner: Extension drift guild
  - Scope: Extension drift row reporting a missing bundle artifact as support-significant drift, not a generic update
  - Worked cases: 1
    - `drift:missing-artifact:0003` → drift `local_ahead` (op `drift_review`), 0 override row(s), 1 kind(s), significance `support_significant`
- **Migration drift view**
  - Owner: Migration drift guild
  - Scope: Migration drift view reconstructing an imported bundle's imported-gap and local-only edits, preserving imported-not-native provenance
  - Worked cases: 1
    - `drift:imported-gap:0004` → drift `diverged` (op `drift_review`), 2 override row(s), 2 kind(s), significance `support_significant`
- **Diagnostics drift report**
  - Owner: Diagnostics drift guild
  - Scope: Diagnostics drift report covering a mirror-served policy narrowing and a one-step bundle removal preserving local overrides
  - Worked cases: 2
    - `drift:mirror-policy:0005` → drift `diverged` (op `drift_review`), 1 override row(s), 1 kind(s), significance `support_significant`
    - `drift:remove-rollback:0006` → drift `diverged` (op `remove`), 2 override row(s), 2 kind(s), significance `support_significant`
- **Support / export replay**
  - Owner: Support / export guild
  - Scope: Offline replay reconstructing drift truth from an offline cache with a stale-certification narrowing
  - Worked cases: 1
    - `drift:offline-replay:0007` → drift `diverged` (op `drift_review`), 1 override row(s), 2 kind(s), significance `support_significant`
