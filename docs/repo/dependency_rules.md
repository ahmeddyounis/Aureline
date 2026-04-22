# Crate dependency rules

This document is the source of truth for which seeded crates are allowed to
depend on which other seeded crates. Future validation tooling reads this file
(or the structured form in `artifacts/governance/package_inventory.yaml`) to
gate pull requests.

## Layering

Crates are organized into ascending layers. A crate may depend only on crates
in a strictly lower layer, with the explicit exemptions noted below.

| Layer | Crates                                                  |
|-------|---------------------------------------------------------|
| L0    | `aureline-text`, `aureline-telemetry`                   |
| L1    | `aureline-rpc`                                          |
| L2    | `aureline-render`, `aureline-buffer`, `aureline-vfs`    |
| L3    | `aureline-shell-spike`                                  |
| LX    | `aureline-bench`, `aureline-largefile-proto`, `aureline-reactive-state`, `aureline-graph-proto` (off the cone; explicit allowances listed below) |

## Allowed edges

| From                    | May depend on                                               |
|-------------------------|-------------------------------------------------------------|
| `aureline-text`         | (no internal deps)                                          |
| `aureline-telemetry`    | (no internal deps)                                          |
| `aureline-rpc`          | `aureline-telemetry`                                        |
| `aureline-render`       | `aureline-text`, `aureline-telemetry`                       |
| `aureline-buffer`       | `aureline-text`, `aureline-telemetry`                       |
| `aureline-vfs`          | `aureline-text`, `aureline-telemetry`                       |
| `aureline-shell-spike`  | any seeded crate                                            |
| `aureline-bench`        | any seeded crate                                            |
| `aureline-largefile-proto` | (no internal deps today; experimental/off-cone)         |
| `aureline-reactive-state` | (no internal deps today; experimental/off-cone)          |
| `aureline-graph-proto`  | (no internal deps today; experimental/off-cone)             |

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
