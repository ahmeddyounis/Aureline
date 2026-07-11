# M5 Notebook Document/Kernel/Output Component Profile Certification

- Packet: `m5-notebook-kernel-output-component-certification:stable:0001`
- As of: `2026-07-11T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-notebook-kernel-output-proof/support_export.json`
- Profiles: 8 / 8 certified (4 green, 4 yellow, 0 red)
- Families covered: true
- Guardrails held: true
- Auto-narrowed profiles: 4
- Report clean: true

## Profiles

- **cert:local-trusted-kernel** — profile=local_trusted_kernel claimed=live_trusted_result certified=live_trusted_result status=green narrowed_axes=0
- **cert:remote-isolated-kernel** — profile=remote_isolated_kernel claimed=reviewable_result certified=reviewable_result status=green narrowed_axes=0
- **cert:managed-kernel** — profile=managed_kernel claimed=reviewable_result certified=reviewable_result status=green narrowed_axes=0
- **cert:trusted-local-output** — profile=trusted_local_output claimed=reviewable_result certified=reviewable_result status=green narrowed_axes=0
- **cert:stale-output** — profile=stale_output claimed=reviewable_result certified=stale_output_projection status=yellow narrowed_axes=1
- **cert:degraded-origin-kernel** — profile=degraded_origin_kernel claimed=reviewable_result certified=degraded_origin_projection status=yellow narrowed_axes=1
- **cert:restarted-kernel** — profile=restarted_kernel claimed=reviewable_result certified=no_kernel_projection status=yellow narrowed_axes=1
- **cert:disconnected-reconnecting-kernel** — profile=disconnected_reconnecting_kernel claimed=reviewable_result certified=partial_kernel_parity_projection status=yellow narrowed_axes=1
