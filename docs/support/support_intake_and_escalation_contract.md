# Support-intake, escalation-packet, and field-readiness expectation contract

This document freezes the support-intake contract Aureline uses when a
user reports a problem from any in-product or headless support surface.
It turns supportability into a first-class product contract by pinning
how problems are classified, what evidence is safe to include, which
repairs are allowed (and which are not), and how a local-only path
stays viable without blind data sharing.

If this document, the
[`scenario_picker.schema.json`](../../schemas/support/scenario_picker.schema.json)
schema, the
[`escalation_packet.schema.json`](../../schemas/support/escalation_packet.schema.json)
schema, and the
[`scenario_cases/`](../../fixtures/support/scenario_cases/) corpus
disagree, the frozen support-bundle contract, the object-handoff
contract, the recovery-ladder contract, the repair-transaction
contract, the project-doctor scenario matrix, and the record-class
registry win for tooling and this packet plus its companion artifacts
update in the same change.

## Companion artifacts

- [`/schemas/support/scenario_picker.schema.json`](../../schemas/support/scenario_picker.schema.json)
  — boundary schema for `support_scenario_picker_record`,
  `support_capability_card_record`, and
  `support_scenario_intake_index_record`. Defines the closed scenario
  family vocabulary, the four required intake surfaces, the seven
  Support Center capability cards, the closed approved/forbidden
  fix vocabularies, and the per-environment parity row shape.
- [`/schemas/support/escalation_packet.schema.json`](../../schemas/support/escalation_packet.schema.json)
  — boundary schema for `support_escalation_packet_record` and
  `support_escalation_packet_seed_case_record`. Defines the
  intake-emitted escalation packet that preserves stable scenario
  family, finding ids, build/profile identity, deployment class,
  evidence ids, reproduction steps, recommended-repair review rows,
  per-environment parity, and delivery posture (with `local_only_review`
  pinned to primary equal prominence).
- [`/fixtures/support/scenario_cases/`](../../fixtures/support/scenario_cases/)
  — one seed case per required scenario family. Cases project onto
  `support_escalation_packet_seed_case_record`, bind 1:1 to a row in
  `fixtures/support/scenario_matrix.yaml`, and resolve to a case in
  `fixtures/support/escalation_packet_completeness_cases/`.
- [`/fixtures/support/scenario_matrix.yaml`](../../fixtures/support/scenario_matrix.yaml)
  — Project Doctor scenario matrix. Every required finding code on a
  picker row MUST exist here.
- [`/fixtures/support/escalation_packet_completeness_cases/`](../../fixtures/support/escalation_packet_completeness_cases/)
  — exact-build, route, redaction, and provenance completeness rules
  the escalation packet projects onto.
- [`/docs/support/project_doctor_packet.md`](./project_doctor_packet.md),
  [`/schemas/support/doctor_finding.schema.json`](../../schemas/support/doctor_finding.schema.json),
  and
  [`/schemas/support/probe_catalog_entry.schema.json`](../../schemas/support/probe_catalog_entry.schema.json)
  — Doctor finding, probe-catalog, repair-class, recovery-rung,
  no-touch boundary, support-context, and unsupported-state vocabulary
  the picker reuses verbatim rather than re-minting.
- [`/docs/support/repair_transaction_contract.md`](./repair_transaction_contract.md),
  [`/schemas/support/repair_transaction.schema.json`](../../schemas/support/repair_transaction.schema.json),
  [`/schemas/support/repair_preview.schema.json`](../../schemas/support/repair_preview.schema.json),
  and
  [`/schemas/support/repair_outcome.schema.json`](../../schemas/support/repair_outcome.schema.json)
  — repair-class family, transaction reversal class, apply-mode class,
  forbidden-action class, and preview/apply/rollback grammar the picker
  references when an approved repair is offered.
- [`/docs/support/recovery_ladder_packet.md`](./recovery_ladder_packet.md)
  and
  [`/schemas/support/recovery_action.schema.json`](../../schemas/support/recovery_action.schema.json)
  — recovery-rung vocabulary every picker row defaults onto.
- [`/docs/support/support_bundle_contract.md`](./support_bundle_contract.md),
  [`/docs/support/support_bundle_preview_contract.md`](./support_bundle_preview_contract.md),
  [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json),
  [`/docs/support/diagnostic_artifact_matrix.md`](./diagnostic_artifact_matrix.md),
  and
  [`/schemas/support/support_pack_item.schema.json`](../../schemas/support/support_pack_item.schema.json)
  — support-bundle redaction posture, item-level inclusion classes,
  and high-risk consent vocabulary the picker reuses for evidence
  selection.
