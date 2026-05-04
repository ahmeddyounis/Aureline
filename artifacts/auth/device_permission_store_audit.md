# Device-permission store audit

This audit freezes the reviewer-side rules a deployment uses to keep
**device-permission stores**, **the open-system-settings handoff**,
**revoked / unavailable states**, and **the no-plaintext-secret
fallback** legible across mic, camera, screen-capture, system-audio
loopback, and accessibility-input monitoring rows. It is the
permission-store companion to the
[capture / export review matrix](./capture_export_review_matrix.yaml)
and the
[device-permission and capture-boundary contract](../../docs/auth/device_permission_and_capture_boundary_contract.md).

The audit does not mint vocabulary. Every axis is re-exported by value
from one of the upstream schemas:

- [`/schemas/auth/device_permission_row.schema.json`](../../schemas/auth/device_permission_row.schema.json)
  defines `device_permission_row_record`, the closed `device_class`,
  `permission_state_class`, `processing_locus_class`,
  `retention_posture_class`, `action_class`, and
  `surface_redaction_class` vocabularies.
- [`/schemas/auth/mic_state.schema.json`](../../schemas/auth/mic_state.schema.json)
  defines `mic_state_record` and the persistent mic-state pill / strip
  vocabulary the audit's revoked / unavailable rows lean on.
- [`/schemas/auth/credential_state.schema.json`](../../schemas/auth/credential_state.schema.json)
  defines `store_capability_record`, the closed `store_source_class`,
  `storage_mode_class`, `store_verification_state`, the
  `plaintext_fallback_allowed = false` invariant, and the
  `support_export_posture_class` / `automation_export_posture_class`
  classes the no-plaintext-secret fallback row reuses.
- [`/schemas/ux/speech_privacy_ledger.schema.json`](../../schemas/ux/speech_privacy_ledger.schema.json)
  defines the closed `audio_locality_class`, `transcript_locality_class`,
  `consent_class`, `export_redaction_class`, and
  `export_destination_class` vocabularies the export and provider-
  processing rows quote.

If this audit disagrees with those upstream sources, those sources win
and this audit plus its companion fixtures and matrix update in the
same change.

## 1. Scope

This audit covers the reviewer-grade questions a deployment must answer
for every row that holds device-permission state on disk, in memory, or
in delegated storage. The audit is closed against four areas:

1. **Permission store classes.** Which store classes hold device-
   permission state on the host, with what verification posture, and
   which capabilities they expose.
2. **Open-system-settings handoff.** How a permission row routes the
   user to the OS / system-settings repair path without inventing
   product-owned re-prompt copy that drifts from the OS surface.
3. **Revoked, denied, restricted, and unavailable states.** How the
   audit keeps these distinguishable so a "revoked" row is never
   confused with a "denied" row, and so "unavailable" never silently
   means "policy-blocked".
4. **No-plaintext-secret fallback.** Why permission-store rows MUST
   NOT carry credential or token material as a fallback when the OS
   permission grant is unavailable, and which alternate paths the
   audit admits.

The audit is reusable in diagnostics, support exports, admin exports,
release evidence, and docs/help. Surfaces quote rows by stable id.

## 2. Out of scope

- Live OS permission backend implementation (TCC on macOS, the
  Windows microphone privacy layer, PipeWire portal flows, etc.). The
  audit pins the reviewer-side rules; per-OS adapters land in their
  own qualification packets.
- Per-provider speech / vision SDKs. The audit references
  `byok_remote_provider`, `vendor_hosted_service`, and
  `enterprise_managed_service` by closed class only.
- Final user-facing microcopy. The audit pins state classes, store
  classes, and action classes; product writing chooses final strings
  inside those limits.
- New `device_class`, `permission_state_class`, `mic_state_class`,
  `processing_locus_class`, `retention_posture_class`, `action_class`,
  `store_source_class`, or `storage_mode_class` values. Every axis is
  re-exported; widening requires a decision row in the owning schema.

## 3. Closed permission-store class table

