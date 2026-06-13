# Materialized AI Memory Classes

- Packet: `m5-ai-memory-class-materialization:stable:0001`
- Schema: `schemas/ai/implement-turn-thread-workspace-or-org-memory-classes-prompt-result-cache-objects-and-deletion-export-retention-truth.schema.json`
- Support export: `artifacts/ai/m5/implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth/support_export.json`
- Fixture: `fixtures/ai/m5/implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth/`
- Doc: `docs/ai/m5/implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth.md`

## Coverage

The packet materializes the individual memory objects the recall matrix
qualifies, across all four scopes (turn, thread, workspace, org) and all five
artifact classes (ephemeral turn state, evictable derived cache, prompt-result
cache, reusable semantic memory, durable saved memory).

- **turn-ephemeral-state** (turn / ephemeral_turn_state): per-turn composer
  working state, `session_only`, auto-expiring, local-device only.
- **thread-derived-cache** (thread / evictable_derived_cache): recomputable
  thread context cache, user-deletable/exportable, workspace-local.
- **thread-prompt-result-cache** (thread / prompt_result_cache): content-hash +
  TTL keyed result cache, bounded retention, never a shadow telemetry store.
- **workspace-semantic-memory** (workspace / reusable_semantic_memory): reusable
  semantic memory retained until the user revokes, workspace-deletable/exportable.
- **workspace-saved-memory** (workspace / durable_saved_memory): explicit durable
  saved memory, `durable_until_deleted`, workspace-deletable/exportable.
- **org-saved-memory** (org / durable_saved_memory): org/tenant-scoped saved
  memory, tenant-region-pinned, org-deletable/exportable.
- **org-semantic-memory-region-blocked** (org / reusable_semantic_memory):
  policy-blocked by a region gate and labeled precisely rather than collapsed to
  a generic "memory unavailable" state.

## Deletion / export / retention truth

Every durable object declares an actionable delete and export posture and a
retention class consistent with its artifact class. Ephemeral objects auto-expire.
The prompt-result cache stays content-hash and TTL keyed with a bounded retention,
so it can never masquerade as a durable telemetry store.

## Degradation

A missing or policy-blocked object degrades to a precise `availability` class plus
a non-generic `degraded_label`, never one generic "memory unavailable" state.
Validation rejects a non-available object whose label is missing or generic
(`availability_label_missing`).

## Safety

No cross-workspace or cross-tenant memory is created by default, ephemeral state
stays separated from durable saved memory, prompt-result caches are not used as
shadow telemetry stores, locality never widens past scope, and mixed
retrieval/embedding generations are always labeled. Raw prompt bodies, cached
result bodies, raw embeddings, raw provider payloads, credentials, exact token
counts, and exact cost amounts never cross the support boundary.
