# Hardened crash capture, exact-build symbolication, crash-loop detection, and evidence preview/export

This document defines the M4 stable lane that hardens the alpha crash-capture
path into a typed, export-safe, redacted-by-default system.

## What this row owns

- The [`CrashLoopSignal`] emitted when incident trails breach restart budgets
  for the same exact-build identity and fault domain.
- The [`EvidencePreview`] that shows what a support export would contain before
  any export occurs.
- The [`EvidenceExportPacket`] that serves as the metadata-safe export
  projection, excluding raw dumps, raw stack bodies, and secret-bearing material
  by default.
- The [`HardenedCrashCaptureEvaluator`] that validates crash-loop signals,
  evidence previews, and export packets against the stable contract.
- The boundary schema at
  [`/schemas/support/harden_crash_capture_exact_build_symbolication_crash_loop.schema.json`](../../schemas/support/harden_crash_capture_exact_build_symbolication_crash_loop.schema.json).
- The protected fixture corpus at
  [`/fixtures/support/m4/harden_crash_capture_exact_build_symbolication_crash_loop/`](../../fixtures/support/m4/harden_crash_capture_exact_build_symbolication_crash_loop/).

## Acceptance and how this row meets it

- Every crash-loop signal carries a closed [`CrashLoopScenarioClass`] and at
  least one [`RecoveryLadderHook`] that preserves user-owned state.
- Crash-loop detection honors exact-build identity: trails with mismatched
  builds are flagged via `any_build_mismatch_observed`.
- Evidence previews always list included and omitted items separately so the
  user knows what will leave the device.
- Export packets carry `raw_dump_exported = false`,
  `raw_private_material_excluded = true`, and
  `ambient_authority_excluded = true` by construction.
- Chain-of-custody entries are mandatory on every export packet so support
  handoff can trace the packet lineage.
- Seeded support scenarios cover all five crash-loop families so no stable
  scenario is untested.

## Failure-drill posture

- Empty strike counts, zero strike budgets, and missing recovery hooks are
  rejected with typed [`HardenedCrashCaptureViolation`] rows.
- Evidence previews that claim `raw_dump_exported = true` are rejected.
- Export packets with empty `export_items` or empty `chain_of_custody` are
  rejected.
- Schema-version and record-kind mismatches are rejected.

## First consumers

- The implementation lives in
  [`crates/aureline-crash/src/harden_crash_capture_exact_build_symbolication_crash_loop/mod.rs`](../../../crates/aureline-crash/src/harden_crash_capture_exact_build_symbolication_crash_loop/mod.rs).
- The primary evaluator is [`HardenedCrashCaptureEvaluator`].

## Related contracts

- [`crash_symbolication_alpha.md`](../m3/crash_symbolication_alpha.md) — exact-build symbolication contract.
- [`incident_trail_alpha.md`](../incident_trail_alpha.md) — incident trail contract.
- [`crash_loop_recovery.schema.json`](../../schemas/support/crash_loop_recovery.schema.json) — recovery center signal schema.
- [`export_redaction_profile.schema.json`](../../schemas/support/export_redaction_profile.schema.json) — default-redacted export profile.

## Out of scope for this row

- Live crash-dump byte reading (owned by platform-specific crash handlers).
- Support bundle assembly (owned by aureline-support-bundle).
- Repair transaction execution (owned by aureline-doctor repair lanes).
- Benchmark-lab execution (owned by the benchmark and CI lanes).
