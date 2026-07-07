# M5 promotion-timeline-step / rollback-or-revocation-row primitive contract

This contract governs the reusable M5 **promotion-timeline step** and its
**rollback-or-revocation row**: one resolver plus a parity matrix that let a user or a
support team reconstruct — from the timeline itself — exactly **what changed and why**,
so channel movement and emergency actions stay **attributable, bounded, and
reconstructable** from the same object model used by human review and automation.

- Rust module: `crates/aureline-release/src/implement_promotion_timeline_steps_and_rollback_or_revocation_rows_across_claimed_m5_release_histories`
- Boundary schemas: `schemas/ui/m5-promotion-timeline-step.schema.json`, `schemas/ui/m5-rollback-revocation-row.schema.json`
- Frozen component matrix this narrows from: `schemas/ui/m5-release-center-components.schema.json`
- Support export (canonical): `artifacts/release/m5-promotion-timeline-and-rollback-revocation-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-promotion-timeline-and-rollback-revocation-proof/matrix.csv`
- Markdown report: `artifacts/components/m5-promotion-timeline-step-and-rollback-revocation-row-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-promotion-timeline-and-rollback-revocation-primitive/`

The Rust validator (`M5ReleaseHistoryPrimitivePacket::validate`) is the authoritative
gate; the schema documents the shape. The headless emitter bin is the only
mint-from-truth path for the checked-in artifacts and fixtures.

## The two halves

1. **Resolver** — `resolve_release_history_event(&M5ReleaseHistoryEventInput)` derives one
   `M5ResolvedReleaseHistoryEvent` carrying:
   - the **history posture** (`M5ReleaseHistoryPosture`, 10 states), derived in a fixed
     blocking-first order from the event kind, the immutable-digest joins, the break-glass
     attribution, the last-known-good target, the promotion stage state, and the reversible
     window;
   - the **promotion-step view** (`M5PromotionStepView`) — source stage, destination stage,
     stage state, rollout ring, and reversible window — for a promotion step, or the
     **rollback/revocation view** (`M5RollbackRevocationView`) — affected node set, blast
     radius, node targeting, last-known-good target, continuity note, and revocation scope —
     for a rollback / revocation row, so a rollback never reads like a generic status change;
   - the **reconstruction readiness** (`M5ReleaseHistoryReconstruction`), true when the
     timeline alone carries the actors, digest joins, evidence refs, and time needed to
     reconstruct what changed and why;
   - a self-contained **`M5ReleaseHistoryBanner`** whenever the event is blocked, naming the
     exact reason, the bound event, its digest join, its actors, and the next action — never
     a generic `history unavailable`.

2. **Parity matrix** — `M5ReleaseHistoryPrimitivePacket` binds one row per claimed M5
   release-history consumer to the shared step/row anatomy, vocabulary, export fields, and
   non-visual accessibility routes, plus worked resolution cases that must reproduce the
   resolver output exactly.

## History ladder (blocking-first)

1. `history_blocked_missing_digest_join` — the event carries no immutable-digest join
   (artifact-graph consistency requires one before it can be recorded).
2. `history_blocked_unattributed` — an emergency break-glass action carries no attributed
   actor. Break-glass must stay attributable.
3. `history_blocked_missing_last_known_good` — a rollback / revocation names no
   last-known-good target.
4. `emergency_break_glass_recorded` — an attributed emergency action, recorded and kept
   visible in the same history model rather than disappearing into CI-only metadata.
5. Promotion: `promotion_blocked` (stage blocked), `promotion_in_progress` (pending or in
   progress), `promotion_recorded_reversible` (promoted inside an open reversible window or
   rolled back), `promotion_recorded_irreversible` (window expired or irreversible by
   design — honest about it).
6. Rollback: `revocation_recorded` (trust material — artifact, signing key, or trust root —
   revoked) or `rollback_recorded_bounded` (a soft rollback or tag repoint).

## Claimed consumers (`M5ReleaseHistoryConsumerSurface`)

- `release_center_timeline`
- `update_center_history`
- `cli_history_inspect`
- `admin_history_report`
- `support_history_export`

## Hard invariants

Per row (all must be `false`):

- `reads_rollback_as_generic_status`
- `drops_break_glass_attribution`
- `hides_blast_radius_or_unaffected_nodes`
- `lets_emergency_disappear_into_ci_only_metadata`

## Coverage lints (acceptance criteria)

- `history_coverage_unproven` — the matrix must prove a recorded promotion, a recorded
  rollback / revocation, and a blocked event.
- `rollback_not_generic_unproven` — the matrix must prove a rollback / revocation with a
  blast radius wider than a single artifact, an explicit partial node targeting, and a
  non-empty affected node set.
- `emergency_visible_in_history_unproven` — the matrix must prove an emergency break-glass
  action recorded and visible in the history model.
- `break_glass_attribution_unproven` — the matrix must both preserve an attributed
  break-glass and block an unattributed one.
- `reconstructable_from_timeline_unproven` — the matrix must prove an event reconstructable
  from the timeline alone.
- `reversible_window_unproven` — the matrix must prove both a reversible and an irreversible
  promotion.
- `blocked_banner_self_contained_unproven` — the matrix must prove a blocked event whose
  banner carries a reason, a next action, the bound event, and its digest.

## Export safety

Raw URLs, raw signing keys, raw tokens, credentials, private endpoints, and user text
bodies never cross this boundary. Every event id, digest join, actor, node ref, and
last-known-good target is carried only as an opaque, export-safe representation.
