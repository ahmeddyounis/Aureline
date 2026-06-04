# Stable Docs Source, Result, Pack, and Citation Contract

This contract defines the metadata packet shared by docs browser rows, Help/About cards, onboarding and guided-tour steps, AI explainers, extension/help APIs, pack detail sheets, citation drawers, and support exports.

The packet is reference-only. It carries source descriptors, docs-result objects, docs-pack manifests, pack detail sheets, derived citation sets, drawer parity rows, and consumer projections. It does not carry raw docs bodies, raw URLs, secrets, provider payloads, or ambient authority.

Required stable behavior:

- Project docs, generated reference, mirrored vendor docs, and curated knowledge packs keep explicit source precedence. Project docs outrank vendor docs for repo-specific explanations unless an override is disclosed.
- Docs-result objects preserve source ref, version-match state, freshness, symbol refs, citation anchors, render config refs, suggestion refs, and validation result refs.
- Docs-pack and knowledge-pack detail sheets expose owner, version, locale coverage, trust/support class, pin state, offline availability, remove/update/offline/citation actions, and browser-handoff or online-only state.
- Derived citation sets preserve cited files, symbols, docs refs, graph epoch, locale, derivation tool/version, omitted-source markers, inference markers, and reference-only export posture.
- Citation drawers preserve supporting file/symbol/docs anchors plus omitted-source and inference markers across docs browser, Help/About, onboarding, AI explainers, and support export.
