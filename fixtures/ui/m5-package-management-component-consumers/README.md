# M5 package-management component consumer fixtures

Protected fixtures for the closing consumer-adoption lane
`add_shared_package_explorer_search_detail_help_support_diagnostics_and_export_consumers_so_package_components_keep_scope_auth_and_lockfile_language_aligned`
in `crates/aureline-deps` (M05-977, batch B115).

Each fixture is a `PackageComponentConsumerPacket` validated against
[`schemas/ui/m5-package-management-component-consumer.schema.json`](../../../schemas/ui/m5-package-management-component-consumer.schema.json)
and by the module's `validate()` — the same eight components bound to the package
explorer, dependency search/detail pane, Help surface, support packet, diagnostics
view, and exported view, proving that the same package object presents identical
manifest-scope, registry-source/auth, script/lockfile-risk, and rollback/checkpoint
language across surfaces.

| Fixture | Scenario |
| --- | --- |
| `mirror_and_offline_narrowed.json` | Some objects narrow to `mirror_or_offline_narrowed` (mirror-backed / offline snapshot); the mirror/offline continuity note stays explicit and every parity facet is preserved. |
| `auth_required_and_stale.json` | Some objects narrow to `auth_required_narrowed` (registry auth required) or `unknown_or_stale_narrowed` (package state unknown/stale); each narrowing is disclosed through its banner and note. |

Regenerate the checked-in support export, summary, and fixtures after a contract
change:

```sh
GEN_PACKAGE_COMPONENT_CONSUMER_ARTIFACTS=1 cargo test -p aureline-deps --lib \
  regenerate_package_component_consumer_artifacts
```
