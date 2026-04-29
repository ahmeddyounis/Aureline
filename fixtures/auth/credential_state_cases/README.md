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
