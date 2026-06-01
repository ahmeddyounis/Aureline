# Approval-Ticket Audit and Target-Identity Lineage — Stable Packet

- Packet: `policy:stabilize_approval_ticket_audit_target_identity:default`
- Schema version: `1`
- Contract ref: `policy:stabilize_approval_ticket_audit_target_identity:v1`
- Qualification: `stable` (derived, not asserted)
- Upstream approval-ticket beta page defects: 0
- Stabilize defects: 0
- Withdrawn rows: 0
- Stable rows: all

## Lane coverage

| Profile | Sandbox class | Tickets | Envelopes | Spend attempts |
|---|---|---|---|---|
| `connected` | `provider_mutation_sandbox` | 1 | 1 | present |
| `mirror_only` | `remote_helper_sandbox` | 1 | 1 | present |
| `offline` | `local_only_authority` | 1 | 1 | present |
| `enterprise_managed` | `credential_projection_sandbox` | 1 | 1 | present |

## Evidence sources

- Approval-ticket beta audit:
  `security:approval_ticket_beta:v1`
  — `docs/security/approval_ticket_beta_contract.md`

## Key invariants verified

1. The upstream `ApprovalTicketBetaPage` audits with zero defects.
2. All four required beta profiles (`connected`, `mirror_only`, `offline`, `enterprise_managed`) have at least one issued ticket row.
3. All four sandbox-profile classes (`local_only_authority`, `provider_mutation_sandbox`, `remote_helper_sandbox`, `credential_projection_sandbox`) are covered by sandbox-profile rows.
4. Every capability envelope and ticket row carries a non-empty target identity (`target_ref` non-empty).
5. No ticket admits capabilities beyond its capability envelope's allowed set (no silent authority widening).
6. Every `bounded_reuse` ticket (remembered-approval row) carries at least one evidence ref that proves fresh-ticket-at-use-time lineage.
7. Credential projection ticket rows are backed by a `credential_projection_sandbox` sandbox-profile row; projection mode is visible and verifiable.

## Hard guardrails — withdrawal conditions

The following force `Withdrawn` immediately and cannot be overridden:

- A `raw_authority_material_present` defect in the upstream beta page
  (narrow reason: `raw_authority_material_present`).
- A `self_authorization_attempted` defect in the upstream beta page
  (narrow reason: `self_authorization_attempted`).

## Canonical paths

- Doc: `docs/enterprise/m4/stabilize-approval-ticket-audit-and-target-identity-lineage.md`
- Runtime owner: `aureline_policy::stabilize_approval_ticket_audit_and_target_identity_lineage`
- Fixtures: `fixtures/enterprise/m4/stabilize-approval-ticket-audit-and-target-identity-lineage/`
- Schema: `schemas/enterprise/stabilize-approval-ticket-audit-and-target-identity-lineage.schema.json`