- [`/docs/support/object_handoff_packet.md`](./object_handoff_packet.md)
  and
  [`/schemas/support/object_handoff_packet.schema.json`](../../schemas/support/object_handoff_packet.schema.json)
  — object handoff packet the escalation packet composes with by
  stable ref. Every escalation packet resolves to one handoff packet
  rather than re-flattening route, transport, and destination
  vocabulary.
- [`/docs/support/runbook_execution_contract.md`](./runbook_execution_contract.md),
  [`/schemas/support/runbook_packet.schema.json`](../../schemas/support/runbook_packet.schema.json),
  and
  [`/schemas/support/runbook_step_result.schema.json`](../../schemas/support/runbook_step_result.schema.json)
  — guided-repair runbook contract the `guided_repair` capability
  card resolves to.
- [`/docs/support/support_center_concept.md`](./support_center_concept.md)
  — product-facing Support Center concept that names the seven
  capability cards (Doctor, safe mode, bisect, support bundle, crash
  triage, guided repair, issue escalation) the picker MUST be able to
  route from a symptom to.
- [`/docs/support/support_center_information_architecture.md`](./support_center_information_architecture.md),
  [`/schemas/support/support_center_capability_card.schema.json`](../../schemas/support/support_center_capability_card.schema.json),
  [`/artifacts/support/support_center_routes.yaml`](../../artifacts/support/support_center_routes.yaml),
  and
  [`/fixtures/support/support_center_cases/`](../../fixtures/support/support_center_cases/)
  — Support Center information architecture that adds two IA-only
  modules (`advisory_or_incident_history`, `field_diagnostics`) on
  top of the seven intake-bound capabilities, pins the closed six
  symptom-surface classes and five deployment-context classes, and
  binds every route to a closed list of preserved evidence-id
  classes so a later escalation packet can reconstruct the case
  without re-asking the user. The intake-bound
  `support_capability_class` vocabulary in this packet remains
  source of truth for the picker; the IA aligns each intake-bound
  capability with one IA module via
  `aligned_intake_capability_class`.
- [`/schemas/support/support_packet_index.schema.json`](../../schemas/support/support_packet_index.schema.json)
  — support-packet family registry; the escalation packet belongs to
  the `object_issue_handoff` family by way of the handoff packet it
  composes with.

## Normative sources projected here

- `.t2/docs/Aureline_PRD.md` §10.15 (diagnostics), §10.22 (support
  export), and §10.23 (recovery ladder).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §8.10 (fault
  domain and supervisor), §24.2.2 (recovery rungs), §24.2.3 (checkpoint
  and reversal), §24.4 (repair preview), §24.5 (support export), and
  Appendix I (support packet posture).
- `.t2/docs/Aureline_Technical_Design_Document.md` offline handoff,
  browser handoff, and support-export passages.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §22.20 (Support Center)
  and §23.26 (Doctor surface).
- `.t2/docs/Aureline_Milestones_Document.md` §3.20 (supportability),
  §3.21 (evidence), and §7.4 (blocked-user recovery).

If this document disagrees with those sources, those sources win and
this packet plus its companion artifacts update in the same change.

## Why this exists

The repository already had adjacent contracts for support bundles,
object handoff, recovery ladders, repair transactions, Project Doctor
findings, and runbook execution. What stayed implicit was the intake
the user actually meets when they report a problem: how the symptom is
classified into a scenario family, which repairs the picker is allowed
to surface (and which it MUST refuse), how the escalation packet
preserves the case so the next reviewer does not start from a blank
page, and how the local-only path stays at primary prominence so the
user is not forced into upload to get help.

Without this contract:

- the picker would invent surface-local symptom taxonomies that drift
  from Doctor finding codes;
- "Fix" affordances would creep past the closed forbidden-action set
  the repair-transaction contract pins;
- the local-only path would drift below the upload or vendor-case
  paths in prominence;
- escalation packets would re-flatten route, build, and redaction
  fields the handoff packet already preserves; and
- desktop would be the implicit "happy path" with CLI/headless,
  managed, and offline parity left to fall out of band.

