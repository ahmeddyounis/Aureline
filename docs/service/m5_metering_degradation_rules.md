# Metering degradation rules

The metering-degradation rule set is the canonical, inspectable description of how a managed
lane behaves when its metering or rating path goes stale or unreachable. Where the
commercial-control-plane matrix (`docs/service/m5_commercial_control_plane.md`) freezes the
per-lane fail posture and the usage-and-forecast views
(`docs/service/m5_usage_forecast_views.md`) render the customer-visible number, this set
freezes the runtime degradation behavior: which local-safe promise keeps running, whether
the one spend-bearing optional managed action gates and why, and the retry and details
actions the surface offers. It is owned by the `aureline-service` crate
(`crates/aureline-service/src/m5_metering_degradation_rules/`), checked in at
`artifacts/service/m5-metering-degradation-rules.json`, and bounded by
`schemas/service/m5-metering-degradation-rules.schema.json`.

## What it freezes

- **One rule per service family and degradation trigger.** The AI gateway, settings sync,
  the companion relay, the registry/mirror surface, support ingest, and the managed
  workspace each carry a rule for a stale meter, an unreachable metering service, and an
  unavailable rating path — an exhaustive 6 × 3 matrix of 18 rules. Each rule names the
  affected service family, the control-plane lane it projects, the fail posture, the
  disposition, the non-empty local-safe promise, the gated optional action and its blocking
  reason (when the lane fails closed), the value disclosure, the freshness and last-contact
  as-of time, the retry and details actions, and the account-loss states it stays distinct
  from.
- **One binding per consumer surface.** Service-health diagnostics, the account/usage
  surface, Help/About, the support/admin packet, and claim/public-truth automation each
  resolve through the rules rather than retyping their state, projecting the effective claim,
  rendering the local-safe promise, and naming the blocking reason.

## Invariants

- The local core fails open and never blocks: every rule keeps a non-empty local-safe
  promise and `narrows_to_local_safe_only` is always `false`, so a stale or unreachable
  metering path narrows only the relevant managed action and never local editing, search,
  Git, or already-authorized local automation.
- The disposition matches the frozen matrix: it is recomputed from the lane's fail posture,
  so a fail-open lane keeps its local-safe path and a fail-closed lane gates exactly one
  optional action. `cross_check_against_control_plane` confirms the posture against the
  canonical control-plane lane.
- A gate is specific, not generic: a fail-closed rule names exactly one optional managed
  action and a blocking reason rather than failing closed across the whole lane.
- No number crosses the boundary bare: a stale number is labeled and bound to its unit,
  as-of time, and scope owner; an unreachable number is suppressed; every rule carries a
  last-contact as-of time.
- A degradation is metering posture, not an account error: every rule stays distinct from a
  seat loss, an org switch, a grace window, and a sign-in failure, and only the stale
  trigger borrows the `meter_stale` managed state.
- The marketed claim narrows automatically: every rule declares the full managed claim and
  narrows the effective claim to managed-narrowed — never staying full, never collapsing to
  local-safe-only.

## How to consume it

Call `current_stable_metering_degradation_rule_set()` to read and validate the checked-in
set; call `MeteringDegradationRuleSet::rule_for(family, trigger)` to resolve a single rule,
`MeteringDegradationRuleSet::rules_for_family(family)` to list a family's rules, and
`MeteringDegradationRuleSet::cross_check_against_control_plane()` to confirm each rule
projects its control-plane lane's fail posture. The reviewer contract is
`docs/m5/add-stale-or-unreachable-metering-degradation-rules-fail-open-local-core-behavior-and-fail-closed-optional-managed-action-gates-across-m5-commercial-surfaces.md`.
