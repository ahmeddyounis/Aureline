# Manifest-scope switchers and registry/mirror rows

- Packet: `manifest-scope-registry:stable:0001`
- Surface: `Manifest-scope switchers and registry/mirror rows`
- Switchers: 4 (2 regenerate the shared root lockfile)
- Registry/mirror rows: 5 (1 offline/cache-only)
- Proof freshness SLO: 168 hours (last refresh: 2026-07-08T00:00:00Z)

## Manifest-scope switchers

- **Cargo.toml (workspace root)** (root_manifest) — change `root_wide_change` [shared_root_lockfile]
- **packages/web/package.json** (member_package) — change `member_change_shared_lock` [shared_root_lockfile]
- **packages/api/auth/module.json** (module_manifest) — change `member_scoped_change` [member_scoped_lockfile]
- **rust-toolchain.toml (toolchain)** (tool_manifest) — change `tool_manifest_change` [no_lockfile_coupling]

## Registry / mirror rows

- **registry.npmjs.org (public default)** [public_default] auth `anonymous_public`, fresh_reachable 
- **nexus.corp.example (enterprise mirror)** [enterprise_mirror] auth `token_authenticated`, fresh_reachable 
- **packages.internal (self-hosted)** [self_hosted] auth `sso_session`, stale_cached 
- **local offline cache** [offline_cache] auth `anonymous_public`, offline_cache_only (offline/cache-only)
- **policy-pinned source** [policy_pinned_source] auth `anonymous_public`, fresh_reachable 
