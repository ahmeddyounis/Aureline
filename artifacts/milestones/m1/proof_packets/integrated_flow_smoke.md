# Integrated Flow Smoke Proof Packet

## Scope

This packet owns the unattended binary smoke for the protected small-project
dogfood fixtures. The lane validates that `aureline_shell` can open each
fixture repository, emit first-frame startup trace evidence with protected
journey budgets, and complete a headless edit/save round trip for the
edit-save action rows.

## Evidence

- Runner: `tests/desktop/m1_integrated_flow_smoke/run_integrated_flow_smoke.py`
- Fixture matrix: `artifacts/milestones/m1/dogfood_matrix.yaml`
- Latest capture: `artifacts/milestones/m1/captures/integrated_flow_smoke_validation_capture.json`
- Binary hook: `crates/aureline-shell/src/bootstrap/native_shell.rs`

## Current Posture

`open`, `quick_open`, and `edit_save` are enforced. `terminal`,
`restore_session`, and `missing_target_recovery` are represented in the capture
as `pending_upstream` rows until their binary-level paths are claim-bearing.

## Refresh Command

```bash
python3 tests/desktop/m1_integrated_flow_smoke/run_integrated_flow_smoke.py --repo-root .
```
