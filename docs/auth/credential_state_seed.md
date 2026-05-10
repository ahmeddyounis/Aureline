# Credential-state row, locked / unavailable store, and provider / account registry seed

This document is the reviewer-facing landing page for the M1 seed that
explains credential authority on a protected row in the live shell. It points
at the canonical Rust seed objects, the locked / unavailable store handling
the seed enforces, the provider / account registry shape later browser
handoff and publish-later work reuse, the credential-state chip a protected
terminal-pane row consumes, the failure-drill fixture, and the cross-tool
boundary vocabulary the seed grows into without forking truth.

## What this seed owns

- one inspectable [`CredentialStateRow`](../../crates/aureline-auth/src/credential_state/mod.rs)
  Rust object that carries:
  - `state_class` (`absent`, `handle_only`, `available`, `locked`, `expired`,
    `revoked`, `rotated`, `store_unavailable`, `policy_blocked`),
  - `display_label` and an opaque `provider_account_ref` joining the row to
    the registry,
  - `source_label` plus optional `authority_alias_ref` and
    `authority_handle_ref` so the saved alias survives a locked or
    unavailable store,
  - a [`CredentialScope`] block (scope label, audience label, optional
    workspace / tenant / actor refs),
  - a [`StoragePosture`] block (`storage_mode`, `store_source`,
    `session_only_downgrade_visible`, `plaintext_fallback_allowed = false`,
    `raw_secret_material_present = false`, reviewable storage note),
  - a [`CredentialLifetime`] block (`lifetime_class`, optional `issued_at`,
    optional `expires_at`, `revocation_path_label`, typed `revoke_action`),
  - the joined `identity_mode` re-exported from
    [`aureline_runtime::IdentityMode`](../../crates/aureline-runtime/src/execution_context/mod.rs)
    and the `trust_state` re-exported from
    [`aureline_workspace::TrustState`](../../crates/aureline-workspace/src/recent_work/mod.rs),
  - a `local_work_continues` flag that keeps the no-account local path
    readable on every protected row,
  - an optional typed `unavailable_reason`
    ([`CredentialUnavailableReason`]: `store_locked`, `store_unavailable`,
    `no_secure_store_configured`, `policy_blocked`, `credential_missing`,
    `credential_expired`),
  - a `recovery_copy_label` and a typed `primary_recovery_action`
    (re-exported from
    [`crate::RetryPathClass`](../../crates/aureline-auth/src/browser_callback/mod.rs)
    so the auth lane and the credential lane share one retry vocabulary),
    and
  - an optional `execution_context_ref` so a support export can join the
    row to the canonical
    [`ExecutionContext`](../../crates/aureline-runtime/src/execution_context/mod.rs)
    record.
- one [`ProviderAccountRecord`] seed for connected accounts and providers —
  the registry's join target, carrying the provider domain label,
  destination class label, account-boundary class, identity mode, trust
  state, optional bound workspace / tenant / actor refs, and the list of
  credential-state row ids the account authorities against.
- one [`ProviderAccountRegistry`] join object that pairs accounts and rows.
  The registry exposes `upsert_account`, `upsert_credential_state`,
  `rows_for_account`, `find_row`, `lock_store`, and `mark_store_unavailable`
  helpers so a consumer can drive the failure drills against the seed
  registry without minting a private "provider list" cache.
- one [`CredentialStateChip`](../../crates/aureline-auth/src/credential_state/mod.rs)
  projection that compresses a row to a single chip the shell consumer
  renders next to the existing
  [`ShellAuthChip`](../../crates/aureline-auth/src/browser_callback/mod.rs).
  The chip carries the `state_class`, `storage_mode`, `store_source`,
  `scope_label`, `audience_label`, `lifetime_class`, optional `expires_at`,
  `revocation_path_label`, typed `revoke_action`, `identity_mode`,
  `trust_state`, `local_work_continues`, `visible_recovery_required`,
  `recovery_copy_label`, `primary_recovery_action`, optional typed
  `unavailable_reason`, and the no-plaintext-fallback / no-raw-material
  flags so a support export round-trips the same truth that a
  terminal-pane row, an activity-center row, and a status mirror render.

## Cross-tool boundary

The seed is a deliberate subset of the frozen cross-tool boundary
vocabulary in
[`docs/auth/credential_state_and_secret_prompt_contract.md`](credential_state_and_secret_prompt_contract.md)
and
[`schemas/auth/credential_state.schema.json`](../../schemas/auth/credential_state.schema.json).
Tokens shared between the seed and the boundary include:

- the credential-state class tokens `absent`, `handle_only`, `available`,
  `locked`, `expired`, `revoked`, `rotated`, `store_unavailable`,
  `policy_blocked`;
