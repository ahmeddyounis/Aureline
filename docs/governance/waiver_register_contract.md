# Waiver register, mitigation ledger, and renewal-or-close decision contract

This contract freezes one shared vocabulary for the waiver register, the
mitigation ledger that hangs off each register row, and the renewal-or-
close decision objects that record sponsor / forum attribution and the
evidence each decision class is required to cite. It exists so a waiver
is reviewable as a first-class governed object — not as a comment on a
pull request, not as a line in a meeting note, not as a paragraph in a
hidden release-notes appendix.

The waiver register is a projection over the underlying waiver records.
A register entry pins one waiver to its affected requirement row or
protected path, names the typed owner (DRI, lane, decision forum, and
backup-owner posture), names the typed mitigation ledger (compensating
controls, target correction work, residual risk, and whether the waiver
narrows a marketed claim or stays internal-only), names the typed
expiry envelope, names the typed current evidence gap, names the typed
renewal-requested state, names the typed register status, names the
typed grouping / compare classes for repeated waivers on the same
protected path across milestones or release trains, names the linked
claim / support-impact refs, and renders verbatim across the dashboard,
the milestone scorecard, the release-evidence shiproom packet, the
support bundle, the public claim manifest, the governance packet, and
the weekly governance review.

The waiver decision is a separate object family. Each `waiver_decision_record`
records one renewal-or-close decision against an entry — `renew`,
`close`, `narrow_claim`, `escalate_correction`, or `reject` — together
with the sponsor or forum attribution and the typed evidence links the
decision class is required to cite. Multiple decision records compose
the entry's `decision_history[]`.

The contract is pre-implementation. It defines the reusable record
shapes, the closed vocabularies, the projection rules, the export-
parity floor, and the fixture corpus. It does not implement an
approval-workflow runner, an executive routing pipeline, or a renewal
reminder. External project-management systems and executive routing
software are explicitly out of scope.

## Companion artifacts

- [`/schemas/governance/waiver_register.schema.json`](../../schemas/governance/waiver_register.schema.json)
  — boundary schema for one `waiver_register_entry_record`.
- [`/schemas/governance/waiver_decision.schema.json`](../../schemas/governance/waiver_decision.schema.json)
  — boundary schema for one `waiver_decision_record`.
- [`/fixtures/governance/waiver_cases/`](../../fixtures/governance/waiver_cases/)
  — worked records covering an active waiver inside its expiry window,
  an expired waiver pending decision, a renewed waiver carrying a
  paired decision row, a closed waiver with correction landed, a
  narrowed-claim waiver that pulled a marketed claim back to internal-
  only, and a repeated-protected-path cluster spanning two release
  trains.
- [`/schemas/governance/waiver_expiry.schema.json`](../../schemas/governance/waiver_expiry.schema.json)
  and
  [`/schemas/governance/waiver_expiry_item.schema.json`](../../schemas/governance/waiver_expiry_item.schema.json)
  — the underlying waiver record vocabulary and the dashboard-side
  expiry queue. Every register entry's `waiver_record_ref` resolves
  through the waiver-expiry record family; the register reuses the
  waiver-kind, escalation-path, and authority vocabularies frozen
  there.
