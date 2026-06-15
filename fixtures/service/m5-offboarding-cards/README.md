# Fixtures: offboarding card set

This directory carries the fixture metadata for the frozen grace-period,
seat-loss, cancellation, and org-switch offboarding card set.

The canonical set is checked in at:

`artifacts/service/m5-offboarding-cards.json`

Its boundary schema is:

`schemas/service/m5-offboarding-cards.schema.json`

## Coverage

- The set freezes exactly one card per lifecycle event — `grace_period`,
  `seat_loss`, `cancellation`, and `org_switch` — an exhaustive 4-card set.
- Each card states the event type, the effective date, the impacted managed
  features and service families, the export rights and export guarantee, the
  non-empty local-safe continuation, the deletion timeline (with the
  export-before-suspend/delete deadline), and the owner/contact handoff.
- Each card carries an artifact separation that names the local artifacts (on
  device, user-owned) apart from the tenant-scoped managed state (rebinds, is
  reclaimed, or is tenant-retained).
- Five surface bindings — account, diagnostics, Help/About, support/admin packet,
  and claim/public-truth automation — each resolve through real card ids.

## What the corpus proves

- **The local core is never blocked or deleted.** Every card keeps a non-empty
  `local_safe_continuation`, and every deletion timeline sets
  `local_artifacts_deleted` to `false`, so a wind-down never deletes or pauses
  local editing, search, Git, or existing local automation.
- **Local and tenant-scoped state stay separated.** Every card's
  `artifact_separation` names both sides; the seat-loss and org-switch cards in
  particular keep the local artifacts distinct from the tenant-scoped managed
  state rather than blurring them into one bucket.
- **Export and local continuation are never buried.** Each card's ranked actions
  put `export_now` (rank 1) and `continue_local` (rank 2) above any
  `upgrade_or_renew` prompt (rank 5); the grace and cancellation cards carry an
  upgrade prompt and still rank it last, while the seat-loss and org-switch cards
  carry none.
- **The four events stay distinct.** Every card lists the other three events in
  `must_not_collapse_with` and sets `distinct_from_sign_in_failure` and
  `not_a_generic_account_error`, so a grace window, a seat loss, a cancellation,
  and an org switch never collapse into one account error and none is a sign-in
  failure.
- **No number crosses the boundary bare.** The grace and cancellation cards show
  a final figure `bound_to_unit_as_of_scope`; the seat-loss and org-switch cards
  `suppress` it. Every card carries an `as_of` time.
- **The marketed claim narrows automatically.** Every card declares `managed_full`
  and narrows the effective claim from the lifecycle event's cap —
  `managed_narrowed` for grace and an org switch, `local_safe_only` for a seat
  loss and a cancellation.
- **Cards project the control plane.** `cross_check_against_control_plane`
  confirms each card that maps to a managed state (`grace_period`, `seat_removed`,
  `org_switched`) agrees with the control-plane row on the entitlement state,
  posture origin, and claim cap.

## Regeneration

The set is built and validated by `canonical_offboarding_card_set`, which
recomputes every card's derived fields and the inspection block; any drift between
a stored value and the recomputation is a test failure in
`crates/aureline-service/src/m5_offboarding_cards/tests.rs`. Regenerate the
checked-in artifact deterministically with:

```text
cargo run -p aureline-service --example dump_m5_offboarding_cards -- canonical \
  > artifacts/service/m5-offboarding-cards.json
```
