# M5 Review-Pack Invalidation-Record and Rerun-Compare Registries

- Packet: `m5-review-pack-invalidation-and-rerun-compare-registries:stable:0001`
- Label: `M5 review-pack invalidation-record and rerun-compare registries emitting one machine-readable review-pack-invalidation-record per stale transition — one typed record naming the exact invalidation cause: base-revision drift, head-revision drift, worktree-scope drift, review-pack version drift, review-pack content-digest drift, or environment-capsule drift — each bound to one pack identity with its evaluator lineage, base / head, worktree scope, and pack version / digest, so a review result never drops its pack version / digest or template attribution and no stale local parity estimate reads as fresh provider-authoritative mergeability, with canonical / accessible / audit resolution-form coverage, and a machine-readable review-pack-rerun-compare (previous-packet-binding, current-packet-binding, or preserved-draft-evidence-binding) that lets a rerun-review and compare action inspect what changed between the previous evaluator packet and the current base/head or pack revision while preserving draft-only notes and local evidence marked stale — surfacing partial-scope, slice-omitted, stale-pack, ci-only, or provider-unavailable rather than a green summary — so no review surface keeps queue eligibility, approval validity, or AI policy compliance green after a material pack / base / environment drift across review, AI-review, provider-handoff, and support / export surfaces`
- Consumer surfaces: 6
- Scope selectors: base_revision_drift, head_revision_drift, worktree_scope_drift, pack_version_drift, pack_digest_drift, environment_capsule_drift, invalidation_cause_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **review_detail**: `stable`
  - Owner: Review-detail owner
  - Scope: Review detail resolves a base-revision drift to one typed review-pack-invalidation-record object — the invalidation cause, the pack version and content digest, the drifted base / head / worktree scope, the target diff identity, and the evaluator outcome — from the shared registry and proves the previous-packet rerun/compare binding for that pack; a record that cannot name its invalidation cause and a compare that would let a stale local parity estimate read as fresh provider-authoritative mergeability degrade honestly instead of leaving a stale pack to read as a fresh, authoritative review result
  - Review-pack-record entries: 2 / review-pack-rerun-compare entries: 2
- **ai_review_panel**: `stable`
  - Owner: AI-review owner
  - Scope: The AI review panel resolves the head-revision drift and its current-packet rerun/compare binding while keeping the analyzed scope and the pack version the review ran under visible; a record widening a stale local estimate into fresh provider-authoritative mergeability and a resolution-form gap on a compare are caught before a green summary can reintroduce an authoritative reading, so AI policy compliance never stays green under an undisclosed or drifted pack version
  - Review-pack-record entries: 2 / review-pack-rerun-compare entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the pack-version drift while keeping the pack version / digest and comment / summary template attribution bound to the export, and reports the preserved-draft-evidence rerun/compare binding so draft-only notes and local evidence carry forward marked stale rather than discarded; a record that is a hand-copied per-entry assumption and a compare on an unclassified rerun/compare binding degrade honestly so pack identity, the invalidation cause, and evaluator lineage are never dropped on export or reopen
  - Review-pack-record entries: 2 / review-pack-rerun-compare entries: 1
- **review_pack_summary**: `stable`
  - Owner: Review-pack-summary owner
  - Scope: The review-pack summary resolves the worktree-scope drift and the stale transition it forces — stale-pack, partial-scope, or slice-omitted — bound to the registry so a stale or partially evaluated pack can no longer read as a fresh, full-coverage review result; a record that cannot name its invalidation cause is caught before it can silently drift
  - Review-pack-record entries: 2 / review-pack-rerun-compare entries: 1
- **local_ci_parity_strip**: `stable`
  - Owner: Local-CI-parity owner
  - Scope: The local-CI parity strip renders the same resolved review-pack-invalidation-record (environment-capsule drift) and review-pack-rerun-compare truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied table; the ci-only / not-evaluated-here / provider-unavailable label and the previous-versus-current compare binding stay inspectable off-renderer so a stale local parity estimate never reads as fresh provider-authoritative mergeability
  - Review-pack-record entries: 1 / review-pack-rerun-compare entries: 1
- **provider_handoff**: `stable`
  - Owner: Provider-handoff owner
  - Scope: The provider handoff feed carries the same resolved review-pack-invalidation-record (pack-digest drift) and review-pack-rerun-compare truth, so a dropped pack version / digest, an unnamed invalidation cause, a stale estimate masquerading as fresh provider-authoritative, or a stale-pack compare shown as current is visible in evidence — a base/head change, a pack-version-and-digest change, or an environment-capsule change — rather than hidden behind a green summary that keeps queue eligibility or approval validity green after a material drift
  - Review-pack-record entries: 1 / review-pack-rerun-compare entries: 1
