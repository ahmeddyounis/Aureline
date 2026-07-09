# Secret-access prompt sheets and credential-store-capability rows

- Packet: `m5-secret-access-prompt-store-capability-controls:stable:0001`
- Surface: `M5 secret-access prompt sheets and credential-store-capability rows: asking actor, purpose, requested scope, raw-secret-versus-handle-only posture, retention, allow/deny/once semantics, what still works if denied, store type, verification state, portability/export posture, platform limitations, and session-only fallback`
- Secret-access prompts: 6 (1 request a raw reveal)
- Store-capability rows: 6 (4 not securely stored)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Secret-access prompt sheets

- **Aureline sync engine** (first_party_feature) — class `oauth_token`, reveal `handle_only` → `handle_only_available`
- **GitHub connector** (provider_connector) — class `personal_access_token`, reveal `reveal_on_demand` → `raw_reveal_requested`
- **npm registry client** (registry_client) — class `api_key`, reveal `masked_last_four` → `scoped_reveal_only`
- **Warehouse attach** (remote_or_database_attach) — class `client_certificate`, reveal `policy_blocked_reveal` → `reveal_policy_blocked`
- **Release signer** (release_publisher) — class `ssh_or_signing_key`, reveal `never_revealed` → `handle_only_available`
- **Delegated automation agent** (delegated_agent) — class `device_code_grant`, reveal `clipboard_scoped` → `scoped_reveal_only`

## Credential-store-capability rows

- **Hardware security module** — type `os_keychain`, verification `hardware_attested`, export `handle_reference_only` → `securely_stored`
- **OS login keychain** — type `os_keychain`, verification `os_verified`, export `raw_secret_excluded` → `securely_stored`
- **Encrypted session store** — type `encrypted_vault`, verification `encrypted_verified`, export `metadata_only` → `limited_assurance`
- **Unverified file store** — type `external_reference`, verification `unverified`, export `redacted_share` → `unverified_store`
- **Failed keystore probe** — type `encrypted_vault`, verification `verification_failed`, export `endpoints_masked` → `unverified_store`
- **Platform keyring (unsupported)** — type `no_secret_stored`, verification `unsupported`, export `export_blocked` → `unsupported_store`
