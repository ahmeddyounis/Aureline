# Related-evidence cards and offline-handoff packet cards

Status: implemented (M05-984, batch B116)

This lane narrows two components frozen in the
[M5 work-item component matrix](m5_work_item_component_matrix.md) — the
`related_evidence_card` and the `offline_handoff_packet_card` — into one implemented,
export-safe packet with two co-equal control vectors. Together they preserve the
engineering evidence tied to a work item and keep publish-later recovery explicit when
the provider is unavailable or a write is blocked.

- Boundary schema: [`schemas/ui/m5-related-evidence-offline-handoff-controls.schema.json`](../../schemas/ui/m5-related-evidence-offline-handoff-controls.schema.json)
- Per-component contracts: [`schemas/ui/m5-related-evidence-card.schema.json`](../../schemas/ui/m5-related-evidence-card.schema.json), [`schemas/ui/m5-offline-handoff-packet-card.schema.json`](../../schemas/ui/m5-offline-handoff-packet-card.schema.json)
- Rust module: `crates/aureline-provider/src/implement_related_evidence_cards_and_offline_handoff_packet_cards_with_summary_first_evidence_redaction_state_publish_later_target_and_copy_export_retry_truth`
- Headless emitter: `cargo run -p aureline-provider --bin aureline_related_evidence_offline_handoff_primitive -- <subcommand>`
- Release proof: [`artifacts/release/m5-related-evidence-offline-handoff-proof/`](../../artifacts/release/m5-related-evidence-offline-handoff-proof/)
- Scenario fixtures: [`fixtures/ui/m5-related-evidence-offline-handoff-controls/`](../../fixtures/ui/m5-related-evidence-offline-handoff-controls/)

## Related-evidence card

A `RelatedEvidenceCard` summarizes one linked engineering context attached to a work
item — a review thread, a branch/worktree change, a failing/passing test, a CI check,
an incident/runbook artifact, or a docs/ADR reference — leading with a plain summary
and an **open-detail** action rather than dumping the raw artifact first. It reuses the
frozen evidence kinds (`test_result`, `ci_check`, `review_thread`, `linked_change`,
`attached_artifact`, `external_reference`) and states a summarized outcome (`passing`,
`failing`, `informational`, `unknown_outcome`).

Each card carries a **derived** freshness class — never asserted:

| condition | derived `freshness_class` | note required |
| --- | --- | --- |
| freshness not known | `unknown_freshness` | yes |
| not provider-backed | `local_only_evidence` | yes |
| provider-backed but out of date | `stale_evidence` | yes |
| provider-backed and current | `current_evidence` | no |

A failing outcome requires a `failure_note`. Every card must set `leads_with_summary`
and offer `open_detail` + `copy_reference`; a card that drops the summary or the
open-detail action fails with `raw_artifact_dumped_before_summary` /
`evidence_summary_missing` / `evidence_open_detail_missing`. This is the teeth behind
the acceptance criterion that **work-item detail surfaces expose summary-first evidence
instead of dumping raw artifacts first**. The seed corpus covers all six evidence
kinds, all four outcomes, and all four freshness classes.

## Offline-handoff packet card

An `OfflineHandoffPacketCard` shows the packet type, the included metadata/evidence,
the **redaction state** (reusing the frozen export boundaries `metadata_safe`,
`body_excluded`, `identifiers_masked`, `credentials_scrubbed`, `local_only`,
`full_disclosure_blocked`), and the **publish-later target** (reusing the frozen
handoff destinations `local_queue`, `provider_publish`, `exported_packet`,
`support_bundle`, `another_device`, `discard_after_review`). It reuses the frozen
local-versus-provider states and derives its acceptance class:

| condition | derived `acceptance_class` | consequences |
| --- | --- | --- |
| prior publish failed (or `publish_failed`) | `publish_failed_retryable` | failure-recovery note + retry action required; never accepted |
| provider publish + synced | `provider_accepted` | the only class that may imply acceptance |
| local-queue destination | `held_local_only` | retry action required; never accepted |
| exported / bundle / device / discard destination | `exported_for_handoff` | never accepted |
| any other unsynced provider publish | `queued_not_yet_accepted` | retry action required; never accepted |

Only `provider_accepted` may set `implies_provider_accepted`; a held, queued, or failed
packet that claims acceptance fails with `provider_acceptance_misrepresented` — the
teeth behind *never imply provider acceptance*. Every packet must keep copy/export
parity (`copy_packet` + `export_packet` mandatory), must offer `retry_publish` whenever
it can still reach the provider, must set `remains_visible_after_failure`, and must not
set `collapses_into_error_banner`; a packet that hides itself behind a generic banner
fails with `packet_collapsed_into_error_banner`. This is the teeth behind the
acceptance criterion that **offline packets remain visible, retryable, and exportable
after failure rather than collapsing into generic error banners**. The seed corpus
covers all five acceptance classes, all six handoff destinations, and all six export
boundaries.

## Guardrails

- Generic ticket/task wording may never conceal evidence provenance, the handoff
  destination, or the export boundary (`generic_ticket_wording_used`).
- Raw work-item bodies, pasted paths, credentials, and private endpoints never cross
  the support boundary; the export-safe packet carries only opaque metadata strings
  (`raw_boundary_material_in_export`).
- Stale proof automatically narrows the lane (`auto_narrow_on_stale`).
