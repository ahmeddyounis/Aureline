# Commercial honesty certification

The honesty-certification packet is the canonical, inspectable certification that
the managed-service economics boundary tells the truth before any M5 surface
promotes the managed claim. Where the commercial-control-plane matrix
(`docs/service/m5_commercial_control_plane.md`) freezes the per-lane posture and
the managed-state vocabulary, and the sibling lanes each render one slice of the
truth — the entitlement summary
(`docs/service/m5_entitlement_summary.md`), the usage and forecast views
(`docs/service/m5_usage_forecast_views.md`), the metering-degradation rules
(`docs/service/m5_metering_degradation_rules.md`), the chargeback-scope views
(`docs/service/m5_chargeback_scope_views.md`), the offboarding cards
(`docs/service/m5_offboarding_cards.md`), and the commercial-boundary cards
(`docs/service/m5_commercial_boundary_cards.md`) — this packet **certifies** that
each honesty dimension actually holds across the claimed deployment profiles and
**narrows** the marketed claim the moment a drill fails or its evidence goes
stale. It is owned by the `aureline-service` crate
(`crates/aureline-service/src/m5_commercial_honesty_certification/`), checked in at
`artifacts/service/m5-commercial-honesty-certification.json`, and bounded by
`schemas/service/m5-commercial-honesty-certification.schema.json`.

## What it freezes

- **One certification row per honesty dimension.** Entitlement, metering,
  forecast, chargeback, downgrade/offboarding, and commercial-boundary honesty —
  a 6-row set. Each row names the sibling consumer that backs it, the managed
  service families it applies across, the deployment profiles it is certified in
  and the profiles its managed lane is honestly not offered in, the certification
  drills that exercise it, and a non-empty local-safe baseline.
- **Nine certification drills across the rows.** The stale-meter drill, the
  fail-open-local-core and fail-closed-managed-action drills, the seat-loss,
  org-switch, and grace-period drills, the export-rights validation, the
  chargeback-scope export check, and the residual-dependency disclosure review.
  Each drill carries a grade and a backing-evidence freshness status.
- **One binding per surface.** The release center, Help/About, diagnostics,
  service health, the support/admin packet, and claim/public-truth automation each
  project the effective certified claim; the release center and claim automation
  narrow the marketed claim on a failed certification.

## Invariants

- The local core is never blocked: every row keeps a non-empty
  `local_safe_baseline`, so a failed certification narrows only an optional
  managed claim, never local editing, search, Git, or already-authorized local
  automation.
- A failed drill narrows the claim automatically: each drill's cap is the weaker
  of its grade cap (`certified`/`not_applicable` → `managed_full`, `narrowed` →
  `managed_narrowed`) and its evidence cap (`current` → `managed_full`, `stale` →
  `managed_narrowed`, `missing`/`downgraded` → `local_safe_only`), and a row's
  `effective_certified_claim` is the weakest of its declared claim and every drill
  cap, so a narrowed drill or stale evidence drops the row's claim instead of
  inheriting broader managed marketing language.
- Stale evidence never stays green: a `certified` drill with `stale` evidence
  still narrows the row, so stale metering/export evidence cannot ride a green
  certification across release trains.
- The certification is never vendor-managed-online-only: every row partitions all
  five deployment profiles between `certified_profiles` and
  `not_offered_profiles`, the commercial-boundary row certifies in every profile
  including air-gapped, and at least one self-host or air-gapped profile is
  certified.
- The certification rides real consumers: `cross_check_backing_consumers` loads
  the control-plane matrix and each sibling packet, confirms they validate
  cleanly, and confirms each row cites the backing consumer its dimension
  requires — never a parallel scorecard.

## How to consume it

Call `current_stable_honesty_certification_packet()` to read and validate the
checked-in packet; call `HonestyCertificationPacket::row(dimension)` to resolve a
single dimension, `HonestyCertificationPacket::narrow_for_drill_failure(dimension,
drill)` to exercise the narrowing deterministically, and
`HonestyCertificationPacket::cross_check_backing_consumers()` to confirm the
certification rides its backing sibling packets. The reviewer contract is
`docs/m5/certify-entitlement-metering-forecast-chargeback-and-downgrade-honesty-across-claimed-m5-managed-deployment-profiles.md`.
