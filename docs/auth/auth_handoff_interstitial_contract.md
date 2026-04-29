# Auth-handoff interstitial, embedded-auth exception register, and callback-origin review contract

This document freezes the host-rendered review surface Aureline shows
before any sign-in, step-up, account-link, scope-grant, org-switch,
credential-recovery, or deprovision-acknowledge handoff widens authority
or resumes a privileged flow. It also freezes the auth-lane source of
truth for the embedded-auth exception register and the
callback-origin review rules every interstitial must apply.

The contract is normative. Where it disagrees with the upstream
system-browser callback packet, managed-authentication contract, or
embedded-surface boundary ADR, those upstream sources win and this
document, the schemas, and the fixtures must change in the same patch.

## Companion artifacts

- [`/schemas/auth/auth_handoff_interstitial.schema.json`](../../schemas/auth/auth_handoff_interstitial.schema.json)
  defines the `auth_handoff_interstitial_record` carried by the host
  shell, support exports, admin exports, release evidence, and the
  docs/help diagnostics surfaces.
- [`/schemas/auth/embedded_auth_exception.schema.json`](../../schemas/auth/embedded_auth_exception.schema.json)
  defines the `embedded_auth_exception_register_record` — the auth-lane
  source of truth for every approved embedded auth path.
- [`/fixtures/auth/auth_handoff_cases/`](../../fixtures/auth/auth_handoff_cases/)
  contains worked examples for system-browser-first sign-in, device-code
  fallback, passkey step-up, embedded-session-refresh exception,
  embedded-password exception, user-rejected handoff, and a
  callback-origin mismatch denied at the interstitial.

## Upstream and sibling contracts

This contract composes with existing owners and does not replace them:

- [`/docs/auth/system_browser_callback_packet.md`](./system_browser_callback_packet.md)
  freezes the outbound system-browser handoff, callback correlation,
  return-route, account-boundary, and credential-store lock-state
  packet. Every interstitial cites the bound `auth_callback_packet_record`
  by id.
- [`/docs/auth/managed_auth_and_session_continuity_contract.md`](./managed_auth_and_session_continuity_contract.md)
  freezes managed-session states, reauth-requirement objects, and the
  passkey-capable disclosure rules. Every interstitial whose action
  class touches a managed session cites the bound
  `managed_session_state_record` by id.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  owns `browser_handoff_packet`, `provider_class`,
  `provider_actor_class`, and the connected-provider record family. The
  interstitial cites the handoff packet by ref rather than re-minting
  it.
- [`/docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`](../adr/0015-embedded-surface-boundary-and-auth-handoff.md)
  owns the embedded-surface boundary record family,
  surface-family vocabulary, and the host-rendered
  `embedded_auth_exception_record` that re-renders register entries on
  boundary cards. The auth-lane register schema published here is the
  source of truth that the boundary-card record mirrors.
- [`/docs/ux/embedded_surface_boundary_cards.md`](../ux/embedded_surface_boundary_cards.md)
  owns the render-side boundary card. The card cites the same register
  entry id this document publishes.

If this document disagrees with those sources, those sources win.

## Why this exists

System-browser-first auth and account-free local mode are only honest
postures if every embedded fallback is reviewable and every callback
origin is verifiable before authority widens. Without one auth-handoff
interstitial vocabulary and one auth-lane exception register:

- a sign-in button could route through an embedded webview "just this
  once" without leaving an auditable record of the exception;
- a step-up authority prompt could land on a provider page whose
  callback origin Aureline never previewed, so the reviewer would
  authorise a return route they never saw;
- device-code fallback, passkey availability, and system-browser
  posture would each be expressed in surface-local prose, so a deployment
  could advertise a path it cannot honour;
- product-owned/native review surfaces (security messaging, update
  verification, workspace-trust elevation, rollback / restore
  confirmation, AI apply review, high-risk approval sheets) could be
  imitated by an embedded provider page with a subtly different chrome;
