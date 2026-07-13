# M5 Shortcut-Notation and Command-Label Registries

- Packet: `m5-shortcut-notation-and-command-label-registries:stable:0001`
- Label: `M5 shortcut-notation and command-label registries with platform-native macOS glyph notation, Windows / Linux modifier names, explicit reserved-key fallbacks, visual / spoken / searchable notation-form coverage, and stable-command-ID / human-label / shortcut-text discovery across shell, settings, docs, onboarding, CLI, and support surfaces`
- Consumer surfaces: 6
- Host platforms: macos, windows, linux, platform_unknown
- Notation forms: visual_notation, spoken_accessible_form, searchable_command_text
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell renders the macOS ⌘S menu accelerator and the Save menu label from the shared keybinding registry; a hand-copied per-platform notation and a command-label mapping that is not discoverable by ID, label, and shortcut degrade honestly instead of reading as a clean pass
  - Shortcut-notation entries: 2 / command-label entries: 2
- **settings_ui**: `stable`
  - Owner: Settings surface owner
  - Scope: The keybinding inspector renders the Windows Ctrl+Shift+P palette accelerator and the palette label from the registry; a macOS entry rendered with a Windows modifier name is caught as mislabeled for its host
  - Shortcut-notation entries: 2 / command-label entries: 1
- **docs_help**: `stable`
  - Owner: Docs/help surface owner
  - Scope: Docs and help render the macOS ⌘⇧/ help accelerator and the help label across the visual, spoken, and searchable notation forms; a notation and a label that omit a notation form degrade honestly so a screenshot cannot reintroduce incorrect notation
  - Shortcut-notation entries: 2 / command-label entries: 2
- **onboarding**: `stable`
  - Owner: Onboarding surface owner
  - Scope: Onboarding renders the Windows Ctrl+N new-file accelerator from the registry while keeping the command ID stable; a notation that would change command identity and a label with an unclassified kind degrade honestly
  - Shortcut-notation entries: 2 / command-label entries: 1
- **cli_export**: `stable`
  - Owner: CLI/export owner
  - Scope: The CLI export renders the Linux Ctrl+K Ctrl+S chord from the keybinding inspector registry and the palette label; a shortcut reserved by the OS without an explained fallback degrades honestly instead of silently dropping the action
  - Shortcut-notation entries: 2 / command-label entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved shortcut-notation and command-label truth, so a hand-copied constant or an unstated registry token is visible in evidence rather than hidden behind a screenshot
  - Shortcut-notation entries: 2 / command-label entries: 1
