# M5 Build/Remote-Boundary Component Profile Certification

- Packet: `m5-build-remote-boundary-component-certification:stable:0001`
- As of: `2026-07-11T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-build-remote-boundary-proof/support_export.json`
- Profiles: 8 / 8 certified (4 green, 4 yellow, 0 red)
- Families covered: true
- Guardrails held: true
- Auto-narrowed profiles: 4
- Report clean: true

## Profiles

- **cert:local-execution** — profile=local_execution claimed=full_truth certified=full_truth status=green narrowed_axes=0
- **cert:ssh-execution** — profile=ssh_execution claimed=resolved_truth certified=resolved_truth status=green narrowed_axes=0
- **cert:container-execution** — profile=container_execution claimed=resolved_truth certified=resolved_truth status=green narrowed_axes=0
- **cert:devcontainer-execution** — profile=devcontainer_execution claimed=resolved_truth certified=resolved_truth status=green narrowed_axes=0
- **cert:managed-workspace** — profile=managed_workspace claimed=resolved_truth certified=degraded status=yellow narrowed_axes=1
- **cert:suspend-resume** — profile=suspend_resume claimed=resolved_truth certified=unverified status=yellow narrowed_axes=1
- **cert:rebuild-recreate** — profile=rebuild_recreate claimed=resolved_truth certified=unverified status=yellow narrowed_axes=1
- **cert:expiry-local-safe** — profile=expiry_local_safe claimed=resolved_truth certified=unsupported status=yellow narrowed_axes=1