This contract closes those gaps in one packet family pair: the
scenario picker (reviewable picker rows + capability cards + index)
and the escalation packet (reviewable packet shape + per-family seed
cases).

## Required intake surfaces (closed at four)

Every conforming support-intake flow exposes the same four surfaces.
Hiding any surface — for example, jumping straight from picker to
upload without a review surface — is non-conforming.

| Surface (`support_intake_surface_class`) | Job |
|---|---|
| `scenario_picker_surface` | Classify the symptom into one closed `scenario_family_class`; offer the first actionable diagnosis target; show approved repairs and forbidden fixes. |
| `issue_report_builder_surface` | Build the escalation packet draft from picker output; bind finding codes, evidence refs, reproduction steps, and recommended-repair reviews. |
| `escalation_packet_review_surface` | Preview the packet before delivery; show what is included, what is excluded, what is redacted, and which delivery paths are available at what prominence. |
| `handoff_timeline_surface` | Show the user where the packet has gone (saved local-only, ready for browser handoff, attached by reference, submitted out-of-band, local review complete) without inventing surface-local copy. |

## Scenario families (closed at six)

The picker's `scenario_family_class` vocabulary is closed at this
revision. Adding a family is breaking and requires a new decision row
in `artifacts/governance/decision_index.yaml`.

| Family | Default first actionable diagnosis target | Default recovery rung | Default redaction |
|---|---|---|---|
| `execution_context_mismatch` | `doctor_finding_record` | `typed_repair_flow` | `metadata_only_default` |
| `trust_policy_identity_approval_block` | `doctor_finding_record` | `restricted_reopen` | `support_bundle_by_reference` |
| `network_ca_proxy_mirror_failure` | `doctor_finding_record` | `typed_repair_flow` | `metadata_only_default` |
| `extension_or_host_regression` | `doctor_finding_record` | `extension_quarantine` | `metadata_only_default` |
| `state_corruption_schema_drift_low_disk_recovery` | `doctor_finding_record` | `cache_reset_candidate` | `metadata_only_default` |
| `remote_route_collaboration_mismatch` | `doctor_finding_record` | `restricted_reopen` | `support_bundle_by_reference` |

Per-family rules are pinned in
[`scenario_cases/`](../../fixtures/support/scenario_cases/). Every
case row binds one
[`scenario_matrix.yaml`](../../fixtures/support/scenario_matrix.yaml)
row by `scenario_matrix_row_ref` so support review can pivot from
intake → scenario family → matrix row → escalation completeness case
→ exported handoff packet without restating the user's case.

## Per-scenario picker contract

For every scenario family the picker row MUST declare:

- `first_actionable_diagnosis_target_class` — the record kind the
  picker hands the user to before any repair is proposed
  (`doctor_finding_record`, `support_bundle_preview_item`,
  `recovery_action_record`, `repair_preview_record`,
  `object_handoff_packet_record`, `trace_capture_request`, or
  `crash_envelope_ref`).
- `required_finding_codes` — at least one `doctor.finding.*` code from
  `scenario_matrix.yaml` the row may surface.
- `approved_repairs[]` — closed list of `approved_repair_class` tokens
  the row may propose. Each row MUST explain
  `why_safe_summary`, `what_may_change_summary`,
  `if_declined_rollback_summary`, and `if_declined_evidence_summary`
  so users do not have to guess what they keep if they decline.
- `forbidden_fix_classes[]` — closed list of actions the picker MUST
  NOT propose. By rule the list MUST include at least
  `widen_workspace_trust`, `publish_route`,
  `mutate_user_authored_files`, `read_or_rotate_credentials`, and
  `factory_reset_as_first_offer`.
- `default_data_class_boundary` — the diagnostic data ceiling
  (`metadata_only`, `environment_adjacent`, `code_adjacent`, or
  `high_risk`). Widening past the default requires an explicit user
  consent marker.
- `default_redaction_choice_class` — re-exported from the handoff
  vocabulary.
- `default_recovery_rung_class` — re-exported from the support-bundle
  vocabulary.
- `minimum_escalation_packet` — names the
  `escalation_packet_completeness_cases/*.yaml` case the picker
  projects onto, the minimum required finding codes, the default
  redaction, and the minimum required provenance bindings the exported
  packet MUST carry.
- `delivery_paths[]` — every row MUST list `local_only_review` at
  `primary_equal_prominence` so the local-only path is presented at
  the same prominence as `vendor_case_handoff` and
  `user_initiated_upload`. Hiding the local-only path is
  non-conforming.
