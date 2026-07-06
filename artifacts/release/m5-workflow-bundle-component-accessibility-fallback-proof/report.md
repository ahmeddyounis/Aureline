# M5 Workflow-Bundle Component Accessibility & Auto-Narrowing

- Packet: `m5-workflow-bundle-component-accessibility-fallback:stable:0001`
- As of: `2026-07-06T00:00:00Z`
- Families: 9 certified across 9 / 9 frozen families
- Status: 2 green / 7 yellow / 0 red

## Rows

- **a11y:start-center-bundle-card** (start_center_bundle_card) — family=start_center_bundle_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=certified effective_claim=certified status=parity
- **a11y:certified-archetype-badge-group** (certified_archetype_badge_group) — family=certified_archetype_badge_group keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=certified effective_claim=retest_pending status=narrowed_disclosed
  - Auto-narrow: certified → retest_pending (dimension=certification_evidence, trigger=stale_certification) — Certification aged — badges shown retest-pending, not current
- **a11y:bundle-detail-page** (bundle_detail_page) — family=bundle_detail_page keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=certified effective_claim=policy_blocked status=narrowed_disclosed
  - Auto-narrow: certified → policy_blocked (dimension=dependency_posture, trigger=entitlement_dependency_unmet) — Entitlement dependency unmet — bundle blocked by policy
- **a11y:bundle-install-update-review-sheet** (bundle_install_update_review_sheet) — family=bundle_install_update_review_sheet keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=supported effective_claim=supported status=parity
- **a11y:bundle-drift-banner** (bundle_drift_banner) — family=bundle_drift_banner keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=certified effective_claim=limited status=narrowed_disclosed
  - Auto-narrow: certified → limited (dimension=bundle_freshness, trigger=local_override_drift) — Local overrides diverged — bundle support limited to unchanged scope
- **a11y:bundle-local-override-row** (bundle_local_override_row) — family=bundle_local_override_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=certified effective_claim=mirror_only status=narrowed_disclosed
  - Auto-narrow: certified → mirror_only (dimension=artifact_availability, trigger=mirror_stale) — Override asset served from a stale mirror — mirror-only
- **a11y:bundle-rollback-remove-card** (bundle_rollback_remove_card) — family=bundle_rollback_remove_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=certified effective_claim=offline_cache_only status=narrowed_disclosed
  - Auto-narrow: certified → offline_cache_only (dimension=artifact_availability, trigger=offline_cache_only) — Rollback checkpoint reachable from offline cache only
- **a11y:bundle-class-disclosure-card** (bundle_class_disclosure_card) — family=bundle_class_disclosure_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=certified effective_claim=imported status=narrowed_disclosed
  - Auto-narrow: certified → imported (dimension=source_provenance, trigger=imported_not_native) — Imported user handoff — not a native first-party bundle
- **a11y:bundle-claim-narrowing-row** (bundle_claim_narrowing_row) — family=bundle_claim_narrowing_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=certified effective_claim=retest_pending status=narrowed_disclosed
  - Auto-narrow: certified → retest_pending (dimension=certification_evidence, trigger=stale_certification) — Certification stale — claim narrowed to retest-pending