- the storage-mode tokens `system_credential_store`,
  `enterprise_secret_store`, `session_only`, `handle_only`, `delegated`,
  `not_configured`;
- the store-source tokens `os_keychain`, `enterprise_vault_adapter`,
  `agent_socket`, `file_backed_secret_ref`, `remote_session_scoped_handle`,
  `hardware_backed_or_passkey_adjacent`, `managed_policy_injector`,
  `browser_device_code_handoff`, `no_secure_store`;
- the lifetime-class tokens `operation_scoped`, `session_only`,
  `time_bounded`, `persistent_until_revoked`,
  `rotated_successor_required`, `unavailable`;
- the identity-mode tokens `account_free_local`, `self_hosted_org`, and
  `managed_workspace` (the seed maps the latter two through
  [`IdentityMode::SelfHostedOrg`] and
  [`IdentityMode::ManagedConvenience`]);
- the trust-state tokens `trusted`, `restricted`; and
- the typed retry-path tokens shared with the system-browser callback seed
  (`continue_local_without_sign_in`, `retry_in_system_browser`,
  `resume_after_credential_store_unlock`, `contact_support_with_export`,
  and the rest).

The seed Rust object covers a subset of the boundary schema's fields.
Adding fields is additive-minor and does not bump the seed's
[`CREDENTIAL_STATE_SEED_SCHEMA_VERSION`](../../crates/aureline-auth/src/credential_state/mod.rs);
widening a vocabulary is additive-minor; repurposing a token is breaking
and requires a new decision row.

The seed introduces a small admin-facing revoke-action vocabulary
([`RevokeActionClass`]) named in plain shell terms —
`remove_saved_provider_session`, `sign_out_of_managed_session`,
`sign_out_of_self_hosted_session`, `disconnect_provider_account`,
`rotate_and_rebind_handle`, `purge_session_only_credential`,
`no_revoke_action_available`. These tokens grow on top of the
`safe_action_class` vocabulary in the broader contract; the broader
vocabulary is the cross-tool boundary, the seed tokens are the
shell-facing labels rendered next to the row.

## Joined with the execution-context lane and the system-browser callback seed

The seed re-exports [`aureline_runtime::IdentityMode`] and
[`aureline_workspace::TrustState`] so the credential-state lane, the
execution-context lane, and the system-browser callback seed share one
identity-mode + trust-state vocabulary. Every credential-state row carries
an optional `execution_context_ref` so a support export can join the row
to the canonical
[`ExecutionContext`](../../crates/aureline-runtime/src/execution_context/mod.rs)
record minted by the resolver in
[`aureline-runtime`](../../crates/aureline-runtime/src/lib.rs).

The seed also re-exports
[`AccountBoundaryClass`](../../crates/aureline-auth/src/browser_callback/mod.rs)
and
[`RetryPathClass`](../../crates/aureline-auth/src/browser_callback/mod.rs)
from the system-browser callback seed so a managed provider account, its
saved provider-session credential row, and the system-browser handoff
packet stay on one boundary / retry vocabulary.

## Protected walk

1. Open a terminal session — the bottom-panel
   [`TerminalPaneSnapshot`](../../crates/aureline-shell/src/terminal_pane/mod.rs)
   consumes the seed [`ProviderAccountRegistry`] through
   `with_credential_registry` and renders one
   [`CredentialStateChip`](../../crates/aureline-auth/src/credential_state/mod.rs)
   per credential-state row.
2. Inspect the locked / unavailable rows — `has_unavailable_credential_state`
   exposes whether any chip sits in an unavailable state class so the
   chrome can promote the visible-recovery row.
3. Drive the failure drill — call
   [`ProviderAccountRegistry::lock_store`](../../crates/aureline-auth/src/credential_state/mod.rs)
   or
   [`ProviderAccountRegistry::mark_store_unavailable`](../../crates/aureline-auth/src/credential_state/mod.rs)
   against the named store source and re-project the snapshot. The
   credential-state chip flips onto the typed locked / unavailable state
   class, the unavailable reason is set verbatim, the recovery action and
   recovery copy travel onto the chip, the saved alias survives, and the
   `local_work_continues` flag stays true.
4. Confirm the no-account local path — even when the managed credential
   row is locked or unavailable the credential-state chip stays
   `local_work_continues = true` and the seed contract forbids a silent
   plaintext-file fallback so editing, save, undo, search, local Git,
   local tasks, and BYOK AI keep working honestly.

## Failure drill

The named failure drill from
[`.plans/M01-080.md`](../../.plans/M01-080.md) — _lock or remove the
credential store and confirm provider/account rows surface unavailable
state without pretending the account is usable_ — is exercised end to end:

