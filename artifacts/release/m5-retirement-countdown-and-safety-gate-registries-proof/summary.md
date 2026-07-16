# M5 Retirement-Countdown and Pre-Retirement-Safety-Gate Registries

- Packet: `m5-retirement-countdown-and-safety-gate-registries:stable:0001`
- Label: `M5 retirement-countdown and pre-retirement-safety-gate registries emitting one export-safe grace-window countdown per affected install / update, settings / help, docs, marketplace, and support surface — one classified countdown field per published fact (the first-deprecated version, cutoff version / date, remaining overlap window, successor route, fallback action, or no-surprises explanation) with an owning team and joined to the current compatibility / public-proof state and the successor path or manual fallback, so a retirement stops being a hidden date in release notes and becomes a visible, inspectable countdown, with canonical / accessible / audit resolution-form coverage, and a machine-readable pre-retirement safety gate (missing-rollback-or-export-path, missing-archive-bundle, or missing-successor-or-fallback-route) that blocks a candidate from passing to final closure while it is still missing a declared rollback / export / archive path or successor / fallback route, so a product surface and an operator / support surface open the same cutoff and successor data without contradiction and no surface transitions to Retired through a surprise shutdown`
- Consumer surfaces: 6
- Countdown fields: first_deprecated_version, cutoff_version_or_date, remaining_overlap_window, successor_route, fallback_action, no_surprises_explanation, countdown_field_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **release_center**: `stable`
  - Owner: Release-center owner
  - Scope: The release center resolves an affected surface to one typed retirement-countdown object — the classified countdown field (here the first-deprecated version), its owning team, the exact-build joins, and the successor route or manual fallback — from the shared registry and proves the missing-rollback-or-export-path safety gate for that candidate; a countdown missing its exact-build joins and a gate that keeps support language ahead of the closed support note degrade honestly instead of leaving a retiring surface to read as safe to close
  - Countdown entries: 2 / safety-gate entries: 2
- **help_docs**: `stable`
  - Owner: Help/docs owner
  - Scope: Help / docs resolves the cutoff-version-or-date countdown field and the missing-successor-or-fallback-route safety gate while keeping the active gate reason visible; a retiring surface widening its claim without a preserved rollback / export route and a resolution-form gap on a gate are caught before a screenshot can reintroduce a safe-to-close reading
  - Countdown entries: 2 / safety-gate entries: 2
- **support**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the successor-route countdown field while keeping its public-facing successor / fallback claim matched to the closed support note and reports the pre-retirement-safety-gate outcome; a countdown entry that is a hand-copied per-entry assumption and a gate on an unclassified gate scope degrade honestly
  - Countdown entries: 2 / safety-gate entries: 1
- **marketplace_registry**: `stable`
  - Owner: Marketplace/registry owner
  - Scope: The marketplace / registry resolves the remaining-overlap-window countdown field and the missing-archive-bundle safety gate bound to the registry so a retiring surface can no longer be selected in a new install or by a new tenant while it is still missing a declared safe-exit route; an unstated registry token on a countdown entry is caught before it can drift
  - Countdown entries: 2 / safety-gate entries: 1
- **install_update**: `stable`
  - Owner: Install/update owner
  - Scope: Install / update renders the same resolved retirement-countdown and pre-retirement-safety-gate truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied class table; the no-surprises-explanation countdown field and the missing-successor-or-fallback-route safety gate stay inspectable off-renderer so no new install can still select a retiring surface with an open pre-cutoff blocker
  - Countdown entries: 1 / safety-gate entries: 1
- **partner_procurement**: `stable`
  - Owner: Partner/procurement owner
  - Scope: The partner / procurement feed carries the same resolved retirement-countdown and pre-retirement-safety-gate truth, so a hand-copied constant, an unstated registry token, a countdown widening its claim without a preserved rollback / export route, or support language running ahead of the closed support note is visible in evidence — a candidate missing its declared rollback / export path, archive bundle, or successor / fallback route — rather than hidden behind a screenshot
  - Countdown entries: 1 / safety-gate entries: 1
