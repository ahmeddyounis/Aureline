# Browser-or-device-code handoff cards and delegated-credential rows (M05-991)

This lane implements two components frozen in the
[M5 credential component matrix](m5_credential_component_matrix.md) — the
`browser_device_code_handoff_card` and the `delegated_credential_row` — into one
export-safe packet with two co-equal control vectors. Together they keep every
crossing from the local shell into remote / provider-controlled authority explicit
about **which handoff path** is being used and **which identity** is being forwarded
or delegated.

- Crate module:
  `crates/aureline-provider/src/implement_browser_or_device_code_handoff_cards_and_delegated_credential_rows_with_handoff_boundary_and_delegated_identity_origin_truth/`
- Boundary schema:
  [`schemas/ui/m5-browser-device-code-handoff-delegated-credential-controls.schema.json`](../../schemas/ui/m5-browser-device-code-handoff-delegated-credential-controls.schema.json)
- Release proof:
  `artifacts/release/m5-browser-device-code-handoff-delegated-credential-proof/`
- Scenario fixtures:
  `fixtures/ui/m5-browser-device-code-handoff-delegated-credential-controls/`
- Headless emitter:
  `cargo run -p aureline-provider --bin aureline_browser_device_code_handoff_delegated_credential_primitive -- <subcommand>`

## Goal

Make auth acquisition and delegated identity boundaries explicit anywhere Aureline
crosses from local shell into remote / provider-controlled authority.

## Reused vocabulary

The auth-handoff classes, delegated-identity states, storage modes, credential
classes, reveal postures, lifecycle states, degraded states, required labels, surface
families, deployment lines, consumer surfaces, accessibility routes, and downgrade
triggers are reused verbatim from the frozen matrix, so this lane never invents a
parallel credential grammar. It mints new vocabulary only for what that matrix left
implicit about these two controls.

## Browser-or-device-code handoff card

A `BrowserDeviceCodeHandoffCard` always names its **provider / org**, its auth-handoff
**flow kind** (`system_browser_redirect`, `device_code_poll`, `embedded_prompt`,
`passkey_step_up`, `delegated_forward`, or `offline_deferred`), its **fallback state**,
its **local-continuity note**, and **why a safer boundary is preferred**. It always
offers keyboard-complete `continue_preferred_boundary` and `cancel` actions.

Its **handoff-boundary class is derived**, never asserted, from the auth-handoff class,
so a system-browser redirect, a device-code poll, an in-app local capture, and a
delegated / offline-deferred handoff can never blur into one generic sign-in state, and
an in-app local capture can never quietly present as an out-of-app system-browser
boundary:

| Auth-handoff class | Derived handoff-boundary class |
| --- | --- |
| `system_browser_redirect` | `system_browser_boundary` |
| `device_code_poll` | `device_code_boundary` |
| `embedded_prompt` / `passkey_step_up` | `local_capture_boundary` |
| `delegated_forward` / `offline_deferred` | `delegated_or_deferred_boundary` |

Only a `system_browser_boundary` card may claim an out-of-app boundary
(`claims_out_of_app_boundary`). A system-browser card must name its out-of-app
boundary, a device-code card must name its **code / expiry**, an in-app local-capture
card must disclose why a safer boundary is preferred, and a delegated / deferred card
must say it is not a direct sign-in. The cards cover all six auth-handoff classes and
all four handoff-boundary classes.

## Delegated-credential row

A `DelegatedCredentialRow` always names its **source identity** (a delegated-identity
state), its **target scope** (`provider`, `registry`, `request`, `database`, `remote`,
or `release`), its **storage class** (storage mode), its **expiration** (lifecycle
state), and its **policy owner**. It always offers keyboard-complete `stop_forward` and
`rotate` actions.

Its **identity origin is derived**, never asserted, from the delegated-identity state
and the storage mode, so a forwarded, remote-vault-held, or service-issued identity can
never read as a locally stored credential and delegated / forwarded identity always
stays visually distinct from locally stored credentials:

| Condition | Derived identity origin |
| --- | --- |
| identity is `service_account_acting` | `service_issued` |
| storage is `secret_broker_handle` / `external_reference` | `remote_vault` |
| identity is `forwarded_identity` / `delegated_on_behalf` / `impersonation_scoped` / `delegation_revoked` | `forwarded` |
| a local identity backed by local storage | `locally_stored` |

Only a `locally_stored` row may claim to be locally stored
(`claims_locally_stored`). A forwarded row must carry its forwarded note, a
remote-vault row its remote-vault note, a service-issued row its service-issued note,
and a revoked-delegation row its revoked note. The rows cover all six
delegated-identity states and all four identity origins.

## Guardrails

Each control carries three hard invariants, all of which must be `false`:

- Handoff card — `masks_storage_or_reveal_posture`,
  `blurs_handoff_into_generic_sign_in` (system-browser / device-code / local capture
  never blur into one generic sign-in state), and `uses_friendly_connected_wording`.
- Delegated row — `masks_forwarded_or_delegated_identity` (a forwarded or delegated
  identity never reads as locally stored), `implies_raw_secret_exportable`, and
  `uses_friendly_connected_wording`.

Raw secret values, tokens, passphrases, and private endpoints never cross the export
boundary. The support export is metadata-only and export-safe.

## Acceptance criteria

- Auth handoff flows no longer blur system-browser / device-code / local capture into
  one generic sign-in state — the handoff card names its flow kind and its derived
  handoff-boundary class keeps each path distinct while forcing an in-app local capture
  to disclose why a safer boundary is preferred.
- Delegated and forwarded identity remain visually distinct from locally stored
  credentials on every claimed M5 surface — the delegated row's derived identity origin
  never lets a forwarded, remote-vault-held, or service-issued identity read as locally
  stored, and it always names its source identity, target scope, storage class,
  expiration, and policy owner.