- support exports, release evidence, admin exports, and docs/help
  diagnostics would each invent their own copy for the same handoff,
  breaking traceability across surfaces; and
- an embedded-auth exception would live as a per-provider implementation
  detail rather than an explicit, scoped, and time-limited record that
  reviewers, support, and admins can quote by id.

This contract closes that gap before any live OAuth, OIDC, SAML,
device-code, or passkey integration lands. Live IdP implementations and
provider-specific protocol flows remain out of scope; the vocabulary
below is what those integrations will honour.

## Scope

Frozen at this revision:

- one `auth_handoff_interstitial_record` shape for every reviewable
  auth handoff;
- one `embedded_auth_exception_register_record` shape for every
  approved embedded auth path;
- closed vocabularies for requested-action class, callback-origin
  class, availability class, fallback rule class, native-approval
  boundary, lower-trust cue, capability limitation, interstitial-state
  class, denial reason, audit-event id, and review state;
- callback-origin review rules for loopback HTTP return, platform
  deep-link return, device-code poll return, manual return, and
  platform-authenticator-native return;
- the rules that bind the interstitial, the exception register, the
  system-browser callback packet, the managed-session state record, and
  the embedded-surface boundary card so a single review object can be
  quoted across desktop UI, support export, admin export, release
  evidence, and docs/help diagnostics.

Out of scope:

- live OAuth / OIDC / SAML provider implementations and their wire
  protocols;
- per-provider docs, marketplace, dashboard, or admin UI specifics
  beyond the closed vocabularies the interstitial and register quote;
- passkey / WebAuthn platform adapters and device-code transport;
- managed-admin policy-bundle formats and policy-engine implementations.

## Interstitial render invariant

An `auth_handoff_interstitial_record` is the only legitimate way to
widen authority or resume a privileged flow through a browser, device-
code, or platform-authenticator-native handoff in Aureline.

A surface that:

- launches a system browser, opens a device-code companion card, opens
  a platform authenticator, or opens an embedded auth page without
  quoting an `auth_handoff_interstitial_record` by id;
- collapses requested action class, provider/domain identity, callback
  origin, target scope, expiry/replay policy, and confirm/reject actions
  into a single "Sign in" button;
- routes the confirm action into a host-native review surface
  (security messaging, update verification, workspace-trust elevation,
  rollback / restore confirmation, AI apply review, high-risk approval
  sheet) without first surfacing the interstitial; or
- silently retries, silently downgrades to embedded auth, or silently
  reopens a different tenant or workspace context after the reviewer
  rejected the handoff,

is non-conforming. The interstitial is **render of the truth**, not a
separate truth.

## Required interstitial fields

Every `auth_handoff_interstitial_record` carries the following slots.
Renderers may project subsets of optional rows but they MUST NOT drop
the required slots or replace them with prose-only state.

