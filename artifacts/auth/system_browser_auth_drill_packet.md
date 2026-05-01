# System-browser auth drill packet

Reviewer-side drill packet for the system-browser-first auth lane.
This packet exists so normal callback, expired callback, wrong tenant,
wrong origin, stale approval, browser/device-code fallback, and passkey
available/unavailable behaviors are recorded as one inspectable trail
rather than as per-provider folklore.

The packet is review evidence. It does not mint vocabulary. Every axis
is re-exported from the upstream contracts and schemas:

- [`/docs/auth/auth_handoff_interstitial_contract.md`](../../docs/auth/auth_handoff_interstitial_contract.md)
  freezes the host-rendered interstitial review surface, the callback-
  origin review rules, the requested-action review rules, the
  availability disclosure rules, and the native-approval boundary
  linkage every drill case binds to.
- [`/docs/auth/system_browser_callback_packet.md`](../../docs/auth/system_browser_callback_packet.md)
  freezes the outbound system-browser handoff, return route, callback
  correlation, preserved-local-work block, recovery path, account-
  boundary class, and the four reserved rows (passkey_capable,
  reauth_required, seat_loss, deprovision_preserves_local_work).
- [`/docs/auth/managed_auth_and_session_continuity_contract.md`](../../docs/auth/managed_auth_and_session_continuity_contract.md)
  freezes managed-session state, the passkey-availability disclosure,
  and the reauth-requirement object the step-up cases bind to.
- [`/schemas/auth/auth_handoff_interstitial.schema.json`](../../schemas/auth/auth_handoff_interstitial.schema.json)
  defines the `auth_handoff_interstitial_record`, the closed
  `interstitial_state_class`, `denial_reason_class`, `availability_class`,
  `callback_origin_class`, `origin_verification_state`, and the
  `audit_event_id` vocabulary used below.
- [`/schemas/auth/auth_callback_state.schema.json`](../../schemas/auth/auth_callback_state.schema.json)
  defines `auth_callback_packet_record`,
  `pending_session_state_class`, `pending_session_denied_reason`, and
  the closed `retry_path_class` vocabulary the `safe_recovery_action`
  field on every drill row resolves to.

If this packet disagrees with those upstream sources, those sources win
and this packet plus its companion fixtures update in the same change.

## 1. Scope

A system-browser auth drill is the closed inspection a reviewer runs
when the host shell mints an `auth_handoff_interstitial_record`,
launches a system-browser handoff, falls back to device code, routes a
passkey step-up natively, or fails closed on a callback-origin / replay
/ tenant-or-workspace / native-boundary check. The drill answers, for
each case:

- which `auth_flow_class` and `requested_action_class` the
  interstitial declared,
- which `callback_origin_class` plus `origin_verification_state` the
  reviewer pre-saw before any state mutation,
- which `interstitial_state_class` the surface landed in (and, on a
  fail-closed row, which `denial_reason_class` was named),
- which `availability_disclosure` rows (system-browser, device-code,
  passkey) the reviewer was allowed to read,
- which typed `confirm_action.primary_action_class` and
  `reject_action.rejection_outcome_class` the reviewer could exercise,
- whether authority widened, narrowed, or stayed unchanged, and
- which `audit_event_id` closed the trail.

A drill is conforming when every case row resolves to one fixture
under
[`/fixtures/auth/callback_origin_skew_cases/`](../../fixtures/auth/callback_origin_skew_cases/)
or
[`/fixtures/auth/auth_handoff_cases/`](../../fixtures/auth/auth_handoff_cases/),
that fixture validates as one
`auth_handoff_interstitial_record`, and the drill row's expected
state, denial reason, safe recovery action, and authority-change class
all resolve to closed vocabularies in the upstream schemas.

## 2. Out of scope

- Live OAuth, OIDC, SAML, device-code, or passkey provider integrations
  and their wire protocols.
- Per-OS browser launcher, deep-link broker, certificate store, policy
  engine, or platform-authenticator adapter implementations.
- Final user-facing microcopy. The packet pins state classes, denial
  reasons, recovery action classes, and audit event ids; product
  writing chooses final strings inside those limits.
- New `auth_flow_class`, `requested_action_class`,
  `callback_origin_class`, `origin_verification_state`,
  `availability_class`, `interstitial_state_class`,
  `denial_reason_class`, `retry_path_class`, or `audit_event_id`.
  Every axis is re-exported; widening requires a decision row in the
  owning schema.

## 3. Closed authority-change vocabulary

