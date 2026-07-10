# M5 framework-component matrix contract

This document is the human-readable companion to the frozen M5 framework-component matrix.
The authoritative gate is the Rust validator in
`crates/aureline-templates/src/freeze_the_m5_framework_pack_header_route_endpoint_row_component_service_tree_node_convention_diagnostic_row_generator_preview_sheet_run_config_scaffold_card_and_derived_relationship_banner_component_matrix`.
The checked-in support export under `artifacts/release/m5-framework-component-proof/` is the
single source of truth; the schemas under `schemas/ui/` document the shape.

## Purpose

The matrix freezes the reusable framework-aware and topology-explorer components so
framework-pack, route-explorer, topology-explorer, convention-diagnostics, and generator-review
surfaces stop drifting across claimed M5 framework surfaces. It names each component family once
and binds it to framework pack identity / version / support class, exact-versus-heuristic-versus-
runtime-confirmed certainty, proving-source linkage, local-versus-remote execution boundary,
file / dependency / config impact, and rollback / regenerate recovery language before widening
consumer coverage.

## Component families

- `framework_pack_header` — which pack and version is active (identified versioned, version
  pinned, version drifted, multiple detected, unversioned, unknown pack) and how it is supported
  (officially supported, community supported, experimental, bridge only, deprecated,
  unsupported).
- `route_endpoint_row` — how a route or endpoint is known (exact from source, heuristic
  convention, runtime confirmed, derived by convention, partial evidence, unresolved) and
  whether it is authored or generated (authored, generated, generated then edited, framework
  provided, runtime only, unknown origin).
- `component_service_tree_node` — what a topology node represents (component, service, module,
  dependency edge, external boundary, unknown node) and how the relationship is known (exact
  from source, heuristic inferred, runtime confirmed, derived by convention, partial evidence,
  unresolved).
- `convention_diagnostic_row` — how confident a convention diagnostic is (verified, high
  confidence, heuristic convention, derived by convention, low confidence, unknown) and its
  severity (error, warning, hint, info, suppressed, stale).
- `generator_preview_sheet` — what a generator or codemod will change (file write, dependency
  change, config change, script or task change, no change, unknown impact) and what it permits
  (preview ready, review required, apply ready, rollback available, regenerate available,
  blocked).
- `run_config_scaffold_card` — where a convenience action will actually run (local process,
  container, SSH remote, managed workspace, cloud remote, unknown boundary) and what it writes
  (creates config file, edits config file, adds dependency, no-write preview, rollback
  available, unknown mutation).
- `derived_relationship_banner` — how a relationship is known (exact from source, inferred from
  runtime, heuristic link, derived by convention, partial link, unresolved link) and how firmly
  it links to its proving source (proving source linked, source linked partial, runtime evidence
  only, convention only, no proving source, unknown proving).

## The one controlled certainty vocabulary

Every consumer binds one controlled vocabulary — no surface invents a parallel word:

`core_native`, `framework_pack`, `bridge`, `heuristic_convention`, `verified`,
`derived_by_convention`, `runtime_confirmed`, `partial`.

## Hard invariants

Each component row is a hard `false` on all six of the following. A row that flips any of them
fails validation:

- `hides_pack_identity_version_or_support_class` — a pack header never leaves which pack /
  version is active or how it is supported implicit.
- `lets_heuristic_masquerade_as_exact` — a heuristic route, endpoint, tree, or relationship is
  never presented as exact.
- `implies_no_op_write_while_mutating_config_or_dependencies` — a generator or run-config card
  never implies a no-op write when it changes config or dependencies.
- `hides_local_container_ssh_or_managed_boundary` — the local / container / SSH / managed
  execution boundary never hides behind framework convenience language.
- `omits_proving_source_or_rollback_path` — the proving-source linkage and the rollback /
  regenerate recovery path stay explicit.
- `invents_alternate_state_label` — no surface invents an alternate label for a governed state.

## Downgrade triggers

`pack_identity_unstated`, `support_class_unstated`, `exact_versus_heuristic_unstated`,
`authorship_unstated`, `execution_boundary_unstated`, `impact_undisclosed`,
`proving_source_omitted`, `rollback_path_omitted`, `derived_state_unlabeled`,
`convention_confidence_overstated`, `alternate_state_label_invented`, `proof_stale`.

## Deployment lines and accessibility

Every component keeps the same truth across `local_oss`, `self_hosted`, `managed`, `air_gapped`,
and `mirror_offline`, and offers a non-visual accessibility route (keyboard focusable, screen
reader announced, non-hover reachable, pointer optional, high-contrast safe, support
exportable). No framework truth is hover-only, pointer-only, or visually encoded alone.

## Artifacts

- Support export: `artifacts/release/m5-framework-component-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-framework-component-proof/matrix.csv`
- Design report: `artifacts/design/m5-framework-component-matrix.md`
- Narrowed fixtures: `fixtures/ui/m5-framework-components/`

## Narrowed variants

Two checked-in narrowed fixtures show honest auto-narrowing while every component stays visible:
the `route_endpoint_row` is held at Beta because route resolution is convention- and
runtime-dependent, and the `generator_preview_sheet` is narrowed to Preview pending apply /
rollback and execution-boundary parity proof across every generator-review surface.

## Regenerating

```text
cargo run -p aureline-templates --example dump_framework_component_matrix -- support-export
cargo run -p aureline-templates --example dump_framework_component_matrix -- csv
cargo run -p aureline-templates --example dump_framework_component_matrix -- report
cargo run -p aureline-templates --example dump_framework_component_matrix -- fixture-route-endpoint-row-beta-narrowed
cargo run -p aureline-templates --example dump_framework_component_matrix -- fixture-generator-preview-sheet-preview-narrowed
```
