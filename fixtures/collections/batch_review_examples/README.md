# Batch-review-packet contract fixtures

Worked fixtures for the contract frozen in
[`/docs/ux/collection_view_contract.md`](../../../docs/ux/collection_view_contract.md)
and the schema at
[`/schemas/collections/batch_review_packet.schema.json`](../../../schemas/collections/batch_review_packet.schema.json).

The fixtures exist so search, review, log, package, work-item, and
admin surfaces — and the support / parity-audit tooling that reads
their captures — can compare against the same honesty model for:

- visible vs loaded vs matching vs selected truth, with a typed
  count status (`exact`, `approximate`, `provider_limited`,
  `stale`, `cached`, `partial`, `unknown`);
- included vs excluded vs blocked vs unavailable vs skipped vs
  hidden-selected vs not-loaded breakouts;
- client-local vs provider-authoritative vs mixed execution
  origin, and the matching provider-side query scope;
- pinned item identity that survives sort, filter, and
  virtualisation between mint and apply.

Each JSON file is a single `batch_review_packet_record`. The
`__fixture__` prelude is reviewer metadata; the canonical
vocabulary lives in the record itself.

## Cases

- [`search_collection_provider_authoritative_all_matching.json`](./search_collection_provider_authoritative_all_matching.json)
  — search-collection batch that escalates from visible rows to
  all-matching-query, with provider-limited matching truth, three
  policy-blocked rows, and stable provider-owned identity pinned at
  mint.
- [`review_collection_visible_only_mixed_truth.json`](./review_collection_visible_only_mixed_truth.json)
  — review-collection batch on the visible window with mixed
  client-then-provider execution, two protected-path blocks, one
  concurrent-edit block, two user-excluded rows, and a stable
  client-side identity.
- [`package_inventory_explicit_custom_set_blocked.json`](./package_inventory_explicit_custom_set_blocked.json)
  — package-inventory batch on an explicit custom set with
  ownership-blocked and provider-unsupported breakout, hidden-
  selected disclosure across a filter change, and a saved-view
  drift state pinned via the paired saved-view ref.
- [`log_collection_provider_offline_unavailable.json`](./log_collection_provider_offline_unavailable.json)
  — log-collection export batch on a stream whose provider went
  offline mid-review: stale matching count, five unavailable rows
  attributed to the provider, evidence-only recovery class, and a
  redaction-aware support-export posture.
