# M5 Retirement-Closure-Ledger and Propagation-Blocker-Gate Registries

- Packet: `m5-retirement-closure-ledger-and-propagation-blocker-gate-registries:stable:0001`
- Label: `M5 retirement-closure-ledger and propagation-blocker-gate registries propagating retirement manifests, tombstones, and last-supported archive refs into mirror metadata, offline bundle manifests, self-hosted registry / catalog paths, policy bundles, and managed new-tenant / new-workspace gates across the release-center, help / docs, support, marketplace / registry, install / update, and partner / procurement surfaces so mirrors, offline bundles, self-hosted registries, and managed tenant gates all converge on the same closed retired-state truth — one export-safe retirement closure ledger per retiring object per deployment profile (recording its migration outcome, disable evidence, support-note closure, archival note, propagation status, and any remaining carve-outs, joined to its exact build) with canonical / accessible / audit resolution-form coverage, and a machine-readable propagation-blocker gate (profile-propagation-lag, profile-retired-state-mismatch, or still-advertising-after-closure) that blocks final retirement certification while a claimed profile still lags, diverges from the closed profiles, or keeps advertising a retired line or capability after another profile closed it, so a managed consumer and a mirror / offline / self-hosted consumer agree on retired-state truth for the same object and the propagation naming the archival / successor path each profile needs never leaks internal-only detail`
- Consumer surfaces: 6
- Closure fields: migration_outcome, disable_evidence, support_note_closure, archival_note, propagation_status, remaining_carve_out, closure_field_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **release_center**: `stable`
  - Owner: Release-center owner
  - Scope: The release center resolves a retiring object to one typed retirement closure ledger — its migration outcome, disable evidence, and support-note closure recorded per deployment profile from the shared registry, joined to its exact build — and raises the profile-propagation-lag blocker when a mirror or offline bundle has not yet propagated the retirement manifest, tombstone, and last-supported archive ref; a ledger missing its migration outcome and a propagation blocker whose profile still lags after another profile closed the line degrade honestly instead of letting one profile keep advertising a retired line as still live
  - Closure-ledger entries: 2 / propagation-blocker-gate entries: 2
- **help_docs**: `stable`
  - Owner: Help/docs owner
  - Scope: Help / docs resolves the disable-evidence closure field and the still-advertising-after-closure propagation blocker while keeping the migration outcome and archival note visible; a profile still offering a retired capability for new install after another profile closed it and a resolution-form gap on a propagation blocker are caught before a help / docs card can point at a line one profile has already retired
  - Closure-ledger entries: 2 / propagation-blocker-gate entries: 2
- **support**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the archival-note closure field while keeping its public-facing archival / successor claim matched to the successor path each profile still needs and reports the propagation-blocker-gate outcome; a ledger that is a hand-copied per-entry assumption and a propagation blocker on an unclassified scope degrade honestly, and no internal-only detail leaks while the archival / successor path is named
  - Closure-ledger entries: 2 / propagation-blocker-gate entries: 1
- **marketplace_registry**: `stable`
  - Owner: Marketplace/registry owner
  - Scope: The marketplace / registry surface resolves the support-note-closure field and the profile-retired-state-mismatch propagation blocker bound to the registry so a self-hosted registry or policy bundle can never keep advertising a retired line whose retired-state truth disagrees with the profiles that already closed it, while staying inspectable by its migration outcome and disable evidence; an unstated registry token on a ledger is caught before it can drift
  - Closure-ledger entries: 2 / propagation-blocker-gate entries: 1
- **install_update**: `stable`
  - Owner: Install/update owner
  - Scope: Install / update surfaces render the same resolved retirement-closure-ledger and propagation-blocker-gate truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied class table; the remaining-carve-out closure field and the still-advertising-after-closure blocker stay inspectable off-renderer so a managed new-tenant gate blocks a retired line from new provisioning and an offline consumer can read the closure state without live vendor dependencies
  - Closure-ledger entries: 1 / propagation-blocker-gate entries: 1
- **partner_procurement**: `stable`
  - Owner: Partner/procurement owner
  - Scope: The partner / procurement and CLI / headless inspect feed carries the same resolved retirement-closure-ledger and propagation-blocker-gate truth, so a hand-copied constant, an unstated registry token, a mirror still lagging its retirement manifest, or a managed tenant diverging from the profiles that already closed the line is visible in evidence — the closure ledger records propagation success / failure per profile and blocks final retirement certification while any claimed profile still diverges — rather than hidden behind a screenshot
  - Closure-ledger entries: 1 / propagation-blocker-gate entries: 1
