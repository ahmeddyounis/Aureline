# M5 Breakpoint / Device-Preview Row Primitive: Device Row, Runtime-Truth Cue, and Continuity Actions

- Packet: `m5-breakpoint-device-preview-primitive:stable:0001`
- Label: `M5 Breakpoint / Device-Preview Row Primitive: Device Row, Runtime-Truth Cue, and Continuity Actions`
- Visual-design surfaces: 6 / 6
- Runtime origins: live_dev_runtime, local_mock_runtime, captured_snapshot, tethered_device, simulator_runtime
- Data postures: live, mock, captured
- Continuity actions: compare_across_targets, open_source_for_breakpoint, reattach_runtime, pin_capture, inspect_only

## Visual-design surfaces

- **Desktop Designer**: `visual_surface_mapping`
  - Owner: Visual Designer Platform
  - Scope: Desktop designer breakpoint row for a live, source-anchored element across viewports
  - Worked previews: 2
    - `target:desktop:hero-heading:0001` → node `HeroHeading` device `desktop_viewport`, posture `live`, origin `live_dev_runtime`, freshness `fresh`
    - `target:desktop:hero-heading:0001` → node `HeroHeading` device `mobile_viewport`, posture `live`, origin `live_dev_runtime`, freshness `fresh`
- **Source-First Preview**: `source_first_framework_preview`
  - Owner: Source-First Preview
  - Scope: Source-first preview breakpoint row disclosing mock data on a source-anchored node
  - Worked previews: 1
    - `target:preview:pricing-card:0001` → node `PricingCard` device `tablet_viewport`, posture `mock`, origin `local_mock_runtime`, freshness `fresh`
- **Browser-Runtime Inspector**: `browser_runtime_inspection`
  - Owner: Browser Runtime Inspector
  - Scope: Browser-runtime inspector breakpoint row for a runtime-only node with no saved source
  - Worked previews: 1
    - `target:runtime:status-badge:0001` → node `StatusBadge` device `mobile_viewport`, posture `live`, origin `live_dev_runtime`, freshness `fresh`
- **Framework-Pack Preview**: `device_or_simulator_preview`
  - Owner: Framework Packs
  - Scope: Framework-pack device preview row for a tethered device whose live view went stale
  - Worked previews: 1
    - `target:framework:cart-badge:0001` → node `CartBadge` device `device_tethered`, posture `live`, origin `tethered_device`, freshness `stale`
- **Embedded Shell Designer**: `embedded_webview_preview`
  - Owner: Embedded Designer
  - Scope: Embedded shell designer breakpoint row replaying a captured snapshot on a custom viewport
  - Worked previews: 1
    - `target:shell:onboarding-card:0001` → node `OnboardingCard` device `custom_viewport`, posture `captured`, origin `captured_snapshot`, freshness `aging`
- **Support-Export Replay**: `support_export_projection`
  - Owner: Support Export
  - Scope: Support-export replay of a captured breakpoint preview with unknown freshness
  - Worked previews: 1
    - `target:support:list-item:0001` → node `ListItemRow` device `simulator_preview`, posture `captured`, origin `captured_snapshot`, freshness `unknown`