Every drill case names exactly one `authority_change_class`. This
captures whether the reviewer's confirm/reject outcome — or the
fail-closed path — moved authority outward, narrowed it, or kept it as
it was at mint time.

| `authority_change_class` | When the case names it |
|---|---|
| `authority_unchanged_local_continuity_preserved` | Reject path, normal sign-in to a new session that did not yet bind elevated capability, or browser/device-code fallback that preserved the prior posture. Local edit, save, undo, search, local Git, local tasks, and BYOK AI keep working. |
| `authority_unchanged_managed_held_until_revalidation` | Wake-from-sleep, expired callback, or stale approval where the host shell holds managed authority over the privileged action until a typed reopen completes. Local work intact; managed capability narrowed visibly. |
| `authority_widened_inside_native_review_only` | Step-up authority, scope grant, switch org or tenant, admin step-up, or deprovision acknowledge confirm completed on a host-native review surface (`product_security_messaging`, `update_verification`, `workspace_trust_elevation`, `rollback_or_restore_confirmation`, `ai_apply_review`, `high_risk_approval_sheet`). No embedded surface, OS notification, or companion shortcut delivered the final approval. |
| `authority_narrowed_managed_only_revoked` | Tenant or workspace mismatch, exception register revoked, or policy_blocked path narrowed managed capability to local-safe rows. Local work stays intact. |
| `authority_narrowed_visible_recovery_required` | System-browser blocked, device-code unavailable, or passkey unavailable on this deployment. The card narrows the path with a typed `retry_path_class` recovery instead of widening into an embedded substitute. |
| `authority_denied_failed_closed_no_widening` | Origin mismatch, replay denial, native-boundary violation, or expiry/replay policy violation that fails closed. The interstitial never widens; preserved-local-work posture remains intact. |

A row that needs an outcome outside the closed set opens a decision
row against
[`/docs/auth/auth_handoff_interstitial_contract.md`](../../docs/auth/auth_handoff_interstitial_contract.md)
instead of widening this table.

## 4. Required drill case classes

Every release evidence pack and support packet that cites this drill
packet MUST include at least one row per case class below. A claim
that is narrower (for example, a deployment that cannot speak device
code at all) records the row as `not_applicable` with a typed reason
rather than omitting it.

| Drill case class | `auth_flow_class` | Default `interstitial_state_class` | Required upstream fixture kind | Notes |
|---|---|---|---|---|
| `normal_callback_system_browser_first` | `system_browser` | `confirmed` | `auth_handoff_interstitial_record` + `auth_callback_packet_record` | Loopback-port-pinned origin, `verified` verification state, `confirm_action.primary_action_class = open_in_system_browser`, audit closes with `auth_handoff_interstitial_confirmed`. |
| `expired_callback_reopen_required` | `system_browser` | `expired_before_review` | `auth_handoff_interstitial_record` | Pending review aged past `expiry_replay_policy.expires_at` (for example, the user returned from sleep). Denial reason `expiry_or_replay_policy_violation`; authority held until the reviewer reopens. |
| `wrong_tenant_authority_narrowed` | `system_browser` | `denied_native_boundary` or `rejected_by_policy` | `auth_handoff_interstitial_record` | Returning browser state names a different tenant than the bound `target_scope.bound_tenant_or_org_ref`. Denial reason `tenant_or_workspace_mismatch`; managed authority narrows to local-safe rows. |
| `wrong_origin_callback_origin_mismatch` | `system_browser` | `denied_origin_mismatch` | `auth_handoff_interstitial_record` | Loopback port, deep-link scheme, or host drift between mint and return. Denial reason `callback_origin_mismatch`; preserved-local-work intact, no silent retry. |
| `stale_approval_replay_denied` | `system_browser` or `platform_authenticator_native` | `denied_replay` | `auth_handoff_interstitial_record` | Approval ticket past TTL, callback receipt already redeemed, or interstitial replayed after a partial confirm. Denial reason `callback_replay_denied`; audit closes with `auth_handoff_interstitial_denied_replay`. |
| `browser_blocked_device_code_fallback` | `device_code` | `pending_review` then `confirmed` | `auth_handoff_interstitial_record` + `device_code` companion ref | System browser is `blocked_with_visible_recovery` or `policy_disabled`. Preferred flow flips to `device_code`, `code_expiry_label` renders, audit closes with `auth_handoff_interstitial_routed_to_device_code`. |
| `passkey_available_step_up_native` | `platform_authenticator_native` | `confirmed` | `auth_handoff_interstitial_record` + `managed_session_state_record` | Step-up authority routed to the host-native step-up sheet via `open_platform_authenticator`. Authority widens only inside the native review surface; audit closes with `auth_handoff_interstitial_routed_to_native_approval`. |
| `passkey_unavailable_system_browser_fallback` | `system_browser` | `pending_review` then `confirmed` | `auth_handoff_interstitial_record` | `passkey_availability` is `unsupported_by_platform_or_authenticator` or `policy_disabled`. The interstitial keeps system-browser preferred, names the fallback explicitly, and never advertises an embedded passkey substitute. |

