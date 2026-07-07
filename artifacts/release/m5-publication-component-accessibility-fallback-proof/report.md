# M5 Publication-Component Accessibility & Auto-Narrowing

- Packet: `m5-publication-component-accessibility-fallback:stable:0001`
- As of: `2026-07-06T00:00:00Z`
- Families: 6 certified across 6 / 6 frozen families
- Status: 2 green / 4 yellow / 0 red

## Rows

- **a11y:release-candidate-card** (release_candidate_card) — family=release_candidate_card keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=certified effective_claim=certified status=parity
- **a11y:version-bump-row** (version_bump_row) — family=version_bump_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=supported effective_claim=supported status=parity
- **a11y:publish-target-row** (publish_target_row) — family=publish_target_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=certified effective_claim=degraded status=narrowed_disclosed
  - Auto-narrow: certified → degraded (dimension=target_auth_posture, trigger=target_auth_source_masked) — Target auth partially resolved — publish shown degraded until the scoped credential wins over the ambient one
- **a11y:artifact-provenance-bundle-card** (artifact_provenance_bundle_card) — family=artifact_provenance_bundle_card keyboard=reachable_and_labeled screen_reader=disclosed_reduced_but_reachable cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=certified effective_claim=unverified status=narrowed_disclosed
  - Auto-narrow: certified → unverified (dimension=signature_attestation_state, trigger=signature_or_attestation_overclaimed) — Signature and attestation unverified on this build — provenance shown from unproven material, not certified
- **a11y:promotion-timeline-step** (promotion_timeline_step) — family=promotion_timeline_step keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=certified effective_claim=provisional status=narrowed_disclosed
  - Auto-narrow: certified → provisional (dimension=mirror_verification, trigger=proof_stale) — Mirror verification stale — promotion shown from last-known mirror state until re-verification lands
- **a11y:rollback-revocation-row** (rollback_revocation_row) — family=rollback_revocation_row keyboard=reachable_and_labeled screen_reader=reachable_and_labeled cli=reachable_and_labeled export=reconstructable_without_screenshot full_claim=supported effective_claim=policy_blocked status=narrowed_disclosed
  - Auto-narrow: supported → policy_blocked (dimension=rollback_blast_radius, trigger=rollback_blast_radius_understated) — Rollback blocked by policy — blast radius not executable until an approver signs off
