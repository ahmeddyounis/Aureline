# Credential-state and secret-access prompt fixtures

These fixtures anchor the credential-state, store-capability, and
secret-access prompt vocabulary frozen in
[`/docs/auth/credential_state_and_secret_prompt_contract.md`](../../../docs/auth/credential_state_and_secret_prompt_contract.md)
and validated by
[`/schemas/auth/credential_state.schema.json`](../../../schemas/auth/credential_state.schema.json).

The examples carry opaque refs, aliases, source labels, class names,
expiry posture, and audit refs only. Raw secret material, private keys,
refresh tokens, OAuth codes, PKCE verifiers, raw request headers, and
ambient delegated credentials never appear.

## Index

| Fixture | Record kind | What it proves |
|---|---|---|
| [`store_capability_matrix.yaml`](./store_capability_matrix.yaml) | `store_capability_matrix_record` | OS keychain, enterprise vault, agent socket, file-backed secret ref, remote/session handle, hardware/passkey-adjacent, policy injector, browser/device-code handoff, and no-secure-store rows share one capability vocabulary. |
| [`locked_keychain_on_launch.yaml`](./locked_keychain_on_launch.yaml) | `credential_state_record` | A locked OS keychain pauses credentialed reads but preserves local work and offers unlock/support metadata actions. |
| [`expired_token_handle.yaml`](./expired_token_handle.yaml) | `credential_state_record` | An expired handle fails closed and routes to renewal without exposing the token. |
| [`policy_blocked_registry_auth.yaml`](./policy_blocked_registry_auth.yaml) | `credential_state_record` | Registry auth blocked by policy stays distinct from missing, expired, or locked credentials. |
| [`remote_credential_missing.yaml`](./remote_credential_missing.yaml) | `credential_state_record` | Remote attach can be missing a credential without implying local workspace failure. |
| [`rebind_after_org_switch.yaml`](./rebind_after_org_switch.yaml) | `credential_state_record` | Org switch requires rebinding to a successor handle and invalidates old scope. |
| [`delegated_credential_remote_attach.yaml`](./delegated_credential_remote_attach.yaml) | `credential_state_record` | Service-issued delegated identity is explicit, scoped, expiring, and revocable. |
| [`browser_device_code_handoff.yaml`](./browser_device_code_handoff.yaml) | `credential_state_record` | Browser/device-code handoff is represented as an authority acquisition path, not a hidden credential type. |
| [`secure_store_downgrade_session_only.yaml`](./secure_store_downgrade_session_only.yaml) | `credential_state_record` | Store-unavailable downgrade to session-only auth is visible and non-persistent. |

## M1 credential-state seed fixtures

These `seed_*.json` fixtures back the M1 credential-state seed in
[`crates/aureline-auth/src/credential_state/`](../../../crates/aureline-auth/src/credential_state/).
They cover the user / admin-facing vocabulary the M1 protected row needs —
storage mode, scope, expiry, revoke action, locked / unavailable state — and
deliberately ride a subset of the broader contract so later browser handoff,
device-code, and publish-later work can grow on top without forking truth.
The reviewer-facing landing page is
[`/docs/auth/credential_state_seed.md`](../../../docs/auth/credential_state_seed.md).

| Fixture | Record kind | What it proves |
|---|---|---|
| [`seed_provider_account_registry.json`](./seed_provider_account_registry.json) | `provider_account_registry_seed_record` | One inspectable join object that pairs the no-account local BYOK AI account with its credential-state row and the managed payments-prod workspace with its provider-session row. |
| [`seed_account_free_local_byok_ai.json`](./seed_account_free_local_byok_ai.json) | `credential_state_row_seed_record` | The no-account local path keeps a saved BYOK AI alias addressable in the OS keychain and stays usable without sign-in. |
| [`seed_managed_provider_session.json`](./seed_managed_provider_session.json) | `credential_state_row_seed_record` | A managed provider-session alias names storage mode, scope (workspace + tenant + actor refs), revoke action, and local-work continuity verbatim. |
| [`seed_failure_drill_locked_keychain.json`](./seed_failure_drill_locked_keychain.json) | `credential_state_row_seed_record` | The named failure drill: the OS keychain locks. The row flips to `state_class = locked`, the unavailable reason is `store_locked`, the recovery action is `resume_after_credential_store_unlock`, the saved alias survives, and the seed contract forbids a silent plaintext-file fallback. |
