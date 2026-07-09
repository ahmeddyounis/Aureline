# Implement package explorer rows

Status: Implemented (M05-973, batch B115)

This contract narrows the `package_explorer_row` component frozen in
[`m5-package-management-component-matrix`](freeze_the_m5_package_management_component_matrix.md)
(M05-972) into an implemented, export-safe package browse row. It makes one row
tell the truth about scope, provenance, and action *before* a user clicks
install, update, or remove: from the row alone a reader can see what package is
being acted on, which manifest scope owns it, whether it is direct or transitive,
which registry answered, and what the license, advisory, and changelog signals
say.

- Boundary schema: [`schemas/ui/m5-package-explorer-row.schema.json`](../../../schemas/ui/m5-package-explorer-row.schema.json)
- Producer: `aureline_deps::current_package_explorer_row_export`
- Release proof: [`artifacts/release/m5-package-explorer-row-proof/`](../../../artifacts/release/m5-package-explorer-row-proof/)
- Protected fixtures: [`fixtures/ui/m5-package-explorer-row/`](../../../fixtures/ui/m5-package-explorer-row/)

## What the component carries

Every `PackageExplorerRow` reuses the frozen `M5PackageComponent` tag (gated to
`package_explorer_row`) and answers, from the row alone:

- **Package identity** (`package_label`, `ecosystem`, `current_version`,
  `candidate_version`).
- **Manifest scope** (`manifest_scope`: `runtime_dependency` / `dev_dependency`
  / `optional_dependency` / `peer_dependency` / `build_dependency` /
  `workspace_catalog`) and its **disclosure** (`manifest_scope_disclosure`,
  required and non-empty — which manifest file/section owns the declaration).
- **Direct/transitive relation** (`relation`: `direct` / `transitive` /
  `direct_and_transitive`) with a **relation note** (`relation_note`, required
  when not direct — names the parent that pulls a transitive package).
- **Registry source** (`registry_source`: `public_registry` /
  `private_registry` / `enterprise_mirror` / `offline_snapshot` / `git_source` /
  `path_or_vendored`) with its **disclosure** and a reused
  `M5PackageComponentDegradationState` plus a **degradation note** whenever
  resolution is not exact (mirror-backed, offline-snapshot, auth-required, or
  stale never reads as a clean install).
- **License / advisory / changelog signals** (`license_signal`,
  `advisory_signal`, `changelog_signal`) with a required `signal_disclosure`.
- **Lifecycle state** (`lifecycle`: `installed` / `available` / `outdated` /
  `imported` / `policy_pinned` / `remove_blocked`) — kept visually distinct and
  copy/export safe.
- **Rollback posture** (reused `M5PackageComponentRollbackPosture`).

## Derived honesty (the delta this lane enforces)

Action truth is *derived*, never asserted directly, by
`resolve_package_explorer_row_action(lifecycle, relation)`:

- Read-only lifecycle states (`imported`, `policy_pinned`, `remove_blocked`)
  dominate, and a purely transitive relation is never directly actionable even
  when installed, available, or outdated — it resolves to
  `transitive_read_only` and must name its parent.
- Only a direct (or direct-and-transitive) installed/available/outdated package
  resolves to a directly-actionable class (`manage_installed`,
  `install_available`, `update_available`).

The row's `offers_direct_action` must match the derived
`is_directly_actionable`, so a transitive or blocked package can never present a
plain install/update/remove button as though it were a direct, mutable
dependency (`action_truth_misrepresented`). A directly-actionable row must carry
an `action_provenance_note` (`action_provenance_missing` — no generic action
without provenance), a read-only/blocked row must carry a `blocked_reason`
(`blocked_reason_missing`), an install/update row must name a
`candidate_version` (`candidate_version_missing`), and the rollback posture must
be consistent with the action truth (`rollback_posture_inconsistent`).

## Coverage and boundary

The packet requires the `installed`, `available`, and `outdated` lifecycle
states to be present and distinct (`lifecycle_coverage_missing`) plus at least
one non-actionable read-only/blocked row
(`non_actionable_state_coverage_missing`). Raw manifest bodies, raw lockfile
bodies, registry credentials, private registry URLs, and live registry
responses never cross this boundary; the export is scanned for forbidden
material (`raw_boundary_material_in_export`).

## Regenerating artifacts

```
GEN_PACKAGE_EXPLORER_ROW_ARTIFACTS=1 cargo test -p aureline-deps --lib \
  implement_package_explorer_rows_with_scope_relation_registry_and_signal_truth::tests::generate_artifacts
```
