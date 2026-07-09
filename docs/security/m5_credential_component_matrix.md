# M5 Credential Component Matrix (M05-988)

Batch **B117** freezes the reusable credential / auth-boundary component family so every
credential-bearing M5 surface inherits one canonical storage, scope, reveal, delegation,
lifecycle, and export-safety vocabulary. This document is the contract doc referenced by
the frozen matrix packet; the authoritative gate is the Rust validator in
`crates/aureline-provider/src/freeze_the_m5_credential_component_matrix/`.

## What is frozen

Eight governed component families, each named once and constrained by the same controlled
vocabulary regardless of the surface that renders it:

| Component | Canonical schema |
| --- | --- |
| `credential_state_row` | `schemas/ui/m5-credential-state-row.schema.json` |
| `secret_access_prompt_sheet` | `schemas/ui/m5-secret-access-prompt-sheet.schema.json` |
| `vault_or_keychain_picker` | `schemas/ui/m5-vault-keychain-picker.schema.json` |
| `credential_store_capability_row` | `schemas/ui/m5-credential-store-capability-row.schema.json` |
| `browser_device_code_handoff_card` | `schemas/ui/m5-browser-device-code-handoff-card.schema.json` |
| `delegated_credential_row` | `schemas/ui/m5-delegated-credential-row.schema.json` |
| `rotation_revoke_event_row` | `schemas/ui/m5-rotation-revoke-event-row.schema.json` |
| `export_safety_banner` | `schemas/ui/m5-export-safety-banner.schema.json` |

The combined matrix schema is
`schemas/ui/m5-credential-component-matrix.schema.json`.

## Controlled vocabularies

- **Storage modes**: `os_keychain`, `encrypted_vault`, `secret_broker_handle`,
  `session_memory_only`, `external_reference`, `no_secret_stored`.
- **Credential classes**: `oauth_token`, `api_key`, `personal_access_token`,
  `ssh_or_signing_key`, `client_certificate`, `device_code_grant`.
- **Reveal postures**: `handle_only`, `masked_last_four`, `reveal_on_demand`,
  `clipboard_scoped`, `never_revealed`, `policy_blocked_reveal`.
- **Auth-handoff classes**: `system_browser_redirect`, `device_code_poll`,
  `embedded_prompt`, `passkey_step_up`, `delegated_forward`, `offline_deferred`.
- **Delegated-identity states**: `local_identity`, `forwarded_identity`,
  `delegated_on_behalf`, `impersonation_scoped`, `service_account_acting`,
  `delegation_revoked`.
- **Credential lifecycle states**: `active_current`, `refresh_needed`, `rotation_due`,
  `revoked`, `expired`, `superseded`.
- **Store capabilities**: `persist_across_restart`, `os_locked_at_rest`,
  `sync_across_devices`, `hardware_backed`, `store_export_blocked`, `session_only`.
- **Export-safety classes**: `raw_secret_excluded`, `metadata_only`,
  `handle_reference_only`, `redacted_share`, `endpoints_masked`, `export_blocked`.
- **Degraded states** (every component): `fully_available`, `limited_capability`,
  `stale_needs_reauth`, `offline_cached`, `policy_blocked`, `unavailable`.

## Hard invariants

Every component row asserts, and the validator enforces, that it never:

1. masks its storage mode or handle-only-versus-raw-reveal posture;
2. hides a forwarded / delegated identity;
3. invents an alternate label for a governed state; or
4. implies a raw secret is export-safe.

Friendly "connected" / "signed in" wording that conceals storage, delegation, or reveal
truth, and a hidden session-only fallback before send / run / publish, are downgrade
triggers.

## Bound foundations

The matrix reuses, rather than redesigns, the existing credential foundations:

- `schemas/auth/credential_state.schema.json`
- `schemas/auth/secret_access_prompt.schema.json`
- `schemas/auth/credential_picker_state.schema.json`
- `schemas/security/secret_handle.schema.json`
- `schemas/auth/system_browser_return_paths_beta.schema.json`
- `schemas/security/credential_projection.schema.json`
- `schemas/support/export_redaction_profile.schema.json`

## Proof bundle

- Support export: `artifacts/release/m5-credential-component-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-credential-component-proof/matrix.csv`
- Design report: `artifacts/design/m5-credential-component-matrix.md`
- Narrowed fixtures: `fixtures/ui/m5-credential-components/`

Regenerate every artifact from truth with the headless emitter:

```sh
cargo run -q -p aureline-provider --bin aureline_credential_component_matrix -- support-export
cargo run -q -p aureline-provider --bin aureline_credential_component_matrix -- csv
cargo run -q -p aureline-provider --bin aureline_credential_component_matrix -- report
cargo run -q -p aureline-provider --bin aureline_credential_component_matrix -- validate
```

## Downstream adoption

Downstream M5 credential rows point at one canonical component family (the per-component
schemas above) instead of restating credential UI truth by hand, so future rows cannot
clone credential language ad hoc.
