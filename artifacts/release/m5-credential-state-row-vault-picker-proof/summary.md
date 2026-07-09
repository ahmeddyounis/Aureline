# Credential-state rows and vault-or-keychain pickers

- Packet: `m5-credential-state-row-vault-picker-controls:stable:0001`
- Surface: `M5 credential-state rows and vault/keychain pickers: storage mode, source class, target boundary, expiry/rotation/revoke lifecycle, derived health, auditability, keyboard-complete rotate/revoke/test actions, available source, access scope, reveal policy, derived portability, and open-source-of-truth actions`
- Credential-state rows: 6 (5 not healthy)
- Vault/keychain pickers: 5 (4 not freely portable)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Credential-state rows

- **GitHub provider sign-in** (oauth_token) — storage `os_keychain`, target `provider` [active_current] → `healthy`
- **npm registry publish token** (api_key) — storage `encrypted_vault`, target `registry` [refresh_needed] → `attention_needed`
- **Outbound webhook signing key** (personal_access_token) — storage `secret_broker_handle`, target `request` [rotation_due] → `attention_needed`
- **Analytics warehouse connection** (client_certificate) — storage `encrypted_vault`, target `database` [revoked] → `revoked`
- **Remote build host access** (ssh_or_signing_key) — storage `os_keychain`, target `remote` [expired] → `expired`
- **Release artifact signing key** (ssh_or_signing_key) — storage `os_keychain`, target `release` [superseded] → `superseded`

## Vault/keychain pickers

- **macOS login keychain** — scope `device_local`, storage `os_keychain`, reveal `reveal_on_demand` → `portable`
- **Per-user encrypted vault** — scope `user_profile`, storage `encrypted_vault`, reveal `handle_only` → `handle_reference_only`
- **Team secrets manager** — scope `team_shared`, storage `external_reference`, reveal `masked_last_four` → `export_blocked`
- **Org secret broker** — scope `org_managed`, storage `secret_broker_handle`, reveal `handle_only` → `handle_reference_only`
- **Session memory store** — scope `session_only`, storage `session_memory_only`, reveal `clipboard_scoped` → `session_only_non_portable`
