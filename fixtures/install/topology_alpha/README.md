# Install Topology Alpha Fixtures

These fixtures exercise the alpha install-topology contract consumed by
`crates/aureline-install`. They are protected proof inputs for product and
support surfaces that need to explain install mode, channel, updater owner,
state roots, silent deployment limits, handler ownership, repair or verify
support, mirror/offline posture, and rollback owner without reading installer
logs.

`install_topology_alpha_packet.json` covers:

- per-user Stable with Preview side-by-side behavior;
- side-by-side Preview with distinct durable state roots;
- per-machine admin-owned install;
- portable/unpacked mode with no host-global handler ownership;
- managed fleet rollout with ring and rollback owner truth;
- customer-managed mirror delivery;
- air-gapped bundle delivery.

Run:

```bash
cargo test -p aureline-install
```
