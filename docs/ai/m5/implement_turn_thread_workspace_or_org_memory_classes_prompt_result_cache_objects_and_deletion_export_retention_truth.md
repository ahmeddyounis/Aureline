# Materialized AI Memory Classes, Prompt-Result-Cache Objects, and Deletion/Export/Retention Truth

This contract describes the export-safe packet that materializes the individual
AI memory objects the frozen recall matrix qualifies. Where the recall matrix
([`freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix.md`](freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix.md))
decides whether a whole surface may ship, this packet records the concrete memory
objects those surfaces hold, one record per object, with explicit retention,
delete, export, locality, and invalidation posture.

The canonical Rust type is
`aureline_ai::implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth::MemoryClassMaterializationPacket`.
The boundary schema is
[`schemas/ai/implement-turn-thread-workspace-or-org-memory-classes-prompt-result-cache-objects-and-deletion-export-retention-truth.schema.json`](../../../schemas/ai/implement-turn-thread-workspace-or-org-memory-classes-prompt-result-cache-objects-and-deletion-export-retention-truth.schema.json).
The checked support export is
[`artifacts/ai/m5/implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth/support_export.json`](../../../artifacts/ai/m5/implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth/support_export.json).

## Memory scope classes

Every object is bound to exactly one scope. Scopes never widen recall by default.

| Scope | Lifetime | Locality bound |
| --- | --- | --- |
| `turn` | Single AI turn | `local_device_only` or `workspace_local` |
| `thread` | One conversation thread | `local_device_only` or `workspace_local` |
| `workspace` | Workspace; no cross-workspace recall | `local_device_only` or `workspace_local` |
| `org` | Org/tenant; no cross-tenant recall | `tenant_region_pinned` or `managed_hosted_region_pinned` |

## Artifact classes

The four-way distinction the spec requires is preserved: ephemeral state,
evictable derived cache, reusable semantic memory, and durable saved memory never
collapse into one another. A fifth class — the prompt-result cache — is an
evictable derived cache with extra bounds.

| Artifact class | Durable | Retention | Notes |
| --- | --- | --- | --- |
| `ephemeral_turn_state` | no | `session_only` | Auto-expires; delete/export is `ephemeral_auto_expire`. |
| `evictable_derived_cache` | yes | bounded/manual | Recomputable, not authoritative. |
| `prompt_result_cache` | yes | `ttl_bounded` / `until_manual_evict` | Content-hash + TTL keyed; never a shadow telemetry store. |
| `reusable_semantic_memory` | yes | `until_user_revoked` / hold | Embedding-generation labeled. |
| `durable_saved_memory` | yes | `durable_until_deleted` | Explicit, deletable, exportable. |

## Deletion, export, and retention truth

Every durable object MUST declare an actionable delete and export posture
(`user_scoped`, `workspace_scoped`, `tenant_scoped`, or `org_scoped`). An
ephemeral object MUST auto-expire (`ephemeral_auto_expire`) or declare
`not_applicable`. Retention class MUST agree with the artifact class: a durable
object cannot claim `session_only`, and an ephemeral object cannot claim a
durable retention class. A `prompt_result_cache` MUST carry `content_hash_key`
and `ttl_expiry` invalidation and a bounded retention class so it can never
masquerade as a durable telemetry store.

## Degradation to precise labels

A missing or policy-blocked object degrades to a precise
[`availability`](../../../schemas/ai/implement-turn-thread-workspace-or-org-memory-classes-prompt-result-cache-objects-and-deletion-export-retention-truth.schema.json)
class (`policy_blocked_retention_gate`, `policy_blocked_region_gate`,
`unavailable_unsupported_backend`, `narrowed_stale_proof`, `revoked_by_user`, or
`expired_auto_evicted`) plus a non-generic `degraded_label`, never one generic
"memory unavailable" state. Validation rejects a non-available object whose label
is missing or generic.

## Consumer flows

Each object names the flows that read it — `composer_assist`, `patch_review`,
`docs_browser_recall`, `branch_agent`, and `support_export`. The consumer
projection block records that composer, review, docs/browser, and agent flows all
show which memory classes were used and that support export preserves the class
distinctions.

## Guardrails

- No cross-workspace or cross-tenant memory is created by default.
- Prompt-result caches are not used as shadow telemetry stores.
- Every durable class declares its retention, delete, and export posture.
- Ephemeral state stays separated from durable saved memory.
- Missing or blocked classes degrade to precise labels.
- Mixed retrieval/embedding generations are always labeled.

## Boundary

Raw prompt bodies, cached result bodies, raw embeddings, raw provider payloads,
credentials, exact token counts, and exact cost amounts never cross this
boundary. Consumers project the typed packet directly rather than re-deriving
retention, locality, or availability locally.
