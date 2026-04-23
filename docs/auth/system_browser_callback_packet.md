# System-browser auth callback, credential-store lock state, and local-versus-managed account-boundary packet

This document freezes the shared packet vocabulary Aureline uses for
outbound system-browser auth handoffs, returning browser callbacks,
credential-store lock states, and the local-versus-self-hosted-versus-
managed account boundary. It exists so the first auth lanes land on one
object model instead of each surface inventing its own `Connected`
state, its own retry copy, its own "what still works offline" clause,
and its own idea of whether a callback belongs to the pending session.

Companion artifacts:

- [`/schemas/auth/auth_callback_state.schema.json`](../../schemas/auth/auth_callback_state.schema.json)
  — machine-readable boundary for `auth_callback_packet_record`,
  `credential_store_lock_state_record`, and `account_boundary_record`.
- [`/fixtures/auth/callback_and_lock_state_cases/`](../../fixtures/auth/callback_and_lock_state_cases/)
  — worked examples for account-free local mode, managed sign-in-
  required mode, restricted-managed-only posture, callback failure on
  origin mismatch, credential-store locked on launch, and returning from
  the browser into the bound workspace.
- [`/artifacts/auth/account_boundary_examples/`](../../artifacts/auth/account_boundary_examples/)
  — account-boundary fixture corpus exercising local-only, self-hosted,
  managed, restricted-managed-only, and grace-degraded-managed posture
  with all four reserved rows.
- [`/docs/adr/0001-identity-modes.md`](../adr/0001-identity-modes.md),
  [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md),
  [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md),
  [`/docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`](../adr/0015-embedded-surface-boundary-and-auth-handoff.md)
  — upstream contracts this packet rides on. This document does not
  redefine identity modes, the secret broker, the browser-handoff
  packet, or the embedded-surface boundary; it defines the callback and
  lock-state objects those contracts refer to.

Normative sources this packet projects from:

- `.t2/docs/Aureline_PRD.md` local-first posture, account-free default,
  and system-browser-first auth rows.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §22.6.1
  (provider-handoff), §22.8 (secret broker), and the offline/continuity
  appendices.
- `.t2/docs/Aureline_Technical_Design_Document.md` §7.11.13 and the
  auth/sign-in verification seed.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Appendix AL and the
  system-browser-first auth sheet guidance.

If this document disagrees with those sources, those sources win and
this document plus the schema update in the same change.

## Why this exists

Local-first posture is only real if the auth lane is built around it.
Without one governed packet, every later lane can silently turn every
code path into an account-required experience:

- a signed-in-now banner would claim managed capability even when the
  managed service is unreachable;
- a callback banner would accept a returning browser tab without
  verifying that it belongs to the pending session, the originating
  workspace, or the bound tenant;
- a "sign in" button would quietly fall back to an embedded webview
  when system-browser launch is blocked;
- a credential-store outage would surface as a generic "something went
  wrong" toast and strand the user with no visible recovery copy;
- support packets, release evidence, admin exports, and
  mutation-journal entries would each quote a different retry path
  vocabulary; and
- seat-loss or deprovision would silently narrow or block local work
  that the PRD guarantees stays usable.

This packet closes that gap before the first real OAuth, device-code,
or passkey integration lands. Live provider adapters remain out of
scope at this revision; the vocabulary and invariants below are what
those integrations will honour.

## One packet, three record kinds

The schema freezes three record kinds in one file:

| Record kind                           | Purpose                                                                                                                                                                                       |
|---------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `auth_callback_packet_record`         | Outbound system-browser handoff, return route, callback correlation, preserved-local-work block, and recovery path for one pending sign-in session.                                           |
| `credential_store_lock_state_record`  | Credential-store posture (available / locked / denied / unavailable / degraded / policy_blocked) with export-safe copy catalog, typed retry path, and retained / blocked local capabilities. |
| `account_boundary_record`             | Local-versus-self-hosted-versus-managed account-boundary posture with visible downgrade path and the four reserved rows later managed-auth work extends.                                     |

All three records deliberately share:

- the identity-mode vocabulary (`account_free_local`, `self_hosted_org`,
  `managed_workspace`) re-exported from ADR-0001;
- the deployment-profile scope vocabulary;
- the preserved-local-work block;
- the recovery / retry-path vocabulary;
- the four reserved rows (`passkey_capable`, `reauth_required`,
  `seat_loss`, `deprovision_preserves_local_work`); and
