# Finalize qualification packets for optional surfaces — fixture cases

Negative fixtures for the M4 finalized optional-surface qualification register.
Each JSON file is a complete register that is structurally valid except for one
targeted flaw.

| Case | Flaw | Expected check id |
|---|---|---|
| `deployment_stable_on_narrowed_surface.json` | A deployment row renders stable while the surface is narrowed | `surface.deployment_stable_on_narrowed_surface` |
| `deployment_stable_without_packet.json` | A deployment row renders stable while the surface has no packet | `surface.deployment_stable_without_packet` |
| `missing_deployment_coverage.json` | A release-relevant surface lacks deployment qualifications for all targets | `surface.deployment_coverage_incomplete` |
