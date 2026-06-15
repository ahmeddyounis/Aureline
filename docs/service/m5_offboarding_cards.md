# Offboarding cards

The offboarding-card set is the canonical, inspectable description of the humane
offboarding surface a user or admin sees when a managed entitlement is winding
down. Where the commercial-control-plane matrix
(`docs/service/m5_commercial_control_plane.md`) freezes the per-lane fail posture
and the managed-state vocabulary, and the metering-degradation rules
(`docs/service/m5_metering_degradation_rules.md`) freeze the runtime
metering-degradation behavior, this set freezes the offboarding cards: what the
event is, when it takes effect, which managed features pause, what can still be
exported, what keeps running locally, when managed copies are deleted, and who
owns the next step. It is owned by the `aureline-service` crate
(`crates/aureline-service/src/m5_offboarding_cards/`), checked in at
`artifacts/service/m5-offboarding-cards.json`, and bounded by
`schemas/service/m5-offboarding-cards.schema.json`.

## What it freezes

- **One card per lifecycle event.** A grace period, a seat loss, a cancellation,
  and an org switch each carry a card. Each card names the event type, the
  effective date, the impacted managed features and service families, the export
  rights and export guarantee, the non-empty local-safe continuation, the
  deletion timeline, the owner/contact handoff, and the artifact separation that
  keeps local artifacts distinct from tenant-scoped managed state.
- **One binding per consumer surface.** The account/offboarding surface,
  diagnostics, Help/About, the support/admin packet, and claim/public-truth
  automation each resolve through the cards rather than retyping their state,
  projecting the effective claim, rendering the local-safe continuation, naming
  the owner handoff, and keeping export above any upgrade or renewal prompt.

## Invariants

- The local core is never blocked or deleted: every card keeps a non-empty
  local-safe continuation, and every deletion timeline sets
  `local_artifacts_deleted` to `false`, so a stale metering or rating path and a
  wind-down both leave local editing, search, Git, and already-authorized local
  automation running.
- Local and tenant-scoped state stay separated: every card names what stays on
  device and user-owned apart from what is tenant-scoped and rebinds or is
  reclaimed, so a seat loss or an org switch never blurs the two.
- Export and local continuation are never buried: a card's actions are ranked, and
  no upgrade/renewal action may rank above an export or continue-local action.
- The four events stay distinct: every card lists the other three events and
  asserts it is not a sign-in/reauth failure or a generic account error, so a
  grace window, a seat loss, a cancellation, and an org switch never collapse into
  one error.
- No number crosses the boundary bare: any final usage figure is bound to its
  unit, as-of time, and scope owner or suppressed entirely, and every card carries
  an as-of time.
- The marketed claim narrows automatically: every card declares the full managed
  claim and narrows the effective claim from the event's cap — `managed_narrowed`
  for grace and an org switch, `local_safe_only` for a seat loss and a
  cancellation. `cross_check_against_control_plane` confirms each mapped card
  agrees with its control-plane managed-state row.

## How to consume it

Call `current_stable_offboarding_card_set()` to read and validate the checked-in
set; call `CardSet::card_for(event)` to resolve a single card and
`CardSet::cross_check_against_control_plane()` to confirm each mapped card projects
its control-plane managed-state row. The reviewer contract is
`docs/m5/add-grace-period-seat-loss-cancellation-and-org-switch-offboarding-cards-with-export-rights-local-safe-continuity-deletion-timeline-and-owner-handoff.md`.