- the export-safe label discipline (raw tokens, raw URLs, raw cookies,
  raw provider codes, raw PKCE verifiers, and raw query strings never
  appear).

That is the mechanism that keeps banners, activity-center rows,
support packets, release evidence, and admin exports on one artifact
identity graph.

## Browser-launch posture (frozen)

System-default-browser use is the **default** and required posture for
any supported auth row. The schema exports a closed
`browser_launch_policy_class` vocabulary:

| Class                                     | Meaning                                                                                                                                                          |
|-------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `system_default_browser_required`         | The supported default. The system-default browser launches the outbound handoff; embedded webviews are never a silent fallback.                                  |
| `managed_approved_browser_allowed`        | Narrow case where a managed policy pins a specific approved browser binary over the system default. Still a real browser; still never an embedded webview.      |
| `separately_approved_boundary_contract`   | Admissible only when a separately approved, co-signed boundary contract replaces the system-browser default (for example, a platform-native auth agent surface). |
| `browser_launch_policy_blocked`           | Visible-recovery fail-closed state. Surfaces must render the reason and offer a typed retry path (device code, manual return, continue local without sign-in).   |

Rules (frozen):

1. `system_browser` auth rows MUST use one of the four classes above
   and MUST declare `embedded_fallback_posture = embedded_fallback_forbidden`.
   Silent fallback to an embedded webview is not admissible.
2. `managed_approved_browser_allowed` is not a shortcut. It requires
   the managed policy to have a co-signed browser-approval row; without
   one, the surface fails closed to `browser_launch_policy_blocked`.
3. `browser_launch_policy_blocked` requires
   `recovery_path.visible_recovery_required = true` and at least one
   typed fallback action (`switch_to_device_code`,
   `request_admin_policy_change`, `continue_local_without_sign_in`).
4. `platform_authenticator_native` (passkey / WebAuthn / platform-native
   step-up) is a host-native or browser/platform surface, not a
   general-purpose embedded auth page. Embedded-fallback posture MUST
   remain `embedded_fallback_forbidden`.

## Callback-correlation envelope (frozen)

Every outbound handoff binds a `callback_correlation` envelope that
makes the returning browser state verifiable:

| Field                         | Purpose                                                                                                                                             |
|-------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------|
| `correlation_id`              | Opaque id safe to log.                                                                                                                              |
| `pending_session_id`          | Opaque id for the pending sign-in session.                                                                                                          |
| `state_token_alias`           | Alias for the state token. Raw bytes never cross this boundary.                                                                                     |
| `nonce_alias`                 | Alias for the nonce bound into the outbound request.                                                                                                |
| `pkce_challenge_alias`        | Alias for the PKCE challenge / verifier. Null only when the flow does not use PKCE (for example, device-code poll return).                           |
| `bound_workspace_ref`         | Originating Aureline workspace the session is bound to. Null only for pre-workspace sign-in.                                                         |
| `bound_tenant_or_org_ref`     | Tenant / org scope the session is bound to. Null only for account-free local or unbound surfaces.                                                    |
| `bound_actor_subject_ref`     | Pre-bound acting subject, when applicable.                                                                                                           |
| `issued_at` / `expires_at`    | Monotonic issue and expiry.                                                                                                                         |
| `replay_posture`              | `single_use` or `denied_on_any_replay`.                                                                                                              |
| `redeemed_at`                 | Populated on first successful redemption. Null while pending, denied, or failed.                                                                     |
| `callback_receipt_ref`        | Stable id later quoted by support packets, admin exports, release evidence, and mutation-journal entries.                                            |

Rules (frozen):

1. Callback redemption MUST fail closed with a typed audit event
   (`auth_callback_replay_denied`,
   `auth_callback_origin_mismatch_denied`,
   `auth_callback_tenant_or_workspace_mismatch_denied`,
   `auth_callback_embedded_fallback_denied`) when the returning state
   does not match the pending correlation.
2. A callback MUST NOT mutate local state before the origin, replay,
   tenant / workspace, and policy checks defined on
   `return_route` and `callback_correlation` pass.
3. Correlation aliases are export-safe; raw tokens, raw PKCE verifiers,
   raw nonces, and raw provider query strings never appear on any
   surface that quotes the packet.
4. Supersedence is explicit: a newer outbound handoff that invalidates
   a pending session transitions the old packet to
   `session_superseded`, never to silent deletion.

## Return route (frozen)

