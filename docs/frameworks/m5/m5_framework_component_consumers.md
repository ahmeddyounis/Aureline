# M5 framework component consumers

Status: Stable · Schema `schemas/ui/m5-framework-component-consumer.schema.json` · Record kind `add_shared_preview_runtime_docs_browser_onboarding_template_registry_workflow_bundle_visual_designer_and_support_consumers_so_framework_aware_components_keep_pack_version_evidence_and_boundary_language_aligned_across_claimed_m5_profiles`

This is the **adoption lane** over the frozen M5 framework-component matrix
(`docs/frameworks/m5/m5_framework_component_matrix.md`). The matrix freezes seven governed component
families and four sibling implement lanes narrow them into working primitives:

| Component family | Narrowed primitive | Canonical schema |
| --- | --- | --- |
| `framework_pack_header` | framework pack header / status strip | `schemas/ui/m5-framework-pack-header-status-strip-controls.schema.json` |
| `route_endpoint_row` | route / endpoint row / component / service tree node | `schemas/ui/m5-route-endpoint-component-service-tree-controls.schema.json` |
| `component_service_tree_node` | route / endpoint row / component / service tree node | `schemas/ui/m5-route-endpoint-component-service-tree-controls.schema.json` |
| `convention_diagnostic_row` | convention-diagnostic row / derived-relationship banner | `schemas/ui/m5-convention-diagnostic-derived-relationship-controls.schema.json` |
| `derived_relationship_banner` | convention-diagnostic row / derived-relationship banner | `schemas/ui/m5-convention-diagnostic-derived-relationship-controls.schema.json` |
| `generator_preview_sheet` | generator preview sheet / run-config scaffold card | `schemas/ui/m5-generator-preview-run-config-controls.schema.json` |
| `run_config_scaffold_card` | generator preview sheet / run-config scaffold card | `schemas/ui/m5-generator-preview-run-config-controls.schema.json` |

This lane proves those seven families are **reusable components** — not one framework-pack page plus a
few isolated topology objects — by binding every claimed M5 framework consumer to the same canonical
component schemas and the same descriptor vocabulary.

## Consumers

| Consumer | Token | Role |
| --- | --- | --- |
| Preview Runtime | `preview_runtime` | shows where code runs and which pack / toolchain is active before dispatch |
| Docs / Browser | `docs_browser` | explores routes and topology with exact-versus-heuristic evidence truth |
| Onboarding | `onboarding` | introduces packs and generator writes with impact and write-effect truth |
| Template Registry | `template_registry` | lists framework packs with pack-version and convention-diagnostic truth |
| Workflow Bundle | `workflow_bundle` | reads generator recovery and run-config execution-boundary truth |
| Visual Designer | `visual_designer` | reads component trees and derived relationships with proving-source truth |
| Safe Support / Export Packet | `support_export` | the authoritative rendering; references the canonical schemas so its prose can never drift |

Every family is adopted by **at least two** distinct consumers, and the safe support / export packet
references the canonical schema for every family it adopts.

## Shared descriptor vocabulary

The acceptance criterion is one truth for **pack identity / version and support class, evidence
certainty and proving source, local-versus-remote execution boundary and impact, and the rollback /
regenerate recovery boundary** across every framework surface. Those four descriptors
(`pack_identity_and_support`, `evidence_certainty_and_proving_source`, `execution_boundary_and_impact`,
`recovery_and_rollback_boundary`) are required on every binding, so framework-aware language no longer
drifts between framework-pack cards, route / topology explorers, generator-review sheets, or support
artifacts.

## Auto-narrowing

A consumer that cannot preserve full parity **auto-narrows** its claim language and always discloses a
self-contained banner naming the exact reason and the recovery action — never a generic "degraded"
note:

| Parity-health mode | Narrowing reason | Recovery action | Export caveat |
| --- | --- | --- | --- |
| `pack_or_support_unverified_narrowed` | `pack_or_support_unverified` | `inspect_pack_version_and_support` | `pack_or_support_unverified` |
| `heuristic_evidence_narrowed` | `heuristic_evidence_not_exact` | `open_proving_source_before_trusting` | `evidence_heuristic_not_exact` |
| `execution_boundary_or_write_effect_narrowed` | `execution_boundary_or_write_effect_pending` | `review_execution_boundary_and_impact_before_dispatch` | `execution_boundary_or_write_effect_disclosed_not_silent` |
| `recovery_required_narrowed` | `recovery_required_after_generator_write` | `rollback_or_regenerate_generated_output` | `generated_output_recoverable_not_final` |

### A generator never implies a no-op write and never hides the execution boundary

`execution_boundary_or_write_effect_pending` reflects a generator write or run-config dispatch that
carries a file / dependency / config write effect or a non-local (container / SSH / managed) execution
boundary. The resolver marks such a binding `reflects_write_or_boundary_risk = true`, always narrows it,
and always resolves `presents_safe_action_without_caveat = false`. Only a full-parity binding may
present a safe apply / run action without a caveat. This is the acceptance criterion that a framework
generator never implies a safe or no-op write when it changes files, dependencies, or config, and that
framework convenience never hides the local / container / SSH / managed boundary. Likewise, a heuristic
route, component, or relationship never masquerades as exact.

## Resolver

`resolve_framework_component_binding` takes one consumer's adoption of one component family, the
descriptor set it surfaces, the parity-health mode, and any export caveats, and produces one
`M5FrameworkComponentResolvedBinding`. It rejects an empty or incomplete descriptor set and any
forbidden binding material, keeps the descriptor vocabulary aligned at full parity, auto-narrows under
any weakened mode, and — when narrowed — emits a self-contained banner.

## Governance & proof

The checked support export, matrix CSV, and Markdown report live under
`artifacts/release/m5-framework-component-consumer-proof/`, and the two narrowed fixtures
(preview runtime → Beta, onboarding → Preview) live under
`fixtures/ui/m5-framework-component-consumers/`. All are minted only by the
`dump_framework_component_consumers` example emitter so the in-code matrix, the artifact, the worked
bindings, and the fixtures never drift. Raw file bodies, raw diffs, raw local paths, repository URLs,
credentials, and secrets never cross this boundary.
