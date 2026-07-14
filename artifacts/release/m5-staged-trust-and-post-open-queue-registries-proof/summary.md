# M5 Staged-Trust and Post-Open Bootstrap-Queue Registries

- Packet: `m5-staged-trust-and-post-open-queue-registries:stable:0001`
- Label: `M5 staged-trust and post-open bootstrap-queue registries with one stable staged-trust object resolving per acquisition path, the staged trust staying browse-safe with no repo-owned action running implicitly and an explicit approval recorded before any trust-widening stage, canonical / accessible / audit resolution-form coverage, and the complete queue-item-kind / execution-site / trust-consequence / network-consequence / approval-requirement / attribution post-open-queue object across acquisition-engine, git, trust, diagnostics, CLI, and support surfaces`
- Consumer surfaces: 6
- Trust stages: browse_tree_and_manifests, compute_safe_metadata, review_deferred_repo_actions, run_repo_owned_action_after_approval, hydrate_network_content_after_approval, stage_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last refresh: 2026-07-14T00:00:00Z)

## Consumer surfaces

- **acquisition_engine**: `stable`
  - Owner: Acquisition-engine owner
  - Scope: The acquisition engine resolves the browse-tree-and-manifests trust stage to one stable object — browse scope, computed metadata, deferred repo-owned action set, trust-prompt policy, explicit-approval reference, and staged-trust provenance — from the shared registry and derives the runs-repo-owned-code post-open queue item gated behind an explicit approval; a staged-trust object missing its deferred action set and a queue item that would auto-execute a hook merely because a path was cloned degrade honestly instead of reading as a clean pass
  - Staged-trust entries: 2 / post-open-queue entries: 2
- **git_service**: `stable`
  - Owner: Git-service owner
  - Scope: The git service resolves the compute-safe-metadata trust stage while keeping the tree browse-safe before any hydration, and renders the hydrates-network-backed-content post-open queue item gated behind an explicit approval with a disclosed follow-up; a resolution-form gap on a staging entry and on a queue item is caught before a screenshot can reintroduce a false-truth reading
  - Staged-trust entries: 2 / post-open-queue entries: 2
- **trust_service**: `stable`
  - Owner: Trust-service owner
  - Scope: The trust service reports the review-deferred-repo-actions trust stage and the mutates-reviewed-checkout post-open queue item without manual reconstruction; a run-repo-owned-action stage that would widen trust before an explicit approval is recorded is caught as an early trust widening
  - Staged-trust entries: 2 / post-open-queue entries: 1
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics resolves the run-repo-owned-action-after-approval trust stage while keeping it browse-safe and bound to the registry, and renders the inert-recommendation post-open queue item; a staging entry that is a hand-copied per-entry assumption and a queue item on an unclassified class degrade honestly
  - Staged-trust entries: 2 / post-open-queue entries: 2
- **cli_export**: `stable`
  - Owner: CLI-export owner
  - Scope: The CLI export renders the same resolved staged-trust and post-open-queue truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied queue table
  - Staged-trust entries: 2 / post-open-queue entries: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved staged-trust and post-open-queue truth without embedding raw secrets, so a hand-copied constant, an unstated registry token, an implicitly-executing queue item, or a trust widened before browse-safe metadata is computed is visible in evidence rather than hidden behind a screenshot
  - Staged-trust entries: 2 / post-open-queue entries: 1
