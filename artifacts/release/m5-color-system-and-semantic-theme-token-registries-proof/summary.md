# M5 Color-System and Semantic-Theme-Token Registries

- Packet: `m5-color-system-and-semantic-theme-token-registries:stable:0001`
- Label: `M5 color-system and semantic-theme-token registries with dark / light / high-contrast parity, non-color-only meaning, explicit operational-state mappings for brand/interactive/neutral/success/warning/danger/info/insight and the trust-sensitive restricted/remote/collaboration/ai/debug states, and canonical-token tracing across shell, editor, review, notebook, data, and support surfaces`
- Consumer surfaces: 6
- Operational states: brand, interactive, neutral, success, warning, danger, info, insight, restricted, remote, collaboration, ai, debug, state_unclassified
- Theme modes: dark, light, high_contrast
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell consumes the canonical brand / interactive / neutral palettes and pairs every hue with a non-color cue; a color-only entry and a raw-hex theme token degrade honestly instead of reading as a clean pass
  - Color entries: 4 / theme-token entries: 2
- **editor_ui**: `stable`
  - Owner: Editor surface owner
  - Scope: The editor consumes the canonical success / info status colors with an icon cue across dark, light, and high-contrast; an entry that drops its non-color cue degrades honestly
  - Color entries: 3 / theme-token entries: 1
- **review_ui**: `stable`
  - Owner: Review surface owner
  - Scope: The review surface keeps warning / danger / restricted colors distinct in every mode and traces every token to the canonical color system; a mode-parity gap and a raw-color inlining degrade honestly
  - Color entries: 5 / theme-token entries: 2
- **data_ui**: `stable`
  - Owner: Data surface owner
  - Scope: The data surface keeps insight / collaboration / debug states distinguishable with shape and label cues; an indistinguishable-across-modes entry, an unclassified state, and a drifted theme role all degrade honestly
  - Color entries: 5 / theme-token entries: 2
- **docs_ui**: `stable`
  - Owner: Docs / notebook surface owner
  - Scope: The docs and notebook surfaces keep the trust-sensitive remote and AI states distinct in dark, light, and high-contrast with border and shape cues, tracing each to the canonical theme pair
  - Color entries: 2 / theme-token entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved color and theme truth, so a raw-color regression or an unstated token is visible in evidence rather than hidden behind hue
  - Color entries: 2 / theme-token entries: 1
