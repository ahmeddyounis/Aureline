# Sandbox profiles, capability envelopes, and approval-ticket issuance beta

This document is the reviewer-facing landing page for the beta projection that
binds every high-risk local, provider, remote, and credential-projection
action on a claimed M3 row to a typed authority object minted by the shell
or policy plane. The provider crate landed the alpha
[`/schemas/security/approval_ticket_alpha.schema.json`](../approval_ticket_alpha.schema.json)
packet that binds provider-plane mutations to a short-lived ticket or reviewed
scope. The beta projection landed here lifts that vocabulary onto the
shell/policy plane so extensions, AI tool plans, helpers, CLI scripts,
browser companions, and automation schedulers never self-authorize.

The schema lives at
[`/schemas/security/approval_ticket.schema.json`](../../../schemas/security/approval_ticket.schema.json),
the source matrix at
[`/artifacts/security/m3/approval_ticket/approval_ticket_matrix.yaml`](../../../artifacts/security/m3/approval_ticket/approval_ticket_matrix.yaml),
and the contract is owned by
[`/crates/aureline-auth/src/approval_tickets/mod.rs`](../../../crates/aureline-auth/src/approval_tickets/mod.rs).

## What the projection covers

Every claimed beta page is one
`security_approval_ticket_beta_page_record` carrying four record kinds:

- **`security_approval_ticket_beta_sandbox_profile_row_record`** — one row
  per claimed `(profile, sandbox profile class)` pair. Each row names:
  - Profile (`connected`, `mirror_only`, `offline`, `enterprise_managed`).
  - Sandbox profile class (`local_only_authority`,
    `provider_mutation_sandbox`, `remote_helper_sandbox`,
    `credential_projection_sandbox`).
  - Opaque `sandbox_profile_ref` referenced by envelopes and tickets.
  - Opaque `trust_profile_ref` and `policy_epoch_ref` binding the sandbox
    to the active trust posture and policy epoch.
  - `allowed_capability_classes` — capability classes the sandbox may ever
    admit.
  - `allowed_side_effect_classes` — side-effect classes the sandbox may
    ever bind.
  - `default_use_posture` (`single_use`, `bounded_reuse`) and
    `max_ticket_lifetime_seconds` budget.
- **`security_approval_ticket_beta_capability_envelope_row_record`** —
  one row per envelope minted from a sandbox profile. Each envelope names
  the action class (`external_provider_mutation`,
  `helper_backed_remote_mutation`, `local_high_risk_action`,
  `credential_projection`), the side-effect class, the target identity
  (class, ref, label, fingerprint, version), the allowed capability
  classes, the actor scope (actor class, subject ref, granted scope refs,
  auth-source class), the sealed/expires timestamps, and opaque evidence
  and rollback refs.
- **`security_approval_ticket_beta_ticket_row_record`** — one row per
  ticket minted by the shell, policy service, or supervisor. Each ticket
  names the issuer class (`shell`, `policy_service`, `supervisor`), the
  request-origin class (`user_shell_prompt`, `policy_decision`,
  `supervisor_control_path`, `ai_tool_plan`, `extension_request`,
  `cli_script_request`, `browser_companion_request`,
  `remote_helper_request`, `automation_scheduler_request`), the sandbox
  profile and capability envelope refs, the target identity, the actor
  scope, the trust profile and policy epoch refs, the issued-at and
  expires-at timestamps, the typed use posture, the typed authority
  requirement (`approval_ticket_required`, `reviewed_scope_required`,
  `ticket_or_reviewed_scope`), and opaque evidence/rollback refs. The
  ticket also carries an opaque `runtime_approval_ticket_ref` into
  [`/schemas/runtime/approval_ticket.schema.json`](../../../schemas/runtime/approval_ticket.schema.json)
  and an optional opaque `provider_plane_approval_ticket_ref` into the
  alpha packet.
