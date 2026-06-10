# Route explorers, component trees, and app-topology views with authored/generated/runtime-only truth

This contract describes the export-safe packet that carries the **app-structure
presentation** truth for the route explorer, component tree, app-topology view,
diff-review, run, diagnostics, and support surfaces: the authored/generated/
runtime-only origin of each node, the generator version that produced any
generated node, the view-scan freshness chip, the derivation class, the support
class, and the downgrade banner. The packet is the canonical truth those
surfaces ingest instead of re-describing structure by hand or presenting
heuristic, bridged, or runtime-observed nodes as exact authored or generated
source truth.

- Boundary schema:
  `schemas/templates/ship-route-explorers-component-trees-and-app-topology-views-with-authored-generated-runtime-only-truth.schema.json`
- Implementation:
  `crates/aureline-templates/src/ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth/`
- Checked support export:
  `artifacts/templates/m5/ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth/support_export.json`
- Fixtures:
  `fixtures/templates/m5/ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth/`

This packet **references** the upstream template-manifest, framework-pack, and
generated-project-lineage records — the `template_manifest_alpha`, the
framework-pack header packet, and the `generated_project_lineage_alpha` contracts
frozen in `docs/templates/template_registry_and_scaffold_contract.md` and the
framework-pack lane — by opaque ref (`app_id`, `node_id`, `generator_version`)
rather than embedding them, and reuses the prior managed-zone and support-class
vocabulary instead of inventing parallel terms.

## Boundary discipline

The packet is metadata only. Raw source bodies, raw manifests, runtime payloads,
repository URLs, hostnames, secrets, and user-authored content never cross this
boundary. Rows carry opaque refs, closed-vocabulary class tokens, short
reviewable summaries, structural locators (`node_path`), and export-safe chip
labels. `validate` rejects any export that leaks obviously forbidden material.

## Row truth

Each `topology_row` binds one node of a route explorer, component tree, or
app-topology view to:

- **View and origin** — `view_kind` (`route_explorer`, `component_tree`, or
  `app_topology`), `origin_class` (`authored`, `generated`,
  `authored_in_generated_zone`, `runtime_only`, or `origin_unknown`),
  `origin_summary`, and `node_path`. Every view shows the node's
  authored/generated/runtime-only origin.
- **Generator version** — `generator_version`. A `generated` or
  `authored_in_generated_zone` node always shows the generator version that owns
  its managed zone, so generated truth stays inspectable; authored and
  runtime-only nodes carry a sentinel (`not_generated`, `runtime_registered`).
- **Freshness chip** — `freshness_class` (`fresh`, `rescan_available`, `aging`,
  `stale`, or `freshness_unknown`), `freshness_chip_label`, and `last_scanned`.
  A `stale` or `freshness_unknown` chip must show a downgrade banner.
- **Derivation** — `derivation_class` (`generator_manifest`, `static_analysis`,
  `runtime_trace`, `heuristic_inference`, `bridged_from_external_tool`,
  `derivation_degraded`, or `derivation_unknown`) and `derivation_summary`. Any
  non-exact derivation (anything but manifest, static analysis, or runtime trace)
  must show a banner.
- **Support honesty** — `support_class` keeps bridge/heuristic behavior labeled.
  A `bridge_behavior` or `heuristic_mapping` node must disclose a known issue,
  carry the matching `bridge_behavior_disclosed` / `heuristic_mapping_disclosed`
  downgrade trigger, and show a banner, so inferred or bridged structure is never
  presented as exact authored or generated truth. A `runtime_only` node must
  carry the `runtime_only_disclosed` trigger so it is never folded into the
  authored or generated source views.
- **Downgrade banner** — `downgrade_banner_class` (`no_banner`,
  `freshness_banner`, `derivation_banner`, `support_class_banner`,
  `policy_block_banner`, or `origin_unknown_banner`) makes the narrowing cue
  explicit.
- **Downgrade and projection** — `downgrade_triggers`, `consumer_surfaces`, and
  `admitted_for_display`. A blocked node (stale or unknown freshness, degraded or
  unknown derivation, unknown origin, or a hard-block banner) can never be
  admitted for confident display.

## Downgrade automation

`apply_downgrade_automation` narrows nodes from observed runtime signals so a
stale or underqualified view narrows before it is presented, instead of being
hidden or presented as exact truth:

- **Unresolved origin** marks the origin, freshness, and derivation unknown,
  raises the origin-unknown banner, and withdraws confident display.
- **A yanked generator version** narrows freshness to `stale`, raises a freshness
  banner, and withdraws display.
- **A stale scan** narrows freshness to `stale`, raises a freshness banner, and
  withdraws display.
- **An unverified derivation** narrows the derivation to `derivation_degraded`,
  raises a derivation banner, and withdraws display.
- **Stale proof or a narrowed upstream** withdraws display.

A raised banner is never lowered to a softer cue, and a narrowed node stays a
valid, export-safe packet, so the structure views and support surfaces show a
current, labeled state rather than an optimistic placeholder.

## Consumers

`current_app_topology_export()` reads and validates the checked support export.
It is the first real consumer: a route explorer, component tree, app-topology
view, run, diagnostics, or support-export surface ingests the canonical packet
through it. The two checked fixtures (`origin_unknown_blocked.json`,
`derivation_degraded_withheld.json`) are valid, narrowed packets that exercise
the downgrade behavior the canonical export keeps green.

The artifact and fixtures are regenerated deterministically from the canonical
builder:

```text
cargo run -p aureline-templates --example dump_app_topology_views -- canonical
cargo run -p aureline-templates --example dump_app_topology_views -- markdown
cargo run -p aureline-templates --example dump_app_topology_views -- origin_unknown
cargo run -p aureline-templates --example dump_app_topology_views -- derivation_degraded
```