- `capability_card_links[]` — at least the `doctor`, `support_bundle`,
  and `issue_escalation` cards MUST be linked. Other cards link when
  applicable.
- `context_parity_rows` — exactly one row per `support_context_class`
  (`desktop`, `cli_headless`, `remote_managed`, `offline_local`).

### Approved repair classes (closed)

Re-exported from
[`schemas/support/doctor_finding.schema.json`](../../schemas/support/doctor_finding.schema.json)
`repair_class`. The picker MAY only propose tokens from this set:

- `observe_only_no_repair`
- `disposable_state_rebuild`
- `extension_isolation`
- `extension_rollback_reinstall`
- `execution_context_reresolve`
- `remote_runtime_repair`
- `policy_entitlement_refresh`
- `guided_export_escalation`
- `support_bundle_export`
- `trace_capture`

### Forbidden `fix` classes (closed)

Re-exported from
[`schemas/support/repair_transaction.schema.json`](../../schemas/support/repair_transaction.schema.json)
`forbidden_action_class` with two intake-specific additions:

- `widen_workspace_trust`
- `publish_route`
- `run_repo_hook_silently`
- `silent_extension_reinstall`
- `silent_helper_rebind`
- `mutate_managed_policy`
- `mutate_user_authored_files`
- `read_or_rotate_credentials`
- `auto_retarget_without_user`
- `mutate_authoritative_profile_store`
- `embed_raw_secret_in_export`
- `auto_widen_redaction_choice`
- `auto_upload_without_user_review`
- `factory_reset_as_first_offer`

`auto_upload_without_user_review` ensures the picker cannot skip the
escalation-packet review surface. `factory_reset_as_first_offer`
ensures the picker offers the narrowest safe repair before any
destructive reset.

## Recommended-repair review rule

Every approved repair the picker surfaces MUST carry four reviewable
sentences (the schema enforces this):

1. **`why_safe_summary`** — why this repair is safe for this scenario
   family, in one sentence.
2. **`what_may_change_summary`** — the impacted state classes apply
   may touch, in one sentence.
3. **`if_declined_rollback_summary`** — the rollback path that remains
   if the user declines the repair, in one sentence.
4. **`if_declined_evidence_summary`** — the evidence path the user
   keeps if they decline (support bundle preview, escalation packet,
   local-only review), in one sentence.

These sentences are reviewer-safe: the Support Center, Doctor surface,
and CLI/headless picker MAY render them verbatim. They replace
free-text "Fix" copy.

## Escalation-packet contract

Every export the picker emits projects onto a
`support_escalation_packet_record` (or its
`support_escalation_packet_seed_case_record` companion). The packet:

- preserves stable `scenario_family_class` and one or more
  `finding_codes`;
- preserves the build/profile identity block
  (`exact_build_identity_ref`, `install_mode_class`,
  `install_channel_class`, `deployment_profile_class`,
  `managed_admin_required`);
- preserves the evidence-reference block
  (`selected_evidence_refs`, `support_bundle_refs`,
  `incident_workspace_packet_refs`, `repair_transaction_refs`,
  `checkpoint_refs`, `trace_capture_request_refs`,
  `crash_envelope_refs`, `withheld_artifact_refs`,
  `default_data_class_boundary`, `redaction_choice_class`, and any
  `redaction_widening_consent_marker`);
- preserves typed `reproduction_steps[]` rows drawn from the closed
  `reproduction_step_class` vocabulary so the next reviewer does not
  receive prose-only repro instructions;
- preserves `recommended_repair_reviews[]` so the four declined-path
  sentences travel with the packet;
- preserves the delivery posture (`selected_delivery_path_class`,
  `delivery_state_class`, `local_only_path_available`,
  `vendor_case_path_available`, `user_upload_path_available`,
  `managed_admin_handoff_required`,
  `private_security_channel_required`);
- preserves exactly one `context_parity_row` per
  `support_context_class` so capability gaps per environment are
  explicit;
- preserves `linkage` to the picker row, the handoff packet, the
  scenario matrix row, the escalation completeness case, and the
  optional support-bundle, incident, recovery, and repair refs;
- preserves the `completeness_outcome` (`complete`,
  `complete_with_typed_unknowns`, or `incomplete_refused_export`) and
  the typed unknown field list when applicable.

