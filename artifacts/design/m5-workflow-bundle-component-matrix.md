# M5 Workflow-Bundle Component Matrix

- Packet: `m5-workflow-bundle-component-matrix:stable:0001`
- Label: `M5 Workflow-Bundle Component Matrix`
- Components: 10 across 9 / 9 families (3 degraded)

## Components

- **component:start-center-bundle-card:0001** (start_center_bundle_card) — Start-center card offering a certified launch bundle
  - A start-center bundle card keeps signer/source, support class, and certification freshness explicit before a stack is chosen
  - family=start_center_bundle_card truth=live class=launch_bundle export_safe=true assistive=true
- **component:start-center-bundle-card:0002** (start_center_bundle_card) — Start-center card offering an imported bundle from a stale mirror
  - A start-center bundle card discloses that this imported bundle was read from a stale mirror rather than imply a live certified source
  - family=start_center_bundle_card truth=mirrored class=imported_handoff_bundle export_safe=true assistive=true
  - Degraded: trigger=mirror_stale — This bundle was read from a mirror last refreshed beyond its freshness window; the card names the mirror age and offers a refresh route
- **component:certified-archetype-badge-group:0001** (certified_archetype_badge_group) — Certified-archetype badge group for a confirmed Rust workspace
  - A certified-archetype badge group projects the shared certification vocabulary and never mints a private badge meaning
  - family=certified_archetype_badge_group truth=live class=framework_pack export_safe=true assistive=true
- **component:bundle-detail-page:0001** (bundle_detail_page) — Bundle detail page for a managed-approved bundle
  - A bundle detail page keeps signer/source, certification, compatible range, entitlement dependencies, and mirror/offline posture explicit
  - family=bundle_detail_page truth=live class=org_managed_bundle export_safe=true assistive=true
- **component:bundle-install-update-review-sheet:0001** (bundle_install_update_review_sheet) — Install/update review sheet for a bundle update
  - An install/update review sheet keeps diff scope and local-override state inspectable and never applies before review
  - family=bundle_install_update_review_sheet truth=live class=launch_bundle export_safe=true assistive=true
- **component:bundle-drift-banner:0001** (bundle_drift_banner) — Drift banner for a diverged bundle
  - A drift banner keeps bundle drift distinct from a generic package update and discloses local-override state
  - family=bundle_drift_banner truth=live class=launch_bundle export_safe=true assistive=true
  - Degraded: trigger=local_override_drift — Local overrides on this bundle have diverged from the certified revision; the banner names the diverged assets and offers a compare/adopt route
- **component:bundle-local-override-row:0001** (bundle_local_override_row) — Local-override row for an overridden bundle-owned asset
  - A local-override row keeps override ownership explicit and never silently discards local work
  - family=bundle_local_override_row truth=live class=launch_bundle export_safe=true assistive=true
- **component:bundle-rollback-remove-card:0001** (bundle_rollback_remove_card) — Rollback/remove card for a bundle removal
  - A rollback/remove card names the rollback path and side effects before a durable removal
  - family=bundle_rollback_remove_card truth=live class=template_bundle export_safe=true assistive=true
- **component:bundle-class-disclosure-card:0001** (bundle_class_disclosure_card) — Bundle class disclosure card for a community template bundle
  - A bundle class disclosure card explains a bundle's class and source using the shared class vocabulary and never invents a private class meaning
  - family=bundle_class_disclosure_card truth=live class=template_bundle export_safe=true assistive=true
- **component:bundle-claim-narrowing-row:0001** (bundle_claim_narrowing_row) — Claim-narrowing row for a bundle with stale certification
  - A claim-narrowing row narrows a bundle claim on stale certification and names the reason rather than coining private stale-claim wording
  - family=bundle_claim_narrowing_row truth=imported class=imported_handoff_bundle export_safe=true assistive=true
  - Degraded: trigger=stale_certification — This imported bundle's certification is past its freshness window, so the row narrows the claim to bridged and names the required re-certification
