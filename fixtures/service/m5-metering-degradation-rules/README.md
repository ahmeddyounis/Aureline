# Fixtures: metering-degradation rule set

This directory carries the fixture metadata for the frozen stale-or-unreachable
metering degradation rule set.

The canonical set is checked in at:

`artifacts/service/m5-metering-degradation-rules.json`

Its boundary schema is:

`schemas/service/m5-metering-degradation-rules.schema.json`

## Coverage

- The set freezes exactly one rule per service family (the AI gateway, settings sync, the
  companion relay, the registry/mirror surface, support ingest, and the managed workspace)
  and per degradation trigger (`metering_stale`, `service_unreachable`,
  `rating_path_unavailable`) — an exhaustive 6 × 3 matrix of 18 rules.
- Each rule names the affected service family, the control-plane lane it projects, the
  fail posture, the disposition, the non-empty local-safe promise, the gated optional
  action and its blocking reason (when the lane fails closed), how any number is disclosed,
  the measurement freshness and last-contact as-of time, the retry and details actions, and
  the account-loss states it stays distinct from.
- Five surface bindings — diagnostics, account, Help/About, support/admin packet, and
  claim/public-truth automation — each resolve through real rule ids.

## What the corpus proves

- **The local core never blocks.** Every rule keeps a non-empty `local_safe_promise` and
  sets `narrows_to_local_safe_only` to `false`, so a stale or unreachable metering path
  narrows only the relevant managed action and never local editing, search, Git, or
  existing local automation.
- **Fail posture matches the frozen matrix.** Each rule's `disposition` is recomputed from
  its control-plane lane's `fail_posture`: the four fail-open lanes (AI gateway, settings
  sync, registry/mirror, support ingest) keep their local-safe path and gate nothing, while
  the two fail-closed lanes (companion relay, managed workspace) gate exactly one
  spend-bearing optional action. `cross_check_against_control_plane` confirms the posture
  against the canonical control-plane lane.
- **A gate is specific, not generic.** A fail-closed rule names exactly one optional
  managed action in `gated_optional_action` together with a `blocking_reason`; it never
  fails closed across the whole lane.
- **No number crosses the boundary bare.** Under `metering_stale` the last-known number is
  `labeled_stale_bound_to_unit_as_of_scope` with `freshness_stale`; under
  `service_unreachable` and `rating_path_unavailable` it is `suppressed_no_managed_number`.
  Every rule carries a last-contact `as_of` time.
- **A degradation is not an account error.** Every rule sets `not_an_account_error` and
  lists `seat_removed`, `org_switched`, `grace_period`, and `reauth_required` in
  `distinct_from_account_states`; only the `metering_stale` trigger borrows the
  `meter_stale` managed state.
- **The marketed claim narrows automatically.** Every rule declares `managed_full` and
  narrows the effective claim to `managed_narrowed` — never staying full and never
  collapsing to `local_safe_only`.

## Regeneration

The set is built and validated by `canonical_stable_metering_degradation_rule_set`, which
recomputes every rule's disposition, value disclosure, freshness, and the inspection block;
any drift between a stored value and the recomputation is a test failure in
`crates/aureline-service/src/m5_metering_degradation_rules/tests.rs`. Regenerate the
checked-in artifact deterministically with:

```text
cargo run -p aureline-service --example dump_m5_metering_degradation_rules -- canonical \
  > artifacts/service/m5-metering-degradation-rules.json
```
