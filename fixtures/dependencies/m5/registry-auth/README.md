# Registry auth flow fixtures

These fixtures exercise the registry auth flows object
(`aureline-deps`, `registry_auth_flows`) across the sign-in, secret-handle, and
degradation cases the lane must keep honest:

- **`browser_sso_reachable.json`** — a browser SSO sign-in to a private registry
  with an active OS-store handle; the registry is reachable and fresh, so the flow
  is mutation-ready.
- **`device_code_awaiting_auth_required.json`** — a device-code continuity flow
  against an enterprise mirror with the second-device code not yet entered; auth
  is required, no handle is bound, and the state renders a specific auth-required
  disclosure rather than a generic failure.
- **`keychain_revoked_auth_required.json`** — an OS keychain handle that was
  revoked; auth is required again and the flow offers a rebind alongside retry,
  revoke, and switch-account while a mutation is held back.
- **`vault_handle_mirror_stale.json`** — an enterprise mirror reached with a
  secret-vault handle whose metadata is stale; the staleness is disclosed
  specifically and a mutation is held back, but trust is not hard-blocked.
- **`anonymous_offline_snapshot_only.json`** — anonymous public-registry access
  with only an offline snapshot available; the offline state discloses itself
  specifically instead of reading as a generic no-results or connection failure.

Each file is a `registry_auth_flows` packet validated against
`schemas/deps/registry-auth-flows.schema.json` and the typed model. Credentials
are kept as handles only — an opaque handle ref, a redacted account label, and a
lifecycle state, with `stores_secret_body` always `false` and retention
`broker_resolved_never_persisted`. They use controlled enum values, durable
profile and handle ids, and redacted source and account labels only; they carry
no raw registry URLs, tokens, or credential material. Every row binds to the
frozen package-state matrix through `references_matrix_id`.
