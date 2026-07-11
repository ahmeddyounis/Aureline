# M5 Workspace-Expiry-Banner and Local-Safe-Continuation-Card Controls

- Packet: `m5-workspace-expiry-banner-local-safe-continuation-card-controls:stable:0001`
- Label: `M5 workspace-expiry-banner and local-safe-continuation-card controls with expiry timing, triggering owner/source, affected capabilities, export-before-loss and renew/reopen actions, preserved files/context, lost live state, next safe actions, and no-exact-continuity-overclaim truth`
- Consumer surfaces: 5
- Expiry windows: none, idle_window, hibernation_window, hard_deadline, control_plane_outage
- Proof freshness SLO: 168 hours (last refresh: 2026-07-11T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell renders a workspace-expiry banner naming its exact expiry timing, triggering owner/source, affected capabilities, and export-before-loss or renew/reopen actions, so an idle-window expiry never reads as a generic disconnect; the local-safe continuation card names what remains local-safe and what live state is lost
  - Banner examples: 2 / card examples: 2
- **preview_ui**: `stable`
  - Owner: Preview surface owner
  - Scope: Preview targets reuse the same expiry-banner and local-safe continuation vocabulary, distinguishing hibernation-window expiry and degrading honestly when the triggering source or affected capabilities are unstated; the continuation card names its lost previews and background jobs
  - Banner examples: 3 / card examples: 2
- **companion_ui**: `stable`
  - Owner: Companion surface owner
  - Scope: Companion handoff reuses the same expiry banner and local-safe continuation cards so a hard-deadline expiry is distinguishable before the user loses context, and both components degrade rather than present a gone runtime as exact continuity
  - Banner examples: 2 / card examples: 2
- **incident_ui**: `stable`
  - Owner: Incident/ops surface owner
  - Scope: Incident and ops surfaces keep the same expiry and fallback language, distinguishing a control-plane outage and degrading honestly when a banner offers no export-before-loss route or a continuation card hides local-safe continuation
  - Banner examples: 2 / card examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved expiry-banner and local-safe continuation truth, so an unstated timing, an undisclosed preserved-vs-lost state, or a hidden local-safe continuation is visible in evidence rather than hidden behind feature-local prose
  - Banner examples: 1 / card examples: 2
