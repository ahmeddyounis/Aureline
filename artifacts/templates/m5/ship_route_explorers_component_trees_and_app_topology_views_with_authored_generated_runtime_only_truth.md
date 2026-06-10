# Route Explorers, Component Trees, and App-Topology Views with Authored/Generated/Runtime-Only Truth

- Packet: `app-topology:stable:0001`
- Label: `Route Explorers, Component Trees, and App-Topology Views with Authored/Generated/Runtime-Only Truth`
- Rows: 6 (4 admitted for display)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-08T00:00:00Z)

## Rows

- **Dashboard route** `/dashboard` (route_explorer): authored / exactly_modeled
  - Origin: Hand-authored route outside any managed zone; the route explorer marks it authored, the freshness chip reads fresh against the last scan, and static analysis derives it exactly so it is shown with no downgrade banner
  - Freshness chip: scanned · fresh (fresh)
  - Derivation: static_analysis (banner: no_banner)
  - Generator version: not_generated
  - Displayed: true
- **Users API route** `/api/users` (route_explorer): generated / exactly_modeled
  - Origin: Generated into a managed zone by the scaffold; the route explorer marks it generated and shows the pinned generator version, the freshness chip reads fresh, and the generator manifest derives it exactly
  - Freshness chip: scanned · fresh (fresh)
  - Derivation: generator_manifest (banner: no_banner)
  - Generator version: 1.8.0
  - Displayed: true
- **UserCard component** `components/managed/UserCard` (component_tree): authored_in_generated_zone / exactly_modeled
  - Origin: User-authored edits inside a generated component zone; the component tree marks it authored-in-generated-zone with managed-zone honesty, shows the generator version that owns the zone, and static analysis derives the edited node exactly so it is shown without claiming the edits are generated
  - Freshness chip: scanned · rescan available (rescan_available)
  - Derivation: static_analysis (banner: no_banner)
  - Generator version: 1.8.0
  - Displayed: true
- **LegacyWidget component** `components/legacy/LegacyWidget` (component_tree): authored / heuristic_mapping
  - Origin: Component whose place in the tree is inferred from naming and layout conventions rather than modeled exactly; the component tree marks it heuristic, the support-class banner discloses the heuristic mapping and its known issue, and the node is held from being presented as exact authored or generated truth
  - Freshness chip: scanned · aging (aging)
  - Derivation: heuristic_inference (banner: support_class_banner)
  - Generator version: not_generated
  - Displayed: false
- **Webhook handler (runtime)** `runtime:/webhooks/:provider` (app_topology): runtime_only / runtime_observed
  - Origin: Route registered dynamically at runtime and absent from source; the app-topology view marks it runtime-only and observes it from a runtime trace, so it is shown but never presented as authored or generated source truth
  - Freshness chip: traced · fresh (fresh)
  - Derivation: runtime_trace (banner: no_banner)
  - Generator version: runtime_registered
  - Displayed: true
- **Orphan module (unresolved)** `module:unresolved.orphan` (app_topology): origin_unknown / support_unknown
  - Origin: App-topology node whose origin could not be resolved to authored, generated, or runtime-only; the view marks it origin-unknown, the freshness chip reads unverified, the derivation reads unknown, and the origin-unknown banner blocks confident display rather than hiding the node
  - Freshness chip: scan · unverified (freshness_unknown)
  - Derivation: derivation_unknown (banner: origin_unknown_banner)
  - Generator version: unknown
  - Displayed: false
