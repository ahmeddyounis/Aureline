# Stale-or-unreachable metering degradation rules with fail-open local-core behavior and fail-closed optional managed-action gates

Reviewer contract for the canonical metering-degradation rule set: the runtime degradation
behavior for each claimed managed lane — the AI gateway, settings sync, the companion relay,
the registry/mirror surface, support ingest, and the managed workspace — when a metering or
rating path goes stale or unreachable. This row is a depth-lane proof governed by the
canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/service/m5-metering-degradation-rules.json`
- Boundary schema: `schemas/service/m5-metering-degradation-rules.schema.json`
- Human-readable rendering: `artifacts/m5/add-stale-or-unreachable-metering-degradation-rules-fail-open-local-core-behavior-and-fail-closed-optional-managed-action-gates-across-m5-commercial-surfaces.md`
- Overview companion: `docs/service/m5_metering_degradation_rules.md`
- Fixture corpus: `fixtures/service/m5-metering-degradation-rules/`
- Owning crate module: `crates/aureline-service/src/m5_metering_degradation_rules/`

## Projects the frozen control-plane matrix

Each rule reuses the closed vocabularies already frozen by the commercial-control-plane
matrix (`docs/service/m5_commercial_control_plane.md`) — the service-family, meter-family,
fail-posture, managed-state, marketed-claim, and consumer-surface classes — plus the
snapshot-freshness vocabulary from the entitlement summaries
(`docs/service/m5_entitlement_summary.md`), rather than minting a parallel synonym set. Each
rule's `lane_ref` resolves to a control-plane lane, and
`MeteringDegradationRuleSet::cross_check_against_control_plane` confirms the rule's service
family, meter family, and fail posture match the lane and that the disposition is the
matrix's posture recomputed. The new tokens are only the degradation vocabulary the matrix
did not carry: the degradation trigger, the disposition, the value disclosure, and the
action kind.

## The matrix

One rule per service family and degradation trigger — an exhaustive 6 × 3 matrix:

- Triggers: `metering_stale`, `service_unreachable`, `rating_path_unavailable`.
- Fail-open lanes (local-safe path or labeled number, no gate): the AI gateway, settings
  sync, the registry/mirror surface, and support ingest.
- Fail-closed lanes (one spend-bearing optional action gated): the companion relay and the
  managed workspace.

That is 12 fail-open rules and 6 fail-closed rules, each projecting the lane's frozen fail
posture.

## What the set proves

- **Local-core productivity is never blocked.** Every rule keeps a non-empty
  `local_safe_promise` and sets `narrows_to_local_safe_only` to `false`, so a stale or
  unreachable metering path narrows only the relevant managed action and never local
  editing, search, save, Git, or already-authorized local automation.
- **Fail-open and fail-closed behavior matches the frozen matrix.** Each `disposition` is
  recomputed from the lane's `fail_posture` via `DegradationDisposition::for_posture`, so a
  fail-open lane falls back to its local-safe path or shows a labeled number and a
  fail-closed lane gates exactly one optional action. The cross-check against the
  control-plane lane is a test failure on drift.
- **A gate is specific, not generic.** A fail-closed rule names exactly one optional managed
  action in `gated_optional_action` together with a `blocking_reason` (for example, "Spend
  cannot be bounded because the meter is stale, so this one action waits"); it never fails
  closed across the whole lane when the risk applies to one action.
- **No spend or quota number without unit, as-of time, and scope owner.** Under
  `metering_stale` the last-known number is `labeled_stale_bound_to_unit_as_of_scope` with
  `freshness_stale`; under `service_unreachable` and `rating_path_unavailable` it is
  `suppressed_no_managed_number` with `freshness_unknown`. Every rule carries a last-contact
  `as_of` time.
- **A degradation is not a generic account error.** Every rule sets `not_an_account_error`
  and lists `seat_removed`, `org_switched`, `grace_period`, and `reauth_required` in
  `distinct_from_account_states`, so a metering posture never collapses a seat loss, an org
  switch, a grace window, and a sign-in failure into one error. Only the `metering_stale`
  trigger borrows the `meter_stale` managed state.
- **The marketed claim narrows automatically.** Every rule declares `managed_full` and
  narrows the effective claim to `managed_narrowed` — never staying full and never collapsing
  to `local_safe_only` — so a marketed managed claim narrows when its metering evidence goes
  stale or unreachable.
- **One packet, many surfaces.** Service-health diagnostics, the account/usage surface,
  Help/About, the support/admin packet, and claim/public-truth automation each bind to the
  set and project the effective claim — never a stronger one — render the local-safe promise,
  and name the blocking reason.

## Regeneration

`canonical_stable_metering_degradation_rule_set` builds the set;
`current_stable_metering_degradation_rule_set` reads and validates the checked-in packet.
Drift between a stored value and the recomputation is a test failure in
`crates/aureline-service/src/m5_metering_degradation_rules/tests.rs`. Regenerate the artifact
with `cargo run -p aureline-service --example dump_m5_metering_degradation_rules -- canonical`.
