# M5 Version-Bump Row and Publish-Target Review-Sheet Primitive

- Packet: `m5-publish-target-review-sheet-primitive:stable:0001`
- Label: `M5 version-bump row and publish-target review-sheet primitive: prior/next version, delta kind, public-surface impact, publish-target class, visibility, mutability, auth source, dry-run availability, and rollout ring`
- Publication consumers: 5 (5 stable)
- Readiness postures: publishable, publishable_with_review, publishable_dry_run_first, narrowed_surface_review_pending, narrowed_reversibility_unproven, blocked_ambient_credential, blocked_surface_impact_stale, blocked_surface_impact_missing, blocked_unknown_state
- Public-surface impacts: no_public_surface_change, additive_public_surface, breaking_public_surface, runtime_behavior_shift, migration_required_public_surface
- Block reasons: ambient_credential_inheritance, surface_impact_stale, surface_impact_missing, review_state_unknown, surface_review_pending, destination_reversibility_unproven
- Destination reversibilities: dry_run_proven, immutable_by_design, reversibility_unproven
- Proof freshness SLO: 720 hours (last refresh: 2026-07-06T00:00:00Z)

## Publication consumers

- **Release-Center Publish Sheet**: `stable`
  - Owner: Release-center publish-sheet owner
  - Scope: The release-center publish sheet renders the shared version-bump / publish-target primitive so a minor additive bump to a public registry with a scoped CI identity and a supported dry-run reads as publishable, while a major breaking bump to a managed control plane that would inherit ambient credentials reads as blocked with a self-contained banner naming the reason and the disclose-auth-source next action
  - Worked resolutions: 2
    - `5.1.4` → `5.2.0` on `registry_target` → `publishable` (additive_public_surface impact, dry_run_proven reversibility, banner `clear`)
    - `5.1.4` → `6.0.0` on `managed_control_plane_target` → `blocked_ambient_credential` (breaking_public_surface impact, reversibility_unproven reversibility, banner `ambient_credential_inheritance`)
- **Update-Center Publish Row**: `stable`
  - Owner: Update-center publish-row owner
  - Scope: The update-center publish row renders the shared primitive so a patch runtime-behaviour bump to a mutable channel pointer whose public-surface analysis has gone stale reads as blocked-surface-impact-stale with a refresh next action, while a prerelease forward-incompatible bump to a mirror missing its surface analysis reads as blocked-surface-impact-missing with a provide next action
  - Worked resolutions: 2
    - `5.1.4` → `5.1.5` on `channel_pointer_target` → `blocked_surface_impact_stale` (runtime_behavior_shift impact, dry_run_proven reversibility, banner `surface_impact_stale`)
    - `5.1.4` → `5.2.0-rc.1` on `mirror_target` → `blocked_surface_impact_missing` (breaking_public_surface impact, dry_run_proven reversibility, banner `surface_impact_missing`)
- **CLI Publish Inspect**: `stable`
  - Owner: CLI publish-inspect owner
  - Scope: The CLI publish-inspect surface renders the shared primitive so a build-metadata-only publish to an immutable local store whose review state has not been evaluated reads as blocked-unknown-state with a run-review next action and an immutable-by-design reversibility, while a minor additive bump to a registry whose surface review is pending sign-off reads as narrowed-surface-review-pending with a complete-review next action
  - Worked resolutions: 2
    - `5.3.0` → `5.3.0+build.7` on `local_artifact_store_target` → `blocked_unknown_state` (no_public_surface_change impact, immutable_by_design reversibility, banner `review_state_unknown`)
    - `5.0.8` → `5.0.9` on `registry_target` → `narrowed_surface_review_pending` (additive_public_surface impact, dry_run_proven reversibility, banner `surface_review_pending`)
- **Admin Publish Report**: `stable`
  - Owner: Admin publish-report owner
  - Scope: The admin publish report renders the shared primitive so a patch additive bump to a mutable channel pointer with no dry-run reads as narrowed-reversibility-unproven with an enable-dry-run next action rather than reading like an immutable step, while a major schema-migration bump to a managed control plane held under a disclosed waiver with a required dry-run reads as publishable-dry-run-first
  - Worked resolutions: 2
    - `4.9.6` → `4.9.7` on `channel_pointer_target` → `narrowed_reversibility_unproven` (additive_public_surface impact, reversibility_unproven reversibility, banner `destination_reversibility_unproven`)
    - `5.9.9` → `6.0.0` on `managed_control_plane_target` → `publishable_dry_run_first` (migration_required_public_surface impact, dry_run_proven reversibility, banner `clear`)
- **Support / Evaluation Export**: `stable`
  - Owner: Support / evaluation export owner
  - Scope: The support / evaluation export renders the shared primitive so a minor additive bump whose surface analysis is aging reads as publishable-with-review rather than clean or blocked, and a republish with no version change to a mirror published by a broadly-scoped delegated bot reads as publishable-with-review — the same version-bump / publish-target vocabulary a support or evaluation reviewer reads elsewhere
  - Worked resolutions: 2
    - `5.1.4` → `5.2.0-rc.3` on `registry_target` → `publishable_with_review` (additive_public_surface impact, dry_run_proven reversibility, banner `clear`)
    - `5.1.4` → `5.1.4` on `mirror_target` → `publishable_with_review` (no_public_surface_change impact, dry_run_proven reversibility, banner `clear`)
