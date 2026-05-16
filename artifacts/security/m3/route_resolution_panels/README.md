# Provider route-resolution, browser-handoff, and authority-truth panels (beta)

Reviewer-facing artifacts for the route-resolution beta projection. The
projection makes external and browser-linked lanes honest by naming the
current route, owner, grant type, and fallback path on every claimed
provider-linked beta row, plus typed browser-handoff and authority-truth
panels that share the same route and scope model as command and support
packets.

The contract is owned by
[`/crates/aureline-provider/src/route_resolution/mod.rs`](../../../../crates/aureline-provider/src/route_resolution/mod.rs).
The source matrix that lists axes, lane classes, route choices, route
owners, action classes, fallback modes, route-degraded states,
browser-handoff reasons, and authority-truth states lives at
[`route_resolution_matrix.yaml`](route_resolution_matrix.yaml). The
reviewer-facing landing page lives at
[`/docs/security/m3/provider_route_truth_beta.md`](../../../../docs/security/m3/provider_route_truth_beta.md).

## Files

| File | Kind | Notes |
| --- | --- | --- |
| `route_resolution_matrix.yaml` | matrix | Frozen vocabulary, profiles, lane classes, route choices, owners, identity classes, fallback modes, degraded states, browser-handoff reasons, and defect kinds the beta projection must inspect. |
| `baseline_support_export.json` | `providers_route_resolution_beta_support_export_record` | Baseline export wrapping the seeded beta page across the four required profiles. |

## How to regenerate

```sh
cargo run -q -p aureline-provider --bin aureline_provider_route_resolution_beta -- support-export > artifacts/security/m3/route_resolution_panels/baseline_support_export.json
```

Other subcommands:

```sh
cargo run -q -p aureline-provider --bin aureline_provider_route_resolution_beta -- page
cargo run -q -p aureline-provider --bin aureline_provider_route_resolution_beta -- validate
```

`validate` prints the typed defect list. A passing beta page emits an empty
list and exits with status `0`; any defect exits non-zero.
