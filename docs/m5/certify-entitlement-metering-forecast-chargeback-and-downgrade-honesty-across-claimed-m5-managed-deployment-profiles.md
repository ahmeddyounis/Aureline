# Certify entitlement, metering, forecast, chargeback, and downgrade honesty across the claimed managed deployment profiles

Reviewer contract for the canonical honesty-certification packet: the cross-lane
certification that the managed-service economics boundary tells the truth before
any M5 surface promotes the managed claim. One row per honesty dimension —
entitlement, metering, forecast, chargeback, downgrade/offboarding, and
commercial-boundary honesty — certifies that the backing lane's behavior holds
across the claimed deployment profiles and narrows the marketed claim the moment a
certification drill fails or its evidence goes stale. This row is a depth-lane
proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/service/m5-commercial-honesty-certification.json`
- Boundary schema: `schemas/service/m5-commercial-honesty-certification.schema.json`
- Human-readable rendering: `artifacts/m5/certify-entitlement-metering-forecast-chargeback-and-downgrade-honesty-across-claimed-m5-managed-deployment-profiles.md`
- Overview companion: `docs/service/m5_commercial_honesty_certification.md`
- Fixture corpus: `fixtures/service/m5-commercial-honesty-certification/`
- Owning crate module: `crates/aureline-service/src/m5_commercial_honesty_certification/`

## Rides real consumers, not a parallel scorecard

The certification does not retype the truth the sibling lanes already freeze. Each
row names a `backing_consumer`, and
`HonestyCertificationPacket::cross_check_backing_consumers` loads the
commercial-control-plane matrix (`docs/service/m5_commercial_control_plane.md`) and
each sibling packet — the entitlement summary, the usage and forecast views, the
metering-degradation rules, the chargeback-scope views, the offboarding cards, and
the commercial-boundary cards — confirms each validates cleanly, confirms the
boundary cards still project the control plane, and confirms each row cites the
backing consumer its dimension requires. The closed marketed-claim, service-family,
and deployment-profile vocabularies are re-exported from the control plane and the
boundary cards rather than minting a parallel synonym set; the new tokens are only
the certification vocabulary those sources did not carry: the honesty dimension,
the certification drill, the drill grade, and the certification surface.

## The rows

One row per honesty dimension — a 6-row set:

- **Entitlement honesty** — backed by the entitlement summary. Certifies that
  plan, seat owner, role, scope, and quota-snapshot age render distinctly, that a
  seat loss or org switch degrades to an explicit managed-blocked state rather
  than a generic sign-in error, and that local-only continuation always holds.
  Drills: seat-loss, org-switch, grace-period, export-rights validation.
- **Metering honesty** — backed by the metering-degradation rules. Certifies that
  a stale meter keeps its number labeled, that the local core fails open, and that
  exactly one named spend-bearing managed action fails closed with its blocking
  reason. Drills: stale-meter, fail-open-local-core, fail-closed-managed-action.
- **Forecast honesty** — backed by the usage and forecast views. Certifies that
  the month-to-date measurement stays bound to its unit, as-of time, and scope
  owner, that the forecast banner explains what changes next, and that the view
  exports at CSV/JSON parity. Drills: stale-meter, fail-open-local-core,
  export-rights validation.
- **Chargeback honesty** — backed by the chargeback-scope views. Certifies that
  personal, workspace, team, and organization cost stay distinct, that direct cost
  is separated from an inherited share that names its parent scope, and that the
  scope set exports at parity. Drills: chargeback-scope export check, export-rights
  validation.
- **Downgrade/offboarding honesty** — backed by the offboarding cards. Certifies
  that a grace period, a seat loss, a cancellation, and an org switch stay
  distinct, that each states its effective date, impacted features, export rights,
  deletion timeline, and owner handoff, and that export and local continuation stay
  above any upgrade prompt. Drills: seat-loss, org-switch, grace-period,
  export-rights validation.
- **Commercial-boundary honesty** — backed by the commercial-boundary cards.
  Certifies that the open-versus-paid boundary, the residual vendor-hosted
  dependency disclosure, the deployment-profile qualifiers, and the
  procurement/support packet parity all hold, including the air-gapped profile
  where the local open core stands alone. Drills: residual-dependency disclosure
  review, export-rights validation.

## What the packet proves

- **Local-core productivity is never blocked.** Every row keeps a non-empty
  `local_safe_baseline`, so a failed certification narrows only an optional managed
  claim — never local editing, search, Git, or already-authorized local
  automation.
- **A failed drill narrows the claim, not the marketing.** Each drill's cap is the
  weaker of its grade cap and its evidence-freshness cap, and a row's
  `effective_certified_claim` is the weakest of its declared claim and every drill
  cap. A narrowed drill or stale evidence drops `managed_full` to
  `managed_narrowed` (or `local_safe_only`) automatically, so a row that fails
  commercial-boundary or downgrade honesty narrows instead of inheriting broader
  managed marketing language.
- **Stale evidence never stays green across release trains.** A `certified` drill
  with `stale` evidence still narrows the row; `missing` or `downgraded` evidence
  drops it to the local-safe baseline.
- **The four account-loss conditions stay distinct.** The seat-loss, org-switch,
  and grace-period drills certify that a seat loss, an org switch, a grace window,
  and a sign-in failure are never collapsed into one generic account error — that
  distinctness is owned by `m5_commercial_control_plane` and `m5_offboarding_cards`
  and exercised here.
- **No number is certified bare.** The forecast and chargeback rows certify that
  every exposed figure is bound to its unit, as-of time, and scope owner; the
  certification itself carries no spend or quota numbers.
- **Not certified from one vendor-managed online profile alone.** Every row
  partitions all five deployment profiles between certified and not-offered, so the
  self-host, air-gapped, and mirror profiles are always addressed. The
  commercial-boundary row certifies in every profile, including air-gapped, and at
  least one self-host or air-gapped profile is certified overall.
- **One packet, many surfaces.** The release center, Help/About, diagnostics,
  service health, the support/admin packet, and claim/public-truth automation each
  bind the packet and project the effective certified claim — never a stronger one.
  The release center and claim automation narrow the marketed claim on a failed
  certification.

## Regeneration

`canonical_honesty_certification_packet` builds the packet;
`current_stable_honesty_certification_packet` reads and validates the checked-in
artifact. Drift between a stored value and the recomputation is a test failure in
`crates/aureline-service/src/m5_commercial_honesty_certification/tests.rs`.
Regenerate the artifact with
`cargo run -p aureline-service --example dump_m5_commercial_honesty_certification -- canonical`.