| Slot                              | Required content                                                                                                                                       | Non-conforming collapse                                                                                                  |
|-----------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------|
| `interstitial_id`                 | Opaque, stable id of this review surface.                                                                                                              | Surface-local DOM id with no durable identity.                                                                           |
| `interstitial_state`              | One of the closed `interstitial_state_class` values.                                                                                                   | Free-form `pending` chip with no closed state.                                                                           |
| `auth_flow_class`                 | One of `system_browser`, `device_code`, `platform_authenticator_native`, `embedded_session_refresh`, `embedded_password_exception`, `not_applicable`.  | Generic "Sign in" with no flow class.                                                                                    |
| `requested_action_class`          | One of `sign_in_new_session`, `resume_existing_session`, `step_up_authority`, `refresh_existing_session`, `link_account`, `scope_grant`, `switch_org_or_tenant`, `admin_step_up`, `credential_recovery`, `deprovision_acknowledge`. | Two unrelated outcomes packed into one prompt.                            |
| `requested_action_label`          | Reviewable single-sentence label for the action.                                                                                                       | Marketing copy or vendor tagline.                                                                                        |
| `provider_identity`               | Provider class, label, domain, optional scope, and acting-actor class as separate slots.                                                               | A generic `Connected` chip.                                                                                              |
| `callback_origin_disclosure`      | Callback origin class, plain origin label, host or domain label, return-anchor label, and origin-verification state.                                   | Bare URL string with no class or verification state.                                                                     |
| `target_scope`                    | Identity mode, account boundary class, scope summary label, bound workspace ref, bound tenant or org ref, bound actor subject ref.                     | "Workspace" with no scope summary.                                                                                       |
| `expiry_replay_policy`            | Issued-at, expires-at, replay posture, single-use flag, expiry summary label.                                                                          | "Expires later" prose.                                                                                                   |
| `availability_disclosure`         | System-browser, device-code, and passkey availability rows; preferred flow class; ordered fallback flow list; availability summary label.              | Hiding fallback availability behind a tooltip or inspector.                                                              |
| `native_approval_boundary`        | `product_owned_native_required`, `embedded_handoff_forbidden`, `host_native_step_up_required`, list of `reserved_native_surface_refs`, summary label.  | Routing the confirm action into a host-native review surface without disclosing it on the interstitial.                  |
| `confirm_action`                  | Action label, primary action class, host-native target requirement, optional hook ref, optional browser-handoff packet ref.                            | A `Sign in` button with no action class or target.                                                                       |
| `reject_action`                   | Action label, rejection outcome class, optional hook ref, preserved-local-work assertion.                                                              | Silent abort, silent embedded retry, or silent strand.                                                                   |
| `preserved_local_work_summary`    | Posture class, summary label, and a quote-by-ref pointer to the canonical preserved-local-work block on the bound callback or session record.          | Re-describing local-continuity locally on the interstitial.                                                              |
| `auth_callback_packet_ref`        | Opaque ref to the bound `auth_callback_packet_record` (null only for `not_applicable` and pre-callback recovery flows).                                | Minting an interstitial without a backing callback packet.                                                               |
| `embedded_auth_exception_ref`     | Required for `embedded_session_refresh` and `embedded_password_exception` flows; null for everything else.                                              | Quoting an embedded path without an active register entry.                                                               |
| `policy_context`                  | Identity mode, policy epoch, trust state, optional execution context.                                                                                  | Interstitial minted without policy context.                                                                              |
| `redaction_class`                 | Closed redaction class for support / export.                                                                                                           | Local synonym such as `safe`, `internal`, `restricted`.                                                                  |
| `minted_at`                       | Monotonic timestamp from the host.                                                                                                                     | Surface-local wall-clock string.                                                                                         |

The schema enforces the required slots through `required` plus
per-`auth_flow_class` and per-`requested_action_class` `allOf` branches.

## Callback-origin review rules

`callback_origin_disclosure` is the row reviewers use to verify the
returning browser state before any state mutation. Rules:

1. The interstitial MUST disclose the callback origin class, plain
   origin label, host or domain label, return-anchor label, and
   origin-verification state before the reviewer confirms the handoff.
2. `loopback_http_origin` MUST carry `loopback_port_pinned` origin
   verification (re-exported from the system-browser callback packet)
   and a `return_anchor_label` that names the workspace and tenant
   scope.
3. `platform_deep_link_origin` MUST carry `deep_link_scheme_pinned`
   verification and a deep-link scheme label whose host the reviewer
   can read in plain language.
4. `device_code_poll_origin` carries `device_code_poll_only`
   verification and a `code_expiry_label` on the
   `expiry_replay_policy` row. Raw codes MUST NOT appear; the visible
   cue is the only place a short-lived code is shown to the user.
5. `manual_resume_origin` is admissible only as a last-resort path. The
   surface MUST keep `preserved_local_work_summary.posture_class` set
   to a `local_work_intact*` value or to a typed narrowing class so
   reviewers see what continues locally during the manual resume.