Every device-permission row that persists state on the host MUST
resolve to one of the rows below. The first three rows are the OS-
owned permission stores; the remainder cover the in-product carriers
that surface permission state without owning the OS grant.

| `permission_store_class` | `store_source_class` (credential broker) | What it holds | Verification posture | Persistent across restart? |
|---|---|---|---|---|
| `os_native_permission_store` | n/a (OS-owned) | The OS permission grant for `device_class` (microphone, camera, screen capture, system-audio loopback, accessibility input monitoring) and the deny / restricted state when applicable. | `verified_available` while the OS reports the grant; `verified_unavailable` when the OS reports denial / restriction; `policy_blocked` when an MDM / configuration profile blocks the request. | Yes — the OS persists the grant. Aureline never persists a duplicate. |
| `os_native_permission_indicator` | n/a (OS-owned) | The OS-owned active-capture indicator (mic / camera / screen capture / orange / green dot, taskbar tray icon, etc.). Aureline mirrors but never replaces it. | The mic-state pill MUST defer to this indicator and MUST NOT show `Listening` while the OS indicator is off. | Yes — owned by the OS. |
| `os_native_permission_settings_route` | n/a (OS-owned) | The OS settings deep link or pane that manages this device class for this app. | `verified_available` when the deep link resolves; `verified_unavailable` when the OS / desktop environment does not expose a deep link. | Yes — owned by the OS. |
| `policy_managed_permission_decision_store` | `managed_policy_injector` | The managed decision (allow / block / restricted) the policy bundle expresses for `device_class` on this deployment. Resolves through the policy service; never carries OS grant material. | `verified_available` while the policy bundle is current and the trust state is `trusted`; `policy_blocked` when the bundle blocks the row; `degraded_session_only` is forbidden on this row. | Yes — bundle-scoped; refreshed on policy epoch roll. |
| `local_inert_metadata_store` | `file_backed_secret_ref` | Aureline-owned metadata about the row: revoke history, last-checked timestamp, the `permission_row_id`, the bound `surface_ref`, and the `policy_owner_ref` when present. Carries no OS grant material and no credential / token material. | `verified_available` while the metadata file is readable; `verified_unavailable` when the file is missing or corrupt; `policy_blocked` when policy denies metadata persistence. | Yes — local file; survives restart. |
| `session_only_permission_indicator_cache` | `no_secure_store` | Process-only mirror of the OS grant used to keep the mic-state pill and capture-detail panes inspectable in the current process. Cleared on process exit, policy epoch roll, trust downgrade, or explicit user lock. | `verified_available` while the process is running and the OS grant remains; downgrades visibly to `verified_unavailable` on revoke. | No — process-scoped only. |
| `delegated_grant_handoff_record` | `browser_device_code_handoff` | Opaque record naming a provider / managed-service consent that gates capture handoff (for example, the BYOK provider consent the system browser captured). Holds the consent receipt class and the bound `voice_mode_session_ref` only. | `verified_available` while the consent is current; `verified_unavailable` when the consent is missing; `policy_blocked` when policy revokes the path. | Yes — receipt-scoped; expires per provider terms. |
| `no_permission_store_present` | `no_secure_store` | The deployment has no carrier for this row at all (for example, a device class the platform does not expose). Reviewers see this row as a typed "not applicable" rather than as silence. | `not_configured`. | n/a. |

A row that needs an outcome outside the closed set above opens a
decision row against
[`/docs/auth/device_permission_and_capture_boundary_contract.md`](../../docs/auth/device_permission_and_capture_boundary_contract.md)
instead of widening this table.

### 3.1 Per-store rules

1. **OS-owned stores stay OS-owned.** Aureline MUST NOT shadow the OS
   permission grant in a product-owned store; the
   `os_native_permission_store` row is read-only from Aureline's side.
   The product mirror is `session_only_permission_indicator_cache`;
   any persistent mirror outside the OS store fails closed at audit.
