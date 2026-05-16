# Install ownership audit fixture

`install_ownership_audit_packet.json` is the protected fixture for the
desktop-entry ownership audit projection in
[`crates/aureline-install/src/ownership_audit/`](../../../crates/aureline-install/src/ownership_audit/).

The packet references rows from the install topology alpha fixture in
[`fixtures/install/topology_alpha/install_topology_alpha_packet.json`](../topology_alpha/install_topology_alpha_packet.json)
and projects which channel/build owns each OS-level handoff surface
across stable+preview, stable+portable, stable+managed, and
air-gapped layouts. Every dispatching surface lists the deep-link
route checks the validator applies and asserts the in-product
invocation runs the same family.

Run `cargo test -p aureline-install` to verify the fixture round-trips
through validation, surface projection, support export, and the
portable / managed / displaced-owner posture rules.