6. `platform_authenticator_native_origin` is admissible only when the
   `auth_flow_class` is `platform_authenticator_native`. The
   `native_approval_boundary` MUST keep
   `embedded_handoff_forbidden = true`.
7. A returning browser state whose origin, replay, tenant/workspace, or
   policy check fails MUST transition the interstitial to one of
   `denied_origin_mismatch`, `denied_replay`, or
   `denied_native_boundary` and MUST emit the matching
   `auth_handoff_interstitial_denied_*` audit event. Silent retry,
   silent embedded fallback, and silent reopen of a different tenant
   or workspace context are forbidden.
8. The interstitial MUST never widen the callback origin between mint
   time and confirm time. A surface that re-renders with a different
   origin or anchor without bumping `interstitial_id` is non-conforming.

## Requested-action review rules

`requested_action_class` names exactly one widening or resumption
outcome. Reviewers see one row per pending review; a single
interstitial MUST NOT pack two unrelated outcomes into one confirm.

| Class                          | Required behavior                                                                                                                                        |
|--------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------|
| `sign_in_new_session`          | Pre-workspace or pre-provider sign-in. The interstitial MAY have a null `bound_workspace_ref` or `bound_tenant_or_org_ref` only when the identity mode is `account_free_local` or the flow is pre-provider sign-in. |
| `resume_existing_session`      | Resume a session whose binding the interstitial restates. `auth_callback_packet_ref` and `managed_session_state_ref` MUST both be present.                |
| `step_up_authority`            | Crosses a product-owned/native boundary. The schema fixes `product_owned_native_required = true` and `host_native_target_required = true`.               |
| `refresh_existing_session`     | Narrow renewal inside an authenticated session. Embedded paths admissible only with an active `embedded_session_refresh` register entry.                  |
| `link_account`                 | Bounded account-link without widening scope. An embedded path requires an `embedded_password_exception` register entry whose `exception_scope_class` is `limited_account_link`. |
| `scope_grant`                  | Crosses a product-owned/native boundary. The interstitial MUST list the reserved native surfaces the confirm action lands on.                            |
| `switch_org_or_tenant`         | Crosses a product-owned/native boundary. The interstitial MUST disclose the current and target scope. A failed switch degrades visibly to a managed-blocked or local-safe state. |
| `admin_step_up`                | Crosses a product-owned/native boundary. The confirm action MUST land on a host-native admin step-up sheet.                                              |
| `credential_recovery`          | Routes through system-browser, device-code, credential-store unlock, or admin-assisted recovery. CAPTCHA-only and knowledge-question-only sole paths are forbidden. |
| `deprovision_acknowledge`      | Reviewer-only acknowledgement of a managed deprovision. The interstitial MUST quote the preserved-local-work block by ref and the offboarding/export path on the bound managed-session record. |

The schema enforces that `step_up_authority`, `admin_step_up`,
`scope_grant`, `switch_org_or_tenant`, and `deprovision_acknowledge`
all set `native_approval_boundary.product_owned_native_required = true`
and `confirm_action.host_native_target_required = true`. Any of those
classes routed through an embedded auth path is non-conforming.

## Availability disclosure rules

`availability_disclosure` renders the system-browser, device-code, and
passkey rows together so reviewers see the full honest posture. Rules:

1. Every interstitial MUST render every row, even when one path is
   `not_applicable`. Hiding a row is non-conforming.
2. `system_browser_availability` MUST be one of `supported`,
   `blocked_with_visible_recovery`, `unknown_with_disclosed_fallback`,
   or `policy_disabled` whenever the deployment supports a system
   browser. A deployment that cannot honour a system-browser path
   MUST disclose `unsupported_by_deployment` rather than silently
   advertising it.
3. `passkey_availability` MAY be `supported` only when the
   deployment, identity provider, platform, and policy support it. A
   surface that advertises `supported` without a backing deployment
   posture is non-conforming.
