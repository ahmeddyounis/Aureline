# M5 Display-Topology-Recovery Bounds-Recovery and Role-Continuity Registries

- Packet: `m5-display-topology-recovery-and-role-continuity-registries:stable:0001`
- Label: `M5 display-topology-recovery bounds-recovery and role-continuity registries with one stable bounds-recovery object resolved per window / dialog / sheet, the bounds resolved onto visible bounds before the surface is presented, the monitor-affinity hint and layout intent kept distinct from the keyboard-reach plan, canonical / accessible / audit resolution-form coverage, and the preserved-role-label / boundary-label / provenance-hint disclosure triple across shell, recovery, diagnostics, admin, workspace, session, and support surfaces`
- Consumer surfaces: 6
- Bounds-recovery states: affinity_monitor_restored, clamped_onto_visible_bounds, rescaled_for_dpi_change, relocated_to_primary_fallback, restored_fullscreen_to_windowed, bounds_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last refresh: 2026-07-14T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell resolves each window, dialog, or sheet to one stable bounds-recovery object — window surface, monitor-affinity hint, resolved visible bounds, layout intent, provenance class, and the distinct keyboard-reach plan — from the shared registry, returns a window to its remembered monitor via the preserved affinity hint, and fences a follow / presentation state so it is never reset into a generic window; a bounds object missing its layout intent and a fence that resets a present role degrade honestly instead of reading as a clean pass
  - Bounds-recovery entries: 2 / role-continuity-fence entries: 2
- **restore_coordinator**: `stable`
  - Owner: Restore-coordinator owner
  - Scope: The restore coordinator resolves a clamped-onto-visible-bounds recovery that records the material topology adjustment in provenance, and fences a collaboration role badge that was present before the remap so it stays visible rather than resetting to generic; a resolution-form gap on a bounds entry and on a fence entry is caught before a screenshot can hide a reduced fidelity
  - Bounds-recovery entries: 2 / role-continuity-fence entries: 2
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics reports the DPI-rescaled bounds recovery and the auxiliary-window purpose fence that discloses its reduced layout fidelity rather than hiding it, without manual reconstruction; a dialog that was presented before its bounds were resolved onto visible bounds is caught as an off-screen present
  - Bounds-recovery entries: 2 / role-continuity-fence entries: 1
- **workspace_service**: `stable`
  - Owner: Workspace-service owner
  - Scope: The workspace service resolves the relocated-to-primary-fallback recovery while keeping it bound to the registry, and fences the collaboration authority window; a bounds recovery that is a hand-copied per-surface remap assumption and a fence on an unclassified role class degrade honestly
  - Bounds-recovery entries: 2 / role-continuity-fence entries: 2
- **session_service**: `stable`
  - Owner: Session-service owner
  - Scope: The session service renders the same resolved bounds-recovery and role-continuity truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied bounds table
  - Bounds-recovery entries: 2 / role-continuity-fence entries: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved bounds-recovery and role-continuity truth, so a hand-copied constant, an unstated registry token, an off-screen present, or a role reset is visible in evidence rather than hidden behind a screenshot, and it distinguishes a full-fidelity restore from a bounds-only or reduced-fidelity remap
  - Bounds-recovery entries: 2 / role-continuity-fence entries: 1
