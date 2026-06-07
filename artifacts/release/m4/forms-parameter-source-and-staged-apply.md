# Forms, Parameter Sources, And Staged Apply Release Evidence

The stable structured-input lane is backed by:

- Rust contract and validator:
  `crates/aureline-shell/src/forms_parameter_source_and_staged_apply`
- Fixture corpus:
  `fixtures/forms/m4/forms-parameter-source-and-staged-apply/`
- Boundary schema:
  `schemas/release/forms-parameter-source-and-staged-apply.schema.json`
- Contract narrative:
  `docs/m4/forms-parameter-source-and-staged-apply.md`

## Evidence

The corpus exercises immediate, staged, preview-first, and policy-locked forms
across desktop, remote workspace, managed workspace, degraded/offline,
browser-companion, and restricted-client contexts.

The replay test proves:

- source precedence keeps default, detected, imported, workspace, policy, user
  override, and secret-reference values distinct;
- pending validation preserves last-known result, and stale validation
  invalidates on target/dependency changes;
- staged and preview-first flows preserve dirty state, checkpoint/revert,
  review-sheet refs, save/resume posture, and exact final action labels;
- secret/path/reference/code-backed field rows disclose storage, basis, stable
  identity, preview, preservation, and export behavior;
- restricted authority is disclosed before final submit;
- keyboard, screen-reader, IME, RTL, reduced-motion, and focus-return evidence
  remains present on every packet.

## Replay Command

```sh
cargo test -p aureline-shell --test forms_parameter_source_and_staged_apply_fixtures
```