## 5. Per-case drill row

Reviewers fill one block per case row from §4. Free text is allowed
only in the `notes` field; every other field resolves to a closed
vocabulary or a stable opaque ref.

```yaml
- case_id: system-browser-auth-case:<short-slug>
  drill_case_class: <one of §4>
  auth_flow_class: <system_browser|device_code|platform_authenticator_native|embedded_session_refresh|embedded_password_exception|not_applicable>
  requested_action_class: <sign_in_new_session|resume_existing_session|step_up_authority|refresh_existing_session|link_account|scope_grant|switch_org_or_tenant|admin_step_up|credential_recovery|deprovision_acknowledge>
  callback_origin_disclosure:
    callback_origin_class: <loopback_http_origin|platform_deep_link_origin|device_code_poll_origin|manual_resume_origin|platform_authenticator_native_origin|not_applicable>
    origin_verification_state: <verified|unverified|certificate_failed|policy_blocked|cross_origin_limited|offline_cached|not_applicable>
  expected_interstitial_state: <pending_review|confirmed|rejected_by_user|rejected_by_policy|expired_before_review|superseded|denied_origin_mismatch|denied_replay|denied_native_boundary>
  expected_denial_reason: <callback_origin_mismatch|callback_replay_denied|tenant_or_workspace_mismatch|embedded_handoff_without_exception|embedded_handoff_for_high_risk_action|system_browser_blocked_no_visible_fallback|device_code_unavailable_no_visible_fallback|passkey_unavailable_no_visible_fallback|native_approval_boundary_violation|expiry_or_replay_policy_violation|exception_register_entry_revoked_or_expired> | null
  availability_disclosure:
    system_browser_availability: <availability_class>
    device_code_availability: <availability_class>
    passkey_availability: <availability_class>
    preferred_flow_class: <auth_flow_class>
  upstream_fixture_refs:
    auth_handoff_interstitial_record_ref: <fixtures/auth/auth_handoff_cases/...> | <fixtures/auth/callback_origin_skew_cases/...>
    auth_callback_packet_record_ref: <fixtures/auth/callback_and_lock_state_cases/...> | null
    managed_session_state_record_ref: <fixtures/auth/managed_session_cases/...> | null
    embedded_auth_exception_register_record_ref: <fixtures/auth/auth_handoff_cases/...> | null
  canonical_backing_refs:
    interstitial_id_ref: <auth-handoff-interstitial:...>
    auth_callback_packet_ref: <auth-callback-packet:...> | null
    browser_handoff_packet_ref: <browser-handoff-packet:...> | null
    device_code_ref: <device-code-companion:...> | null
    embedded_auth_exception_ref: <embedded-auth-exception-register:...> | null
  policy_context:
    identity_mode: <account_free_local|self_hosted_org|managed_workspace>
    policy_epoch: <policy-epoch:...>
    trust_state: <trusted|restricted>
  authority_change_class: <one of §3>
  safe_recovery_action_class: <retry_in_system_browser|switch_to_device_code|resume_after_step_up|resume_after_credential_store_unlock|request_admin_policy_change|continue_local_without_sign_in|import_signed_session_snapshot|return_to_account_free_local|contact_support_with_export|no_recovery_without_superseding_action>
  preserved_local_work_posture: <local_work_intact|local_work_intact_with_managed_narrowed|local_work_intact_with_self_hosted_narrowed|local_work_narrowed_by_workspace_trust|local_work_blocked_by_policy>
  must_not_happen:
    - <one assertion tied to silent retry, embedded fallback, hidden focus steal, callback widening, or authority spread>
  audit_event_id: <one of auth_handoff_interstitial_rendered|auth_handoff_interstitial_confirmed|auth_handoff_interstitial_rejected|auth_handoff_interstitial_expired|auth_handoff_interstitial_superseded|auth_handoff_interstitial_denied_origin|auth_handoff_interstitial_denied_replay|auth_handoff_interstitial_denied_native_boundary|auth_handoff_interstitial_routed_to_native_approval|auth_handoff_interstitial_routed_to_device_code|auth_handoff_interstitial_exception_register_quoted>
  notes: >
    <free-text reviewer rationale; closed-vocab fields above are the
    actual record of truth>
```