4. `device_code_availability` MUST be `supported` whenever the
   `auth_flow_class` is `device_code`; the schema enforces this. A
   `not_supported` row is admissible only when the IdP cannot speak
   device-code at all.
5. `preferred_flow_class` MUST be the `auth_flow_class` of this
   record. `fallback_flow_order` MUST list at least one ordered
   fallback that the deployment can honour.
6. `availability_summary_label` is the export-safe sentence support,
   admin, release-evidence, and docs/help surfaces quote. Surfaces
   MUST NOT mint local copy that disagrees with this label.

## Native-approval boundary linkage

`native_approval_boundary` is the row that prevents an embedded auth
page from ever rendering a final approval in disguise. Rules:

1. `product_owned_native_required = true` MUST be set when the
   requested action class crosses a product-owned/native boundary
   (step-up authority, admin step-up, scope grant, switch org or
   tenant, deprovision acknowledge). The schema enforces this.
2. `embedded_handoff_forbidden` MUST be `true` for `system_browser`,
   `device_code`, and `platform_authenticator_native` flows. The
   schema enforces this and also forbids quoting an
   `embedded_auth_exception_ref` on those flows.
3. `host_native_step_up_required` MUST be `true` whenever a
   subsequent host-native step-up sheet is required before any
   high-risk action lands. Required true for
   `embedded_password_exception`.
4. `reserved_native_surface_refs` MUST list every host-owned native
   surface the confirm action lands on. The closed set re-exports
   the ADR-0015 reserved-native-surface vocabulary.
5. `product_owned_review_surface_ref` is the opaque ref to the
   downstream native review surface. Null is admissible only when the
   action does not need a downstream native review (for example, a
   pure `sign_in_new_session` whose confirm action lands on the system
   browser and returns to the activity center, not to a native review).
6. The boundary summary label is reusable on the activity center,
   support packets, admin exports, release evidence, and docs/help
   diagnostics so the same string identifies the same handoff across
   surfaces.

## Confirm and reject actions

The interstitial carries exactly one `confirm_action` and one
`reject_action`. Both are typed.

`confirm_action.primary_action_class` is closed:

- `open_in_system_browser`
- `switch_to_device_code`
- `open_platform_authenticator`
- `open_host_native_step_up`
- `resume_after_step_up`
- `open_embedded_session_refresh`
- `open_embedded_password_exception`
- `open_admin_step_up`
- `open_deprovision_review`
- `no_confirm_admissible`

`reject_action.rejection_outcome_class` is closed:

- `continue_local_without_sign_in`
- `return_to_account_free_local`
- `request_admin_policy_change`
- `switch_to_device_code`
- `contact_support_with_export`
- `abort_pending_handoff_silently_forbidden`

Rules (frozen):

1. `confirm_action.host_native_target_required = true` is mandatory
   for any action whose requested-action class crosses a product-owned/
   native boundary, and for `embedded_password_exception` flows.
2. `confirm_action.browser_handoff_packet_ref` MUST point at an
   ADR-0010 `browser_handoff_packet` whenever the primary action class
   is `open_in_system_browser`. Null is admissible only for device-
   code, embedded-exception, host-native-only, and `no_confirm_admissible`
   confirms.
3. `reject_action.preserves_local_work_assertion` MUST be `true`
   whenever the bound preserved-local-work block declares a
   `local_work_intact*` posture. Rejecting a handoff never strands
   the user with broken local work as a side effect.
4. `abort_pending_handoff_silently_forbidden` exists only as the
   forbidden value: a surface that silently aborts the pending handoff
   instead of routing to a typed outcome MUST emit
   `auth_handoff_interstitial_denied_native_boundary` and fail closed.

## Embedded-auth exception register rules

The `embedded_auth_exception_register_record` is the auth-lane source
of truth for every approved embedded auth path. The host-rendered
`embedded_auth_exception_record` carried on a boundary card is a
mirror of this register entry.

Required register-entry fields:

- `register_entry_id` — opaque, stable id support, admin, release
  evidence, and docs/help diagnostics quote.
