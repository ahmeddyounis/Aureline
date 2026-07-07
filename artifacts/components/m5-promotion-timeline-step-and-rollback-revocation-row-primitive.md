# M5 Promotion-Timeline-Step and Rollback/Revocation-Row Primitive

- Packet: `m5-promotion-timeline-and-rollback-revocation-primitive:stable:0001`
- Label: `M5 promotion-timeline-step and rollback/revocation-row primitive: event identity, event kind, source and destination stage, immutable-digest joins, evidence refs, approving actors, effective time, reversible window, affected node set, blast radius, node targeting, last-known-good target, continuity note, revocation scope, and break-glass attribution`
- History consumers: 5 (5 stable)
- History postures: promotion_recorded_reversible, promotion_recorded_irreversible, promotion_in_progress, promotion_blocked, rollback_recorded_bounded, revocation_recorded, emergency_break_glass_recorded, history_blocked_unattributed, history_blocked_missing_last_known_good, history_blocked_missing_digest_join
- Blast radii: single_artifact, family_scoped, train_scoped, cross_train_scoped, fleet_wide
- Revocation scopes: no_revocation, tag_repoint_only, artifact_revoked, signing_key_revoked, trust_root_rotated
- Break-glass postures: standard_change_control, break_glass_attributed, break_glass_pending_review, break_glass_unattributed
- Proof freshness SLO: 720 hours (last refresh: 2026-07-06T00:00:00Z)

## History consumers

- **Release-Center Promotion Timeline**: `stable`
  - Owner: Release-center promotion-timeline owner
  - Scope: The release-center promotion timeline renders the shared history primitive so a promotion step from the canary ring to the pilot ring that was reversed inside its reversible window reads as promotion-recorded-reversible — reconstructable from its approving actors, immutable-digest joins, and evidence refs — while a promotion step whose stage is blocked reads as promotion-blocked with a self-contained banner naming the reason, the bound digest, its actors, and the resolve-stage-blocker next action
  - Worked resolutions: 2
    - `event:promote-core-runtime 5.2.0 canary->pilot` (promotion_step, digests 2) → `promotion_recorded_reversible` (banner `recorded`)
    - `event:promote-registry 5.2.0 pilot->broad` (promotion_step, digests 1) → `promotion_blocked` (banner `stage_promotion_blocked`)
- **Update-Center Release History**: `stable`
  - Owner: Update-center release-history owner
  - Scope: The update-center release history renders the shared primitive so a promotion step whose reversible window has expired reads as promotion-recorded-irreversible — honest that it can no longer be reversed — while a bounded rollback that repoints a mutable tag over an explicitly enumerated partial node set within a family-scoped blast radius, restoring a named last-known-good, reads as rollback-recorded-bounded rather than a generic status change
  - Worked resolutions: 2
    - `event:promote-shell 5.2.0 pilot->broad` (promotion_step, digests 1) → `promotion_recorded_irreversible` (banner `recorded`)
    - `event:rollback-update 5.2.0->5.1.9 family` (rollback_revocation_row, digests 1) → `rollback_recorded_bounded` (banner `recorded`)
- **CLI History Inspect**: `stable`
  - Owner: CLI history-inspect owner
  - Scope: The CLI history-inspect surface renders the shared primitive so a promotion step still moving through the early-access ring reads as promotion-in-progress, while a trust-root rotation attempted with no immutable-digest join reads as history-blocked-missing-digest-join with a record-immutable-digest-join next action — artifact-graph consistency requires a digest join before the event can be recorded
  - Worked resolutions: 2
    - `event:promote-graph 5.3.0 early-access` (promotion_step, digests 1) → `promotion_in_progress` (banner `recorded`)
    - `event:rotate-trust-root fleet` (rollback_revocation_row, digests 0) → `history_blocked_missing_digest_join` (banner `missing_immutable_digest_join`)
- **Admin History Report**: `stable`
  - Owner: Admin history-report owner
  - Scope: The admin history report renders the shared primitive so a signing-key revocation with an explicit train-scoped blast radius over the whole affected node set, restoring a named last-known-good, reads as revocation-recorded, while an emergency artifact revocation carrying no attributed actor reads as history-blocked-unattributed with an attribute-emergency-actor next action — break-glass must stay attributable and never disappear into CI-only metadata
  - Worked resolutions: 2
    - `event:revoke-signing-key train` (rollback_revocation_row, digests 1) → `revocation_recorded` (banner `recorded`)
    - `event:emergency-revoke-artifact cross-train` (rollback_revocation_row, digests 1) → `history_blocked_unattributed` (banner `emergency_action_unattributed`)
- **Support History Export**: `stable`
  - Owner: Support history-export owner
  - Scope: The support history export renders the shared primitive so an emergency break-glass promotion attributed to a named actor reads as emergency-break-glass-recorded and stays visible in the same history model, a rollback that names no last-known-good target reads as history-blocked-missing-last-known-good with a record-last-known-good-target next action, and an emergency break-glass artifact revocation with review pending stays attributed and visible — the same history vocabulary a support or evaluation reviewer reads across every surface
  - Worked resolutions: 3
    - `event:emergency-promote-hotfix ga` (promotion_step, digests 1) → `emergency_break_glass_recorded` (banner `recorded`)
    - `event:rollback-no-lkg single` (rollback_revocation_row, digests 1) → `history_blocked_missing_last_known_good` (banner `missing_last_known_good_target`)
    - `event:emergency-revoke-artifact pending-review family` (rollback_revocation_row, digests 1) → `emergency_break_glass_recorded` (banner `recorded`)