Rule: every escalation packet MUST resolve to one
`object_handoff_packet_ref` and one
`escalation_packet_completeness_case_ref`. The escalation packet does
not re-flatten route, transport, or destination vocabulary; the
handoff packet remains the source of truth for those layers.

Rule: a packet whose `delivery_posture.local_only_path_available =
false` is non-conforming. Intake refuses export rather than ship a
hidden default upload.

Rule: a packet whose `redaction_choice_class` widens past the picker
row's `default_redaction_choice_class` MUST carry a non-null
`redaction_widening_consent_marker`. Otherwise the
`completeness_outcome` MUST be `incomplete_refused_export`.

## Support Center capability-card linkage

Every conforming intake flow MUST be able to route from a symptom to
the seven Support Center capability cards. Cards are
`support_capability_card_record` instances:

| Capability (`support_capability_class`) | Primary target record kind | Schema |
|---|---|---|
| `doctor` | `doctor_finding_record` | `schemas/support/doctor_finding.schema.json` |
| `safe_mode` | `recovery_action_record` (rung `safe_mode`) | `schemas/support/recovery_action.schema.json` |
| `extension_bisect` | `recovery_action_record` (rung `extension_bisect`) | `schemas/support/recovery_action.schema.json` |
| `support_bundle` | `support_bundle_record` / `support_bundle_preview_item` | `schemas/support/support_bundle.schema.json`, `schemas/support/support_bundle_preview_item.schema.json` |
| `crash_triage` | `crash_envelope_ref` (with symbolication) | re-exported from `docs/support/exact_build_symbolication_smoke.md` |
| `guided_repair` | `runbook_packet` / `repair_preview_record` | `schemas/support/runbook_packet.schema.json`, `schemas/support/repair_preview.schema.json` |
| `issue_escalation` | `support_escalation_packet_record` | `schemas/support/escalation_packet.schema.json` |

Every card carries the same four-row `context_parity_rows` block as a
picker row. A card whose desktop row is `available` and whose
remaining rows are `unavailable` MUST say so explicitly; the schema
forbids omitting any context.

## Field-readiness parity (closed at four contexts)

Every picker row, every capability card, and every escalation packet
case MUST carry exactly one parity row per `support_context_class`:

| Context | Default expectation |
|---|---|
| `desktop` | Reference surface; full picker, review, and timeline available. |
| `cli_headless` | Same picker rows and packet preview, emitted to a local file. |
| `remote_managed` | Picker available; reapproval / vendor-case may need managed-admin sign-off; never auto-retargets. |
| `offline_local` | Picker available; repair / vendor-case / upload paths narrow gracefully; local-only review is always available. |

Each row carries an `availability_class` token from
[`schemas/support/doctor_finding.schema.json`](../../schemas/support/doctor_finding.schema.json)
(`available`, `available_read_only`, `available_local_only`,
`limited_available`, `read_only`, `unavailable`), a
`narrowed_when_summary`, and an `alternate_path_summary`. A row that
declares `unavailable` MUST name an alternate path (for example,
`local_only_review` or a deferred handoff via the `handoff_timeline_surface`).

Rule: a picker row, capability card, or packet case that implies a
desktop-only happy path — for example, by setting
`cli_headless.availability_class = unavailable` without naming a CLI
alternate path — is non-conforming.

## Summary

This seed packet freezes:

- one `support_scenario_picker_record` shape per scenario family,
  pinning first actionable diagnosis target, approved repair classes,
  forbidden fix classes, default data-class boundary, default
  redaction, default recovery rung, minimum escalation packet refs,
  delivery-path prominence, capability-card linkage, and per-context
  parity;
- one `support_capability_card_record` shape per Support Center
  capability so the picker can route from symptom to allowed evidence
  without inventing per-surface card copy;
- one `support_scenario_intake_index_record` shape that pins the four
  required intake surfaces, the six scenario family summary rows, the
  six picker row refs, the seven capability card refs, the
  delivery-path invariants (`local_only_review` at
  `primary_equal_prominence`), and the governance bindings;
- one `support_escalation_packet_record` shape every export emits,
  composing with one `object_handoff_packet_record` rather than
  re-flattening route or destination vocabulary;
- one closed `scenario_family_class` vocabulary covering execution-
  context mismatch, trust/policy/identity/approval block, network/CA/
  proxy/mirror failure, extension or host regression, state corruption
  / schema drift / low-disk recovery, and remote/route/collaboration
  mismatch;
