# M5 Required-Evidence-Check and Local-CI-Parity Registries

- Packet: `m5-required-evidence-check-and-local-ci-parity-registries:stable:0001`
- Label: `M5 required-evidence-check and local-CI-parity registries emitting one machine-readable required-evidence-check row per required check — a must-run test, scanner, docs / migration note, incident link, or rollout note — carrying its evidence-check state (required, optional, skipped, suppressed, timed out, ci-only, not evaluated here, or provider unavailable) and whether Aureline evaluated it locally, imported it, or could not evaluate it here, so the eight states never collapse into one success / failure bucket and no local parity estimate reads as provider-authoritative or queue-eligible mergeability, with canonical / accessible / audit resolution-form coverage, and a machine-readable local-CI-parity strip (local-parity-estimate-binding, provider-authoritative-binding, or capability-difference-binding) that compares the local parity estimate against the provider-authoritative state and names the capability difference — environment, secrets, runner class, service dependencies, branch protections, or provider-only merge simulation — rather than implying mergeability from one green summary across review, AI-review, provider-handoff, and support / export surfaces`
- Consumer surfaces: 6
- Evidence-check states: required, optional, skipped, suppressed, timed_out, ci_only, not_evaluated_here, provider_unavailable, evidence_state_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **review_detail**: `stable`
  - Owner: Review-detail owner
  - Scope: Review detail resolves each required check to one typed required-evidence-check row — the check identity, its evidence-check state (required, optional, skipped, suppressed, timed out, ci-only, not evaluated here, or provider unavailable), and whether Aureline evaluated it locally, imported it, or could not evaluate it here — from the shared registry and proves the local-parity-estimate compare for that check; a row that collapses an unevaluated check into a pass and a strip that would let a local parity estimate read as provider-authoritative mergeability degrade honestly instead of leaving a ci-only or not-evaluated-here check to read as a green success
  - Required-evidence-check entries: 2 / local-CI-parity entries: 2
- **ai_review_panel**: `stable`
  - Owner: AI-review owner
  - Scope: The AI review panel resolves the provider-authoritative binding and the capability-difference compare while keeping each required check's evidence-check state and evaluation origin visible; a strip widening a local estimate into provider-authoritative mergeability and a resolution-form gap on a compare are caught before a green summary can reintroduce an authoritative reading, and an AI review can never present a ci-only or not-evaluated-here check as satisfied
  - Required-evidence-check entries: 2 / local-CI-parity entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves each required check's evidence-check state and evaluation origin while keeping the capability-difference compare bound to the export, and reports the capability-difference binding; a row that is a hand-copied per-check assumption and a compare on an unclassified parity binding degrade honestly so the evidence-check state and the environment / secrets / provider-only-merge-simulation deltas are never dropped on export or reopen
  - Required-evidence-check entries: 2 / local-CI-parity entries: 1
- **review_pack_summary**: `stable`
  - Owner: Review-pack-summary owner
  - Scope: The review-pack summary resolves the evidence-check state and the parity binding — local-parity-estimate, provider-authoritative, or capability-difference — bound to the registry so a skipped, suppressed, timed-out, ci-only, not-evaluated-here, or provider-unavailable check can no longer read as a fresh, full-coverage green result; an unstated evidence-check state on a row is caught before it can drift
  - Required-evidence-check entries: 2 / local-CI-parity entries: 1
- **local_ci_parity_strip**: `stable`
  - Owner: Local-CI-parity owner
  - Scope: The local-CI parity strip renders the same resolved required-evidence-check and local-CI-parity truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied table; the ci-only / not-evaluated-here / provider-unavailable state and the capability-difference compare — environment, secrets, runner class, service dependencies, branch protections, or provider-only merge simulation — stay inspectable off-renderer so a local parity estimate never reads as provider-authoritative mergeability
  - Required-evidence-check entries: 2 / local-CI-parity entries: 1
- **provider_handoff**: `stable`
  - Owner: Provider-handoff owner
  - Scope: The provider handoff feed carries the same resolved required-evidence-check and local-CI-parity truth, so a dropped evidence-check state, an unstated evaluation origin, a local estimate masquerading as provider-authoritative, or a provider-unavailable check shown as current is visible in evidence — a local-parity-estimate binding, a provider-authoritative binding, or a capability-difference binding — rather than hidden behind a green summary
  - Required-evidence-check entries: 2 / local-CI-parity entries: 1
