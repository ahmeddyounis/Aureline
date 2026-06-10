# Durable Review Headers, Local-CI Parity, and Anchor Rehydration

- Packet: `durable-review-header:stable:0001`
- Schema: `schemas/review/implement-durable-review-workspace-headers-local-ci-parity-and-stable-anchor-rehydration.schema.json`
- Support export: `artifacts/review/m5/implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration/support_export.json`
- Contract doc: `docs/review/m5/implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration.md`
- Fixtures: `fixtures/review/m5/implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration/`
- Producer: `aureline_review::current_durable_review_header_export`

## Coverage

- **Durable review-workspace headers** carry the target identity (what is under
  review), the durable anchor id, the base-freshness class, the approval state,
  and the mutation authority. The header surface always shows what is being
  reviewed and how fresh it is; stale-base and outdated-diff states are labeled
  rather than hidden, and approval resets when the base changes.
- **Local-CI parity** records, per check lane, how the local result lines up with
  the CI expectation it mirrors. Verdicts are explicit (`parity_match`,
  `local_only_advisory`, `divergence_labeled`, `unavailable_offline`) and every
  labeled divergence carries a non-empty divergence label. At least one required
  check lane anchors the parity claim.
- **Stable anchor rehydration** records the outcome of re-attaching a durable
  anchor after an edit, rebase, reopen, base change, or external sync. Drifted or
  orphaned anchors are relabeled with a non-empty drift label rather than being
  silently dropped, and every header's durable anchor has a rehydration record.

## Trust guardrails

The `trust_review` block encodes the hard invariants — all must hold for the
packet to validate: anchors rehydrate durably; anchor drift and stale-base states
are labeled rather than hidden; approval resets on base change; local-CI parity
and divergence are explicit; header target identity is explicit; no header surface
creates hidden write scope; downgrade narrows the claim instead of hiding the
lane; and stale or underqualified rows block promotion.

Proof freshness SLO is 168 hours with automatic narrowing on stale proof. The
supported downgrade triggers are `proof_stale`, `policy_blocked`, `anchor_drift`,
`parity_divergence_unlabeled`, `stale_base_unlabeled`, `trust_narrowing`,
`scope_expansion_unqualified`, and `upstream_dependency_narrowed`.

## Boundary

Raw diff bodies, raw build logs, raw pipeline artifacts, raw provider payloads,
credentials, and live provider responses never cross this boundary. The packet
carries only metadata, qualification truth, and contract references. Every header
action stays read-only or attributable and reviewable.
