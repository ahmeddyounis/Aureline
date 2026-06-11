# Fixtures: Project Doctor explainability panes and cross-surface parity

This directory contains fixture metadata for the
`project_doctor_explainability_parity` packet.

The canonical full corpus is checked in at:

`artifacts/doctor/m5/project-doctor-explainability-parity.json`

## Coverage

- Every pane carries a `probe_id` and `probe_version`, at least one
  `evidence_ref`, and a `finding_code` beginning with `doctor.finding.`, so
  support and release packets can reference probe versions and finding ids rather
  than screenshots or prose.
- Repair availability covers every class: `available`, `not_applicable_healthy`,
  `blocked_unsupported_context`, `blocked_managed_policy`,
  `blocked_partial_evidence`, and `blocked_reversal_unproven`. An `available`
  pane always names a `repair.`-prefixed candidate and a real reversal class with
  no block reason; a `blocked_*` pane always names a specific, non-generic block
  reason with no candidate and `not_applicable` reversal.
- Reversal class covers `reversible_transactional`, `reversible_with_snapshot`,
  `irreversible_guarded`, and `not_applicable`.
- Diagnosis state covers `healthy`, `partial`, `stale`, `unsupported`,
  `policy_blocked`, and `target_mismatch`; each maps to its canonical CLI exit
  class (`ok_healthy`/`advisory_findings`/`blocked`/`unsupported`/`policy_denied`)
  and stable exit code.
- Every pane renders on the four core surfaces (`desktop_pane`, `cli_row`,
  `headless_json`, `support_export`) and on `incident_packet` and `public_truth`,
  so desktop, CLI/headless, support, incident, and public-truth views present the
  same finding and repair-candidate identity.
- Every pane carries the locale-invariant `machine_meaning_keys`
  (`finding_code`, `diagnosis_state`, `repair_availability`, `cli_exit_class`),
  so localized prose stays additive and never changes machine meaning.
- Every pane is metadata-safe: `redaction_class: metadata_safe_default` and
  `raw_private_material_excluded: true`.
