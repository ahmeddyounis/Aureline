# Stabilize system-browser auth, passkey-capable step-up, and recovery flows

This stable lane makes the system-browser authentication path — including
passkey step-up, reauth, and recovery flows — visible and verifiable enough
that product, security review, support export, and release packets can all
explain: what identity mode was used, whether passkey step-up was required,
whether target/action identity was preserved across reauth and recovery, and
what typed fallback exists when passkey is unavailable. The runtime owner is
`aureline_auth::stabilize_system_browser_auth_passkey_capable_step_up`.

Return-path rows and passkey step-up rows alone are not enough. For every
claimed identity row on a stable return path, the packet binds — for one
`source_claim_row_ref` — the return-path posture from the system-browser beta
audit and the passkey step-up posture from the passkey beta audit, then derives
a single qualification token from both.

## Contract

The packet does **not** re-derive system-browser callback, OIDC, or passkey
credential truth. The `aureline_auth::system_browser::beta` return-paths audit,
the `aureline_auth::passkey` step-up beta audit, and the frozen callback /
credential-state boundary remain canonical for their own slices. This packet
re-exports those qualification tokens verbatim, references their rows by id,
and adds the stability invariants a single evidence packet needs to carry:

- **Return-path posture** — for every claimed identity row, the default auth
  mode resolves to `system_browser` or a closed `SystemBrowserPolicyExceptionClass`
  exception token. No row may silently fall through to a wider mode.
- **Passkey step-up posture** — every row that claims passkey capability names
  a closed `PasskeyStepUpPostureClass` token from the safe set. A row that
  claims capability but lacks a step-up posture narrows below Stable.
- **Target/action identity preservation** — reauth and recovery passkey lanes
  carry a `target_action_preservation` token of `preserved`. A lane that widens
  or reroutes the identity across reauth/recovery is withdrawn immediately.
- **Typed fallback** — every passkey lane whose lifecycle or outcome leaves it
  unsatisfied names a typed fallback path. A lane without a fallback narrows
  below Stable rather than implying passkey is the only path.
- **Exportable evidence lineage** — the `source_claim_row_ref` field is the
  join key for reconstructing the full posture from the two beta pages and this
  stable packet in any support export.

## Required behavior

`validate_system_browser_auth_stabilize_page` rejects a page when:

- either the return-paths beta page or the passkey step-up beta page has one or
  more defects (the upstream audits must be clean before stability is derived);
- any claimed identity row's default mode is not `system_browser` and no closed
  exception token is present;
- any row that claims passkey capability lacks a closed step-up posture token;
- any reauth or recovery passkey lane carries a `target_action_preservation`
  token other than `preserved`; or
- any passkey lane that is unsatisfied names no typed fallback.

Two conditions force `Withdrawn` immediately and cannot be overridden by any
other row-level token:

- A `ReturnWidensAuthorityScope` defect in the return-paths beta page, or a
  `GrantedAuthorityWidensRequested` defect in the passkey step-up beta page
  (narrow reason: `authority_widening_on_return`).
- A `ReauthOrRecoveryWidened` defect in the passkey step-up beta page (narrow
  reason: `identity_widening_on_return`).

## Boundary

The following material stays outside this packet's support boundary:

- Raw credentials, session tokens, or plaintext user identity.
- Raw OIDC callback parameters, code verifiers, or nonce values.
- Raw passkey attestation objects or private key material.
- Per-user tenancy assignments or raw tenant configuration.

Every exported field carries either a closed-vocabulary token, a plain-language
label, an opaque ref, or a schema-version integer.

## Truth source

| Slice | Canonical source |
|-------|-----------------|
| Return-path posture | `aureline_auth::system_browser::beta` |
| Passkey step-up posture | `aureline_auth::passkey` |
| Stable qualification | this module (derived from both) |
| Artifact evidence | `artifacts/enterprise/m4/stabilize-system-browser-auth-passkey-capable-step-up.md` |

## Verify

```bash
# Build
cargo build -p aureline-auth

# Tests
cargo test -p aureline-auth -- stabilize_system_browser
```

All 11 tests must pass. `seeded_system_browser_auth_stabilize_page()` must
produce zero defects and a `stable` qualification token.
