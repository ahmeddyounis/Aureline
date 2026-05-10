# Proof packet: credential-state row, locked / unavailable store, and provider / account registry seed

Purpose: anchor proof captures for the M1 credential-state seed that explains
storage mode, scope, expiry, revoke action, and locked / unavailable store
posture for credentials and delegated handles used by the initial managed /
provider lanes. The seed surfaces (terminal-pane credential-state chip,
recovery copy, preserved-local-work hint) read structured row records
through this seed; they do not invent a generic "Connected" / "Signed in"
badge and never silently fall back to plaintext-file credential storage.

Reviewer landing page: [`docs/auth/credential_state_seed.md`](../../../docs/auth/credential_state_seed.md).

Canonical sources:

- Crate: `crates/aureline-auth/`
  - `src/lib.rs` — public re-exports for the credential-state seed.
  - `src/credential_state/mod.rs` — credential-state row, lifetime, scope,
    storage posture, typed unavailable-reason vocabulary, provider-account
    record, registry, and the `lock_store` / `mark_store_unavailable`
    failure-drill helpers.
- Frozen cross-tool boundary:
  `docs/auth/credential_state_and_secret_prompt_contract.md` and
  `schemas/auth/credential_state.schema.json`.
- Seed fixtures: `fixtures/auth/credential_state_cases/seed_*.json`.
- Failure-drill fixture:
  `fixtures/auth/credential_state_cases/seed_failure_drill_locked_keychain.json`.
- Integration tests:
  - `crates/aureline-auth/tests/credential_state_cases.rs` (fixture-driven
    seed coverage including the registry-helper failure drill).
- Named consumer wiring:
  - `crates/aureline-shell/src/terminal_pane/mod.rs::TerminalPaneSnapshot::with_credential_registry`
    (and `with_credential_rows`).

Protected walk: open the bottom-panel terminal pane and project a
`TerminalPaneSnapshot` with `with_credential_registry` against the seed
[`ProviderAccountRegistry`]. Confirm the projected
[`CredentialStateChip`](../../../crates/aureline-auth/src/credential_state/mod.rs)
quotes the `state_class`, `storage_mode`, `store_source`, `scope_label`,
`revocation_path_label`, typed `revoke_action`, `local_work_continues`, and
`primary_recovery_action` verbatim. Evidence:
`crates/aureline-shell/src/terminal_pane/mod.rs::tests::snapshot_attaches_credential_state_chips_from_provider_registry`.

Failure drill: stage the failure-drill fixture and call
[`ProviderAccountRegistry::lock_store`](../../../crates/aureline-auth/src/credential_state/mod.rs)
against the OS keychain. Confirm the row flips to `state_class = locked`,
the unavailable reason is `store_locked`, the recovery action is
`resume_after_credential_store_unlock`, the saved alias / handle refs
survive, and the credential-state chip surfaces a visible-recovery posture
with `local_work_continues = true`. Evidence:
`fixtures/auth/credential_state_cases/seed_failure_drill_locked_keychain.json`,
`aureline_auth::credential_state::tests::lock_store_failure_drill_keeps_alias_and_flips_to_locked_state`,
`crates/aureline-auth/tests/credential_state_cases.rs::failure_drill_fixture_surfaces_locked_state_without_widening_local_path`,
`crates/aureline-auth/tests/credential_state_cases.rs::lock_store_helper_drives_the_failure_drill_against_the_registry_fixture`,
`crates/aureline-shell/src/terminal_pane/mod.rs::tests::snapshot_surfaces_locked_store_chip_after_failure_drill`.

Validation commands:

```
cargo test -p aureline-auth
cargo test -p aureline-shell --lib terminal_pane
```

Evidence storage:

- Crate sources: `crates/aureline-auth/`
- Reviewer doc: `docs/auth/credential_state_seed.md`
- Frozen cross-tool boundary:
  `docs/auth/credential_state_and_secret_prompt_contract.md`,
  `schemas/auth/credential_state.schema.json`
- Seed fixtures: `fixtures/auth/credential_state_cases/seed_*.json`
- Integration test: `crates/aureline-auth/tests/credential_state_cases.rs`
- Consumer wiring: `crates/aureline-shell/src/terminal_pane/mod.rs`
