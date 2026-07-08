# M5 Review-Request, Checks, and Merge-Queue Component Matrix

This document is the contract for the frozen M5 matrix that locks seven reusable
review components. The matrix is the canonical M5 component source for this lane:
review workspaces, merge-queue panels, pending-review trays, merge-readiness
panels, Help/About surfaces, and support exports consume the checked-in packet
rather than cloning row text or minting provider-specific badges.

- Record kind: `freeze_m5_review_request_check_and_merge_queue_component_matrix`
- Schema: [`schemas/ui/m5-review-request-check-queue-component-matrix.schema.json`](../../../schemas/ui/m5-review-request-check-queue-component-matrix.schema.json)
- Canonical support export: [`artifacts/release/m5-review-request-check-queue-proof/support_export.json`](../../../artifacts/release/m5-review-request-check-queue-proof/support_export.json)
- Summary artifact: [`artifacts/release/m5-review-request-check-queue-proof/summary.md`](../../../artifacts/release/m5-review-request-check-queue-proof/summary.md)
- Design matrix: [`artifacts/design/m5-review-request-check-queue-component-matrix.md`](../../../artifacts/design/m5-review-request-check-queue-component-matrix.md)
- Fixtures: [`fixtures/ui/m5-review-request-check-queue-components/`](../../../fixtures/ui/m5-review-request-check-queue-components/)
- Producer: `aureline_review::current_stable_m5_review_component_matrix_export`

## Components

| Component | Maturity | Source contract |
| --- | --- | --- |
| `review_request_row` | Stable | [`schemas/review/review_workspace.schema.json`](../../../schemas/review/review_workspace.schema.json) |
| `checks_summary_card` | Stable | [`schemas/ci/pipeline_run_row.schema.json`](../../../schemas/ci/pipeline_run_row.schema.json) |
| `pending_review_tray` | Stable | [`schemas/review/review_surface_record.schema.json`](../../../schemas/review/review_surface_record.schema.json) |
| `merge_readiness_panel` | Stable | [`schemas/review/landing_candidate.schema.json`](../../../schemas/review/landing_candidate.schema.json) |
| `merge_queue_entry` | Stable | [`schemas/review/merge_queue_entry.schema.json`](../../../schemas/review/merge_queue_entry.schema.json) |
| `stack_dependency_chip` | Beta | [`schemas/review/change_lineage.schema.json`](../../../schemas/review/change_lineage.schema.json) |
| `approval_invalidation_banner` | Preview | [`schemas/review/add-merge-queue-readiness-stale-base-invalidation-and-approval-recomputation-flows.schema.json`](../../../schemas/review/add-merge-queue-readiness-stale-base-invalidation-and-approval-recomputation-flows.schema.json) |

Each component row binds a maturity class to the exact provider-versus-local
distinction, stale-provider downgrade vocabulary, browser-handoff boundary, and
local-continue fallback it must preserve, plus its evidence requirement, required
evidence packet refs, downgrade triggers, rollback posture, source contracts, and
the consumer surfaces that must project the component's truth.

## Provider / local distinction

Every component keeps provider-managed truth and local estimate separate. The
`provider_local_distinction` field on each row names exactly which fields are
provider-backed and which are local-computed, and the `trust_review` invariant
`provider_local_estimate_distinct` requires this separation to hold for the matrix
to validate.

## Stale-provider downgrade vocabulary

The `stale_provider_downgrade_vocab` on each row draws from a fixed vocabulary:
`provider_fresh`, `provider_refreshing`, `provider_stale`, `provider_unreachable`,
`provider_conflict`, and `local_only_continuation`. Provider staleness is named
with this vocabulary rather than flattened into a local estimate or hidden behind a
generic warning; `stale_provider_downgrade_explicit` encodes the invariant.

## Browser-handoff boundary and local-continue fallback

Each row names its `browser_handoff_boundary` — the point at which opening the
provider host is an explicit handoff with a labeled return path — and its
`local_continue_fallback` — how local review continues when provider freshness is
degraded. The `browser_handoff_explicit` and
`local_continue_preserved_on_degraded_freshness` invariants keep these honest.

## Track invariant

Provider identity, base/head or stack relation, check class, queue owner,
local-versus-provider estimate, freshness, approval invalidation, and
offline/browser-handoff continuity stay explicit wherever Aureline lists, opens,
triages, exports, or hands off reviews. The `trust_review` block encodes these as
hard invariants — all must hold:

- `provider_local_estimate_distinct`, `stale_provider_downgrade_explicit`, and
  `approval_invalidation_never_generic_warning`.
- `browser_handoff_explicit` and
  `local_continue_preserved_on_degraded_freshness`.
- `stack_blocking_explicit`, `queue_ownership_explicit`, and
  `check_class_explicit`.
- `no_forced_raw_provider_navigation_for_triage`,
  `downgrade_narrows_instead_of_hides`, and
  `stale_or_underqualified_blocks_promotion`.

## Downgrade and freshness

`proof_freshness` carries the SLO (168 hours) and last-refresh timestamp; when
proof goes stale `auto_narrow_on_stale` narrows the affected component. The
supported downgrade triggers are `proof_stale`, `policy_blocked`,
`provider_freshness_stale`, `approval_invalidated`, `stack_parent_blocked`,
`queue_ownership_unresolved`, `check_class_unverified`,
`browser_handoff_unavailable`, `trust_narrowing`, `scope_expansion_unqualified`,
and `upstream_dependency_narrowed`. The
[fixtures](../../../fixtures/ui/m5-review-request-check-queue-components/) show a
merge-queue entry narrowing on stale provider status and a held
approval-invalidation banner; both remain valid packets because narrowing is
explicit, not hidden.

## Boundary

Raw diff bodies, raw check logs, raw provider payloads, credentials, and live
provider responses never cross this boundary. The packet carries only metadata,
component truth, and contract references. Every provider mutation, browser handoff,
and publish-later action stays attributable and reviewable.
