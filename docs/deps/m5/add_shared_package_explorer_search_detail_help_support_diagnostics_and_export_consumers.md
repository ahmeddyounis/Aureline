# Shared package-management component consumers

Status: Implemented (M05-977, batch B115)

This contract is the closing consumer-adoption lane for the reusable M5
package-management components frozen in
[`m5-package-management-component-matrix`](freeze_the_m5_package_management_component_matrix.md)
(M05-972) and implemented by the package-explorer-row (M05-973), manifest-scope /
registry-or-mirror (M05-974), install-review / lockfile-impact (M05-975), and
script-risk / grouped-update / rollback-checkpoint (M05-976) lanes. It binds each
shared component to the consumers that render it and proves — by fixtures, not
screenshots — that the same package object keeps the same scope, provenance, auth,
and lockfile language wherever Aureline browses, searches, helps, supports,
diagnoses, or exports a package operation.

- Boundary schema: [`schemas/ui/m5-package-management-component-consumer.schema.json`](../../../schemas/ui/m5-package-management-component-consumer.schema.json)
- Producer: `aureline_deps::current_package_component_consumer_export`
- Release proof: [`artifacts/release/m5-package-management-component-consumers-proof/`](../../../artifacts/release/m5-package-management-component-consumers-proof/)
- Protected fixtures: [`fixtures/ui/m5-package-management-component-consumers/`](../../../fixtures/ui/m5-package-management-component-consumers/)

## Consumers

Six consumer surfaces reuse the shared components:

- `package_explorer` — the package browse surface.
- `dependency_search_detail` — the dependency search / detail pane.
- `help_surface` — the Help / About surface.
- `support_packet` — the exported support bundle.
- `diagnostics` — the diagnostics / doctor view.
- `exported_view` — exported package-operation evidence.

`help_surface`, `support_packet`, and `exported_view` are handoff surfaces: each of
their bindings must point at the frozen component matrix **and** the canonical
implement-lane schema for its component (`points_at_canonical_contracts`), so a
component can never be re-homed onto generic manage-package chrome that hides its
canonical source of truth.

## Parity facets

For a given `package_object_id`, every consumer surface must present identical
values for four parity facets, reused verbatim rather than reworded per surface:

- `manifest_scope_label` — target manifest and direct/transitive scope.
- `registry_source_auth_label` — registry source and auth posture.
- `risk_language` — script/native-build risk and lockfile churn.
- `recovery_language` — grouped-update reason and rollback/checkpoint identity.

Any drift between two surfaces for the same object trips
`parity_drift_across_surfaces`.

## Narrowing and disclosure

A surface may narrow *how much* it shows when the registry answer degrades, but it
may never change the parity language. The registry/resolution state is reused from
the frozen matrix (`M5PackageComponentDegradationState`) and mapped to a render
mode by `resolve_package_component_render_disclosure`:

| Registry state | Render mode | Required disclosure |
| --- | --- | --- |
| `resolved_exact` | `full_parity` | none (and no narrow banner) |
| `manifest_range_only` | `manifest_range_narrowed` | narrow banner |
| `mirror_backed` / `offline_snapshot_only` | `mirror_or_offline_narrowed` | narrow banner + continuity note |
| `auth_required_unsatisfied` | `auth_required_narrowed` | narrow banner + auth note |
| `unknown_or_stale` | `unknown_or_stale_narrowed` | narrow banner |

Every narrowed binding carries an explicit narrow banner naming the reason, the
preserved facets, and the next action. Mirror/offline continuity and registry-auth
truth stay explicit through their own notes rather than collapsing the object out
of view.

## Guardrails

Each binding pins five guardrail row-invariants to `false`; any true value is a
violation:

- `uses_generic_manage_package_language_hiding_scope`
- `uses_one_click_update_language_hiding_risk`
- `conceals_registry_auth_posture`
- `hides_broad_lockfile_regeneration`
- `drops_mirror_offline_or_rollback_truth`

## Coverage and reuse

`validate()` proves every one of the six consumers and every one of the eight
components appears among the bindings, and that each component is adopted by at
least two distinct consumers (`package_component_reuse_unproven` otherwise). Raw
manifests, raw lockfile bodies, registry credentials, private registry URLs, and
live registry responses stay outside the support boundary.

## Regenerate

```sh
GEN_PACKAGE_COMPONENT_CONSUMER_ARTIFACTS=1 cargo test -p aureline-deps --lib \
  regenerate_package_component_consumer_artifacts
```
