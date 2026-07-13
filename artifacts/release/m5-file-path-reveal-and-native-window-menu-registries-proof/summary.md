# M5 File-Path-Presentation and Native-Window / Menu Registries

- Packet: `m5-file-path-reveal-and-native-window-menu-registries:stable:0001`
- Label: `M5 file-path-presentation and native-window / menu registries with host-correct separators and reveal verbs (Reveal in Finder / Show in Explorer / Open Containing Folder), explicit literal-versus-canonical path truth, host-styled / canonical / accessible presentation-form coverage, and stable-ID / in-product-surface / command reachability across shell, settings, docs, onboarding, CLI, and support surfaces`
- Consumer surfaces: 6
- Host platforms: macos, windows, linux, platform_unknown
- Presentation forms: host_styled_display, canonical_truth, accessible_announcement
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell renders the macOS /Users open-dialog path and the Reveal in Finder verb from the shared path registry, and keeps Reveal reachable from the command palette; a hand-copied per-platform verb and an action reachable only through OS chrome degrade honestly instead of reading as a clean pass
  - File-path entries: 2 / window-menu entries: 2
- **settings_ui**: `stable`
  - Owner: Settings surface owner
  - Scope: The settings path presentation renders the Windows C:\ save-dialog path and the Show in Explorer verb from the registry, and keeps Save reachable from the toolbar; a Windows entry rendered with a forward-slash separator is caught as mislabeled for its host
  - File-path entries: 2 / window-menu entries: 1
- **docs_help**: `stable`
  - Owner: Docs/help surface owner
  - Scope: Docs and help render the Windows Program Files reveal path across the host-styled, canonical, and accessible presentation forms, and keep Open reachable from the command list; a path and an action that omit a presentation form degrade honestly so a screenshot cannot reintroduce an incorrect path verb
  - File-path entries: 2 / window-menu entries: 2
- **onboarding**: `stable`
  - Owner: Onboarding surface owner
  - Scope: Onboarding renders the macOS breadcrumb path from the registry while keeping the literal-versus-canonical path truth explicit; a path that drops canonical truth and an action with an unclassified product surface degrade honestly
  - File-path entries: 2 / window-menu entries: 1
- **cli_export**: `stable`
  - Owner: CLI/export owner
  - Scope: The CLI export renders the Linux Open Containing Folder reveal verb from the path registry and keeps Reveal reachable from the command palette; a reveal target that is unavailable without an explained fallback degrades honestly instead of silently dropping the action
  - File-path entries: 2 / window-menu entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved file-path and window / menu truth, so a hand-copied constant or an unstated registry token is visible in evidence rather than hidden behind a screenshot
  - File-path entries: 2 / window-menu entries: 1
