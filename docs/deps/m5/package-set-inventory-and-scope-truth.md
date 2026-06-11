# Package-Set Inventory And Scope Truth

The monorepo package-set inventory, dependency-tree, and manifest/workset scope
packet lives at `artifacts/deps/m5/package-set-inventory-and-scope-truth.json`
and is embedded in `aureline-deps`.

Stable claims are limited to `cargo` and `node_pnpm`. The packet proves that
package-set inventories and dependency trees stay honest about scope, manifest
ownership, and freshness:

- whole-workspace, selected-manifest, and workset/slice scopes are kept distinct
  and never collapse into one generic inventory;
- every package carries a stable identity, the manifests that own it,
  owner/runtime context, and a converged/diverged/conflicted state;
- per-manifest requested ranges stay separate from resolved exact identities;
- dependency-tree edges preserve the owning manifest and disclose duplicate or
  conflicting versions;
- mirror/offline freshness is visible per package;
- large package sets stay virtualized and honest about loaded versus matching
  versus total scope;
- there is no hidden server-side widening of scope.

Desktop, CLI/headless, and support/export consumers project the same scope and
package identity vocabulary via the embedded packet and its
`export_projection`, instead of reconstructing truth from resolver output text.

The user-facing Help projection is
`docs/help/deps/package-set-inventory-and-scope-truth.md`.
