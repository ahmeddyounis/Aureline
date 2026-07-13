# M5 Install-Topology, Mutable-State-Boundary, Portable-Update, and Fleet-Rollout Contract

Status: frozen (B140 opening matrix)

This contract freezes Aureline's concrete delivery-topology behavior into one export-safe matrix. It is
the canonical source of install-topology truth for M5: later packaging flows, updater/state-root
resolution, About / update / diagnostics / admin surfaces, docs/help, support/export, and rollout-evidence
tooling consume it directly rather than copying packaging prose by hand.

- Matrix schema: `schemas/install/m5-install-topology-matrix.schema.json`
- Install-topology domain schema (install mode / updater owner / binary root): `schemas/install/m5-install-topology.schema.json`
- State-root-boundaries domain schema (writable state roots / policy roots): `schemas/install/m5-state-root-boundaries.schema.json`
- Support export: `artifacts/release/m5-install-topology-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-install-topology-proof/matrix.csv`
- Design report: `artifacts/install/m5-install-topology-matrix.md`
- Narrowed fixtures: `fixtures/install/m5-delivery-topologies/`
- Authoritative validator: `crates/aureline-ui` (`m5_install_topology_matrix`)
- Emitter (single mint-from-truth path): `cargo run -p aureline-ui --example dump_m5_install_topology_matrix`

## Governed delivery-topology families

The matrix freezes **five** delivery-topology families, each qualified independently and each pointing at
one canonical domain schema:

| Family | Install mode | Updater owner | Binary root | Writable state root | Domain schema |
| --- | --- | --- | --- | --- | --- |
| `per_user_managed` | Per-user managed | Per-user updater | User-scoped binary root | User-writable state root | install-topology |
| `per_machine_managed` | Per-machine managed | Admin / system updater | Machine-scoped binary root | Shared machine state root | install-topology |
| `side_by_side_stable_preview` | Coexisting stable + preview | Per-channel updater | Isolated channel binary root | Isolated channel state namespace | install-topology |
| `portable_mode` | Portable, self-contained | Portable updater (no machine-global spill) | Self-contained binary root | Colocated writable state root | state-root-boundaries |
| `offline_airgap_bundle` | Offline / air-gap bundle | Offline updater | Bundled artifact root | Bundled policy + state root | state-root-boundaries |

## Shared install-topology-role vocabulary

Every consumer binds to one controlled role vocabulary; no surface invents a parallel word:

`install_mode`, `updater_owner`, `binary_root`, `writable_state_roots`, `policy_roots`, `rollback_target`,
`rollout_ring`.

The ownership / isolation roles (`updater_owner`, `writable_state_roots`, `policy_roots`,
`rollback_target`) must preserve state isolation and ownership under coexistence — a topology change may
never hide who owns the updater, spill durable state into hidden machine-global paths, corrupt a coexisting
channel, or narrow rollback below the full artifact graph. The descriptive placement / identity roles
(`install_mode`, `binary_root`, `rollout_ring`) are inspectable descriptors.

## Hard invariants

Every row carries five hard-invariant booleans that must be `false`, and the governance-review block
asserts the corresponding fleet-level guarantees:

1. Portable mode never writes hidden machine-global durable settings, secrets, or services.
2. A preview channel never reuses a stable state namespace without an explicit import / handoff.
3. A rollback never targets only the primary executable while sidecars or metadata drift — it restores the
   full artifact graph.
4. Updater ownership and admin control are never hidden in a managed flow.
5. A deployment claim never outpaces its rollout-ring or repair / verify evidence.

## Automatic narrowing

Claim publication and support/export narrow install-topology claims automatically when the B140 registry is
missing, stale, or not yet qualified. Two narrowed fixtures demonstrate honest narrowing while keeping
every family visible:

- `side_by_side_channel_beta_narrowed.json` — side-by-side coexistence held at **Beta**.
- `offline_airgap_bundle_preview_narrowed.json` — offline / air-gap bundle narrowed to **Preview** pending
  complete rollback and rollout-ring evidence.

## Bound source contracts

The matrix binds back to already-landed truth so install-topology truth is never split across scattered
notes: the coexistence / fleet-rollout schema (`schemas/install/m5-coexistence-and-fleet-rollout.schema.json`)
and the native-desktop matrix (`schemas/platform/m5-native-desktop-matrix.schema.json`).
