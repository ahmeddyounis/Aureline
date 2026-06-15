# Grace-period, seat-loss, cancellation, and org-switch offboarding cards with export rights, local-safe continuity, deletion timeline, and owner handoff

Reviewer contract for the canonical offboarding-card set: the humane offboarding
surface a user or admin sees when a managed entitlement is winding down. One card
per lifecycle event — a grace period, a seat loss, a cancellation, and an org
switch — states what remains, what pauses, what can still be exported, when
deletion happens, and who owns the next step. This row is a depth-lane proof
governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/service/m5-offboarding-cards.json`
- Boundary schema: `schemas/service/m5-offboarding-cards.schema.json`
- Human-readable rendering: `artifacts/m5/add-grace-period-seat-loss-cancellation-and-org-switch-offboarding-cards-with-export-rights-local-safe-continuity-deletion-timeline-and-owner-handoff.md`
- Overview companion: `docs/service/m5_offboarding_cards.md`
- Fixture corpus: `fixtures/service/m5-offboarding-cards/`
- Owning crate module: `crates/aureline-service/src/m5_offboarding_cards/`

## Projects the frozen control-plane matrix

Each card reuses the closed vocabularies already frozen by the
commercial-control-plane matrix (`docs/service/m5_commercial_control_plane.md`) —
the service-family, managed-state, entitlement-state, posture-origin,
grace-period-right, export-guarantee, marketed-claim, and consumer-surface classes
— rather than minting a parallel synonym set. A card that maps to a frozen managed
state (`grace_period`, `seat_removed`, `org_switched`) is cross-checked by
`CardSet::cross_check_against_control_plane`, which confirms the card's linked
entitlement state, posture origin, and effective claim match the control-plane row.
The new tokens are only the offboarding vocabulary the matrix did not carry: the
lifecycle event, the card-action kind, the handoff owner and contact channel, and
the final-usage disclosure.

## The cards

One card per lifecycle event — an exhaustive 4-card set:

- **Grace period** — `entitlement_in_grace`, posture origin `account`, narrows to
  `managed_narrowed`. Managed actions stay admissible per the window; export is
  admissible before suspension; a renewal prompt is offered but ranked last.
- **Seat loss** — `entitlement_suspended_admin`, posture origin `seat`, narrows to
  `local_safe_only`. Managed actions for the seat stop; the seat administrator
  owns the next step; the seat's tenant-scoped state is reclaimed apart from the
  user's local artifacts.
- **Cancellation** — `entitlement_expired`, posture origin `plan`, narrows to
  `local_safe_only`. Managed copies are scheduled for deletion after the export
  window; export is admissible before the deadline; a renewal prompt is offered
  but ranked last. Cancellation maps to no single managed-state token.
- **Org switch** — `entitlement_pending_recheck`, posture origin `org`, narrows to
  `managed_narrowed`. Managed scope rebinds to the new org; the prior org's
  tenant-scoped state stays with that tenant; nothing is deleted by the switch.

## What the set proves

- **Local-core productivity is never blocked, and local data is never deleted.**
  Every card keeps a non-empty `local_safe_continuation`, and every deletion
  timeline sets `local_artifacts_deleted` to `false`, so a wind-down never deletes
  or pauses local editing, search, save, Git, or already-authorized local
  automation. Stale or unavailable metering never gates this surface.
- **Org switch and seat loss separate local from tenant-scoped state.** Every card
  carries an `artifact_separation` naming the local artifacts (on device,
  user-owned) apart from the tenant-scoped managed state (rebinds, is reclaimed,
  or is tenant-retained); the support/admin packet binds these two cards
  specifically.
- **Export and local continuation are never buried.** Each card's ranked actions
  put `export_now` and `continue_local` above any `upgrade_or_renew` prompt; the
  grace and cancellation cards carry a renewal prompt and still rank it last.
- **The four events never collapse into one account error.** Every card lists the
  other three events in `must_not_collapse_with` and sets
  `distinct_from_sign_in_failure` and `not_a_generic_account_error`, so a grace
  window, a seat loss, a cancellation, and an org switch stay distinct, and none
  is a sign-in failure.
- **No spend or quota number without unit, as-of time, and scope owner.** The
  grace and cancellation cards show a final figure `bound_to_unit_as_of_scope`;
  the seat-loss and org-switch cards `suppress` it. Every card carries an `as_of`
  time.
- **Deletion is on a stated timeline, not silent.** Every card's deletion timeline
  states the effective date and the export-before-suspend/delete deadline; the
  grace and cancellation cards also state when managed copies are deleted, while
  the seat-loss and org-switch cards leave managed deletion to tenant retention
  rather than inventing a date.
- **The marketed claim narrows automatically.** Every card declares `managed_full`
  and narrows the effective claim from the event's cap, so a marketed managed
  claim narrows when the entitlement winds down.
- **One packet, many surfaces.** The account/offboarding surface, diagnostics,
  Help/About, the support/admin packet, and claim/public-truth automation each
  bind to the set and project the effective claim — never a stronger one — render
  the local-safe continuation, name the owner handoff, and keep export above any
  upgrade prompt.

## Regeneration

`canonical_offboarding_card_set` builds the set;
`current_stable_offboarding_card_set` reads and validates the checked-in packet.
Drift between a stored value and the recomputation is a test failure in
`crates/aureline-service/src/m5_offboarding_cards/tests.rs`. Regenerate the
artifact with `cargo run -p aureline-service --example dump_m5_offboarding_cards -- canonical`.
