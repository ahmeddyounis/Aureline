# Package-set inventory and scope

This page describes the canonical package-set inventory and scope packet used by
Aureline's package-set inventory, dependency tree, scope selector, package
detail sheet, CLI/headless inspect, Help, support exports, and release evidence.
The source artifact is
`artifacts/deps/m5/package-set-inventory-and-scope-truth.json`.

## Scope is always explicit

The inventory and dependency tree are always rendered for one of three distinct
scopes. They are never collapsed into one generic inventory, and the scope you
are looking at is always shown.

| Scope | Meaning |
|---|---|
| Full workspace | The full resolved workspace package set across every manifest. |
| Selected manifests | A subset of manifests you selected. |
| Workset slice | A workset or slice limited to your active working set of files. |

Switching scope never widens it server-side without your intent: the packet
records `server_side_widening` and it must stay `false`.

## Loaded, matching, and total

Large monorepo package sets stay virtualized. Each scope reports three counts so
you never mistake the loaded window for the whole set:

- **loaded** — rows currently materialized in the view;
- **matching** — rows matching the active filter within scope;
- **total** — every row in scope regardless of filter.

## Package identity and convergence

Every package carries a stable identity that is reused across the UI,
CLI/headless output, and support exports. Each package shows the manifests that
own it, the requested range and resolved exact version per manifest, and a
convergence state:

| State | Meaning |
|---|---|
| `unique` | Exactly one manifest owns the package. |
| `converged` | Multiple manifests own it and all resolve to one version. |
| `diverged` | Multiple resolved versions coexist without a hard conflict. |
| `conflicted` | Multiple resolved versions coexist and at least one conflicts. |

## Dependency trees disclose duplicates and conflicts

Dependency-tree and list views preserve the owning manifest for every edge and
disclose duplicate or conflicting versions rather than hiding them in resolver
text:

| State | Meaning |
|---|---|
| `none_known` | No duplicate or conflict is known. |
| `duplicate_versions` | Multiple resolved versions coexist for the same package. |
| `version_conflict` | Constraints conflict and cannot be unified. |
| `feature_unification` | Multiple requests were unified onto a single resolved version. |

## Freshness is never hidden

Each package row shows the freshness of the registry or mirror data behind it:

| State | Meaning |
|---|---|
| `live` | Live registry or local data current enough to trust. |
| `mirror_stale` | Mirror data exists but is stale. |
| `offline_snapshot_only` | Only offline snapshot metadata is available. |
| `unknown_or_stale` | Freshness cannot support a stable claim. |

## Export-safe escapes

Every package offers export-safe escapes — open the raw resolver output
(`open_raw`) or open the owning manifest (`open_manifest`) — without exporting
credentials or raw provider payloads.
