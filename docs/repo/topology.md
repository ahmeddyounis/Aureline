# Repository topology

This document is the authoritative map of where things live in the Aureline
repository. It is normative for path expectations: future tooling, governance
checks, and CI gates may consume it. Move-don't-fork: when a directory needs
to change, update this map in the same change.

## Top-level layout

| Path           | Purpose                                                                                              |
|----------------|------------------------------------------------------------------------------------------------------|
| `Cargo.toml`   | Root Cargo workspace manifest. Lists every internal crate.                                           |
| `CODEOWNERS`   | Pull-request review routing. Paired with `artifacts/governance/ownership_matrix.yaml` for ownership. |
| `crates/`      | All Rust crates. One directory per crate; crate name matches directory name.                        |
| `docs/`        | Design and governance docs that ship with the repository (not external product docs).                |
| `schemas/`     | Machine-readable schemas (JSON Schema, protobuf, etc.) consumed by tooling and runtime.              |
| `fixtures/`    | Reusable test inputs and golden artifacts. Subtrees grow per protected-path corpus.                  |
| `tools/`       | Repository-local tooling (lint helpers, codegen scripts, governance checkers).                       |
| `ci/`          | CI configuration shared across pipelines (job definitions, gate scripts).                            |
| `artifacts/`   | Checked-in evidence and governance outputs. Subtrees: `governance/`, `compat/`, `evidence/`, `release/`, `platform/`, `qa/`, `ux/`, `accessibility/`, `support/`.  |

## Reserved subtrees inside `artifacts/`

| Path                   | Purpose                                                                              |
|------------------------|--------------------------------------------------------------------------------------|
| `artifacts/governance/`| Package inventory, ownership matrix, requirement/waiver registers, scorecard/packet templates, and public-truth claim/parity seeds. |
| `artifacts/compat/`    | Compatibility qualification matrix, version-skew register, and boundary-row seeds.   |
| `artifacts/evidence/`  | Shared evidence metadata catalogs and freshness field vocabularies used by release-facing packets. |
| `artifacts/release/`   | Release-artifact graph rules, provenance, SBOMs, compatibility reports, claim manifests, rollback packets. |
| `artifacts/platform/`  | Claimed desktop profile registry and platform-owned primitive matrices that bind desktop claims to named OS/profile rows. |
| `artifacts/qa/`        | Seed QA verification matrices and claimed-profile continuity drill vocabularies. |
| `artifacts/ux/`        | Design-system snapshots and UX review packets.                    |
| `artifacts/accessibility/` | Accessibility and platform-input matrices, shell-conformance checklists, and accessibility-tree coverage rows. |
| `artifacts/support/`   | Support-bundle templates, recovery drill outputs, Project Doctor seeded scenarios.   |

The `artifacts/compat/` subtree is the pre-release compatibility seed
home. Later release-time compatibility reports still land under
`artifacts/release/`; they extend the seeded row ids rather than
replacing them.

Other M0 tasks land additional subtrees (release engineering, UX,
accessibility, support); they extend this map rather than relocating
it.

## Seeded crates

| Crate                     | Path                                  | Role                                                                  |
|---------------------------|---------------------------------------|-----------------------------------------------------------------------|
| `aureline-shell-spike`    | `crates/aureline-shell-spike/`        | Throwaway end-to-end spike for the shell.                             |
| `aureline-render`         | `crates/aureline-render/`             | GPU-accelerated rendering primitives.                                 |
| `aureline-text`           | `crates/aureline-text/`               | Foundational text encoding and segmentation.                          |
| `aureline-buffer`         | `crates/aureline-buffer/`             | Editor buffer core: piece tree, selections, undo/redo.                |
| `aureline-vfs`            | `crates/aureline-vfs/`                | Workspace VFS: roots, watchers, canonical path identity.              |
| `aureline-rpc`            | `crates/aureline-rpc/`                | Cross-process RPC transport.                                          |
| `aureline-telemetry`      | `crates/aureline-telemetry/`          | Hot-path instrumentation, tracing, metrics.                           |
| `aureline-bench`          | `crates/aureline-bench/`              | Benchmark harness and trace-fixture host.                             |
| `aureline-largefile-proto`| `crates/aureline-largefile-proto/`    | Off-cone prototype for large-file capability posture and limited mode. |
| `aureline-reactive-state` | `crates/aureline-reactive-state/`     | Off-cone prototype for reactive state and subscription-envelope lanes. |
| `aureline-graph-proto`    | `crates/aureline-graph-proto/`        | Off-cone prototype for semantic workspace graph and query vocabularies. |

## Layering at a glance

```
                aureline-shell-spike      (spike, not protected)
                       |
                       v
        +--------------+--------------+
        |              |              |
   aureline-render  aureline-buffer  aureline-vfs
        |              |              |
        +------+-------+------+-------+
               |              |
         aureline-text   aureline-telemetry   aureline-rpc
                                                  |
                                            aureline-telemetry

 aureline-bench, aureline-largefile-proto, aureline-reactive-state,
 and aureline-graph-proto are layer LX off-cone crates; nothing in the
 production cone depends on them.
```

`aureline-text` and `aureline-telemetry` are leaf foundations. `aureline-rpc`
depends only on `aureline-telemetry`. The spike crate is allowed to reach
across layers because it is explicitly disposable. The prototype crates are
also disposable/off-cone: they validate frozen vocabularies and artifact
shapes without widening the protected dependency cone.

The seeded crates do not yet declare internal dependencies; they will be added
incrementally and validated against the rules in `dependency_rules.md`.

## Product boundary

Every crate above is on the local-core side of the open-source core versus
managed / service-plane boundary. The boundary is drawn explicitly in
[`/docs/product/boundary_manifest_strawman.md`](../product/boundary_manifest_strawman.md)
and conforms to
[`/schemas/product/boundary_manifest.schema.json`](../../schemas/product/boundary_manifest.schema.json).
When a new crate, service, or managed dependency is added, it must map to an
existing boundary-manifest row or land a new row in the same change;
introducing a capability without a boundary row is a governance error.
