# Waiver register and waiver decision fixtures

Worked fixtures for the waiver register, mitigation ledger, and
renewal-or-close decision contract frozen in
[`/docs/governance/waiver_register_contract.md`](../../../docs/governance/waiver_register_contract.md)
and the boundary schemas
[`/schemas/governance/waiver_register.schema.json`](../../../schemas/governance/waiver_register.schema.json)
and
[`/schemas/governance/waiver_decision.schema.json`](../../../schemas/governance/waiver_decision.schema.json).

Each register-entry case ships as a complete
`waiver_register_entry_record` covering one of the typed register
statuses. Each decision case ships as a complete
`waiver_decision_record` paired with the matching register entry. The
fixtures exercise the typed waiver-kind, authority, affected-target,
mitigation-ledger, expiry, current-evidence-gap, renewal-requested-
state, register-status, repeated-path-grouping, decision-history,
sponsor-or-forum, decision-class, and required-evidence-link
vocabularies plus the export-parity floor.

## Index

| Case | Fixture | Posture |
| --- | --- | --- |
| Standing single-maintainer-backup waiver | `active_within_expiry_single_maintainer_backup.yaml` | `register_active_within_expiry`, audit-only mitigation, `not_yet_due_review_window_open` expiry through 2026-10-19 |
| Expired waiver pending decision | `expired_pending_decision_release_floor_breach.yaml` | `register_expired_no_decision`, `expired_past_due` proximity, `renewal_requested_in_review` |
| Renewed under new decision | `renewed_under_new_decision.yaml` (paired with `decision_renew_save_pipeline.yaml`) | `register_renewed_under_new_decision`, `renewal_resolved_renewed`, narrower envelope |
| Closed correction landed | `closed_correction_landed.yaml` (paired with `decision_close_buffer_operations.yaml`) | `register_closed_correction_landed`, `correction_landed_renewal_minted`, audit-only on close |
| Narrowed marketed claim to subset | `narrowed_marketed_claim_to_subset.yaml` (paired with `decision_narrow_claim_ai_broker.yaml`) | `register_narrowed_claim_published`, `marketed_public_claim_narrowed`, managed-cloud descope |
| Repeated-protected-path cluster across release trains | `repeated_protected_path_cluster_across_release_trains.yaml` | `register_active_pending_renewal`, `repeated_across_release_trains`, `residual_risk_severe` |

## Intended usage

- **First-class governed object conformance.** Every register fixture
  carries a stable `waiver_register_entry_id`, a typed
  `waiver_kind_class`, a typed `waiver_authority_class`, a typed
  `affected_target_class` plus the matching non-null target ref, a
  typed mitigation ledger across all four axes, a typed expiry
  envelope, a typed current-evidence-gap class, a typed renewal-
  requested state, a typed register status, and a typed repeated-
  path grouping class. A surface that renders a waiver as a free-text
  comment is non-conforming.
- **Mitigation-ledger conformance.** Every fixture renders the typed
  `compensating_control_class`, `correction_work_state_class`,
  `residual_risk_class`, and `claim_scope_narrowing_class` plus the
  bounded reviewable summaries. A surface that drops the marketed-
  or-internal axis on `claim_scope_narrowing_class` is non-conforming.
- **Decision-history conformance.** Every register fixture whose
  `renewal_requested_state_class` is one of the resolved classes
  cites the matching decision record in `decision_history[]`. A
  decision recorded only as a meeting note is non-conforming and
  rejected by the schema's `allOf` block.
- **Decision-record conformance.** Every decision fixture cites the
  affected register entry id, the typed decision class, the typed
  sponsor or forum, the chronology, and the typed
  required-evidence-link entries the decision class is required to
  cite. A `narrow_claim` decision without a `narrowed_claim` block is
  non-conforming.
- **Repeated-path-cluster conformance.** The cluster fixture pins
  `repeated_path_grouping_class = repeated_across_release_trains` and
  cites the prior register-entry ids in `cluster_member_refs[]`. A
  surface that filters the cluster out as an isolated incident is
  non-conforming.
- **Export-parity conformance.** Every register fixture sets every
  `export_fields` boolean to `true`. The dashboard, the milestone
  scorecard, the release packet, the support export, the governance
  packet, the claim manifest, and the weekly governance review MUST
  consume the same record. A surface that reformats the register
  into a screenshot or a parallel narrative status note is
  non-conforming.

## Acceptance coverage

The acceptance criteria from
[`/.plans/M00-518.md`](../../../.plans/M00-518.md) are covered as
follows:

- **"A reviewer can assess active waivers, mitigations, and expiry
  without hunting across documents or chat history."** — every
  register fixture lays out the affected target, the owner DRI / lane
  / forum, the typed mitigation ledger, the typed expiry envelope,
  the typed current-evidence-gap, the typed renewal-requested state,
  and the typed register status as one schema-validated record.
- **"Expired waivers and repeated-path waivers are visible as
  operational risks rather than hidden appendix details."** — the
  `expired_pending_decision_release_floor_breach.yaml` fixture pins
  `expiry_proximity_class = expired_past_due` and
  `register_status_class = register_expired_no_decision`; the
  `repeated_protected_path_cluster_across_release_trains.yaml` fixture
  pins `repeated_path_grouping_class = repeated_across_release_trains`
  with `residual_risk_class = residual_risk_severe` and
  `cluster_member_refs[]` non-empty.
- **"Release, support, and governance packets can consume the same
  register without reformatting or screenshot capture."** — every
  register fixture sets every `export_fields` boolean to `true` and
  cites the matching `linked_release_truth_summary_refs`,
  `linked_nightly_report_row_refs`, and `linked_claim_or_support_impact_refs[]`.

## Out of scope

Approval-workflow runners, executive-routing pipelines, and
renewal-reminder schedulers are explicitly out of scope per
[`.plans/M00-518.md`](../../../.plans/M00-518.md). The contract
defines the source object; integrations consume it.
