# Offline-draft and provider-reconciliation fixtures

Worked cases for the contract frozen in
[`/docs/work_items/offline_draft_and_reconciliation_contract.md`](../../../docs/work_items/offline_draft_and_reconciliation_contract.md).

Each fixture is a self-contained YAML document carrying one record
that is schema-valid against one of the boundary schemas:

- [`/schemas/work_items/offline_draft_packet.schema.json`](../../../schemas/work_items/offline_draft_packet.schema.json)
- [`/schemas/work_items/provider_reconciliation.schema.json`](../../../schemas/work_items/provider_reconciliation.schema.json)

Every fixture carries only opaque workspace / branch / revision /
provider-host / provider-tenant / provider-issue / actor /
account-mapping / approval-ticket / consequence-preview / queue-item
/ browser-handoff packet / callback-envelope / change-object /
patch-stack / review-workspace / review-pack /
review-evaluation-result / qa-run / freshness-floor / policy-epoch /
execution-context handles plus monotonic placeholder timestamps and
redaction-aware reviewable labels (no raw provider URLs, no raw
provider issue bodies, no raw comment bodies, no raw label values
that disclose customer / tenant identity, no raw delegated tokens,
no raw branch / commit URLs, no raw author identity strings, no raw
absolute paths, no raw notification payloads, no raw automation
payloads).

## Coverage matrix

| Fixture | Record kind | Origin / Outcome | Strategy / Lifecycle | Acceptance bullet(s) covered |
| --- | --- | --- | --- | --- |
| `no_network_offline_draft_packet.yaml` | `offline_draft_packet_record` | `human_authored_no_network_offline` | `capture_offline_pending_drain` / `captured_offline_pending_drain` | No-network draft preserves the typed local change set, intended target, expiry window, and reconciliation strategy under provider unreachable conditions instead of dropping the user's edit on connectivity restoration. |
| `queued_publish_within_expiry_offline_draft_packet.yaml` | `offline_draft_packet_record` | `human_authored_user_deferred_explicit_choice` | `queue_for_publish_later_within_expiry` / `queued_for_publish_within_expiry` | User-deferred draft admitted to the publish-later queue within a typed 24-hour expiry window; the bound queue item carries the apply path; the typed extension_admissibility within freshness floor pins the safe extension behaviour. |
| `expired_queued_action_re_review_required.yaml` | `offline_draft_packet_record` | `human_authored_user_deferred_explicit_choice` | `re_review_required_after_expiry` / `expired_re_review_required` | Expired queued action pinned to `re_review_required_after_expiry`; the typed `block_reason_summary` and `linked_provider_reconciliation_record_id_ref` carry the typed denial-truth so the surface renders 'must re-review' instead of silently advancing or silently dropping the queued action. |
| `conflict_local_vs_provider_reconciliation.yaml` | `provider_reconciliation_record` | `conflict_observed_local_vs_provider` | `re_review_required_then_retry` next-safe-action | Reconciliation observed conflict across two provider fields (priority_or_severity, assignee) between the offline-captured draft and the live provider state; per-field conflict rows pin the local-draft and provider-remote labels so the user sees typed conflict truth before re-reviewing. The bound offline_draft_packet must flip to `blocked_by_conflict_pending_re_review`. |

## Truthful escape hatches

Every degraded fixture exposes at least one truthful escape hatch
named through the typed vocabularies (see contract section 4):

- **Re-review** — `re_review_required_after_expiry` /
  `re_review_required_then_retry` are the typed re-review next-safe-
  actions on the expired-queued and conflict fixtures.
- **Capture offline** — `capture_offline_pending_drain` strategy on
  the no-network fixture, with the bound
  `linked_offline_handoff_packet_record_id_ref`.
- **Queue for later** —
  `linked_publish_later_queue_item_record_id_ref` on the
  queued-within-expiry fixture admits the typed
  `queue_for_publish_later_within_expiry` strategy.
- **Withdraw before apply** — `withdrawn_before_apply` lifecycle
  pins a typed terminal state with a non-empty `withdrawn_at`
  (illustrated in the contract; not in this corpus).

## Cross-record lineage

The fixtures align with the upstream change_intent /
external_publish_preview / status_transition_packet / offline_handoff
fixtures:

- `work_items:offline_draft_packet:01` (no-network) is authored
  against `work_items:detail:02`, cites
  `work_items:change_intent:04` and
  `work_items:transition_packet:06`, and binds to
  `work_items:offline_handoff_packet:01` (the existing
  `offline_handoff_packet_provider_unreachable.yaml` fixture).
- `work_items:offline_draft_packet:02` (queued within expiry) is
  authored against `work_items:detail:01`, cites
  `work_items:change_intent:02` and
  `work_items:transition_packet:01`, and binds to
  `providers:queue_item:07`.
- `work_items:offline_draft_packet:03` (expired) supersedes the
  same queued admission and binds to
  `work_items:provider_reconciliation:01` (illustrative — the
  reconciliation lifecycle fired the `re_review_required_expired_intent`
  outcome).
- `work_items:provider_reconciliation:02` (conflict) reconciles
  `work_items:offline_draft_packet:01` against the live provider
  state and observes per-field drift on `priority_or_severity` and
  `assignee` fields.
