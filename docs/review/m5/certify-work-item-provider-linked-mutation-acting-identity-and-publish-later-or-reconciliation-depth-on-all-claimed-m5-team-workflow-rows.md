# M5 Provider-Backed Team-Workflow Certification

This document is the contract for the M5 certification packet that qualifies
every claimed provider-backed work-item and team-workflow row on the mainline
branch. The
[frozen governance matrix](freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix.md)
locks the canonical row claims; this packet certifies those same rows against
current evidence and publishes one export-safe result that release, Help/About,
service-health, public-truth, and support surfaces all ingest.

- Record kind: `certify_work_item_provider_linked_mutation_acting_identity_and_publish_later_or_reconciliation_depth_on_all_claimed_m5_team_workflow_rows`
- Schema: [`schemas/review/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows.schema.json`](../../../schemas/review/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows.schema.json)
- Canonical support export: [`artifacts/review/m5/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows/support_export.json`](../../../artifacts/review/m5/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows/support_export.json)
- Summary artifact: [`artifacts/review/m5/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows.md`](../../../artifacts/review/m5/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows.md)
- Fixtures: [`fixtures/review/m5/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows/`](../../../fixtures/review/m5/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows/)
- Producer / first consumer: `aureline_review::certify_from_current_team_workflow_exports`
- Reader: `aureline_review::current_m5_team_workflow_certification_export`

## Certified rows

Each certified row keeps five facts together:

1. the claimed qualification and certification verdict;
2. the current feature-scorecard summary and evidence refs;
3. provider-family compatibility so issue-tracker, code-host, and CI capability
   gaps do not collapse into one generic supported badge;
4. acting-identity plus offline or publish-later continuity posture; and
5. reconciliation proof that governs typed import, dedupe, replay, and
   callback-deny audit paths.

| Row | Claimed | Verdict | Primary evidence |
| --- | --- | --- | --- |
| `work_item_object_vocabulary` | Stable | Certified | [governance matrix](freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix.md) |
| `provider_linked_mutation` | Stable | Certified | [work-item mutation review](ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews.md) |
| `acting_identity_and_effective_scope` | Stable | Certified | [`docs/providers/m5/provider_scope_review.md`](../../../docs/providers/m5/provider_scope_review.md) |
| `browser_handoff_continuity` | Stable | Certified | [browser handoff continuity](ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links.md) |
| `deferred_publish_continuity` | Stable | Certified | [deferred publish recovery](ship_deferred_publish_queue_recovery_packets.md) |
| `provider_event_reconciliation` | Beta | Narrowed | [`docs/providers/m5/event_ingestion.md`](../../../docs/providers/m5/event_ingestion.md) |

## First-consumer certification

`certify_from_current_team_workflow_exports` is the first real consumer of the
claimed rows. It validates:

- the checked governance matrix export;
- the checked work-item mutation review export;
- the checked browser-handoff continuity export;
- the checked deferred-publish recovery export;
- the checked provider-scope fixture; and
- the checked provider-event-ingestion fixture.

If any row's evidence no longer validates, that row is blocked immediately. If
the row's proof freshness or an upstream dependency narrows, the row degrades to
`narrowed_certified` instead of inheriting broader M5 marketing.

## Downgrade automation

`apply_downgrade_automation` consumes per-row observations and enforces the
guardrails from the frozen matrix:

- stale provider-authority proof narrows the affected claim;
- stale publish-later continuity proof narrows the affected claim;
- stale reconciliation proof narrows the affected claim; and
- invalid evidence blocks the affected row outright.

The compatibility report is recomputed every time, so release or support tooling
cannot publish greener than the row verdicts prove.

## Boundary

Raw provider payloads, raw callback URLs, secrets, bearer tokens, and hidden
scope text never cross this boundary. The packet carries only certification
metadata, typed verdicts, compact provider-family posture, and contract
references.
