# Status-transition review packet fixtures

Worked cases for the contract frozen in
[`/docs/work_items/status_transition_review_contract.md`](../../../docs/work_items/status_transition_review_contract.md).

Each fixture is a self-contained YAML document carrying one record
that is schema-valid against the boundary schema:

- [`/schemas/work_items/transition_review.schema.json`](../../../schemas/work_items/transition_review.schema.json)

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

| Fixture | Trigger | Disposition | Acceptance bullet(s) covered |
| --- | --- | --- | --- |
| `local_only_transition_review.yaml` | `rename_or_metadata_change_trigger` | `admissible_local_draft_only` | Local transition only; no provider mutation can fire under a local-draft disposition; the typed `local_metadata_change_fanout` row plus the `no_followon_automation` disclosure surface the typed truth instead of an empty surface. |
| `provider_authoritative_status_change_transition_review.yaml` | `status_change_trigger` | `admissible_now_publish_now` | Provider-authoritative transition with assignee notification fanout; one `provider_mutation_fanout` row plus one `notification_emission_fanout` row disclose target account, authority source, publish mode, undo / rollback posture, and offline / deferred handling per the consequence-field contract. |
| `policy_blocked_transition_review.yaml` | `assignee_change_trigger` | `blocked_workspace_trust_unset_or_restricted` | Policy-blocked transition under restricted workspace trust; the disposition pins the typed blocked lane, the typed `block_reason_summary` carries the typed escape hatch (grant ADR-0001 trust), and the `no_followon_automation` disclosure confirms no automation will fire while the review is blocked. |
| `linked_review_with_automation_fanout_transition_review.yaml` | `linked_review_or_change_object_trigger` | `admissible_now_publish_now` | Linked-review transition that triggers a code-review request and a build-or-check-run automation; five typed fanout rows disclose provider mutation, linked review update, notification emission, and two queued follow-on automation queueings (admissible via queue-for-publish-later, revocable before drain). |

## Truthful escape hatches

Every degraded fixture exposes at least one truthful escape hatch
named through the typed vocabularies (see contract section 3):

- **Open externally** — `linked_browser_handoff_packet_ref` is the
  ref a `admissible_via_browser_handoff_only` disposition cites; not
  used in this corpus's blocked fixture (restricted-trust workspaces
  cannot launch browser-handoff packets) but documented as the
  typed escape hatch when the workspace is trusted but local write
  authority is not admissible.
- **Queue for later** — `linked_publish_later_queue_item_record_id_ref`
  on the `admissible_via_queue_for_publish_later` disposition; the
  `linked_review_with_automation_fanout_transition_review` fixture
  uses queued automation rows that point to typed publish-later
  queue items.
- **Capture offline** —
  `linked_offline_handoff_packet_record_id_ref` on rows whose
  `offline_deferred_handling_class` is
  `deferred_publish_captured_offline_pending_drain`.
- **Inspect-only what-if** — `admissible_inspect_only_what_if`
  disposition pins a structured what-if review with no apply path;
  paired with per-row `publish_mode_class = inspect_only` and
  `offline_deferred_handling_class = deferred_publish_inspect_only_what_if`.
- **Withdraw before apply** — `withdrawn_before_apply` lifecycle
  pins a typed terminal state with a non-empty `withdrawn_at`.

## Cross-record lineage

The fixtures align with the upstream change_intent / external_publish_preview /
status_transition_packet fixtures:

- `work_items:transition_review:01` reviews
  `work_items:transition_packet:04` (the local-draft-only title rename
  packet) against `work_items:detail:03`; cites
  `work_items:change_intent:01` (the local-only-draft change intent).
- `work_items:transition_review:02` reviews
  `work_items:transition_packet:01` (the publish-now lifecycle
  packet) against `work_items:detail:01`; cites
  `work_items:change_intent:02` (the review-linked lifecycle change
  intent) and `work_items:external_publish_preview:01` (the publish-
  now external publish preview).
- `work_items:transition_review:03` is a blocked review that does
  not bind a status-transition packet (the blocked disposition
  forbids apply); cites `work_items:change_intent:03` (the routed-
  browser-handoff change intent — proposed but blocked under
  restricted trust on this workspace).
- `work_items:transition_review:04` reviews
  `work_items:transition_packet:01` against `work_items:detail:01`
  with five fanout rows including queued follow-on automation;
  cites `work_items:change_intent:02` and
  `work_items:external_publish_preview:01`.
