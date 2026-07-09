# Credential-state rows and vault-or-keychain pickers (M05-989)

This lane implements two components frozen in the
[M5 credential component matrix](m5_credential_component_matrix.md) — the
`credential_state_row` and the `vault_or_keychain_picker` — into one export-safe
packet with two co-equal control vectors. Together they make everyday credential
state visible and explainable **before** a user opens a secondary auth or recovery
flow: a user can tell where authority lives and what boundary it applies to without
reading logs or provider docs.

- Crate module:
  `crates/aureline-provider/src/implement_credential_state_rows_and_vault_or_keychain_pickers_with_source_target_boundary_expiry_portability_and_rotate_revoke_test_truth/`
- Boundary schema:
  [`schemas/ui/m5-credential-state-row-vault-picker-controls.schema.json`](../../schemas/ui/m5-credential-state-row-vault-picker-controls.schema.json)
- Release proof:
  `artifacts/release/m5-credential-state-row-vault-picker-proof/`
- Scenario fixtures:
  `fixtures/ui/m5-credential-state-row-vault-picker-controls/`
- Headless emitter:
  `cargo run -p aureline-provider --bin aureline_credential_state_row_vault_picker_primitive -- <subcommand>`

## Goal

Make everyday credential state visible and explainable before the user opens
secondary auth or recovery flows.

## Reused vocabulary

The storage modes, credential classes, reveal postures, lifecycle states, store
capabilities, degraded states, required labels, surface families, deployment lines,
consumer surfaces, accessibility routes, and downgrade triggers are reused verbatim
from the frozen matrix, so this lane never invents a parallel credential grammar.
It mints new vocabulary only for what that matrix left implicit about these two
controls.

## Credential-state row

A `CredentialStateRow` always names its storage mode, credential (source) class,
reveal posture, and the target boundary it applies to (`provider`, `registry`,
`request`, `database`, `remote`, or `release`). It always names its auditability
and always offers keyboard-complete `rotate`, `revoke`, and `test` actions.

Its **health class is derived**, never asserted, from the credential lifecycle
state, so a revoked or expired credential can never read as healthy:

| Lifecycle state | Derived health class |
| --- | --- |
| `active_current` | `healthy` |
| `refresh_needed` / `rotation_due` | `attention_needed` |
| `revoked` | `revoked` |
| `expired` | `expired` |
| `superseded` | `superseded` |

Only a `healthy` row may claim to be healthy (`claims_healthy`), and a row that
needs attention, is revoked, is expired, or is superseded must carry the matching
explicit note. The rows cover all five derived health classes and all six target
boundaries.

## Vault-or-keychain picker

A `VaultOrKeychainPicker` always names its available source (store), access scope
(`device_local`, `user_profile`, `team_shared`, `org_managed`, or `session_only`),
reveal policy, and store capabilities. It always offers an `open_source_of_truth`
action so a user can inspect the store of record.

Its **portability class is derived**, never asserted, from the storage mode, store
capabilities, and reveal policy, so a store-export-blocked or session-only store can
never read as freely portable:

| Condition | Derived portability class |
| --- | --- |
| store capability includes `store_export_blocked` | `export_blocked` |
| storage is `session_memory_only` or capability includes `session_only` | `session_only_non_portable` |
| storage is `secret_broker_handle` or reveal is handle-only / never-revealed / policy-blocked | `handle_reference_only` |
| otherwise | `portable` |

Only a `portable` picker may claim to be portable (`claims_portable`). The pickers
cover all four portability classes and all five access scopes.

## Guardrails

Every control carries three hard invariants, all of which must be `false`:

- `masks_storage_or_reveal_posture` — storage mode and reveal posture stay explicit.
- `implies_raw_secret_exportable` — raw-secret handling is never normalized; a
  picker never implies a raw secret is export-safe.
- `uses_friendly_connected_wording` — friendly "connected" / "signed in" wording
  never conceals storage mode, reveal posture, or delegation.

Raw secret values, tokens, passphrases, and private endpoints never cross the
export boundary. The support export is metadata-only and export-safe.

## Acceptance criteria

- Users can tell where authority lives and what boundary it applies to without
  reading logs or provider docs — the credential-state row names storage mode,
  source class, and target boundary inline, with a derived health state.
- Picker and row surfaces preserve storage-mode clarity without normalizing
  raw-secret handling — the vault picker names its available source, access scope,
  reveal policy, and derived portability, and no control implies a raw secret is
  export-safe.
