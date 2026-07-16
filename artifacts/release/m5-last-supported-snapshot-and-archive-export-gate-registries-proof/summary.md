# M5 Last-Supported-Snapshot and Archive-Export-Gate Registries

- Packet: `m5-last-supported-snapshot-and-archive-export-gate-registries:stable:0001`
- Label: `M5 last-supported-snapshot and archive-export-gate registries shipping last-supported snapshot and retirement archive bundles for a retiring M5 line or stable-facing surface across the release-center, help / docs, support, marketplace / registry, install / update, and partner / procurement surfaces so migration, audit, procurement, and support can inspect what was retired without keeping the retired surface live — one export-safe last-supported snapshot per retiring object (its docs / help truth, schema / contract set, known-limits snapshot, compatibility report, provenance / SBOM reference, and support-article links captured for the final supported build or line state and joined to its exact build) with canonical / accessible / audit resolution-form coverage, and a machine-readable archive-export gate (live-dependency-present, internal-only-or-secret-leak, or unbound-manifest-or-review-packet) that blocks an archive bundle from being handed off while it carries a live vendor dependency, would leak a secret or internal-only detail, or is not bound back to the retirement manifest and review packet, so self-hosted, offline, and procurement / support consumers open one export-safe historical reference that names the final supported version / channel and the successor path without contradiction`
- Consumer surfaces: 6
- Snapshot fields: docs_help_truth, schema_contract_set, known_limits_snapshot, compatibility_report_reference, support_article_links, provenance_sbom_reference, snapshot_field_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **release_center**: `stable`
  - Owner: Release-center owner
  - Scope: The release center resolves a retiring object to one typed last-supported snapshot — its docs / help truth, schema / contract set, known-limits snapshot, and compatibility report from the shared registry, joined to its exact build — and proves the live-dependency-present archive-export gate for that bundle; a snapshot missing its docs / help truth and an archive-export gate that would hand off a bundle carrying a live vendor dependency degrade honestly instead of shipping an untrustworthy historical reference
  - Snapshot entries: 2 / archive-export-gate entries: 2
- **help_docs**: `stable`
  - Owner: Help/docs owner
  - Scope: Help / docs resolves the schema / contract-set snapshot field and the unbound-manifest-or-review-packet archive-export gate while keeping the docs / help truth and compatibility report visible; an archive bundle not bound back to the retirement manifest and a resolution-form gap on an archive-export gate are caught before a help / docs card can point at a non-reproducible historical reference
  - Snapshot entries: 2 / archive-export-gate entries: 2
- **support**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the compatibility-report snapshot field while keeping its public-facing compatibility / support claim matched to the archived successor and reports the archive-export-gate outcome; a snapshot that is a hand-copied per-entry assumption and an archive-export gate on an unclassified scope degrade honestly
  - Snapshot entries: 2 / archive-export-gate entries: 1
- **marketplace_registry**: `stable`
  - Owner: Marketplace/registry owner
  - Scope: The marketplace / registry surface resolves the known-limits-snapshot field and the internal-only-or-secret-leak archive-export gate bound to the registry so a retirement archive bundle can never be handed off carrying a leaked secret or internal-only detail while staying inspectable by its docs / help truth and known-limits snapshot; an unstated registry token on a snapshot is caught before it can drift
  - Snapshot entries: 2 / archive-export-gate entries: 1
- **install_update**: `stable`
  - Owner: Install/update owner
  - Scope: Install / update surfaces render the same resolved last-supported-snapshot and archive-export-gate truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied class table; the provenance / SBOM snapshot field and the archive-export gate stay inspectable off-renderer so an offline consumer can open the historical reference without live vendor dependencies
  - Snapshot entries: 1 / archive-export-gate entries: 1
- **partner_procurement**: `stable`
  - Owner: Partner/procurement owner
  - Scope: The partner / procurement and CLI / headless inspect feed carries the same resolved last-supported-snapshot and archive-export-gate truth, so a hand-copied constant, an unstated registry token, an archive bundle carrying a live dependency, leaking a secret / internal-only detail, or unbound from its retirement manifest and review packet is visible in evidence — an archive bundle blocked from handoff until it is export-safe and mirror-aware — rather than hidden behind a screenshot
  - Snapshot entries: 1 / archive-export-gate entries: 1