## 6. Conformance assertions

Every system-browser auth drill case is non-conforming unless ALL of
the assertions below hold. Tooling and reviewers compare against this
list; new assertions require a decision row.

1. **Callback origin pre-disclosed.** The reviewer reads
   `callback_origin_class`, `callback_origin_label`,
   `host_or_domain_label`, `return_anchor_label`, and
   `origin_verification_state` before any state mutation. A row whose
   reviewer never saw the origin fails closed regardless of the IdP
   outcome.
2. **Origin never widens between mint and confirm.** A surface that
   re-renders with a different `callback_origin_disclosure` without
   bumping `interstitial_id` is non-conforming. Origin drift between
   mint and return MUST transition to `denied_origin_mismatch`.
3. **Replay denied closes the trail.** Returning state replayed past a
   single redemption MUST land on `denied_replay` with denial reason
   `callback_replay_denied` and audit event
   `auth_handoff_interstitial_denied_replay`. Silent re-prompt with a
   freshly minted policy after rejection is forbidden.
4. **Tenant or workspace mismatch fails closed.** Returning state
   bound to a different tenant or workspace than
   `target_scope.bound_tenant_or_org_ref` /
   `bound_workspace_ref` MUST fail closed with denial reason
   `tenant_or_workspace_mismatch` and MUST NOT silently reopen the
   bound workspace.
5. **No silent embedded fallback.** When system-browser is blocked or
   device-code is unavailable, the row MUST present a typed
   `retry_path_class` (for example, `switch_to_device_code`,
   `request_admin_policy_change`, `continue_local_without_sign_in`)
   instead of routing into an embedded webview without an active
   register entry.
6. **Availability disclosed honestly.** `availability_disclosure`
   renders all three rows (system-browser, device-code, passkey) on
   every interstitial. A path the deployment cannot honour MUST be
   `unsupported_by_deployment` or `unsupported_by_platform_or_authenticator`,
   not `supported`. `preferred_flow_class` matches the
   `auth_flow_class` on the record.
7. **Native approval stays native.** When the requested action class
   crosses a product-owned/native boundary
   (`step_up_authority`, `admin_step_up`, `scope_grant`,
   `switch_org_or_tenant`, `deprovision_acknowledge`),
   `native_approval_boundary.product_owned_native_required` is `true`,
   `confirm_action.host_native_target_required` is `true`, and the
   confirm action lands on a `reserved_native_surface_refs` value, not
   on an embedded body, OS notification, or companion shortcut.
8. **Preserved local work intact on every fail-closed row.**
   `preserved_local_work_summary.posture_class` resolves to one of
   `local_work_intact`, `local_work_intact_with_managed_narrowed`, or
   `local_work_intact_with_self_hosted_narrowed`. A drill row that
   strands the user with broken local edit / save / undo / search /
   local Git / local tasks / BYOK AI is non-conforming.
9. **Reject lands on a typed outcome.** The reject path resolves to
   one of `continue_local_without_sign_in`,
   `return_to_account_free_local`, `request_admin_policy_change`,
   `switch_to_device_code`, or `contact_support_with_export`.
   `abort_pending_handoff_silently_forbidden` exists only as the
   forbidden value.
10. **Audit row attributable.** A typed `audit_event_id` and the
    interstitial id, callback packet ref (when present), browser
    handoff packet ref (when present), device-code ref (when present),
    and embedded exception ref (when present) close the trail.
    Raw tokens, raw provider URLs, raw cookies, raw codes, raw
    nonces, raw PKCE verifiers, raw passkey material, raw passwords,
    and raw provider query strings never appear.
11. **Reviewer can distinguish origin drift from tenant drift from
    expiry drift from native-boundary drift.** Each row's
    `expected_interstitial_state` plus `expected_denial_reason` is
    enough to tell which class of drift fired without rereading
    upstream prose.

## 7. Reuse rules

A system-browser auth drill packet is reusable in release evidence,
support packets, admin exports, and docs/help diagnostics when:

- every required case class from §4 is filled with a row that resolves
  to an upstream fixture ref under
  [`/fixtures/auth/callback_origin_skew_cases/`](../../fixtures/auth/callback_origin_skew_cases/)
  or
  [`/fixtures/auth/auth_handoff_cases/`](../../fixtures/auth/auth_handoff_cases/);
- every row's `authority_change_class` is one of the closed six in
  §3;