`return_route` names the mode, anchor, target label, origin-validation
class, tenant / workspace match rule, and policy-check refs the callback
must satisfy:

| Field                                     | Meaning                                                                                                                                |
|-------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------|
| `return_mode_class`                       | `loopback_http_return`, `platform_deep_link_return`, `device_code_poll_return`, `manual_return_resume`, `not_applicable`.              |
| `return_anchor_ref`                       | Stable anchor the user returns to; round-trips independently of provider query-string behavior.                                         |
| `return_target_label`                     | Export-safe label ("Aureline desktop – payments-prod workspace"). Raw URLs MUST NOT appear.                                             |
| `return_origin_validation_class`          | `strict_origin_match_required`, `loopback_port_pinned`, `deep_link_scheme_pinned`, `device_code_poll_only`, `manual_resume_only`.       |
| `return_tenant_or_workspace_match_rule`   | Which binding a returning browser state must match before callback mutation: bound workspace + tenant, bound tenant, bound workspace, or no binding (account-free local only). |
| `return_policy_check_refs`                | Typed policy-check rows the return path must satisfy. Empty is admissible for `account_free_local`.                                    |

Rules (frozen):

1. `no_tenant_or_workspace_binding` is admissible **only** for
   pre-workspace `account_free_local` sign-in. Every other flow MUST
   bind at least one of workspace or tenant and validate the binding on
   return.
2. The returning browser MUST NOT reopen a different tenant or
   workspace context than the pending session was bound to. A mismatch
   fails closed with `auth_callback_tenant_or_workspace_mismatch_denied`
   and visible recovery copy.
3. `manual_return_resume` is a last-resort path. When admissible, the
   surface MUST keep preserved-local-work visible and route to a typed
   repair hook rather than strand the user.

## Preserved-local-work block (frozen)

Every packet carries one `preserved_local_work` block:

- `posture_class` — one of `local_work_intact`,
  `local_work_intact_with_managed_narrowed`,
  `local_work_intact_with_self_hosted_narrowed`,
  `local_work_narrowed_by_workspace_trust`,
  `local_work_blocked_by_policy`.
- `note` — short, reviewable sentence.
- `retained_capabilities[]` — export-safe labels for what still works
  locally while sign-in is incomplete, failed, or paused. Editing,
  save, undo, search, local Git, local tasks, local model execution,
  and BYOK AI MUST remain representable without implying network or
  account dependency.
- `blocked_capabilities[]` — what is narrowed; empty is admissible for
  `account_free_local`.

Rules (frozen):

1. Local work is product truth, not optional documentation. Banners,
   activity-center rows, support packets, release evidence, and admin
   exports quote this block by reference; they do not re-describe it
   locally.
2. `local_work_blocked_by_policy` is admissible only as a visibly-
   degraded fail-closed state with a typed repair hook. It is never a
   silent default.
3. `account_free_local` identity mode MUST carry
   `local_work_intact` unless workspace trust narrows a specific local
   capability, in which case `local_work_narrowed_by_workspace_trust`
   is the only other admissible value.

## Recovery path and retry vocabulary (frozen)

Every packet carries one `recovery_path`:

| Field                         | Meaning                                                                                                                                |
|-------------------------------|----------------------------------------------------------------------------------------------------------------------------------------|
| `primary_recovery_action`     | One of the closed `retry_path_class` values.                                                                                           |
| `fallback_recovery_actions[]` | Typed fallback actions in order.                                                                                                       |
| `visible_recovery_required`   | `true` for any denied / failed / expired / superseded callback and for any non-available credential-store lock state.                  |
| `recovery_copy_label`         | Export-safe label the banner, activity-center, support packet, and admin-export surfaces quote verbatim.                                |
| `repair_hook_ref`             | Optional typed repair hook for diagnostics / Project Doctor flows.                                                                     |

Closed `retry_path_class` set:

- `retry_in_system_browser`
- `switch_to_device_code`
- `resume_after_step_up`
- `resume_after_credential_store_unlock`
- `request_admin_policy_change`
- `continue_local_without_sign_in`
- `import_signed_session_snapshot`
- `return_to_account_free_local`
- `contact_support_with_export`
- `no_recovery_without_superseding_action`

Rules (frozen):

1. Denial states MUST name a typed retry path. Silent retry, silent
   downgrade, and silent embedded fallback are forbidden.
2. `continue_local_without_sign_in` is always available as a fallback
   whenever the identity mode or boundary class admits local continuity.
