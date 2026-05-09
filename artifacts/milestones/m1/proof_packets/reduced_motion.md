# Proof packet: reduced-motion policy and protected overlay guards

Purpose: anchor Aureline’s reduced-motion posture contract and ensure protected
shell overlays consult the shared motion policy before running transitions so
input is never delayed and state meaning remains intact.

Canonical sources (non-exhaustive):

- `docs/design/reduced_motion_contract.md`
- `docs/design/motion_timing_contract.md`
- `artifacts/design/motion_tokens.yaml`
- `fixtures/design/motion_cases/overlay_dialog_enter.yaml`
- `fixtures/design/motion_cases/overlay_dialog_exit.yaml`
- `crates/aureline-ui/src/motion/mod.rs`
- `crates/aureline-ui/src/themes/session.rs`
- `crates/aureline-shell/src/bootstrap/native_shell.rs`
- `tools/ci/validate_motion_cases.py`

Evidence storage:

- Validation captures: `artifacts/milestones/m1/captures/`
- Screenshots (optional): `artifacts/milestones/m1/screenshots/`

