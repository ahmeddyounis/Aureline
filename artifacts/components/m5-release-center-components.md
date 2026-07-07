# M5 Release-Candidate-Card, Version-Bump-Row, Publish-Target-Row, Artifact-Provenance-Bundle-Card, and Promotion-Timeline Component Matrix

- Packet: `m5-release-center-components:stable:0001`
- Label: `M5 release-candidate-card, version-bump-row, publish-target-row, artifact-provenance-bundle-card, and promotion-timeline component matrix`
- Component families: 6 (6 stable)
- Target auth sources: ci_federated_identity, maintainer_key, org_managed_identity, hardware_token_signer, delegated_bot_identity, unauthenticated_mirror
- Rollback blast radii: single_artifact, family_scoped, train_scoped, cross_train_scoped, fleet_wide
- Proof freshness SLO: 720 hours (last refresh: 2026-07-06T00:00:00Z)

## Component families

- **release_candidate_card**: `stable`
  - Owner: Release-candidate component owner
  - Scope: One release-candidate-card model carrying candidate scope — single family, multi family, full train, hotfix, backport line, or preview channel — and the current blocker state with its freshness, so a candidate is never shown as clear while a hard blocker or a stale evaluation is open
  - Required labels: identity, state, keyboard_route, evidence_freshness
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **version_bump_row**: `stable`
  - Owner: Version-bump component owner
  - Scope: One version-bump-row model naming the proposed bump class — major, minor, patch, prerelease, build-metadata-only, or republish — and its compatibility impact, so a breaking change is never hidden behind a version number
  - Required labels: identity, state, keyboard_route, evidence_freshness
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **publish_target_row**: `stable`
  - Owner: Publish-target component owner
  - Scope: One publish-target-row model naming the target's visibility, its mutability, the identity authorized to publish to it, and whether a dry-run preview is available, so a mutable target, an unauthenticated mirror, or a no-dry-run target is never presented as a clean, safe publish
  - Required labels: identity, state, keyboard_route, auth_source, evidence_freshness
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **artifact_provenance_bundle_card**: `stable`
  - Owner: Provenance/attestation component owner
  - Scope: One artifact-provenance-bundle-card model carrying signature, attestation, and SBOM status over an immutable digest lineage, so an unsigned, unattested, partial-SBOM, or broken-lineage bundle is never shown as verified
  - Required labels: identity, state, keyboard_route, evidence_freshness
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **promotion_timeline_step**: `stable`
  - Owner: Promotion-timeline component owner
  - Scope: One promotion-timeline-step model naming its rollout ring — canary, pilot, early access, broad, general availability, or held — and its stage state, so a blocked or rolled-back stage is never shown as promoted and the current ring is always explicit
  - Required labels: identity, state, keyboard_route, evidence_freshness
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **rollback_revocation_row**: `stable`
  - Owner: Rollback/revocation component owner
  - Scope: One rollback-revocation-row model naming a rollback's blast radius — single artifact through fleet-wide — and its revocation scope, so a fleet-wide rollback or a key/trust-root rotation is never understated as a soft, single-artifact undo
  - Required labels: identity, state, keyboard_route, rollback_vocabulary, evidence_freshness
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
