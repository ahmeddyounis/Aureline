# M5 Breakpoint / Device-Preview Row Primitive: Device Row, Runtime-Truth Cue, and Continuity Actions

This contract implements the breakpoint / device-preview row family frozen in the
M5 visual-designer component matrix (tracked under
`docs/preview/m5/m5_visual_designer_component_matrix.md`) as **one reusable
breakpoint / device-preview primitive**. Aureline resolves a single preview
target's device / viewport situation once and projects it onto a device preview
row, a live-versus-mock runtime-truth cue, and a compare / open-source continuity
block that share a single target identity, so a preview never blurs whether the
user is looking at live runtime data, mock data, or a stale / captured view, and
never loses the source anchor when the user moves across device targets, variants,
or runtime origins.

Where the [selected-node primitive](m5_visual_designer_selected_node_primitive_contract.md)
implements the canvas / tree / inspector families and the
[round-trip-honesty primitive](m5_source_round_trip_honesty_primitive_contract.md)
implements the source-sync-chip / conflict-banner / unsupported-card families, this
primitive implements the breakpoint / device-preview family that keeps viewport and
runtime truth explicit.

- **Module:** `crates/aureline-preview/src/implement_the_m5_breakpoint_and_device_preview_row_primitive`
- **Boundary schema:** [`schemas/ui/m5-breakpoint-device-preview-primitive.schema.json`](../../schemas/ui/m5-breakpoint-device-preview-primitive.schema.json)
- **Checked support export:** `artifacts/release/m5-breakpoint-device-preview-proof/support_export.json`
- **Matrix CSV:** `artifacts/release/m5-breakpoint-device-preview-proof/matrix.csv`
- **Report:** `artifacts/components/m5-breakpoint-device-preview-primitive.md`
- **Fixtures:** [`fixtures/ui/m5-breakpoint-device-preview-primitive/`](../../fixtures/ui/m5-breakpoint-device-preview-primitive/)

## The two halves

### 1. The resolver

`resolve_breakpoint_preview(&M5BreakpointPreviewInput) -> Result<M5ResolvedBreakpointPreview, M5BreakpointPreviewResolutionError>`
takes one preview target — its identity, node / route label, viewport label,
active breakpoint, theme / state variant, device class, data posture, runtime
origin, freshness, source-mapping quality, source-sync class, and an optional
source span — and produces:

- **Device preview row** (`M5ResolvedDevicePreviewRow`): the viewport label, the
  active breakpoint, the theme / state variant, the device class, the data posture,
  the mapping quality, the runtime origin, and a precise live-versus-mock cue label.
- **Runtime-truth cue** (`M5ResolvedRuntimeTruthCue`): the honest answer to "what am
  I looking at right now?" — the data posture (`live` / `mock` / `captured`), the
  runtime origin, the freshness (`fresh` / `aging` / `stale` / `unknown`), the
  `is_live_data` / `is_stale` flags, and a precise truth label.
- **Continuity block** (`M5ResolvedContinuity`): the compare-across-targets action
  (always offered), the open-source-for-breakpoint action (when source-anchored),
  and the always-true `preserves_selection_context` guard so the selected node
  survives across device targets, variants, and origins.
- **Downgrade trigger** (`M5VisualDesignerDowngradeTrigger`, when the preview
  degraded below a live, fresh, source-anchored view): a shared-vocabulary trigger
  so every consumer explains the degrade the same way.

### 2. The parity matrix

`M5BreakpointPreviewPacket` binds one row per claimed M5 visual-design surface
family — desktop designer, source-first preview, browser-runtime inspector,
framework-pack preview, embedded shell designer, and support-export replay — and
carries worked resolution cases whose stored resolution must equal a fresh resolve
of its input. The Rust validator is the authoritative gate.

## Acceptance criteria and how they are enforced

| Acceptance criterion | Mechanism |
| --- | --- |
| Preview surfaces no longer hide whether the user is seeing live runtime data, mock data, or a stale / captured view. | Every resolved preview carries a runtime-truth cue naming the data posture, the runtime origin, and the freshness, and its `is_live_data` / `is_stale` flags are consistent with them. `discloses_runtime_truth` proves it, and the `RuntimeTruthDisclosureUnproven` lint requires a worked mock / captured / stale case and that every case discloses. The runtime origin also anchors the posture: a captured snapshot can never claim live data, and a live runtime can never claim a captured snapshot (`ContradictoryRuntimeOrigin`). |
| Device / breakpoint switching remains source-anchored and reviewable. | A source-mapped preview keeps its source anchor (`source_anchored`) and offers an open-source action, the selected node survives across every target (`preserves_selection_context`), and the target identity is identical across the row, the cue, and the continuity block. `switching_stays_source_anchored` and `identity_consistent` prove it, and the `SourceAnchoredSwitchingUnproven` lint requires a worked source-anchored case and that every case preserves continuity. |
| Framework-pack and preview consumers use the same component truth instead of feature-local labels. | Every degrade below a live, fresh, source-anchored view carries a typed `M5VisualDesignerDowngradeTrigger` from the shared vocabulary. `degrade_is_explained` proves it, and the `DegradeExplanationUnproven` lint requires a worked degraded case with a trigger and that every case is explained. The vocabulary set is frozen (`VocabularySetDrift`) so no surface invents a parallel breakpoint-preview grammar. |

## Honesty guards enforced by the resolver

- A source-mapped preview (exact / approximate mapping quality) with no source span
  is refused with `MissingSpanForSourceMapping` — a preview cannot claim a source
  anchor it has no span for.
- An unmapped preview carrying a source span is refused with
  `ContradictoryUnmappedSpan`, and a runtime-only-no-source preview carrying a span
  is refused with `ContradictoryRuntimeSpan`.
- A declared data posture that contradicts the runtime origin is refused with
  `ContradictoryRuntimeOrigin`.

## Boundary hygiene

Raw source bodies, screenshots, runtime payloads, credentials, and raw URLs never
cross this boundary. Node / viewport labels and variant / breakpoint tokens and the
serialized packet are screened for forbidden material (`api_key`, `password`,
`secret`, bearer tokens, URLs, PEM blocks); a violation fails resolution or packet
validation.

## Reused vocabulary

The device-preview class (`M5DevicePreviewClass`), data posture
(`M5PreviewDataPosture`), breakpoint mapping quality (`M5BreakpointMappingQuality`),
preview freshness (`PreviewFreshnessClass`), source-sync class (`SourceSyncClass`),
downgrade triggers (`M5VisualDesignerDowngradeTrigger`), preview surface
(`PreviewSurface`), and visual-design surface families
(`M5VisualDesignSurfaceFamily`) are reused verbatim from the frozen visual-designer
component matrix and the sibling primitives. This module mints new vocabulary only
for the preview runtime origin (`M5PreviewRuntimeOrigin`), the continuity action
(`M5BreakpointContinuityAction`), and the export fields (`M5BreakpointExportField`).
