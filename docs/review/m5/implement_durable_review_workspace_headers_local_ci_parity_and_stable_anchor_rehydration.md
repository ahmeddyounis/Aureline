# Durable Review-Workspace Headers, Local-CI Parity, and Stable Anchor Rehydration

This document is the contract for the M5 packet that keeps a local review
workspace honest across edits, rebases, and reopens. The packet is the canonical
M5 control source for this lane: the review-workspace header, CLI/headless
output, diagnostics, Help/About, and support exports ingest the checked-in packet
rather than cloning status text.

- Record kind: `durable_review_header_local_ci_parity_and_anchor_rehydration`
- Schema: [`schemas/review/implement-durable-review-workspace-headers-local-ci-parity-and-stable-anchor-rehydration.schema.json`](../../../schemas/review/implement-durable-review-workspace-headers-local-ci-parity-and-stable-anchor-rehydration.schema.json)
- Canonical support export: [`artifacts/review/m5/implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration/support_export.json`](../../../artifacts/review/m5/implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration/support_export.json)
- Summary artifact: [`artifacts/review/m5/implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration.md`](../../../artifacts/review/m5/implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration.md)
- Fixtures: [`fixtures/review/m5/implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration/`](../../../fixtures/review/m5/implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration/)
- Producer: `aureline_review::current_durable_review_header_export`

## Pillars

### Durable review-workspace headers

Each `headers[]` row binds a review workspace to its `target_identity_label`
(what is under review), a `durable_anchor_id` that survives edits, rebases, and
reopens, a `base_freshness` class, an `approval_state`, a `mutation_authority`
class, and the `header_fields_shown` the surface projects. Stale-base and
outdated-diff states are labeled, never silently hidden, and approval state
resets on base change.

| Field | Source contract |
| --- | --- |
| target identity, anchor id | [`schemas/review/review_workspace.schema.json`](../../../schemas/review/review_workspace.schema.json) |
| anchor stability / rehydration | [`schemas/review/review_stabilization.schema.json`](../../../schemas/review/review_stabilization.schema.json) |

### Local-CI parity

Each `parity_lanes[]` row records a `check_id` shared by the local run and the CI
expectation it mirrors, an `enforcement` class, a `verdict`, the
`local_result_label`, the `ci_expectation_label`, and a `divergence_label`. The
verdict is one of `parity_match`, `local_only_advisory`, `divergence_labeled`, or
`unavailable_offline`. A `divergence_labeled` verdict must carry a non-empty
`divergence_label`, and at least one `required` check lane must be present so the
parity claim is anchored on a gating check. Parity mirrors the CI run contract at
[`schemas/ci/pipeline_run_row.schema.json`](../../../schemas/ci/pipeline_run_row.schema.json)
and the review-pack contract at
[`schemas/review/review_pack.schema.json`](../../../schemas/review/review_pack.schema.json).

### Stable anchor rehydration

Each `rehydration_events[]` row records an `anchor_id`, the `trigger` (`edit`,
`rebase`, `reopen`, `base_change`, `external_sync`), the `resulting_state`, and a
`drift_label`. A `drifted_relabeled` or `orphaned_flagged` state must carry a
non-empty `drift_label`. Every `durable_anchor_id` referenced by a header row must
appear in at least one rehydration event, so no header points at an anchor that
was never rehydrated.

## Track invariant

The `trust_review` block encodes the hard invariants — all must hold for the
packet to validate:

- `anchors_rehydrate_durably` and `anchor_drift_labeled_not_hidden` — anchors
  re-attach across edits, rebases, and reopens, and drift is relabeled rather
  than silently dropped.
- `stale_base_labeled_explicit` and `approval_resets_on_base_change` — stale-base
  and outdated-diff states are labeled, and approval resets when the base changes.
- `local_ci_parity_explicit` and `divergence_labeled_not_hidden` — parity
  verdicts are explicit and divergence is labeled.
- `target_identity_explicit`, `no_hidden_write_scope`,
  `downgrade_narrows_instead_of_hides`, and
  `stale_or_underqualified_blocks_promotion`.

## Downgrade and freshness

`proof_freshness` carries the SLO (168 hours) and last-refresh timestamp; when
proof goes stale `auto_narrow_on_stale` narrows the lane. The supported downgrade
triggers are `proof_stale`, `policy_blocked`, `anchor_drift`,
`parity_divergence_unlabeled`, `stale_base_unlabeled`, `trust_narrowing`,
`scope_expansion_unqualified`, and `upstream_dependency_narrowed`. The
[fixtures](../../../fixtures/review/m5/implement_durable_review_workspace_headers_local_ci_parity_and_stable_anchor_rehydration/)
show a stale-base header with a drifted anchor and an offline CI-parity packet;
both remain valid because narrowing is explicit, not hidden.

## Boundary

Raw diff bodies, raw build logs, raw pipeline artifacts, raw provider payloads,
credentials, and live provider responses never cross this boundary. The packet
carries only metadata, parity verdicts, anchor-rehydration outcomes, and contract
references. Every header action stays read-only or attributable and reviewable.