- [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  — DRI, backup-owner, decision-forum, lane, and waiver register seed.
  Every waiver register entry's `owner.primary_dri`, `owning_lane`, and
  `decision_forum_ref` resolves into this matrix.
- [`/schemas/governance/requirement_status_snapshot.schema.json`](../../schemas/governance/requirement_status_snapshot.schema.json)
  — requirement-status snapshot. A register row whose
  `affected_target_class = requirement_row` carries a `requirement_id`
  that resolves through the snapshot's requirement-register link.
- [`/schemas/governance/claim_manifest.schema.json`](../../schemas/governance/claim_manifest.schema.json)
  — claim manifest. A register row that narrows a marketed claim
  cites the affected `claim_row` ids verbatim.
- [`/artifacts/governance/decision_register.yaml`](../../artifacts/governance/decision_register.yaml)
  and
  [`/schemas/governance/decision_register.schema.json`](../../schemas/governance/decision_register.schema.json)
  — decision register. A waiver decision that escalates correction
  work or rejects a renewal cites the gating decision register row
  verbatim.

## Normative sources projected here

- `.t2/docs/Aureline_PRD.md` — waiver, narrowing, evidence, signoff,
  release-evidence, and claim-publication requirements (RFC 2119
  MUST / SHOULD language).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` — release
  governance, decision rights, escalation paths, and the waiver
  register's role in the protected-path posture.
- `.t2/docs/Aureline_Technical_Design_Document.md` — release-evidence
  and supportability-evidence record shapes.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — waiver disclosure,
  release-status, and known-limit disclosure rules.

If this contract disagrees with those sources, those sources win and
this contract, the schemas, and the fixtures update in the same
change.

## 1. Why this contract exists

1. **Scattered waivers fail review.** Without one shared register row
   shape, a waiver lives as a comment on a pull request, a paragraph
   in a meeting note, a sentence in a hidden release-notes appendix,
   or a bullet on a private slide. The reviewer cannot assess active
   waivers, their mitigations, or their expiry without hunting across
   documents and chat history. The register exists so every consuming
   surface renders the **same** typed entry with the **same** stable
   IDs.
2. **Mitigation must be visible next to the waiver.** A waiver without
   a typed compensating control, a typed target correction work item,
   and a typed residual-risk class is not a governed waiver — it is a
   deferral with no plan. The mitigation ledger pins those fields on
   the register row so a reviewer can read the mitigation posture
   directly off the entry.
3. **Marketed-or-internal narrowing must be explicit.** A waiver that
   narrows a marketed claim must say so; a waiver that narrows only an
   internal-only path must say so. The `claim_scope_narrowing_class`
   pins the marketed-or-internal axis on the entry so the public claim
   manifest, the About / Help disclosure, and the support packet quote
   the same narrowing posture.
4. **Decisions are first-class objects.** A renewal, a close, a claim-
   narrowing, an escalation, or a rejection is a typed
   `waiver_decision_record` with a sponsor or forum attribution and
   the typed evidence the decision class is required to cite. The
   decision history of a register row is the array of decision records
   that cite the row's `waiver_register_entry_id`. A decision recorded
   only as a meeting note is non-conforming.
5. **Repeated waivers on the same path are operational risks.** A
   waiver that keeps coming back on the same protected path across
   milestones or release trains is chronic drift, not an isolated
   incident. The `repeated_path_grouping_class` and the
   `cluster_member_refs` array surface the cluster on the register row
   so the dashboard, the weekly governance review, and the release
   council read the cluster as a pattern.
6. **Expired waivers cannot hide.** An expired waiver is an open
   operational risk until a decision is recorded; it must remain
   visible on every consuming surface and MUST NOT be filtered out
   under an "owner inspecting" affordance.
7. **Release, support, and governance packets consume the same
   register.** The release-evidence shiproom packet, the support
   bundle, the public claim manifest, the governance packet, and the
   weekly governance review read the same `waiver_register_entry_record`
   and the same `decision_history[]` projection. A surface that
   reformats the register, takes a screenshot of the dashboard, or
   reconstructs a parallel register from raw notes is non-conforming.

## 2. Register entry shape

A `waiver_register_entry_record` carries:

- `waiver_register_entry_id` — stable, machine-readable id quoted by
  every consuming surface.
- `evaluated_at` — RFC 3339 UTC timestamp at which the entry was
  projected. Distinct from the underlying waiver `declared_chronology`
  and from the decision-record chronology; the entry is reprojected
  whenever expiry proximity changes, a decision is recorded, or
  mitigation status changes.
- `waiver_record_ref` — opaque ref into a `waiver_expiry_record` on
  [`/schemas/governance/waiver_expiry.schema.json`](../../schemas/governance/waiver_expiry.schema.json)
  or into the waiver register seed on
  [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml).
- `waiver_kind_class` — typed waiver kind mirroring
  [`/schemas/governance/waiver_expiry_item.schema.json`](../../schemas/governance/waiver_expiry_item.schema.json)
  (§3).
- `waiver_authority_class` — typed authority mirroring
  [`/schemas/governance/waiver_expiry_item.schema.json`](../../schemas/governance/waiver_expiry_item.schema.json).
- `affected_target` — typed `affected_target_class` plus the matching
  `requirement_id`, `protected_path_ref`, `claim_row_ref`,
  `release_train_ref`, `milestone_lane_ref`,
  `compatibility_surface_ref`, `retention_path_ref`, or
  `export_pipeline_ref` (§4).
- `owner` — typed `primary_dri`, `evidence_owner`, `owning_lane`,
  `decision_forum_ref`, and `backup_owner` (§5). Personal handles MUST
  resolve into the ownership matrix.
- `mitigation_ledger` — typed `compensating_control_class`,
  typed `correction_work_state_class`, typed `residual_risk_class`,
  typed `claim_scope_narrowing_class`, plus bounded reviewable
  `compensating_controls_summary`, `target_correction_work_summary`,
  `residual_risk_summary`, and `claim_scope_summary` (§6).
- `expiry` — typed `expiry_proximity_class`, the `expires_at`
  timestamp copied from the waiver record, and a bounded reviewable
  `expiry_summary` (§7).
- `current_evidence_gap` — typed `current_evidence_gap_class` plus a
  bounded reviewable `evidence_gap_summary` and the
  `linked_evidence_gap_refs[]` into the absent or stale evidence
  packets (§8).
- `renewal_requested_state` — typed `renewal_requested_state_class`
  plus the bounded reviewable `renewal_requested_summary` and the
  `linked_decision_refs[]` into the open or in-review decision rows
  (§9).
- `decision_history[]` — typed entries citing every closed
  `waiver_decision_record` that resolved against this register row
  (§10). Empty only when `register_status_class` is
  `register_active_within_expiry` and no decision has been recorded.
- `register_status_class` — typed register status (§11).
- `repeated_path_grouping_class` — typed grouping vocabulary for
  repeated waivers on the same protected path across milestones or
  release trains (§12).
- `cluster_member_refs` — refs into other waiver register entry ids
  that share the cluster.
- `linked_claim_or_support_impact_refs[]` — typed refs into the claim
  surfaces (claim manifest rows, About / Help disclosures, public
  proof rows) and support surfaces (support-export packets, support
  drills, recovery rehearsals) that the waiver narrows or affects
  (§13).
- `linked_requirement_status_row_refs` — refs into the requirement-
  status snapshot rows whose state is held by this register row.
- `linked_nightly_report_row_refs` — refs into the nightly-row ids
  that triggered or are gated by this register row.
- `linked_release_truth_summary_refs` — refs into the release-truth
  summaries that quote this register row.
- `headline_label` and `register_summary` — bounded reviewable label
  and one-sentence summary.
- `export_fields` — typed booleans for dashboard, milestone scorecard,
  release packet, support export, governance packet, claim manifest,
  and weekly governance review (§14).

## 3. Waiver-kind, authority, and escalation vocabularies

The waiver-kind and waiver-authority vocabularies on the register row
are inherited verbatim from
[`/schemas/governance/waiver_expiry_item.schema.json`](../../schemas/governance/waiver_expiry_item.schema.json):

- `waiver_kind_class` — closed nine-class vocabulary
  (`freeze_exception`, `policy_narrowing`, `retention_exception`,
  `release_floor_breach_waiver`, `support_drill_pending_waiver`,
  `compatibility_window_waiver`, `claim_attestation_pending_waiver`,
  `single_maintainer_backup`, `other_named_in_summary`).
- `waiver_authority_class` — closed five-class vocabulary
  (`performance_council`, `architecture_council`, `release_council`,
  `shiproom_executive_scope_review`, `not_applicable_no_active_waiver`).
  `not_applicable_no_active_waiver` is forbidden on a register entry;
  the register exists because a waiver was minted.

The decision record carries an additional `decision_sponsor_class`
(§16) so a sponsor or forum attribution is required on every decision.

## 4. Affected-target vocabulary

Closed eight-class `affected_target_class` vocabulary:

| Class | Meaning |
| --- | --- |
| `requirement_row` | Affects a canonical requirement row from the requirement register. `requirement_id` MUST be non-null. |
| `protected_path` | Affects a protected fitness-function-catalog row, a protected-metric, or a protected-merge-class. `protected_path_ref` MUST be non-null. |
| `claim_row` | Affects a claim manifest row directly. `claim_row_ref` MUST be non-null. |
| `release_train` | Affects an entire release train (stable, LTS, preview). `release_train_ref` MUST be non-null. |
| `milestone_lane` | Affects a milestone scorecard lane. `milestone_lane_ref` MUST be non-null. |
| `compatibility_surface` | Affects a compatibility-surface row from the inventory. `compatibility_surface_ref` MUST be non-null. |
| `retention_path` | Affects a retention / deletion matrix row. `retention_path_ref` MUST be non-null. |
| `export_pipeline` | Affects an export, delete, or disclosure pipeline. `export_pipeline_ref` MUST be non-null. |

A register entry whose target cannot be typed denies with
`affected_target_class_unresolved` rather than defaulting. The schema
enforces that the matching ref is non-null and the non-matching refs
are null.

## 5. Owner block

The `owner` block carries:

- `primary_dri` — primary DRI handle. Resolves into
  [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml).
- `evidence_owner` — evidence owner handle. Resolves into the same
  matrix.
- `owning_lane` — owning scorecard lane. Resolves into
  `ownership_matrix.scorecard_lane_index`.
- `decision_forum_ref` — decision forum that minted the waiver and
  that owns renewal authority.
- `backup_owner` — backup owner handle. Null is admissible only when
  `waiver_kind_class = single_maintainer_backup`.

Personal handles, raw email addresses, raw phone numbers, raw chat-
room URLs, raw on-call rotation entries, and raw calendar URLs MUST
NOT appear; the register carries opaque role / lane / forum refs only.

## 6. Mitigation ledger

The mitigation ledger pins four typed axes plus four bounded
reviewable summaries on every register row.

### 6.1 Compensating-control vocabulary

Closed six-class `compensating_control_class` vocabulary:

| Class | Meaning |
| --- | --- |
| `narrower_scope_until_correction` | Compensated by narrowing the scope (corpus, profile, claim row, marketed surface) until the correction lands. |
| `slower_path_until_correction` | Compensated by routing through a slower path (manual review, secondary verifier, deferred publication) until the correction lands. |
| `manual_review_until_correction` | Compensated by mandatory manual review on every change in the affected surface until the correction lands. |
| `replicated_review_until_correction` | Compensated by replicated review by a second reviewer or a second forum until the correction lands. |
| `alternate_artifact_until_correction` | Compensated by serving an alternate artifact (alternate corpus, alternate evidence packet, alternate claim row) until the correction lands. |
| `none_required_audit_only` | Standing audit-only waiver (e.g. `single_maintainer_backup`); no compensating control is owed. Admissible only when `waiver_kind_class = single_maintainer_backup` or the register-row `register_status_class` is `register_closed_correction_landed`. |

### 6.2 Correction-work-state vocabulary

Closed seven-class `correction_work_state_class` vocabulary:

| Class | Meaning |
| --- | --- |
| `correction_not_started` | No correction work has been scoped. Pairs with `register_status_class = register_active_pending_renewal` or `register_expired_no_decision` only when an owner is named. |
| `correction_in_design` | Correction is in design; the design packet ref MUST be cited in `target_correction_work_summary`. |
| `correction_in_implementation` | Correction implementation is landing. |
| `correction_in_verification` | Correction implementation has merged; verification evidence is being captured. |
| `correction_landed_pending_renewal` | Correction landed; the waiver is held only because the renewal-or-close decision has not been recorded. |
| `correction_landed_renewal_minted` | Correction landed and the matching `waiver_decision_record` has been recorded; the register status MUST be `register_closed_correction_landed`. |
| `correction_descoped_with_decision` | The correction was descoped via a `narrow_claim` or `escalate_correction` decision and the decision record MUST be cited. |

### 6.3 Residual-risk vocabulary

Closed five-class `residual_risk_class` vocabulary:

| Class | Meaning |
| --- | --- |
| `residual_risk_minor` | Residual risk does not affect any marketed claim, support drill, or release train. |
| `residual_risk_moderate` | Residual risk narrows an internal-only surface or a single profile of a marketed claim. |
| `residual_risk_material` | Residual risk narrows a marketed claim, gates a support drill, or holds a release train. |
| `residual_risk_severe` | Residual risk crosses release trains or breaches a release floor; surfaces MUST NOT collapse this into `material`. |
| `residual_risk_unknown_pending_assessment` | Risk classification is pending a decision-forum assessment; the register row MUST cite an open `escalate_correction` or `renew` decision in `linked_decision_refs`. |

### 6.4 Claim-scope-narrowing vocabulary

Closed five-class `claim_scope_narrowing_class` vocabulary:

| Class | Meaning |
| --- | --- |
| `marketed_public_claim_narrowed` | The waiver narrows a marketed public claim row; `linked_claim_or_support_impact_refs` MUST cite at least one `claim_row` and one `public_proof_row`. |
| `marketed_public_claim_held_no_narrowing` | The waiver holds a marketed public claim row but does not narrow it (e.g. expiry inside the marketing window); the row remains marketed and `claim_row` refs are required. |
| `internal_only_no_marketed_claim` | The waiver covers an internal-only surface; no marketed claim is narrowed. |
| `internal_only_narrowed_to_subset` | The waiver narrows an internal-only surface to a subset (e.g. air-gapped lab only) without marketing impact. |
| `claim_scope_unresolved` | The marketed-or-internal axis is pending a decision-forum assessment; the register row MUST cite an open decision. |

A register row whose mitigation ledger cannot be typed on any of the
four axes denies with `mitigation_ledger_class_unresolved` rather than
defaulting.

Schema-enforced pairings (register `allOf` block):

- `compensating_control_class = none_required_audit_only` requires
  `waiver_kind_class = single_maintainer_backup` OR
  `register_status_class = register_closed_correction_landed`.
- `correction_work_state_class = correction_landed_renewal_minted`
  requires `register_status_class = register_closed_correction_landed`
  and a non-empty `decision_history[]`.
- `residual_risk_class = residual_risk_unknown_pending_assessment`
  requires a non-empty `linked_decision_refs[]` array citing at least
  one open `escalate_correction` or `renew` decision.
- `claim_scope_narrowing_class = marketed_public_claim_narrowed`
  requires at least one `claim_row` and one `public_proof_row` in
  `linked_claim_or_support_impact_refs[]`.
- `claim_scope_narrowing_class = claim_scope_unresolved` requires a
  non-empty `linked_decision_refs[]` array.

## 7. Expiry block

The `expiry` block reuses the closed six-class `expiry_proximity_class`
vocabulary frozen on
[`/schemas/governance/waiver_expiry_item.schema.json`](../../schemas/governance/waiver_expiry_item.schema.json):
`expired_past_due`, `due_today_or_within_24h`,
`nearing_expiry_within_seven_days`,
`nearing_expiry_within_thirty_days`,
`not_yet_due_review_window_open`, `no_expiry_pinned`.

`expires_at` is required when `expiry_proximity_class != no_expiry_pinned`.
`no_expiry_pinned` is admissible only for `waiver_kind_class =
single_maintainer_backup` or `waiver_kind_class =
other_named_in_summary` and the `expiry_summary` MUST name why no
expiry is pinned.

`expired_past_due` requires `register_status_class` to be one of
`register_active_pending_renewal`,
`register_renewed_under_new_decision`,
`register_escalated_pending_resolution`,
`register_rejected_no_protection`, or `register_expired_no_decision`.
A register row in `expired_past_due` cannot be filed under
`register_active_within_expiry`.

## 8. Current-evidence-gap vocabulary

Closed seven-class `current_evidence_gap_class` vocabulary:

| Class | Meaning |
| --- | --- |
| `evidence_gap_none_audit_only` | No evidence gap is open; the waiver is audit-only (e.g. `single_maintainer_backup`). |
| `evidence_gap_recapture_pending` | A recurring evidence packet is pending recapture against the affected surface. |
| `evidence_gap_threshold_breach_active` | A protected-fitness-function threshold is currently in breach; the row MUST cite the failing nightly-row ref. |
| `evidence_gap_partial_profile_only` | Evidence has been captured on a subset of the declared profile set; the partial-profile gap is named. |
| `evidence_gap_compatibility_window_open` | A compatibility-window report has not yet been published for the affected surface. |
| `evidence_gap_support_drill_pending` | A recurring support drill has not yet been re-run since the last waiver mint. |
| `evidence_gap_claim_attestation_pending` | The claim attestation packet has not yet been re-attested since the last waiver mint. |

The `linked_evidence_gap_refs[]` array carries opaque refs into the
absent or stale evidence packets the gap names. The array MAY be empty
only when `current_evidence_gap_class = evidence_gap_none_audit_only`.

## 9. Renewal-requested-state vocabulary

Closed nine-class `renewal_requested_state_class` vocabulary:

| Class | Meaning |
| --- | --- |
| `not_yet_due` | Renewal is not yet due; pairs with `expiry_proximity_class` in `{not_yet_due_review_window_open, nearing_expiry_within_thirty_days}`. |
| `renewal_requested_open` | Renewal has been requested but not yet routed to a forum; the request packet ref MUST be cited. |
| `renewal_requested_in_review` | Renewal has been routed to the decision forum and is in review; the gating decision-record ref MUST be cited in `linked_decision_refs[]`. |
| `renewal_resolved_renewed` | Renewal was approved; the matching `waiver_decision_record` of class `renew` MUST appear in `decision_history[]`. |
| `renewal_resolved_closed` | The waiver was closed; the matching `waiver_decision_record` of class `close` MUST appear in `decision_history[]`. |
| `renewal_resolved_narrowed_claim` | The waiver was resolved by narrowing a marketed claim; the matching `waiver_decision_record` of class `narrow_claim` MUST appear in `decision_history[]`. |
| `renewal_resolved_escalated` | The waiver was escalated to correction work; the matching `waiver_decision_record` of class `escalate_correction` MUST appear in `decision_history[]`. |
| `renewal_resolved_rejected` | The waiver was rejected (no further protection granted); the matching `waiver_decision_record` of class `reject` MUST appear in `decision_history[]`. |
| `no_renewal_planned` | No renewal is planned (e.g. `single_maintainer_backup` standing waiver inside its expiry window); the `renewal_requested_summary` MUST name the rationale. |

## 10. Decision-history projection

The `decision_history[]` array carries one entry per resolved
`waiver_decision_record` that cites this register row. Each entry
carries:

- `decision_record_ref` — opaque ref into a
  `waiver_decision_record` on
  [`/schemas/governance/waiver_decision.schema.json`](../../schemas/governance/waiver_decision.schema.json).
- `decision_class` — typed decision class (§15).
- `decided_at` — RFC 3339 UTC timestamp from the decision record.
- `sponsor_or_forum_class` — typed sponsor / forum (§16).
- `decision_summary` — bounded reviewable one-sentence summary copied
  verbatim from the decision record.

The schema requires that every entry's `decision_class` matches the
register row's `renewal_requested_state_class` when the state is one
of the resolved classes (e.g.
`renewal_resolved_renewed` requires at least one entry of class
`renew`).

The schema forbids `decision_history[]` entries that reference a
decision record whose `decided_at` is after the register row's
`evaluated_at`. Reprojection MUST advance `evaluated_at`.

## 11. Register-status vocabulary

Closed nine-class `register_status_class` vocabulary:

| Class | Meaning |
| --- | --- |
| `register_active_within_expiry` | Waiver is active and inside its expiry window; mitigation ledger is in flight. |
| `register_active_pending_renewal` | Waiver is active inside its expiry window but a renewal request is open or in review. |
| `register_renewed_under_new_decision` | A renewal decision was recorded; the row remains active under the new expiry. |
| `register_closed_correction_landed` | Correction work landed; the waiver was closed via a `close` or `escalate_correction` decision. |
| `register_closed_no_renewal` | The waiver was closed via a `close` decision because no renewal was needed; pairs with `correction_work_state_class = correction_landed_renewal_minted` or `correction_descoped_with_decision`. |
| `register_narrowed_claim_published` | The waiver was resolved by narrowing a marketed claim; the public claim manifest renders the narrowing. |
| `register_escalated_pending_resolution` | The waiver was escalated to correction work; resolution is pending. |
| `register_rejected_no_protection` | The waiver was rejected; the affected surface no longer rides on this register row. |
| `register_expired_no_decision` | The waiver expired and no decision has been recorded; the row is an open operational risk and MUST NOT be filtered out. |

A register row whose status cannot be typed denies with
`register_status_class_unresolved` rather than defaulting.

## 12. Repeated-path grouping vocabulary

Closed five-class `repeated_path_grouping_class` vocabulary:

| Class | Meaning |
| --- | --- |
| `not_repeated_first_observation` | First waiver observed on this protected path / requirement row inside the rolling observation window. |
| `repeated_within_milestone` | At least two waivers minted on the same protected path inside the same milestone; `cluster_member_refs` MUST be non-empty. |
| `repeated_across_milestones` | At least two waivers minted on the same protected path across distinct milestones; `cluster_member_refs` MUST be non-empty. |
| `repeated_across_release_trains` | At least two waivers minted on the same protected path across distinct release trains (stable, LTS, preview); `cluster_member_refs` MUST be non-empty and `escalation_path_class` on every linked decision MUST be `escalate_to_release_council` or `escalate_to_shiproom`. |
| `repeated_across_program_lifetime` | The protected path has carried waivers spanning more than the rolling twelve-month window; the cluster MUST be reviewed by the governance council. |

A register row whose grouping cannot be typed denies with
`repeated_path_grouping_class_unresolved` rather than defaulting to
"isolated."

## 13. Linked-claim-or-support-impact vocabulary

Closed seven-class `claim_or_support_impact_kind_class` vocabulary on
each entry of `linked_claim_or_support_impact_refs[]`:

| Class | Meaning |
| --- | --- |
| `claim_row` | Refs a claim manifest row id from `schemas/governance/claim_manifest.schema.json`. |
| `public_proof_row` | Refs a public proof / claim manifest publication row. |
| `about_help_disclosure` | Refs an About / Help disclosure row. |
| `release_notes_row` | Refs a release-notes row id. |
| `support_export_packet` | Refs a support-export packet id. |
| `support_drill_packet` | Refs a recurring support-drill packet id. |
| `recovery_rehearsal_packet` | Refs a recovery rehearsal packet id. |

Every linked entry carries a typed `claim_or_support_impact_kind_class`,
the opaque `linked_ref`, and a bounded reviewable
`impact_summary`. Raw policy bodies, raw waiver justifications, and
raw user identifiers MUST NOT appear.

## 14. Export parity

Every consuming surface that renders the register row MUST render the
same typed fields. The parity floor is enforced by the schema's
`export_fields` block.

Required on every consuming surface:

- `waiver_register_entry_id`, `waiver_record_ref`,
  `waiver_kind_class`, `waiver_authority_class`;
- `affected_target.affected_target_class` plus the matching
  non-null target ref;
- `owner.primary_dri`, `owner.owning_lane`, `owner.decision_forum_ref`;
- `mitigation_ledger.compensating_control_class`,
  `mitigation_ledger.correction_work_state_class`,
  `mitigation_ledger.residual_risk_class`,
  `mitigation_ledger.claim_scope_narrowing_class`;
- `expiry.expiry_proximity_class`, `expiry.expires_at`;
- `current_evidence_gap.current_evidence_gap_class`;
- `renewal_requested_state.renewal_requested_state_class`;
- `register_status_class`;
- `repeated_path_grouping_class`;
- `decision_history[]` (each entry's `decision_record_ref`,
  `decision_class`, `sponsor_or_forum_class`, `decision_summary`);
- `linked_claim_or_support_impact_refs[]` (each entry's
  `claim_or_support_impact_kind_class`, `linked_ref`,
  `impact_summary`);
- `headline_label`, `register_summary`.

Forbidden collapses on dashboard, milestone-scorecard, release-packet,
support-export, governance-packet, claim-manifest, and weekly-
governance-review surfaces:

- Rendering an `expired_past_due` register row as
  `register_active_within_expiry` to keep the dashboard clean.
- Dropping `mitigation_ledger.claim_scope_narrowing_class` so a
  marketed-claim narrowing reads as internal-only.
- Dropping `repeated_path_grouping_class` so a chronic cluster reads
  as an isolated incident.
- Dropping `decision_history[]` so a resolved register row reads as
  open.
- Reformatting the register into a screenshot or a parallel narrative
  status note.
- Filtering out `register_expired_no_decision` rows under an
  "owner inspecting" or "needs review" affordance.

## 15. Decision-class vocabulary

Closed five-class `decision_class` vocabulary on
`waiver_decision_record`:

| Class | Meaning |
| --- | --- |
| `renew` | Renew the waiver against a new expiry envelope; the matching register row's `register_status_class` flips to `register_renewed_under_new_decision`. |
| `close` | Close the waiver because the protected condition no longer applies (correction landed, surface retired, narrowing was made permanent); the register row's `register_status_class` flips to `register_closed_correction_landed` or `register_closed_no_renewal`. |
| `narrow_claim` | Resolve the waiver by narrowing a marketed claim; the public claim manifest, the About / Help disclosure, and the release notes render the narrowing. |
| `escalate_correction` | Escalate to a named correction work item; the gating decision register row MUST be cited and the register row's `register_status_class` flips to `register_escalated_pending_resolution`. |
| `reject` | Reject the renewal; the affected surface no longer rides on this register row and the register row's `register_status_class` flips to `register_rejected_no_protection`. |

A decision whose class cannot be typed denies with
`decision_class_unresolved` rather than defaulting.

## 16. Sponsor-or-forum vocabulary

Closed eleven-class `sponsor_or_forum_class` vocabulary on
`waiver_decision_record`:

| Class | Meaning |
| --- | --- |
| `dri_self_decision` | Decided by the lane DRI under documented self-decision authority (e.g. solo-maintainer posture). |
| `backup_owner_decision` | Decided by the backup owner under documented authority. |
| `architecture_council` | Decided by the architecture council. |
| `performance_council` | Decided by the performance council. |
| `release_council` | Decided by the release council. |
| `security_trust_review` | Decided by the security / trust review forum. |
| `support_council` | Decided by the support council. |
| `governance_council` | Decided by the governance council. |
| `shiproom_executive_scope_review` | Decided at shiproom executive scope review. |
| `executive_steering` | Decided by executive steering for cross-program risk. |
| `decision_forum_unresolved` | Sponsor / forum is unresolved; admissible only on a decision record whose `decision_class = escalate_correction` and whose `linked_escalation_decision_register_row_ref` is non-null. |

## 17. Required-evidence-link vocabulary on the decision record

Closed seven-class `required_evidence_link_class` vocabulary on each
entry of the decision record's `required_evidence_links[]` array:

| Class | Meaning |
| --- | --- |
| `mitigation_landed_evidence` | Evidence that the compensating control is in flight (a manual-review log, a narrower-scope artifact, an alternate-artifact packet). Required on every `renew` decision. |
| `correction_design_evidence` | Evidence packet citing the design / RFC for the correction work. Required on every `escalate_correction` decision. |
| `correction_test_evidence` | Verification evidence that the correction landed. Required on every `close` decision whose underlying `correction_work_state_class = correction_landed_renewal_minted`. |
| `claim_narrowing_disclosure_evidence` | Evidence that the marketed claim narrowing was published (claim manifest row, About / Help disclosure, release notes row). Required on every `narrow_claim` decision. |
| `support_drill_evidence` | Evidence that the affected support drill was re-run. Required on every `renew` decision whose `current_evidence_gap_class` was `evidence_gap_support_drill_pending`. |
| `escalation_packet_evidence` | Evidence packet citing the escalation routing (decision register row, council packet). Required on every `escalate_correction` and `reject` decision. |
| `rejection_rationale_evidence` | Evidence packet citing the rationale and the affected-surface posture after rejection. Required on every `reject` decision. |

Schema-enforced pairings on the decision record (schema `allOf`
block):

- `decision_class = renew` requires at least one entry of class
  `mitigation_landed_evidence`.
- `decision_class = close` requires at least one entry of class
  `correction_test_evidence` OR `claim_narrowing_disclosure_evidence`.
- `decision_class = narrow_claim` requires at least one entry of
  class `claim_narrowing_disclosure_evidence`.
- `decision_class = escalate_correction` requires at least one entry
  each of `correction_design_evidence` and `escalation_packet_evidence`.
- `decision_class = reject` requires at least one entry each of
  `escalation_packet_evidence` and `rejection_rationale_evidence`.

## 18. Authoring rules

When a waiver is minted, renewed, narrowed, escalated, rejected, or
closed:

1. Mint or update a `waiver_register_entry_record` and link it from
   every requirement-status snapshot row whose state is gated by the
   waiver and from every nightly-row that triggered or is gated by it.
2. Resolve `affected_target` against the requirement register, the
   protected fitness-function catalog, the protected-metrics register,
   the claim manifest, the release-train index, the milestone
   scorecard, the compatibility-surface inventory, the retention /
   deletion matrix, or the export pipeline registry.
3. Resolve `owner` against the ownership matrix.
4. Resolve `mitigation_ledger` against the compensating-control,
   correction-work-state, residual-risk, and claim-scope-narrowing
   vocabularies.
5. Resolve `expiry` against the live evaluation time; the proximity
   class is recomputed every time the register is rendered.
6. Resolve `current_evidence_gap` against the linked evidence packet
   refs.
7. Resolve `renewal_requested_state` against the open / in-review
   decision rows.
8. Append every closed `waiver_decision_record` to `decision_history[]`
   in chronological order; the matching register status flips per §11.
9. Resolve `repeated_path_grouping_class` against the rolling
   observation window.
10. Resolve `linked_claim_or_support_impact_refs[]` against the claim
    manifest, the public proof index, the About / Help registry, the
    release-notes seed, the support-export catalog, and the recovery
    rehearsal index.
11. Surfaces render `register_summary` and `decision_summary` verbatim;
    they MUST NOT substitute free-text fear copy.

A reprojection is required when:

- The underlying waiver record's `expires_at` changes.
- A `waiver_decision_record` is recorded against the register row.
- A linked evidence packet's freshness flips.
- A linked claim manifest row narrows or republishes.
- A linked nightly-row state flips into or out of `waived_failure` /
  `waiver_expired_failure`.

The reprojection MUST advance `evaluated_at` and recompute
`expiry_proximity_class`, `register_status_class`,
`repeated_path_grouping_class`, and the `decision_history[]` entries.

## 19. Out of scope

This contract does not implement:

- An approval-workflow runner, an executive-routing pipeline, or a
  renewal-reminder scheduler.
- The council decision rights themselves (those live in
  [`/docs/governance/decision_rights_and_signoff_matrix.md`](./decision_rights_and_signoff_matrix.md)
  and the signoff matrix).
- The underlying waiver records (those live in
  [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  and on
  [`/schemas/governance/waiver_expiry.schema.json`](../../schemas/governance/waiver_expiry.schema.json)).
- The dashboard-side waiver-expiry queue projection (that lives on
  [`/schemas/governance/waiver_expiry_item.schema.json`](../../schemas/governance/waiver_expiry_item.schema.json)
  and
  [`/docs/governance/nightly_report_and_waiver_queue_contract.md`](./nightly_report_and_waiver_queue_contract.md)).

This contract is the projection vocabulary that the underlying waiver
records flow through when they reach the dashboard, the milestone
scorecard, the weekly governance review, the release-evidence
shiproom packet, the support bundle, the public claim manifest, and
the governance packet — and the typed decision-object family that
replaces unattributed renewal-or-close notes.
