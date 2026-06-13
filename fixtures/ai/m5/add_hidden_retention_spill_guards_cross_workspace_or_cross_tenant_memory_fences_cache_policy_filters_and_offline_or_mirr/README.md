# Memory Fence, Spill Guard, Cache-Policy Filter, and Fallback Fixtures

## managed_outage_offline_fallback.json

A failure-drill fixture for a managed control-plane outage across claimed M5
profiles. The on-device and BYOK prompt-result caches stay content-keyed and
lifetime-bounded — the BYOK cache's telemetry export attempt is refused and
labeled `shadow_store_blocked`. The managed reusable semantic memory is narrowed to
the tenant by `tenant_isolation` and, with the managed route unreachable, is
`mirror_served` from the workspace mirror. The offline-mirror durable saved memory
is `offline_local_served` from the on-device pack. The hybrid durable saved memory
is `fully_blocked` by a region gate and degrades to `policy_blocked_degraded` with
a precise label rather than a generic provider error.

The fixture demonstrates that, across a managed outage, every non-primary lane
keeps a precise fallback label, offline and mirrored lanes are offline-safe, no
cross-workspace or cross-tenant boundary crosses by default, caches never become
shadow-telemetry stores, and every durable artifact declares its delete and export
posture.

The fixture validates against
`schemas/ai/add-hidden-retention-spill-guards-cross-workspace-or-cross-tenant-memory-fences-cache-policy-filters-and-offline-or-mirr.schema.json`.
