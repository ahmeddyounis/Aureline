# Managed authentication and session-continuity contract

This contract freezes the managed-authentication state model Aureline
uses when a local install participates in self-hosted or managed
identity. Managed identity is an additive capability. It is never the
hidden prerequisite for opening local files, keeping unsaved edits,
using local Git, or exporting user-owned artifacts.

Companion artifacts:

- [`/schemas/auth/managed_session_state.schema.json`](../../schemas/auth/managed_session_state.schema.json)
  defines the `managed_session_state_record` shown by auth, account,
  support, admin, service-health, and offboarding surfaces.
- [`/schemas/auth/reauth_requirement.schema.json`](../../schemas/auth/reauth_requirement.schema.json)
  defines the `reauth_requirement_record` cited by a managed-session
  state when managed capabilities pause behind fresh auth.
- [`/fixtures/auth/managed_session_cases/`](../../fixtures/auth/managed_session_cases/)
  contains worked examples for local-only use, passkey-capable sign-in,
  accessible fallback, dirty-buffer expiry, org switch, seat transfer,
  and seat removal / deprovisioning with local artifacts preserved.
- [`/docs/auth/system_browser_callback_packet.md`](./system_browser_callback_packet.md)
  remains the lower-level browser-handoff, callback-correlation,
  credential-store, and account-boundary packet. This contract extends
  that vocabulary; it does not replace it.

Normative sources this contract projects from:

- `.t2/docs/Aureline_PRD.md` Section 5.36 and 10.21 for account-free
  local use, system-browser OIDC, passkey-capable sign-in, and
  deprovisioning without local-data corruption.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` Appendix BC
  for managed sign-in, step-up auth, passkey capability, and
  recovery-preserves-local-work checks.
- `.t2/docs/Aureline_Technical_Design_Document.md` Section 7.11.7 for
  system-browser-first managed auth, short-lived step-up, and
  deprovision/session-revocation behavior.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` Section 18.17 and
  Appendix Q for managed-session continuity copy, passkey disclosure,
  fallback actions, and diagnostic separation.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` review
  checklists for expired/deprovisioned credentials, offboarding export,
  and local-only continuation after entitlement loss.

If this document disagrees with those sources, those sources win and
this contract plus the schemas must be updated in the same change.

## Principles

1. Managed auth is additive. `account_free_local` remains a valid and
   complete posture for desktop-core work.
2. Auth state is scoped. "Signed in" never implies unbounded provider,
   tenant, org, or admin authority.
3. System-browser-first is the default for managed sign-in and reauth.
   Embedded password collection is not a silent fallback.
4. Passkey capability is disclosed honestly. A passkey-capable path may
   be shown only when the deployment, identity provider, platform, and
   policy support it; otherwise the same surface must name an approved
   fallback.
5. Local work continuity is explicit on every state. Local edit, save,
   undo/redo, and user-owned export remain representable even when
   managed actions pause, seats move, or accounts are deprovisioned.
6. No silent sign-out is admissible. Sign-out, org switch, revocation,
   seat loss, and deprovisioning are visible state transitions with
   recovery or export paths.
7. Diagnostics separate cause classes. Network failure, tenant policy,
   missing credential, authenticator incompatibility, revoked session,
   seat loss, and account deprovisioning never collapse into a generic
   auth error.

## Managed session states

Every state is represented by one `managed_session_state_record`.

| State | Meaning | Required copy | Local continuity | Managed posture | Export / offboarding note |
|---|---|---|---|---|---|
| `local_only` | No managed identity is bound. | "No account is required for local work." | Edit, save, undo, local Git, local tasks, diagnostics, and user-owned export remain available. | Managed-only actions are not configured. | Local export is available without sign-in. |
| `signed_in` | A managed or self-hosted session is current for its declared scope. | Provider/org, actor scope, expiry/refresh posture, and what depends on sign-in. | Local work remains local-authoritative. | Scoped managed capabilities may run until expiry, policy change, or revocation. | Export/offboarding paths remain visible even while signed in. |
| `reauth_required` | Fresh auth is needed before an affected managed action can continue. | Exact reason, affected scope, fallback actions, and local-continuity note. | Edit, save, undo, diagnostics, and export remain available. | Affected managed capabilities pause. | Support export includes the reauth requirement object. |
| `session_expiring` | Session expiry is near or refresh is due. | Expiry/refresh note, affected managed scope, and available reauth paths. | Dirty and clean buffers remain editable/saveable locally. | Managed actions may continue until the declared expiry boundary, then pause. | User-owned export does not depend on successful refresh. |
| `managed_blocked` | Managed auth cannot proceed because policy, network, credential, or authenticator state blocks it. | Cause class and next safe action. | Local work continues unless a separate workspace-trust policy narrows it. | Blocked managed actions stay disabled with reason. | Diagnostics export names the failing class. |
| `revoked` | The session was revoked or invalidated by an identity/provider authority. | Revocation reason, affected tenant/org scope, and local-continuity note. | Local artifacts remain intact. | Managed actions that require the session stop immediately. | Support/admin export includes revocation refs, not raw tokens. |
| `seat_lost` | Seat, license, or org membership no longer grants the managed capability. | Seat-loss reason, affected feature set, and appeal/transfer path. | Local files, unsaved edits, local Git, local tasks, and user-owned exports remain available. | Seat-bound managed capabilities pause or revoke. | Offboarding and export paths remain available without a live paid seat for user-owned data. |
| `deprovisioned` | Account deprovisioning has removed managed access. | What stopped, what remains local, and which admin/export path applies. | Local artifacts and recoverable local history remain intact. | Managed capabilities are revoked except any explicitly retained legal/audit reads. | Offboarding export and local archive paths remain visible. |

The schema requires each state to carry `required_copy`,
`preserved_local_work`, `managed_capability_posture`,
`diagnostics.support_export_fields`, and `export_and_offboarding`.
Surfaces quote those fields by reference rather than inventing local
copy for each banner or sheet.

## Flow contract

`flow_context.flow_class` names why the state exists. Each flow obeys
the same no-silent-signout and local-continuity rules.

| Flow | Required behavior |
|---|---|
| `managed_sign_in` | Launches the system browser by default, binds callback state through the existing callback packet, discloses passkey availability, and offers device-code or equivalent fallback where policy and IdP support it. |
| `step_up_auth` | Shows the exact reason and affected action scope before elevation. Failure denies only the protected managed action, not the local session. |
| `session_expiry_dirty_buffers` | Dirty buffers remain editable, saveable, undoable, diagnosable, and exportable locally. Managed actions pause at expiry until reauth succeeds. |
| `session_expiry_clean` | Same as dirty-buffer expiry, without implying local artifacts are unsafe or deleted. |
| `sign_out` | Requires visible confirmation of what managed scopes will stop and what local artifacts remain. It must not silently delete local workspaces, local caches needed for user-owned history, or exported artifacts. |
| `org_switch` | Shows current and target org scope, affected managed services, any reauth requirement, and a local-continuity note. A failed switch degrades to a visible local-safe or managed-blocked state. |
| `seat_transfer` | Preserves local artifacts while the seat binding changes. Managed capabilities may pause until the new seat or group entitlement is visible. |
| `seat_removal` | Revokes seat-bound managed capabilities promptly, names the support/admin path, and keeps local edit/save/undo/export available. |
| `account_deprovisioning` | Consumes a signed revocation or policy update locally, preserves local artifacts, and exposes offboarding/export guidance without requiring a live managed seat. |
| `local_only_start` | Starts without sign-in and renders managed surfaces as not configured, not broken. |
| `credential_recovery` | Routes through system-browser, device-code, credential-store unlock, or admin-assisted recovery as declared by policy. CAPTCHA-only or knowledge-question-only recovery is not an admissible sole path. |

Every flow also declares:

- `system_browser_posture` - required, blocked with visible recovery,
  or not applicable for local-only.
- `embedded_auth_posture` - forbidden unless a separate approved
  boundary contract exists.
- `passkey_availability` - supported, unsupported by deployment stack,
  unsupported by platform/authenticator, policy-disabled, unknown with
  disclosed fallback, or not applicable.
- `no_silent_signout_guarantee = true`.

## Reauth requirement object

`reauth_requirement_record` is the durable answer to "why did this
managed action pause?" The object carries:

- `reason_class` and `reason_label` - exact reason, such as
  `session_expired`, `step_up_required`,
  `tenant_policy_requires_reauth`, `authenticator_incompatible`,
  `revoked_session`, `seat_removed`, or `account_deprovisioned`.
- `affected_action_scope` - scope class, scope ref, affected actions,
  paused managed capabilities, and local capabilities that remain
  unaffected.
- `expiry_or_refresh` - whether the session is already expired, expires
  at a known time, needs policy refresh, or cannot refresh while
  offline, plus a reviewable refresh note.
- `allowed_fallback` - primary fallback and ordered fallback classes:
  passkey, system browser, device code or equivalent, credential-store
  unlock, admin-assisted recovery, continue local-only, support export,
  contact admin, or no managed fallback.
- `local_work_continuation` - boolean `may_continue_while_managed_paused`
  fixed to `true`, retained local capabilities, paused managed
  capabilities, and a continuity note.
- `diagnostics` and `support_export` - typed evidence refs and redacted
  labels that support, admin, and release-evidence surfaces can quote.

A reauth requirement is not a sign-out event. It cannot delete local
buffers, close the workspace, discard undo history, or hide the export
path.

## Diagnostics and support export

Each `managed_session_state_record` contains
`diagnostics.primary_failure_class` plus a support-export field for
each failure family:

| Support-export field | Meaning |
|---|---|
| `network_failure_ref` | DNS, proxy, transport, IdP reachability, or managed control-plane reachability evidence. |
| `tenant_policy_ref` | Policy bundle, org rule, browser restriction, entitlement, or conditional-access evidence. |
| `missing_credential_ref` | Missing OS-store, enterprise-vault, session-only, or delegated credential evidence. |
| `authenticator_incompatibility_ref` | Passkey/WebAuthn/platform-authenticator incompatibility evidence. |
| `revoked_session_ref` | Revoked or invalidated session evidence. |
| `seat_loss_ref` | Seat removal, org membership loss, or entitlement transfer evidence. |
| `account_deprovisioning_ref` | Deprovisioning, offboarding, or signed lifecycle-update evidence. |
| `credential_store_ref` | Credential-store locked, denied, unavailable, degraded, or policy-blocked evidence. |

Fields that do not apply are `null`; they are not omitted. That keeps
support packets from flattening distinct causes into one generic error.
Raw tokens, raw cookies, raw provider URLs, raw passkey material, raw
directory attributes, and raw account identifiers never appear.

## Surface consumption rules

| Surface | Required fields | Forbidden shortcut |
|---|---|---|
| Auth sheet | `state_class`, `flow_context`, `passkey_availability`, `required_copy`, `preserved_local_work`, `reauth_requirement` when present | "Something went wrong" with no cause class or local-continuity note. |
| Status bar / account chip | `state_class`, `required_copy.local_continuity_label`, `session_window`, and active org/tenant refs | A generic `Connected` badge that hides expiry, seat, or org scope. |
| Activity center | `session_state_id`, `flow_class`, `primary_failure_class`, recovery/action labels, and export path | Toast-only auth failure with no durable row. |
| Support bundle | Full diagnostics support-export fields, redaction class, reauth requirement refs, and offboarding/export posture | Unstructured notes that omit the failure family. |
| Admin/offboarding export | State, flow, tenant/org refs, policy context, seat/deprovision refs, export/offboarding path, and local-continuity rules | Requiring a live managed seat to export user-owned local artifacts. |
| Service-health panel | Distinct auth/control-plane health from local workspace health | Implying the IDE is unsafe because managed auth is blocked. |

## Fixture coverage

The fixture corpus covers:

- local-only mode with no account dependency;
- passkey-capable managed sign-in through the system browser;
- accessible fallback when passkey/platform authenticator support is
  unavailable;
- session expiry while dirty buffers remain editable/saveable;
- org switch that preserves local workspace authority;
- seat transfer with temporary managed pause and local continuity; and
- seat removal / account deprovisioning with preserved local artifacts
  and offboarding export available.

## Change control

- Adding a new managed-session state, flow class, passkey-availability
  class, fallback class, diagnostic failure class, or support-export
  field is additive-minor and requires schema, contract, and fixture
  updates in the same change.
- Repurposing an existing value is breaking and requires security,
  accessibility, support, and owning-lane review.
- Any change that weakens local edit/save/undo/export continuity,
  allows silent embedded-auth fallback, or permits silent sign-out must
  update this contract, the schemas, and the browser-callback contract
  together.