- **`security_approval_ticket_beta_spend_attempt_event_record`** — one
  event per attempt to spend a ticket against the current authority
  context. The event captures the current actor scope, target identity,
  sandbox/envelope/trust/policy refs, the typed evaluation outcome
  (`admitted`, `denied_missing_authority`, `denied_expired`,
  `denied_target_drift`, `denied_trust_profile_drift`,
  `denied_sandbox_profile_drift`, `denied_policy_epoch_drift`,
  `denied_actor_scope_mismatch`, `denied_capability_envelope_drift`,
  `denied_self_authorization_attempted`), the typed native-reapproval
  route (`not_required`, `native_reapproval_sheet`,
  `refresh_target_then_reapprove`, `reauth_then_reapprove`,
  `rescope_then_reapprove`, `inspect_only_denied`), an export-safe
  explanation, and opaque audit-event refs.

## Acceptance posture

- **High-risk actions are bound to typed authority objects.** Every claimed
  ticket cites a sandbox profile ref, a capability envelope ref, a target
  identity (class, ref, fingerprint, version), an actor scope (class,
  subject ref, granted scope refs, auth-source class), a trust profile
  ref, a policy epoch ref, an `issued_at`/`expires_at` window, a typed
  use posture, and a typed authority requirement. Extensions, AI tool
  plans, CLI scripts, helpers, browser companions, and automation
  schedulers route through the shell, policy service, or supervisor —
  their `request_origin_class` MUST be paired with a typed
  `requesting_surface_ref`.
- **Drift invalidates stale tickets rather than permitting replay.** The
  seeded page covers eight typed spend attempts, including
  `denied_target_drift` (target fingerprint advanced after rebase),
  `denied_expired` (ticket spent past `expires_at`),
  `denied_sandbox_profile_drift` (sandbox profile rev advanced),
  `denied_capability_envelope_drift` (envelope rev advanced),
  `denied_policy_epoch_drift` (policy epoch advanced),
  `denied_missing_authority` (no ticket presented), and
  `denied_self_authorization_attempted` (a non-intrinsic request origin
  attempted to spend a ticket minted for a different origin). Every
  denial declares a typed `native_reapproval_route` other than
  `not_required` and at least one `audit_event_ref`. The validator
  surfaces a typed defect when any drift outcome silently admits, when a
  denial collapses the reapproval route to `not_required`, or when a
  denial misses an audit-event ref.
- **No self-authorization, no silent widening, no plaintext secrets, no
  public-endpoint fallback.** Every record carries
  `raw_authority_material_present`, `self_authorization_attempted`,
  `silent_widening_attempted`, `plaintext_secret_present`,
  `public_endpoint_fallback_offered` as `false`, and
  `local_editing_preserved` as `true`. The validator surfaces typed
  defects when any of those flip.
- **Envelope and ticket lineage is enforceable.** The validator surfaces
  a typed defect when a capability envelope is derived from a sandbox
  profile class that does not match its action class, when an envelope
  admits a capability or side-effect class outside its sandbox profile,
  when a ticket lifetime exceeds its sandbox profile's
  `max_ticket_lifetime_seconds` budget, when `lifetime_seconds` drifts
  from `expires_at - issued_at`, or when an envelope/sandbox/ticket ref
  is unknown.
- **Fail-closed profile and sandbox coverage.** All four beta profiles
  (`connected`, `mirror_only`, `offline`, `enterprise_managed`) and all
  four sandbox profile classes (`local_only_authority`,
  `provider_mutation_sandbox`, `remote_helper_sandbox`,
  `credential_projection_sandbox`) must appear; missing coverage surfaces
  a typed defect.
- **Lineage-preserved support and audit packets.**
  [`ApprovalTicketBetaSupportExport`](../../../crates/aureline-auth/src/approval_tickets/mod.rs)
  wraps the page in a redaction-safe envelope that preserves sandbox
  profile, capability envelope, ticket, spend attempt, and audit-event
  ref lineage verbatim. The export proves the
  no-self-authorization invariant.

| Profile               | First-claim authority shape                                                              |
| --------------------- | ---------------------------------------------------------------------------------------- |
| `connected`           | Provider mutation sandbox on a live provider; shell-minted tickets bound to user prompts. |
| `mirror_only`         | Remote-helper sandbox served from a signed mirror; policy-service-minted tickets only.    |
| `offline`             | Local-only authority sandbox; shell-minted tickets for destructive local actions.         |
| `enterprise_managed`  | Credential-projection sandbox; supervisor-minted tickets for scheduled projections.        |

