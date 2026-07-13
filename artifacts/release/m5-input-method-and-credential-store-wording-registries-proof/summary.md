# M5 Input-Method and Credential-Store-Wording Registries

- Packet: `m5-input-method-and-credential-store-wording-registries:stable:0001`
- Label: `M5 input-method and credential-store-wording registries with committed text arriving intact across the macOS / Windows / Linux input stacks, preserved command / shortcut / trust fidelity, truthful and non-leaky credential copy, literal / canonical / accessible presentation-form coverage, and the generic-wording / disclosure-route / truthful disclosure triple across shell, settings, docs, onboarding, CLI, and support surfaces`
- Consumer surfaces: 6
- Input stacks: macos_input_methods, windows_ime_tsf, linux_ime_ibus_fcitx, stack_unclassified
- Presentation forms: literal_rendering, canonical_truth, accessible_announcement
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The editor delivers macOS marked-text IME composition intact from the shared input registry and keeps command interpretation and trust copy uncorrupted; a hand-copied per-platform composition assumption degrades honestly instead of reading as a clean pass, and a settings credential message that hides a plaintext downgrade is caught before it reads as truthful
  - Input entries: 2 / credential entries: 2
- **settings_ui**: `stable`
  - Owner: Settings surface owner
  - Scope: The terminal delivers Linux IBus / fcitx dead-key composition intact and the auth recovery dialog keeps its credential wording truthful; a dead-key composition that drifts from its expected text is caught as corrupted for its stack
  - Input entries: 2 / credential entries: 1
- **docs_help**: `stable`
  - Owner: Docs/help surface owner
  - Scope: Docs and help render the Windows AltGr settings composition across the literal, canonical, and accessible presentation forms and keep the support credential diagnostics wording truthful; an input entry and a credential entry that omit a presentation form degrade honestly so a screenshot cannot reintroduce a false-truth reading
  - Input entries: 2 / credential entries: 2
- **onboarding**: `stable`
  - Owner: Onboarding surface owner
  - Scope: Onboarding delivers macOS emoji composition from the registry while preserving command and trust fidelity; a composition that fights shortcut routing and a credential message on an unclassified surface degrade honestly
  - Input entries: 2 / credential entries: 1
- **cli_export**: `stable`
  - Owner: CLI/export owner
  - Scope: The CLI export delivers Windows layout-switch composition from the input registry and keeps the settings credential wording truthful; a composition unsupported on its surface without an explained fallback degrades honestly instead of silently dropping text entry
  - Input entries: 2 / credential entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved input-composition and credential-wording truth, so a hand-copied constant, an unstated registry token, or a hidden storage downgrade is visible in evidence rather than hidden behind a screenshot
  - Input entries: 2 / credential entries: 1
