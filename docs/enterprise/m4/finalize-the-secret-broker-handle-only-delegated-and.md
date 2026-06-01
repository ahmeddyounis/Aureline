# Secret-Broker Handle-Only, Delegated, and Session-Only Finalize Packet

This lane finalizes the secret-broker handle-only, delegated, and session-only
modes across claimed stable rows, binding the upstream beta secret-broker audit to
a typed evidence packet. It makes the handle class, rotation/expiry state,
remembered-approval lineage, and export-safe replay posture explicit, verifiable,
and visible to product, security review, support export, and release packets.

The runtime owner is
`aureline_policy::finalize_the_secret_broker_handle_only_delegated_and`.

## What this proves

For every required flow class (`request_workspace`, `database`, `env_config`,
`provider`, `managed_runtime`) across the applicable deployment profiles
(connected, enterprise-managed, offline), the packet binds — for one
`(flow_class × handle_class × profile)` row — a typed handle class, a
`CredentialRotationState` block, and any remembered-approval rows bound to that
credential posture. It then derives a qualification token from the upstream beta
audit and the six stability conditions.

The proof establishes:

- **Handle-class explicitness**: Every row carries one of the six first-class
  handle-class tokens (`os_keychain`, `enterprise_vault`, `delegated_identity`,
  `session_only`, `workspace_variable`, `missing`). No row flattens a brokered
  handle, vault ref, or delegated credential into literal-looking text or durable
  workspace history.
- **Delegated / session-only separation**: Delegated-identity rows and
  session-only rows each carry an explicit rotation event type, a typed rotation
  note, an expiry window, and a redaction-safe replay posture. Generic reconnect
  copy is rejected by the validator.
- **Narrow remembered-approval lineage**: Every remembered-decision approval
  bound to a delegated or session-only posture is bound to exactly one
  `(actor × target × action family × environment × expiry window)` tuple and
  carries typed revocation and reapproval triggers.
- **Export safety**: Raw credentials, session tokens, plaintext workspace-variable
  values, vault entry bodies, and raw handle ids never cross the support-export
  boundary. Closed-vocabulary tokens, opaque refs, plain-language labels, and
  schema-version integers are the only values that appear in export records.

## Contract

The packet does **not** re-derive OIDC session continuity, passkey step-up, or
managed-exit boundary semantics. Those slices remain canonical in their own
modules. This packet adds the handle-class, rotation-state, and
remembered-approval invariants that a standalone secret-broker evidence packet
must carry.

### Required behavior

`validate_finalize_secret_broker_page` rejects a page when:

- the upstream beta page has one or more defects (beta narrowing — upstream must
  be clean before this lane can qualify stable);
- any required flow class (`request_workspace`, `database`, `env_config`,
  `provider`, `managed_runtime`) has no row (`FlowClassCoverageGap` — forces
  `preview`);
- any delegated or session-only row has a generic or empty rotation note
  (`RotationNoteIsGeneric` — beta narrowing);
- any remembered approval is missing one of the five required narrow fields
  (`RememberedApprovalNotNarrow` — beta narrowing);
- any remembered approval is missing a revocation trigger when the row's rotation
  event invalidates remembered decisions (`RevocationTriggerMissing` — beta
  narrowing).

Three conditions force `Withdrawn` and cannot be overridden:

- **`RawSecretMaterialPresent`** — any row claims that raw secret material is
  present, or a remembered approval carries `raw_credential_excluded: false`.
- **`LiteralFlatteningDetected`** — any row claims that a brokered handle, vault
  ref, or delegated credential was flattened to literal-looking text or written to
  durable workspace history.
- **`MissingHandleOnStableClaim`** — any row has handle class `missing` while
  simultaneously claiming a non-degraded (stable) credential posture.

### Boundary

The following material stays outside this packet's support boundary:

- Raw credentials, session tokens, or plaintext secrets.
- Raw workspace-variable values or vault entry bodies.
- Raw handle ids or raw session-token strings.
- Raw provider configuration or raw device-code payloads.

Every exported field carries either a closed-vocabulary token, a plain-language
label, an opaque ref, or a schema-version integer.

## Truth source

| Slice | Canonical source |
|-------|-----------------|
| Handle-class taxonomy | `aureline_policy::finalize_the_secret_broker_handle_only_delegated_and` |
| Rotation / expiry state | `aureline_policy::finalize_the_secret_broker_handle_only_delegated_and` |
| Remembered-approval lineage | `aureline_policy::finalize_the_secret_broker_handle_only_delegated_and` |
| Upstream beta audit | `aureline_auth::secret_broker` |
| Artifact evidence | `artifacts/enterprise/m4/finalize-the-secret-broker-handle-only-delegated-and.md` |
| JSON schema | `schemas/enterprise/finalize-the-secret-broker-handle-only-delegated-and.schema.json` |

## Verify

```
cargo test -p aureline-policy -- finalize_the_secret_broker
```
