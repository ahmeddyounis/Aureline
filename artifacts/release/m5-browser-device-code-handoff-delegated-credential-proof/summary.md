# Browser-or-device-code handoff cards and delegated-credential rows

- Packet: `m5-browser-device-code-handoff-delegated-credential-controls:stable:0001`
- Surface: `M5 browser-or-device-code handoff cards and delegated-credential rows: provider/org, auth-handoff flow kind, derived handoff boundary, fallback state, local continuity, device code/expiry, why a safer boundary is preferred, source identity, target scope, storage class, expiration, policy owner, and local-versus-forwarded-versus-remote-vault-versus-service-issued identity origin`
- Handoff cards: 6 (2 are in-app local captures)
- Delegated-credential rows: 6 (5 are not locally stored)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Browser-or-device-code handoff cards

- **GitHub (acme-org)** (System-browser sign-in) — flow `system_browser_redirect` → boundary `system_browser_boundary`
- **Azure DevOps (contoso)** (Device-code sign-in) — flow `device_code_poll` → boundary `device_code_boundary`
- **Self-hosted GitLab (internal)** (In-app credential capture) — flow `embedded_prompt` → boundary `local_capture_boundary`
- **Aureline account (self)** (Passkey step-up) — flow `passkey_step_up` → boundary `local_capture_boundary`
- **Release bot (platform-team)** (Delegated forward to release bot) — flow `delegated_forward` → boundary `delegated_or_deferred_boundary`
- **npm registry (mirror)** (Offline-deferred sign-in) — flow `offline_deferred` → boundary `delegated_or_deferred_boundary`

## Delegated-credential rows

- **You (local identity)** — state `local_identity`, scope `provider`, storage `os_keychain` → origin `locally_stored`
- **Forwarded from teammate (a.jordan)** — state `forwarded_identity`, scope `registry`, storage `encrypted_vault` → origin `forwarded`
- **On behalf of release owner** — state `delegated_on_behalf`, scope `release`, storage `secret_broker_handle` → origin `remote_vault`
- **Impersonating support agent (scoped)** — state `impersonation_scoped`, scope `request`, storage `external_reference` → origin `remote_vault`
- **CI service account (ci-runner)** — state `service_account_acting`, scope `database`, storage `session_memory_only` → origin `service_issued`
- **Revoked forward (former contractor)** — state `delegation_revoked`, scope `remote`, storage `encrypted_vault` → origin `forwarded`
