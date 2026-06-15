# Fixtures: commercial-boundary card set

This directory carries the fixture metadata for the frozen Help/About,
release-center, diagnostics, and procurement commercial-boundary card set.

The canonical set is checked in at:

`artifacts/service/m5-commercial-boundary-cards.json`

Its boundary schema is:

`schemas/service/m5-commercial-boundary-cards.schema.json`

## Coverage

- The set freezes exactly one local-open-core card plus one card per managed
  service family — the AI gateway, settings sync, the companion relay, the
  registry/mirror surface, support ingest, and the managed workspace — a 7-card
  set.
- Each card states its open-versus-paid boundary class, the deployment profiles
  its boundary holds in (and the profiles it is not offered in), the residual
  vendor-hosted dependencies a managed lane carries, the procurement/support
  evidence available at an export-parity guarantee, and a non-empty local-safe
  baseline.
- Six surface bindings — Help/About, release center, diagnostics, the
  procurement packet, the support/admin packet, and claim/public-truth
  automation — each resolve through real card ids. Procurement and support reuse
  the same evidence object.

## What the corpus proves

- **The local core is never blocked.** Every card — local or managed — keeps a
  non-empty `local_safe_baseline`, so a stale or unreachable metering/rating path
  narrows only an optional managed action, never local editing, search, Git, or
  already-authorized local automation.
- **Open-versus-paid is honest.** The local-open-core card makes only the
  `local_safe_only` claim, binds no managed service family, and declares no
  residual vendor dependency; every managed card declares the `managed_full`
  claim and discloses at least one residual vendor-hosted dependency.
- **No open boundary is overstated.** Every residual dependency names whether it
  `remains_vendor_hosted` and whether it is `eliminated_under_self_host`, and
  every card names the deployment profiles its boundary `holds_in_profiles`. The
  AI gateway, relay, support ingest, and managed workspace are honestly *not
  offered* air-gapped or individual-local; the registry/mirror is offered
  everywhere via a signed mirror; the open core holds in every profile.
- **Procurement and support reuse one object model.** Both the procurement-packet
  and support/admin bindings project the same `ProcurementSupportEvidence` object
  — open-source license manifest, residual-dependency disclosure, usage/forecast
  and chargeback exports, entitlement summary, and support bundles — at the same
  export guarantee.
- **Upsell never outranks truth.** Each card's ranked actions put
  `export_evidence` (rank 1), `continue_local` (rank 2), and
  `view_procurement_packet` (rank 3) above any `learn_about_paid` prompt (rank 6);
  the local open core carries no upsell.
- **No number crosses the boundary bare.** Every card defers spend/quota figures
  to the metering surfaces (`cost_figure_disclosure: deferred_to_metering_surfaces`)
  and carries an `as_of` time.
- **The marketed claim narrows automatically.** A managed card's effective claim
  is the declared claim capped by its `evidence_status`: `managed_full` when
  current, `managed_narrowed` when stale, `local_safe_only` when missing or
  downgraded. The local open core never narrows.
- **Cards project the control plane.** `cross_check_against_control_plane`
  confirms each managed card agrees with the control-plane lane for its service
  family on the declared claim, the export guarantee, and a non-empty local-safe
  baseline.

## Regeneration

The set is built and validated by `canonical_commercial_boundary_card_set`, which
recomputes every card's derived fields and the inspection block; any drift between
a stored value and the recomputation is a test failure in
`crates/aureline-service/src/m5_commercial_boundary_cards/tests.rs`. Regenerate the
checked-in artifact deterministically with:

```text
cargo run -p aureline-service --example dump_m5_commercial_boundary_cards -- canonical \
  > artifacts/service/m5-commercial-boundary-cards.json
```
