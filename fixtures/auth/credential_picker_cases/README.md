# Credential picker fixtures

These fixtures anchor the picker and row vocabulary frozen in:

- [`/docs/auth/credential_picker_contract.md`](../../../docs/auth/credential_picker_contract.md)
- [`/schemas/auth/credential_picker_state.schema.json`](../../../schemas/auth/credential_picker_state.schema.json)
- [`/docs/auth/credential_state_and_secret_prompt_contract.md`](../../../docs/auth/credential_state_and_secret_prompt_contract.md)
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../../../docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md)

The examples carry opaque refs, export-safe labels, scope labels, expiry
posture, store/source classes, and audit refs only. Raw secret material,
private keys, refresh tokens, OAuth codes, PKCE verifiers, raw request
headers, and ambient delegated credentials never appear.

## Index

| Fixture | What it proves |
|---|---|
| [`provider_linking_browser_handoff.yaml`](./provider_linking_browser_handoff.yaml) | Provider linking uses handle-returning browser/device-code acquisition paths, not embedded secret entry. |
| [`package_registry_auth_store_unavailable.yaml`](./package_registry_auth_store_unavailable.yaml) | Store-unavailable is explicit; session-only fallback is visible; reference-only workspace/env options remain handle-only. |
| [`request_workspace_auth_reference_only.yaml`](./request_workspace_auth_reference_only.yaml) | Request-workspace auth can select reference-only sources without emitting raw secret bytes into portable state. |
| [`remote_connector_auth_delegated_vs_local.yaml`](./remote_connector_auth_delegated_vs_local.yaml) | Remote connector auth distinguishes delegated identity from local handle use and from reference-only workspace sources. |
| [`policy_denied_picker_requires_repair.yaml`](./policy_denied_picker_requires_repair.yaml) | Policy-denied states remain explicit, with safe repair/escalation actions and no misleading “unknown” fallback. |

