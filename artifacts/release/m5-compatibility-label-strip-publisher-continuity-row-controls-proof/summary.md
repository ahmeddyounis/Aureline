# M5 Compatibility-Label-Strip and Publisher-Continuity-Row Controls

- Packet: `m5-compatibility-label-strip-publisher-continuity-row-controls:stable:0001`
- Label: `M5 compatibility-label-strip and publisher-continuity-row controls with host/version range, manifest-schema, lifecycle and replacement path, publisher continuity and transfer history, and no-stale-certified-overclaim across listing, detail, install, diagnostics, and export`
- Consumer surfaces: 5
- Registry source classes: public_registry, mirrored_registry, enterprise_registry, side_loaded, verified_partner, source_unknown
- Proof freshness SLO: 720 hours (last refresh: 2026-07-11T00:00:00Z)

## Consumer surfaces

- **marketplace_ui**: `stable`
  - Owner: Marketplace catalog owner
  - Scope: The marketplace listing renders one compatibility-label strip per artifact naming the compatibility range, host / runtime model, host-version and manifest-schema range, lifecycle state, and replacement path, and one publisher-continuity row naming verified, transferred, lost, mirrored, or unverifiable continuity so a compare decision needs no disconnected page
  - Compatibility-strip examples: 2 / publisher-continuity-row examples: 2
- **extensions_ui**: `stable`
  - Owner: Extensions manager owner
  - Scope: The extensions detail surface reuses the same lifecycle grammar, shows a deprecated artifact carrying its replacement path, names a transferred publisher's continuity language, and degrades honestly when the replacement path or continuity language is hidden
  - Compatibility-strip examples: 2 / publisher-continuity-row examples: 2
- **install_review_ui**: `stable`
  - Owner: Install-review owner
  - Scope: The install-review sheet keeps compatibility and continuity explicit before install trust silently continues, degrading honestly when the manifest-schema version or lifecycle state cannot be resolved or available transfer history is hidden
  - Compatibility-strip examples: 2 / publisher-continuity-row examples: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved strip and row truth, so an incompatible-shown-ready strip, a missing replacement path, a hidden continuity language, or a stale or unverifiable Certified overclaim is visible in evidence rather than hidden behind compact chrome
  - Compatibility-strip examples: 3 / publisher-continuity-row examples: 4
- **product_ui**: `stable`
  - Owner: In-product diagnostics owner
  - Scope: In-product listing and diagnostics surfaces reuse the same fact grammar, keep continuous and verified publishers explicit, and degrade honestly when the artifact identity is missing so no stale trust is quietly carried forward into installed-state diagnostics
  - Compatibility-strip examples: 2 / publisher-continuity-row examples: 2
