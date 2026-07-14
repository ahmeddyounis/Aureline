# Shared Install-Topology Consumers: One Registry Across Surfaces

- Packet: `m5-install-topology-shared-consumers:stable:0001`
- Surface: `M5 install-topology shared consumers (one registry across surfaces)`
- Consumer bindings: 15 (6 narrowed)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-14T00:00:00Z)

## Consumer bindings

- **Per-user managed install (user-scoped updater ownership)** [`itsc-per-user-updater`]: family `per_user_managed` on `updater_service`, representation `desktop_full`, role `updater_owner`
- **Per-user managed install (user-scoped updater ownership)** [`itsc-per-user-about`]: family `per_user_managed` on `shell_ui`, representation `desktop_full`, role `updater_owner`
- **Per-user managed install (user-scoped updater ownership)** [`itsc-per-user-cli`]: family `per_user_managed` on `cli_export`, representation `exported_redacted`, role `updater_owner`
- **Per-machine managed install (machine policy roots)** [`itsc-per-machine-admin`]: family `per_machine_managed` on `admin`, representation `desktop_full`, role `policy_roots`
- **Per-machine managed install (machine policy roots)** [`itsc-per-machine-installer`]: family `per_machine_managed` on `installer`, representation `desktop_full`, role `policy_roots`
- **Per-machine managed install (machine policy roots)** [`itsc-per-machine-support`]: family `per_machine_managed` on `support_export`, representation `exported_redacted`, role `policy_roots`
- **Side-by-side stable-plus-preview (isolated channel state namespace)** [`itsc-side-by-side-diagnostics`]: family `side_by_side_stable_preview` on `diagnostics`, representation `desktop_full`, role `writable_state_roots`
- **Side-by-side stable-plus-preview (isolated channel state namespace)** [`itsc-side-by-side-about`]: family `side_by_side_stable_preview` on `shell_ui`, representation `desktop_full`, role `writable_state_roots`
- **Side-by-side stable-plus-preview (isolated channel state namespace)** [`itsc-side-by-side-product`]: family `side_by_side_stable_preview` on `product_ui`, representation `remote_projected`, role `writable_state_roots`
- **Portable mode (colocated install-mode disclosure)** [`itsc-portable-docs`]: family `portable_mode` on `docs_help`, representation `desktop_full`, role `install_mode`
- **Portable mode (colocated install-mode disclosure)** [`itsc-portable-diagnostics`]: family `portable_mode` on `diagnostics`, representation `desktop_full`, role `install_mode`
- **Portable mode (colocated install-mode disclosure)** [`itsc-portable-product`]: family `portable_mode` on `product_ui`, representation `remote_projected`, role `install_mode`
- **Offline / air-gap bundle (complete rollback-target set)** [`itsc-offline-admin`]: family `offline_airgap_bundle` on `admin`, representation `desktop_full`, role `rollback_target`
- **Offline / air-gap bundle (complete rollback-target set)** [`itsc-offline-installer`]: family `offline_airgap_bundle` on `installer`, representation `compact_narrowed`, role `rollback_target`
- **Offline / air-gap bundle (complete rollback-target set)** [`itsc-offline-support`]: family `offline_airgap_bundle` on `support_export`, representation `exported_redacted`, role `rollback_target`
