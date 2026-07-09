# Secret-access prompt sheets and credential-store-capability rows (M05-990)

This lane implements two components frozen in the
[M5 credential component matrix](m5_credential_component_matrix.md) — the
`secret_access_prompt_sheet` and the `credential_store_capability_row` — into one
export-safe packet with two co-equal control vectors. Together they keep every
request for secret access explicit about **who is asking**, **what can be avoided**,
and **what the current store can actually guarantee**.

- Crate module:
  `crates/aureline-provider/src/implement_secret_access_prompt_sheets_and_credential_store_capability_rows_with_actor_scope_handle_only_and_session_fallback_truth/`
- Boundary schema:
  [`schemas/ui/m5-secret-access-prompt-store-capability-controls.schema.json`](../../schemas/ui/m5-secret-access-prompt-store-capability-controls.schema.json)
- Release proof:
  `artifacts/release/m5-secret-access-prompt-store-capability-proof/`
- Scenario fixtures:
  `fixtures/ui/m5-secret-access-prompt-store-capability-controls/`
- Headless emitter:
  `cargo run -p aureline-provider --bin aureline_secret_access_prompt_store_capability_primitive -- <subcommand>`

## Goal

Keep every request for secret access explicit about who is asking, what can be
avoided, and what the current store can actually guarantee.

## Reused vocabulary

The storage modes, credential classes, reveal postures, store capabilities,
export-safety classes, degraded states, required labels, surface families,
deployment lines, consumer surfaces, accessibility routes, and downgrade triggers
are reused verbatim from the frozen matrix, so this lane never invents a parallel
credential grammar. It mints new vocabulary only for what that matrix left implicit
about these two controls.

## Secret-access prompt sheet

A `SecretAccessPromptSheet` always names its asking **actor** (`first_party_feature`,
`provider_connector`, `registry_client`, `remote_or_database_attach`,
`release_publisher`, or `delegated_agent`), its **purpose**, its **requested scope**,
its credential class, and its raw-secret-versus-handle-only reveal posture. It always
carries a **retention** note, always offers keyboard-complete `allow_once`,
`allow_and_store`, and `deny` actions (the allow / deny / once semantics), and always
names **what still works if denied**.

Its **handle-availability class is derived**, never asserted, from the reveal posture,
so when a handle-only path exists a user sees it surfaced instead of being nudged
toward raw-secret sprawl, and a flow that requests the raw secret can never quietly
present as handle-only:

| Reveal posture | Derived handle-availability class |
| --- | --- |
| `handle_only` / `never_revealed` | `handle_only_available` |
| `masked_last_four` / `clipboard_scoped` | `scoped_reveal_only` |
| `reveal_on_demand` | `raw_reveal_requested` |
| `policy_blocked_reveal` | `reveal_policy_blocked` |

Only a `handle_only_available` prompt may claim a handle-only path exists
(`claims_handle_only_path`). A prompt with a handle-only or scoped path must surface
it, a prompt requesting a raw reveal must disclose that explicitly, and a
policy-blocked prompt must name the block. The prompts cover all six actors and all
four handle-availability classes.

## Credential-store-capability row

A `CredentialStoreCapabilityRow` always names its **store type** (storage mode), its
store capabilities, and its **verification state** (`hardware_attested`,
`os_verified`, `encrypted_verified`, `unverified`, `verification_failed`, or
`unsupported`). It always names its **portability / export posture** (an
export-safety class) and its **platform limitations**, and it always offers
keyboard-complete `verify_store` and `choose_different_store` actions.

Its **trust class is derived**, never asserted, from the verification state and the
store capabilities, so an unverified, verification-failed, or unsupported store can
never read as "securely stored" and no vague "saved securely" message stands in for
an unproven store:

| Condition | Derived trust class |
| --- | --- |
| verification is `unsupported` | `unsupported_store` |
| verification is `unverified` / `verification_failed` | `unverified_store` |
| verified, but session-only (capability `session_only` or no `persist_across_restart`) | `limited_assurance` |
| verified and persistent | `securely_stored` |

Only a `securely_stored` row may claim to be securely stored
(`claims_securely_stored`). An unverified row must carry its unverified note, an
unsupported row its unsupported note, and a limited-assurance row its session-only
fallback note. The rows cover all six verification states and all four trust classes.

## Guardrails

Every control carries three hard invariants, all of which must be `false`:

- `masks_storage_or_reveal_posture` — storage mode and reveal posture stay explicit.
- `implies_raw_secret_exportable` — raw-secret handling is never normalized; a
  control never implies a raw secret is export-safe.
- `uses_friendly_connected_wording` — no friendly "connected" / vague "saved
  securely" wording conceals storage mode, reveal posture, or an unverified store.

Raw secret values, tokens, passphrases, and private endpoints never cross the
export boundary. The support export is metadata-only and export-safe.

## Acceptance criteria

- Users can see when a handle-only path exists instead of being nudged toward
  raw-secret sprawl — the secret-access prompt names its actor, purpose, and
  requested scope, and its derived handle-availability class surfaces a handle-only
  path wherever one exists while forcing any raw reveal to be explicit.
- Unsupported or unverified secure-store states no longer degrade into vague "saved
  securely" messaging — the store-capability row's derived trust class never lets an
  unverified, verification-failed, or unsupported store read as securely stored, and
  it always names its platform limitations and session-only fallback.
