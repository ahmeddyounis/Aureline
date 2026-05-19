# Runtime authority issuer boundaries

This document is the reviewer-facing landing page for the runtime-authority
issuer-boundary projection owned by
[`/crates/aureline-policy/src/runtime_authority_issuers/mod.rs`](../../../crates/aureline-policy/src/runtime_authority_issuers/mod.rs).
It layers on top of the
[authority-ticket beta](authority_ticket_and_root_authority.md) and closes the
issuer-integrity and remembered-decision-narrowing gap so that high-risk
mutation, provider mutation, credential projection, privileged attach, and
admin changes always derive from explicit issuer lineage rather than ambient
privilege.

The machine-readable contracts live at:

- [`/schemas/security/runtime_authority_issuer.schema.json`](../../../schemas/security/runtime_authority_issuer.schema.json)
  for the full page, issuer record, requesting-surface record,
  remembered-decision rule, boundary-request, boundary-decision, defect,
  summary, and lineage-packet shapes.
- [`/schemas/security/remembered_decision_rule.schema.json`](../../../schemas/security/remembered_decision_rule.schema.json)
  for the narrow remembered-decision rule used by the boundary evaluator.

The release-evidence packet is
[`/artifacts/security/m3/runtime_authority_lineage_packet.md`](../../../artifacts/security/m3/runtime_authority_lineage_packet.md).

## Allowed issuer set

Only three surfaces may mint or refresh authority objects:

- `shell` — the desktop shell approval surface. Mints local, provider,
  credential projection, and privileged debug attach tickets after a fresh
  user prompt. Cannot mint policy, trust, or admin changes.
- `policy_service` — the policy service issuer. Mints any ticket class,
  including policy/trust/admin changes when accompanied by a signed root proof.
- `supervisor` — the supervisor control plane. Mints external provider,
  credential projection, and policy/trust/admin tickets for the control-plane
  lanes the shell does not own.

The projection records each issuer with the closed set of ticket classes it
may mint, the requesting surfaces it routes, the authority sources it may
attest, and whether it may mint root-authority changes. The validator rejects
records that overreach (for example, a `shell` issuer claiming
`policy_trust_admin_change`) with a
`unauthorized_root_authority_claim` or `issuer_overreach` defect.

## Closed requesting-surface set

Every other surface — AI tool plan, extension, recipe runner, CLI script,
browser companion, remote helper, admin console, local admin tool, automation
scheduler — is registered as a requesting surface. Requesting surfaces may
only submit boundary requests routed through one of the allowed issuers. The
runtime never accepts a request from an unregistered surface.

## Structured rejection reasons

Refusals carry one or more closed `rejection_reason` tokens that remain
visible across the UI, CLI, support exports, and the audit stream:

- `self_authorization_by_non_issuer`
- `ambient_privilege_inferred`
- `missing_issuer_binding`
- `issuer_not_allowed_for_surface`
- `remembered_decision_missing`
- `remembered_decision_too_broad`
- `remembered_decision_lifetime_exceeds_budget`
- `remembered_decision_forbidden_class`
- `remembered_decision_target_drift`
- `remembered_decision_actor_drift`
- `authority_source_mismatch`
- `authority_source_unreachable_target`
- `policy_epoch_drift`
- `sandbox_binding_drift`
- `root_authority_proof_missing`

Refused decisions also preserve local editing and force a reprompt before
retry. The validator surfaces `decision_dropped_recovery_guidance` whenever
those preconditions are violated.

## Remembered-decision narrowing

A remembered decision compiles to a `remembered_decision_rule` record bound
to a single target identity, actor subject, authority source, sandbox profile,
policy epoch, scope, renewable ticket lifetime, and rule-level expiry. The
record names the owning issuer and carries an opaque revoke path that the UI,
CLI, support, and admin audit can use to retire the rule.

Only `local_mutation` and `external_provider_mutation` tickets may be backed
by a remembered rule. Credential projection, privileged debug attach, and
policy/trust/admin changes must reprompt and are rejected with
`remembered_decision_forbidden_class` if they try to ride a remembered rule.
The validator further rejects rules whose renewable lifetime exceeds the
ticket-class budget owned by the authority-ticket beta.

When a request asks to renew a remembered rule the boundary evaluator must:

- match the rule's target, actor, ticket class, authority source class,
  policy epoch, and sandbox profile;
- emit `decision_class: remembered_decision_narrowed` rather than
  `granted`; and
- refuse the renewal if the rule has expired or the request widens any field.

The validator surfaces `decision_admitted_beyond_rule_expiry` and
`decision_admitted_on_source_mismatch` for any drift.

## Actor and authority source projection

Each boundary decision exports both `actor_class` and `authority_source_class`
tokens. `authority_source_class` is the closed projection that distinguishes:

- `human_account` — a signed-in human user, including local or organization
  admin step-ups;
- `installation_grant` — an installation, application, or
  policy-injected service grant;
- `delegated_credential` — a delegated credential issued by an upstream
  identity; and
- `local_only_authority` — a local-only authority that cannot reach
  provider-managed targets.

The validator enforces that `authority_source_class` is the canonical
projection of the actor class on both requests and decisions, and that
`local_only_authority` decisions never admit `provider_object` targets.
Provider-linked and local-only actions therefore never conflate.

## Failure-mode drills

The seeded `seeded_runtime_authority_issuer_page()` covers:

- a shell-issuer ticket minted for an AI tool plan;
- a policy-service renewal of a remembered local-format rule;
- an extension self-authorization that is refused with
  `self_authorization_by_non_issuer` and `missing_issuer_binding`;
- a browser companion ambient-privilege inference that is refused with
  `ambient_privilege_inferred`;
- a recipe attempt to broaden a remembered rule onto a different target that
  is refused with `remembered_decision_too_broad` and
  `remembered_decision_target_drift`;
- a remote helper using a local-only authority against a provider target,
  refused with `authority_source_mismatch` and
  `authority_source_unreachable_target`; and
- a supervisor-mediated trust-root rotation that is granted only because the
  request carries a recorded root-authority proof.

The validator unit and integration tests flip the seeded records to admitted
states to confirm the corresponding defects fire.

## What this does not own

This projection does not own the policy evaluation language, the signed
policy bundle format, or the broker secret lifecycle. It enforces that the
authority objects flowing through every Aureline surface have explicit issuer
lineage and remembered-decision narrowing — those neighbours plug into the
existing
[authority-ticket beta](authority_ticket_and_root_authority.md) and the
[policy simulation projection](../../../crates/aureline-policy/src/simulation/mod.rs).