3. `no_recovery_without_superseding_action` is admissible only for
   `session_superseded` and for credential-store denials where the only
   valid path is a later policy, rotation, or admin action.

## Credential-store lock-state model (frozen)

`credential_store_lock_state_record` exports the closed
`credential_store_lock_state_class` set:

| State             | Meaning                                                                                                                                                             |
|-------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `available`       | Preferred store is unlocked and usable.                                                                                                                             |
| `locked`          | Store known and present; not currently unlocked. Surface offers unlock.                                                                                             |
| `denied`          | ADR-0007 typed broker denial refused the request (class rule, consumer capability mismatch, tool-reveal denial, remote scope mismatch, approval ticket missing).    |
| `unavailable`     | Store reachable-but-broken, corrupted, or missing.                                                                                                                  |
| `degraded`        | `degraded_session_only` session-memory-cache fallback active. Non-signing classes only; visible badge; no persistent upgrade; expires on process / trust / policy event. |
| `policy_blocked`  | Policy bundle or profile enforcement denies the class / store / operation at this policy epoch.                                                                     |

Required `copy_catalog` fields (all export-safe, quoted by reference):

- `primary_headline_label`
- `plain_language_summary`
- `retry_action_label`
- `secondary_action_label`
- `export_safe_status_label`

Rules (frozen):

1. `denied` and `policy_blocked` MUST carry a typed
   `denial_reason_class` from the ADR-0007 denial-reason set.
2. `degraded` MUST declare a `session_fallback_posture` row naming
   whether session-memory-cache fallback is allowed for this class
   (never for signing / device / provider-session classes).
3. Non-available lock states MUST carry
   `recovery_path.visible_recovery_required = true` and a
   reviewable `recovery_copy_label`.
4. The lock state MUST NOT gate local editing, save, undo, search,
   local Git, local tasks, or BYOK AI unless the underlying secret
   class is required for that specific capability. A locked OS
   keychain does not block a local-only edit.
5. Lock-state records name which pending
   `auth_callback_packet_record` ids they narrow or invalidate via
   `affects_pending_callback_refs`.

## Account-boundary packet (frozen)

`account_boundary_record` exports the closed `account_boundary_class`
set:

- `local_only` — no account or network required; default under
  `account_free_local`.
- `self_hosted` — customer-run IdP over the system browser; policy as
  signed bundles.
- `managed` — vendor-hosted convenience layer on top of the
  `self_hosted_org` protocols.
- `restricted_managed_only` — managed posture where a specific org
  policy narrows the surface to managed-sign-in-required for that
  capability. Local-only work stays truthful.
- `grace_degraded_managed` — bounded posture when managed services are
  unreachable but local and self-hosted capability stay truthful.
- `unknown_boundary` — fail-closed state routed to repair.

Required fields:

- `boundary_label` — export-safe label (never ambiguous free text).
- `plain_language_summary` — short reviewable sentence.
- `visible_downgrade_path` — typed row with
  `downgrade_to_boundary_class`, `downgrade_trigger_class`
  (`managed_service_unreachable`, `self_hosted_idp_unreachable`,
  `seat_removed`, `org_membership_lost`, `policy_narrowed`,
  `credential_store_locked`, `callback_failure`, `not_applicable`),
  `downgrade_note`, and `preserves_local_work` boolean.
- `preserved_local_work` — the shared block.
- `reserved_rows[]` — at least one entry for each reserved row kind.

Rules (frozen):

1. `account_free_local` identity mode MUST pin `boundary_class =
   local_only` and `visible_downgrade_path.preserves_local_work = true`.
2. Every downgrade path MUST preserve local work unless the PRD
   explicitly allows a narrower posture. The schema enforces this for
   `account_free_local`; every other mode SHOULD preserve local work
   and document the narrow exception.
3. `unknown_boundary` is admissible only as a fail-closed state; it
   MUST route to a repair hook and MUST NOT render as an ambiguous
   `Connected` badge.

## Reserved rows (frozen)

Every callback packet and account-boundary record reserves the
following row kinds so later managed-auth work extends the shared
packet instead of replacing it. Each row names its kind, state,
applicable boundary classes, optional `preserves_local_work` boolean,
optional repair hook ref, and a reviewable note.