- `approved_embedded_flow_class` — `embedded_session_refresh` or
  `embedded_password_exception`. `system_browser`, `device_code`, and
  `platform_authenticator_native` are NOT admissible because they are
  the supported defaults and never need a register entry.
- `exception_reason` — exactly one of
  `legacy_provider_requires_embedded_password`,
  `managed_kiosk_or_browser_policy_blocks_launch`,
  `loopback_or_deep_link_return_unavailable`,
  `device_code_not_supported`,
  `session_refresh_without_password_only`, or
  `temporary_enterprise_override`.
- `exception_scope_class` — `session_refresh_only`,
  `limited_account_link`, or `no_high_risk_actions`.
- `allowed_provider_class`, `allowed_provider_label`,
  `allowed_provider_domain_label` — one register entry authorises
  exactly one provider domain. Wildcards are forbidden; a sibling
  domain is a new register entry.
- `applies_to_surface_families` — closed list, defaults to
  `embedded_auth_confirmation`.
- `applies_to_action_classes` — closed list of the requested-action
  classes the entry authorises. The schema forbids
  `step_up_authority`, `admin_step_up`, `scope_grant`,
  `switch_org_or_tenant`, and `deprovision_acknowledge` here.
- `fallback_rule` — preferred-flow order, fallback rule classes,
  `no_fallback_admissible` flag, and a reviewable summary label.
- `lower_trust_presentation` — required lower-trust cues, lower-trust
  badge requirement, host-native step-up requirement, visible
  capability limitations, presentation summary label, and (for
  password exceptions) the no-password-persistence flag.
- `host_native_approval_required` — required true for
  `embedded_password_exception`.
- `review_owner`, `review_state`, `review_at`, `review_expiry_at` —
  who reviewed, when, and when the entry expires.
- `policy_context`, `redaction_class`, `minted_at`.

Rules (frozen):

1. `embedded_password_exception` register entries MUST require a
   visible lower-trust badge, host-native step-up, and a no-password-
   persistence cue. The schema enforces this through an
   `if`/`then` branch.
2. `embedded_session_refresh` entries MUST set
   `exception_scope_class = session_refresh_only` and MUST include a
   `session_refresh_only_note` cue.
3. `applies_to_action_classes` MUST NOT include any class that crosses
   a product-owned/native boundary. The schema enforces this with a
   `not` constraint.
4. `review_state = revoked` MUST carry a revocation timestamp and a
   reviewable revocation reason label.
5. The auth-handoff interstitial MUST cite the register entry id when
   its `auth_flow_class` is `embedded_session_refresh` or
   `embedded_password_exception`. The schema enforces this on the
   interstitial side and rejects an embedded auth path that has no
   active register entry behind it.
6. The boundary card mirror is a render projection: when the register
   entry transitions from `active` to `expiring`, `expired`, or
   `revoked`, the host shell MUST downgrade every quoting boundary
   card to `host_owned_browser_only`, `host_owned_inspect_only`, or
   `no_permission_within_product` immediately. The card never
   silently keeps lower-trust authority alive after the register
   entry expires.

## Surface consumption rules

