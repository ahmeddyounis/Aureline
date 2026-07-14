# M5 Workspace-Authority and Window-Topology Registries

- Packet: `m5-workspace-authority-and-window-topology-registries:stable:0001`
- Label: `M5 workspace-authority and window-topology registries with one stable workspace-authority object resolving per workspace, window-local selection and focus staying window-local while one authority backs multiple windows, shared dirty-buffer / save / checkpoint state kept distinct from the profile-defaults reference, canonical / accessible / audit resolution-form coverage, and the window-local pane-tree / focus-history / display-affinity disclosure triple across shell, recovery, diagnostics, admin, workspace, session, and support surfaces`
- Consumer surfaces: 6
- Authority scopes: single_window_authority_scope, multi_window_shared_authority_scope, detached_auxiliary_window_scope, scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last refresh: 2026-07-14T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell resolves the per-window workspace authority to one stable object — backing windows, stable pane-tree IDs, shared dirty-buffer / save / checkpoint state, authoritative state root, and the distinct profile-defaults reference — from the shared registry and renders the primary window topology; an authority object missing a pane-tree ID and a window topology that privately copies shared authority state degrade honestly instead of reading as a clean pass
  - Workspace-authority entries: 2 / window-topology entries: 2
- **restore_coordinator**: `stable`
  - Owner: Restore-coordinator owner
  - Scope: The restore coordinator resolves one shared workspace authority backing multiple windows while selection and focus stay window-local, and rebuilds the auxiliary window topology; a resolution-form gap on an authority entry and on a window topology is caught before a screenshot can reintroduce a false-truth reading
  - Workspace-authority entries: 2 / window-topology entries: 2
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics reports the detached / auxiliary window sharing the workspace authority and the diagnostics window topology without manual reconstruction; a window-local selection that overwrites the shared authority is caught as a window-local overwrite for its scope
  - Workspace-authority entries: 2 / window-topology entries: 1
- **workspace_service**: `stable`
  - Owner: Workspace-service owner
  - Scope: The workspace service resolves the multi-window shared authority object while keeping it bound to the registry; an authority that is a hand-copied per-window assumption and a window topology on an unclassified surface degrade honestly
  - Workspace-authority entries: 2 / window-topology entries: 2
- **session_service**: `stable`
  - Owner: Session-service owner
  - Scope: The session service renders the same resolved workspace-authority and window-topology truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied ownership table
  - Workspace-authority entries: 2 / window-topology entries: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved workspace-authority and window-topology truth, so a hand-copied constant, an unstated registry token, a window-local overwrite, or a privately-copied authority is visible in evidence rather than hidden behind a screenshot
  - Workspace-authority entries: 2 / window-topology entries: 1
