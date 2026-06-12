# Crate dependency rules

This document is the source of truth for which seeded crates are allowed to
depend on which other seeded crates. Future validation tooling reads this file
(or the structured form in `artifacts/governance/package_inventory.yaml`) to
gate pull requests.

Protected service-plane direction and hot-path module sentinels are now
tracked separately in:

- `docs/architecture/service_topology_and_process_placement.md`
- `artifacts/architecture/protected_path_dependency_rules.yaml`
- `artifacts/architecture/process_placement_map.yaml`

This file stays focused on exact crate edges and layering. The architecture
artifacts answer the adjacent protected-path questions: which plane owns a
crate or module, where that work may run, and which hot-path imports must
fail CI immediately.

## Layering

Crates are organized into ascending layers. A crate may depend only on crates
in a strictly lower layer, with the explicit exemptions noted below.

| Layer | Crates                                                  |
|-------|---------------------------------------------------------|
| L0    | `aureline-build-info`, `aureline-text`, `aureline-telemetry` |
| L1    | `aureline-commands`, `aureline-rpc`, `aureline-render`, `aureline-buffer`, `aureline-ui`, `aureline-workspace`, `aureline-vfs` |
| L2    | `aureline-editor`, `aureline-input`                     |
| L3    | `aureline-shell`, `aureline-shell-spike`                |
| LX    | `aureline-bench`, `aureline-largefile-proto`, `aureline-reactive-state`, `aureline-graph-proto` (off the cone; explicit allowances listed below) |

## Allowed edges

| From                    | May depend on                                               |
|-------------------------|-------------------------------------------------------------|
| `aureline-build-info`   | (no internal deps)                                          |
| `aureline-text`         | (no internal deps)                                          |
| `aureline-telemetry`    | (no internal deps)                                          |
| `aureline-commands`     | (no internal deps)                                          |
| `aureline-rpc`          | `aureline-telemetry`                                        |
| `aureline-render`       | `aureline-text`, `aureline-telemetry`                       |
| `aureline-buffer`       | `aureline-text`, `aureline-telemetry`                       |
| `aureline-ui`           | (no internal deps)                                          |
| `aureline-workspace`    | (no internal deps)                                          |
| `aureline-vfs`          | `aureline-text`, `aureline-telemetry`                       |
| `aureline-editor`       | `aureline-buffer`, `aureline-render`, `aureline-text`, `aureline-ui` |
| `aureline-input`        | `aureline-commands`                                         |
| `aureline-shell`        | `aureline-build-info`, `aureline-commands`, `aureline-editor`, `aureline-input`, `aureline-render`, `aureline-text`, `aureline-buffer`, `aureline-ui`, `aureline-workspace`, `aureline-vfs`, `aureline-rpc`, `aureline-telemetry` |
| `aureline-shell-spike`  | any seeded crate                                            |
| `aureline-bench`        | any seeded crate                                            |
| `aureline-largefile-proto` | (no internal deps today; experimental/off-cone)         |
| `aureline-reactive-state` | (no internal deps today; experimental/off-cone)          |
| `aureline-graph-proto`  | (no internal deps today; experimental/off-cone)             |

## Incremental contract-crate edges

The following governed contract crates landed after the initial seed table.
Until the full layer map is rewritten around the broader workspace, these
crate-local edges are the allowed internal dependency deltas for the touched
M5 surface packets:

| From                | May depend on |
|---------------------|---------------|
| `aureline-auth`     | (no additional internal deps in this lane) |
| `aureline-api`      | `aureline-auth` |
| `aureline-data`     | `aureline-auth` |
| `aureline-infra`    | `aureline-auth` |
| `aureline-provider` | `aureline-auth`, `aureline-support` |
| `aureline-remote`   | `aureline-auth` |
| `aureline-support`  | (no additional internal deps in this lane) |

## Forbidden edges (non-exhaustive)

The following edges are explicitly disallowed and should fail review:

- L0 crates depending on L1, L2, or L3 crates.
- `aureline-rpc` depending on `aureline-render`, `aureline-buffer`,
  `aureline-vfs`, `aureline-text`, or `aureline-shell-spike`.
- `aureline-render`, `aureline-buffer`, or `aureline-vfs` depending on each
  other (siblings on the same layer must not cross-couple).
- Any production crate depending on `aureline-shell-spike`,
  `aureline-bench`, `aureline-largefile-proto`,
  `aureline-reactive-state`, or `aureline-graph-proto`.
- Cycles of any length.

## Process rules

- New internal crates must land alongside an entry in
  `artifacts/governance/package_inventory.yaml` and an updated edge table here.
- Promoting a new edge requires updating both this document and the inventory
  in the same change; the validator treats them as the joint source of truth.
- Renames or relocations must update `docs/repo/topology.md`,
  `dependency_rules.md`, and `package_inventory.yaml` atomically.
- Spike crates (`*-spike`) are time-boxed. Each spike must carry a documented
  removal trigger and must not accumulate downstream consumers.
- Off-cone prototype crates (`aureline-largefile-proto`,
  `aureline-reactive-state`, `aureline-graph-proto`) stay non-production
  until their promotion lands with updated package inventory, ownership, and
  dependency policy in the same change.

## Out of scope for the seed

- Third-party (crates.io) dependency policy: tracked separately under release
  engineering (provenance, SBOM, license review).
- Feature-flag policy across crates: deferred until the first production
  crate-pair needs it.
