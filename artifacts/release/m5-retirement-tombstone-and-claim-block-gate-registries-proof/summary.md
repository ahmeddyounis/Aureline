# M5 Retirement-Tombstone and Claim-Block-Gate Registries

- Packet: `m5-retirement-tombstone-and-claim-block-gate-registries:stable:0001`
- Label: `M5 retirement-tombstone and claim-block-gate registries adding retired-state tombstones and claim-block logic to the install / update pickers, marketplace / detail surfaces, help / About truth cards, CLI / headless inspect paths, and managed new-tenant / new-workspace creation flows so a retired M5 line or stable-facing surface stops looking selectable or claimable — one export-safe retirement tombstone per retired object (its stable identity anchor, last-supported version marker, archival pointer, replacement / successor path, and removed active-selection affordance, so green / support badges and active enablement are gone while the stable ID, last supported version, and archive / replacement pointer stay discoverable historically) with canonical / accessible / audit resolution-form coverage, and a machine-readable claim-block gate (new-install-selection, new-tenant-provisioning, or active-enablement-toggle) that blocks a retired object from being offered for new install, new tenant, or active enablement, so help / About, marketplace, and CLI / headless inspection agree on one retired-state truth for the same object and no retired surface disappears without a tombstone, successor pointer, or archival route`
- Consumer surfaces: 6
- Tombstone fields: stable_identity_anchor, last_supported_version_marker, archival_pointer, replacement_path_pointer, removed_active_affordance_marker, historical_discoverability_note, tombstone_field_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **release_center**: `stable`
  - Owner: Release-center owner
  - Scope: The release center resolves a retired object to one typed retirement tombstone — its stable identity anchor, last-supported version marker, archival pointer, and replacement / successor path from the shared registry — and proves the new-install-selection claim-block gate for that object; a tombstone missing its stable-identity anchor and a claim-block gate that would still offer the object for a new install degrade honestly instead of leaving a retired surface to read as selectable
  - Tombstone entries: 2 / claim-block-gate entries: 2
- **help_docs**: `stable`
  - Owner: Help/docs owner
  - Scope: Help / About resolves the last-supported-version tombstone field and the active-enablement-toggle claim-block gate while keeping the stable identity and replacement pointer visible; a retired object still offered for active enablement and a resolution-form gap on a claim-block gate are caught before a help / About card can reintroduce a selectable reading
  - Tombstone entries: 2 / claim-block-gate entries: 2
- **support**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the replacement-path tombstone field while keeping its public-facing replacement / archival claim matched to the archived successor and reports the claim-block-gate outcome; a tombstone that is a hand-copied per-entry assumption and a claim-block gate on an unclassified scope degrade honestly
  - Tombstone entries: 2 / claim-block-gate entries: 1
- **marketplace_registry**: `stable`
  - Owner: Marketplace/registry owner
  - Scope: The marketplace / detail surface resolves the archival-pointer tombstone field and the new-tenant-provisioning claim-block gate bound to the registry so a retired surface can no longer be selected in a new install or provisioned for a new tenant while staying discoverable by its stable ID and archival pointer; an unstated registry token on a tombstone is caught before it can drift
  - Tombstone entries: 2 / claim-block-gate entries: 1
- **install_update**: `stable`
  - Owner: Install/update owner
  - Scope: Install / update pickers render the same resolved retirement-tombstone and claim-block-gate truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied class table; the historical-discoverability tombstone field and the active-enablement-toggle claim-block gate stay inspectable off-renderer so no new install can still select a retired surface as an active choice
  - Tombstone entries: 1 / claim-block-gate entries: 1
- **partner_procurement**: `stable`
  - Owner: Partner/procurement owner
  - Scope: The partner / procurement and CLI / headless inspect feed carries the same resolved retirement-tombstone and claim-block-gate truth, so a hand-copied constant, an unstated registry token, a retired object still offered for new install / new tenant / active enablement, or a green / support badge left on a retired surface is visible in evidence — a retired object blocked from new-install selection, new-tenant provisioning, or active enablement — rather than hidden behind a screenshot
  - Tombstone entries: 1 / claim-block-gate entries: 1
