# Provider Work-Item Link and Comment-Sync Beta

Provider-backed work items now have a typed beta page in
`aureline-provider` that finishes the tracked-work lane by adding stable
link, comment-sync, and publish-review semantics on top of the durable
work-item detail records. Every claimed beta provider row preserves the
distinction between provider-owned truth, local-only engineering context,
and offline or queued handoff packets instead of collapsing them into
one generic linked-item state.

The page groups:

- **Link state.** Branch, review, change-object, peer, and handoff links
  bound to the same work-item object model so "linked branch," "linked
  review," "provider comment," and "offline handoff" cannot drift into
  separate private states. Each link carries source class, relation
  state, freshness, write scope, local-draft state, sync-pending state,
  and conflict-resolution posture.
- **Comment sync state.** Provider-authoritative comments, local draft
  comments, offline-capture-packet comments, queued publishes, failed
  publishes (with a typed retry route), and conflicted publishes each
  occupy a distinct enum lane.
- **Publish-review sheets.** Comment create/edit/delete, branch/review
  link/unlink, status-transition-plus-comment, and retry-after-conflict
  flows render a publish-review row with publish-review source class,
  actor scope, disposition, publish mode, and side-effect fanout.

## Schemas

- `schemas/providers/work_item_link_state.schema.json`
- `schemas/providers/comment_sync_state.schema.json`
- `schemas/providers/publish_review.schema.json`

The canonical work-item vocabularies remain in `schemas/work_items/`
and the work-item-transition beta records remain authoritative for
durable detail headers and status-transition packets. The Rust boundary
is `crates/aureline-provider/src/work_item_sync/`.

## Verification

```sh
cargo run -p aureline-provider --bin aureline_provider_work_item_sync_beta -- page
cargo run -p aureline-provider --bin aureline_provider_work_item_sync_beta -- validate
cargo test -p aureline-provider --test work_item_sync_beta
```

## Support export

```sh
cargo run -p aureline-provider --bin aureline_provider_work_item_sync_beta -- support-export
```

The export intentionally carries only opaque refs, enum classes, acting
identity class, and redaction-safe summaries. It does not carry raw
provider URLs, provider payloads, comment text, project names, or token
material — tracked-work continuity is explainable without leaking any
provider material.

## Acceptance posture

For every claimed beta provider row the page makes four facts explicit:

1. **What is provider truth.** `link_relation_state_class`,
   `comment_sync_state_class`, and `comment_publish_posture_class`
   distinguish provider-observed rows from anything still local.
2. **What is still local draft state.** `link_local_draft_state_class`
   and `comment_origin_class` keep create/edit/remove drafts visible
   even when the provider row is otherwise authoritative.
3. **What is queued for publish.** Queue and offline packet refs are
   first-class, never inferred from string text.
4. **What is conflicted or failed.** `link_conflict_resolution_posture_class`,
   `comment_conflict_class`, `comment_sync_state_class =
   publish_failed_typed_retry`, and the typed retry route record
   capture conflicted and failed paths separately from happy paths.

Branch, review, change-object, and offline-handoff linkage all bind to
the same `work_item_detail_record_id_ref`, so a "linked branch," a
"linked review," and an "offline handoff" cannot drift into private
states after reconnect or import.
