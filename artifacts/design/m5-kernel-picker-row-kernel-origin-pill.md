# Kernel picker rows and kernel origin pills

- Packet: `m5-kernel-picker-row-kernel-origin-pill-controls:stable:0001`
- Surface: `M5 kernel picker rows and kernel origin pills: kernel class, environment identity, locality, trust limits, exact or degraded provenance, and rerun/reattach continuity across claimed notebook surfaces`
- Kernel picker rows: 6 (3 not selectable right now)
- Kernel origin pills: 6 (4 not exact provenance)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-11T00:00:00Z)

## Kernel picker rows

- **CPython 3.11 (local interpreter)** — kind `local_interpreter`, selection `selected` → `currently_selected`, last seen `Last seen: now (attached)`, deep link `kernel_manager`
- **analysis-venv (virtual env)** — kind `virtual_env`, selection `recommended` → `recommended_choice`, last seen `Last seen: 2 minutes ago`, deep link `kernel_manager`
- **ml-conda (conda env)** — kind `conda_env`, selection `available` → `available_choice`, last seen `Last seen: 10 minutes ago`, deep link `docs_anchor`
- **legacy-py38 (container)** — kind `container_kernel`, selection `incompatible` → `incompatible_choice`, last seen `Last seen: 1 hour ago`, deep link `docs_anchor`
- **gpu-pool (remote)** — kind `remote_kernel`, selection `needs_install` → `needs_setup_first`, last seen `Last seen: available for provisioning`, deep link `kernel_manager`
- **team-managed (managed)** — kind `managed_kernel`, selection `unavailable` → `unavailable_choice`, last seen `Last seen: 30 minutes ago`, deep link `support_bundle`

## Kernel origin pills

- **Kernel origin: local host** — origin `local_host`, trust `trusted_origin` → `exact_provenance`, fingerprint `fingerprint_matched`, deep link `kernel_manager`
- **Kernel origin: SSH remote** — origin `ssh_remote`, trust `first_party` → `exact_provenance`, fingerprint `fingerprint_matched`, deep link `kernel_manager`
- **Kernel origin: container** — origin `container`, trust `third_party` → `degraded_provenance`, fingerprint `fingerprint_drifted`, deep link `docs_anchor`
- **Kernel origin: devcontainer** — origin `devcontainer`, trust `unverified_origin` → `degraded_provenance`, fingerprint `fingerprint_drifted`, deep link `docs_anchor`
- **Kernel origin: managed workspace** — origin `managed_workspace`, trust `restricted_origin` → `restricted_provenance`, fingerprint `fingerprint_unknown`, deep link `support_bundle`
- **Kernel origin: browser bridge** — origin `browser_bridge`, trust `unknown_origin` → `unknown_provenance`, fingerprint `fingerprint_not_evaluated`, deep link `no_deep_link`
