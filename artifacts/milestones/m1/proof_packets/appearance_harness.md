# Proof packet: appearance goldens and token-adoption harness

Purpose: keep the protected shell chrome surface visually stable across theme
classes and density/reduced-motion postures by pairing (a) single-frame
screenshot goldens with (b) token-adoption baselines that prove the shell keeps
loading styling primitives from shared token registries.

Canonical sources (non-exhaustive):

- `docs/design/m1_appearance_audit.md`
- `docs/design/appearance_session_contract.md`
- `tests/golden/appearance/README.md`
- `tests/golden/appearance/shell_chrome/README.md`
- `tests/golden/appearance/shell_chrome/token_adoption_baseline.json`
- `crates/aureline-shell/src/bootstrap/native_shell.rs`
- `crates/aureline-shell/src/bootstrap/appearance_golden.rs`
- `crates/aureline-ui/src/components/state_registry.rs`
- `tools/ci/check_token_adoption.py`
- `tools/ci/capture_appearance_goldens.py`
- `tools/ci/compare_appearance_goldens.py`

Evidence storage:

- Validation captures: `artifacts/milestones/m1/captures/`
- Screenshots (optional): `artifacts/milestones/m1/screenshots/`
