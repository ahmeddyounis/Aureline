# Fixtures: commercial honesty-certification packet

This directory carries the fixture metadata for the cross-lane certification of
entitlement, metering, forecast, chargeback, and downgrade/offboarding honesty
across the claimed managed deployment profiles.

The canonical packet is checked in at:

`artifacts/service/m5-commercial-honesty-certification.json`

Its boundary schema is:

`schemas/service/m5-commercial-honesty-certification.schema.json`

## Coverage

- The packet freezes exactly one certification row per honesty dimension —
  entitlement, metering, forecast, chargeback, downgrade/offboarding, and
  commercial-boundary honesty — a 6-row set.
- Each row names the sibling consumer packet that backs it, the managed service
  families it applies across, the deployment profiles it is certified in and the
  profiles its managed lane is honestly not offered in, the certification drills
  that exercise it, and a non-empty local-safe baseline.
- Across the rows, all nine certification drills are exercised: the stale-meter
  drill, the fail-open-local-core and fail-closed-managed-action drills, the
  seat-loss, org-switch, and grace-period drills, the export-rights validation,
  the chargeback-scope export check, and the residual-dependency disclosure
  review.
- Six surface bindings — the release center, Help/About, diagnostics, service
  health, the support/admin packet, and claim/public-truth automation — each
  project the effective certified claim.

## What the corpus proves

- **The local core is never blocked.** Every row keeps a non-empty
  `local_safe_baseline`, so a failed certification narrows only an optional
  managed claim, never local editing, search, Git, or already-authorized local
  automation.
- **The certification is never vendor-managed-online-only.** Every row partitions
  all five deployment profiles between `certified_profiles` and
  `not_offered_profiles`, so the self-host, air-gapped, and mirror profiles are
  always addressed. The commercial-boundary row certifies in every profile,
  including air-gapped, where the local open core stands alone; the metered lanes
  are honestly not offered individual-local or air-gapped.
- **A failed drill narrows the claim automatically.** Each drill's cap is the
  weaker of its grade cap (`certified`/`not_applicable` → `managed_full`,
  `narrowed` → `managed_narrowed`) and its evidence cap (`current` →
  `managed_full`, `stale` → `managed_narrowed`, `missing`/`downgraded` →
  `local_safe_only`). A row's `effective_certified_claim` is the weakest of its
  declared claim and every drill cap, so a narrowed drill or stale evidence drops
  the row's claim instead of inheriting broader managed marketing language.
- **Stale evidence never stays green.** A `certified` drill with `stale` evidence
  still narrows the row, so stale metering/export evidence cannot ride a green
  certification across release trains.
- **The certification rides real consumers.**
  `cross_check_backing_consumers` loads the control-plane matrix and each sibling
  packet, confirms they validate cleanly, and confirms each row cites the backing
  consumer its dimension requires — never a parallel scorecard.
- **Release tooling narrows from the verdict.** The release-center and
  claim/public-truth-automation bindings carry `narrows_on_failure: true`;
  diagnostics, Help/About, service health, and the support/admin packet project
  the effective claim read-only.

## Regeneration

The packet is built and validated by `canonical_honesty_certification_packet`,
which recomputes every drill cap, each row's effective claim, the narrowing
reasons, and the inspection block; any drift between a stored value and the
recomputation is a test failure in
`crates/aureline-service/src/m5_commercial_honesty_certification/tests.rs`.
Regenerate the checked-in artifact deterministically with:

```text
cargo run -p aureline-service --example dump_m5_commercial_honesty_certification -- canonical \
  > artifacts/service/m5-commercial-honesty-certification.json
```
