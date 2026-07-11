# M5 Marketplace-Result-Row and Marketplace-Detail-Fact-Grid Controls

- Packet: `m5-marketplace-result-row-detail-fact-grid-controls:stable:0001`
- Label: `M5 marketplace-result-row and marketplace-detail-fact-grid controls with compatibility, runtime model, permission posture, support class, performance evidence, publisher continuity, and registry source truth aligned across list and detail`
- Consumer surfaces: 5
- Registry source classes: public_registry, mirrored_registry, enterprise_registry, side_loaded, verified_partner, source_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-11T00:00:00Z)

## Consumer surfaces

- **marketplace_ui**: `stable`
  - Owner: Marketplace catalog owner
  - Scope: The marketplace catalog renders one compact result row per artifact naming source class, compatibility, runtime model, permission posture, activation budget, support class, and publisher continuity, and the detail fact grid adds richer version ranges and lifecycle so a compare decision needs no disconnected page
  - Result-row examples: 2 / detail-fact-grid examples: 2
- **extensions_ui**: `stable`
  - Owner: Extensions manager owner
  - Scope: The extensions manager reuses the same fact grammar, names permission widening a widened artifact requests, and degrades honestly when permission widening is hidden behind compact chrome
  - Result-row examples: 2 / detail-fact-grid examples: 2
- **registry_ui**: `stable`
  - Owner: Registry admin owner
  - Scope: The registry admin surface degrades honestly when the registry source or lifecycle cannot be resolved or activation cost is hidden, keeping public versus mirrored versus enterprise source class explicit before mutation
  - Result-row examples: 2 / detail-fact-grid examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved row and grid truth, so a collapsed source class, an incompatible-shown-ready artifact, a hidden publisher transfer, or missing docs linkage is visible in evidence rather than hidden behind compact chrome
  - Result-row examples: 3 / detail-fact-grid examples: 4
- **product_ui**: `stable`
  - Owner: In-product marketplace owner
  - Scope: In-product surfaces reuse the same fact grammar a user sees in the marketplace catalog, always offering the command-backed detail path and degrading honestly when the artifact identity, support class, or detail path is missing
  - Result-row examples: 4 / detail-fact-grid examples: 2
