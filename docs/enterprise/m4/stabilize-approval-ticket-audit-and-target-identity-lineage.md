# Stabilize approval-ticket audit and target-identity lineage for external mutation, credential projection, and privileged debug surfaces

This stable lane makes the approval-ticket issuance surface — including sandbox
profiles, capability envelopes, issued tickets, spend-attempt events, and
target-identity lineage — visible and verifiable enough that product, security
review, support export, and release packets can all explain: which sandbox
profile a ticket was minted under, which capability envelope bounds its
authority, what target identity the ticket binds, whether any credential
projection is routed through the correct sandbox, and whether remembered
approvals mint fresh tickets at use time. The runtime owner is
`aureline_policy::stabilize_approval_ticket_audit_and_target_identity_lineage`.

The packet does **not** re-derive raw ticket bodies, raw credential payloads,
raw delegated-token material, or plaintext secret bytes. The
`aureline_auth::approval_tickets` beta audit remains canonical for its own
slice. This packet re-exports those qualification tokens verbatim and adds the
stability invariants needed for a single evidence packet.

## Contract

For the stable claim to hold, **all seven** of the following conditions must be
verified simultaneously:

1. **Upstream beta page clean** — `aureline_auth::approval_tickets::audit_approval_ticket_beta_page` returns zero defects.
2. **All four beta profiles covered** — every required profile (`connected`, `mirror_only`, `offline`, `enterprise_managed`) has at least one issued ticket row.
3. **All four sandbox-profile classes covered** — every required sandbox class (`local_only_authority`, `provider_mutation_sandbox`, `remote_helper_sandbox`, `credential_projection_sandbox`) has at least one sandbox-profile row.
4. **Target identity non-empty** — every capability envelope and every ticket row carries a non-empty `target_ref` in its `target_identity`.
5. **No silent capability widening** — no ticket admits capability classes beyond its bound envelope's allowed set (caught by the upstream beta audit).
6. **Remembered-approval lineage complete** — every ticket with `bounded_reuse` use-posture carries at least one `evidence_ref` proving fresh-ticket-at-use-time lineage.
7. **Credential projection sandbox declared** — when any credential projection ticket exists, a `credential_projection_sandbox` sandbox-profile row must be present so projection mode is verifiable.

## Required behavior

`validate_stabilize_approval_ticket_page` rejects a page when its `defects`
list is non-empty.

`audit_stabilize_approval_ticket_page` runs the combined check and returns a
typed `Vec<StabilizeApprovalTicketDefect>`. Each defect carries a closed
`narrow_reason_token` and an export-safe `note`. The absence of defects is the
stable claim.

Two conditions force `Withdrawn` immediately and cannot be overridden:

- A `raw_authority_material_present` defect in the upstream beta page (narrow
  reason: `raw_authority_material_present`). The function returns immediately
  with this single defect and skips all other checks.
- A `self_authorization_attempted` defect in the upstream beta page (narrow
  reason: `self_authorization_attempted`). The function returns immediately
  with this single defect and skips all other checks.

A missing required beta profile or sandbox-profile class narrows to `Preview`
rather than `Beta` because the coverage gap prevents any verifiable claim for
that profile or sandbox class.

## Separation of approval tickets and capability envelopes

Approval tickets and capability envelopes are kept as separate governed objects:

- **Approval tickets** bind: actor scope, action class, target identity, trust-profile ref, policy-epoch ref, issued-at/expires-at, use-posture, and authority requirement.
- **Capability envelopes** bind: sandbox-profile ref, envelope fingerprint, action class, side-effect class, target identity, actor scope, sealed-at/expires-at, evidence refs, and rollback refs.

A ticket references its envelope via `capability_envelope_ref`. Remembered
approvals (`bounded_reuse` posture) MUST carry evidence refs that prove the
envelope and ticket were re-evaluated at use time rather than replaying a
stale authority grant.

## Boundary

The following material stays outside this packet's support boundary:

- Raw ticket bodies or raw approval-ticket payloads.
- Raw credential payloads, raw delegated-token bodies, plaintext secret bytes.
- Raw evidence bodies or raw audit logs.
- Raw identities, raw hostnames, raw tenant identifiers, raw key bytes.

Every exported field carries either a closed-vocabulary token, a plain-language
label, an opaque ref, a count, or a schema-version integer.

## Truth source

| Slice | Canonical source |
|-------|-----------------|
| Sandbox profile posture | `aureline_auth::approval_tickets` |
| Capability envelope lineage | `aureline_auth::approval_tickets` |
| Issued ticket lineage | `aureline_auth::approval_tickets` |
| Spend-attempt audit | `aureline_auth::approval_tickets` |
| Stable qualification | this module (derived from all of the above) |
| Artifact evidence | `artifacts/enterprise/m4/stabilize-approval-ticket-audit-and-target-identity-lineage.md` |

## Verify

```bash
# Build
cargo build -p aureline-policy

# Tests
cargo test -p aureline-policy -- stabilize_approval_ticket
```

All tests under
`stabilize_approval_ticket_audit_and_target_identity_lineage::tests` must pass.
`seeded_stabilize_approval_ticket_page()` must produce zero defects and a
`stable` overall qualification token.
