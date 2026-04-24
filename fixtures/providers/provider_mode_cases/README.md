# Provider-mode, callback, and publish-later fixtures

Worked cases for the contract frozen in
[`/docs/providers/provider_mode_contract.md`](../../../docs/providers/provider_mode_contract.md).

Each fixture is a self-contained JSON document bundling the records a
single scenario would emit. Every record is schema-valid against one
of the boundary schemas:

- [`/schemas/providers/provider_callback_envelope.schema.json`](../../../schemas/providers/provider_callback_envelope.schema.json)
- [`/schemas/providers/publish_later_record.schema.json`](../../../schemas/providers/publish_later_record.schema.json)

The `__fixture__` header on every file names the scenario, the mutation
mode(s) exercised, the relation class(es), and the surface class(es)
the scenario covers. The `records` array carries the concrete records.

Coverage across the seeded scenarios:

| Scenario file                                            | Surface                         | Mutation mode       | Relation classes                          | Notable hooks exercised                                                    |
|----------------------------------------------------------|---------------------------------|---------------------|-------------------------------------------|----------------------------------------------------------------------------|
| `code_host_publish_now_callback_return.json`             | `code_host_surface`             | `publish_now`       | `provider_authoritative_object`            | callback return, dedup key, intent signature verified, import refresh      |
| `issue_tracker_local_draft_offline_capture.json`         | `issue_or_planning_surface`     | `local_draft` → `deferred_publish` | `local_draft`, `queued_publish`            | offline capture, pending prerequisites, reopen semantics                  |
| `ci_rerun_open_in_provider.json`                         | `ci_or_checks_surface`          | `open_in_provider`  | `browser_handoff`                         | browser handoff packet, callback return, inspect-only refresh              |
| `docs_portal_queue_item_account_switch.json`             | `docs_or_portal_surface`        | `deferred_publish`  | `queued_publish`                          | account mapping stale_after_account_switch, queue review hold             |
| `artifact_registry_inspect_only_snapshot.json`           | `artifact_registry_surface`     | `inspect_only`      | `cached_read_only_shadow`                 | import session, imported snapshot, bounded partial snapshot               |
| `release_publisher_deferred_publish_preview.json`        | `release_publisher_surface`     | `deferred_publish`  | `queued_publish`                          | consequence preview, irreversible_release_class, force_overwrite_with_preview, queue review admit |
| `callback_replay_invalid_denied.json`                    | `code_host_surface`             | (deny)              | —                                         | replay invalid deny, audit event, repair hook                             |
| `webhook_dedup_redelivery.json`                          | `issue_or_planning_surface`     | `inspect_only`      | `cached_read_only_shadow`                 | provider_webhook callback, dedup scope provider_delivery_id, idempotency  |

Every fixture declares its canonical values via the `exercised` block
so later coverage audits can confirm each vocabulary member is hit at
least once.
