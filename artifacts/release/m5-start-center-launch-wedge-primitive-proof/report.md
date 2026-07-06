# M5 Start-Center Launch-Wedge Primitive: Start-Center Bundle Card and Certified-Archetype Badge Group

- Packet: `m5-start-center-launch-wedge-primitive:stable:0001`
- Label: `M5 Start-Center Launch-Wedge Primitive: Start-Center Bundle Card and Certified-Archetype Badge Group`
- Launch-wedge surfaces: 6 / 6
- Source classes: certified, managed_approved, community_reviewed, imported_pending_review, local_draft
- Entry-assurance tiers: certified, approximate, local_only
- Badge downgrade states: none, limited, retest_pending

## Launch-wedge surfaces

- **Start-center bundle card**
  - Owner: Start-center guild
  - Scope: Start-center bundle card naming name, persona tag, support class, certification, range, signer, and a Review action
  - Worked cases: 2
    - `wedge:certified-rust-service:0001` → source `certified` (tier `certified`), badge `none`, range `>=2026.6, <2027.0`
    - `wedge:mirrored-certified:0006` → source `certified` (tier `certified`), badge `limited`, range `>=2026.6, <2027.0`
- **Workspace switcher**
  - Owner: Workspace-switcher guild
  - Scope: Workspace switcher row naming the active stack's source class and support class
  - Worked cases: 1
    - `wedge:managed-web-app:0002` → source `managed_approved` (tier `certified`), badge `none`, range `>=2026.7, <2027.0`
- **Bundle-picker list**
  - Owner: Bundle-picker guild
  - Scope: Bundle-picker list keeping certified, approximate, and local-only entries visibly distinct
  - Worked cases: 2
    - `wedge:community-data-pipeline:0003` → source `community_reviewed` (tier `approximate`), badge `limited`, range `>=2026.4, <2027.0`
    - `wedge:offline-community:0008` → source `community_reviewed` (tier `approximate`), badge `retest_pending`, range `>=2026.4, <2027.0`
- **Docs / help bundle entry**
  - Owner: Docs / help guild
  - Scope: Docs / help bundle entry preserving imported-not-native provenance and the retest-pending state
  - Worked cases: 1
    - `wedge:imported-monorepo:0004` → source `imported_pending_review` (tier `approximate`), badge `retest_pending`, range `>=2026.2, <2026.7`
- **Diagnostics bundle view**
  - Owner: Diagnostics guild
  - Scope: Diagnostics bundle view naming a local-only draft without inheriting an official tier
  - Worked cases: 1
    - `wedge:local-draft-cli:0005` → source `local_draft` (tier `local_only`), badge `retest_pending`, range `>=2026.7, <2027.0`
- **Support / export replay**
  - Owner: Support / export guild
  - Scope: Offline replay reconstructing launch-wedge truth from an imported certified snapshot
  - Worked cases: 1
    - `wedge:support-replay:0007` → source `certified` (tier `certified`), badge `none`, range `>=2026.6, <2027.0`
