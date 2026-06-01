# Secret-Broker Handle-Only, Delegated, and Session-Only Finalize Packet — Stable Packet

- Packet: `policy:finalize-secret-broker:stable:0001`
- Schema version: `1`
- Contract ref: `policy:finalize_secret_broker_handle_only_delegated:v1`
- Qualification: `stable` (derived, not asserted)
- Withdrawn rows: 0
- Stable rows: all (6)
- Beta-page defects: 0

## Lane coverage

| Flow class | Handle class | Profile | Remembered approvals | Qualification |
|------------|-------------|---------|----------------------|---------------|
| `request_workspace` | `os_keychain` | connected | 0 | `stable` |
| `database` | `enterprise_vault` | enterprise_managed | 0 | `stable` |
| `env_config` | `workspace_variable` | connected | 0 | `stable` |
| `provider` | `delegated_identity` | connected | 1 (valid, narrow) | `stable` |
| `managed_runtime` | `session_only` | enterprise_managed | 1 (expired, narrow) | `stable` |
| `database` | `enterprise_vault` | offline | 0 | `stable` |

## Evidence sources

- Finalize module:
  `policy:finalize_secret_broker_handle_only_delegated:v1`
  — `crates/aureline-policy/src/finalize_the_secret_broker_handle_only_delegated_and/mod.rs`
- Upstream beta module:
  `security:secret_broker_beta:v1`
  — `crates/aureline-auth/src/secret_broker/mod.rs`

## Key invariants verified

1. All five required flow classes (`request_workspace`, `database`, `env_config`,
   `provider`, `managed_runtime`) have at least one row in the finalize packet.
2. Every row carries an explicit first-class handle class token. No row flattens a
   brokered handle, vault ref, or delegated credential into literal text or durable
   workspace history.
3. Every delegated-identity and session-only row carries an explicit rotation event
   type, a typed rotation note (not generic), an expiry window, and a
   redaction-safe replay posture token.
4. Every remembered-approval row is narrow: it names actor, target, action family,
   environment, and expiry window. Revocation and reapproval triggers are preserved.
5. The upstream beta page audits with zero defects before the finalize claim is
   awarded.
6. No raw secret material appears in any row or remembered-approval record.
   `raw_secret_material_excluded` and `raw_credential_excluded` are `true` on
   every record.

## Hard guardrails — withdrawal conditions

All three of the following force `Withdrawn` and cannot be overridden:

- **`RawSecretMaterialPresent`**: any row or remembered approval carries raw
  secret material.
- **`LiteralFlatteningDetected`**: any row flattens a brokered handle, vault ref,
  or delegated credential into literal-looking text.
- **`MissingHandleOnStableClaim`**: any row has handle class `missing` while
  claiming a non-degraded stable posture.

## Canonical paths

- Doc: `docs/enterprise/m4/finalize-the-secret-broker-handle-only-delegated-and.md`
- Runtime owner: `aureline_policy::finalize_the_secret_broker_handle_only_delegated_and`
- Schema: `schemas/enterprise/finalize-the-secret-broker-handle-only-delegated-and.schema.json`