| Surface                                  | Required fields from the record(s)                                                                                                                                                                            | Forbidden shortcut                                                                                                       |
|------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------|
| **Auth-handoff interstitial sheet**      | `auth_flow_class`, `requested_action_class`, `provider_identity`, `callback_origin_disclosure`, `target_scope`, `expiry_replay_policy`, `availability_disclosure`, `native_approval_boundary`, `confirm_action`, `reject_action`, `preserved_local_work_summary`. | A "Sign in" button with no callback origin, scope, or fallback disclosure.                                              |
| **Activity center / durable attention**  | `interstitial_id`, `interstitial_state`, `requested_action_label`, `provider_domain_label`, `availability_summary_label`, `recovery_path` from the bound callback packet.                                                                       | Toast-only auth prompt with no durable row.                                                                              |
| **Boundary card (embedded auth surface)**| `embedded_auth_exception_ref`, `lower_trust_presentation.required_lower_trust_cues`, `applies_to_surface_families`, `fallback_rule.fallback_summary_label`.                                                                                       | Boundary card renders embedded auth without an active register entry.                                                    |
| **Support packet / support bundle**      | `interstitial_id`, `register_entry_id`, `auth_callback_packet_ref`, `managed_session_state_ref`, `support_export_refs`, `audit_event_refs`, `policy_context`, `redaction_class`.                                                                  | Collapsing the review object into an unstructured note.                                                                  |
| **Admin export / admin handoff**         | The full interstitial and (when present) the full register entry, including `policy_context`, `review_owner`, `review_state`, `review_expiry_at`.                                                                                                  | Replacing durable refs with prose-only summary.                                                                          |
| **Release evidence**                     | `interstitial_id`, `register_entry_id`, `auth_callback_packet_ref`, the bound `browser_handoff_packet_ref` (if any), and the matching `audit_event_refs`.                                                                                          | Minting evidence-local vocabulary that disagrees with the records.                                                       |
| **Docs/help diagnostics**                | `availability_summary_label`, `boundary_summary_label`, `fallback_summary_label`, `presentation_summary_label`, `expiry_summary_label`. The docs/help pane quotes by ref.                                                                          | Re-describing system-browser-first or embedded-exception copy locally per article.                                       |

Rules (frozen):

1. Surfaces quote fields by id; they do not re-describe callback
   origin, target scope, availability disclosure, or fallback rule in
   local prose.
2. Raw tokens, raw URLs, raw cookies, raw codes, raw nonces, raw PKCE
   verifiers, raw passkey material, raw passwords, and raw provider
   query strings MUST NOT appear on any surface that quotes the
   interstitial or the register entry.
3. Any surface that cannot resolve the interstitial or its bound
   register entry MUST fail closed (for example, to a typed
   `denied_native_boundary` chip with a repair hook) rather than
   rendering an unlabeled "Sign in" button.

## Audit and export rules

1. The interstitial emits typed audit events through the
   `audit_event_id` vocabulary on
   `auth_handoff_interstitial.schema.json`. The register emits its own
   typed audit events through the vocabulary on
   `embedded_auth_exception.schema.json`. Surfaces MUST NOT mint new
   audit-event ids locally.
2. Audit rows carry typed vocabulary only. Raw secret material, raw
   provider URLs, raw query strings, and raw error bodies MUST NOT
   appear.
3. Support export, admin export, and release evidence resolve the
   interstitial id and the register entry id once and quote the
   resolved labels by ref everywhere downstream. The docs/help
   diagnostics surfaces quote the same labels so a reader sees the same
   wording in the product, in support exports, and in release evidence.

## Out of scope at this revision

- Live OAuth / OIDC / SAML / device-code / passkey provider
  integrations and their wire protocols.
- Per-provider docs, marketplace, dashboard, or admin UI specifics
  beyond the closed vocabularies the interstitial and register quote.
- Managed-admin policy-bundle formats and policy-engine
  implementations.
- Per-OS browser launcher, deep-link broker, certificate store, or
  policy engine implementations themselves.

## Change control

- Adding a new requested-action class, callback-origin class,
  availability class, fallback rule class, lower-trust cue, capability
  limitation, interstitial-state class, denial reason, audit-event id,
  or review state is additive-minor and requires a schema, document,
  and fixture update in the same change.
- Repurposing an existing value is breaking and requires a new
  decision row co-signed by `security_trust_review` and the owning
  lane (workspace / identity / managed).
- Any change that alters the system-browser-first invariant, the
  native-approval boundary, the preserved-local-work quoting rule,
  the embedded-auth exception scope rules, or the callback-origin
  review rules MUST update this document, both schemas, and the
  adjacent system-browser callback packet, managed-authentication
  contract, ADR-0010, and ADR-0015 references in the same change.
