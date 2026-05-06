# Post-import validation runner contract

This document freezes the **post-import validation runner** contract that runs
after an import apply commits durable state. The runner exists so migration
cannot end at a generic “succeeded” banner: Aureline must be able to prove the
imported workflow survived translation, record the validation outcomes in a
machine-readable form, and present an explicit next-step state.

The contract is structural. It does not implement a shipping validation engine.
It defines the runner inputs, the validator classes and result vocabulary, the
headless/CLI execution posture, and the record shapes that must be emitted for
support, docs/help projections, enterprise rollout reviews, and design-partner
evaluation.

Companion artifacts:

- [`/schemas/migration/post_import_validation.schema.json`](../../schemas/migration/post_import_validation.schema.json)
  — machine-readable schema for validator run and summary records.
- [`/schemas/migration/migration_report.schema.json`](../../schemas/migration/migration_report.schema.json)
  — machine-readable migration report schema that MUST include validation
  outcomes and an explicit next-step state.
- [`/docs/migration/post_import_decision_sheet.md`](./post_import_decision_sheet.md)
  — the canonical rollback/keep/adopt-bundle/review/export decision sheet.
- [`/docs/migration/migration_center_object_model.md`](./migration_center_object_model.md),
  [`/schemas/migration/migration_session.schema.json`](../../schemas/migration/migration_session.schema.json),
  and [`/schemas/migration/importer_outcome.schema.json`](../../schemas/migration/importer_outcome.schema.json)
  — durable migration session and importer outcome vocabulary.
- [`/docs/migration/first_run_import_diff_and_rollback_contract.md`](./first_run_import_diff_and_rollback_contract.md)
  — preview/apply/checkpoint/rollback contract that creates the checkpoint the
  validator runner depends on.
- [`/schemas/commands/keybinding_resolver.schema.json`](../../schemas/commands/keybinding_resolver.schema.json)
  — keybinding conflict and high-frequency shortcut diff vocabulary reused by
  the `keybinding_conflict` validator.
- Worked cases: [`/fixtures/migration/post_import_cases/`](../../fixtures/migration/post_import_cases/)
  — clean import, weak validation, missing-extension suggestion, bundle handoff,
  and rollback-after-failed-validation cases.

If this contract disagrees with the PRD, technical architecture, technical
design, or UI/UX spec, those sources win and this contract plus its schemas MUST
update in the same change.

## Scope

Frozen at this revision:

- Runner invocation **after apply commits** and the rollback checkpoint exists,
  not during preview.
- A closed set of validator classes (shared across migration-center and
  entry/restore projections):
  - `keybinding_conflict`
  - `missing_extension_suggestion`
  - `launch_config_sanity`
  - `cli_headless_smoke`
  - `workflow_smoke`
  - `settings_schema_migration`
  - `bundle_readiness`
- The outcome vocabulary for each validator run:
  `passed`, `passed_with_warnings`, `failed_recoverable`, `failed_blocking`,
  `skipped_not_applicable`.
- A machine-readable run record per validator and a summary record that can be
  cited by migration sessions, importer outcome packets, migration reports,
  docs/help projections, and support exports.
- The rule that imports cannot finish without:
  - a machine-readable migration report, and
  - an explicit next-step state (decision required/optional/recorded) plus a
    distinct set of next-step actions.

Out of scope:

- Implementing the full validator engine, task runner, or extension installer.
- Defining final UI copy for every validator row or every action.

## Runner inputs and outputs

### Inputs (required)

The runner consumes stable refs rather than raw payload bodies:

- `migration_session_ref`
- `outcome_packet_ref` (grouped `importer_outcome_packet_record`)
- rollback checkpoint linkage (`restore_record_ref` and/or import checkpoint ref)
- `exact_build_identity_ref` (the build running the validation)
- `target_descriptor` (profile/workspace target; opaque ref + redaction-aware
  label only)

### Outputs (required)

1. One `post_import_validation_run_record` per validator class (even when the
   outcome is `skipped_not_applicable`).
2. One `post_import_validation_summary_record` enumerating all validator refs
   and carrying an overall outcome posture.
