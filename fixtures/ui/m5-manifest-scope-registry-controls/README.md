# M5 manifest-scope switcher and registry/mirror row fixtures

Protected fixtures for the `manifest_scope_switcher` and `registry_or_mirror_row`
components implemented in
`aureline_deps::implement_manifest_scope_switchers_and_registry_or_mirror_rows`.

Each fixture is an export-safe `ManifestScopeRegistryControlsPacket` that
validates against
[`schemas/ui/m5-manifest-scope-registry-controls.schema.json`](../../../schemas/ui/m5-manifest-scope-registry-controls.schema.json)
and passes `ManifestScopeRegistryControlsPacket::validate`.

- `member_shared_root_lockfile.json` — spotlights a member manifest coupled to a
  shared root lockfile; the switcher names the target member and discloses that
  the change regenerates the shared root lockfile.
- `offline_cache_only.json` — a public-default registry that was unreachable and
  answered from the offline cache; offline/cache-only continuity stays explicit
  and never reads as a clean live upstream read.

Regenerate with:

```
GEN_MANIFEST_SCOPE_REGISTRY_ARTIFACTS=1 cargo test -p aureline-deps --lib \
  implement_manifest_scope_switchers_and_registry_or_mirror_rows::tests::generate_artifacts
```
