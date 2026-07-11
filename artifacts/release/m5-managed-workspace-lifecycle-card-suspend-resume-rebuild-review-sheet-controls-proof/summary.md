# M5 Managed-Workspace-Lifecycle-Card and Suspend-Resume-Rebuild-Review-Sheet Controls

- Packet: `m5-managed-workspace-lifecycle-card-suspend-resume-rebuild-review-sheet-controls:stable:0001`
- Label: `M5 managed-workspace-lifecycle-card and suspend-resume-rebuild-review-sheet controls with lifecycle state, persistence class, continuity class, expiry timing, template/image provenance, changed persistence, preserved-vs-lost state, reattach/rerun consequences, and local-safe continuation truth`
- Consumer surfaces: 5
- Lifecycle states: provision, warm, ready, suspended, resumed, reconnecting, rebuild_required, recreate_required, expired, local_safe_continuation
- Proof freshness SLO: 168 hours (last refresh: 2026-07-11T00:00:00Z)

## Consumer surfaces

- **run_test_debug_ui**: `stable`
  - Owner: Run/test/debug surface owner
  - Scope: Every run, test, and debug target renders a managed-workspace lifecycle card naming its lifecycle state, persistence class, continuity class, and expiry timing before the user trusts a target; the suspend/resume/rebuild review sheet names its action class, template/image provenance, changed persistence, preserved-vs-lost state, and reattach/rerun consequences before commit
  - Card examples: 4 / sheet examples: 2
- **preview_ui**: `stable`
  - Owner: Preview surface owner
  - Scope: Preview targets reuse the same lifecycle card and review-sheet vocabulary, distinguishing suspended and resumed states and degrading honestly when the persistence class or expiry timing is unstated; the rebuild review sheet names its successor-image provenance and changed persistence before commit
  - Card examples: 4 / sheet examples: 2
- **companion_ui**: `stable`
  - Owner: Companion surface owner
  - Scope: Companion handoff reuses the same lifecycle cards and review language so a reconnecting or rebuild-required workspace is distinguishable before the user acts, and the review sheet degrades rather than present a materially changed runtime as exact continuity
  - Card examples: 3 / sheet examples: 2
- **incident_ui**: `stable`
  - Owner: Incident/ops surface owner
  - Scope: Incident and ops surfaces keep the same lifecycle language, distinguishing recreate-required and expired states and degrading honestly when an outage state hides local-safe continuation; the review sheet degrades rather than appear after a destructive action
  - Card examples: 3 / sheet examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved lifecycle card and review-sheet truth, so a hidden persistence change, an unstated preserved-vs-lost state, or an undisclosed consequence is visible in evidence rather than hidden behind feature-local prose, and local-safe continuation stays legible
  - Card examples: 1 / sheet examples: 3
