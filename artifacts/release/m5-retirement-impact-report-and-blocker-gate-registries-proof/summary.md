# M5 Retirement-Impact-Report and Impact-Blocker-Gate Registries

- Packet: `m5-retirement-impact-report-and-blocker-gate-registries:stable:0001`
- Label: `M5 retirement-impact-report and impact-blocker-gate registries emitting one export-safe retirement impact report per retirement candidate — one classified dependency finding per detected dependency (a workflow bundle, migration pack, command / deep link, CLI alias, SDK contract row, saved artifact, profile, recipe, marketplace entry, mirror, or managed / new-tenant offering), typed as blocking, migration-required, historical-only, mirror-only, tenant-gated, or informational with an owning team and joined to the current compatibility / public-proof state and the successor path or manual fallback, so a retiring surface never closes its support window over a dangling dependency and no new install or new tenant can still select it, with canonical / accessible / audit resolution-form coverage, and a machine-readable impact blocker gate (bundle-still-points-at-candidate, tenant-still-points-at-candidate, or schema-or-public-artifact-still-points-at-candidate) that blocks closure while a live dependency still points at the candidate across review-packet, support-export, and public-proof surfaces`
- Consumer surfaces: 6
- Report sections: blocking, migration_required, historical_only, mirror_only, tenant_gated, informational, impact_class_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **release_center**: `stable`
  - Owner: Release-center owner
  - Scope: The release center resolves a retirement candidate to one typed retirement-impact-report object — the classified dependency finding, its owning team, the exact-build joins, and the successor path or manual fallback — from the shared registry and proves the bundle-still-points-at-candidate blocker for that candidate; an impact report missing its exact-build joins and a blocker that keeps support language ahead of the closed support note degrade honestly instead of leaving a retiring surface to read as safe to close
  - Correction-report entries: 2 / claim-history-diff entries: 2
- **help_docs**: `stable`
  - Owner: Help/docs owner
  - Scope: Help / docs resolves the migration-required finding and the schema-or-public-artifact-still-points-at-candidate blocker while keeping the active gate reason visible; a retiring surface widening its claim without a preserved rollback / export route and a resolution-form gap on a blocker are caught before a screenshot can reintroduce a safe-to-close reading
  - Correction-report entries: 2 / claim-history-diff entries: 2
- **support**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the mirror-only finding while keeping its public-facing successor / fallback claim matched to the closed support note and reports the impact-blocker-gate outcome; an impact-report entry that is a hand-copied per-entry assumption and a blocker on an unclassified gate scope degrade honestly
  - Correction-report entries: 2 / claim-history-diff entries: 1
- **marketplace_registry**: `stable`
  - Owner: Marketplace/registry owner
  - Scope: The marketplace / registry resolves the historical-only finding and the tenant-still-points-at-candidate blocker bound to the registry so a retiring surface can no longer be selected in a new install or by a new tenant while a dependent remains; an unstated registry token on an impact-report entry is caught before it can drift
  - Correction-report entries: 2 / claim-history-diff entries: 1
- **install_update**: `stable`
  - Owner: Install/update owner
  - Scope: Install / update renders the same resolved retirement-impact-report and impact-blocker-gate truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied class table; the informational finding and the schema-or-public-artifact-still-points-at-candidate blocker stay inspectable off-renderer so no new install can still select a retiring surface with an open blocker
  - Correction-report entries: 1 / claim-history-diff entries: 1
- **partner_procurement**: `stable`
  - Owner: Partner/procurement owner
  - Scope: The partner / procurement feed carries the same resolved retirement-impact-report and impact-blocker-gate truth, so a hand-copied constant, an unstated registry token, an impact report widening its claim without a preserved rollback / export route, or support language running ahead of the closed support note is visible in evidence — a bundle, a tenant, or a schema / public artifact still pointing at the candidate — rather than hidden behind a screenshot
  - Correction-report entries: 1 / claim-history-diff entries: 1
