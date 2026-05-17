# Install Topology Beta Baseline

This baseline turns the install-topology matrix into a supportable diagnostic
packet for beta rows. It does not implement an installer or fleet-control
service; it freezes the evidence that product, CLI, support export, and
enterprise inventory surfaces must render before they describe an install.

## Source Artifacts

- Topology contract:
  [`artifacts/release/install_topology_matrix.yaml`](../../../artifacts/release/install_topology_matrix.yaml)
- State-root map:
  [`artifacts/release/state_root_map.yaml`](../../../artifacts/release/state_root_map.yaml)
- Exact-build identity schema:
  [`schemas/build/exact_build_identity.schema.json`](../../../schemas/build/exact_build_identity.schema.json)
- Diagnostics schema:
  [`schemas/release/install_diagnostics.schema.json`](../../../schemas/release/install_diagnostics.schema.json)
- Canonical diagnostics packet:
  [`artifacts/release/m3/install_diagnostics/install_diagnostics_packet.json`](../../../artifacts/release/m3/install_diagnostics/install_diagnostics_packet.json)
- Support-export projection:
  [`artifacts/release/m3/install_diagnostics/support_export_projection.json`](../../../artifacts/release/m3/install_diagnostics/support_export_projection.json)

## Product And CLI Truth

Every row in the diagnostics packet carries the same truth fingerprint across
About, update, diagnostics, install-review, CLI, and support-export surfaces:

- install mode;
- channel;
- updater owner;
- binary-root class;
- exact-build identity ref;
- durable state-root refs;
- policy-root refs;
- state-root review class;
- last verification state;
- rollback target.

Surfaces may render different labels, but they must not compute these fields
independently. The Rust consumer validates this by comparing surface
fingerprints from the same packet.

## Portable Baseline

The portable row is constrained to
`state.portable_colocated_root.portable_stable`. It carries
`portable_no_os_ownership`, exposes the root in product, CLI, and support export,
and names no policy root. A portable diagnostic row that points at an installed
profile root or claims an admin policy root is non-conforming.

## Side-By-Side Baseline

The Stable and Preview rows name each other as peers and require
`explicit_import_review_required`. Their durable roots are channel-suffixed and
must not overlap unless a future row is explicitly marked shared read-only.
Shared file associations or protocol defaults remain governed by the ownership
audit; diagnostics only confirms that mutable state is not silently shared.

## Fleet Rollout Baseline

The managed fleet row includes a `fleet_rollout` block with:

- rollout ring;
- managed-package report ref;
- inventory probe ref;
- policy-root refs;
- rollback target;
- exact-build identity ref;
- verification-status evidence.

This lets fleet tooling answer "which exact build is installed on this host?"
from the managed package report and support-export packet instead of reading
host-specific installer logs.

## Verification

Run:

```bash
cargo test -p aureline-install --test install_diagnostics_beta
```

The test validates the packet, compares product/CLI/support truth
fingerprints, confirms side-by-side and portable root isolation, verifies the
checked-in support-export projection, and rejects representative stale or
unsafe diagnostics mutations.