2. **Policy decision is separate from OS grant.** The
   `policy_managed_permission_decision_store` row holds the policy
   bundle's allow / block / restricted decision; it does not hold OS
   grant material. A row that is allowed by policy but denied by the
   OS resolves to `permission_state_class = denied` with the OS
   reason; a row that is granted by the OS but blocked by policy
   resolves to `permission_state_class = policy_blocked` with the
   `policy_owner_ref` quoted.
3. **Local metadata never carries grant material.** The
   `local_inert_metadata_store` row holds opaque ids, timestamps, and
   stable refs only. It MUST NOT carry OS grant bytes, raw audio
   buffers, raw frames, raw screen captures, raw transcript bodies,
   raw URLs, raw provider tokens, or raw credential material.
4. **Session-only mirror downgrades visibly.** The
   `session_only_permission_indicator_cache` row mirrors the OS grant
   for the current process only. On revoke or process exit, the
   mic-state pill MUST downgrade visibly (to `idle`, `unavailable`,
   or `policy_blocked`) and MUST NOT continue to show `Listening` or
   `Processing`.
5. **Delegated handoff records do not store OS grants.** A
   `delegated_grant_handoff_record` row holds the consent receipt
   class and the bound `voice_mode_session_ref` only. Provider
   tokens, refresh tokens, callback bytes, PKCE verifiers, OAuth
   codes, and raw cookies live in the credential-state schema, not
   in this audit.
6. **No-plaintext-secret fallback is invariant.** No store row in
   this table sets `plaintext_fallback_allowed = true`. Adding a row
   that holds OS grant material or credential material in a plaintext
   file is non-conforming.

## 4. Open-system-settings handoff rules

Every device-permission row that names an `open_system_settings_action`
MUST resolve to one of the rows below. The audit keeps "the OS knows
about this app" distinguishable from "the OS does not expose a deep
link" and from "the desktop environment hides the pane".

| `open_settings_handoff_class` | When it applies | What the surface MUST render |
|---|---|---|
| `os_settings_app_scoped_deep_link` | The OS exposes a deep link to the app-specific privacy pane for `device_class`. | A button labelled `Open system settings` whose `route_ref` resolves to the OS deep link by stable id (for example, `os_settings:privacy_microphone:app:aureline`). |
| `os_settings_global_pane_deep_link` | The OS exposes a deep link to the global privacy pane for `device_class` but not an app-scoped one. | The same button label, plus a short note that the user MUST locate Aureline in the pane. The `route_ref` resolves to the global pane id. |
| `os_settings_no_deep_link_visible_path_required` | The desktop environment exposes no deep link (for example, on certain Linux desktops). | A short reviewable sentence naming the visible path the user MUST follow (for example, "Open System Settings → Privacy → Microphone"). The `route_ref` is `null`; the `action_label` MUST NOT pretend a deep link exists. |
| `os_settings_blocked_by_policy` | Policy blocks the OS settings handoff (for example, kiosk lockdown). | The button is disabled with a `policy_owner_ref` quoted and a typed visible repair path (`request_admin_policy_change` or `contact_admin`). |
| `os_settings_unavailable_no_repair_path` | The platform does not surface a settings pane at all (for example, headless / air-gapped envelopes). | The handoff row resolves to `unavailable` with a typed reason. The surface MUST NOT offer "try again"; it MUST quote the underlying envelope. |

### 4.1 Per-handoff rules

1. **The deep link resolves through `route_ref`, never as a raw URL.**
   The action affordance carries an opaque `route_ref` per the
   device-permission row schema. Raw `os_settings:` URIs, raw shell
   commands, and raw `xdg-open` invocations never appear in product
   copy or in support exports.
2. **The handoff is read-only.** Open-system-settings buttons MUST
   land on the OS pane and MUST NOT pre-grant the permission, mutate
   the OS pane, or simulate user input. The product never asserts
   "we just enabled it for you".
3. **No silent retry after the user returns.** When the user returns
   from the OS pane, the surface re-reads the OS grant and re-renders
   the row. A surface that silently restarts capture without an
   explicit `request_permission` action is non-conforming.
