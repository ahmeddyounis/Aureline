# Profile-Compare Cards, Threshold or Waiver State, and Confounder Disclosure

## Scope

This document governs the M5 profile-compare surface family:

- Profile-compare cards that show side-by-side, delta, trend, or baseline-vs-current comparisons.
- Threshold inspector that shows whether a comparison metric is within bounds, in warning, breached, waived, or provisional.
- Waiver badge that surfaces waiver status, cause, expiry proximity, and honest expiry labels.
- Confounder disclosure panel that names factors that may weaken or invalidate the comparison claim.

## Contract

Every profile-compare surface MUST resolve through the checked-in qualification packet at:

- [`/artifacts/perf/m5/implement-profile-compare-cards-threshold-or-waiver-state-and-confounder-disclosure.json`](../../../artifacts/perf/m5/implement-profile-compare-cards-threshold-or-waiver-state-and-confounder-disclosure.json)
- Schema: [`/schemas/perf/implement-profile-compare-cards-threshold-or-waiver-state-and-confounder-disclosure.schema.json`](../../../schemas/perf/implement-profile-compare-cards-threshold-or-waiver-state-and-confounder-disclosure.schema.json)

## Invariants

1. **Capture identity visible**: Every compare card shows left and right capture refs so users know exactly what is being compared.
2. **Comparison basis visible**: The comparison kind (side-by-side, delta, trend, baseline-vs-current) is explicit.
3. **Threshold state honest**: Threshold states are `within`, `warning`, `breach`, `waived`, or `provisional`. A breached metric MUST NOT render as passing unless an active waiver covers it.
4. **Waiver expiry visible**: Waiver badges show status, cause, and expiry proximity. Expired waivers MUST NOT cover breaches.
5. **Confounder disclosure mandatory**: Compare cards MUST show confounder disclosures when present. Blocking confounders (critical or major severity) MUST narrow the comparison claim automatically.
6. **Downgrade on weak guards**: Surfaces downgrade from `stable` to `preview` or below when required guards are missing or when blocking confounders are present.

## Surface Guard Set

A stable profile-compare surface MUST show:

- Compare card
- Threshold inspector
- Waiver badge
- Confounder disclosure panel
- Capture identity (left and right)
- Comparison basis
- Threshold bar
- Waiver expiry
- Degraded-state label when applicable
- Mapping quality

## Types

### ProfileCompareCardRow

Binds left and right capture refs, comparison kind, threshold state ref, and confounder refs.

### ThresholdStateRow

Carries metric family, threshold value, current value, threshold state, and visual-bar truth.

### WaiverStateRow

Carries threshold ref, waiver status, waiver cause, expiry proximity, and honest expiry labels.

### ConfounderDisclosureRow

Carries confounder kind, severity, blocking status, disclosure text, and mitigation hint.

## Fixtures

- [`fixtures/performance/m5/implement-profile-compare-cards-threshold-or-waiver-state-and-confounder-disclosure/`](../../../fixtures/performance/m5/implement-profile-compare-cards-threshold-or-waiver-state-and-confounder-disclosure/)

## Integration

Consumers:

- `crates/aureline-profiler` — owns the typed records and qualification packet.
- `crates/aureline-debug` — may reference compare cards when linking trace and profile evidence.
- `crates/aureline-support` — may include compare-card metadata in support exports.
- `docs/help` — help surfaces project the packet onto user-facing copy.
- `ci` — CI may validate the embedded packet on every build.
