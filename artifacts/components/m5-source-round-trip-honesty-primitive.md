# M5 Source-Round-Trip Honesty Primitive: Sync Chip, Conflict Banner, Unsupported Card, and Boundary Notice

- Packet: `m5-source-round-trip-honesty-primitive:stable:0001`
- Label: `M5 Source-Round-Trip Honesty Primitive: Sync Chip, Conflict Banner, Unsupported Card, and Boundary Notice`
- Visual-design surfaces: 6 / 6
- Chip states: in_sync, unsaved, needs_refresh, unsupported_construct, conflict
- Write authorities: writable, writable_with_review, source_only_fallback, read_only
- Boundary classes: author_owned, generated_managed, protected_read_only, mixed_managed_region, external_vendored

## Visual-design surfaces

- **Desktop Designer**: `visual_surface_mapping`
  - Owner: Visual Designer Platform
  - Scope: Desktop designer source-sync chip and round-trip status for an author-owned element
  - Worked statuses: 2
    - `target:desktop:hero-heading:0001` → node `HeroHeading` chip `in_sync`, authority `writable`, boundary `author_owned`
    - `target:desktop:hero-cta:0002` → node `HeroCta` chip `unsaved`, authority `writable`, boundary `author_owned`
- **Source-First Preview**: `source_first_framework_preview`
  - Owner: Source-First Preview
  - Scope: Source-first preview conflict banner when canonical source changed under a visual edit
  - Worked statuses: 1
    - `target:preview:pricing-card:0001` → node `PricingCard` chip `conflict`, authority `source_only_fallback`, boundary `author_owned`
- **Browser-Runtime Inspector**: `browser_runtime_inspection`
  - Owner: Browser Runtime Inspector
  - Scope: Browser-runtime inspector source-sync chip for a runtime-only node with no saved source
  - Worked statuses: 1
    - `target:runtime:status-badge:0001` → node `StatusBadge` chip `needs_refresh`, authority `read_only`, boundary `author_owned`
- **Framework-Pack Preview**: `visual_edit_transform`
  - Owner: Framework Packs
  - Scope: Framework-pack preview unsupported-construct card for a dynamically bound value
  - Worked statuses: 1
    - `target:framework:cart-count:0001` → node `CartCount` chip `unsupported_construct`, authority `source_only_fallback`, boundary `author_owned`
- **Embedded Shell Designer**: `embedded_webview_preview`
  - Owner: Embedded Designer
  - Scope: Embedded shell designer boundary notice for a generated / managed file target
  - Worked statuses: 1
    - `target:shell:generated-route:0001` → node `GeneratedRouteTable` chip `in_sync`, authority `read_only`, boundary `generated_managed`
- **Support-Export Replay**: `support_export_projection`
  - Owner: Support Export
  - Scope: Support-export replay of a captured round-trip status for a drifted node on a mixed managed region
  - Worked statuses: 1
    - `target:support:list-item:0001` → node `ListItemRow` chip `needs_refresh`, authority `source_only_fallback`, boundary `mixed_managed_region`
