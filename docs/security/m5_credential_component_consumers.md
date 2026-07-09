# M5 credential component consumers

**Status:** Stable (adoption lane over the frozen M5 credential component matrix)

This lane proves the eight reusable M5 credential components are adopted consistently across
every claimed M5 credential-bearing surface, so the same storage-mode, credential-class,
reveal-posture, delegated-identity, expiry, and raw-secret-excluded export language survives
outside the primary sign-in or connector lane. It is the closing consumer lane of batch B117,
sitting on top of:

- the frozen matrix
  (`crate::freeze_the_m5_credential_component_matrix`, schema
  `schemas/ui/m5-credential-component-matrix.schema.json`), and
- the four sibling implement lanes that narrow the eight families into working primitives:
  - credential-state row + vault-or-keychain picker →
    `schemas/ui/m5-credential-state-row-vault-picker-controls.schema.json`
  - secret-access-prompt sheet + credential-store-capability row →
    `schemas/ui/m5-secret-access-prompt-store-capability-controls.schema.json`
  - browser/device-code handoff card + delegated-credential row →
    `schemas/ui/m5-browser-device-code-handoff-delegated-credential-controls.schema.json`
  - rotation/revoke-event row + export-safety banner →
    `schemas/ui/m5-rotation-revoke-export-safety-controls.schema.json`

## Consumers

Ten claimed M5 credential consumers each adopt the shared components and point at the canonical
component schemas instead of re-wording facts in local prose:

| Consumer | Role |
| --- | --- |
| `settings` | Credential Settings |
| `request` | Request Auth Surface |
| `database` | Database Attach |
| `registry` | Registry / Provider Auth |
| `release` | Release Publish |
| `remote` | Remote Target Attach |
| `ai_assistant` | AI Model Provider |
| `help` | Help / Docs |
| `support` | Support / Export Desk |
| `export` | Export Packet |

The `help`, `support`, and `export` consumers are held to a stronger check: every family they
adopt must reference the canonical component schema, so a help, support, or export surface can
never drift from the product truth.

## Shared descriptor vocabulary

Every binding keeps all six descriptors explicit — the track invariant for this lane:

`storage_mode`, `credential_class`, `reveal_posture`, `delegated_identity`, `expiry_lifecycle`,
`export_safety`.

## Parity-health, narrowing, and usability honesty

A consumer renders a component under one parity-health mode. Full parity preserves the
descriptor vocabulary with no banner. Any weakened mode auto-narrows the claim and always
discloses a self-contained banner naming the exact reason, the preserved descriptors, and the
recovery action — never a generic "degraded" note:

| Parity-health mode | Narrowing reason | Recovery action | Unusable / forwarded? |
| --- | --- | --- | --- |
| `full_parity` | — | — | no |
| `handle_only_narrowed` | `handle_only_path` | `use_handle_reference_no_raw_copy` | no |
| `expired_or_revoked_narrowed` | `credential_expired_or_revoked` | `rotate_or_reauthenticate` | yes |
| `delegated_or_forwarded_narrowed` | `identity_forwarded_or_delegated` | `review_delegation_source` | yes |
| `session_only_or_policy_blocked_narrowed` | `session_only_or_policy_blocked` | `store_durably_or_request_policy_grant` | yes |

A handle-only path narrows only the reveal posture — the credential is still usable — so it is
not counted against the usability-honesty invariant. A binding that reflects an expired/revoked,
forwarded/delegated, or session-only/policy-blocked credential always narrows and never asserts
that the credential is usable and locally stored, so an unusable or forwarded credential never
masquerades as a usable, locally stored one.

## Guardrails (enforced by `validate`)

- Every one of the eight component families is adopted by at least two distinct consumers —
  proof that they are reusable components, not one sign-in view plus isolated export objects.
- At least one worked binding proves a narrowed rendering with a self-contained banner, and at
  least one proves a full-parity rendering with no banner.
- At least one worked binding reflects an expired/revoked, forwarded/delegated, or session-only
  credential and never asserts usable-and-local; any such binding that claims usable-and-local
  fails validation.
- Friendly "connected" / "signed in" wording never conceals storage mode, forwarded/delegated
  identity, reveal posture, or export-safety limits.

## Artifacts

Minted only by `cargo run -p aureline-provider --bin aureline_credential_component_consumers`:

- `artifacts/release/m5-credential-component-consumer-proof/support_export.json`
- `artifacts/release/m5-credential-component-consumer-proof/matrix.csv`
- `artifacts/release/m5-credential-component-consumer-proof/report.md`
- `fixtures/ui/m5-credential-component-consumers/registry_beta_narrowed.json`
- `fixtures/ui/m5-credential-component-consumers/database_preview_narrowed.json`

The checked-in support export and fixtures are validated against the seed builder by the inline
tests, so the in-code matrix and the on-disk artifacts can never drift.
