# Proof packet: connected-account registry, provider descriptor, browser-handoff packet seed, and publish-later object-model seed

Purpose: anchor proof captures for the unattended M1 lane that
validates the connected-account / provider registry seed shape, the
provider-entry browser-handoff packet seed shape, and the publish-later
object-seed shape against the canonical record set on
`/schemas/providers/connected_account_record.schema.json`,
`/schemas/providers/publish_later_record.schema.json`, and
`/schemas/integration/browser_handoff_packet.schema.json`, and proves
the seed is consumable by a named docs/help reviewer surface without
re-encoding the connected-account or publish-later vocabularies.

Reviewer entry point:
[`/docs/providers/m1_connected_provider_seed.md`](../../../docs/providers/m1_connected_provider_seed.md).

## Canonical sources

- `fixtures/providers/connected_provider_seed_rows/m1_connected_provider_seed_rows.yaml`
  — seed-row matrix the runner consumes; one row per protected
  provider-linked entry path with a typed
  `mutation_disposition_class`, `provider_actor_class`,
  `publish_later_object_class` (when applicable), and a named
  failure drill.
- `schemas/providers/connected_account_registry.schema.json` — the
  registry-entry, provider-descriptor, and publish-later object-seed
  vocabulary. Freezes closed enums for `mutation_disposition_class`,
  `freshness_class`, `registry_entry_status`,
  `publish_later_object_class`, `provider_actor_class`, and
  `provider_capability_class`.
- `schemas/providers/browser_handoff_packet.schema.json` — the
  provider-entry browser-handoff packet seed, return summary, and
  packet audit-event vocabulary. Composes with
  `schemas/integration/browser_handoff_packet.schema.json` (the
  canonical ADR-0010 packet record); it does not redefine it.
- `schemas/providers/connected_account_record.schema.json` — canonical
  connected-account sub-records (human account link, install grant,
  delegated credential, project-scoped grant, policy-injected service
  identity, acting-identity badge, invalidation event). The registry
  entry references one of these through
  `connected_provider_record_ref` and never duplicates them.
- `schemas/providers/publish_later_record.schema.json` — canonical
  publish-later queue items, queue-review records, consequence
  previews, account mappings, and provider-object relations. The
  publish-later object seed references queue items through
  `publish_later_queue_item_ref`.
- `schemas/integration/browser_handoff_packet.schema.json` — canonical
  browser-handoff packet record (ADR-0010). The packet seed references
  this record through `integration_browser_handoff_packet_ref`.
- `tests/providers/m1_connected_provider_seed_lane/run_m1_connected_provider_seed_lane.py`
  — unattended runner that replays every row, asserts schema
  membership and cross-record agreement, and emits the durable JSON
  capture.

## Named runtime consumer

- `docs/providers/m1_connected_provider_seed.md` — reviewer-facing
  landing page. Wired as the M1 named consumer through
  `consumer_bindings.named_runtime_consumer` on the matrix; consumed
  fields include `connected_account_registry_entry_record.mutation_disposition_class`,
  `connected_account_registry_entry_record.actor_scope.primary_actor_class`,
  `connected_account_registry_entry_record.freshness.freshness_class`,
  `connected_account_registry_entry_record.registry_entry_status`,
  `provider_descriptor_record.supported_capabilities`,
  `provider_entry_browser_handoff_packet_record.reason_code`,
  `provider_entry_browser_handoff_packet_record.expected_authority_on_destination`,
  and `publish_later_object_seed_record.publish_later_object_class`.

## Live runtime consumers (read-only)

- `artifacts/build/build_identity.json` — exact-build identity that the
  capture embeds for cross-artifact traceability.

## Validation captures

- `artifacts/milestones/m1/captures/connected_provider_seed_validation_capture.json`

## Refresh policy

Re-run the validation lane after a change to:

- the seed-row matrix,
- either schema (registry or packet),
- the reviewer-facing landing page,
- the connected-account record or publish-later record vocabularies the
  seed composes with.

## Closure rule

The lane stays open until the latest capture lands under the governed
proof root and every row reports PASS for:

- closed-vocabulary membership
  (`mutation_disposition_class`, `freshness_class`,
  `registry_entry_status`, `publish_later_object_class`,
  `provider_actor_class`, `provider_capability_class`),
- descriptor-admits-entry agreement
  (`mutation_disposition_not_supported_by_descriptor`,
  `actor_not_supported_by_descriptor`),
- publish-later coverage rules
  (`publish_later_seed_required`,
  `inspect_only_must_have_no_publish_later_seed`,
  `browser_handoff_routing_summary_required`,
  `publish_later_queue_item_required`,
  `imported_snapshot_must_be_inspect_only`,
  `browser_handoff_pending_disposition_invalid`,
  `local_draft_only_disposition_invalid`),
- freshness / status agreement
  (`never_observed_must_not_be_live`,
  `live_observed_must_not_carry_unknown_actor`),
- packet-seed disposition agreement
  (`packet_seed_disposition_mismatch`,
  `packet_seed_disposition_invalid`),
- and the row's named failure drill —

and the six required actor classes plus the four required mutation
disposition classes are all observed.
