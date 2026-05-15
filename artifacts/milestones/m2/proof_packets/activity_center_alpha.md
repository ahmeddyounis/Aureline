# Proof Packet: Activity Center Alpha

```yaml
as_of: 2026-05-15
freshness_date: 2026-05-15
captured_at: 2026-05-15T17:24:31Z
stale_after: P14D
source_revision: git:7ef49d38b543d94113d56e1b3aa289eea9e62c2e
trigger_revision: alpha_activity_center_contract_set@2026-05-15
validator: ci/check_activity_center_alpha.py
validation_capture: artifacts/milestones/m2/captures/activity_center_alpha_validation_capture.json
claim_change_state: no_claim_widening
```

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
