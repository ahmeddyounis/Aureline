# M5 Layer-Order and Portal Registries

- Packet: `m5-layer-order-and-portal-registries:stable:0001`
- Label: `M5 layer-order and portal registries with a canonical base / sticky / floating / menu / dialog / toast / critical z-tier ordering, owning-surface attachment and restore-safe portal semantics, no hard-coded always-on-top bypass, and one shared z-order model no private overlay bypasses across shell, dialog, panel, embedded, notification, and support surfaces`
- Consumer surfaces: 6
- Z-tiers: base, sticky, floating, menu, dialog, toast, critical, tier_unclassified
- Attachment modes: owning_window_anchored, anchor_element_tracked, focus_scope_contained, owner_driven_teardown, restore_safe_reparent, none_disallowed
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell resolves the base workspace and palette menu tier through the canonical z-tier grammar and anchors its palette portal to the owning window; a hard-coded always-on-top overlay and a detached portal degrade honestly instead of reading as a clean pass
  - Layer-tier entries: 3 / portal entries: 2
- **editor_ui**: `stable`
  - Owner: Editor surface owner
  - Scope: The editor renders the sticky affix and floating hover / peek tiers under the shared model and tears its peek portal down with its owner; a tier that stacks outside the shared model and a restore-unsafe portal both degrade honestly
  - Layer-tier entries: 3 / portal entries: 2
- **onboarding_ui**: `stable`
  - Owner: Onboarding surface owner
  - Scope: The onboarding wizard renders the dialog tier and re-parents its embedded step portal restore-safe; an unclassified z-tier and a portal missing its attachment mode degrade honestly instead of stranding an orphaned overlay
  - Layer-tier entries: 2 / portal entries: 2
- **marketplace_ui**: `stable`
  - Owner: Marketplace / embedded surface owner
  - Scope: The embedded marketplace surface renders the critical prompt tier and governs its extension portal under the shared model; a raw z-index inlined instead of a canonical token and a disallowed detached portal role degrade honestly
  - Layer-tier entries: 2 / portal entries: 2
- **settings_ui**: `stable`
  - Owner: Settings surface owner
  - Scope: The settings and notification surfaces render the transient toast tier under the shared z-order model and anchor the notification portal to the owning window; a private layer that bypasses the shared model and an unstated portal token degrade honestly
  - Layer-tier entries: 2 / portal entries: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved layer-tier and portal truth, so a hard-coded always-on-top bypass or a detached portal is visible in evidence rather than hidden behind a raw z-index
  - Layer-tier entries: 2 / portal entries: 2
