# M5 Motion-Token and Reduced-Motion Registries

- Packet: `m5-motion-token-and-reduced-motion-registries:stable:0001`
- Label: `M5 motion-token and reduced-motion registries with canonical duration / easing families, reduced-motion / power-saver / thermal clamp coverage, no-protected-path-delay and no-layout-shift guarantees for command-palette / menu / typing / inline-editor / diagnostic surfaces, static-fallback equivalence, and canonical-token tracing across shell, dialog, panel, embedded, notification, and support surfaces`
- Consumer surfaces: 6
- Motion surface classes: command_palette_input, menu_navigation, typing_caret, inline_editor, diagnostic_surface, dialog_entrance, panel_transition, overlay_reveal, notification_entrance, tooltip_reveal, progress_indicator, onboarding_sequence, focus_transition, surface_class_unclassified
- Motion clamps: reduced_motion, power_saver, thermal
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell resolves command-palette and menu motion through the canonical instant grammar and never delays protected input; a motion entry that delays the palette and a reduced-motion entry that rides on motion alone degrade honestly instead of reading as a clean pass
  - Motion entries: 3 / reduced-motion entries: 2
- **editor_ui**: `stable`
  - Owner: Editor surface owner
  - Scope: The editor keeps the typing caret and inline editor effectively instant with a static fallback across every clamp; a dropped fallback and an introduced layout shift both degrade honestly
  - Motion entries: 4 / reduced-motion entries: 1
- **onboarding_ui**: `stable`
  - Owner: Onboarding surface owner
  - Scope: The onboarding and diagnostic surfaces clarify origin and completion while keeping diagnostics instant and tracing each token to the canonical motion system; a clamp-incomplete motion entry and a clamp-incomplete reduced-motion entry degrade honestly
  - Motion entries: 3 / reduced-motion entries: 2
- **marketplace_ui**: `stable`
  - Owner: Marketplace / embedded surface owner
  - Scope: The embedded dialog surface consumes the canonical dialog entrance duration and traces every token to the motion system; a raw-duration motion entry and a non-equivalent static fallback degrade honestly
  - Motion entries: 2 / reduced-motion entries: 2
- **settings_ui**: `stable`
  - Owner: Settings surface owner
  - Scope: The settings and notification surfaces route attention with a completion cue and respect the user's reduced-motion preference; an unclassified surface class degrades honestly instead of animating outside the grammar
  - Motion entries: 2 / reduced-motion entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved motion and reduced-motion truth, so a protected-path delay or an unstated token is visible in evidence rather than hidden behind an animation curve
  - Motion entries: 2 / reduced-motion entries: 1
