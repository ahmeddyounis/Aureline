# M5 Support-Intake / Escalation Component Matrix (M05-900)

Frozen contract for Aureline's reusable support-intake and escalation components across
every claimed M5 supportability and escalation surface. The authoritative gate is the
Rust validator in
`crates/aureline-support/src/freeze_the_m5_support_scenario_picker_row_issue_report_builder_step_escalation_packet_summary_handoff_timeline_row_and_unsafe_fix_blocked_note_component_matrix/`.
This doc describes the shape; the code and the checked-in support export are the truth.

## Why this lane exists

The current sheet already covers Project Doctor probes / finding codes, recovery
ladders, crash forensics, repair transactions, evidence chronology, safe mode / bisect,
and support-bundle / export redaction. What it still lacked was governed truth for the
reusable **components users actually interact with when they start diagnosis, review a
suggested repair, build a report, or escalate a case**: the support-scenario picker
rows, the issue-report builder steps, the escalation-packet summaries, the
handoff-timeline rows, and the unsafe-fix blocked notes. M5 cannot honestly claim
self-diagnosis and safe escalation if users still have to assemble cases from generic
logs, guess which repair is approved, or lose scenario / finding / packet lineage
between local diagnosis and human handoff.

## Component families (5)

| Family | Owns (family-specific vocabulary) |
| --- | --- |
| `support_scenario_picker_row` | scenario families, incident scopes, Doctor finding families |
| `issue_report_builder_step` | report-builder step kinds, evidence classes |
| `escalation_packet_summary` | packet destinations, redaction states (+ case dispositions) |
| `handoff_timeline_row` | handoff stages, next human steps |
| `unsafe_fix_blocked_note` | unsafe-fix block reasons, approved repair classes (+ case dispositions) |

Every row also declares the shared vocabularies: surface families, deployment lines,
consumer surfaces, accessibility routes, required labels, a qualification class, and
downgrade triggers.

## Stable field vocabulary

- **Scenario / scope** — `scenario_families` (crash_recovery, performance_health,
  extension_conflict, data_integrity, connectivity_sync, uncategorized_scenario) and
  `incident_scopes` (single_file, workspace, account, device_host, remote_service,
  unknown_scope), surfaced by the `scenario_and_scope` required label.
- **Doctor finding lineage** — `doctor_finding_families` (startup_health,
  index_integrity, storage_pressure, extension_fault, sync_connectivity,
  uncategorized_finding). Scenario vocabulary is bound to the Doctor finding families
  rather than reinvented.
- **Evidence classes** — `builder_step_kinds` (choose_scenario, describe_symptom,
  attach_evidence, review_redaction, confirm_scope, submit_or_export) and
  `evidence_classes` (doctor_finding, crash_forensics, repair_transaction,
  activity_timeline, environment_snapshot, user_note). Selected and omitted evidence is
  always explicit.
- **Packet destination / redaction** — `packet_destinations` (local_only_bundle,
  self_serve_export, vendor_support_case, enterprise_admin, community_forum,
  blocked_destination) and `redaction_states` (full_metadata, paths_redacted,
  bodies_omitted, credentials_scrubbed, policy_restricted, export_blocked). A local-only
  bundle is never mislabelled as a shared case; a redacted packet is never shown as a
  full export.
- **Handoff stage / next step** — `handoff_stages` (diagnosis_started, repair_suggested,
  repair_attempted, case_built, handed_off, awaiting_human) and `next_human_steps`
  (run_doctor, apply_approved_repair, gather_more_evidence, export_bundle,
  contact_vendor, wait_for_response). Scenario / finding / packet lineage is never lost
  between local diagnosis and human handoff.
- **Unsafe-fix block reason / approved repair** — `unsafe_fix_block_reasons`
  (approval_required, irreversible_change, out_of_scope_repair, insufficient_evidence,
  policy_blocked, unsupported_scenario) and `approved_repair_classes` (cache_rebuild,
  index_repair, settings_repair, state_migration, targeted_reset, no_safe_repair). An
  unsafe fix always names why it is blocked and which repair class is approved instead.
- **Case disposition (shared)** — `case_dispositions` (local_only, vendor_case,
  uncategorized, unsafe_fix_blocked, resolved_locally). No surface invents an alternate
  label for these states.

## Hard invariants (every row)

- `masks_scenario_or_scope` — MUST be false.
- `hides_unsafe_fix_block_reason` — MUST be false.
- `invents_alternate_state_label` — MUST be false.
- `bypasses_escalation_packet_minimums` — MUST be false.

Any true value raises `component_invariant_violated`.

## Consumer obligations

Project Doctor, support-center, report-builder, escalation-desk, recovery-center, Help,
and admin surfaces inherit the same support-intake / escalation component grammar. Every
surface projects from this one canonical packet; no surface invents a second scenario,
evidence, destination, or repair vocabulary. Support/export, CLI/headless, and
help/admin consumers read the same stable field names and downgrade states.

## Bound source contracts

The picker row binds scenario vocabulary to the Doctor finding families
(`schemas/support/scenario_picker.schema.json`,
`schemas/support/doctor_finding.schema.json`); the escalation-packet summary binds the
escalation minimums and redaction profile
(`schemas/support/escalation_packet.schema.json`,
`schemas/support/export_redaction_profile.schema.json`); and the unsafe-fix blocked note
binds the approved repair classes (`schemas/support/recovery_action.schema.json`).

## Artifacts

- Boundary schema: `schemas/ui/m5-support-intake-escalation-component-matrix.schema.json`
- Design matrix report: `artifacts/design/m5-support-intake-escalation-component-matrix.md`
- Release proof (canonical support export + CSV):
  `artifacts/release/m5-support-intake-escalation-proof/`
- Narrowed fixtures: `fixtures/ui/m5-support-intake-escalation-components/`

## Regenerating the checked-in artifacts

```sh
cargo run -q -p aureline-support --bin aureline_support_support_intake_escalation_component_matrix -- support-export > artifacts/release/m5-support-intake-escalation-proof/support_export.json
cargo run -q -p aureline-support --bin aureline_support_support_intake_escalation_component_matrix -- csv > artifacts/release/m5-support-intake-escalation-proof/matrix.csv
cargo run -q -p aureline-support --bin aureline_support_support_intake_escalation_component_matrix -- report > artifacts/design/m5-support-intake-escalation-component-matrix.md
cargo run -q -p aureline-support --bin aureline_support_support_intake_escalation_component_matrix -- validate
```
