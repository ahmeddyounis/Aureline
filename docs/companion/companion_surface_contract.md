# Browser / mobile companion surface contract: scoped capability, notification, and offline triage

This document freezes the contract Aureline uses for browser /
mobile companion surfaces (browser web companion, browser
extension, mobile native app, mobile web PWA, embedded inspection
widget). Companion surfaces are scoped, no-bypass clients of the
desktop hero product. This contract exists so later browser /
mobile work cannot silently become a parity shortcut around the
desktop: a companion is a read-mostly inspection / approval-
request / scoped-handoff surface that reuses the desktop's object
ids, approval ids, canonical event ids, audit ids, and redaction
posture rather than minting a private control plane.

If this document, the companion schemas, and the worked fixtures
disagree, the normative sources in `.t2/docs/` win and this
document plus its companions update in the same change.

## Companion artifacts

- [`/schemas/companion/companion_capability_manifest.schema.json`](../../schemas/companion/companion_capability_manifest.schema.json)
  — boundary schema for `companion_capability_manifest_record`.
  One manifest per scoped companion client pinning the closed
  list of canonical objects the surface MAY read, the closed
  list of scoped allowed actions the surface MAY emit, the
  authentication posture, the step-up requirement, the freshness
  / offline posture, the notification policy, the quiet-hours
  inheritance rule, the target-attach posture, and the no-bypass
  rules every applicable surface / platform combination
  declares.
- [`/schemas/companion/offline_triage_snapshot.schema.json`](../../schemas/companion/offline_triage_snapshot.schema.json)
  — boundary schema for `offline_triage_snapshot_record`. One
  snapshot per captured-on-companion triage observation
  (incident-workspace inspection, runbook inspection, evidence-
  bundle inspection, work-item-detail inspection, review-
  workspace inspection, alert acknowledgment, approval-request
  capture, comment capture, scoped-follow handoff capture). The
  snapshot reuses the canonical object id, canonical_event_id,
  and audit_event_id rows from the desktop / review / incident
  contracts; the companion never mints a parallel canonical id,
  canonical_event_id, or audit lineage.
- [`/fixtures/companion/companion_cases/`](../../fixtures/companion/companion_cases/)
  — worked fixtures exercising the contract.

This contract composes with (and does not replace):

- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — secret-broker handle and raw-secret-forbidden boundary for
  every OAuth / API key / push-notification provider token a
  companion surface might otherwise persist. Companion surfaces
  hold credentials by handle; raw tokens never cross the
  companion boundary.
