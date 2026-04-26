# Hosted-review inbox, workspace-state, and merge-queue-entry worked fixtures

These YAML fixtures exercise the hosted-review inbox, provider-
authoritative state, and merge-policy contract frozen in
[`/docs/vcs/hosted_review_and_merge_policy_contract.md`](../../../docs/vcs/hosted_review_and_merge_policy_contract.md)
and the boundary schemas at
[`/schemas/vcs/review_inbox_row.schema.json`](../../../schemas/vcs/review_inbox_row.schema.json),
[`/schemas/vcs/review_workspace_state.schema.json`](../../../schemas/vcs/review_workspace_state.schema.json),
and
[`/schemas/vcs/merge_queue_entry.schema.json`](../../../schemas/vcs/merge_queue_entry.schema.json).

Every fixture is one record validated against `oneOf` in the
appropriate schema. Each carries only opaque workspace / branch /
revision / provider-rule-snapshot / approval-ticket / actor / command /
patch-stack-member / review-evaluation-result / review-pack handles
plus monotonic placeholder timestamps and redaction-aware labels — no
raw absolute paths, no raw branch / commit URLs, no raw author identity
strings, no raw comment bodies, no raw provider rule bodies, and no
raw URLs.

| Fixture | Record kind | Acceptance bullet |
|---|---|---|
| `local_parity_only.yaml` | `review_workspace_state_record` | Acceptance bullet 3 — local parity only; checks evaluated on the user's clone, no provider overlay; declared check class with no evaluator surfaces a typed `not_evaluated_on_this_surface` cell rather than disappear into a green-by-omission view; draft-only-local approval visibly distinct. |
| `provider_authoritative_checks.yaml` | `review_workspace_state_record` | Acceptance bullet 1 + bullet 3 — provider-authoritative checks alongside local parity-estimate cells, never collapsed into one ambiguous "ready to merge" badge; provider-authoritative approvals carry an approval ticket and a published-at timestamp. |
| `merge_queue_blocked_stale_base.yaml` | `merge_queue_entry_record` | Acceptance bullet 3 — merge-queue blocked on stale base; entry never appears queue-eligible while the base is stale; rerun authority resolves to provider-authoritative (the local rerun is advisory only). |
| `stacked_review_dependency.yaml` | `merge_queue_entry_record` | Acceptance bullet 3 — stacked review dependency; downstream entry inherits an upstream member's block through the stack-dependency strip rather than re-deriving an independent block reason. |
| `provider_outage_local_continuation.yaml` | `review_inbox_row_record` | Acceptance bullet 1 + bullet 2 + bullet 3 — provider outage with continued local review / export path; inbox row chips as `local_parity_estimate` under `inbox_row_unverifiable_provider_unreachable` freshness, with `provider_outage_local_continues_publish_queued` offline-continuation so a publish-later action is mechanically distinguishable from a real-time publish. |

## Cross-walk to the spec

- The local-parity / provider-authoritative pair
  (`local_parity_only.yaml` + `provider_authoritative_checks.yaml`)
  covers acceptance bullet 1 (compare local evidence with provider
  evidence without collapsing) and acceptance bullet 2 (offline draft
  approvals visibly distinct from provider-synced approvals).
- The two merge-queue entries
  (`merge_queue_blocked_stale_base.yaml` +
  `stacked_review_dependency.yaml`) cover acceptance bullet 3
  (mergeability, queue eligibility, queue blocked, checks stale,
  policy blocked, approval invalidated, local-vs-provider rerun
  authority) plus the explicit stacked-review-dependency requirement.
- The provider-outage row (`provider_outage_local_continuation.yaml`)
  covers acceptance bullet 3's outage path plus the deliverable that
  forbids a stale provider overlay from being relabelled as
  authoritative.
- Forward dependency slots (`merge_policy_record_id_ref` on every
  fixture) are set to `null`; they will become non-null when the
  merge-policy resolver lands.
