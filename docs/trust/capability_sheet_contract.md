# Capability sheet, transitive-scope, and revocation-surface contract

This document freezes the contract for permission/capability sheets: the
reusable, inspectable surface Aureline uses whenever an extension, AI
tool, remote connector, package adapter, or policy/admin lane requests
meaningful access.

The goal is to make capability review:

- consistent across subsystems (same anatomy, same vocabulary),
- understandable without decoding internal API names, and
- revocable from a stable surface using the same language shown at
  approval time.

Companion artifacts:

- [`/schemas/trust/capability_sheet.schema.json`](../../schemas/trust/capability_sheet.schema.json)
  — machine-readable boundary for one `capability_sheet_record`.
- [`/fixtures/trust/capability_sheets/`](../../fixtures/trust/capability_sheets/)
  — worked sheet fixtures covering extension review, AI tool access,
  remote connector widening, package/script risk review, and
  policy-preapproved / policy-denied cases.

This contract composes with (and does not replace):

- [`/docs/ux/trust_prompt_contract.md`](../ux/trust_prompt_contract.md)
  — the prompt-time request shape; capability sheets are the durable
  *review language* that survives after the prompt closes.
- [`/docs/ux/prompt_grammar_contract.md`](../ux/prompt_grammar_contract.md)
  — button/title grammar; capability sheets reuse the same action-label
  constraints.
- [`/docs/governance/runtime_authority_contract.md`](../governance/runtime_authority_contract.md)
  — authority tickets, issuer rules, invalidation, and audit lineage.
- [`/schemas/extensions/effective_permission.schema.json`](../../schemas/extensions/effective_permission.schema.json)
  — declared vs effective permission and transitive-closure vocabulary
  for extension dependency review.
- [`/docs/verification/install_review_packet.md`](../verification/install_review_packet.md)
  — install/update review disclosure discipline, including transitive
  permission growth visibility.

If this document disagrees with those sources, those sources win and the
sheet contract + schema + fixtures update in the same change.

## Scope

The contract applies to any surface that asks the user to grant, narrow,
deny, revoke, or inspect capabilities:

- extension install / update / activation permission review,
- AI tool read/write/apply authority review,
- remote attach and connector capability widening,
- package manager install/update script-risk review,
- provider handoff and platform-mandated host permission steps, and
- policy-admin preapproval, denial, or ceiling-narrowing explanations.

Out of scope: backend enforcement, UI styling, animation, and layout.
The sheet may render as a modal, side sheet, dedicated review route, CLI
projection, or headless trace, but it MUST collapse to the same record
shape.

## Sheet anatomy (required)

A capability sheet MUST render these slots together with any approve,
deny, narrow, or revoke action:

| Slot | Record field(s) |
|---|---|
| Actor identity | `requester`, `authority_owner` |
| Requested capabilities grouped by consequence/risk | `capability_groups[]`, `capability_groups[].primary_risk_class` |
| Plain-language why | `capability_groups[].why_needed_label` |
| Allowed scope choices | `target_scope.grant_scope_options[]`, `target_scope.grant_scope_selected` |
| Reduced-mode option | `reduced_mode_options[]` |
| Governing policy source and lock posture | `policy_lock`, `capability_groups[].policy_posture` |
| Transitive/inherited disclosure | `transitive_scope_disclosure` |
| Remembered approvals and revocation routes | `remembered_approvals[]`, `target_scope.revocation_route_label`, `sheet_actions[]` |
| Host / browser / device steps when required | `approval_path` |
| Details entry point | `details_action` |

A sheet that cannot populate these slots MUST render as blocked or
details-only; it MUST NOT fall back to generic warning copy.

## Capability rows (real-world consequence first)

Capability rows are grouped by real-world consequence/risk, not by
internal API names. Each row carries:

- a stable capability class (`capability_group_class`),
- the human-readable access description (`requested_access_label`),
- the concrete reason it is needed (`why_needed_label`),
- the policy posture (`policy_posture`), and
- a primary risk class plus any additional risk classes.

Internal identifiers MAY appear in the details surface, but the primary
sheet view must be reviewable without them.

## Transitive scope disclosure (fail closed)

If the effective capability set is wider than what a reviewer might
assume from the top-level actor alone, the sheet MUST disclose the
transitive closure:

- Which closure members exist (`transitive_scope_disclosure.closure_members[]`).
- Which capability groups each member contributes.
- Which inheritance edges widened the effective set
  (`transitive_scope_disclosure.inheritance_edges[]`).

Transitive disclosure is required for extension dependency closure,
connector/provider expansion, helper binaries, remote-side components,
and policy-injected grants. A sheet that cannot enumerate its closure
members or edges is non-conforming and must be blocked with a typed
reason rather than silently omitting the growth.

## Remembered approvals and revocation surfaces

If a decision can be remembered beyond the current action, the sheet
MUST make that durable meaning explicit:

- the remembered scope and consequence (`target_scope.*`),
- any existing remembered approvals that would be reused
  (`remembered_approvals[]`), and
- at least one stable revocation route using the same vocabulary as the
  approval-time sheet (actions such as `Revoke …`, `Open … permissions`,
  or `Request admin review`).

Revocation routes must be stable: a settings/trust surface, policy
center, or provider/OS surface explicitly labeled as host-owned. A sheet
may not claim revocability if revocation is only possible by deleting
files, clearing caches, or other “secret handshake” operations.

## Approval path and host-mandated steps

When the approval path requires a browser handoff, device-code flow, or
platform-mandated OS permission sheet, the capability sheet MUST say so
explicitly and keep the product-owned consequence/scope disclosure
visible:

- `approval_path.path_class` identifies whether the path is product-only
  or includes host/provider steps.
- `sheet_actions[].platform_mandated_host_flow` distinguishes product
  actions from host-owned actions, with a product explanatory label
  alongside the host step.

## Cross-surface projection

The same capability sheet record projects into:

| Consumer | Required projection |
|---|---|
| Product UI | actor identity, capability rows grouped by risk, scope options, reduced mode, transitive disclosure, actions |
| Settings / trust surfaces | the same capability vocabulary plus remembered approvals and revocation actions |
| Support export | sheet id, actor/owner, policy lock state, capability/risk classes, transitive closure summary, revocation route refs |
| Audit/evidence | sheet id, authority owner/source, decision action (if any), and revocation lineage refs |
| CLI/headless trace | the same scope vocabulary and stable capability labels; no “allow/deny” implied by button text alone |

No consumer may infer the effective decision from rendered prose alone.
The stable vocabulary fields (`policy_lock_state`, grant scope tokens,
capability classes, and action/result tokens) are the durable truth.

## Non-conformance

A capability sheet is non-conforming when it:

- lists only internal API names without plain-language consequence,
- omits transitive scope growth when dependencies/helper components
  widen the effective capability set,
- implies a remembered approval without a stable revocation route,
- renders policy preapproval/denial without naming the governing source,
- hides host/provider steps behind a “Continue” style action label, or
- emits different capability/scope vocabulary across UI, settings,
  support exports, and audit traces.