## Failure-mode drills

The seed example regenerates each drill fixture under
[`/fixtures/security/m3/approval_ticket/`](../../../fixtures/security/m3/approval_ticket/):

- `drill_raw_authority_material.json` — a ticket row sets
  `guardrails.raw_authority_material_present=true`; the validator surfaces
  `raw_authority_material_present`.
- `drill_self_authorization_attempted.json` — a capability envelope sets
  `guardrails.self_authorization_attempted=true`; the validator surfaces
  `self_authorization_attempted`.
- `drill_admitted_under_drift.json` — the seeded
  `denied_target_drift` event is flipped to `admitted`; the validator
  surfaces `spend_admitted_under_drift`.
- `drill_denial_missing_audit_ref.json` — the seeded `denied_expired`
  event drops its `audit_event_refs`; the validator surfaces
  `spend_denial_missing_audit_ref`.
- `drill_ticket_lifetime_exceeds_sandbox_budget.json` — the seeded
  connected ticket widens its expiry past the sandbox budget; the
  validator surfaces `ticket_lifetime_exceeds_sandbox_budget`.
- `drill_envelope_capability_outside_sandbox.json` — the seeded
  connected envelope admits a capability class outside its sandbox; the
  validator surfaces `envelope_capability_outside_sandbox`.
- `drill_missing_requesting_surface_ref.json` — the seeded AI-tool-plan
  ticket drops its `requesting_surface_ref`; the validator surfaces
  `missing_requesting_surface_ref`.

## Regeneration

```sh
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- page > fixtures/security/m3/approval_ticket/page.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- sandbox-profile-rows > fixtures/security/m3/approval_ticket/sandbox_profile_rows.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- capability-envelope-rows > fixtures/security/m3/approval_ticket/capability_envelope_rows.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- ticket-rows > fixtures/security/m3/approval_ticket/ticket_rows.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- spend-attempt-events > fixtures/security/m3/approval_ticket/spend_attempt_events.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- defects > fixtures/security/m3/approval_ticket/defects.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- support-export > fixtures/security/m3/approval_ticket/support_export.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- drill-raw-authority-material > fixtures/security/m3/approval_ticket/drill_raw_authority_material.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- drill-self-authorization-attempted > fixtures/security/m3/approval_ticket/drill_self_authorization_attempted.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- drill-admitted-under-drift > fixtures/security/m3/approval_ticket/drill_admitted_under_drift.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- drill-denial-missing-audit-ref > fixtures/security/m3/approval_ticket/drill_denial_missing_audit_ref.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- drill-ticket-lifetime-exceeds-sandbox-budget > fixtures/security/m3/approval_ticket/drill_ticket_lifetime_exceeds_sandbox_budget.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- drill-envelope-capability-outside-sandbox > fixtures/security/m3/approval_ticket/drill_envelope_capability_outside_sandbox.json
cargo run -q -p aureline-auth --example dump_approval_ticket_beta_fixtures -- drill-missing-requesting-surface-ref > fixtures/security/m3/approval_ticket/drill_missing_requesting_surface_ref.json
```

## Verification

- Unit tests at
  [`crates/aureline-auth/src/approval_tickets/mod.rs`](../../../crates/aureline-auth/src/approval_tickets/mod.rs)
  cover the seeded page, every guardrail flip, drift admission, denial
  route collapse, denial audit-ref absence, ticket lifetime overrun,
  envelope-vs-sandbox mismatch, missing requesting-surface ref, profile
  coverage, target-drift sanity, support-export redaction posture, and
  the timestamp parser.
- Fixture-driven coverage at
  [`crates/aureline-auth/tests/approval_ticket_beta_cases.rs`](../../../crates/aureline-auth/tests/approval_ticket_beta_cases.rs)
  parses every fixture under
  `/fixtures/security/m3/approval_ticket/` and verifies the seeded page,
  every drill defect, and the support-export wrapper.
