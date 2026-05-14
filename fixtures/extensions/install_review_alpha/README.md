# Install-review alpha fixtures

These JSON fixtures exercise the marketplace/package install-review alpha
contract implemented in `crates/aureline-extensions/src/install_review/`.

The fixtures intentionally compose existing truth sources instead of cloning
their contracts:

- manifest and effective-permission vocabulary from `crates/aureline-extensions`;
- provider/account scope vocabulary from `crates/aureline-provider`;
- runtime reachability and host-boundary vocabulary from `crates/aureline-runtime`;
- service-boundary vocabulary from `crates/aureline-auth`;
- install-topology rows from `fixtures/install/topology_alpha/install_topology_alpha_packet.json`.

## Case index

| Fixture | Covers |
| --- | --- |
| `native_marketplace_package_lane.json` | Product-owned native review sheet admits after rendering permission deltas, compatibility labels, activation-budget evidence, and install-topology truth. |
| `hosted_provider_lane_parity_denied.json` | Provider-owned hosted lane renders owner/origin/scope/network/boundary truth but is denied when it tries to approve directly. |
| `hosted_provider_lane_hidden_boundary_denied.json` | Hosted lane is denied when it hides service-boundary disclosure before enablement. |
