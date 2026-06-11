# Fixtures: Package-set inventory and scope truth

This directory contains fixture metadata for the
`package_set_inventory_and_scope_truth` packet.

The canonical full corpus is checked in at:

`artifacts/deps/m5/package-set-inventory-and-scope-truth.json`

## Coverage

- Cargo and pnpm are the only stable ecosystems claimed.
- Whole-workspace, selected-manifest, and workset/slice scopes are represented
  as distinct scope views and never collapse into one generic inventory.
- Each scope reports honest loaded, matching, and total counts, and none widen
  scope server-side (`server_side_widening` is always `false`).
- Package identity is stable and reused across scope views, dependency edges,
  and the export projection.
- Convergence covers `unique`, `converged`, `diverged`, and `conflicted`.
- Per-manifest requested ranges or sources stay separate from resolved exact
  versions or sources, with owner/runtime context.
- Dependency relation covers `direct`, `transitive`, `workspace_local`, `path`,
  and `vcs`.
- Freshness covers `live`, `mirror_stale`, `offline_snapshot_only`, and
  `unknown_or_stale`.
- Dependency-tree edges preserve the owning manifest and disclose
  `duplicate_versions`, `version_conflict`, and `feature_unification` instead of
  hiding them in resolver text.
- Every package offers export-safe `open_raw` and `open_manifest` escapes.
