# M5 Platform-Convention, Shortcut-Notation, File-Path-Reveal, Theme/Contrast Live-Change, Credential-Store Wording, and Input-Method Platform-Fit Matrix

- Packet: `m5-platform-fit:stable:0001`
- Label: `M5 platform-convention, shortcut-notation, file-path-reveal, theme/contrast live-change, credential-store wording, and input-method platform-fit matrix`
- Platform-fit families: 6 (6 stable)
- Platform-fit roles: shortcut, window_menu, path_terminology, appearance, credential_wording, input_fidelity, command_stability
- Shortcut-notation roles: modifier_glyph_notation, accelerator_label, chord_sequence, platform_adaptive_notation, stable_command_id_binding, hardcoded_platform_notation_disallowed
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Platform-fit families

- **platform_convention**: `stable`
  - Owner: Native desktop integration owner
  - Canonical schema: `schemas/platform/m5-shortcut-notation.schema.json`
  - Scope: One platform-convention table naming window-control placement, menu-bar behavior, title-bar convention, and system-chrome integration for macOS, Windows, and Linux so high-frequency actions are never hidden in OS chrome alone and command IDs stay stable while platform labels adapt
  - Required labels: identity, semantic_role, registry_reference, host_platform
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **shortcut_notation**: `stable`
  - Owner: Keyboard and command owner
  - Canonical schema: `schemas/platform/m5-shortcut-notation.schema.json`
  - Scope: One shortcut-notation contract naming the modifier glyphs, accelerator labels, and chord sequences so notation adapts per platform (⌘/⌥/⌃/⇧ on macOS, Ctrl/Alt/Shift elsewhere) while the underlying command ID stays stable and is never hard-coded for one platform
  - Required labels: identity, semantic_role, registry_reference, shortcut_notation
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **file_path_reveal**: `stable`
  - Owner: File and path terminology owner
  - Canonical schema: `schemas/platform/m5-file-path-and-reveal.schema.json`
  - Scope: One file-path-reveal contract naming the reveal verb (Reveal in Finder / Show in Explorer / Open Containing Folder), the save-dialog terminology, and host-matched separators and case so file, path, reveal, and save wording matches the host platform and is never mislabeled in screenshots or help
  - Required labels: identity, semantic_role, registry_reference, path_verb
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **theme_contrast_live_change**: `stable`
  - Owner: Appearance-session owner
  - Canonical schema: `schemas/platform/m5-file-path-and-reveal.schema.json`
  - Scope: One theme-contrast-live-change contract naming the live theme, contrast, accent, and text-scale response so system appearance changes apply live or explain their fallback rather than silently drifting, and so appearance survives zoom and high-contrast modes
  - Required labels: identity, semantic_role, registry_reference, host_platform
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **credential_store_wording**: `stable`
  - Owner: Credential-state owner
  - Canonical schema: `schemas/platform/m5-file-path-and-reveal.schema.json`
  - Scope: One credential-store-wording contract naming the host store (Keychain / Credential Manager / Secret Service), the truthful storage claim, non-leaky wording, and any disclosed fallback so credential messaging never claims stronger protection than it has and never silently falls back to plaintext storage
  - Required labels: identity, semantic_role, registry_reference, host_platform
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **input_method**: `stable`
  - Owner: Input-handling owner
  - Canonical schema: `schemas/platform/m5-input-method-behavior.schema.json`
  - Scope: One input-method contract naming IME composition, dead keys and AltGr, dictation and emoji, and layout switching so text and trust fidelity are preserved under every input method and layout, and so no input path corrupts committed text or trust semantics
  - Required labels: identity, semantic_role, registry_reference, host_platform
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
