# Work-item detail, status-transition, and offline-handoff fixtures

Worked cases for the contract frozen in
[`/docs/work_items/work_item_contract.md`](../../../docs/work_items/work_item_contract.md).

Each fixture is a self-contained YAML document carrying one record
that is schema-valid against one of the three boundary schemas:

- [`/schemas/work_items/work_item_detail.schema.json`](../../../schemas/work_items/work_item_detail.schema.json)
- [`/schemas/work_items/status_transition_packet.schema.json`](../../../schemas/work_items/status_transition_packet.schema.json)
- [`/schemas/work_items/offline_handoff_packet.schema.json`](../../../schemas/work_items/offline_handoff_packet.schema.json)

Every fixture carries only opaque workspace / branch / revision /
provider-host / provider-tenant / provider-issue / actor /
approval-ticket / consequence-preview / queue-item / browser-handoff
packet / callback-envelope / change-object / patch-stack /
review-workspace / review-pack / review-evaluation-result /
support-bundle / incident-workspace / object-handoff / freshness-floor
/ policy-epoch / execution-context handles plus monotonic placeholder
timestamps and redaction-aware reviewable labels (no raw provider
URLs, no raw provider issue bodies, no raw comment bodies, no raw
label values that disclose customer / tenant identity, no raw
delegated tokens, no raw branch / commit URLs, no raw author identity
strings, no raw absolute paths, no raw notification payloads).

## Coverage matrix

| Fixture                                                         | Record                                | Authority / mode / admission lane                              | Acceptance bullet(s) covered                                                                                  |
|-----------------------------------------------------------------|---------------------------------------|----------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------|
| `provider_authoritative_synced_detail.yaml`                     | `work_item_detail_record`             | `provider_authoritative_synced` / live-fresh / provider-writeable | Engineering relations on all five axes resolved by reference; users can tell the row is provider-authoritative. |
| `provider_overlay_stale_local_continues_detail.yaml`            | `work_item_detail_record`             | `provider_authoritative_stale_local_continues` / degraded-beyond-grace / provider-unreachable | Provider outage continues to render the row honestly with the freshness chip degraded; offline-handoff path is open. |
| `local_draft_no_provider_object_detail.yaml`                    | `work_item_detail_record`             | `local_draft_no_provider_object` / never-published / local-draft-only | Local-draft authority is visibly distinct from provider-authoritative.                                         |
| `queued_publish_local_authored_detail.yaml`                     | `work_item_detail_record`             | `queued_publish_local_authored` / never-published / queue-only  | Queued-publish authority is visibly distinct from provider-authoritative; pinned to a publish-later queue item. |
| `linked_review_only_no_provider_overlay_detail.yaml`            | `work_item_detail_record`             | `linked_review_only_no_provider_overlay` / warm-within-grace    | Engineering relations resolve through review-pack and evaluation result by reference.                          |
| `imported_handoff_evidence_only_detail.yaml`                    | `work_item_detail_record`             | `imported_handoff_evidence_only` / imported-snapshot-no-refresh | Imported-evidence authority is visibly distinct from a live-provider row.                                      |
| `publish_now_status_transition_packet.yaml`                     | `status_transition_packet_record`     | `mutation_mode = publish_now`                                  | Transition discloses it will mutate provider state; preview pinned.                                            |
| `deferred_publish_status_transition_packet.yaml`                | `status_transition_packet_record`     | `mutation_mode = deferred_publish`                              | Transition discloses it will queue for publish-later, not mutate now; notification fan-out flagged unknown until drain. |
| `open_in_provider_status_transition_packet.yaml`                | `status_transition_packet_record`     | `mutation_mode = open_in_provider`                              | Transition routes through a typed browser-handoff packet (no raw URL launch).                                  |
| `local_draft_only_status_transition_packet.yaml`                | `status_transition_packet_record`     | `mutation_mode = local_draft`                                  | Transition discloses it will save a local draft only; no provider call, no notification, no preview required. |
| `captured_offline_status_transition_packet.yaml`                | `status_transition_packet_record`     | `mutation_mode = deferred_publish` / `captured_offline_pending_drain` | Transition captured offline; admissibility resolves to blocked_pending_prerequisites until connectivity returns. |
| `offline_handoff_packet_provider_unreachable.yaml`              | `offline_handoff_packet_record`       | `provider_unreachable_offline_capture` / `not_submitted_local_capture_only` | Offline capture preserves snapshotted state, owner row, and full five-axis relations; export / retry routes typed. |
| `offline_handoff_packet_browser_blocked.yaml`                   | `offline_handoff_packet_record`       | `browser_handoff_blocked_managed_workstation_no_system_browser` | Captured-pending-export with a typed browser-handoff packet pinned for re-attempt when a browser becomes available. |
| `offline_handoff_packet_imported_evidence_only.yaml`            | `offline_handoff_packet_record`       | `imported_from_support_export_no_live_provider_path`            | Imported handoff resolves provider-acceptance to evidence-only and retry-route to no_retry_imported_evidence_only. |
| `offline_handoff_packet_drained_provider_accepted.yaml`         | `offline_handoff_packet_record`       | `provider_unreachable_offline_capture` → `provider_accept_confirmed_publish_later_drained` | Drain only flipped to accepted after the provider-callback envelope arrived; queue item and callback envelope both bound. |
| `offline_handoff_packet_drained_provider_rejected.yaml`         | `offline_handoff_packet_record`       | `provider_unreachable_offline_capture` → `provider_accept_rejected_with_typed_reason` | Drain rejected with a typed denial reason from the publish-later denial vocabulary; user offered manual retry. |
| `silent_provider_mutation_under_local_draft_denial.yaml`        | `status_transition_packet_audit_event_record` | denial: `silent_provider_mutation_under_local_draft_label_forbidden` | Author of a status-transition packet labelling provider mutation as local-draft is denied, with typed reason. |
| `captured_handoff_must_not_advance_to_accepted_denial.yaml`     | `offline_handoff_packet_audit_event_record`   | denial: `captured_handoff_must_not_advance_to_accepted_without_callback_envelope` | Drain path attempting to flip an offline-handoff packet to confirmed-accepted without a callback envelope is denied, with typed reason. |