- [`/docs/adr/0009-execution-context-and-scope.md`](../adr/0009-execution-context-and-scope.md)
  and
  [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
  — target identity, sandbox posture, and policy epoch every
  scoped companion handoff resolves against.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  and
  [`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json),
  [`/schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json)
  — browser-handoff packet and approval-ticket envelope every
  companion-initiated mutation routes through. The companion
  itself never mutates provider state directly; it captures an
  approval request or a scoped follow / handoff and routes the
  envelope through the desktop.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  and
  [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  — capability-lifecycle row, client-scope, freshness-class, and
  redaction-class re-exported without modification. A capability
  whose `client_scope` excludes `companion_surface` MUST NOT
  render as available on a companion manifest.
- [`/docs/adr/0018-workspace-trust-and-restricted-mode.md`](../adr/0018-workspace-trust-and-restricted-mode.md)
  — workspace-trust state every mutating companion action
  inherits. A companion bound to a desktop in restricted /
  untrusted mode resolves every allowed action to read-only
  inspection or offline-triage capture.
- [`/docs/ux/notification_delivery_contract.md`](../ux/notification_delivery_contract.md)
  and
  [`/schemas/ux/event_lineage.schema.json`](../../schemas/ux/event_lineage.schema.json),
  [`/schemas/ux/activity_event_envelope.schema.json`](../../schemas/ux/activity_event_envelope.schema.json)
  — canonical_event_id, dismissal verb, suppression reason,
  privacy payload class, delivery surface class, and quiet-hours
  mode the companion reuses without re-minting.
- [`/artifacts/governance/deployment_profiles.yaml`](../../artifacts/governance/deployment_profiles.yaml)
  — deployment-profile vocabulary the companion manifest cites
  by class (individual_local, self_hosted, enterprise_online,
  air_gapped, managed_cloud). air-gapped deployments forbid
  companion push to a hosted provider.
- [`/docs/ops/incident_workspace_contract.md`](../ops/incident_workspace_contract.md)
  and the operational incident / runbook / evidence-handoff
  bundle schemas — the companion reuses the operational
  workspace's incident_id, runbook_packet_id, action-ledger
  entry id, evidence-handoff bundle id, and audit_event_id rows.
- [`/schemas/work_items/work_item_detail.schema.json`](../../schemas/work_items/work_item_detail.schema.json)
  and
  [`/schemas/vcs/review_workspace.schema.json`](../../schemas/vcs/review_workspace.schema.json)
  — work-item detail and review-workspace boundary the companion
  inspection reuses by reference.

Normative sources this contract projects from:

- `.t2/docs/Aureline_PRD.md` desktop-hero, parity-prohibition,
  and companion-surface passages.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` notification
  delivery, lock-screen redaction, quiet-hours inheritance,
  digest release, and companion-surface passages.

If this document disagrees with those sources, those sources
win and this document plus the companion schemas update in the
same change.

## Why this exists

A browser / mobile companion left to its own devices reaches for
one of four failure modes:

1. **Pseudo-live parity.** The companion renders an inspection
   row against an offline cache and labels it "current", giving
   the user the impression they are reading live state when the
   desktop has not actually refreshed in hours. Postmortem and
   support readers cannot tell what the user actually saw.
2. **Private control plane.** The companion implements its own
   approval flow, its own audit chronology, its own
   canonical_event_id, and its own dismissal vocabulary. The
   desktop's audit lineage drifts apart from the companion's
   lineage; reviewers cannot resolve a single approval, comment,
   or acknowledgment back to the canonical row.
3. **Hidden authority widening.** The companion launches a
   local shell, attaches a kernel, opens a container engine, or
   otherwise extends authority beyond what the user explicitly
   admitted on the desktop's remote / managed-attach path. The
   user thinks they are inspecting; the companion is actually
   running.
4. **Hidden lock-screen widening.** The companion mints a
   private privacy_payload_class on a lock-screen summary and
   leaks workspace identity, customer identity, or AI prompt /
   completion text past the desktop's lock-screen-safe class.

This contract closes those gaps by freezing one companion
object family:

- **`companion_capability_manifest_record`** — the scoped, no-
  bypass capability manifest every companion surface is admitted
  under. Closed lists of allowed canonical objects, allowed
  scoped actions, auth posture, step-up requirement, freshness /
  offline posture, notification policy, quiet-hours inheritance,
  target-attach posture, and no-bypass rules. Every applicable
  rule for the surface / platform / offline combination MUST
  appear on the manifest's `no_bypass_rules_set`.
- **`offline_triage_snapshot_record`** — the captured-on-
  companion triage observation. The snapshot fails closed at
  `drain_state_class = captured_local_only_pending_drain` until
  the desktop publish-later / approval-ticket / handoff-packet /
  action-ledger / canonical_event_id-dismissal envelope admits
  the drain. The snapshot reuses the canonical object id, the
  canonical_event_id, and the audit_event_id rows from the
  desktop / review / incident contracts.

## Scope

Frozen at this revision:

- one `companion_capability_manifest_record` carrying surface
  class, platform class, deployment-profile scope, policy
  context, auth posture, freshness posture, offline posture,
  notification policy, quiet-hours inheritance, target-attach
  class, redaction class, the closed list of allowed canonical
  object classes, the closed list of scoped allowed-action
  rows (each pinned to its own step-up requirement and the
  matching desktop envelope ref), the closed list of no-bypass
  rules, the canonical event-class refs the surface MAY mirror,
  and linkage refs to claim manifests / capability-lifecycle
  rows / offline-triage snapshots;
- one `offline_triage_snapshot_record` per captured triage
  observation carrying snapshot kind, referenced canonical
  object kind and target ref, reused canonical_event_id and
  audit_event_id refs, linked incident-workspace / runbook /
  evidence-handoff bundle / work-item-detail / review-workspace
  / approval-ticket / browser-handoff packet / console-handoff
  session / publish-later queue item refs, drain state, drain
  admission class and admitted-at, drain rejection reason,
  superseder refs, replay posture, freshness posture, redaction
  class, and policy context.

Out of scope at this revision (named explicitly so reviewers
know what is *not* being decided here):

- implementing browser or mobile companion clients, browser
  extensions, embedded inspection widgets, push-notification
  infrastructure (APNs, FCM, Web Push, mobile MDM push), or
  managed-cloud companion gateways — every contract above is a
  boundary schema and narrative companion, not a user-facing
  surface or an implementation crate;
- platform notification adapters (iOS User Notifications,
  Android NotificationManager, macOS / Windows / Linux toasts,
  browser-extension toasts) — the companion reuses the
  desktop's `delivery_surface_class`, `privacy_payload_class`,
  and `quiet_hours_mode` vocabularies, but adapters bind later;
- final user-facing copy on the companion, in-product onboarding
  copy, store-listing copy, or per-platform settings copy;
- companion-side biometric / passkey / hardware-attestation
  implementation — the manifest declares the step-up
  requirement; the platform binding lands later.

## 1. Read-mostly default

The companion is read-mostly by default. Every manifest carries
at least one `read_only_inspect_canonical_object` action row;
mutating-shaped action rows resolve through the matching
desktop envelopes:

- `request_approval_via_approval_ticket` cites a non-null
  `approval_ticket_ref` (ADR-0010) and resolves
  `step_up_requirement_class` outside
  `no_step_up_required_inspect_only`.
- `append_comment_or_acknowledgment_via_canonical_event_id` and
  `acknowledge_alert_via_canonical_event_id` cite a non-null
  `reused_canonical_event_id_ref`. The companion never mints a
  private `canonical_event_id`
  (`denial = companion_must_not_mint_private_canonical_event_id`).
- `scoped_follow_handoff_to_browser_or_console_via_packet`
  cites a non-null `browser_handoff_packet_ref` or a non-null
  `console_handoff_session_ref`. The companion never launches
  ambient navigation, ambient terminal, ambient kernel, or
  ambient container engine
  (`denial = companion_must_not_launch_local_shell_or_kernel`).
- `capture_offline_triage_snapshot_for_drain_to_desktop`
  resolves through `offline_triage_snapshot_record`. The
  snapshot fails closed at
  `captured_local_only_pending_drain` until the desktop admits
  the drain through a typed envelope.

A companion bound to a desktop in `workspace_trust_state =
restricted` or `untrusted` resolves every allowed action to
`read_only_inspect_canonical_object` or
`capture_offline_triage_snapshot_for_drain_to_desktop`.
`mutating_action_forbidden_under_workspace_trust_restricted`
closes the loop.

## 2. Auth posture and step-up requirement

The companion never persists a raw OAuth token, a raw bearer
token, a raw API key, or a raw push-notification provider
token. Auth-bearing values resolve through ADR-0007 by handle:

| `auth_posture_class`                                      | Use                                                                                                            |
|-----------------------------------------------------------|----------------------------------------------------------------------------------------------------------------|
| `delegated_oauth_pkce_with_managed_redirect`              | Web companion redirects through the desktop's managed OAuth PKCE flow; raw tokens never reach the companion.   |
| `device_code_handoff_to_desktop`                          | Mobile / TV-class flows hand the device code to the desktop for redemption.                                    |
| `managed_workspace_remote_attach_session_handle`          | Managed-cloud / enterprise companion attaches via a session handle bound to the managed workspace.             |
| `browser_handoff_session_only_no_persistent_token`        | Browser / extension companion holds a session-scoped handle; nothing persists past the browser session.        |
| `secret_broker_handle_only_no_raw_token`                  | Generic broker handle (ADR-0007) for any other delegated identity (mTLS, managed-service-identity, AWS / GCP / Azure). |
| `session_unauthenticated_inspect_only_public_evidence`    | Public-evidence inspection (claim manifest, public release evidence) only; every allowed action is read-only. |
| `auth_posture_unknown_pending_review`                     | Safe-default; strips every mutating allowed-action class from the manifest.                                    |

Mutating-shaped action rows resolve `step_up_requirement_class`
outside `no_step_up_required_inspect_only`:

- `biometric_or_local_passkey_required_for_each_approval` —
  per-approval biometric / passkey gate.
- `hardware_attestation_required_for_each_approval` —
  hardware-bound attestation (TPM / Secure Enclave / Android
  StrongBox).
- `managed_admin_approval_required_per_action` — managed-cloud
  admin co-signs every approval the companion routes.
- `ai_tool_proposed_step_up_pending_review` — AI-proposed
  step-up that forbids leaving pending review.
- `step_up_unknown_pending_review` — safe-default; strips every
  mutating allowed-action class.

## 3. Freshness posture and no pseudo-live parity

The companion stamps every rendered object with one of the
typed `freshness_posture_class` rows:

- `live_authoritative_fresh_within_grace` — desktop has
  refreshed inside the freshness window.
- `warm_within_grace_companion_marks_warm` — companion shows a
  warm chip; desktop refresh is in flight or recently completed.
- `degraded_beyond_grace_companion_marks_stale` — companion
  shows a stale chip; mutating-shaped actions are still
  admissible only if the manifest's `freshness_posture_class`
  remains in the live / warm / managed-stale set.
- `imported_snapshot_no_refresh_path` — replay / imported
  bundle; mutating actions denied.
- `offline_no_refresh_path` — companion is offline; only
  `read_only_inspect_canonical_object`,
  `capture_offline_triage_snapshot_for_drain_to_desktop`, and
  `ai_tool_proposed_action_pending_review` are admissible.
- `freshness_unverifiable_user_review_required` — companion
  cannot resolve freshness; mutating actions denied.
- `freshness_class_unknown_pending_review` — safe-default;
  fails closed.

`companion_must_not_paint_stale_object_as_live` closes the
loop. Companion fixtures exercise stale and offline rows
explicitly so reviewers can confirm the companion narrows
honestly rather than rendering stale state as fresh.

## 4. Notification policy and canonical event id reuse

The companion reuses the desktop's `canonical_event_id`,
`event_class`, `delivery_surface_class`,
`privacy_payload_class`, `dismissal_verb`, `suppression_reason`,
and `quiet_hours_mode` vocabularies without modification. The
manifest pins one `notification_policy_class`:

| `notification_policy_class`                                  | `privacy_payload_class` row | Use                                                                                          |
|--------------------------------------------------------------|-----------------------------|----------------------------------------------------------------------------------------------|
| `no_companion_push_local_only_inspection`                    | n/a                         | air-gapped / no-push companions; `supported_canonical_event_class_refs` MUST be empty.       |
| `lock_screen_safe_generic_only_no_object_identity`           | `lock_screen_safe_generic`  | Category-only ("New review item"); never workspace / object identity.                        |
| `lock_screen_safe_scoped_with_workspace_or_session_label`    | `lock_screen_safe_scoped`   | Workspace / session-scoped preview; never object identity.                                   |
| `in_product_only_no_lock_screen_payload`                     | `in_product_only`           | Companion delivers in-product chrome only; OS / lock-screen surfaces denied.                 |
| `policy_forbidden_on_lock_screen_audit_trail_only`           | `policy_forbidden_on_lock_screen` | Lock-screen delivery denied entirely; lineage records `audit_trail_only` linkback.    |
| `ai_tool_proposed_notification_policy_pending_review`        | n/a                         | AI-proposed policy; forbids leaving pending review.                                          |

Rules (frozen):

1. **One canonical_event_id, many envelopes.** The companion
   delivery for a desktop event shares the desktop's
   `canonical_event_id`. Cross-client duplicates collapse via
   `cross_client_canonical_event_id`
   (`denial = companion_must_not_mint_private_canonical_event_id`,
   `cross_client_divergence` on the desktop's
   activity-event-envelope audit stream).
2. **No private dismissal verbs.** The companion reuses
   `acknowledge`, `resolve`, `dismiss`, `snooze`, `mute`,
   `suppress` from the desktop dismissal taxonomy.
   `companion_must_not_mint_private_dismissal_alias` closes the
   loop.
3. **Lock-screen widening forbidden.** The companion's
   `privacy_payload_class` MUST NOT widen beyond the desktop's
   declared class
   (`denial = companion_must_not_widen_lock_screen_payload_beyond_desktop_class`).
4. **Air-gapped deployments forbid companion push.**
   `deployment_profile_class_set` containing `air_gapped`
   resolves `notification_policy_class` to
   `no_companion_push_local_only_inspection`
   (`denial = air_gapped_companion_push_forbidden`).

## 5. Quiet-hours inheritance

The companion inherits the desktop's `quiet_hours_mode` set
and the desktop's `suppression_reason` set. The manifest pins
one `quiet_hours_inheritance_class`:

- `inherit_desktop_canonical_event_id_suppression_set` —
  default; the companion mirrors the desktop's suppression
  state for each `canonical_event_id`.
- `narrow_locally_never_widen_beyond_desktop_set` — the
  companion MAY silence a class on this device (e.g. mute
  `mentions` on the personal phone) but MAY NOT widen beyond
  the desktop policy.
- `managed_admin_quiet_hours_overlay_narrowing_only` —
  managed-cloud overlay that narrows further; never widens.
- `quiet_hours_inheritance_unknown_pending_review` —
  safe-default; fails closed.

`companion_must_not_widen_quiet_hours_beyond_desktop_set`
closes the loop. Held envelopes still open a lineage; the
companion MUST mirror `delivery_surface_class =
not_delivered_held` rather than dropping the audit row.

## 6. Approval, comment, and acknowledgment lineage parity

Any approval, comment, or acknowledgment from a companion
surface creates the same approval / audit lineage as the
equivalent desktop flow. Specifically:

- `request_approval_via_approval_ticket` lands an approval
  request through the desktop's `approval_ticket_record`
  (ADR-0010). The desktop's approval-lineage chronology records
  the request; the companion does not mint a parallel approval
  id.
- `append_comment_or_acknowledgment_via_canonical_event_id` and
  `acknowledge_alert_via_canonical_event_id` reuse the desktop
  `canonical_event_id`. The companion delivery folds into the
  same `event_lineage_record` the desktop opened
  (`schemas/ux/event_lineage.schema.json`).
- A captured-on-companion approval-request snapshot
  (`approval_request_capture_snapshot`) drains through
  `drain_admitted_via_approval_ticket_envelope` and cites the
  same `approval_ticket_record_id_ref`. The desktop audit
  chronology resolves through the same approval id; no
  companion-only audit lineage exists.

`companion_must_not_initiate_provider_mutation_outside_desktop_envelope`
closes the loop on every companion-side mutating intent: the
companion captures and routes; the desktop mutates.

## 7. Scoped follow / handoff and target-attach posture

The companion never launches a local shell, kernel, container
engine, or filesystem-mutating runtime. The only admissible
mutating route out of the companion is through a typed
browser-handoff packet or a typed console-handoff session:

- `read_only_inspection_via_remote_attach_session_handle` —
  read-only inspection of a remote / managed target; no local
  attach.
- `scoped_follow_handoff_via_browser_handoff_packet_only` —
  exit through the system browser via the typed packet.
- `scoped_follow_handoff_via_console_handoff_session_only` —
  exit through an attached console-handoff session.
- `no_target_attach_inspection_only` — companion does not
  attach to a target at all; inspection-only.
- `target_attach_unknown_pending_review` — safe-default; fails
  closed.

`companion_must_not_launch_local_shell_or_kernel` and
`companion_must_not_widen_authority_beyond_remote_managed_path`
close the loop.

## 8. Offline triage capture and drain

The companion captures offline triage observations through
`offline_triage_snapshot_record`. Each snapshot:

- pins a `bound_companion_capability_manifest_id_ref` to the
  manifest the snapshot was captured under;
- pins a `referenced_canonical_object_kind_class` and a
  `referenced_canonical_object_target_ref` reusing the
  canonical object id from the desktop / review / incident
  contracts;
- pins a `reused_canonical_event_id_ref` (when the snapshot
  reuses a desktop canonical_event_id) and a non-empty
  `reused_audit_event_id_refs` (when the snapshot reuses
  desktop audit rows);
- pins a `freshness_posture_class` recording what the
  companion saw at capture time;
- starts at `drain_state_class =
  captured_local_only_pending_drain`;
- transitions to `drained_to_desktop_admitted_under_envelope`
  only when the desktop admits the drain through a typed
  envelope (publish-later queue item, approval-ticket envelope,
  browser-handoff packet, console-handoff session, action-
  ledger entry append, canonical_event_id dismissal reuse).

Drain admissions failing closed:

| `drain_admission_class`                                         | Required ref                                                          |
|-----------------------------------------------------------------|------------------------------------------------------------------------|
| `drain_admitted_via_publish_later_queue_item`                   | `linked_publish_later_queue_item_record_id_ref`                       |
| `drain_admitted_via_approval_ticket_envelope`                   | `linked_approval_ticket_record_id_ref`                                |
| `drain_admitted_via_browser_handoff_packet`                     | `linked_browser_handoff_packet_record_id_ref`                         |
| `drain_admitted_via_console_handoff_session`                    | `linked_console_handoff_session_record_id_ref`                        |
| `drain_admitted_via_action_ledger_entry_append`                 | `linked_action_ledger_entry_record_id_ref`                            |
| `drain_admitted_via_canonical_event_id_dismissal_reuse`         | `reused_canonical_event_id_ref`                                       |
| `drain_blocked_pending_workspace_trust`                         | n/a (pending state)                                                    |
| `drain_blocked_pending_step_up`                                 | n/a (pending state)                                                    |
| `drain_blocked_pending_target_resolution`                       | n/a (pending state)                                                    |
| `drain_blocked_pending_freshness_floor`                         | n/a (pending state)                                                    |
| `drain_admission_unknown_pending_review`                        | n/a (safe-default; fails closed)                                       |

Captured snapshots whose freshness posture is
`captured_imported_snapshot_no_live_path_at_capture`,
`captured_offline_no_live_path_at_capture`, or
`freshness_unverifiable_user_review_required` cannot drain
through `drain_admitted_via_canonical_event_id_dismissal_reuse`
— a stale capture cannot silently flip a desktop event row to
acknowledged. The drain class either resolves to a typed
envelope (publish-later, approval-ticket, browser-handoff,
console-handoff, action-ledger append) or stays in a blocked /
unknown state until the responder admits it from the desktop.

## 9. No-bypass rules

Every manifest declares a non-empty `no_bypass_rules_set`
spanning the rules applicable to its surface / platform /
offline combination:

- `forbid_local_shell_or_kernel_launch_on_companion`
- `forbid_widening_authority_beyond_remote_managed_path`
- `forbid_minting_private_canonical_event_id`
- `forbid_minting_private_dismissal_alias`
- `forbid_minting_private_redaction_pass`
- `forbid_minting_private_audit_event_id`
- `forbid_silent_lock_screen_widening`
- `forbid_silent_quiet_hours_widening`
- `forbid_companion_only_approval_lineage_separate_from_desktop`
- `forbid_pseudo_live_parity_against_offline_or_stale_object`
- `forbid_raw_secret_or_token_persistence_on_companion`
- `forbid_companion_initiated_provider_mutation_outside_desktop_envelope`

Omitting an applicable rule is non-conforming
(`denial = no_bypass_rule_must_be_declared_for_applicable_surface`).

## 10. Redaction posture

`redaction_class` is re-exported from ADR-0007 / ADR-0010 /
ADR-0011 without modification. Raw provider URLs, raw provider
payloads, raw OAuth tokens, raw lock-screen payloads, raw push-
notification provider tokens, raw stack frames, raw absolute
paths, raw command lines, raw response bodies, raw alert
payloads, raw operator identity strings, raw approval-ticket
bodies, raw browser-cookie material, and raw secret material
never cross the companion boundary regardless of class. The
broker-owned redaction pass (ADR-0007) runs before bytes reach
any companion-visible sink, including OS notification payloads,
lock-screen summaries, and companion push payloads.

## 11. Composition rules

- **incident workspace / runbook / evidence-handoff bundle**:
  the companion reuses
  `incident_workspace_record_id`,
  `runbook_packet_record_id`,
  `action_ledger_entry_record_id`,
  `evidence_handoff_bundle_record_id`,
  and the operational audit_event_id rows by reference.
- **work-item detail / review workspace**: the companion reuses
  `work_item_detail_record_id` and
  `review_workspace_record_id` by reference; it never mints a
  parallel work-item or review object.
- **claim manifest / capability lifecycle**: the companion
  manifest pins
  `linked_claim_manifest_refs` and
  `linked_capability_lifecycle_row_refs` so claim review and
  capability-lifecycle review resolve the same rows the
  companion is bound to.
- **support / export bundle**: support / export readers
  resolve through
  `linked_offline_triage_snapshot_refs` on the manifest; the
  prior bundle is never deleted, backdated, or rewritten in
  place.

## Out of scope

- implementing the browser web companion, browser extension,
  mobile native app, mobile web PWA, or embedded inspection
  widget — every contract above is a boundary schema and
  narrative companion, not a user-facing surface;
- push-notification infrastructure (APNs, FCM, Web Push, mobile
  MDM push), companion gateways, or managed-cloud companion
  proxy services;
- platform notification adapters and per-platform settings
  rendering;
- final user-facing copy, store-listing copy, in-product
  onboarding copy.

## Worked fixtures

See [`/fixtures/companion/companion_cases/`](../../fixtures/companion/companion_cases/)
for worked fixtures exercising the contract.
