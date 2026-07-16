# M5 Supported-Line Truth-Feed and Audience-Packet Registries

- Packet: `m5-supported-line-truth-feed-and-audience-packet-registries:stable:0001`
- Label: `M5 supported-line truth-feed and audience-packet registries bundling one truth feed per active stable or LTS-candidate line — one row per feed section: a public-proof summary, a migration-scoreboard summary, a transparency snapshot, a correction-history summary, a claim-history summary, and a release-evidence link, tracked against exact build / release-line identity — each bound to one supported-line identity with its stable ID and freshness date and its links out to compatibility reports, known limits, migration guides, and release evidence, public-safe correction-history and claim-history summaries separated from internal-only incident / security payloads, exact-build provenance preserved so a claim never runs ahead of it, canonical / accessible / audit resolution-form coverage, and export-safe audience-packet variants (support-bundle, procurement-bundle, or partner-review-bundle) that project one canonical feed for a named audience, excluding internal-only detail by default while still naming the current claim, evidence freshness, migration posture, and correction history across release / help, docs, support, procurement, and partner surfaces`
- Consumer surfaces: 6
- Feed sections: public_proof_summary, migration_scoreboard_summary, transparency_snapshot, correction_history_summary, claim_history_summary, release_evidence_link, feed_section_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **shiproom**: `stable`
  - Owner: Shiproom owner
  - Scope: The shiproom resolves the line's public-proof summary to one typed truth feed — the feed section, its current claim, its evidence freshness, and the owning roster — from the shared registry and projects the support-bundle audience packet for that line; a truth feed missing its exact-build provenance and a packet variant that keeps a claim ahead of current proof degrade honestly instead of leaving a stale line to read as still green
  - Truth-feed entries: 2 / audience-packet entries: 2
- **release_center**: `stable`
  - Owner: Release-center owner
  - Scope: The release center resolves the migration-scoreboard-summary feed section and the partner-review-bundle audience packet while keeping the active packet note visible; a line widening its claim on stale proof and a resolution-form gap on a packet variant are caught before a screenshot can reintroduce a still-green reading
  - Truth-feed entries: 2 / audience-packet entries: 2
- **executive_steering**: `stable`
  - Owner: Executive-steering owner
  - Scope: Executive steering resolves the correction-history-summary feed section (public-facing) while keeping its published summary matched to current exact-build proof and reports the audience-packet outcome; a truth feed that is a hand-copied per-entry assumption and a packet variant on an unclassified packet scope degrade honestly
  - Truth-feed entries: 2 / audience-packet entries: 1
- **program_governance**: `stable`
  - Owner: Program-governance owner
  - Scope: Program governance resolves the transparency-snapshot feed section and the procurement-bundle audience packet bound to the registry; an unstated registry token on a truth feed is caught before it can drift
  - Truth-feed entries: 2 / audience-packet entries: 1
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics renders the same resolved truth-feed and audience-packet truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied line table; the release-evidence-link feed section and the partner-review-bundle packet variant stay inspectable off-renderer
  - Truth-feed entries: 1 / audience-packet entries: 1
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved truth-feed and audience-packet truth, so a hand-copied constant, an unstated registry token, a widen-on-stale-proof attempt, or a claim running ahead of current proof is visible in evidence — the support, procurement, and partner-review packet variants projected from one canonical feed — rather than hidden behind a shiproom note or private materials
  - Truth-feed entries: 1 / audience-packet entries: 1
