# M5 Deployment/Continuity Component Accessibility & Auto-Narrowing

- Packet: `m5-deployment-continuity-accessibility-fallback:stable:0001`
- As of: `2026-07-04T00:00:00Z`
- Families: 9 certified across 9 / 9 frozen families
- Status: 2 green / 7 yellow / 0 red

## Rows

- **a11y:install-profile-card** (install_profile_card) — family=install_profile_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=fully_current_managed effective_claim=fully_current_managed status=parity
- **a11y:rollout-ring-row** (rollout_ring_row) — family=rollout_ring_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=fully_current_managed effective_claim=review_required status=narrowed_disclosed
  - Auto-narrow: fully_current_managed → review_required (dimension=rollout_state, trigger=rollout_paused) — Rollout ring held — promotion gated behind explicit review
- **a11y:deployment-summary-card** (deployment_summary_card) — family=deployment_summary_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=fully_current_managed effective_claim=local_cached_only status=narrowed_disclosed
  - Auto-narrow: fully_current_managed → local_cached_only (dimension=control_plane_freshness, trigger=control_plane_impaired) — Control plane cached — deployment shown local-cached-only, not live-current
- **a11y:residual-dependency-row** (residual_dependency_row) — family=residual_dependency_row keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=fully_current_managed effective_claim=review_required status=narrowed_disclosed
  - Auto-narrow: fully_current_managed → review_required (dimension=residual_dependency, trigger=residual_vendor_dependency) — Residual vendor dependency remains — self-hosted claim gated behind review
- **a11y:control-data-plane-status-strip** (control_plane_data_plane_status_strip) — family=control_plane_data_plane_status_strip keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=fully_current_managed effective_claim=inspect_only status=narrowed_disclosed
  - Auto-narrow: fully_current_managed → inspect_only (dimension=control_plane_freshness, trigger=control_plane_impaired) — Control plane unreachable — status inspect-only; local runtime unaffected
- **a11y:mirror-offline-artifact-row** (mirror_offline_artifact_row) — family=mirror_offline_artifact_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=fully_current_managed effective_claim=local_cached_only status=narrowed_disclosed
  - Auto-narrow: fully_current_managed → local_cached_only (dimension=mirror_verification, trigger=mirror_stale) — Mirror stale — artifact shown mirror-cached, never as a current live source
- **a11y:mode-change-review-sheet** (mode_change_review_sheet) — family=mode_change_review_sheet keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=review_required effective_claim=review_required status=parity
- **a11y:side-by-side-import-sheet** (side_by_side_import_sheet) — family=side_by_side_import_sheet keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=fully_current_managed effective_claim=review_required status=narrowed_disclosed
  - Auto-narrow: fully_current_managed → review_required (dimension=handler_ownership, trigger=handler_ownership_contested) — Handler ownership contested — import gated behind review, no default capture
- **a11y:channel-association-review-row** (channel_association_review_row) — family=channel_association_review_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=fully_current_managed effective_claim=inspect_only status=narrowed_disclosed
  - Auto-narrow: fully_current_managed → inspect_only (dimension=handler_ownership, trigger=handler_ownership_contested) — Handler pinned by policy — association inspect-only, current owner shown
