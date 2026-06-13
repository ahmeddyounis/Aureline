# Memory Fences, Spill Guards, Cache-Policy Filters, and Fallback Truth

This contract describes the export-safe packet that, per claimed M5 deployment
profile, hardens the *policy posture* every memory or cache object carries. Where
the recall matrix
([`freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix.md`](freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix.md))
qualifies whole surfaces, the materialized memory-class lane
([`implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth.md`](implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth.md))
materializes the per-scope memory objects, and the semantic-recall records lane
([`ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth.md`](ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth.md))
materializes the derived retrieval artifacts, this packet binds each governed
artifact on each profile to four interlocking guardrails.

The canonical Rust type is
`aureline_ai::add_hidden_retention_spill_guards_cross_workspace_or_cross_tenant_memory_fences_cache_policy_filters_and_offline_or_mirr::MemoryFenceFallbackPacket`.
The boundary schema is
[`schemas/ai/add-hidden-retention-spill-guards-cross-workspace-or-cross-tenant-memory-fences-cache-policy-filters-and-offline-or-mirr.schema.json`](../../../schemas/ai/add-hidden-retention-spill-guards-cross-workspace-or-cross-tenant-memory-fences-cache-policy-filters-and-offline-or-mirr.schema.json).
The checked support export is
[`artifacts/ai/m5/add_hidden_retention_spill_guards_cross_workspace_or_cross_tenant_memory_fences_cache_policy_filters_and_offline_or_mirr/support_export.json`](../../../artifacts/ai/m5/add_hidden_retention_spill_guards_cross_workspace_or_cross_tenant_memory_fences_cache_policy_filters_and_offline_or_mirr/support_export.json).

## Claimed M5 profiles

Each row is governed under one claimed profile: `local_only`, `byok_direct`,
`managed_hosted`, `offline_mirror`, or `hybrid_managed`. A `local_only` or
`offline_mirror` profile MUST keep its retrieval fallback offline-safe.

## Hidden-retention spill guards

Each row carries a `spill_guard` declaring a retention class, whether it is keyed
by a content hash, a bounded-lifetime label, whether telemetry export is allowed,
and a `spill_state`. A bounded cache (`evictable_derived_cache`,
`prompt_result_cache`) MUST be content-keyed and lifetime-bounded
(`content_keyed_bounded` or `ttl_bounded`) with `telemetry_export_allowed` set to
`false`, so it can never become a hidden retention or shadow-telemetry store. A
cache that would retain past its bound is `would_spill_blocked`; one that would
export to telemetry is `shadow_store_blocked`. A blocked spill degrades to a
precise `degraded_label`.

## Cross-workspace-or-cross-tenant memory fences

Each row carries a `fence` declaring the recall scope it is bound to and the
cross-workspace and cross-tenant fence states. Neither boundary crosses by default:
each state MUST be `fenced` or `breach_blocked` unless an `explicitly_consented`
crossing is backed by a recorded `consent_ref`. The fence is always visible
in-product (`fence_visible`). A `breach_blocked` boundary degrades to a precise
label.

## Cache-policy filters

Each row carries a `policy_filter` whose `filter_state` is `unfiltered`,
`narrowed`, or `fully_blocked`. A narrowing or blocking filter MUST disclose a
`narrowed_reason` (`policy_class`, `region_gate`, `retention_floor`,
`tenant_isolation`, or `byok_boundary`) and a precise `narrowed_disclosure`, so a
policy-filtered path always says what it narrowed and why rather than silently
dropping rows.

## Offline-or-mirror-safe retrieval fallback truth

Each row carries a `fallback` with a resolved `fallback_state`
(`primary_available`, `mirror_served`, `offline_local_served`,
`policy_blocked_degraded`, `terminal_non_ai`), an ordered `fallback_chain` that
MUST end in exactly one `non_ai_terminal` hop, an `offline_safe` flag, and a
`precise_label`. Any non-primary lane MUST carry a precise label so a spend or
route failure never collapses into a generic provider error when a more precise
mirror, offline, cached, or policy-blocked fallback exists. A `mirror_served` or
`offline_local_served` lane MUST be offline-safe and backed by an offline-capable
hop in its chain.

## Deletion and export truth

A durable artifact (`reusable_semantic_memory`, `durable_saved_memory`) MUST
declare an actionable delete and export posture (`user_scoped`,
`workspace_scoped`, `tenant_scoped`, or `org_scoped`). A policy-blocked row still
names its org/tenant-scoped delete and export posture so operators retain control
over what is retained.

## Consumer surfaces

Each row names the surfaces that read it — `composer_assist`,
`docs_browser_recall`, `code_understanding`, `semantic_search`, `support_export`,
and `managed_offline_report`. The consumer projection block records that composer
shows the fence and fallback posture, docs/browser shows what a policy filter
narrowed, search shows the retrieval fallback state, support export shows retention
and fence posture, managed/offline reporting shows fallback truth, and blocked or
degraded lanes are labeled below current.

## Guardrails

- No cross-workspace or cross-tenant recall happens by default.
- Prompt-result caches never behave like shadow-telemetry stores.
- Caches stay content-keyed and lifetime-bounded.
- Policy-filtered paths disclose what is narrowed and why.
- Offline, mirrored, and policy-blocked fallback truth stays visible in export.
- Spend or route failures keep a precise fallback rather than a generic error.
- Every durable artifact declares its delete and export posture.

## Boundary

Raw prompt bodies, cached result bodies, raw embeddings, raw provider payloads,
credentials, raw endpoint URLs, exact token counts, and exact cost amounts never
cross this boundary. Consumers project the typed packet directly rather than
re-deriving spill, fence, filter, or fallback posture locally.
