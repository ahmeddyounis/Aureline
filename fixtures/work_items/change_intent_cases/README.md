# Change-intent and external-publish preview fixtures

Worked cases for the contract frozen in
[`/docs/work_items/change_intent_and_publish_preview_contract.md`](../../../docs/work_items/change_intent_and_publish_preview_contract.md).

Each fixture is a self-contained YAML document carrying one record
that is schema-valid against one of the two boundary schemas:

- [`/schemas/work_items/change_intent.schema.json`](../../../schemas/work_items/change_intent.schema.json)
- [`/schemas/work_items/external_publish_preview.schema.json`](../../../schemas/work_items/external_publish_preview.schema.json)

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
absolute paths, no raw notification payloads).

## Coverage matrix

| Fixture                                                                | Record                                          | Lifecycle / publish-timing lane                                                                                  | Acceptance bullet(s) covered                                                                                                                              |
|------------------------------------------------------------------------|-------------------------------------------------|------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------|
| `local_only_draft_change_intent.yaml`                                  | `change_intent_record`                          | `local_only_draft_intent` / `local_workspace_only_no_provider_target` / `save_local_draft_only_no_provider_path` | Local-draft intent; no provider mutation can fire under a local-only target scope; intent stays visible in the lifecycle.                                  |
| `publish_now_external_publish_preview.yaml`                            | `external_publish_preview_record`               | `publish_immediate_publish_now` / `not_deferred_publish_immediate` / `notify_immediately_on_publish`             | Provider mutation preview states target account/context, affected provider fields with before/after labels, and the immediate publish-timing lane.        |
| `review_linked_status_change_change_intent.yaml`                       | `change_intent_record`                          | `pinned_to_publish_preview` / `provider_object_lifecycle_transition` / `submit_publish_now`                      | Linked-review relation survives as typed `linked_review_workspace_with_provider_overlay` plus typed lineage refs through review workspace and evaluation. |
| `deferred_publish_browser_handoff_change_intent.yaml`                  | `change_intent_record`                          | `routed_browser_handoff` / `provider_object_comment_or_subscriber_change` / `open_in_provider_via_browser_handoff` | Publish deferred to the system browser through a typed browser-handoff packet; intent remains visible across local draft -> review -> handoff.            |
| `deferred_publish_account_pending_external_publish_preview.yaml`       | `external_publish_preview_record`               | `publish_queued_for_publish_later_deferred` / `account_mapping_binding_pending_user_resolution` / `notification_unknown_until_drain` | Publish deferred to the publish-later queue under a pending account-mapping binding; the typed deferred-publish consequences block discloses the account-remap freshness-invalidation lane and the revoke-before-drain admissibility. |
| `release_class_change_intent_pinned_to_publish_later.yaml`             | `change_intent_record`                          | `pinned_to_publish_preview` / `release_artifact_publish` / `pin_publish_preview`                                  | Release-class side effect gated on a release-manager approval, a security-review approval, and a pinned external publish preview through the schema's allOf gates. |
| `silent_publish_under_local_only_target_scope_denial.yaml`             | `change_intent_audit_event_record`              | denial: `silent_publish_under_local_only_target_scope_forbidden`                                                  | Author of a change intent claiming a local-only target scope while citing a provider-bound apply ref is denied with the typed reason.                     |

## Truthful escape hatches

Every degraded fixture exposes at least one truthful escape hatch
named through the typed vocabularies (see contract section 4):

- **Open externally** — `linked_browser_handoff_packet_ref` on the
  routed-browser-handoff change intent; `target_account_class =
  browser_handoff_account_session_only` is the typed account lane on
  the matching external publish preview when the workstation's only
  admissible publish path is the system browser.
- **Copy summary** — every record carries a `rationale_summary`
  (intent) and a `summary` (intent + preview); the deferred-publish
  fixture preserves the typed deferred-publish window-closes-at so
  consumers can render a copyable summary of when the deferred
  window expires.
- **Export packet** — the deferred-publish fixture's preview carries
  a `linked_publish_later_queue_item_record_id_ref` and a
  `linked_account_mapping_binding_record_id_ref` so the queue-review
  and admin-reconciliation consumers always have a routable carrier
  rather than a dead-end "publish failed" copy.
- **View sync diagnostics** — both record families carry an
  audit-event vocabulary (`change_intent_*`,
  `external_publish_preview_*`) that lets queue-review,
  support-export, and admin-reconciliation consumers render typed
  sync diagnostics rather than generic "something failed" copy. The
  five `external_publish_preview_invalidated_*` events name the
  typed invalidation lane that re-confirmation MUST satisfy.

## Cross-record lineage

The fixtures form a small lineage:

- `work_items:change_intent:01` is the local-only-draft change intent
  against `work_items:detail:03`.
- `work_items:change_intent:02` is the review-linked lifecycle change
  intent against `work_items:detail:01`; it pins
  `work_items:external_publish_preview:01`, the publish-now external
  publish preview, which cites `providers:consequence_preview:01` and
  `work_items:transition_packet:01`.
- `work_items:change_intent:03` is the routed-browser-handoff change
  intent against `work_items:detail:02`; it pins
  `work_items:external_publish_preview:02` (not in this directory; the
  external publish preview that backs this lifecycle is the
  browser-handoff variant) and `work_items:transition_packet:03`.
- `work_items:change_intent:04` (not in this directory; the
  deferred-publish change intent that backs the deferred preview)
  pins `work_items:external_publish_preview:03`, the
  deferred-publish-account-pending external publish preview against
  `work_items:detail:04`.
- `work_items:change_intent:05` is the release-class change intent
  against `work_items:detail:01`; it pins
  `work_items:external_publish_preview:05` (not in this directory).
