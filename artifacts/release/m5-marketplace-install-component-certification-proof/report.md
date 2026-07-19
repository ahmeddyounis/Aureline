# M5 Marketplace / Install-Review Component Surface Certification

- Packet: `m5-marketplace-install-component-certification:stable:0001`
- As of: `2026-07-11T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-marketplace-install-proof/support_export.json`
- Profiles: 8 / 8 certified (4 green, 4 yellow, 0 red)
- Families covered: true
- Guardrails held: true
- Auto-narrowed profiles: 4
- Report clean: true

## Profiles

- **cert:public-verified-registry** — profile=public_verified_registry claimed=install_ready_result certified=install_ready_result status=green narrowed_axes=0
- **cert:mirrored-registry** — profile=mirrored_registry claimed=reviewable_listing_result certified=reviewable_listing_result status=green narrowed_axes=0
- **cert:enterprise-registry** — profile=enterprise_registry claimed=reviewable_listing_result certified=reviewable_listing_result status=green narrowed_axes=0
- **cert:side-load-reviewed-registry** — profile=side_load_reviewed_registry claimed=reviewable_listing_result certified=reviewable_listing_result status=green narrowed_axes=0
- **cert:stale-compatibility-registry** — profile=stale_compatibility_registry claimed=reviewable_listing_result certified=compatibility_unverified_projection status=yellow narrowed_axes=1
- **cert:over-budget-throttled-registry** — profile=over_budget_throttled_registry claimed=reviewable_listing_result certified=activation_budget_projection status=yellow narrowed_axes=1
- **cert:rollback-unverifiable-registry** — profile=rollback_unverifiable_registry claimed=reviewable_listing_result certified=rollback_unverified_projection status=yellow narrowed_axes=1
- **cert:transferred-publisher-registry** — profile=transferred_publisher_registry claimed=reviewable_listing_result certified=publisher_continuity_projection status=yellow narrowed_axes=1
