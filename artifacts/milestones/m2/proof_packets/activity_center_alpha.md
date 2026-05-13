# Proof Packet: Activity Center Alpha

## Scope

This packet covers the activity-center alpha row model for indexing,
restore, install/update, task, and test job families.

## Evidence

- Contract schema: `schemas/events/activity_row.schema.json`
- Runtime consumer: `crates/aureline-shell/src/activity_center/alpha.rs`
- Support/export consumer: `crates/aureline-shell/src/support_seed/mod.rs`
- UX contract: `docs/ux/activity_center_alpha.md`
- Protected fixtures: `fixtures/ux/activity_center_alpha/`
- Validator: `ci/check_activity_center_alpha.py`
- Validation capture: `artifacts/milestones/m2/captures/activity_center_alpha_validation_capture.json`

## Validation Result

`python3 ci/check_activity_center_alpha.py --repo-root . --report artifacts/milestones/m2/captures/activity_center_alpha_validation_capture.json`

The checked-in capture records `status: pass` with no findings.

## Residual Limits

The row model, persistence, fixtures, and support projection are live in
the shell crate. Native chrome rendering and subsystem event producers
still need to call this runtime from their concrete indexing, package,
task, and test execution paths as those paths mature.
