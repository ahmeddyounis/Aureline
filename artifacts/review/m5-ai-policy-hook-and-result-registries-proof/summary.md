# M5 AI-Policy-Hook and AI-Policy-Result Registries

- Packet: `m5-ai-policy-hook-and-result-registries:stable:0001`
- Label: `M5 AI-review-policy-hook and AI-policy-result registries binding one machine-readable policy hook per AI review run to the active review pack — the allowed analyzers, the severity thresholds, the suppression classes, and the mandatory citation requirements, each resolving through the same review-pack version / content digest and evaluator lineage as human, local, and CI review — so an AI review never applies a suppression class, severity threshold, or citation expectation from a different or stale pack revision, with canonical / accessible / audit resolution-form coverage, and a machine-readable AI-policy-result (analyzer-result-class-binding, pack-version-and-digest-binding, or rerun-staleness-binding) that surfaces whether the run is full, experimental, or policy-downgraded and marks a prior finding rerun-required or stale after a pack change rather than preserving it as current pack-compliant evidence across review, AI-review, provider-handoff, and support / export surfaces`
- Consumer surfaces: 6
- Policy-hook facets: allowed_analyzer, severity_threshold, suppression_class, policy_downgraded, experimental_analyzer, mandatory_citation, policy_hook_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **review_detail**: `stable`
  - Owner: Review-detail owner
  - Scope: Review detail resolves the active review pack to one typed AI review policy hook — the allowed analyzers, the severity thresholds, the suppression classes, and the mandatory citation requirements — bound to the same review-pack version and content digest and evaluator lineage as human, local, and CI review, and proves the analyzer-result-class binding for that run (full, experimental, or policy-downgraded); a hook that cannot name the pack version it resolved through and a result that would let an experimental or policy-downgraded run read as a full, pack-compliant review degrade honestly instead of applying a suppression class or severity threshold from a different or stale pack revision
  - AI-policy-hook entries: 2 / AI-policy-result entries: 2
- **ai_review_panel**: `stable`
  - Owner: AI-review owner
  - Scope: The AI review panel resolves the pack-version-and-digest binding and the analyzer-result-class result while keeping the active review-pack version / digest, the analyzer class, and whether the result is full, experimental, or policy-downgraded visible; a hook operating with a narrower or different capability set than the declared pack and a resolution-form gap on a result are caught before a green summary can present the run as full pack-compliant evidence, and an AI review can never run under an undisclosed or divergent pack version
  - AI-policy-hook entries: 2 / AI-policy-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the analyzer result class while keeping the review-pack version / digest and the mandatory-citation attribution bound to the export, and reports the rerun / staleness result; a hook that is a hand-copied per-entry assumption and a result on an unclassified binding degrade honestly so the governing pack version and the analyzer lineage are never dropped on export or reopen
  - AI-policy-hook entries: 2 / AI-policy-result entries: 1
- **review_pack_summary**: `stable`
  - Owner: Review-pack-summary owner
  - Scope: The review-pack summary resolves the suppression classes and severity thresholds and the rerun-staleness result — current, rerun-required-after-pack-change, or stale-after-pack-change — bound to the registry so a prior AI finding can no longer read as current pack-compliant evidence after the pack changed; an unstated pack version / digest on a hook is caught before it can drift
  - AI-policy-hook entries: 2 / AI-policy-result entries: 1
- **local_ci_parity_strip**: `stable`
  - Owner: Local-CI-parity owner
  - Scope: The local-CI parity strip renders the same resolved AI-policy-hook and AI-policy-result truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied table; the experimental-analyzer / policy-downgraded label and the analyzer-result-class binding stay inspectable off-renderer so an AI run under a narrower capability set never reads as a full, pack-authoritative review
  - AI-policy-hook entries: 1 / AI-policy-result entries: 1
- **provider_handoff**: `stable`
  - Owner: Provider-handoff owner
  - Scope: The provider handoff feed carries the same resolved AI-policy-hook and AI-policy-result truth, so a dropped review-pack version / digest, an undisclosed divergent pack version, an experimental or policy-downgraded run shown as full, or a stale-after-pack-change finding shown as current is visible in evidence — an analyzer-result-class change, a pack-version-and-digest change, or a rerun-staleness change — rather than hidden behind a green summary
  - AI-policy-hook entries: 1 / AI-policy-result entries: 1
