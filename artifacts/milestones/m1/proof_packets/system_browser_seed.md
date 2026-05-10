# Proof packet: system-browser auth callback seed and local-versus-managed shell vocabulary

Purpose: anchor proof captures for the M1 system-browser auth callback seed
and the local-versus-managed shell vocabulary that distinguishes the
no-account local path from signed-in managed / self-hosted postures. The
seed surfaces (terminal pane chip, recovery copy, preserved-local-work
block) read structured packet records through this seed; they do not
invent a local `is_signed_in` boolean and never collapse the boundary chip
into a generic `Connected` badge.

Reviewer landing page: [`docs/auth/system_browser_seed.md`](../../../docs/auth/system_browser_seed.md).

Canonical sources:

- Crate: `crates/aureline-auth/`
  - `src/lib.rs` — public re-exports.
  - `src/browser_callback/mod.rs` — packet model, validator, shell
    vocabulary projector, typed denial reasons, preserved-local-work
    discipline.
- Frozen cross-tool boundary: `docs/auth/system_browser_callback_packet.md`
  and `schemas/auth/auth_callback_state.schema.json`.
- Seed fixtures: `fixtures/auth/browser_callback_cases/`.
- Failure-drill fixture:
  `fixtures/auth/browser_callback_cases/failure_drill_app_partially_unavailable.json`.
- Integration tests:
  - `crates/aureline-auth/tests/browser_callback_cases.rs` (fixture-driven
    seed validator coverage).
- Named consumer wiring:
  - `crates/aureline-shell/src/terminal_pane/mod.rs::TerminalPaneSnapshot::project_with_auth_packet`.

Protected walk: open the bottom-panel terminal pane and project a
`TerminalPaneSnapshot` with `project_with_auth_packet` against the seed
[`BrowserCallbackPacket`]. Confirm the projected `ShellAuthChip` quotes the
`account_boundary_class`, the [`ShellAuthVocabulary`] token, the recovery
copy, and the `local_path_available` flag verbatim. Evidence:
`crates/aureline-shell/src/terminal_pane/mod.rs::tests::snapshot_attaches_local_only_auth_chip_for_no_account_path`
and
`crates/aureline-shell/src/terminal_pane/mod.rs::tests::snapshot_attaches_reauth_required_chip_when_managed_callback_is_pending`.

Failure drill: stage the failure-drill fixture and replay the returning
callback envelope with `embedded_fallback_attempted = true`. Confirm the
seed packet flips to `return_denied` with the typed
`callback_embedded_fallback_attempted` reason while the
`PreservedLocalWork` block stays readable so the no-account local path
keeps working. Evidence:
`fixtures/auth/browser_callback_cases/failure_drill_app_partially_unavailable.json`,
`aureline_auth::browser_callback::tests::embedded_fallback_attempt_fails_closed_without_widening_local_path`,
`crates/aureline-auth/tests/browser_callback_cases.rs::failure_drill_fixture_denies_silent_embedded_fallback_and_preserves_local_work`.

Validation commands:

```
cargo test -p aureline-auth
cargo test -p aureline-shell --lib terminal_pane
```

Evidence storage:

- Crate sources: `crates/aureline-auth/`
- Reviewer doc: `docs/auth/system_browser_seed.md`
- Frozen cross-tool boundary: `docs/auth/system_browser_callback_packet.md`,
  `schemas/auth/auth_callback_state.schema.json`
- Seed fixtures: `fixtures/auth/browser_callback_cases/`
- Integration test: `crates/aureline-auth/tests/browser_callback_cases.rs`
- Consumer wiring: `crates/aureline-shell/src/terminal_pane/mod.rs`
