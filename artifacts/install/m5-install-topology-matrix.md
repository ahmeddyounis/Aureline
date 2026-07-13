# M5 Install-Topology, Mutable-State-Boundary, Portable-Update, and Fleet-Rollout Execution Matrix

- Packet: `m5-install-topology:stable:0001`
- Label: `M5 install-topology, mutable-state-boundary, portable-update, and fleet-rollout execution matrix`
- Install-topology families: 5 (5 stable)
- Install-topology roles: install_mode, updater_owner, binary_root, writable_state_roots, policy_roots, rollback_target, rollout_ring
- Per-user-managed-install roles: user_scoped_binary_root, per_user_updater_ownership, user_writable_state_root, user_scoped_policy_root, bound_to_topology_registry, machine_global_state_spill_disallowed
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Install-topology families

- **per_user_managed**: `stable`
  - Owner: Updater and install-topology owner
  - Canonical schema: `schemas/install/m5-install-topology.schema.json`
  - Scope: One per-user managed install profile naming the user-scoped binary root, per-user updater ownership, user-writable state root, and user-scoped policy root so binary placement and updater ownership stay inspectable and durable state never spills into hidden machine-global paths
  - Required labels: identity, semantic_role, registry_reference, install_mode
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **per_machine_managed**: `stable`
  - Owner: Managed-deployment and admin owner
  - Canonical schema: `schemas/install/m5-install-topology.schema.json`
  - Scope: One per-machine managed install profile naming the machine-scoped binary root, admin-owned updater, shared machine state root, and machine policy root so updater ownership and admin control are never hidden in managed flows and every writable state root is inspectable
  - Required labels: identity, semantic_role, registry_reference, install_mode, state_root
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **side_by_side_stable_preview**: `stable`
  - Owner: Channel-coexistence owner
  - Canonical schema: `schemas/install/m5-install-topology.schema.json`
  - Scope: One side-by-side channel profile naming the isolated channel binary root, isolated channel state namespace, explicit cross-channel handoff, and per-channel rollback target so stable and preview channels never corrupt one another and no preview channel reuses a stable state namespace without an explicit import or handoff
  - Required labels: identity, semantic_role, registry_reference, install_mode, state_root
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **portable_mode**: `stable`
  - Owner: Portable-mode owner
  - Canonical schema: `schemas/install/m5-state-root-boundaries.schema.json`
  - Scope: One portable-mode profile naming the self-contained binary root, colocated writable state root, no-machine-global-spill guarantee, and disclosed portable limitations so portable mode never writes hidden machine-global durable settings, secrets, or services and every limitation is disclosed
  - Required labels: identity, semantic_role, registry_reference, install_mode, state_root
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **offline_airgap_bundle**: `stable`
  - Owner: Offline / air-gap delivery owner
  - Canonical schema: `schemas/install/m5-state-root-boundaries.schema.json`
  - Scope: One offline / air-gap bundle profile naming the bundled artifact root, offline updater ownership, bundled policy root, and complete rollback-target set so a rollback restores the full artifact graph rather than only the primary executable and no undisclosed network dependency hides in an air-gapped deployment
  - Required labels: identity, semantic_role, registry_reference, install_mode, rollback_target
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