| Row kind                              | Purpose                                                                                                                                               |
|---------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------|
| `passkey_capable`                     | Declares that the flow can step up with a platform authenticator without leaving the system-browser-first posture.                                    |
| `reauth_required`                     | The state after session expiry, password reset, policy-epoch roll, or trust downgrade.                                                                |
| `seat_loss`                           | Managed entitlement is lost (license seat removed, org membership lost). Local-only work must still be representable.                                  |
| `deprovision_preserves_local_work`    | Durable guarantee that local workspace, local Git, local tasks, and BYOK AI keep working after managed deprovision. `preserves_local_work` MUST be `true`. |

Admissible `row_state` values:

- `reserved_for_future_row` — default seed posture.
- `supported_capability_declared`
- `active_state`
- `not_applicable_in_this_posture`

Rules (frozen):

1. Reserved rows MUST cover all four row kinds. The schema enforces
   this with `contains` constraints.
2. `deprovision_preserves_local_work` MUST assert
   `preserves_local_work = true`. Schemas and fixtures reject the
   opposite.
3. Active or expiring reserved rows SHOULD carry a `repair_hook_ref`
   so support / Project Doctor / admin-export surfaces resolve a shared
   repair object instead of minting local copy.

## Surface-consumption rules

| Surface                                  | Required fields from the packet                                                                                                                         | Forbidden shortcut                                                                                      |
|------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------|
| **Auth sheet banner**                    | `auth_flow_class`, `browser_launch_policy_class`, `provider_domain_label`, `pending_session_state`, `preserved_local_work.note`, `recovery_path`         | Inventing a local `is_signed_in` boolean or collapsing preserved-local-work into generic "some features may be unavailable". |
| **Activity center / durable attention**  | `packet_id`, `pending_session_state`, `boundary_class`, `recovery_copy_label`, `callback_receipt_ref`                                                    | Flattening the object into a toast-only message.                                                        |
| **Credential-store banner**              | `credential_store_class`, `lock_state_class`, `copy_catalog`, `recovery_path`, `retained_local_capabilities`                                             | Using a generic "keychain error" toast with no typed retry path.                                        |
| **Account boundary chip**                | `boundary_class`, `boundary_label`, `plain_language_summary`, `visible_downgrade_path`, the four reserved rows                                           | Rendering an ambiguous `Connected` badge or collapsing `restricted_managed_only` into `managed`.        |
| **Support packet / support bundle**      | `packet_id`, `lock_state_id`, `boundary_id`, `preserved_local_work`, `recovery_path`, `reserved_rows`                                                    | Collapsing the packet into an unstructured support note.                                                |
| **Admin export / admin handoff**         | the full record, including `callback_correlation` aliases, `return_route`, `visible_downgrade_path`, and reserved rows                                    | Replacing durable refs with prose-only summary.                                                         |
| **Release evidence**                     | `packet_id`, `lock_state_id`, `boundary_id`, `callback_receipt_ref`                                                                                      | Minting evidence-local vocabulary that disagrees with the packet.                                       |

Rules (frozen):

1. Surfaces quote fields by id; they do not re-describe preserved-
   local-work, recovery copy, or boundary labels in local prose.
2. Raw tokens, raw URLs, raw cookies, raw codes, raw nonces, raw PKCE
   verifiers, and raw passkey material MUST NOT appear on any surface
   that quotes the packet.
3. Any surface that cannot resolve the packet MUST fail closed (for
   example, to an `unknown_boundary` chip with a typed repair hook)
   rather than rendering an unlabeled `Connected` state.

## Out of scope at this revision

- Live OAuth / OIDC / SAML provider integrations and their wire
  protocols;
- Provider-specific flows (GitHub Actions check links, Jira Cloud
  tenant switchers, etc.);
- Device-code poll transport and rate-limit policy;
- Passkey / WebAuthn platform adapters; and
- Managed-admin policy-bundle formats.

The vocabulary above is what those integrations will land on.

## Change control

- Adding a new auth-flow class, callback-failure reason, credential-
  store lock state, retry path, account-boundary class, reserved-row
  kind, browser-launch policy class, or audit-event id is additive-
  minor and requires a schema / document update in the same change.
- Repurposing an existing value is breaking and requires a new
  decision row co-signed by `security_trust_review` and the owning
  lane (workspace / identity / managed).
- Any change that alters the preserved-local-work semantics, the
  visible-downgrade path, the reserved-row coverage rule, or the
  system-browser-first invariant must update this document, the
  schema, and the adjacent ADR-0001 / ADR-0007 / ADR-0010 / ADR-0015
  references in the same change.
