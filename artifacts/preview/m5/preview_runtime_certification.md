# M5 Source-First Preview / Browser-Runtime Release Certification

- Packet: `m5-preview-runtime-certification:stable:0001`
- Label: `M5 Source-First Preview / Browser-Runtime Release Certification`
- Rows: 8 (8 claimed, 2 certified, 2 narrowed, 2 blocked)
- Surfaces: 8 / 8
- Lanes covered: 6 / 6
- Evidence freshness SLO: 168 hours (last refresh: 2026-06-07T00:00:00Z)

## Rows

- **cert-row:source-first-framework:0001** (source_first_framework_preview): claim `certified` -> effective `certified`
  - Every release-required lane is currently proven; the source-first framework preview is release-certified
  - surface=source_first_framework_preview claim=certified effective=certified blocked=false write_capable=true
  - lanes: source_first_preview=current inspect_to_source_fidelity=current round_trip_honesty=current drift_recovery=current
- **cert-row:visual-surface-mapping:0001** (visual_surface_mapping): claim `beta` -> effective `beta`
  - Source-first preview, inspect-to-source, and round-trip proof are current; the mapping surface is certified at beta depth
  - surface=visual_surface_mapping claim=beta effective=beta blocked=false write_capable=true
  - lanes: source_first_preview=current inspect_to_source_fidelity=current round_trip_honesty=current
- **cert-row:browser-runtime-inspection:0001** (browser_runtime_inspection): claim `beta` -> effective `beta`
  - Browser-runtime inspection, inspect-to-source, provider conformance, and drift drills are current; this inspect-only row is certified at beta depth
  - surface=browser_runtime_inspection claim=beta effective=beta blocked=false write_capable=false
  - lanes: browser_runtime_inspection=current inspect_to_source_fidelity=current provider_conformance=current drift_recovery=current
- **cert-row:device-or-simulator:0001** (device_or_simulator_preview): claim `beta` -> effective `beta`
  - Source-first preview, round-trip, drift drills, and provider conformance are current; the device preview is certified at beta depth
  - surface=device_or_simulator_preview claim=beta effective=beta blocked=false write_capable=true
  - lanes: source_first_preview=current round_trip_honesty=current drift_recovery=current provider_conformance=current
- **cert-row:embedded-webview:0001** (embedded_webview_preview): claim `preview` -> effective `preview`
  - Source-first preview, browser-runtime inspection, and inspect-to-source proof are current; the embedded inspect-only row is certified at preview depth
  - surface=embedded_webview_preview claim=preview effective=preview blocked=false write_capable=false
  - lanes: source_first_preview=current browser_runtime_inspection=current inspect_to_source_fidelity=current
- **cert-row:visual-edit-transform:0001** (visual_edit_transform): claim `certified` -> effective `certified`
  - Source-first preview, round-trip honesty, and inspect-to-source proof are current; the write-capable visual-edit row is release-certified
  - surface=visual_edit_transform claim=certified effective=certified blocked=false write_capable=true
  - lanes: source_first_preview=current round_trip_honesty=current inspect_to_source_fidelity=current
- **cert-row:full-stack-loop:0001** (full_stack_preview_loop): claim `beta` -> effective `held`
  - Full-stack preview loop with a stale source map; source-first preview proof is stale, so the claim is narrowed and promotion is blocked
  - surface=full_stack_preview_loop claim=beta effective=held blocked=true write_capable=false
  - lanes: source_first_preview=stale browser_runtime_inspection=current round_trip_honesty=current drift_recovery=current
  - Degraded: The source map backing this full-stack loop went stale; the row is held below beta and promotion is blocked until the source map is refreshed
- **cert-row:support-export:0001** (support_export_projection): claim `beta` -> effective `blocked`
  - Support/export projection missing provider-conformance proof; the claim is blocked from promotion
  - surface=support_export_projection claim=beta effective=blocked blocked=true write_capable=false
  - lanes: source_first_preview=current provider_conformance=missing
  - Degraded: No provider-conformance proof is on hand for this projected row; the claim is blocked until a conformance packet is published
