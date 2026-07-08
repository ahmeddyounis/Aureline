# M5 Deployment Scope Badge Primitive

- Packet: `m5-deployment-scope-badge-primitive:stable:0001`
- Label: `M5 deployment-scope badge primitive: local-only/managed/self-hosted/mirrored/offline-capable/browser-companion operating mode as one distinct, composable cue with residual-dependency and local-safe continuity disclosure`
- Badge consumers: 6 (6 stable)
- Scope values: local_only, managed, self_hosted, mirrored, offline_capable, browser_companion
- Sovereignty postures: locally_sovereign, provider_governed, operator_governed, mirror_synced, offline_resilient, host_delegated
- Residual-dependency classes: signing_and_update_channel, operator_infrastructure, upstream_mirror_sync, cached_capability_window, host_browser_runtime
- Proof freshness SLO: 720 hours (last refresh: 2026-07-08T00:00:00Z)

## Badge consumers

- **Runtime Capability Row**: `stable`
  - Owner: Runtime scope badge owner
  - Scope: The runtime capability row renders the shared deployment-scope badge so a local-only capability reads as locally sovereign while still disclosing the signing-and-update residual dependency it carries, and a managed capability reads as provider-governed with no local authority to overstate — proving the scope is its own axis and never collapses into support class, lifecycle, or channel
  - Worked resolutions: 2
    - scope `local_only` → posture `locally_sovereign` (residual `signing_and_update_channel`)
    - scope `managed` → posture `provider_governed` (residual `no_residual_dependency`)
- **Install / Deployment Card**: `stable`
  - Owner: Install deployment scope badge owner
  - Scope: The install / deployment card renders the shared deployment-scope badge so a self-hosted install reads as operator-governed while disclosing the operator-infrastructure residual dependency it still relies on, and a managed install reads as provider-governed — the same scope vocabulary an install reviewer reads elsewhere
  - Worked resolutions: 2
    - scope `self_hosted` → posture `operator_governed` (residual `operator_infrastructure`)
    - scope `managed` → posture `provider_governed` (residual `no_residual_dependency`)
- **Help / About Panel**: `stable`
  - Owner: Help about scope badge owner
  - Scope: The Help / About panel renders the shared deployment-scope badge so an offline-capable capability reads as offline resilient but only within its cached capability window, and a local-only capability reads as locally sovereign with its signing-and-update dependency stated — deployment posture stays visible whenever capabilities narrow or differ by operating mode
  - Worked resolutions: 2
    - scope `offline_capable` → posture `offline_resilient` (residual `cached_capability_window`)
    - scope `local_only` → posture `locally_sovereign` (residual `signing_and_update_channel`)
- **Diagnostics Report**: `stable`
  - Owner: Diagnostics scope badge owner
  - Scope: The diagnostics report renders the shared deployment-scope badge so a mirrored capability reads as mirror synced and continues with its last mirrored state, disclosing the upstream-mirror-sync residual dependency, and a self-hosted capability reads as operator-governed — the residual-dependency drawer keeps the badge from overstating sovereignty
  - Worked resolutions: 2
    - scope `mirrored` → posture `mirror_synced` (residual `upstream_mirror_sync`)
    - scope `self_hosted` → posture `operator_governed` (residual `operator_infrastructure`)
- **Support Export Row**: `stable`
  - Owner: Support export scope badge owner
  - Scope: The support-export row renders the shared deployment-scope badge so a browser-companion capability reads as host delegated and continues within the host session, disclosing the host-browser-runtime residual dependency as an explicit product truth in exported evidence, and a managed capability reads as provider-governed — exported evidence never loses the scope's meaning
  - Worked resolutions: 2
    - scope `browser_companion` → posture `host_delegated` (residual `host_browser_runtime`)
    - scope `managed` → posture `provider_governed` (residual `no_residual_dependency`)
- **Companion Mode Card**: `stable`
  - Owner: Companion mode scope badge owner
  - Scope: The companion-mode card renders the shared deployment-scope badge so a browser-companion capability reads as host delegated within the host session and an offline-capable capability reads as offline resilient within its cached window — browser companion and offline modes remain explicit product truths a user reads directly rather than hidden footnotes
  - Worked resolutions: 2
    - scope `browser_companion` → posture `host_delegated` (residual `host_browser_runtime`)
    - scope `offline_capable` → posture `offline_resilient` (residual `cached_capability_window`)