- every row's `safe_recovery_action_class` is one of the closed
  `retry_path_class` values from
  `schemas/auth/auth_callback_state.schema.json#/$defs/retry_path_class`;
- every row's `audit_event_id` is one of the closed eleven on
  `schemas/auth/auth_handoff_interstitial.schema.json#/$defs/audit_event_id`;
- the conformance assertions in §6 hold for every row;
- the packet cites
  [`/artifacts/auth/native_approval_boundary_review.yaml`](./native_approval_boundary_review.yaml)
  for any row whose requested action class crosses the product-owned/
  native boundary; and
- the packet cites
  [`/artifacts/auth/account_boundary_examples/`](./account_boundary_examples/)
  for the local-only / self-hosted / managed / restricted-managed-only
  / grace-degraded posture rows referenced by drill cases.

## 8. Companion artifacts

| Artifact | Role |
|---|---|
| [`/fixtures/auth/callback_origin_skew_cases/`](../../fixtures/auth/callback_origin_skew_cases/) | Worked callback-origin skew cases, one per row in §4 plus paired widen/narrow variants. Each fixture validates as one `auth_handoff_interstitial_record`. |
| [`/fixtures/auth/auth_handoff_cases/`](../../fixtures/auth/auth_handoff_cases/) | Upstream interstitial / register fixtures the normal-callback, device-code-fallback, passkey-step-up, scope-grant rejected, and origin-mismatch denied cases bind to. |
| [`/fixtures/auth/callback_and_lock_state_cases/`](../../fixtures/auth/callback_and_lock_state_cases/) | Upstream `auth_callback_packet_record` fixtures the drills cite by ref for the bound callback packet. |
| [`/fixtures/auth/managed_session_cases/`](../../fixtures/auth/managed_session_cases/) | Upstream `managed_session_state_record` fixtures the resume / step-up / refresh / org-switch / deprovision rows bind to. |
| [`/artifacts/auth/native_approval_boundary_review.yaml`](./native_approval_boundary_review.yaml) | Closed review packet showing how high-risk approvals re-enter native surfaces instead of completing inside webviews, OS notifications, or companion shortcuts. |
| [`/artifacts/auth/account_boundary_examples/`](./account_boundary_examples/) | Closed account-boundary fixture corpus (local-only, self-hosted, managed, restricted-managed-only, grace-degraded-managed) the drill rows quote by id. |
| [`/docs/auth/auth_handoff_interstitial_contract.md`](../../docs/auth/auth_handoff_interstitial_contract.md) | Upstream interstitial, register, and callback-origin review contract. |
| [`/docs/auth/system_browser_callback_packet.md`](../../docs/auth/system_browser_callback_packet.md) | Upstream callback packet, return-route, account-boundary, and credential-store lock-state contract. |
| [`/docs/auth/managed_auth_and_session_continuity_contract.md`](../../docs/auth/managed_auth_and_session_continuity_contract.md) | Upstream managed-session contract the resume / step-up / refresh / org-switch / deprovision drill rows bind to. |

## 9. Review checklist

A change touching a system-browser auth drill is conforming only if a
reviewer can answer:

1. Which `auth_flow_class` and `requested_action_class` did the
   interstitial declare, and which fixture under
   `/fixtures/auth/callback_origin_skew_cases/` or
   `/fixtures/auth/auth_handoff_cases/` records the drill?
2. Which `callback_origin_class` plus `origin_verification_state` did
   the reviewer pre-see, and which `interstitial_state_class` did the
   surface land on?
3. On a fail-closed row, which `denial_reason_class` was named, and
   which closed `audit_event_id` closed the trail?
4. What was the `availability_disclosure` posture (system-browser /
   device-code / passkey) on this row, and did the row advertise only
   paths the deployment can honour?
5. Which `safe_recovery_action_class` did the row offer, and did it
   resolve to one of the closed `retry_path_class` values without
   silently routing through an embedded webview, OS notification, or
   companion shortcut?
6. Which `authority_change_class` from §3 did the row land on, and
   does the bound `preserved_local_work_summary.posture_class`
   confirm that local edit / save / undo / search / local Git / local
   tasks / BYOK AI remained intact?
7. For step-up / scope-grant / switch-org-or-tenant / admin-step-up /
   deprovision-acknowledge rows: which entry in
   [`/artifacts/auth/native_approval_boundary_review.yaml`](./native_approval_boundary_review.yaml)
   names the host-native review surface the confirm action landed on,
   and the forbidden surrogate paths the row denied?
