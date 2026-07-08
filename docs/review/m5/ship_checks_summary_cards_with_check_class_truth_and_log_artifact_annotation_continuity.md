# Checks-Summary Cards: Check-Class Truth and Log/Artifact/Annotation Continuity

This document is the contract for the M5 packet that implements the reusable
checks-summary card. It narrows the `checks_summary_card` component frozen in the
[review-request/checks/merge-queue component matrix](../../../schemas/ui/m5-review-request-check-queue-component-matrix.schema.json)
into an implemented card contract. The packet is the canonical M5 control source
for this lane: the review workspace, review lists, companion queues, handoff
packets, CLI/headless output, diagnostics, Help/About, checks-summary drawers, and
support exports ingest the checked-in packet rather than cloning check pills or a
single provider-specific gate number.

- Record kind: `checks_summary_card_check_class_and_evidence_continuity`
- Schema: [`schemas/ui/m5-checks-summary-card.schema.json`](../../../schemas/ui/m5-checks-summary-card.schema.json)
- Canonical support export: [`artifacts/review/m5/ship_checks_summary_cards_with_check_class_truth_and_log_artifact_annotation_continuity/support_export.json`](../../../artifacts/review/m5/ship_checks_summary_cards_with_check_class_truth_and_log_artifact_annotation_continuity/support_export.json)
- Summary artifact: [`artifacts/review/m5/ship_checks_summary_cards_with_check_class_truth_and_log_artifact_annotation_continuity.md`](../../../artifacts/review/m5/ship_checks_summary_cards_with_check_class_truth_and_log_artifact_annotation_continuity.md)
- Fixtures: [`fixtures/ui/m5-checks-summary-cards/`](../../../fixtures/ui/m5-checks-summary-cards/)
- Producer: `aureline_review::current_checks_summary_card_export`

## The card contract

Each `cards[]` entry summarizes the checks for one review. It answers, from the
card alone, which review and provider it belongs to, how fresh that provider truth
is, and — for each check — what its disposition is and where its evidence lives:

| Field | Meaning |
| --- | --- |
| `review_id_label`, `provider_identity_label` | Which review and which provider or local object owns the checks. |
| `provider_freshness` | Provider-freshness state reused verbatim from the frozen matrix (`M5ReviewComponentStaleProviderState`). |
| `presents_single_verdict` | Whether the card shows one pass/fail number; must be `false` when richer per-check evidence exists. |
| `headline_verdict_label` | A human-readable headline shown **alongside** the per-check breakdown, never instead of it. |
| `checks[]` | The per-check entries (see below). |

Each `checks[]` entry carries `check_class`, an `evaluation_reason`, its
`evidence_links`, and its `actions`.

## Check class — the anti-flattening axis

`check_class` is the honesty axis. A reader can tell the seven classes apart from
the card alone:

- `required` — gates merge readiness.
- `optional` — does not gate merge readiness.
- `skipped` — intentionally skipped for this change.
- `suppressed` — result suppressed by policy or configuration.
- `timed_out` — timed out before returning a verdict.
- `stale` — provider-backed result is stale relative to the head it gates.
- `not_evaluated_here` — not evaluated in this local/offline context.

`resolve_checks_summary_card_disclosure(provider_freshness, has_richer_evidence)`
derives what a card must disclose:

- `must_not_flatten_to_single_verdict` holds when the card carries richer evidence —
  more than one check, any log/artifact/annotation link, or any anomalous class. A
  card that sets `presents_single_verdict: true` in that case fails validation with
  `checks_flattened_to_single_verdict`. **This is the AC1 device**: review and
  companion surfaces no longer flatten checks into one pass/fail number when richer
  evidence is available.
- `needs_local_continue_fallback` holds when provider freshness is `provider_stale`,
  `provider_unreachable`, `provider_conflict`, or `local_only_continuation`. A
  missing `local_continue_fallback` fails with `local_continue_fallback_missing`.
- `needs_browser_handoff_boundary` holds for any `provider_unreachable` card. A
  missing `browser_handoff_boundary` fails with `browser_handoff_boundary_missing`.

Every anomalous class (`skipped`, `suppressed`, `timed_out`, `stale`,
`not_evaluated_here`) must carry a non-empty `evaluation_reason`, or validation
fails with `check_evaluation_reason_missing` — so an anomalous check is never
silently folded into a pass/fail.

## Evidence continuity — the identity axis

Each `evidence_links[]` entry carries `review_id_ref` and `check_id_ref`. A link
that drops either fails validation with `evidence_identity_not_preserved`. **This is
the AC2 device**: log/artifact/annotation navigation preserves the originating
review and check identity across open, reopen, and export paths.

## Track invariant

The `trust_review` block encodes the hard invariants — all must hold for the packet
to validate: `required_optional_distinct`, `anomalous_check_states_distinct`,
`checks_never_flattened_when_richer_evidence`,
`log_artifact_annotation_identity_preserved`,
`provider_outage_preserves_local_continuation`,
`stale_sync_never_collapses_review_lane`, `rerun_cancel_only_where_allowed`,
`no_forced_raw_provider_navigation_for_triage`,
`not_evaluated_or_stale_never_shown_as_pass`,
`one_card_contract_no_hidden_provider_meaning`,
`downgrade_narrows_instead_of_hides`, and
`stale_or_underqualified_blocks_promotion`.

Two guardrails are enforced structurally beyond the trust bits:

- **No forced raw-provider navigation.** A card whose checks expose only
  `open_provider_in_browser` for triage fails with `forced_raw_provider_navigation`;
  ordinary triage keeps an in-product action such as `open_log`, `rerun_check`, or
  `continue_local_review`.
- **Check-class coverage.** A packet must include at least one check of every class
  so all seven are distinguishable in the same lane; otherwise it fails with
  `check_class_coverage_missing`.

## Downgrade and freshness

`proof_freshness` carries the SLO (168 hours) and last-refresh timestamp; when proof
goes stale `auto_narrow_on_stale` narrows the lane. The supported downgrade triggers
are `proof_stale`, `policy_blocked`, `provider_freshness_stale`, `check_timed_out`,
`check_evaluation_suppressed`, `browser_handoff_unavailable`, `trust_narrowing`,
`scope_expansion_unqualified`, and `upstream_dependency_narrowed`. The
[fixtures](../../../fixtures/ui/m5-checks-summary-cards/) show a stale provider-backed
card preserving local continuation and an unreachable-provider card whose timed-out
and not-evaluated checks stay distinct; both remain valid because narrowing is
explicit, not hidden.

## Boundary

Raw check logs, raw artifact bytes, raw annotation payloads, credentials, and live
provider responses never cross this boundary. The packet carries only metadata,
freshness states, check-class distinctions, evidence-link identity, and contract
references.