## Truthful escape hatches

Every degraded fixture exposes at least one truthful escape hatch
named through the typed vocabularies (see contract section 5):

- **Open externally** — `linked_browser_handoff_packet_ref` on
  the open-in-provider transition packet and on the
  browser-blocked offline-handoff packet.
- **Copy summary** — `clipboard_or_text_export_user_initiated`
  in the provider-unreachable offline-handoff packet.
- **Export packet** — `support_bundle_attachment_by_reference`
  on the provider-unreachable / drained-accepted /
  drained-rejected offline-handoff packets;
  `external_handoff_export_to_managed_admin_only` and
  `cli_export_command_user_initiated` on the browser-blocked
  packet; `local_only_no_export_path` on the imported
  evidence-only packet (an honest absence rather than a
  silent dead end).
- **View sync diagnostics** — every record family carries an
  audit-event vocabulary (`work_item_detail_*`,
  `status_transition_packet_*`, `offline_handoff_packet_*`)
  that lets queue-review, support-export, and
  admin-reconciliation surfaces render typed sync diagnostics
  rather than generic "something failed" copy.

## Cross-record lineage

The fixtures form a small lineage:

- `work_item_detail:01` is the provider-authoritative-synced row;
  `transition_packet:01` is its publish-now transition;
  `transition_packet:03` is its open-in-provider transition;
  `offline_handoff_packet:02` is a browser-blocked offline capture
  against it.
- `work_item_detail:02` is the provider-overlay-stale row;
  `transition_packet:05` is its captured-offline transition;
  `offline_handoff_packet:01` is its provider-unreachable offline
  capture; `offline_handoff_packet:03` and
  `offline_handoff_packet:04` are the drained-accepted and
  drained-rejected outcomes (they each `supersede`
  `offline_handoff_packet:01`).
- `work_item_detail:03` is the local-draft-no-provider-object row;
  `transition_packet:04` is its local-draft-only title rename.
- `work_item_detail:04` is the queued-publish row;
  `transition_packet:02` is its deferred-publish comment add.
- `work_item_detail:05` is the linked-review-only row.
- `work_item_detail:06` is the imported-handoff evidence-only row;
  `offline_handoff_packet:06` is its bound imported handoff packet.
