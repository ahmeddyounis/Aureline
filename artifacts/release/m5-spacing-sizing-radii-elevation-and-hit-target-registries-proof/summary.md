# M5 Spacing / Sizing / Radii / Border / Elevation and Hit-Target Registries

- Packet: `m5-spacing-sizing-radii-elevation-and-hit-target-registries:stable:0001`
- Label: `M5 spacing / sizing / radii / border / elevation geometry and minimum hit-target registries with canonical density-aware primitives, an overlay / dialog elevation hierarchy, and minimum-target rules for interactive controls and resize handles across the shell, list / table, editor, dialog, review, and support surfaces`
- Consumer surfaces: 6
- Primitive kinds: spacing, sizing, radius, border, elevation, kind_unknown
- Density modes: compact, standard, comfortable, density_unknown
- Control kinds: button, icon_button, resize_handle, toggle, menu_item, control_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell spaces chrome on the canonical spacing step and sizes buttons to the comfortable minimum target; a spacing step that forks the shared foundation degrades honestly instead of reading as a clean pass
  - Geometry: 2 / hit-target: 1
- **data_ui**: `stable`
  - Owner: List / table surface owner
  - Scope: The dense list / table sizes rows on the canonical sizing step and keeps compact-density menu / row targets at the compact minimum; an icon button that shrinks below the supported minimum under compact density degrades honestly
  - Geometry: 1 / hit-target: 2
- **editor_ui**: `stable`
  - Owner: Editor surface owner
  - Scope: The editor rounds controls on the canonical radius step and keeps resize handles at the coarse-pointer minimum; a toggle with inadequate spacing between adjacent targets degrades honestly
  - Geometry: 1 / hit-target: 2
- **settings_ui**: `stable`
  - Owner: Dialog / overlay surface owner
  - Scope: The dialog host elevates modals on the canonical elevation level so overlays and dialogs stay above base content; an elevation entry that loses the intended hierarchy degrades honestly
  - Geometry: 2 / hit-target: 0
- **review_ui**: `stable`
  - Owner: Review surface owner
  - Scope: The review surface draws borders on the canonical hairline step and stays density-aware across compact / standard / comfortable modes; a border that applies one geometry regardless of density degrades honestly
  - Geometry: 2 / hit-target: 0
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved geometry and hit-target truth, so a raw-value regression, an unstated primitive kind, and a raw-layout hit target are visible in evidence rather than hidden behind rendering
  - Geometry: 2 / hit-target: 1
