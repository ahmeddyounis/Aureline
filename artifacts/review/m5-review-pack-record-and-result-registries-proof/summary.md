# M5 Review-Pack-Record and Review-Pack-Result Registries

- Packet: `m5-review-pack-record-and-result-registries:stable:0001`
- Label: `M5 review-pack-record and review-pack-result registries emitting one machine-readable review-pack-record per repo-defined review pack — one typed field per record section: the pack version and content digest, the scope selector, the target diff identity, the worktree / base revision, and the evaluator outcome — each bound to one pack identity with its evaluator lineage, so a review result never drops its pack version / digest or template attribution and no local parity estimate reads as provider-authoritative mergeability, with canonical / accessible / audit resolution-form coverage, and a machine-readable review-pack-result (evaluated-scope-binding, pack-version-and-digest-binding, or divergence-label-binding) that turns a changed evaluated scope, pack digest, or divergence label into a visible, typed evaluator event — partial-scope, slice-omitted, stale-pack, ci-only, or provider-unavailable — rather than a green summary across review, AI-review, provider-handoff, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: changed_files_scope, pull_request_scope, base_head_range_scope, worktree_uncommitted_scope, full_tree_scope, saved_pack_snapshot_scope, pack_scope_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **review_detail**: `stable`
  - Owner: Review-detail owner
  - Scope: Review detail resolves a repo-defined review pack to one typed review-pack-record object — the pack version and content digest, the scope selector, the target diff identity, the worktree / base revision, and the evaluator outcome — from the shared registry and proves the evaluated-scope-binding result for that pack; a record missing its pack digest and a result that would let a local parity estimate read as provider-authoritative mergeability degrade honestly instead of leaving a stale pack to read as a fresh, authoritative review result
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **ai_review_panel**: `stable`
  - Owner: AI-review owner
  - Scope: The AI review panel resolves the pack-version-and-digest binding and the divergence-label result while keeping the analyzed scope and the pack version the review ran under visible; a record widening a local estimate into provider-authoritative mergeability and a resolution-form gap on a result are caught before a green summary can reintroduce an authoritative reading, and an AI review can never run under an undisclosed pack version
  - Review-pack-record entries: 2 / review-pack-result entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the evaluator result class while keeping the pack version / digest and comment / summary template attribution bound to the export, and reports the divergence-label result; a record that is a hand-copied per-entry assumption and a result on an unclassified evaluator binding degrade honestly so pack identity and evaluator lineage are never dropped on export or reopen
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **review_pack_summary**: `stable`
  - Owner: Review-pack-summary owner
  - Scope: The review-pack summary resolves the scope selector and the pack-freshness result — stale-pack, partial-scope, or slice-omitted — bound to the registry so a stale or partially evaluated pack can no longer read as a fresh, full-coverage review result; an unstated pack version / digest on a record is caught before it can drift
  - Review-pack-record entries: 2 / review-pack-result entries: 1
- **local_ci_parity_strip**: `stable`
  - Owner: Local-CI-parity owner
  - Scope: The local-CI parity strip renders the same resolved review-pack-record and review-pack-result truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied table; the ci-only / not-evaluated-here / provider-unavailable label and the evaluated-scope-binding result stay inspectable off-renderer so a local parity estimate never reads as provider-authoritative mergeability
  - Review-pack-record entries: 1 / review-pack-result entries: 1
- **provider_handoff**: `stable`
  - Owner: Provider-handoff owner
  - Scope: The provider handoff feed carries the same resolved review-pack-record and review-pack-result truth, so a dropped pack version / digest, an unstated evaluator result class, a local estimate masquerading as provider-authoritative, or a stale-pack result shown as current is visible in evidence — an evaluated-scope change, a pack-version-and-digest change, or a divergence-label change — rather than hidden behind a green summary
  - Review-pack-record entries: 1 / review-pack-result entries: 1
