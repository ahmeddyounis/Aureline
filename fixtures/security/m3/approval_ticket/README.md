# Approval-ticket beta fixtures

Reviewer-facing fixtures for the beta projection that binds every high-risk
local, provider, remote, and credential-projection action on a claimed M3 row
to a typed approval ticket or capability envelope issued by the shell, policy
service, or supervisor surface.

The canonical record kind is
`security_approval_ticket_beta_page_record`. The schema lives at
[`/schemas/security/approval_ticket.schema.json`](../../../../schemas/security/approval_ticket.schema.json).
The beta module lives at
[`/crates/aureline-auth/src/approval_tickets/mod.rs`](../../../../crates/aureline-auth/src/approval_tickets/mod.rs)
and the reviewer-facing landing page is
[`/docs/security/m3/approval_ticket_beta.md`](../../../../docs/security/m3/approval_ticket_beta.md).

## Files

These JSON files are produced by the seed example:

```sh
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- page > page.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- sandbox-profile-rows > sandbox_profile_rows.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- capability-envelope-rows > capability_envelope_rows.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- ticket-rows > ticket_rows.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- spend-attempt-events > spend_attempt_events.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- defects > defects.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- support-export > support_export.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- drill-raw-authority-material > drill_raw_authority_material.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- drill-self-authorization-attempted > drill_self_authorization_attempted.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- drill-admitted-under-drift > drill_admitted_under_drift.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- drill-denial-missing-audit-ref > drill_denial_missing_audit_ref.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- drill-ticket-lifetime-exceeds-sandbox-budget > drill_ticket_lifetime_exceeds_sandbox_budget.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- drill-envelope-capability-outside-sandbox > drill_envelope_capability_outside_sandbox.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- drill-missing-requesting-surface-ref > drill_missing_requesting_surface_ref.json
```

| File | Purpose |
| --- | --- |
| `page.json` | Full beta page: sandbox-profile rows, capability-envelope rows, ticket rows, spend-attempt events, defects, summary. |
| `sandbox_profile_rows.json` | One sandbox-profile row per claimed `(profile, sandbox profile class)` pair. |
| `capability_envelope_rows.json` | Capability-envelope rows derived from each claimed sandbox profile. |
| `ticket_rows.json` | Shell-, policy-service-, and supervisor-minted tickets across all four profiles. |
| `spend_attempt_events.json` | Spend attempts spanning admitted, target-drift, expired, policy-epoch-drift, missing-authority, self-authorization, sandbox-drift, and envelope-drift outcomes. |
| `defects.json` | Defect array; empty on the seeded page. |
| `support_export.json` | Support-export wrapper that preserves authority lineage and proves the no-self-authorization invariant. |
| `drill_raw_authority_material.json` | Drill: a ticket guardrail flips `raw_authority_material_present=true`; surfaces `raw_authority_material_present`. |
| `drill_self_authorization_attempted.json` | Drill: a capability envelope guardrail flips `self_authorization_attempted=true`; surfaces `self_authorization_attempted`. |
| `drill_admitted_under_drift.json` | Drill: the seeded target-drift event flips to `admitted`; surfaces `spend_admitted_under_drift`. |
| `drill_denial_missing_audit_ref.json` | Drill: the seeded expired event drops its `audit_event_refs`; surfaces `spend_denial_missing_audit_ref`. |
| `drill_ticket_lifetime_exceeds_sandbox_budget.json` | Drill: the connected ticket widens its expiry past the sandbox budget; surfaces `ticket_lifetime_exceeds_sandbox_budget`. |
| `drill_envelope_capability_outside_sandbox.json` | Drill: the connected envelope admits a capability class outside its sandbox; surfaces `envelope_capability_outside_sandbox`. |
| `drill_missing_requesting_surface_ref.json` | Drill: the AI-tool-plan ticket drops its `requesting_surface_ref`; surfaces `missing_requesting_surface_ref`. |

## Protected states covered

- High-risk local, provider, remote, and credential-projection actions
  cite a typed authority object (approval ticket or reviewed scope)
  minted by the shell, policy service, or supervisor.
- Sandbox profiles, capability envelopes, tickets, and spend attempts
  share one vocabulary across connected, mirror-only, offline, and
  enterprise-managed beta profiles.
- Target drift, sandbox-profile drift, capability-envelope drift,
  trust-profile drift, policy-epoch drift, expiry, missing authority,
  and attempted self-authorization invalidate stale tickets instead of
  permitting replay.
- Every denial declares a typed `native_reapproval_route` other than
  `not_required` and at least one `audit_event_ref` so support and admin
  surfaces can explain why authority was denied.
- Every record carries `raw_authority_material_present=false`,
  `self_authorization_attempted=false`,
  `silent_widening_attempted=false`, `plaintext_secret_present=false`,
  `public_endpoint_fallback_offered=false`, and
  `local_editing_preserved=true`. The redaction-safe support export
  preserves sandbox-profile, capability-envelope, ticket, spend-attempt,
  and audit-event lineage verbatim.
