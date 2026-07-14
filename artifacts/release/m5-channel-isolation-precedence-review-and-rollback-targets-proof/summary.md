# M5 Channel-Isolation, Precedence-Review, and Rollback-Target Registries

- Packet: `m5-channel-isolation-precedence-review-and-rollback-targets:stable:0001`
- Label: `M5 side-by-side channel-isolation, association-precedence-review, and full artifact-graph rollback-target registries enforcing isolated channel roots and mutable-state namespaces of channel root / state namespace / secrets namespace / services namespace, a never-reused stable durable-state namespace, explicit isolated-versus-governed-handoff containment, published file-association / protocol-handler / deep-link / default-open precedence rules, and full artifact-graph rollback targets across the installer, update, diagnostics, admin, docs, and support surfaces`
- Consumer surfaces: 6
- Channels: stable, preview, beta, lts, channel_unclassified
- Presentation forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last refresh: 2026-07-14T00:00:00Z)

## Consumer surfaces

- **installer**: `stable`
  - Owner: Installer/side-by-side-channel owner
  - Scope: The installer resolves the stable channel to one inspectable object — the channel, the channel / state-namespace / secrets-namespace roots, and the isolated channel-root / state-namespace / secrets-namespace / services-namespace inventory — from the shared registry and publishes the file-association precedence rule bound to the full rollback artifact graph; an isolation inventory that omits the services namespace and a precedence rule that hides the before/after inspectability and ownership degrade honestly instead of reading as a clean pass
  - Channel entries: 2 / precedence entries: 2
- **updater_service**: `stable`
  - Owner: Updater/channel-coexistence owner
  - Scope: The updater resolves the preview channel and the protocol-handler precedence rule; a preview channel that reused the stable durable-state namespace without a governed handoff and a rollback target narrowed to the primary executable while its artifact-graph continuity is undocumented are caught before an update can corrupt a coexisting channel or restore an install untruthfully
  - Channel entries: 2 / precedence entries: 2
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics reports the beta channel and its published precedence rule without manual reconstruction; a channel whose containment is ambiguous — so a coexisting channel could corrupt its durable state — is caught instead of reading as a clean pass
  - Channel entries: 2 / precedence entries: 1
- **admin**: `stable`
  - Owner: Admin surface owner
  - Scope: Admin resolves the LTS channel and the default-open precedence rule while preserving one registry-bound source; a hand-copied per-profile assumption and a precedence rule on an unclassified domain degrade honestly
  - Channel entries: 2 / precedence entries: 2
- **docs_help**: `stable`
  - Owner: Docs/help surface owner
  - Scope: Docs and help render the same resolved channel-isolation and published-precedence truth the resolvers produced across the canonical, accessible, and audit presentation forms rather than a hand-copied channel-root table
  - Channel entries: 2 / precedence entries: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved channel-isolation and precedence truth, so a hand-copied constant, an unstated registry token, an ambiguous containment, or a preview channel reusing the stable namespace is visible in evidence rather than hidden behind a screenshot
  - Channel entries: 2 / precedence entries: 1
