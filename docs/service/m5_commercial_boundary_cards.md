# Commercial-boundary cards

The commercial-boundary-card set is the canonical, inspectable description of the
commercial-boundary surface a user, admin, or procurement reviewer sees on
Help/About, the release center, diagnostics, and in a procurement/support packet.
Where the commercial-control-plane matrix
(`docs/service/m5_commercial_control_plane.md`) freezes the per-lane fail posture
and the managed-state vocabulary, and the offboarding cards
(`docs/service/m5_offboarding_cards.md`) freeze the wind-down surface, this set
freezes the open-versus-paid boundary: which capabilities are local and
open-source, which are optional managed/paid lanes, what residual dependencies
remain vendor-hosted, which deployment profiles each boundary holds in, and what
procurement/support evidence is available. It is owned by the `aureline-service`
crate (`crates/aureline-service/src/m5_commercial_boundary_cards/`), checked in at
`artifacts/service/m5-commercial-boundary-cards.json`, and bounded by
`schemas/service/m5-commercial-boundary-cards.schema.json`.

## What it freezes

- **One local-open-core card plus one card per managed service family.** The local
  open core declares the `local_safe_only` claim with no residual dependency; each
  managed lane — the AI gateway, settings sync, the companion relay, the
  registry/mirror surface, support ingest, and the managed workspace — declares the
  `managed_full` claim, discloses its residual vendor-hosted dependencies, names the
  deployment profiles its boundary holds in, links its procurement/support
  evidence at an export-parity guarantee, and keeps a non-empty local-safe
  baseline.
- **One binding per surface.** Help/About, the release center, diagnostics, the
  procurement packet, the support/admin packet, and claim/public-truth automation
  each resolve through the cards rather than retyping their state, projecting the
  effective claim, rendering the local-safe baseline, disclosing the residual
  dependencies, naming the deployment-profile qualifier, and keeping evidence above
  any upsell. Procurement and support bind the same evidence object.

## Invariants

- The local core is never blocked: every card keeps a non-empty
  `local_safe_baseline`, so a stale or unreachable metering/rating path narrows
  only an optional managed action, never local editing, search, Git, or
  already-authorized local automation.
- Open-versus-paid is honest: the local-open-core card makes only the local-safe
  claim and declares no residual vendor dependency; every managed card declares the
  full managed claim and discloses at least one residual vendor-hosted dependency.
- No open or self-hosted boundary is overstated: every residual dependency names
  whether it `remains_vendor_hosted` and whether it is `eliminated_under_self_host`,
  and every card names the deployment profiles its boundary `holds_in_profiles`, so
  a lane that is unavailable air-gapped or individual-local says so.
- Procurement and support reuse one object model: both surfaces bind the same
  `ProcurementSupportEvidence` object at the same export guarantee.
- Commercial prompts never outrank truth: a card's actions are ranked, and no
  `learn_about_paid` prompt may rank above an `export_evidence`,
  `view_procurement_packet`, or `continue_local` action; only managed cards upsell.
- No number crosses the boundary bare: every card defers spend/quota figures to the
  metering surfaces and carries an `as_of` time.
- The marketed claim narrows automatically: a managed card's effective claim is the
  declared claim capped by its `evidence_status` — `managed_full` when current,
  `managed_narrowed` when stale, `local_safe_only` when missing or downgraded.
  `cross_check_against_control_plane` confirms each managed card agrees with its
  control-plane lane on the declared claim, export guarantee, and local-safe
  baseline.

## How to consume it

Call `current_stable_commercial_boundary_card_set()` to read and validate the
checked-in set; call `BoundaryCardSet::card_for_family(family)` or
`BoundaryCardSet::local_open_card()` to resolve a single card,
`BoundaryCardSet::apply_evidence_status(status)` to exercise the marketed-claim
narrowing deterministically, and
`BoundaryCardSet::cross_check_against_control_plane()` to confirm each managed card
projects its control-plane lane. The reviewer contract is
`docs/m5/ship-help-about-release-center-diagnostics-commercial-boundary-cards-with-open-versus-paid-truth-residual-dependency-disclosure-and-procurement-support-packet-parity.md`.
