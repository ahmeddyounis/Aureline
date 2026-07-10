# M5 Framework Component Surface Certification (M05-1043)

This is the **closing surface-certification capstone** for the B123 framework-component lane. Where
the frozen matrix (`schemas/ui/m5-framework-component-matrix.schema.json`) defines the seven reusable
**framework-pack-header**, **route-endpoint-row**, **component-service-tree-node**,
**convention-diagnostic-row**, **generator-preview-sheet**, **run-config-scaffold-card**, and
**derived-relationship-banner** components, the M05-1037..1040 primitive lanes narrow each one, the
M05-1041 consumer lane proves they are reusable across the claimed preview-runtime / docs-browser /
onboarding / template-registry / workflow-bundle / visual-designer / support-export consumers, and
the M05-1042 accessibility / auto-narrowing capstone certifies keyboard / screen-reader / CLI /
export parity per family, this capstone **certifies that the shared framework-component truth holds
on every claimed M5 framework-aware surface** — and auto-narrows any surface that cannot sustain it.

- **Module:**
  `crates/aureline-templates/src/certify_framework_pack_header_route_endpoint_row_component_service_tree_node_convention_diagnostic_row_generator_preview_sheet_run_config_scaffold_card_and_derived_relationship_banner_truth_on_every_claimed_m5_framework_aware_surface/`
- **Boundary schema:** `schemas/ui/m5-framework-component-certification.schema.json`
- **Release proof:** `artifacts/release/m5-framework-component-certification/`
  (`support_export.json`, `matrix.csv`, `report.md`)
- **Fixtures:** `fixtures/ui/m5-framework-component-certification/`
- **Canonical bundle every row cites:** `artifacts/release/m5-framework-component-proof/support_export.json`
  (the frozen framework-component matrix release proof — the canonical M5 evidence index entry for
  this lane)

## What is certified

The packet is keyed on the **surface** a user reads a pack header, a route graph, a component tree, a
convention warning, a generator preview, or a run-config card on — not on component family or
primitive lane. Eight claimed surfaces are certified exactly once:

| Surface | Meaning |
| --- | --- |
| `framework_pack_center` | The framework-pack center (pack header / status strip surface). |
| `route_explorer` | The route / endpoint explorer surface. |
| `topology_view` | The component / service topology view. |
| `convention_diagnostics` | The convention-diagnostics center. |
| `generator_review` | The generator-review sheet surface. |
| `run_config_center` | The run-config center. |
| `support_export` | The support / export bundle. |
| `cli_headless` | The CLI / headless surface. |

Each surface is scored on **six truth axes**: `visual`, `keyboard`, `screen_reader`, `export`
(always-on), `degraded_state`, and `source_linkage_and_execution_boundary`. Every one of the seven
frozen component families is certified on at least one surface.

## The invariant

**A degraded axis must produce a visible claim narrowing.** A surface that keeps an
`exact_framework_truth` claim while one of its truth axes is not current — the framework pack's health
cannot be proven, the supported version range cannot be proven for the active project, a
proving-source linkage is missing, a relationship is only heuristically inferred, or a
generator-effect truth is only partial — is over-claiming and **blocks (red)**. A surface that
discloses the reduction by narrowing its exactness claim (with a bound reason and a frozen downgrade
trigger) is honestly **yellow**. A surface with full parity delivers its claim (**green**).

The exactness-claim ladder, strongest first: `exact_framework_truth` (5) >
`unproven_version_range_projection` (4) > `unverified_pack_projection` (3) >
`unlinked_source_projection` (2) > `heuristic_inference_projection` (1) >
`partial_generator_effect_projection` (0). Certification may only **narrow** a claim, never strengthen
it.

### Proving-source and recovery preservation

Framework truth never loses its proving source or recovery boundary: a narrowed surface always
preserves its **pack-identity / support / evidence-certainty / proving-source / execution-boundary /
rollback-or-regenerate recovery** continuity rather than dropping it between a pack header, a route
row, a component-tree node, a convention row, a generator preview, and a derived-relationship banner.
Dropping it blocks the surface (`SourceOrRecoveryDropped`).

### No heuristic-as-exact, no hidden write or execution boundary

No certified surface may let a **heuristic route or component tree masquerade as exact** (an
exact-from-source or runtime-confirmed reading it did not earn) (`HeuristicMasqueradesAsExact`).
No certified surface may **imply a safe or no-op write when a generator changes files / dependencies /
config, or hide the local / container / SSH / managed execution boundary** behind framework
convenience language (`NoOpWriteOrHiddenExecutionBoundary`).

### Always-on export parity

The `export` axis must always stay certified, so support and automation can reconstruct the same
pack / support / certainty / source-linkage / execution-boundary / recovery truth from the same
component identity the user saw. Export must offer text / JSON / Markdown reconstruction and prohibit
a raw-value-only export.

## The five auto-narrow conditions

The seed packet certifies three green surfaces (full parity, claim delivered) and five yellow
surfaces — one for each spec auto-narrow condition (an unverified pack health, an unprovable supported
version range, a missing proving-source linkage, a heuristically-inferred relationship, or a partial
generator-effect truth):

| Surface | Claimed → Certified | Binding axis | Trigger |
| --- | --- | --- | --- |
| `framework_pack_center` | `exact_framework_truth` → `unverified_pack_projection` | `degraded_state` | `support_class_unstated` |
| `route_explorer` | `exact_framework_truth` → `heuristic_inference_projection` | `source_linkage_and_execution_boundary` | `exact_versus_heuristic_unstated` |
| `topology_view` | `exact_framework_truth` → `unlinked_source_projection` | `source_linkage_and_execution_boundary` | `proving_source_omitted` |
| `convention_diagnostics` | `exact_framework_truth` → `unproven_version_range_projection` | `degraded_state` | `pack_identity_unstated` |
| `generator_review` | `exact_framework_truth` → `partial_generator_effect_projection` | `source_linkage_and_execution_boundary` | `impact_undisclosed` |

No surface hides drift (red), no surface lets a heuristic route masquerade as exact, no surface
implies a no-op write or hides its execution boundary, and no surface drops its proving-source /
recovery continuity.

## Metadata-only boundary

The packet is metadata-only: typed class tokens, opaque refs, booleans, and redacted labels. Raw
generated file bytes, secret material, and credential-bearing material never cross this boundary
(`RawFrameworkMaterialInExport`).

## Regenerating the artifacts

The checked-in export is byte-aligned with the in-code seed builder
(`seeded_m5_framework_component_certification_packet`). A drift test fails if they diverge. To
regenerate after an intentional change:

```
GEN_FRAMEWORK_CERT_ARTIFACTS=1 cargo test -p aureline-templates --lib \
  -- certify_framework generate_artifacts
```

Then re-run the suite:

```
cargo test -p aureline-templates --lib -- certify_framework
```
