# M5 Skeleton-First-Restore and Session-Hydration Registries

- Packet: `m5-skeleton-first-restore-and-session-hydration-registries:stable:0001`
- Label: `M5 skeleton-first-restore and session-hydration registries with one stable restore-skeleton object rebuilt per restore, the layout skeleton rebuilt before any heavy dependency hydrates, preserved pane roles and placeholder set kept distinct from the deferred-hydration plan, canonical / accessible / audit resolution-form coverage, and the preserved-pane-role / missing-dependency-class / restore-fidelity-hint disclosure triple across shell, recovery, diagnostics, admin, workspace, session, and support surfaces`
- Consumer surfaces: 6
- Restore-fidelity classes: live_hydrated_pane, pane_role_placeholder, context_only_pane, evidence_only_pane, fidelity_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last refresh: 2026-07-14T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell rebuilds the per-restore layout skeleton to one stable object — window shell, stable pane-tree structure, preserved pane roles, placeholder set, layout-skeleton root, and the distinct deferred-hydration plan — from the shared registry before any heavy dependency hydrates, and hydrates the terminal session without rerunning it; a skeleton object missing its pane-tree structure and a hydration that collapses the pane on a missing dependency degrade honestly instead of reading as a clean pass
  - Skeleton-restore entries: 2 / session-hydration entries: 2
- **restore_coordinator**: `stable`
  - Owner: Restore-coordinator owner
  - Scope: The restore coordinator rebuilds a pane-role-preserving placeholder skeleton first and defers heavy hydration, and substitutes a placeholder for a missing debugger dependency rather than collapsing the layout; a resolution-form gap on a skeleton entry and on a hydration entry is caught before a screenshot can reintroduce a false-truth reading
  - Skeleton-restore entries: 2 / session-hydration entries: 2
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics reports the context-only skeleton and the preview hydration that discloses its restore fidelity rather than overclaiming live, without manual reconstruction; a skeleton whose heavy hydration ran before the layout skeleton was rebuilt is caught as a hydration-first restore
  - Skeleton-restore entries: 2 / session-hydration entries: 1
- **workspace_service**: `stable`
  - Owner: Workspace-service owner
  - Scope: The workspace service rebuilds the evidence-only skeleton object while keeping it bound to the registry; a skeleton that is a hand-copied per-pane restore assumption and a hydration on an unclassified surface degrade honestly
  - Skeleton-restore entries: 2 / session-hydration entries: 2
- **session_service**: `stable`
  - Owner: Session-service owner
  - Scope: The session service renders the same resolved skeleton-restore and session-hydration truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied restore table
  - Skeleton-restore entries: 2 / session-hydration entries: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved skeleton-restore and session-hydration truth, so a hand-copied constant, an unstated registry token, a hydration-first restore, or a collapsed layout is visible in evidence rather than hidden behind a screenshot, and it explains which panes restored live, as placeholders, context-only, or evidence-only
  - Skeleton-restore entries: 2 / session-hydration entries: 1
