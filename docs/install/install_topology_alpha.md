# Install Topology Alpha

This page is the reviewer entry point for the alpha install-topology
implementation in [`/crates/aureline-install`](../../crates/aureline-install).
The crate turns the release topology artifacts into one typed packet consumed by
About, update, diagnostics, install-review, CLI, and support-export surfaces.

The model is intentionally bounded. It does not install, update, repair,
rollback, or manage a fleet. It makes the currently claimed topology truth
inspectable and testable before those mutating systems exist.

## Canonical Inputs

- [`/artifacts/release/install_topology_matrix.yaml`](../../artifacts/release/install_topology_matrix.yaml)
  - install profile cards, updater owner, binary root, rollout ring, repair
  and verify support, and silent install support.
- [`/artifacts/release/state_root_map.yaml`](../../artifacts/release/state_root_map.yaml)
  - durable state-root refs, update markers, recent-item registration, file
  association posture, and protocol-handler ownership.
- [`/artifacts/release/silent_deployment_seed.yaml`](../../artifacts/release/silent_deployment_seed.yaml)
  - unattended result and return-code family vocabulary.
- [`/artifacts/governance/boundary_manifest_alpha.yaml`](../../artifacts/governance/boundary_manifest_alpha.yaml)
  - managed boundary truth consumed by managed and mirror rows.

## Implementation

- [`/crates/aureline-install/src/topology/mod.rs`](../../crates/aureline-install/src/topology/mod.rs)
  defines `InstallTopologyAlphaPacket`, row vocabularies, handler change
  previews, stale handler diagnostics, validation, and surface projections.
- [`/fixtures/install/topology_alpha/install_topology_alpha_packet.json`](../../fixtures/install/topology_alpha/install_topology_alpha_packet.json)
  is the protected fixture packet.
- [`/crates/aureline-install/tests/topology_alpha.rs`](../../crates/aureline-install/tests/topology_alpha.rs)
  proves validation, product/support surface consistency, side-by-side state
  separation, portable-mode limits, silent-deployment posture, and handler
  owner diagnostics.

## Claimed Rows

| Row | What it proves |
| --- | --- |
| `install.topology.windows.per_user.stable` | Stable per-user local install with Preview side-by-side posture, user-owned updates, and per-channel handler defaults. |
| `install.topology.windows.preview.side_by_side` | Preview side-by-side install with distinct state roots and handler ownership staged through review. |
| `install.topology.windows.per_machine.stable` | Per-machine admin-owned install with policy bootstrap, inventory hooks, silent deployment, and admin rollback owner. |
| `install.topology.windows.portable.stable` | Portable mode with colocated state, no file/protocol handler registration, no hidden host-global durable state, and user-owned bundle swap rollback. |
| `install.topology.windows.managed.stable` | Managed fleet lane with ring pinning, managed package report, policy roots, inventory hooks, and fleet-owned rollback. |
| `install.topology.linux.customer_mirror.stable` | Customer-managed mirror lane with mirror metadata verification and admin-owned rollback. |
| `install.topology.airgap.bundle.stable` | Air-gapped signed bundle with offline verification and rollback pack ownership. |

## Surface Contract

Every row must render on all of these surfaces:

- About
- Update
- Diagnostics
- Install review
- CLI
- Support export

The crate derives a shared `InstallTopologyTruthFingerprint` for each row and
the tests assert every surface projects the same fingerprint. A surface cannot
claim a different install mode, channel, updater owner, binary root, durable
state-root set, repair/verify posture, mirror/offline state, handler owner, or
rollback owner without failing the protected path.

## Handler Ownership

Side-by-side and portable rows must not rely on last-writer-wins behavior.

- Stable and Preview use distinct durable state-root refs.
- File associations are selectable candidate handlers, not silent default
  takeovers.
- Protocol/deep-link handlers use channel-suffixed schemes.
- Shared default-open behavior requires explicit user or admin selection.
- A handler-owner change from Stable to Preview is represented by
  `install.handler.preview.windows.shared_default.stable_to_preview` before
  commit.
- A displaced owner can be diagnosed by
  `install.handler.diagnostic.windows.displaced_stable_owner` without reading
  installer logs.

Portable rows must keep file associations, protocol handlers, services, and
host-global recent-item registration closed unless a future explicit
integration review adds a new row.

## Verification

```bash
cargo test -p aureline-install
```

The command covers:

- fixture validation;
- About/update/diagnostics/install-review/CLI/support-export parity;
- Stable and Preview state-root separation;
- handler owner change preview before commit;
- stale/displaced handler diagnostics;
- portable-mode global integration rejection;
- silent-deployment limits and rollback-owner disclosure.
