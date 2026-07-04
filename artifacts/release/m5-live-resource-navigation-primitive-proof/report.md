# M5 Live-Resource Navigation Primitive: Link Row, Compare Card, Explorer Row, and Drift Banner

- Packet: `m5-live-resource-navigation-primitive:stable:0001`
- Label: `M5 Live-Resource Navigation Primitive: Link Row, Compare Card, Explorer Row, and Drift Banner`
- Navigation surfaces: 6 / 6
- Resource kinds: workload, network, config, storage, identity, custom_resource
- Compare verdicts: in_sync, drifted, rendered_only_no_live, live_only_unmanaged, overlay_authoritative, comparison_unavailable
- Permission postures: full_access, read_only, permission_limited, connection_lost, offline

## Navigation surfaces

- **Source-to-live navigator**
  - Owner: Live-resource navigation guild
  - Scope: Source-to-live links joining authored / rendered / live truth without blur
  - Worked cases: 2
    - `resource:web-deployment:0001` → in_sync (live), freshness `live_fresh`, current
    - `resource:api-gateway:0002` → drifted (live), freshness `live_fresh`, narrowed
- **Rendered / live compare card**
  - Owner: Rendered / live compare guild
  - Scope: Compare cards naming exactly what diverged and what stays inspectable
  - Worked cases: 2
    - `resource:api-gateway:0002` → drifted (live), freshness `live_fresh`, narrowed
    - `resource:web-deployment:0001` → in_sync (live), freshness `live_fresh`, current
- **Cluster / resource explorer**
  - Owner: Cluster explorer guild
  - Scope: Explorer rows with kind, identity, freshness, health, and permission notes
  - Worked cases: 1
    - `resource:cache-store:0003` → in_sync (live), freshness `cached_stale`, narrowed
- **Drift / unavailable banner**
  - Owner: Action-safety guild
  - Scope: Drift / unavailable banners disclosing loss before any live action
  - Worked cases: 1
    - `resource:payments-db:0004` → comparison_unavailable (live), freshness `cached_stale`, narrowed
- **Provider-console handoff**
  - Owner: Provider-overlay guild
  - Scope: Provider-console handoff naming overlay-authoritative live truth
  - Worked cases: 1
    - `resource:load-balancer:0005` → overlay_authoritative (provider_overlay), freshness `unknown`, narrowed
- **Support / export replay**
  - Owner: Support / diagnostics guild
  - Scope: Offline replay reconstructing navigation truth from an imported snapshot
  - Worked cases: 1
    - `resource:web-deployment-replay:0006` → comparison_unavailable (planned), freshness `imported_snapshot`, narrowed