4. **No replacement re-prompt.** Aureline MUST NOT impersonate the
   OS permission prompt with an in-product modal that requests
   microphone / camera / screen-capture grants. The native prompt is
   the only conforming path.
5. **Handoff and revoke are paired.** Every row that exposes
   `open_system_settings_action` MUST also expose a `revoke_action`
   when permission is currently `granted`; the two actions never
   collapse into one button.

## 5. Revoked, denied, restricted, and unavailable states

The audit keeps the four nearby states distinguishable. Surfaces quote
the matching `permission_state_class` value verbatim and quote the
matching repair path; reviewers grade each row against the matrix
below.

| `permission_state_class` | When it applies | Required surface response | Capture-side downgrade |
|---|---|---|---|
| `not_requested` | Aureline has never asked the OS for this permission. | The row renders with a `request_permission` action and the OS-prompt boundary disclosed. | Capture is unavailable; the mic-state pill (when applicable) renders `idle` or `unavailable`. |
| `request_in_flight` | An OS prompt is on screen. | The surface MUST NOT pre-celebrate the grant. The row stays in this state until the OS reports a typed outcome. | Capture is unavailable until the OS prompt resolves. |
| `granted` | The OS reports the grant for this `device_class` to this app. | The row exposes `revoke_action` and `open_system_settings_action`. The mic-state pill renders the live state per the speech-privacy ledger. | Capture is admissible while every consent lane on the matrix is satisfied. |
| `denied` | The OS reports denial. The user denied the prompt or revoked the grant via the OS pane. | The row quotes the OS reason. `open_system_settings_action` is the primary repair path; `request_permission` is admissible only when the OS allows a re-prompt. | Capture is unavailable; the pill renders `unavailable` with a reviewable reason and the OS-settings route. |
| `restricted` | The OS reports the grant is restricted (for example, parental controls, MDM device restriction, work-profile boundary). | The row quotes the OS-reported restriction reason. The repair path is the OS / managed-device path, not Aureline. | Capture is unavailable; the pill renders `unavailable` with the restriction reason quoted. |
| `unavailable` | The device is not present, the OS does not expose this permission, or the carrier device is offline. | The row quotes the underlying device / OS reason. `open_system_settings_action` is admissible only when the OS exposes a settings pane; otherwise the row offers no settings handoff. | Capture is unavailable; the pill renders `unavailable` with the device-side reason. |
| `policy_blocked` | A policy bundle blocks this row. | The row quotes the `policy_owner_ref` and the typed repair path (`request_admin_policy_change`, `contact_admin`, or `open_policy_detail`). | Capture is unavailable; the pill renders `policy_blocked` with the policy owner quoted. |
| `unknown_permission_state` | The store cannot tell. | The surface renders the row in an inspect-only posture and MUST NOT permit capture. The audit fails closed. | Capture is unavailable; the pill renders `unavailable`. |

### 5.1 Revoke / re-grant lifecycle rules

1. **Revoke is observable.** When the OS reports the grant has been
   revoked (or when the user clicks the in-product `revoke_action`),
   the surface MUST observe the change within the same process tick
   that capture is permitted, downgrade the mic-state pill, and stop
   any active capture without waiting for the next user action.
2. **Revoke does not delete artifacts on its own.** Revoking the
   permission stops capture; it does not delete previously captured
   transcripts, screenshots, screen recordings, or capture metadata.
   The matching `delete_action_class` row in the matrix governs that.
3. **Re-grant requires explicit user action.** A row that returns
   from `denied` to `granted` does so only because the user re-
   granted the permission via the OS pane. Aureline MUST NOT auto-
   restart capture on re-grant; the user MUST initiate capture
   explicitly.
4. **`unavailable` and `policy_blocked` never collapse.** Surfaces
   never render `unavailable` when the actual reason is policy-block,
   and vice versa. The audit fails closed when the two are conflated.
5. **`denied` and `revoked` never collapse with `restricted`.**
   Restricted states name an OS-side restriction the user cannot
   typically resolve alone (parental controls, MDM, work profile);
   denied / revoked states name a user-resolvable repair path. The
   audit fails closed when these are conflated.

