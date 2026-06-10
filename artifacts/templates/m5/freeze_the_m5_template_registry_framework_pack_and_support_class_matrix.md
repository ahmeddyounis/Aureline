# Template Registry, Framework-Pack, and Support-Class Matrix

- Packet: `template-framework-matrix:stable:0001`
- Label: `Template Registry, Framework-Pack, and Support-Class Matrix`
- Lanes: 4 (3 stable)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-07T00:00:00Z)

## Lanes

- **signed_template_registry**: `stable` (officially_supported)
  - Scope: Signed template registry whose publisher provenance and signatures are verified before a template is offered; template source class and generator version stay inspectable from gallery through recovery
  - Generation truth: authored
  - Evidence: required (2 refs)
  - Rollback: read_only_no_mutation
- **scaffold_planner**: `stable` (officially_supported)
  - Scope: Scaffold planner whose file and directory impact is reviewable or exportable before any write, with a visible rollback boundary and a create-empty alternative offered at equal weight
  - Generation truth: mixed_authored_generated
  - Evidence: required (2 refs)
  - Rollback: previewed_write_with_checkpoint
- **framework_pack**: `beta` (community_supported)
  - Scope: Framework packs that keep authored, generated, and runtime-only behavior explicitly separated and never present heuristic or bridge behavior as exact first-party truth without current support-class and downgrade cues
  - Generation truth: runtime_only
  - Evidence: required (2 refs)
  - Rollback: read_only_no_mutation
- **archetype_health_bundle**: `stable` (officially_supported)
  - Scope: Archetype health bundles that partition blockers, warnings, and optimizations and preserve live, cached, policy-evaluated, and unchecked freshness state instead of collapsing to one pass/fail bit
  - Generation truth: generated
  - Evidence: required (2 refs)
  - Rollback: three_way_lineage_update
