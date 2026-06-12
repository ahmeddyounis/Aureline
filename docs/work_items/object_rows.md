# Work-item object rows, provider chips, and relation strips

This document freezes the export-safe row vocabulary Aureline uses when a provider-backed issue, task, or incident appears in a list, board, queue, search result, detail card, incident card, or companion triage surface.

The machine-readable boundary is [`/schemas/work_items/object_rows.schema.json`](../../schemas/work_items/object_rows.schema.json). The seeded example packet lives at [`/fixtures/work_items/object_rows/canonical_packet.json`](../../fixtures/work_items/object_rows/canonical_packet.json). The checked support/export artifact lives at [`/artifacts/provider/m5/work_item_object_rows/support_export.json`](../../artifacts/provider/m5/work_item_object_rows/support_export.json).

## Required row truth

Every `work_item_object_row_record` keeps these facts visible together:

- provider family, provider label, project or space scope, host scope, and sync scope through the provider chip;
- canonical id, title, exact provider or local lifecycle token, and owner or assignee label where available;
- freshness or offline/imported/local-draft truth without collapsing provider-committed and local-only states into one badge;
- compact relation-strip items for branch or worktree, review, run, incident, and validation evidence with explicit source and freshness;
- export-safe summary fields that preserve provider kind, object kind, link state, sync scope, and relation identity refs without raw provider URLs or hidden account material.

## Guardrails

- Provider-native state tokens stay verbatim. Consumers may not flatten them into generic `Open`, `In progress`, or `Closed` labels.
- `provider_committed`, `local_draft_only`, `queued_publish`, `offline_captured`, `cached_inspect_only`, and `local_relation_only` remain distinct sync-scope classes.
- Relation strips must be keyboard- and export-visible. Hover-only relation disclosure is a contract break.
- Export packets preserve relation identity refs and typed source/freshness posture, but not raw provider payloads or provider URLs.

## First consumers

The current first-consumer wiring reuses this row vocabulary in:

- review work-item linkage detail surfaces;
- search result-truth rows when a result represents tracked work;
- incident workspace packets for linked incident work items;
- companion notification triage and review-queue items.
