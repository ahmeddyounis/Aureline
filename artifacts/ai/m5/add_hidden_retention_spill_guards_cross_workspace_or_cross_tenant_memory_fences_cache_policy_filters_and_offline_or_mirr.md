# Memory Fences, Spill Guards, Cache-Policy Filters, and Fallback Truth

- Packet: `m5-memory-fence-fallback:stable:0001`
- Schema: `schemas/ai/add-hidden-retention-spill-guards-cross-workspace-or-cross-tenant-memory-fences-cache-policy-filters-and-offline-or-mirr.schema.json`
- Support export: `artifacts/ai/m5/add_hidden_retention_spill_guards_cross_workspace_or_cross_tenant_memory_fences_cache_policy_filters_and_offline_or_mirr/support_export.json`
- Fixture: `fixtures/ai/m5/add_hidden_retention_spill_guards_cross_workspace_or_cross_tenant_memory_fences_cache_policy_filters_and_offline_or_mirr/`
- Doc: `docs/ai/m5/add_hidden_retention_spill_guards_cross_workspace_or_cross_tenant_memory_fences_cache_policy_filters_and_offline_or_mirr.md`

## Coverage

The packet binds memory and cache artifacts across all five claimed M5 profiles
(local, BYOK, managed, offline-mirror, hybrid-managed) and demonstrates a guarded
lane plus each of the four guardrails biting.

- **local-prompt-result-cache-guarded** (local_only / prompt_result_cache):
  content-keyed, lifetime-bounded, telemetry-free, primary fallback.
- **byok-prompt-result-cache-telemetry-blocked** (byok_direct / prompt_result_cache):
  telemetry export refused; `shadow_store_blocked` so the cache cannot become a
  shadow-telemetry store.
- **managed-semantic-memory-tenant-narrowed** (managed_hosted / reusable_semantic_memory):
  cache-policy filter `narrowed` by `tenant_isolation`, with a disclosed reason.
- **managed-saved-memory-region-blocked** (managed_hosted / durable_saved_memory):
  filter `fully_blocked` by a `region_gate` and fallback `policy_blocked_degraded`
  with a precise label; delete/export remain org-scoped.
- **offline-mirror-derived-cache-mirror-served** (offline_mirror / evictable_derived_cache):
  managed route unreachable offline; `mirror_served` from the workspace mirror,
  offline-safe.
- **hybrid-semantic-memory-offline-local-served** (hybrid_managed / reusable_semantic_memory):
  cross-tenant attempt `breach_blocked` at the fence; `offline_local_served` from
  the on-device pack rather than a generic provider error.
- **local-ephemeral-turn-state-guarded** (local_only / ephemeral_turn_state):
  session-scoped, dropped at session end, primary fallback.

## Spill, fence, filter, and fallback truth

A bounded cache is content-keyed and lifetime-bounded with telemetry export
refused; a blocked spill (`would_spill_blocked`, `shadow_store_blocked`) degrades to
a precise label. Neither the cross-workspace nor the cross-tenant boundary crosses
by default, and the fence is visible in-product. A narrowing or blocking
cache-policy filter discloses its reason and narrowing. Every fallback chain ends
in a single non-AI terminal, every non-primary lane carries a precise label, and
offline or mirrored lanes are offline-safe.

## Safety

No cross-workspace or cross-tenant recall happens by default, prompt-result caches
never behave like shadow-telemetry stores, policy-filtered paths disclose what is
narrowed and why, fallback truth stays visible in export, spend or route failures
keep a precise fallback rather than a generic error, and every durable artifact
declares an actionable delete and export posture. Raw prompt bodies, cached result
bodies, raw embeddings, raw provider payloads, credentials, raw endpoint URLs,
exact token counts, and exact cost amounts never cross the support boundary.