3. One `migration_report_record` referencing:
   - session/outcome/checkpoint linkage,
   - parity scores and unsupported-item truth,
   - post-import validation outcomes, and
   - an explicit next-step decision state plus available actions.

## Validator classes (required behavior)

All validators:

- MUST be export-safe: raw secrets, raw credentials, raw absolute paths, raw
  extension storage, raw workspace file bodies, and raw log bodies are forbidden
  at this boundary.
- MUST emit one of the five outcome classes.
- MUST carry stable refs (`docs_help_refs`, `support_packet_refs`, `export_refs`)
  when follow-up is required, rather than relying on prose interpretation.

### `keybinding_conflict`

Purpose: ensure imported keybindings do not hide conflicts, reserved-gesture
collisions, or high-frequency muscle-memory risks.

Minimum output detail:

- conflict counts (total + high-frequency bucket counts when available)
- stable refs to keybinding-resolver rows / shortcut-diff rows when emitted
- a clear follow-up cue when conflicts are blocking

### `missing_extension_suggestion`

Purpose: identify missing packages required for the imported workflow to behave
as expected, and recommend compatible native or bridge-backed packages without
silent installation.

Minimum output detail:

- suggestion rows with:
  - source package identity,
  - suggested target package identity,
  - mapping basis (`capability_based` preferred over `name_heuristic`), and
  - optional compatibility scorecard refs when governed scorecards exist.

### `launch_config_sanity`

Purpose: compare imported tasks/run/launch/debug concepts against what Aureline
can execute, and flag semantic mismatches that require review before workflow
parity can be claimed.

Minimum output detail:

- counts for runnable vs review-required vs blocked configs
- refs to any “manual review” rows in the outcome packet where applicable

### `cli_headless_smoke`

Purpose: provide a **headless/CLI runnable** validation path suitable for CI,
enterprise pilots, and support reproduction. This validator must not require UI
scraping to interpret.

Minimum output detail:

- runner mode (`headless_cli` vs interactive)
- suite identity and result posture
- stable artifact refs to emitted logs/artifacts (never raw bodies)

### `workflow_smoke` (optional, opt-in)

Purpose: run optional smoke checks that exercise a claimed workflow wedge (for
example: “launch a dev server” or “run tests”) to validate that the imported
workflow survives beyond configuration translation.

Rules:

- MUST be opt-in and MUST record its opt-in posture.
- A smoke run that would execute arbitrary source scripts MUST be blocked by
policy and recorded as `failed_blocking` (or `skipped_not_applicable` when the
surface never offered the smoke action).

### `settings_schema_migration`

Purpose: ensure the imported settings map is structurally valid against the
destination schema contract and that lossy migrations remain visible as such.

Minimum output detail:

- validated/coerced/rejected key counts (counts only; no raw values)
- refs to schema documents and evidence packets used

### `bundle_readiness`

Purpose: determine whether a **recommended workflow bundle handoff** is
appropriate and safe to present as a next step when parity is weak or gaps are
governed by existing bundle paths.

Rules:

- MUST NOT silently adopt a bundle.
- A recommendation MUST carry a stable `recommended_bundle_ref` and an
  export-safe reason summary.
- Bundle adoption remains a distinct reviewed decision with its own checkpoint.

## CLI/headless contract (required)

The runner MUST be runnable from a headless environment:

- Inputs MUST be addressable via stable refs (session/report) rather than
  UI-only handles.
- Outputs MUST be machine-readable JSON conforming to
  `schemas/migration/post_import_validation.schema.json` (run + summary) and
  referenced by a `migration_report_record`.
- A non-interactive caller MUST be able to treat `failed_blocking` as a hard
  failure without parsing UI text.

## Publication rules

1. A validator MAY be skipped, but the skip MUST be explicit
   (`skipped_not_applicable`) and recorded in the summary record.
2. Migration reports MUST remain available after first run (support, docs/help,
   enterprise rollout review).
3. Rollback and adopt-bundle MUST remain distinct next steps with preserved
   provenance (original report + checkpoint linkage remains addressable after
   either action).

