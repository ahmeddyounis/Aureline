# M5 Install-Topology and State-Root Registries

- Packet: `m5-install-topology-and-state-root-registries:stable:0001`
- Label: `M5 install-topology and state-root registries with one stable install-topology object resolving per delivery profile, explicit shared-versus-isolated state namespaces across managed / user / side-by-side scopes, full-graph rollback and disclosed spill, canonical / accessible / audit resolution-form coverage, and the writable-state-root / policy-root / rollback-target disclosure triple across About, update, diagnostics, admin, docs, and support surfaces`
- Consumer surfaces: 6
- Delivery scopes: per_user_managed_scope, per_machine_managed_scope, side_by_side_channel_scope, scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell/About surface owner
  - Scope: About resolves the per-user managed install to one stable topology object — install mode, channel, updater owner, binary root, state roots, policy roots, rollback target — from the shared registry and inspects the portable state-root boundary; a hand-copied per-profile assumption and a portable boundary that hides a machine-global spill degrade honestly instead of reading as a clean pass
  - Install-topology entries: 2 / state-root entries: 2
- **updater_service**: `stable`
  - Owner: Updater/update-flow owner
  - Scope: The update flow resolves the per-machine managed install object and the offline / air-gap state-root boundary; a resolution-form gap on an install entry and on a state-root boundary is caught before a screenshot can reintroduce a false-truth reading
  - Install-topology entries: 2 / state-root entries: 2
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics reports the side-by-side stable-plus-preview install topology and the portable state-root boundary without manual reconstruction; a preview channel that reuses the stable state namespace without an explicit handoff is caught as unisolated for its scope
  - Install-topology entries: 2 / state-root entries: 1
- **admin**: `stable`
  - Owner: Admin surface owner
  - Scope: Admin resolves the per-machine managed install object while preserving updater ownership and admin control; a topology that hides updater ownership in a managed flow and a state-root boundary on an unclassified surface degrade honestly
  - Install-topology entries: 2 / state-root entries: 2
- **docs_help**: `stable`
  - Owner: Docs/help surface owner
  - Scope: Docs and help render the same resolved install-topology and state-root boundary truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied path table
  - Install-topology entries: 2 / state-root entries: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved install-topology and state-root boundary truth, so a hand-copied constant, an unstated registry token, a reused state namespace, or a hidden machine-global spill is visible in evidence rather than hidden behind a screenshot
  - Install-topology entries: 2 / state-root entries: 1
