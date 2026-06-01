# No-Account Local-Use Proof, Managed-Exit Truth, Deprovision-Preserves-Local-Work, and Org-Switch Semantics

This lane makes the managed-exit boundary — what local work survives sign-out,
org-switch, seat loss, or deprovision, and which org-scoped affordances disappear
— explicit, verifiable, and visible to product, security review, support export,
and release packets without requiring reverse engineering.

The runtime owner is
`aureline_auth::finalize_no_account_local_use_proof_deprovision_preserves`.

## What this proves

For every managed-exit event (sign-out, org-switch, seat loss, deprovision) across
all four required deployment profiles (connected, mirror-only, offline,
enterprise-managed), the packet binds — for one `(exit_event × profile)` row — a
`LocalWorkSurvivalBlock` and an `OrgScopedAffordanceBlock`, then derives a
qualification token from both.

The proof establishes:

- **Local-core continuity**: Local editing, local history, local settings, user-owned
  export paths, and the account-free BYOK lane all carry `preserved_unchanged` on
  every managed-exit row. No managed exit event narrows, blocks, or purges local-core
  work.
- **Prior-export opportunity**: Every row carrying a managed exit event sets
  `prior_export_opportunity: true` — the user is offered an export path before
  any managed-affordance closes.
- **Org-affordance disclosure**: Collab sessions, seat-bound extensions, managed
  secret-broker handles, and policy enforcement are listed as `removed_with_notice`
  or `narrowed_with_notice`. The user is notified before the exit event completes.
- **No account required for core work**: The account-free local lane is a permanent
  passthrough with no managed exit events applicable.

## Contract

The packet does **not** re-derive OIDC session continuity, passkey step-up, or
secret-broker repair. Those slices remain canonical in their own modules. This
packet re-uses their exit-state vocabulary (via `OidcSignOutContinuityClass`
tokens referenced by label) and adds the managed-exit invariants a standalone
evidence packet needs to carry.

### Required behavior

`validate_deprovision_preserves_beta_page` rejects a page when:

- any row's `local_editing_token` is not `preserved_unchanged` (withdrawal);
- any row's `local_export_paths_token` is `silently_purged` (withdrawal);
- any managed-exit row has `prior_export_opportunity: false` (beta narrowing);
- any data-bearing org affordance (collab or managed AI) is
  `removed_without_notice` (beta narrowing).

Two conditions force `Withdrawn` immediately:

- A `LocalWorkSilentlyPurged` defect — local work is removed without an export
  opportunity.
- A `ManagedExitBlocksLocalCore` defect — a managed exit event blocks or narrows
  local editing below `preserved_unchanged`.

### Boundary

The following material stays outside this packet's support boundary:

- Raw credentials, session tokens, or plaintext user identity.
- Raw org membership lists, raw tenant configuration, or raw provisioning payloads.
- Raw SCIM deltas or signed-file bundle bodies.
- Per-user entitlement values or raw seat-license identifiers.

Every exported field carries either a closed-vocabulary token, a plain-language
label, an opaque ref, or a schema-version integer.

## Truth source

| Slice | Canonical source |
|-------|-----------------|
| Local-work survival | `aureline_auth::finalize_no_account_local_use_proof_deprovision_preserves` |
| Org-affordance removal | `aureline_auth::finalize_no_account_local_use_proof_deprovision_preserves` |
| OIDC sign-out continuity | `aureline_auth::oidc` |
| Provisioning / deprovision events | `aureline_auth::provisioning` |
| Artifact evidence | `artifacts/enterprise/m4/finalize-no-account-local-use-proof-deprovision-preserves.md` |

## Verify

```bash
# Build
cargo build -p aureline-auth

# Tests
cargo test -p aureline-auth -- finalize_no_account
```

All tests must pass. `seeded_deprovision_preserves_beta_page()` must produce
zero defects and a `stable` overall qualification token.
