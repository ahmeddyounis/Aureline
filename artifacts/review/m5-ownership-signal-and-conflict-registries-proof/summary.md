# M5 Ownership-Signal-Row and Owner-Conflict Registries

- Packet: `m5-ownership-signal-and-conflict-registries:stable:0001`
- Label: `M5 ownership-signal-row and owner-conflict registries emitting one machine-readable ownership-signal row per owned slice — the owner source class (a CODEOWNERS repo rule, a graph-overlay maintainer, or a provider-suggested reviewer) and the advisory-versus-enforced owner authority, never flattened into one ambiguous owner pill — each bound to its pack association with its reviewer rationale, so an exported review / support packet never drops the owner source class or rationale and no advisory owner is silently promoted into an enforced merge gate, with canonical / accessible / audit resolution-form coverage, and a machine-readable owner-conflict reconciliation (owner-authority-binding, owner-source-provenance-binding, or owner-conflict-rationale-binding) that turns a disagreement between a repo rule, a graph-derived maintainer, and a provider suggestion into a visible, explained event with an explicit winning-versus-advisory relationship rather than a silent last-writer-wins collapse across review lists, review detail, merge-readiness, AI-review, browser handoff, and support / export surfaces`
- Consumer surfaces: 6
- Owner source classes: codeowners_rule_owner, graph_overlay_maintainer, provider_suggested_reviewer, enforced_review_gate_owner, advisory_area_owner, fallback_default_owner, ownership_source_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **review_detail**: `stable`
  - Owner: Review-detail owner
  - Scope: Review detail resolves each owned slice to one typed ownership-signal row from the shared registry — the owner source class (a CODEOWNERS repo rule, a graph-overlay maintainer, or a provider-suggested reviewer) and the advisory-versus-enforced owner authority, never flattened into one ambiguous owner pill — and proves the owner-authority-binding reconciliation for that slice; an ownership row missing its owner provenance and a reconciliation that would promote an advisory owner into an enforced merge gate degrade honestly instead of reading as an authoritative owner signal
  - Ownership-signal-row entries: 2 / owner-conflict entries: 2
- **ai_review_panel**: `stable`
  - Owner: AI-review owner
  - Scope: The AI review panel resolves the owner-source-provenance binding and the owner-conflict rationale while keeping which owner came from a repo rule, a graph overlay, or provider metadata visible; an ownership row flattening advisory and enforced owners and a resolution-form gap on a reconciliation are caught before a green summary can hide the disagreement, and AI review never runs under an undisclosed owner set
  - Ownership-signal-row entries: 2 / owner-conflict entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the enforced-review-gate owner while keeping the owner source class and reviewer rationale bound to the export, and reports the owner-conflict reconciliation; an ownership row that is a hand-copied per-entry assumption and a reconciliation on an unclassified binding degrade honestly so owner provenance and rationale are never dropped on export or reopen
  - Ownership-signal-row entries: 2 / owner-conflict entries: 1
- **review_pack_summary**: `stable`
  - Owner: Review-pack-summary owner
  - Scope: The review-pack summary resolves the provider-suggested reviewer and the owner-source-provenance reconciliation — repo rule versus graph overlay versus provider metadata — bound to the registry so a provider suggestion can no longer silently overwrite a CODEOWNERS repo rule; an unstated owner provenance on a row is caught before it can drift
  - Ownership-signal-row entries: 2 / owner-conflict entries: 1
- **local_ci_parity_strip**: `stable`
  - Owner: Local-CI-parity owner
  - Scope: The local-CI parity strip renders the same resolved ownership-signal-row and owner-conflict truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied table; the advisory-versus-enforced owner authority and the owner-authority-binding reconciliation stay inspectable off-renderer so an advisory owner never reads as an enforced merge gate
  - Ownership-signal-row entries: 1 / owner-conflict entries: 1
- **provider_handoff**: `stable`
  - Owner: Provider-handoff owner
  - Scope: The provider handoff feed carries the same resolved ownership-signal-row and owner-conflict truth, so the conflicting-owner set — a CODEOWNERS repo rule, a graph-overlay maintainer, and a provider-suggested reviewer disagreeing at once — stays visible with an explicit winning-versus-advisory relationship carried by the owner-conflict-rationale reconciliation rather than collapsed into one owner pill or hidden behind a green summary
  - Ownership-signal-row entries: 1 / owner-conflict entries: 1