- fixture: [`fixtures/auth/credential_state_cases/seed_failure_drill_locked_keychain.json`](../../fixtures/auth/credential_state_cases/seed_failure_drill_locked_keychain.json)
- unit coverage:
  `aureline_auth::credential_state::tests::lock_store_failure_drill_keeps_alias_and_flips_to_locked_state`
  and `mark_store_unavailable_failure_drill_blocks_plaintext_fallback`
- integration coverage:
  `aureline-auth/tests/credential_state_cases.rs::failure_drill_fixture_surfaces_locked_state_without_widening_local_path`
  and `lock_store_helper_drives_the_failure_drill_against_the_registry_fixture`
- consumer coverage:
  `aureline-shell::terminal_pane::tests::snapshot_surfaces_locked_store_chip_after_failure_drill`

The fixture stages a managed provider-session alias backed by the OS
keychain. The keychain locks while the row is staged. The seed flips the
row to `state_class = locked`, sets the typed
[`CredentialUnavailableReason::StoreLocked`](../../crates/aureline-auth/src/credential_state/mod.rs)
unavailable reason, swaps the recovery action to
[`RetryPathClass::ResumeAfterCredentialStoreUnlock`](../../crates/aureline-auth/src/browser_callback/mod.rs),
keeps the saved alias / handle refs intact so the unlock prompt can resolve
the same handle, and the
[`CredentialStateChip`](../../crates/aureline-auth/src/credential_state/mod.rs)
flips to a visible-recovery posture with `local_work_continues = true`.
The shell never silently flips the credential row into a generic
"Connected" / "Signed in" badge, never collapses the locked or
unavailable store posture into a generic warning chip, and never falls
back to a plaintext-file credential.

## Acceptance evidence

Spec acceptance from
[`.plans/M01-080.md`](../../.plans/M01-080.md):

- _A protected dogfood flow can exercise the behavior in the live shell
  without relying on a one-off demo or mock-only path._ Coverage:
  `crates/aureline-shell/src/terminal_pane/mod.rs::tests::snapshot_attaches_credential_state_chips_from_provider_registry`
  and `snapshot_surfaces_locked_store_chip_after_failure_drill`.
- _At least one credential-bearing lane shows a structured credential-state
  row instead of vague connected/signed-in text._ Coverage:
  `aureline_auth::credential_state::tests::baseline_chip_quotes_storage_scope_and_revoke_action_verbatim`
  and the fixtures
  [`seed_account_free_local_byok_ai.json`](../../fixtures/auth/credential_state_cases/seed_account_free_local_byok_ai.json)
  and
  [`seed_managed_provider_session.json`](../../fixtures/auth/credential_state_cases/seed_managed_provider_session.json).
- _Locked/unavailable credential-store states degrade honestly and never
  silently fall back to insecure raw-secret storage._ Coverage:
  `aureline_auth::credential_state::tests::lock_store_failure_drill_keeps_alias_and_flips_to_locked_state`,
  `mark_store_unavailable_failure_drill_blocks_plaintext_fallback`, and the
  fixture
  [`seed_failure_drill_locked_keychain.json`](../../fixtures/auth/credential_state_cases/seed_failure_drill_locked_keychain.json).
- _Unit, integration, fixture, or deterministic-state coverage exists for
  nominal behavior and at least one degraded or error case._ Coverage:
  six unit tests under `aureline_auth::credential_state::tests`, six
  integration tests under
  `crates/aureline-auth/tests/credential_state_cases.rs`, and two
  consumer-side tests under `aureline-shell::terminal_pane::tests`.

## How to verify

```
cargo test -p aureline-auth
cargo test -p aureline-shell --lib terminal_pane
```

The `aureline-auth` crate runs six new unit tests (covering nominal
no-account local and managed-provider-session rows, registry upsert / find
helpers, the locked-store failure drill, and the store-unavailable failure
drill plus the schema-version invariants) and six new fixture-driven
integration tests (every seed fixture parses, the failure-drill fixture
flips to the typed locked state, and the registry helper drives the
failure drill end to end). The `aureline-shell` crate's terminal-pane
suite adds two protected-row tests that wire the seed registry onto the
bottom-panel snapshot and assert the projected chips.

## Out of scope (M1)

- Live OAuth / OIDC / SAML provider adapters and their wire protocols;
- Live secret broker, vault adapters, agent sockets, hardware-backed
  authenticators, and policy injectors (the seed freezes the vocabulary
  they will land on);
- Per-secret prompt rendering and the secret-access prompt round trip;
- Org-switch rebinding flows beyond the seed `rotated_successor_required`
  vocabulary; and
- Support / export packet shape (the seed only records that an export
  carries metadata-only credential rows).

The seed vocabulary above is what those integrations will land on.