- one closed `support_intake_surface_class` set of four required
  surfaces (picker, builder, review, timeline);
- one closed `support_capability_class` set of seven capability cards
  (`doctor`, `safe_mode`, `extension_bisect`, `support_bundle`,
  `crash_triage`, `guided_repair`, `issue_escalation`);
- one closed `delivery_path_class` vocabulary plus the
  `delivery_path_prominence_class` pin that binds `local_only_review`
  at `primary_equal_prominence`;
- one closed `forbidden_fix_class` vocabulary that adds
  `auto_upload_without_user_review` and
  `factory_reset_as_first_offer` to the repair-transaction
  forbidden-action set;
- one closed `reproduction_step_class` vocabulary so reproduction
  steps preserve typed evidence refs instead of free-text repro
  prose; and
- one seed case per scenario family — six in total — under
  [`fixtures/support/scenario_cases/`](../../fixtures/support/scenario_cases/),
  each binding 1:1 to a row in
  `fixtures/support/scenario_matrix.yaml` and resolving to a case in
  `fixtures/support/escalation_packet_completeness_cases/`.

It does not claim a live intake runtime, picker engine, escalation
exporter, or hosted support portal is wired up. It claims only that
the support-intake decision objects, the recommended-repair review
rule, the local-only delivery prominence pin, and the field-readiness
parity contract now exist in one reviewable form and reuse the frozen
support vocabulary already landed in this repository.

## Acceptance check

The four acceptance rules from the milestone spec resolve to:

| Acceptance rule | Where enforced |
|---|---|
| Local-only path is equal in prominence to upload or vendor-case paths. | `delivery_path_prominence_class` pin in `scenario_picker.schema.json`; every picker row and every escalation packet case lists `local_only_review` at `primary_equal_prominence`; the schema rejects `local_only_path_available = false` on the escalation packet. |
| Recommended repairs explain why they are safe, what they may change, and what rollback / evidence path remains if declined. | `recommended_repair_review_row` and `approved_repair_row` shapes; both require `why_safe_summary`, `what_may_change_summary`, `if_declined_rollback_summary`, and `if_declined_evidence_summary` (plus `declined_*` variants on the picker row). |
| Escalation packets preserve scenario family, finding ids, build/profile identity, deployment class, evidence ids, and reproduction steps so users do not have to restate their case after handoff. | Required blocks on `support_escalation_packet_record`: `scenario_family_class`, `finding_codes`, `build_profile_identity`, `evidence_references`, `reproduction_steps`, `recommended_repair_reviews`, `linkage` (which binds the handoff packet, scenario matrix row, and completeness case). |
| Support intake declares capability parity or gaps per environment instead of implying a desktop-only happy path. | `context_parity_row_array` is fixed at four rows on every picker row, capability card, and packet case (one per `support_context_class`); each row names `availability_class`, `narrowed_when_summary`, and `alternate_path_summary`. |

## What this seed does not promise

- No live picker engine, intake runtime, escalation exporter,
  capability-card host, or runbook executor is wired up. The picker
  rows, capability cards, and escalation packet shapes are reviewable
  objects only.
- No live network submission or vendor-case ticket integration is in
  scope.
- No new schema changes to
  [`object_handoff_packet.schema.json`](../../schemas/support/object_handoff_packet.schema.json),
  [`support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json),
  [`recovery_action.schema.json`](../../schemas/support/recovery_action.schema.json),
  [`repair_transaction.schema.json`](../../schemas/support/repair_transaction.schema.json),
  or
  [`support_packet_index.schema.json`](../../schemas/support/support_packet_index.schema.json)
  are required. This packet projects onto those vocabularies and adds
  `support_scenario_picker_record`, `support_capability_card_record`,
  `support_scenario_intake_index_record`,
  `support_escalation_packet_record`, and
  `support_escalation_packet_seed_case_record` in two new schemas.
- Adding a finding code or capability-card row under an existing
  scenario family or capability is additive-minor provided the new
  row reuses existing closed vocabularies. Repurposing a
  `scenario_family_class`, `support_capability_class`,
  `support_intake_surface_class`, `delivery_path_class`,
  `delivery_path_prominence_class`, `approved_repair_class`,
  `forbidden_fix_class`, or `reproduction_step_class` token is
  breaking and requires a new decision row in
  `artifacts/governance/decision_index.yaml`.
