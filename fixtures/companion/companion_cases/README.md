# Browser / mobile companion worked cases

These fixtures are short, reviewable companion-surface scenarios
that anchor the contract frozen in
[`/docs/companion/companion_surface_contract.md`](../../../docs/companion/companion_surface_contract.md)
and validated by:

- [`/schemas/companion/companion_capability_manifest.schema.json`](../../../schemas/companion/companion_capability_manifest.schema.json)
- [`/schemas/companion/offline_triage_snapshot.schema.json`](../../../schemas/companion/offline_triage_snapshot.schema.json)

Each fixture names the companion surface class, platform, auth
posture, freshness / offline posture, notification policy, quiet-
hours inheritance, target-attach class, and (for snapshots) the
snapshot kind, the drain state, and the drain admission class so a
reviewer can read the matrix in one pass.

## Manifest cases

| Case | Surface | Platform | Auth posture | Notification policy | Target-attach | Deployment |
|------|---------|----------|--------------|---------------------|----------------|------------|
| [`browser_web_read_only_inspection_manifest.yaml`](./browser_web_read_only_inspection_manifest.yaml) | browser_web_companion_app | platform_web_browser_chromium | delegated_oauth_pkce_with_managed_redirect | lock_screen_safe_generic_only_no_object_identity | read_only_inspection_via_remote_attach_session_handle | self_hosted |
| [`mobile_native_approval_request_manifest.yaml`](./mobile_native_approval_request_manifest.yaml) | mobile_native_app_companion | platform_ios_native | managed_workspace_remote_attach_session_handle | lock_screen_safe_scoped_with_workspace_or_session_label | scoped_follow_handoff_via_browser_handoff_packet_only | managed_cloud |
| [`browser_extension_alert_acknowledgment_manifest.yaml`](./browser_extension_alert_acknowledgment_manifest.yaml) | browser_extension_companion | platform_browser_extension_chromium | browser_handoff_session_only_no_persistent_token | in_product_only_no_lock_screen_payload | scoped_follow_handoff_via_browser_handoff_packet_only | enterprise_online |
| [`air_gapped_no_push_inspection_manifest.yaml`](./air_gapped_no_push_inspection_manifest.yaml) | browser_web_companion_app | platform_web_browser_chromium | managed_workspace_remote_attach_session_handle | no_companion_push_local_only_inspection | read_only_inspection_via_remote_attach_session_handle | air_gapped |

## Offline triage snapshot cases

| Case | Snapshot kind | Drain state | Drain admission | Freshness at capture |
|------|---------------|-------------|------------------|----------------------|
| [`offline_triage_snapshot_captured_pending_drain.yaml`](./offline_triage_snapshot_captured_pending_drain.yaml) | incident_workspace_inspection_snapshot | captured_local_only_pending_drain | drain_blocked_pending_target_resolution | captured_offline_no_live_path_at_capture |
| [`offline_triage_snapshot_drained_via_approval_ticket.yaml`](./offline_triage_snapshot_drained_via_approval_ticket.yaml) | approval_request_capture_snapshot | drained_to_desktop_admitted_under_envelope | drain_admitted_via_approval_ticket_envelope | captured_warm_within_grace_at_capture |
| [`offline_triage_snapshot_alert_acknowledgment_via_canonical_event_id.yaml`](./offline_triage_snapshot_alert_acknowledgment_via_canonical_event_id.yaml) | alert_acknowledgment_snapshot | drained_to_desktop_admitted_under_envelope | drain_admitted_via_canonical_event_id_dismissal_reuse | captured_live_authoritative_at_capture |
| [`offline_triage_snapshot_drained_rejected_freshness_floor.yaml`](./offline_triage_snapshot_drained_rejected_freshness_floor.yaml) | work_item_detail_inspection_snapshot | drained_rejected_with_typed_reason | drain_blocked_pending_freshness_floor | captured_degraded_beyond_grace_at_capture |

## Acceptance highlights

- **Stale or offline labeling rather than pseudo-live parity.**
  Every snapshot stamps a typed `freshness_posture_class` at
  capture time, and any snapshot whose freshness drifted past the
  grace window can only drain through a typed envelope (publish-
  later, approval-ticket, browser-handoff, console-handoff,
  action-ledger append) — never through a silent canonical-event-id
  dismissal. The freshness-floor rejection fixture exercises this.
- **Notification and quiet-hours behavior reuses the desktop
  vocabulary.** Manifests pin `supported_canonical_event_class_refs`
  back to the desktop event-class rows, never invent a new one;
  the air-gapped manifest forces `no_companion_push_local_only_inspection`
  with an empty event-class set; the browser-extension manifest
  reuses `narrow_locally_never_widen_beyond_desktop_set` so a user
  silencing a class on the extension never widens beyond the
  desktop policy.
- **Companion approvals / comments / acknowledgments produce
  desktop lineage.** The approval-ticket drain fixture cites the
  canonical `approval_ticket_record_id` rather than minting a new
  approval id; the alert-acknowledgment fixture reuses the
  canonical `canonical_event_id`; both records remain readable on
  the desktop activity-event-envelope and approval-ticket audit
  streams.

## Scope rules

- Fixtures validate against their named schema; they do not
  encode wire bytes, ADR-0005 subscription envelopes, or ADR-0004
  RPC envelopes.
- Raw provider URLs, raw provider payloads, raw OAuth tokens, raw
  push-notification provider tokens, raw lock-screen payloads,
  raw stack frames, raw absolute paths, raw command lines, raw
  response bodies, raw alert payloads, raw operator identity
  strings, raw approval-ticket bodies, raw browser-cookie
  material, and raw secret material MUST NOT appear; placeholders
  of the shape `<redacted: <secret_class>>` or opaque handles
  stand in.
- Manifest, snapshot, claim-manifest, capability-lifecycle row,
  approval-ticket, browser-handoff packet, console-handoff
  session, publish-later queue item, incident workspace, runbook,
  action-ledger entry, evidence-handoff bundle, work-item-detail,
  review workspace, alert id, audit-event id, and canonical-event
  id refs are opaque.
- At this milestone there is still no live browser web companion,
  browser extension, mobile native app, mobile web PWA, or
  embedded inspection widget wired up. These fixtures remain
  pre-implementation governance artifacts.