## 6. No-plaintext-secret fallback rules

Device-permission rows are not credential carriers. The audit forbids
any "fallback" that turns a missing OS grant into a plaintext-secret
read or that uses a permission-store row as a credential. The rules
below mirror the credential-state contract's `plaintext_fallback_
allowed = false` invariant; this audit imports the rule by reference
and pins the device-permission analogue.

1. **No plaintext credential file substituting for an OS grant.**
   When the OS grant for `device_class` is `denied`, `restricted`,
   `unavailable`, or `policy_blocked`, Aureline MUST NOT read a
   plaintext credential / token / API-key file as a "capture
   alternative" path. The conforming response is `unavailable` /
   `policy_blocked` with the visible repair path.
2. **No plaintext export of OS grant material.** Support bundles,
   portable profile packages, evidence packets, and admin exports
   MUST NOT carry the OS grant bytes for `device_class`. Exports
   carry the `permission_row_id`, `permission_state_class`,
   `processing_locus_class`, `retention_posture_class`, and the
   action affordances by stable id only.
3. **No plaintext provider token persisted to enable capture.** A
   capture flow that depends on a `byok_remote_provider`,
   `enterprise_managed_service`, or `vendor_hosted_service` provider
   MUST acquire the token via the credential-broker boundary defined
   in `schemas/auth/credential_state.schema.json`. The permission
   store rows in §3 MUST NOT carry the token. A row that violates
   this fails closed.
4. **No plaintext consent receipt.** Provider consent receipts (the
   `delegated_grant_handoff_record` row) hold the receipt class and
   the bound `voice_mode_session_ref` only. Raw consent bodies, raw
   redirect URIs, raw OAuth state, raw PKCE verifiers, and raw
   provider response payloads never appear here.
5. **Session-only mirror is the only admissible fallback.** When the
   OS-owned permission store is read-only and the deployment lacks a
   persistent mirror, the
   `session_only_permission_indicator_cache` row is the only
   admissible fallback. It downgrades visibly on process exit, never
   persists across restart, and never holds credential material.
6. **No silent fallback to a different provider class.** A capture
   flow that loses its `byok_remote_provider` consent MUST NOT
   silently fall back to a `vendor_hosted_service` provider, to an
   `enterprise_managed_service`, or to a local provider that has not
   been explicitly opted into. The conforming response is to render
   the unavailable state and quote the visible repair path.
7. **Unredacted export of capture artifacts requires explicit
   consent.** This rule mirrors the
   `exported_unredacted_with_explicit_consent` row in the matrix:
   surfaces never use the "permission was granted once" history as a
   stand-in for explicit export consent.

## 7. Reviewer checklist

A change touching a device-permission store row, the
open-system-settings handoff, the revoked / unavailable surface, or
the no-plaintext-secret fallback is conforming only if a reviewer can
answer every question below.

1. Which `permission_store_class` from §3 does the row resolve to,
   and which `store_source_class` value (when applicable) does the
   credential broker map onto?
2. Which `permission_state_class` does the OS report, and does the
   surface quote that exact state, the matching reason label, and
   the matching repair path?
3. When the row resolves to `policy_blocked`, which
   `policy_owner_ref` does the surface quote and which typed repair
   path (`request_admin_policy_change`, `contact_admin`,
   `open_policy_detail`) is offered?
4. Which `open_settings_handoff_class` from §4 applies, and does
   the action affordance carry an opaque `route_ref` (never a raw
   URL) that resolves to the matching OS pane?
5. On `denied`, `restricted`, `unavailable`, or `policy_blocked`,
   does the surface fail closed without offering a plaintext-secret
   fallback, a different-provider silent fallback, or a re-prompt
   that pretends to grant the permission?
6. When the deployment exposes no settings deep link, does the
   surface name the visible path verbatim and refrain from inventing
   a deep-link button?
7. When capture leaves the device (every `processing_locus_class` in
   `local_companion_or_sandbox`, `enterprise_managed_service`,
   `byok_remote_provider`, `vendor_hosted_service`), does the row
   bind to the matching `consent_to_provider_processing` lane on the
   capture / export review matrix and to the matching speech-privacy
   ledger record?
8. Does the row guarantee that the
   `local_inert_metadata_store` carrier holds no OS grant bytes, no
   raw audio / frames / screen captures / transcripts, no raw URLs,
   no provider tokens, and no credential material?
9. Does `revoke_action` immediately stop active capture and downgrade
   the mic-state pill within the same process tick that capture is
   permitted?
10. Do support exports, admin exports, release evidence, and
    docs/help quote `permission_row_id`, `permission_state_class`,
    `processing_locus_class`, `retention_posture_class`,
    `permission_store_class`, and `open_settings_handoff_class` by
    stable id, never inventing surface-local copy that disagrees
    with the bound labels?

## 8. Companion artifacts

| Artifact | Role |
|---|---|
| [`/artifacts/auth/capture_export_review_matrix.yaml`](./capture_export_review_matrix.yaml) | Per-surface review matrix that the audit's revoke / unavailable / policy-blocked rows project capture and export consequences onto. |
| [`/fixtures/auth/capture_boundary_cases/`](../../fixtures/auth/capture_boundary_cases/) | Worked capture-boundary fixtures (local dictation, remote provider consent, capture/export review, embedded-origin disclosure, device-code fallback, privileged spoken action) the audit cites for per-state worked examples. |
| [`/fixtures/auth/provider_vs_local_processing_cases/`](../../fixtures/auth/provider_vs_local_processing_cases/) | Worked fixtures distinguishing local processing, enterprise-managed processing, and third-party provider processing — including the consent-blocked, air-gapped, screen-capture-third-party, and policy-blocked rows the audit's revoked / unavailable lanes cite. |
| [`/fixtures/auth/credential_state_cases/`](../../fixtures/auth/credential_state_cases/) | Credential-state fixture corpus the audit's no-plaintext-secret-fallback row inherits from (locked keychain, secure-store downgrade, browser/device-code handoff). |
| [`/docs/auth/device_permission_and_capture_boundary_contract.md`](../../docs/auth/device_permission_and_capture_boundary_contract.md) | Upstream contract that froze `device_permission_row_record`, `mic_state_record`, and the capture-boundary surface vocabulary the audit projects from. |
| [`/docs/auth/credential_state_and_secret_prompt_contract.md`](../../docs/auth/credential_state_and_secret_prompt_contract.md) | Upstream contract whose `plaintext_fallback_allowed = false` invariant the no-plaintext-secret-fallback row imports. |
| [`/docs/ux/voice_and_dictation_contract.md`](../../docs/ux/voice_and_dictation_contract.md) | Upstream contract for the speech-privacy ledger vocabulary the audit cites for capture / transcript / export consent classes. |
| [`/artifacts/auth/system_browser_auth_drill_packet.md`](./system_browser_auth_drill_packet.md) | Companion auth-drill packet the `delegated_grant_handoff_record` row references for browser / device-code consent handoff. |

## 9. Change control

- Adding a new `permission_store_class` row in §3, a new
  `open_settings_handoff_class` row in §4, a new
  `permission_state_class` row response in §5, or a new fallback
  rule in §6 is additive-minor and requires updates to this audit,
  the relevant schema(s), the
  [capture / export review matrix](./capture_export_review_matrix.yaml),
  and at least one fixture under
  [`/fixtures/auth/provider_vs_local_processing_cases/`](../../fixtures/auth/provider_vs_local_processing_cases/)
  in the same change.
- Repurposing an existing row is breaking and requires security /
  trust and accessibility review.
- Any change that weakens the no-plaintext-secret fallback rule, the
  separation between OS-owned and product-owned stores, or the
  revoked / denied / restricted / unavailable distinctions is
  non-conforming and must be accompanied by updated fixtures proving
  the new behaviour remains honest.
