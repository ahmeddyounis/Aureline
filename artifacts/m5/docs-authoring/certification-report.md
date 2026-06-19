# M5 Docs Authoring Certification

- Report: `m5-docs-authoring-certification:stable:0001`
- Label: `M5 Docs Authoring Certification`
- Profiles: 6 (6 certified, 0 narrowed, 0 blocked)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-18T00:00:00Z)

## Profiles

- **desktop**: `stable` / `certified` (freshness `current`)
  - Scope: Local desktop docs authoring with first-party packs: workspace, CommonMark preview, maintenance suggestions, validation, and evidence handoff all certified with current proof
- **mirrored**: `stable` / `certified` (freshness `current`)
  - Scope: Mirror-aware authoring backed by a pinned, signed mirror that outranks live vendor docs; recall falls back to last-known-good with explicit freshness labels
- **cached**: `stable` / `certified` (freshness `current`)
  - Scope: Cached / last-known-good authoring while the source is offline, with visible freshness and source-version truth on every surface
- **pinned_pack**: `stable` / `certified` (freshness `current`)
  - Scope: Pinned docs-pack authoring against a frozen pack revision; the pinned revision and its signature stay visible across the authoring stack
- **extension_owned**: `beta` / `certified` (freshness `current`)
  - Scope: Extension-owned docs surface running in a less-trusted host; the authoring stack is capped at Beta and rendered preview stays sanitized with no authority expansion
- **browser_handoff**: `beta` / `certified` (freshness `current`)
  - Scope: Browser-handoff companion docs editing with a safe return path to the IDE; the narrow companion surface is capped at Beta and never widens authority

## Known limits

- Extension-owned and browser-handoff docs authoring are capped at Beta because they run outside the first-party desktop trust boundary.
- Rendered preview never executes diagrams, math, or custom components as privileged code; unsafe or unrequested capabilities are blocked, not rendered.
- Cached and mirrored profiles serve last-known-good docs with explicit freshness labels and never present stale content as current.
- This certification covers the desktop/local-first docs-authoring contract only; no browser-first docs product, collaborative rich-text editor, or remote CMS workflow is claimed.
