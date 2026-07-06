# M5 Bundle Rollback / Remove Primitive: Rollback / Remove Card, Created-versus-Adopted Asset Inventory, and Restore Path

- Packet: `m5-bundle-rollback-remove-primitive:stable:0001`
- Label: `M5 Bundle Rollback / Remove Primitive: Rollback / Remove Card, Created-versus-Adopted Asset Inventory, and Restore Path`
- Bundle-removal surfaces: 6 / 6
- Asset origins: bundle_created, user_created_file, user_profile, local_history, imported_setting, adopted_package
- Safe-to-remove classes: safe_to_remove, keep_local, requires_manual_handling
- Dispositions: reverted, kept_local, manual_follow_up

## Bundle-removal surfaces

- **Workspace rollback card**
  - Owner: Workspace rollback guild
  - Scope: Workspace rollback card reverting bundle-created config while keeping the user's profile and authored files local
  - Worked cases: 1
    - `removal:rust-service:0001` → op `remove`, 3 asset(s): 1 reverted / 2 kept-local / 0 manual
- **Bundle detail remove panel**
  - Owner: Bundle detail guild
  - Scope: Bundle detail remove panel reverting a bundle-owned extension and flagging an adopted package with dependents for manual follow-up
  - Worked cases: 1
    - `removal:web-app:0002` → op `remove`, 2 asset(s): 1 reverted / 0 kept-local / 1 manual
- **Extension remove row**
  - Owner: Extension lifecycle guild
  - Scope: Extension remove row reverting a bundle-owned extension while keeping the user's local history
  - Worked cases: 1
    - `removal:framework-lint:0003` → op `remove`, 2 asset(s): 1 reverted / 1 kept-local / 0 manual
- **Migration rollback view**
  - Owner: Migration rollback guild
  - Scope: Migration rollback view previewing removal of an imported bundle, keeping imported settings and flagging an adopted package for manual handling
  - Worked cases: 1
    - `removal:imported-monorepo:0004` → op `drift_review`, 2 asset(s): 0 reverted / 1 kept-local / 1 manual
- **Diagnostics removal report**
  - Owner: Diagnostics removal guild
  - Scope: Diagnostics removal report covering a mirror-served explicit removal and a read-only removal preview, both keeping checkpoint restore and export before removing anything
  - Worked cases: 2
    - `removal:managed-mirror:0005` → op `remove`, 2 asset(s): 2 reverted / 0 kept-local / 0 manual
    - `removal:diagnostics-preview:0006` → op `drift_review`, 2 asset(s): 1 reverted / 1 kept-local / 0 manual
- **Support / export replay**
  - Owner: Support / export guild
  - Scope: Offline replay reconstructing removal truth from an offline cache with a stale-certification narrowing, keeping imported settings local
  - Worked cases: 1
    - `removal:offline-replay:0007` → op `drift_review`, 2 asset(s): 1 reverted / 1 kept-local / 0 manual
