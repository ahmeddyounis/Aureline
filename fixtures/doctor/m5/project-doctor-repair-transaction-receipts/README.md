# Fixtures: Project Doctor guided repair transaction receipts

This directory contains fixture metadata for the
`project_doctor_repair_transaction_receipts` packet.

The canonical full corpus is checked in at:

`artifacts/doctor/m5/project-doctor-repair-transaction-receipts.json`

## Coverage

- Every receipt declares its transaction before mutation: a `repair.`-prefixed
  `repair_id`, a `receipt:`-prefixed `receipt_id`, at least one
  `doctor.finding.`-prefixed initiating finding, impacted state classes,
  preconditions, a disclosed host/boundary and scope ref, a checkpoint
  disclosure, a reversal class, and a verification plan.
- Failure family covers every M5 lane: `notebook_kernel`, `request_api`,
  `database_connection`, `profiler_replay`, `preview_route`, `sync_offboarding`,
  `companion_handoff`, and `incident_packet`.
- Completion state covers every outcome: `fixed`, `partially_repaired`,
  `reduced_but_not_resolved`, `verification_inconclusive`, `rolled_back_exact`,
  and `rolled_back_compensating` — never a generic success/failure.
- Reversal class covers `reversible_transactional`, `reversible_with_snapshot`,
  `compensating_only`, and `irreversible_guarded`; checkpoint kind covers
  `transactional_snapshot`, `filesystem_snapshot`, `state_export`, and `none`.
- Host boundary covers `local_workspace`, `remote_host`, `container`,
  `devcontainer`, `tunnel`, and `managed_service`.
- Guardrails are exercised: receipts with no checkpoint never claim
  clean/snapshot reversibility or an exact rollback and always offer support
  paths, and durable-state mutations are always either checkpointed or
  guarded-irreversible with support paths.
- Stages run in canonical order (`review` → `dry_run` → `checkpoint` → `apply` →
  `verify` → `rollback`/`compensate`); the checkpoint stage appears iff a
  checkpoint was captured, and rollback/compensate are mutually exclusive.
- Every receipt renders on the four core surfaces (`desktop_receipt`, `cli_row`,
  `headless_json`, `support_export`) and on `incident_packet` and
  `public_truth`, carries the locale-invariant `machine_meaning_keys`
  (`repair_id`, `failure_family`, `completion_state`, `reversal_class`), and is
  metadata-safe (`redaction_class: metadata_safe_default`,
  `raw_private_material_excluded: true`).
