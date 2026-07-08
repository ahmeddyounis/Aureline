# M5 Review-Request, Checks, and Merge-Queue Component Matrix

- Packet: `m5-review-component-matrix:stable:0001`
- Schema: `schemas/ui/m5-review-request-check-queue-component-matrix.schema.json`
- Support export: `artifacts/release/m5-review-request-check-queue-proof/support_export.json`
- Contract doc: `docs/review/m5/freeze_the_m5_review_request_check_and_merge_queue_component_matrix.md`
- Design matrix: `artifacts/design/m5-review-request-check-queue-component-matrix.md`
- Fixtures: `fixtures/ui/m5-review-request-check-queue-components/`

## Coverage

- The review-request row is qualified Stable: provider identity, base/head relation, and freshness stay explicit; provider-authored fields are labeled provider-backed while base/head and diff summary stay local-computed.
- The checks-summary card is qualified Stable: distinct check classes are never collapsed into one status pill, provider verdicts and local re-runs stay separately labeled, and ordinary triage never forces raw-provider navigation.
- The pending-review tray is qualified Stable: owner identity and local-versus-provider origin stay explicit, and locally queued publish-later reviews are never counted as provider-confirmed.
- The merge-readiness panel is qualified Stable: blocking reasons stay explicit rather than collapsed into a single ready/not-ready pill, and the local estimate is never presented as the provider's final gate.
- The merge-queue entry is qualified Stable: provider-managed queue position and owner are never flattened into a local estimate.
- The stack-dependency chip is qualified Beta: stack relation and parent-blocked state stay visible, with local topology derived from change lineage when provider state is stale.
- The approval-invalidation banner is qualified Preview: it names why approvals were recomputed rather than showing a generic warning pill.
- Every component names its provider-versus-local-estimate distinction, stale-provider downgrade vocabulary, browser-handoff boundary, and local-continue fallback, plus required evidence packet refs, downgrade triggers, rollback posture, and consumer-surface parity.
- Proof freshness SLO is 168 hours with automatic narrowing on stale proof.

## Trust guardrails

The matrix proves that provider-managed queue state is never flattened into local
estimates, approval invalidation is never hidden behind a generic warning pill, and
ordinary check triage never forces raw-provider navigation. Browser handoff stays
explicit with a labeled return path, and local-only continuation is preserved
wherever provider freshness is degraded. Stale or underqualified rows automatically
narrow before publication rather than hiding the component.
