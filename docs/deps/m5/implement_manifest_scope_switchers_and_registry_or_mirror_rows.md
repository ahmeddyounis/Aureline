# Implement manifest-scope switchers and registry/mirror rows

Status: Implemented (M05-974, batch B115)

This contract narrows two components frozen in
[`m5-package-management-component-matrix`](freeze_the_m5_package_management_component_matrix.md)
(M05-972) — the `manifest_scope_switcher` and the `registry_or_mirror_row` —
into one implemented, export-safe packet with two co-equal control vectors.
Together they make the **target manifest** and the **source registry** explicit
*before* any dependency state changes, so monorepo and multi-root flows never
guess which manifest they are about to touch and never hide inherited registry
state.

- Boundary schema: [`schemas/ui/m5-manifest-scope-registry-controls.schema.json`](../../../schemas/ui/m5-manifest-scope-registry-controls.schema.json)
- Producer: `aureline_deps::current_manifest_scope_registry_export`
- Release proof: [`artifacts/release/m5-manifest-scope-registry-proof/`](../../../artifacts/release/m5-manifest-scope-registry-proof/)
- Protected fixtures: [`fixtures/ui/m5-manifest-scope-registry-controls/`](../../../fixtures/ui/m5-manifest-scope-registry-controls/)

## Manifest-scope switchers

Every `ManifestScopeSwitcher` reuses the frozen `M5PackageComponent` tag (gated
to `manifest_scope_switcher`) and answers, from the control alone:

- **Active manifest** (`active_manifest_label`, always required and non-empty —
  no generic "package.json"; the exact target manifest is named).
- **Target scope** (`target_scope`: `root_manifest` / `member_package` /
  `module_manifest` / `tool_manifest`) with a required `scope_disclosure` and,
  when the scope sits below the root, a required `member_selection_note` naming
  which member/module manifest is targeted.
- **Lockfile coupling** (`lockfile_coupling`: `shared_root_lockfile` /
  `member_scoped_lockfile` / `no_lockfile_coupling`) with a required
  `lockfile_coupling_note` whenever a lockfile couples the change.
- **Change-scope review action** (`change_scope_action_label`,
  `change_scope_review_note`) — a review action, never a one-click write.
- **Rollback posture** (reused `M5PackageComponentRollbackPosture`, constrained
  to read-only or staged-review — the switcher selects a scope, it never writes).

The change-scope truth is *derived*, never asserted, by
`resolve_manifest_change_scope(target_scope, lockfile_coupling)`: a root manifest
is a `root_wide_change`, a tool manifest a `tool_manifest_change`, and a member or
module manifest is a `member_scoped_change` unless a shared root lockfile couples
it, in which case it becomes a `member_change_shared_lock`. The switcher's
`affects_root_lockfile` must match the derived `affects_shared_root_lockfile`, so
a member change against a shared root lockfile can never read as an isolated
member change (`change_scope_misrepresented`).

## Registry / mirror rows

Every `RegistryOrMirrorRow` reuses the frozen `M5PackageComponent` tag (gated to
`registry_or_mirror_row`) and answers, from the row alone:

- **Source class** (`source_class`: `public_default` / `enterprise_mirror` /
  `self_hosted` / `offline_cache` / `policy_pinned_source`) with a required
  `source_disclosure` — a user can always tell where metadata and artifacts come
  from. The packet requires all five source classes to be represented
  (`source_coverage_missing`).
- **Auth mode** (`auth_mode`: `anonymous_public` / `token_authenticated` /
  `sso_session` / `client_certificate`) with a required `auth_disclosure`
  whenever access is not anonymous public. Only the mode is recorded; no
  credential material crosses the boundary.
- **Freshness / reachability** (`reachability`: `fresh_reachable` /
  `stale_cached` / `offline_cache_only` / `unreachable`) with a required
  `reachability_note` whenever the answer is not fresh.
- **Policy pinning** (`is_policy_pinned`, `policy_pin_note`) and
  **offline/cache-only continuity** (`offline_cache_only`,
  `offline_continuity_note`).
- **Degradation** (reused `M5PackageComponentDegradationState` plus a
  `degradation_note` whenever resolution is not exact) and **rollback posture**
  (constrained to read-only — a source descriptor never mutates state).

Continuity and pinning are *derived* by
`resolve_registry_or_mirror_disclosure(source_class, reachability,
is_policy_pinned)`: a row is offline/cache-only when its source is an offline
cache or its source is unreachable/cache-only, and a `policy_pinned_source`
implies the row is pinned. The row's `offline_cache_only` must match the derived
value (`offline_continuity_misrepresented`), and a policy-pinned source that is
not marked pinned fails (`policy_pinning_misrepresented`), so an offline or
pinned answer never presents as a clean live upstream read.

## Coverage and boundary

The packet requires all four manifest scopes (`scope_coverage_missing`) and all
five registry source classes (`source_coverage_missing`) to be represented, plus
the trust-review invariant `no_generic_manage_package_language` (the batch
guardrail). Raw manifest bodies, raw lockfile bodies, registry credentials,
private registry URLs, and live registry responses never cross this boundary;
the export is scanned for forbidden material (`raw_boundary_material_in_export`).

## Regenerating artifacts

```
GEN_MANIFEST_SCOPE_REGISTRY_ARTIFACTS=1 cargo test -p aureline-deps --lib \
  implement_manifest_scope_switchers_and_registry_or_mirror_rows::tests::generate_artifacts
```
