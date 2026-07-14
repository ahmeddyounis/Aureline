# M5 Portable-Mode State-Containment and Diagnostics Registries

- Packet: `m5-portable-mode-state-containment-and-diagnostics:stable:0001`
- Label: `M5 portable-mode state-containment and diagnostics registries enforcing colocated or explicitly named sibling-state layouts, a complete durable-root inventory of settings / secrets / services / shell hooks, absent-or-blocked hidden machine-global mutation, distinguishable portable-versus-installed state origin, discoverable portable diagnostics, and documented retained-versus-replaced update continuity across About, update, diagnostics, admin, docs, and support surfaces`
- Consumer surfaces: 6
- Containments: colocated_under_executable, named_sibling_directory, hidden_machine_global, containment_unclassified
- Presentation forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell/About surface owner
  - Scope: About resolves the colocated portable layout to one stable object — executable root, colocated state roots, and the durable-root inventory of settings, secrets, services, and shell hooks — from the shared registry and inspects the portable-diagnostics card; an incomplete durable-root inventory and a diagnostics card that hides an unsupported shell-integration path degrade honestly instead of reading as a clean pass
  - Layout entries: 2 / diagnostics entries: 2
- **updater_service**: `stable`
  - Owner: Updater/update-flow owner
  - Scope: The update flow resolves the named-sibling portable layout and the portable-diagnostics update posture; a durable-state spill into a hidden machine-global path and an undocumented retained-versus-replaced continuity note are caught before a portable update can silently drop state
  - Layout entries: 2 / diagnostics entries: 2
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics reports the colocated portable layout and its discoverable diagnostics without manual reconstruction; a layout whose state origin is ambiguous — so support cannot tell portable state from installed state — is caught instead of reading as a clean pass
  - Layout entries: 2 / diagnostics entries: 1
- **admin**: `stable`
  - Owner: Admin surface owner
  - Scope: Admin resolves the colocated portable layout while preserving one registry-bound source; a hand-copied per-profile assumption and a diagnostics record on an unclassified surface degrade honestly
  - Layout entries: 2 / diagnostics entries: 2
- **docs_help**: `stable`
  - Owner: Docs/help surface owner
  - Scope: Docs and help render the same resolved portable layout and discoverable diagnostics truth the resolvers produced across the canonical, accessible, and audit presentation forms rather than a hand-copied path table
  - Layout entries: 2 / diagnostics entries: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved portable layout and diagnostics truth, so a hand-copied constant, an unstated registry token, an ambiguous state origin, or a hidden machine-global spill is visible in evidence rather than hidden behind a screenshot
  - Layout entries: 2 / diagnostics entries: 1
