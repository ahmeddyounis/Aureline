# Review-Component Accessibility, Headless, and Export Parity

This is the accessibility / headless / export capstone over the seven reusable M5
review components frozen in
`freeze_the_m5_review_request_check_and_merge_queue_component_matrix`, implemented by
the review-request-row, checks-summary-card, merge-readiness / merge-queue /
stack-dependency, and pending-review-tray / approval-invalidation-banner lanes, and
adopted by the shared consumers in
`add_shared_review_list_detail_companion_help_support_and_export_consumers_so_review_components_keep_label_action_and_handoff_parity`.

Where the consumer lane proves label / action / handoff parity across desktop
surfaces, this lane proves the harder claim: that review-request, checks-summary,
merge-readiness, queue, and approval-invalidation state is exposed just as honestly
in assistive, headless, and exported forms as it is on the desktop — and that a
claim-bearing component automatically narrows the moment its provider-backed truth
stops being trustworthy.

- Boundary schema: [`schemas/ui/m5-review-component-accessibility-parity.schema.json`](../../../schemas/ui/m5-review-component-accessibility-parity.schema.json)
- Support export: [`artifacts/review/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_provider_freshness_queue_authority_or_approval_lineage_is_stale_or_missing_across_claimed_m5_review_components/support_export.json`](../../../artifacts/review/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_provider_freshness_queue_authority_or_approval_lineage_is_stale_or_missing_across_claimed_m5_review_components/support_export.json)
- Protected fixtures: [`fixtures/ui/m5-review-component-accessibility-parity/`](../../../fixtures/ui/m5-review-component-accessibility-parity/)

## Parity across forms (AC1)

Every claimed component exposes five parity fields and renders on all three surfaces:

- `keyboard_label` — how the component is focused and operated by keyboard.
- `screen_reader_label` — the non-visual label, including its claim.
- `cli_enum_token` — the stable enum a headless CLI prints.
- `export_enum_token` — the stable enum the support export carries.
- `explanation_field` — a human-readable explanation of the current claim.

`rendering_surfaces` must cover `desktop_full`, `cli_headless`, and `support_export`.
No component may be pointer-only (`is_pointer_only`), export-opaque
(`is_export_opaque`), or semantically stronger on the desktop than in CLI or export
(`desktop_stronger_than_cli`) — all three guardrails must be false.

## Automatic claim narrowing (AC2)

Each component carries a claim about how trustworthy its provider-backed truth is,
drawn from `ReviewComponentClaimTier` (strongest first):

| Claim tier | Meaning | Rank |
| --- | --- | --- |
| `provider_backed` | Live provider-backed truth | 5 |
| `locally_reviewable` | Reviewable in full from local truth while backing is degraded | 4 |
| `estimate_only` | Queue authority is a local estimate, not provider ordering | 3 |
| `approval_unverified` | Approval lineage cannot be verified | 2 |
| `handoff_required` | Out of scope in-product; needs a browser handoff | 1 |

`resolve_review_component_claim_narrowing` maps each condition to the ceiling it
permits, the trigger it must disclose, and its next action:

| Condition | Permitted ceiling | Trigger | Next action | Handoff note | Local-continue note |
| --- | --- | --- | --- | --- | --- |
| `provider_fresh` | `provider_backed` | — | — | no | no |
| `provider_freshness_stale` | `locally_reviewable` | `provider_freshness_stale` | `refresh_provider_freshness` | no | yes |
| `queue_authority_local_estimate` | `estimate_only` | `queue_authority_dropped_to_local_estimate` | `reconcile_queue_authority` | no | yes |
| `approval_lineage_missing` | `approval_unverified` | `approval_lineage_missing` | `restore_approval_lineage` | no | yes |
| `browser_handoff_required` | `handoff_required` | `browser_handoff_required` | `open_browser_handoff` | yes | yes |

A component's `effective_claim` may never exceed the ceiling its condition permits
(`ClaimCeilingExceeded`) — this is the AC2 device that prevents a review, Help, or
evaluation surface from overstating provider-backed truth. A weakening condition must
carry an explicit `narrowing` disclosure pinned to that ceiling
(`ClaimNarrowingMissing` / `NarrowedToMismatch` / `NarrowTriggerMismatch` /
`NarrowNextActionMismatch`), keep the browser handoff explicit
(`BrowserHandoffNoteMissing`), and preserve local-only continuation
(`LocalContinueNoteMissing`). A `provider_fresh` row must not carry a narrowing
(`ClaimNarrowingUnexpected`).

## Coverage

The canonical row set covers all seven components (`ComponentCoverageMissing`), all
five conditions (`ConditionCoverageMissing`), and all five claim tiers reached as an
effective claim (`ClaimTierCoverageMissing`). Every row points at its component's
canonical schema and the frozen component matrix
(`CanonicalContractReferenceMissing`).

## Canonical component contracts

`component_canonical_schema_ref` maps each component to the schema of the lane that
implemented it:

| Component(s) | Canonical schema |
| --- | --- |
| `review_request_row` | `schemas/ui/m5-review-request-row.schema.json` |
| `checks_summary_card` | `schemas/ui/m5-checks-summary-card.schema.json` |
| `merge_readiness_panel`, `merge_queue_entry`, `stack_dependency_chip` | `schemas/ui/m5-merge-readiness-panel.schema.json` |
| `pending_review_tray`, `approval_invalidation_banner` | `schemas/ui/m5-pending-review-tray.schema.json` |

## Regenerating artifacts

The support export, Markdown summary, and fixtures are checked in. Regenerate them
after a contract change:

```sh
GEN_REVIEW_COMPONENT_ACCESSIBILITY_ARTIFACTS=1 cargo test -p aureline-review --lib \
  regenerate_review_component_accessibility_artifacts
```
