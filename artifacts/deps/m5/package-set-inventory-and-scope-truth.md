# Proof packet: Package-set inventory and scope truth

Artifact: `artifacts/deps/m5/package-set-inventory-and-scope-truth.json`

## Purpose

This packet is the canonical monorepo package-set inventory, dependency-tree,
and manifest/workset scope truth for the dependency-intelligence lane. It keeps
the package-set inventory, dependency tree, scope selector, package detail,
CLI/headless inspect, Help, and support-export surfaces on one vocabulary.

The packet currently claims stable rows for `cargo` and `node_pnpm` only. Other
ecosystems should stay inspect-only or handoff-only until resolver, workspace,
and freshness semantics are proven.

## The three scopes never collapse

Whole-workspace truth, selected-manifest truth, and workset/slice truth are
separate [`scope_view`] rows. Each carries its own honest loaded/matching/total
counts and a `server_side_widening` flag that must stay `false`: there is no
hidden server-side widening.

| Scope kind | Meaning |
|---|---|
| `full_workspace` | The full resolved workspace package set across every manifest. |
| `selected_manifests` | A user-selected subset of manifests. |
| `workset_slice` | A workset or slice limited to an active working set. |

## Vocabulary

| Domain | Tokens |
|---|---|
| Ecosystem | `cargo`, `node_pnpm` |
| Scope kind | `full_workspace`, `selected_manifests`, `workset_slice` |
| Convergence | `unique`, `converged`, `diverged`, `conflicted` |
| Runtime | `runtime`, `development`, `build`, `optional` |
| Dependency relation | `direct`, `transitive`, `workspace_local`, `path`, `vcs` |
| Freshness | `live`, `mirror_stale`, `offline_snapshot_only`, `unknown_or_stale` |
| Duplicate/conflict | `none_known`, `duplicate_versions`, `version_conflict`, `feature_unification` |
| Open escape | `open_raw`, `open_manifest` |

## Surface contract

| Surface | Contract |
|---|---|
| Package-set inventory | Shows stable package identity, owning manifests, convergence state, resolved identity, and mirror/offline freshness, scoped to one of the three distinct scope kinds. |
| Dependency tree / list | Preserves the owning manifest for every edge and discloses duplicate or conflicting versions instead of hiding them in resolver text. |
| Scope selector | Switches between full-workspace, selected-manifest, and workset/slice scope without collapsing them, and reports loaded versus matching versus total counts. |
| Package detail sheet | Separates requested ranges from resolved exact identity per owning manifest, with owner/runtime context. |
| CLI/headless inspect | Uses the same packet vocabulary as the UI and support export. |
| Support export | Projects the same scope and package identity vocabulary, redaction-safe. |

## Honest virtualization

Large package sets remain virtualized and honest about loaded versus matching
versus total scope. The full-workspace view loads an 8-row window over an
842-package total; the selected-manifest view loads 6 of 120; the workset slice
loads 1 of 2.

## Fixture coverage in the canonical packet

| Scenario | Id | Proof |
|---|---|---|
| Converged package | `pkg:cargo:serde` | Two manifests resolve to `1.0.203`; `converged`. |
| Diverged package | `pkg:cargo:tokio` | `1.38.0` and `1.37.0` coexist on a stale mirror; `diverged`, `mirror_stale`. |
| Conflicted package | `pkg:cargo:openssl-sys` | `^0.9` versus `=0.9.98` cannot unify; `conflicted`, `offline_snapshot_only`. |
| Unknown-freshness package | `pkg:node:react` | `18.2.0` and `18.3.1` diverge with `unknown_or_stale` registry freshness. |
| Workspace / path / vcs identity | `pkg:cargo:aureline-text`, `pkg:cargo:experimental-fork` | Workspace-local, path, and git source identities stay distinct. |
| Duplicate-versions edge | `edge:lock:tokio` | Lockfile discloses both resolved versions. |
| Version-conflict edge | `edge:shell:openssl-sys` | Conflict is disclosed, not hidden. |
| Feature-unification edge | `edge:web:react` | pnpm hoist to a single instance is disclosed. |

## Summary

- 3 scope views: 1 full-workspace, 1 selected-manifest, 1 workset/slice.
- 8 package rows: 4 unique, 1 converged, 2 diverged, 1 conflicted.
- 8 dependency edges, 3 of which disclose a duplicate or conflict.
- 3 package rows on stale-mirror or offline data.

## Owner sign-off

- `team:dependency_tools` — signed off 2026-06-11 for package-set inventory and scope truth.
- `team:design` — signed off 2026-06-11 for scope-distinction and virtualization honesty.
- `team:release_engineering` — signed off 2026-06-11 for CLI/headless and support-export parity.
